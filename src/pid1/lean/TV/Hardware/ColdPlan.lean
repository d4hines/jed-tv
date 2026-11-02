prelude
import Init.System.IO
import Init.Data.String.Basic
import Init.Data.List.Basic
import TV.SettingsFile

set_option compiler.extract_closed false

/-! Exact G06/V109 cold-staging INI policy. This module neither establishes
cold-boot ownership nor accesses the filesystem. The native adapter retains
bounded UTF-8 reads, the read-only eMMC guards, mounts and readback checks. -/
namespace TV.Hardware.ColdPlan

structure Failure where
  /-- The adapter maps 1 to io::ErrorKind::Other and 2 to InvalidData. -/
  kind : UInt32
  message : String

private def other (message : String) : Except Failure α := .error ⟨1, message⟩
private def invalid (message : String) : Except Failure α := .error ⟨2, message⟩

private def flags : List String :=
  ["MI_SYS_INIT", "MI_UART_INIT", "MI_WDT_INIT", "MI_IIC_INIT", "MI_GPIO_INIT",
   "MI_FLASH_INIT", "MI_DISP_INIT", "MI_DISPOUT_INIT", "MI_EXTIN_INIT", "MI_TUNER_INIT",
   "MI_DMX_INIT", "MI_TSIO_INIT", "MI_VIDEO_INIT", "MI_VMON_INIT", "MI_AUDIO_INIT",
   "MI_AOUT_INIT", "MI_PCM_INIT", "MI_SYNC_INIT", "MI_CAP_INIT", "MI_VENC_INIT",
   "MI_OSD_INIT", "MI_IMGDEC_INIT", "MI_INJECT_INIT", "MI_CEC_INIT", "MI_SAR_INIT",
   "MI_PWM_INIT", "MI_SMC_INIT", "MI_DSC_INIT", "MI_KL_INIT", "MI_CIPHER_INIT",
   "MI_ADVCA_INIT", "MI_IR_INIT", "MI_PCMCIA_INIT", "MI_TSR_INIT", "MI_TSENSOR_INIT"]

private def parameters : List (String × String) :=
  [("MI_DISP_OUTPUT_TIMING", "6"), ("MI_DISP_TV_SYSTEM", "6"),
   ("MI_DISP_DE_FLICKER", "0"), ("MI_OSD_WAIT_SYNC", "0"),
   ("MI_OSD_AUTO_FLIP", "0"), ("MI_OSD_WINDOW_CANVAS_MODE", "0"),
   ("MI_OSD_SUPPORT_SURFACE_COUNT", "0"), ("MI_OSD_WAIT_IDLE", "0")]

private def trimStart : List Char → List Char
  | [] => []
  | c :: rest => if TV.SettingsFile.whitespace c then trimStart rest else c :: rest

private def trim (cs : List Char) : List Char :=
  (trimStart (trimStart cs).reverse).reverse

private def uncommentLoop : List Char → List Char → List Char
  | [], acc => acc.reverse
  | c :: rest, acc =>
    if c == '#' || c == ';' then acc.reverse else uncommentLoop rest (c :: acc)

private def uncomment (cs : List Char) : List Char := uncommentLoop cs []

/-- Split once, retaining the unmodified revHead and suffix. -/
private def atEqual : List Char → List Char → Option (List Char × List Char)
  | [], _ => none
  | c :: rest, revHead =>
    if c == '=' then some (revHead.reverse, rest) else atEqual rest (c :: revHead)

/-- LF alone divides lines. CR and all other Unicode whitespace stay in the
line and are handled by Rust-compatible trim, never as extra line separators. -/
private def collectLines (keepLf : Bool) : List Char → List Char → List (List Char) → List (List Char)
  | [], acc, collected => (match acc with | [] => collected | _ => acc.reverse :: collected).reverse
  | '\n' :: rest, acc, collected =>
    let line := if keepLf then ('\n' :: acc).reverse else acc.reverse
    collectLines keepLf rest [] (line :: collected)
  | c :: rest, acc, collected => collectLines keepLf rest (c :: acc) collected

private def lines (cs : List Char) : List (List Char) := collectLines false cs [] []

private def hasKey (key : String) : List (String × String) → Bool
  | [] => false
  | (k, _) :: rest => k == key || hasKey key rest

private def lookup (key : String) : List (String × String) → Option String
  | [] => none
  | (k, v) :: rest => if k == key then some v else lookup key rest

private def known (key : String) : List String → Bool
  | [] => false
  | k :: rest => k == key || known key rest

private def parse : List (List Char) → List (String × String) → Except Failure (List (String × String))
  | [], values => .ok values
  | line :: rest, values =>
    let line := trim (uncomment line)
    match line with
    | [] | '[' :: _ => parse rest values
    | _ => match atEqual line [] with
      | none => parse rest values
      | some (rawKey, rawValue) =>
        let key := String.ofList (trim rawKey)
        let value := String.ofList (trim rawValue)
        if hasKey key values then invalid ("duplicate INI key " ++ key)
        else parse rest ((key, value) :: values)

private def validateFlags (values : List (String × String)) : List String → Except Failure Unit
  | [] => .ok ()
  | key :: rest =>
    let expected := if key == "MI_SYS_INIT" || key == "MI_FLASH_INIT" then "1" else "0"
    if lookup key values != some expected then
      invalid ("explicit-init recipe requires " ++ key ++ "=" ++ expected)
    else validateFlags values rest

private def charEq : List Char → List Char → Bool
  | [], [] => true
  | a :: as, b :: bs => a == b && charEq as bs
  | _, _ => false

private def begins : List Char → List Char → Bool
  | [], _ => true
  | a :: as, b :: bs => a == b && begins as bs
  | _, _ => false

private def initializer (key : String) : Bool :=
  begins ['T', 'I', 'N', 'I', '_'] key.toList.reverse

/-- UTF-8 byte order and Unicode-scalar order agree for valid Rust strings. -/
private def keyLess : List Char → List Char → Bool
  | [], [] => false
  | [], _ :: _ => true
  | _ :: _, [] => false
  | a :: as, b :: bs =>
    if a.toNat == b.toNat then keyLess as bs else a.toNat < b.toNat

private def unknown (values : List (String × String)) (best : Option String) : Option String :=
  match values with
  | [] => best
  | (key, value) :: rest =>
    if initializer key && !known key flags && value != "0" then
      let best := match best with
        | none => some key
        | some old => if keyLess key.toList old.toList then some key else best
      unknown rest best
    else unknown rest best

private def validateParameters (values : List (String × String)) : List (String × String) → Except Failure Unit
  | [] => .ok ()
  | (key, expected) :: rest =>
    if lookup key values != some expected then
      invalid ("exact V109 payload requires " ++ key ++ "=" ++ expected)
    else validateParameters values rest

/-- The exact explicit-init recipe: SYS/FLASH enabled, the other 33 known
initializers disabled, unknown initializers disabled, and eight fixed values. -/
def validate (text : String) : Except Failure Unit := do
  let values ← parse (lines text.toList) []
  validateFlags values flags
  if let some key := unknown values none then invalid ("unknown enabled INIT key " ++ key)
  else validateParameters values parameters

/-- Keep LF bytes in the lines so successful preparation preserves the entire
UTF-8 input except for the one ASCII tuner value byte. -/
private def linesKeep (cs : List Char) : List (List Char) := collectLines true cs [] []

private def replaceOneLoop : List Char → List Char → List Char
  | [], acc => acc.reverse
  | '1' :: rest, acc => acc.reverse ++ '0' :: rest
  | c :: rest, acc => replaceOneLoop rest (c :: acc)

private def replaceOne (cs : List Char) : List Char := replaceOneLoop cs []

private def prepareLines : List (List Char) → Nat → List Char → Except Failure (Nat × List Char)
  | [], changed, output => .ok (changed, output.reverse)
  | line :: rest, changed, output =>
    match atEqual line [] with
    | some (key, value) =>
      if charEq (trim key) ['M','I','_','T','U','N','E','R','_','I','N','I','T'] then
        if !charEq (trim (uncomment value)) ['1'] then
          other "expected stock MI_TUNER_INIT=1"
        else
          let line := key ++ ['='] ++ replaceOne value
          prepareLines rest (changed + 1) (line.reverse ++ output)
      else prepareLines rest changed (line.reverse ++ output)
    | none => prepareLines rest changed (line.reverse ++ output)

/-- Consume the original stock INI once. Already-prepared input is rejected;
success does not grant permission to repeat staging or initialize hardware. -/
def prepare (text : String) : Except Failure String := do
  let (changed, output) ← prepareLines (linesKeep text.toList) 0 []
  if changed != 1 then other "expected one stock tuner INIT key"
  else
    let result := String.ofList output
    validate result
    .ok result

/-- The accepted hardware recipe has exactly the audited initializer set. -/
theorem flag_count : flags.length = 35 := by decide

theorem flag_keys_unique : flags.Nodup := by decide

theorem parameter_count : parameters.length = 8 := by decide

theorem parameter_keys_unique : (parameters.map Prod.fst).Nodup := by decide

/-- Preparation cannot publish bytes that fail the full explicit-init recipe. -/
theorem prepare_success_validates (text prepared : String)
    (accepted : prepare text = .ok prepared) : validate prepared = .ok () := by
  unfold prepare at accepted
  cases hlines : prepareLines (linesKeep text.toList) 0 [] with
  | error failure => simp [hlines, Bind.bind, Except.bind] at accepted
  | ok pair =>
    rcases pair with ⟨changed, output⟩
    simp only [hlines, Bind.bind, Except.bind] at accepted
    split at accepted
    · contradiction
    · cases hvalid : validate (String.ofList output) with
      | error failure => simp [hvalid] at accepted
      | ok value =>
        cases value
        simp [hvalid] at accepted
        subst prepared
        exact hvalid

@[extern "tv_cold_plan_result"]
opaque result (success kind : UInt32) (text : @& String) : IO Unit

@[export tv_cold_plan_validate]
def validateNative (text : String) : IO Unit :=
  match validate text with
  | .ok () => result 1 0 ""
  | .error failure => result 0 failure.kind failure.message

@[export tv_cold_plan_prepare]
def prepareNative (text : String) : IO Unit :=
  match prepare text with
  | .ok prepared => result 1 0 prepared
  | .error failure => result 0 failure.kind failure.message

@[export tv_cold_plan_io_unit]
def ioUnit (_ : UInt32) : IO Unit := pure ()

end TV.Hardware.ColdPlan
