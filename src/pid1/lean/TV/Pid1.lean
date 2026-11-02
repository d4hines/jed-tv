prelude
import Init.System.IO
import TV.Core
import TV.Volume.Native

/-! The native PID 1 program. Rust implements individual Linux/hardware operations;
A minimal Rust recovery HTTP boundary starts before Lean runtime initialization.
Lean owns subsequent service order and failure decisions. The launcher stage
enters Core synchronously on this same thread; its native poll services child
reaping and one finite Lean volume step. Failed startup retains the recovery
child-reaping loop below. -/
namespace TV.Pid1

inductive Stage where
  | recovery | services | launcher
  deriving BEq, Repr

def Stage.code : Stage → UInt32
  | .recovery => 0
  | .services => 1
  | .launcher => 2

/-- Stop at the first failed stage; no service starts before its prerequisite. -/
def startup (run : Stage → IO UInt32) : IO UInt32 := do
  for stage in [Stage.recovery, .services, .launcher] do
    let result ← run stage
    if result != 0 then return stage.code + 1
  return 0

@[extern "tv_pid1_stage"] opaque runStage (stage : UInt32) : IO UInt32
@[extern "tv_pid1_reap"] opaque reapChild : IO UInt32
@[extern "tv_pid1_notice"] opaque notice (event value : UInt32) : IO Unit
@[extern "tv_pid1_sleep"] opaque sleep (milliseconds : UInt32) : IO Unit

/-- Construct the real IO success object at the Rust boundary without duplicating
Lean's heap representation in Rust. The world token is erased at runtime. -/
@[export tv_pid1_io_ok]
def ioOk (value : UInt32) : IO UInt32 := pure value

@[export tv_pid1_io_unit]
def ioUnit (_ : UInt32) : IO Unit := pure ()

/-- Reap all currently exited children before sleeping; Rust preserves each raw
wait status for the foreground app's Lean supervisor policy. -/
partial def drainChildren (count : UInt32 := 0) : IO UInt32 := do
  let result ← reapChild
  if result == 0 then return count
  if result == 0xffffffff then
    notice 1 count
    return count
  drainChildren (count + 1)

partial def eventLoop : IO UInt32 := do
  let count ← drainChildren
  if count != 0 then notice 2 count
  sleep 20
  eventLoop

@[export tv_pid1_main]
def main : IO UInt32 := do
  let failed ← startup (fun stage => runStage stage.code)
  if failed != 0 then notice 0 failed
  -- PID 1 must remain alive even when a stage failed. Diagnostics that started
  -- successfully stay available, and exited recovery children are still reaped.
  eventLoop

end TV.Pid1
