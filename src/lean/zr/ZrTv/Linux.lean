/-
Linux system calls for Lean apps on our runtime (32-bit ARM). ZrLow style:
scalars and raw object addresses, explicit reference counts
(`zr-cgen check` checks this module like the runtime).

Results follow the kernel's convention: a "raw" result `r` is an error iff
`r > SIZE_MAX - 4096`, and then `0 - r` is the errno. Calls that produce a
byte array return `Except Nat ByteArray` (error: errno). Buffers are passed
as borrowed byte arrays, so the caller keeps them alive across the call.
-/
import ZrLean.Low.Arr
import ZrLean.Low.HeaderSupport
import ZrLean.Low.Str

set_option compiler.extract_closed false

namespace ZrTv.Linux
open ZrLean.Low ZrLean.Low.Obj ZrLean.Low.Rc

/-- The system call `nr` with six word arguments; the kernel's raw result. -/
@[extern "zr_sys6"]
opaque sys6 (nr a0 a1 a2 a3 a4 a5 : USize) : BaseIO USize

@[inline] def isErr (r : USize) : Bool := r > sizeMax 0 - 4096

/-- `Except.error (errno : Nat)` / `Except.ok obj`, built by hand. -/
def except (r obj : USize) : BaseIO USize := do
  if isErr r then
    dec obj
    let e ← HeaderObj.allocCtor 0 1 0
    stp (e + ctorObjs 0) (((0 - r) <<< 1) ||| 1)
    pure e
  else
    let e ← HeaderObj.allocCtor 1 1 0
    stp (e + ctorObjs 0) obj
    pure e

/-- A system call with four scalar arguments. -/
@[export zr_tv_sys]
def sys (nr a0 a1 a2 a3 : USize) : BaseIO USize := sys6 nr a0 a1 a2 a3 0 0

/-- openat(AT_FDCWD, path, flags, mode) with `path` a Lean string
(borrowed; its bytes are NUL-terminated): a descriptor, or a raw error. -/
@[export zr_tv_open]
def openPath (path flags mode : USize) : BaseIO USize :=
  sys6 322 (0 - 100) (path + strData 0) flags mode 0 0

/-- read(2) of at most `n` bytes into a fresh array. -/
@[export zr_tv_read]
def read (fd n : USize) : BaseIO USize := do
  let b ← allocSArray 1 0 n
  let r ← sys6 3 fd (b + sarrData 0) n 0 0 0
  if !isErr r then stp (b + sarrSizeOff 0) r
  except r b

/-- write(2) of bytes `off .. off + len` of `b` (borrowed, checked). -/
@[export zr_tv_write]
def write (fd b off len : USize) : BaseIO USize := do
  let sz ← ldp (b + sarrSizeOff 0)
  if off > sz || len > sz - off then pure (0 - 22)
  else sys6 4 fd (b + sarrData 0 + off) len 0 0 0

/-- connect(2) with the socket address in `sa` (borrowed). -/
@[export zr_tv_connect]
def connect (fd sa : USize) : BaseIO USize := do
  sys6 283 fd (sa + sarrData 0) (← ldp (sa + sarrSizeOff 0)) 0 0 0

/-- setsockopt(2) with an int value. -/
@[export zr_tv_setsockopt_int]
def setsockoptInt (fd level name value : USize) : BaseIO USize := do
  let cell ← allocSArray 1 4 4
  st32 (cell + sarrData 0) value.toUInt32
  let r ← sys6 294 fd level name (cell + sarrData 0) 4 0
  dec cell
  pure r

/-- getsockopt(2) of an int option: the value, or a raw error. -/
@[export zr_tv_getsockopt_int]
def getsockoptInt (fd level name : USize) : BaseIO USize := do
  let cell ← allocSArray 1 8 8
  st32 (cell + sarrData 0) 0
  st32 (cell + sarrData 0 + 4) 4
  let r ← sys6 295 fd level name (cell + sarrData 0) (cell + sarrData 0 + 4) 0
  let v ← ld32 (cell + sarrData 0)
  dec cell
  pure (if isErr r then r else v.toUSize)

/-- poll(2) over `fds` (owned: `struct pollfd` records, 8 bytes each); the
array comes back with `revents` filled in. -/
@[export zr_tv_poll]
def poll (fds timeoutMs : USize) : BaseIO USize := do
  let a ← Arr.sarrayEnsureExclusive fds
  let n := (← ldp (a + sarrSizeOff 0)) / 8
  let r ← sys6 168 (a + sarrData 0) n timeoutMs 0 0 0
  except r a

/-- getrandom(2): `n` random bytes (blocking until the pool is ready). -/
@[export zr_tv_getrandom]
def getrandom (n : USize) : BaseIO USize := do
  let b ← allocSArray 1 n n
  let r ← sys6 384 (b + sarrData 0) n 0 0 0 0
  except (if !isErr r && r != n then 0 - 5 else r) b

/-- clock_gettime(2) of `clock` (0 realtime, 1 monotonic) in nanoseconds. -/
@[export zr_tv_clock_ns]
def clockNs (clock : USize) : BaseIO UInt64 := do
  let ts ← allocSArray 1 16 16
  let r ← sys6 263 clock (ts + sarrData 0) 0 0 0 0
  let s ← ldp (ts + sarrData 0)
  let ns ← ldp (ts + sarrData 0 + wordBytes 0)
  dec ts
  pure (if isErr r then 0 else s.toUInt64 * 1000000000 + ns.toUInt64)

/-! ### Program arguments and environment (recorded by the ZrAsm.Linux app entry) -/

@[never_extract, extern "zr_tv_args_addr"]
opaque argsAddr (z : USize) : USize

partial def strlen (p n : USize) : BaseIO USize := do
  if (← ld8 (p + n)) == 0 then pure n else strlen p (n + 1)

/-- A Lean string copied from the C string at `p` (empty unless valid UTF-8). -/
def cString (p : USize) : BaseIO USize := do
  let n ← if p == 0 then pure 0 else strlen p 0
  let ok ← if n == 0 then pure 1 else Str.validate p 0 n
  let n := if ok != 0 then n else 0
  let s ← HeaderSupport.allocString (n + 1) (n + 1) (← Str.utf8Len p 0 n 0)
  copy (s + strData 0) p n
  st8 (s + strData 0 + n) 0
  pure s

/-- The `i`th entry of the NULL-terminated pointer table at `t`, or 0 when
the table ends before it. -/
partial def entry (t i k : USize) : BaseIO USize := do
  let p ← ldp (t + k * wordBytes 0)
  if p == 0 || k == i then pure p else entry t i (k + 1)

/-- Program argument `i` (`""` past the end). -/
@[export zr_tv_arg]
def arg (i : USize) : BaseIO USize := do
  let a := argsAddr 0
  if i < (← ldp a) then cString (← entry (← ldp (a + wordBytes 0)) i 0) else cString 0

/-- Environment entry `i` ("KEY=value"; `""` past the end). -/
@[export zr_tv_env]
def env (i : USize) : BaseIO USize := do
  let t ← ldp (argsAddr 0 + 2 * wordBytes 0)
  if t == 0 then cString 0 else cString (← entry t i 0)

end ZrTv.Linux
