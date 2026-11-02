prelude
import Init.Data.UInt
import Init.Data.List.Lemmas
set_option compiler.extract_closed false

/-! Private settings storage policy. Rust supplies raw metadata and performs the
requested filesystem operations. The scalar exports allocate no Lean objects,
so the same transaction driver can be used by a native Wi-Fi worker without
entering the supervisor's single Lean runtime. No PIN or file format lives here.

These guarantees assume native metadata describes the opened object and each
operation reports its actual result. A successful fsync is the kernel's durability
contract; this module does not prove the filesystem, device or power-loss model.
-/
namespace TV.Storage

def directoryAccepted (mode uid owner : UInt32) : Bool :=
  mode &&& 0xf000 == 0x4000 && uid == owner && mode &&& 0x1ff == 0x1c0

def fileAccepted (mode uid owner : UInt32) (links bytes : UInt64) : Bool :=
  mode &&& 0xf000 == 0x8000 && uid == owner && mode &&& 0x1ff == 0x180 &&
  links == 1 && bytes ≤ 64

def blockAccepted (mode : UInt32) (actual expected : UInt64) : Bool :=
  mode &&& 0xf000 == 0x6000 && actual == expected

def cacheGeometryAccepted (start sectors : UInt64) : Bool :=
  start == 9019392 && sectors == 204800

def filesystemAccepted (magic blocks shift : UInt32) : Bool :=
  magic == 0xef53 && shift ≤ 6 &&
  (blocks.toUInt64 <<< (10 + shift).toUInt64) == 104857600

inductive MountDecision where
  | mounted | mount | refuse
  deriving DecidableEq

def mountDecision (actual expected : UInt64) (owner : UInt32) : MountDecision :=
  if actual == expected then .mounted else if owner == 0 then .mount else .refuse

@[export tv_storage_directory_accepted]
def directoryWord (mode uid owner : UInt32) : UInt32 :=
  if directoryAccepted mode uid owner then 1 else 0

@[export tv_storage_file_accepted]
def fileWord (mode uid owner : UInt32) (links bytes : UInt64) : UInt32 :=
  if fileAccepted mode uid owner links bytes then 1 else 0

@[export tv_storage_block_accepted]
def blockWord (mode : UInt32) (actual expected : UInt64) : UInt32 :=
  if blockAccepted mode actual expected then 1 else 0

@[export tv_storage_cache_geometry_accepted]
def cacheGeometryWord (start sectors : UInt64) : UInt32 :=
  if cacheGeometryAccepted start sectors then 1 else 0

@[export tv_storage_filesystem_accepted]
def filesystemWord (magic blocks shift : UInt32) : UInt32 :=
  if filesystemAccepted magic blocks shift then 1 else 0

@[export tv_storage_mount_decision]
def mountWord (actual expected : UInt64) (owner : UInt32) : UInt32 :=
  match mountDecision actual expected owner with
  | .mounted => 0 | .mount => 1 | .refuse => 2

/-- Each successful action unlocks exactly its successor. Any failure requests
temporary-file cleanup; cleanup's result cannot turn failure into success.
Rename can already have committed the bytes when directory sync fails: report
failure, without pretending the old file can be restored. -/
inductive CommitStage where
  | create | write | syncFile | rename | syncDirectory | cleanup | succeeded | failed
  deriving DecidableEq

def CommitStage.code : CommitStage → UInt32
  | .create => 0 | .write => 1 | .syncFile => 2 | .rename => 3
  | .syncDirectory => 4 | .cleanup => 5 | .succeeded => 6 | .failed => 7

def CommitStage.fromCode (code : UInt32) : CommitStage :=
  if code == 0 then .create else if code == 1 then .write
  else if code == 2 then .syncFile else if code == 3 then .rename
  else if code == 4 then .syncDirectory else if code == 5 then .cleanup
  else if code == 6 then .succeeded else .failed

def commitNext (stage : CommitStage) (ok : Bool) : CommitStage :=
  match stage with
  | .create => if ok then .write else .cleanup
  | .write => if ok then .syncFile else .cleanup
  | .syncFile => if ok then .rename else .cleanup
  | .rename => if ok then .syncDirectory else .cleanup
  | .syncDirectory => if ok then .succeeded else .cleanup
  | .cleanup => .failed
  | .succeeded => .succeeded
  | .failed => .failed

@[export tv_storage_commit_start]
def commitStart (_ : UInt32) : UInt32 := CommitStage.create.code

@[export tv_storage_commit_next]
def commitWord (stage ok : UInt32) : UInt32 :=
  (commitNext (CommitStage.fromCode stage) (ok == 1)).code

/-- Invalid metadata is never accepted as a private settings directory. -/
theorem directoryAcceptance (mode uid owner : UInt32) :
    directoryAccepted mode uid owner = true ↔
      mode &&& 0xf000 = 0x4000 ∧ uid = owner ∧ mode &&& 0x1ff = 0x1c0 := by
  simp [directoryAccepted, Bool.and_eq_true, and_assoc]

/-- Existing settings must be a single-link regular file owned by the
supervisor, exactly mode 0600, and bounded before any bytes are parsed. -/
theorem fileAcceptance (mode uid owner : UInt32) (links bytes : UInt64) :
    fileAccepted mode uid owner links bytes = true ↔
      mode &&& 0xf000 = 0x8000 ∧ uid = owner ∧ mode &&& 0x1ff = 0x180 ∧
      links = 1 ∧ bytes ≤ 64 := by
  simp [fileAccepted, Bool.and_eq_true, and_assoc]

theorem mountNeedsRoot (actual expected : UInt64) (owner : UInt32)
    (h : mountDecision actual expected owner = .mount) : owner = 0 := by
  simp only [mountDecision] at h
  split at h
  · contradiction
  · split at h
    · simpa using ‹(owner == 0) = true›
    · contradiction

theorem commitCodesRoundTrip (stage : CommitStage) :
    CommitStage.fromCode stage.code = stage := by
  cases stage <;> decide

/-- No action before successful directory sync may report a newly completed
commit. The succeeded state is terminal and can only preserve prior success. -/
theorem successNeedsDirectorySync (stage : CommitStage) (ok : Bool) :
    commitNext stage ok = .succeeded ↔
      (stage = .syncDirectory ∧ ok = true) ∨ stage = .succeeded := by
  cases stage <;> cases ok <;> simp [commitNext]

def commitRun (stage : CommitStage) : List Bool → CommitStage
  | [] => stage
  | result :: rest => commitRun (commitNext stage result) rest

theorem failureCannotBecomeSuccess (results : List Bool) :
    commitRun .failed results = .failed := by
  induction results with
  | nil => rfl
  | cons ok rest ih => simpa [commitRun, commitNext] using ih

theorem cleanupAlwaysFails (ok : Bool) : commitNext .cleanup ok = .failed := by
  rfl

theorem cleanupCannotSucceed (results : List Bool) :
    commitRun .cleanup results ≠ .succeeded := by
  cases results with
  | nil => simp [commitRun]
  | cons ok rest => simp [commitRun, commitNext, failureCannotBecomeSuccess]

/-- The five required operations must all succeed, in order. -/
theorem completeCommit (create write syncFile rename syncDirectory : Bool) :
    commitRun .create [create, write, syncFile, rename, syncDirectory] = .succeeded ↔
      create = true ∧ write = true ∧ syncFile = true ∧ rename = true ∧ syncDirectory = true := by
  cases create <;> cases write <;> cases syncFile <;> cases rename <;>
    cases syncDirectory <;> decide

end TV.Storage
