/-!
`zr-qemu32 BINARY [TIMEOUT_SECONDS] [-- QEMU_ARGS...]`: run a static 32-bit
ARM Linux binary as PID 1 in QEMU and print its console.

The repo's virt32 profile: qemu-system-aarch64 with an AArch32 Cortex-A55
and the surrogate Linux 6.18 zImage (assets/build/kernel/arm/zImage), the
binary as `/init` of a newc cpio initramfs written here (with /dev,
/dev/console, /proc and /sys; device nodes go straight into the archive, so
no root is needed). The binary should power the machine off when done;
otherwise QEMU is stopped at the timeout (default 120 s). QEMU_ARGS are
added to the command line (e.g. `-icount shift=0`, `-plugin ...`). Host
tool (upstream Lean).
-/

def hex8 (v : Nat) : String :=
  String.ofList ((List.range 8).reverse.map fun i =>
    let d := (v >>> (4 * i)) % 16
    if d < 10 then Char.ofNat (48 + d) else Char.ofNat (55 + d))

def pad4 (n : Nat) : Nat := (4 - n % 4) % 4

/-- One newc cpio entry (inode `ino`). -/
def entry (ino : Nat) (name : String) (mode : Nat) (data : ByteArray := .empty) (rdev : Nat × Nat := (0, 0)) :
    ByteArray :=
  let hdr := "070701" ++ String.join ([ino, mode, 0, 0, 1, 0, data.size, 0, 0, rdev.1, rdev.2,
    name.utf8ByteSize + 1, 0].map hex8)
  let h := (hdr ++ name).toUTF8.push 0
  let h := h ++ ⟨Array.replicate (pad4 h.size) 0⟩
  h ++ data ++ ⟨Array.replicate (pad4 data.size) 0⟩

/-- The initramfs holding `init`. -/
def initramfs (init : ByteArray) : ByteArray :=
  let blob := entry 2 "dev" 0o40755 ++ entry 3 "dev/console" 0o20600 .empty (5, 1) ++
    entry 4 "proc" 0o40755 ++ entry 5 "sys" 0o40755 ++ entry 6 "init" 0o100755 init ++
    entry 7 "TRAILER!!!" 0
  blob ++ ⟨Array.replicate ((512 - blob.size % 512) % 512) 0⟩

def main (args : List String) : IO UInt32 := do
  let (ours, extra) := match args.span (· != "--") with
    | (a, _ :: rest) => (a, rest)
    | (a, []) => (a, [])
  let binary :: rest := ours | IO.eprintln "usage: zr-qemu32 BINARY [TIMEOUT] [-- QEMU_ARGS...]"; return 2
  let timeout := (rest.headD "120").toNat!
  -- The repo root: the executable is tools/lean/zr/.lake/build/bin/zr-qemu32.
  let root ← IO.FS.realPath ((← IO.appDir) / ".." / ".." / ".." / ".." / ".." / "..")
  let kernel := root / "assets/build/kernel/arm/zImage"
  let initrd := System.FilePath.mk s!"{binary}.initrd.cpio"
  IO.FS.writeBinFile initrd (initramfs (← IO.FS.readBinFile binary))
  let child ← IO.Process.spawn {
    cmd := "qemu-system-aarch64", stdin := .null, stdout := .piped, stderr := .piped,
    args := #["-M", "virt-11.1,highmem=off,gic-version=2", "-cpu", "cortex-a55,aarch64=off", "-m", "1024",
      "-nographic", "-no-reboot", "-kernel", kernel.toString, "-initrd", initrd.toString,
      "-append", "console=ttyAMA0 rdinit=/init quiet loglevel=0 panic=-1"] ++ extra.toArray }
  -- the outputs are read by their own tasks (the console can exceed a pipe's
  -- buffer); the main thread polls for exit until the deadline
  let out ← IO.asTask (prio := .dedicated) child.stdout.readToEnd
  let err ← IO.asTask (prio := .dedicated) child.stderr.readToEnd
  let deadline := (← IO.monoMsNow) + timeout * 1000
  let mut timedOut := false
  repeat
    if (← child.tryWait).isSome then break
    if (← IO.monoMsNow) > deadline then
      child.kill
      let _ ← child.wait
      timedOut := true
      break
    IO.sleep 20
  let out ← IO.ofExcept (← IO.wait out)
  let err ← IO.ofExcept (← IO.wait err)
  IO.FS.removeFile initrd
  let text := (out ++ err).replace "\r\n" "\n"
  IO.print text
  if timedOut then IO.println "\n[qemu32: timeout]"
  return 0
