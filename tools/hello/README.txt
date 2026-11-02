Hello world on the original G06, with stock startup and ADB retained

From this repository root:

  ./hello.sh
  ./hello.sh --seconds 60
  ./hello.sh --stop

Equivalent: nix run .#hello -- --seconds 60.
Build only: nix build .#hello-screen (produces result/hello.jar, not boot.img).
Inspect available display APIs without drawing: ./hello.sh --inspect.

This runs as the existing ADB shell user through app_process32. No APK is
installed, and no firmware, init script or SELinux policy is changed. Stock
Android retains its normal display setup and ADB. The runner checks the
G06 serial, V109 version and boot_completed=1 before running anything.

The helper creates a shell-owned SurfaceFlinger layer, draws a bitmap
"Hello world" at 1280x720, and removes it on normal completion. The default
duration is 20 seconds (1..120 allowed). A separate daemon thread exits the
helper after duration+10 seconds if its main thread gets stuck. --stop can
be run from another terminal; it only signals PIDs currently named tv.hello.
Binder ownership also releases the layer when the helper process dies.

Evidence goes under assets/research/hello/<host-UTC-timestamp>/:
  session.json              target, helper hash and requested duration
  stdout.txt                live startup/display/cleanup phases and errors
  logcat.txt                only the helper's TVHello tag, without clearing logs
  adb-while-visible.txt     independent ADB health/identity check during display
  screen.png                TV screenshot taken while the layer is visible

Files left on the TV: /data/local/tmp/tv-hello/hello.jar (shell-owned, mode0600
inside a mode0700 directory). No helper remains running after timeout/stop.
The program does not grab input or disable Android services. Recovery and
normal adb reboot bootloader remain available through the existing system.

Measured on 2026-10-31:
- The first helper reached surface creation but aborted in Android's native
  font initialization. ADB and Android remained available, and the helper
  exited. Logs captured the assertion.
- Replacing framework text rendering with bitmap rectangles succeeded.
  TV screenshot assets/research/hello/20261031T222119.029583Z/screen.png shows
  Hello world. An independent USB ADB call succeeded while the layer was
  visible as uid2000(shell). The 30-second timeout logged phase=closed;
  the helper and layer were absent afterward, with boot_completed=1.
- The independent --stop command was tested on a second run: both helper
  and layer disappeared, and ADB still returned boot_completed=1. Evidence:
  assets/research/hello/stop-check/result.json. The interrupted display
  runner reports a nonzero exit; the separate stop command succeeds.
- This proves drawing through the running Android compositor. It does not
  prove cold panel setup or framebuffer access without vendor userspace.

Physical panel confirmation: the owner watched the 60-second rerun at
2026-10-31T22:23:24Z and confirmed "I see it!!!!". The independent ADB check
passed while visible. Evidence: assets/research/hello/20261031T222324.451726Z/.

Java compilation uses pinned D8 8.3.37 and the Nix-pinned JDK. The helper
uses reflection so it needs no downloaded Android SDK or app-signing key.
Firmware experiments remain separate from this fast ADB iteration loop.

Root-shell iteration was also verified on 2026-11-01T03:21Z. ./shell.sh
opened an interactive uid0 prompt; launching this same helper from root
drew Hello in a captured TV screenshot. Concurrent root commands worked.
After the overlay closed, the boot ID was unchanged and root stayed live.
Evidence: assets/research/android-root-shell/20261101T032149Z/.
