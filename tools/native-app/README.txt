Native TV application workspace

The boot supervisor remains PID 1 and provides the root console (./repl.sh).
It also owns global volume keys and publishes the volume overlay; the display backend composites that overlay above every application.

The launcher runs as a thread inside our Rust PID 1. Its menu reducer is Lean.
There is no separate tvinit process on the TV. The standalone tvinit binary
uses the same library for the smaller QEMU app harness.
YouTube is a separate ytplay process with its own Search and suggestions screen.
Native builds also install YouTube (Lean) and Doom (Lean) beside the classic
apps for comparison. They share the same app permissions and master volume.
Once the replacements pass playtests, the classic versions can be removed.
Back returns through the app; Home terminates the foreground app and returns
to the launcher. Slides and Doom are separate application processes too.

Build the ARM applications: ./tools/native-app/build.sh
Stage and start the full runtime on the connected G06: python3 tools/native-app/run.py
This uses RAM files and direct Rust display/audio drivers, with no flash.
The current PID1 cold-start candidate initializes hardware directly in Rust;
its remaining vendor process is the traced TEE supplicant. The first cold sequence now displays the launcher on the TV. PCM playback
required an additional persistent speaker-output owner; that owner is
integrated in the next build, with a live helper test awaiting sound confirmation. run.py only deploys applications after PID1
reports that hardware and its media services are ready.
The script validates the connected G06 serial before starting the runtime.

For the recovered G13 / Android 14 TV (serial 18ACC22D1C47):
  nix develop -c adb connect 192.168.77.3:5555
  nix develop -c python3 tools/native-app/run-g13.py --network-only
The installed native startup already runs the launcher. --network-only
restores the host proxy/replay links and clock after a boot, leaving it running.
Ethernet + UART is the standard; see research/G13_DEVELOPMENT.txt.
G13 startup milestones appear as DANDE_BOOT on UART, ending in READY after
display frames and audio readiness. Install/update those startup scripts with
`nix develop -c python3 tools/native-app/g13-boot.py logging` (no reboot).
The supervisor now overlaps audio initialization with launcher preparation;
update an existing G13 with `nix develop -c python3 tools/native-app/g13-boot.py parallel`.
This backs up and verifies the runtime script/manifest without restarting it.
Current-boot timing logs are /dev/dande-boot.log and
/dev/dande-native/early-dmesg.log on the TV; see the development guide for
measured delays and the distinction between software and panel readiness.
The verified Lean startup logo and its rollback commands are documented in
research/G13_UNLOCK_LOGO.txt. Its MBOOT asset write is separate from app builds.

For a manual G13 session after selecting and booting Android fallback:
  python3 tools/native-app/run-g13.py
  python3 tools/native-app/run-g13.py --no-build  # reuse a completed build
  python3 tools/native-app/run-g13.py --stop
This uses adb root, a private chroot, Android-native display/audio helpers,
and ADB networking relays over the selected transport. It builds
.#g13-apps with UIDs 19001, 19002, 19003, 19005 and 19006, checking for Android
package collisions;
the G06 and QEMU UID policy stays unchanged. The manual runner does not flash
or configure autostart, and refuses to replace an active native boot session.
TV_ADB_SERIAL=18ACC22D1C47 selects USB instead of the Ethernet default when
needed in Android fallback, which does not import the native Ethernet service.
If python3 is absent, use `nix shell nixpkgs#python3 -c python3 ...`.
See research/G13_TVINIT_NOTES.txt for verified behavior and the two volume
controls. Home returns to tvinit; its Power action exits back to Android.

For a small change, upload a newly built executable with ./repl.sh --put and a
new temporary filename, stop that app, then rename the executable into place.
Do not overwrite a running executable. POST /launcher/stop before promoting
new files, then POST /launcher/reload to rebuild the menu and resume. PID 1,
its root console and direct display/audio drivers stay alive. Preserve /tmp/app-volume.

Useful checks:
  ./repl.sh -c 'cat /tmp/app-volume'
  python3 tools/pid1/watch.py --seconds 1
  python3 tools/native-app/capture.py --require-opaque --name app-check
  python3 tools/native-app/key.py home

Frame capture checks application pixels and alpha, not physical panel scanout.
Live YouTube requires the host CONNECT proxy on 192.168.77.1:8899; TLS remains
inside ytplay. Local replay uses port 8090. run.py starts both host services.

The platform layer adapts shared RGBA frames and loopback PCM streams
for the TV; QEMU continues to use framebuffer and ALSA devices. Passing in QEMU
does not yet prove vendor driver or Android-service behavior on the TV.

Integrated confinement (d4hines/pid1-demo)
The app launcher now applies the Lean UID/limit/syscall policy before exec.
It owns app remote input and forwards events over a per-launch Unix socket.
The same PID 1 owns global volume, the root console and the app child reaper. The launcher grants group write access to
the shared frame file as a temporary display capability.

QEMU exercises both transports:
  nix run .#pid1 -- --check
  nix run .#pid1 -- --check-tv-bridge
  nix run .#pid1 -- --check-supervisor
The bridge check uses a synthetic PCM peer and a shared-frame adapter;
it does not model Android HAL initialization, amplifier behavior or scanout.

App interface v1
See research/APP_API_V1.md and src/platform/include/tv_app_v1.h. Rust frame
leases and the C-compatible ABI are tested with the current app backends.
Shared frames now use TVF2 and advisory region locks; update the app binaries
and FrameStream together. Mixing an old reader with a new producer is not
supported. The audio wire protocol is unchanged. QEMU needs FILE_LOCKING=y.

Direct Rust backends (2026-11-01)
--------------------------------
PID 1 owns direct MI OSD/MMA presentation and PCM/AOUT playback. Rebuild and RAM-boot the
current REPL image before using the launcher; it checks their service status.
SurfaceFlinger and AudioFlinger do not present app frames or play app PCM.
Earlier physical playback used stock initialization; the new owned cold
initialization now has a physically confirmed launcher display. Its integrated
speaker-output creation still needs the next hardware boot.

Use --prepare-only to build/upload without restarting the UI.
Global software volume is preserved; a missing gain file starts muted.
Staged app-next files are promoted after old applications stop. The display
owner remains alive across app restarts because vendor window destruction is
unreliable; never replace its executable while it is running.

Physical validation: moving Doom with remote controls and audible gunshots;
moving local test video with audio. Those physical tests used the earlier direct framebuffer path. The external
OSD path submitted a 120-frame video with zero drops; its physical confirmation
is pending. Both retain scoped speaker attenuation and Doom input boost.
These tests do not establish live YouTube or cold-boot initialization on this
new backend. No firmware flash was needed for the tests.

G13 automatic startup (2026-11-01, later device work)
----------------------------------------------------
The G13/AU13 startup image and metadata config can boot this launcher without
Android's framework. Stock init remains PID 1 and vendor native services are
retained. See research/G13_NATIVE_BOOT_NOTES.txt for the current verified
configuration, image hash, boot tests and exact rollback prerequisites.

  python3 tools/native-app/build-g13-boot.py --output <new assets directory>
  python3 tools/native-app/g13-boot.py stage <candidate directory>
  python3 tools/native-app/g13-boot.py once
  python3 tools/native-app/g13-boot.py persistent
  python3 tools/native-app/g13-boot.py android

Build is host-only; stage requires the previous working root image. The
selection commands require the matching startup image already installed.
None of these tools flashes or reboots automatically. Do not delete the
metadata directory while the startup image is installed. The Android-hosted
runner must not be started on top of an init-managed native session.
