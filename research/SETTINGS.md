# Settings owned by the supervisor

Settings is a system screen inside the resident Rust supervisor, not a child
application with its own privilege. The five app identities are YouTube=1,
Doom=2, Slides=3, Infinite RPG=5, City flight=6. ID 4 is retired and never launchable. Settings itself cannot be disabled.

The parental policy is authored in `src/lean/TV/Settings.lean` and called through
scalar C ABI exports. The checked theorems state that a wrong PIN cannot change
the PIN or re-enable an app; invalid replacement PINs cannot replace it; the
correct PIN can replace it; changes preserve the four-digit bound; and disabling
one of the five apps removes that app's access. `TV/Proofs.lean` is included in
the Lean check. These are policy proofs, not proofs of Linux, disk durability,
UI rendering, native calls or arbitrary privileged shell commands.

The default PIN is 0000. Changing it requests current PIN, new PIN, and matching
confirmation. Disabled apps remain visible and refuse launch at the supervisor's
launch gate. Re-enabling requires the current PIN. The UI displays compiled
UID/network/thread capabilities from `TV/Apps.lean`, not a duplicated text table.

The original G06 uses only its validated 100 MiB cache partition for persistence:
mmcblk0p29, start sector 9019392, with matching device identity and ext4 geometry.
It is mounted at `/settings` with nosuid,nodev,noexec. Only that partition's
read-only block flag is cleared; there is no format, raw partition write, boot
image update, or access to userdata/RPMB. Ordinary files live in root-only
`/settings/dande-tv`. A factory reset or cache wipe can reset these development
settings. The G13 supervisor binds its `/metadata/dande-tv/settings` directory
at the same location. QEMU uses `/tmp/dande-tv-settings`; those QEMU settings
are intentionally volatile across full VM restarts.

Files use a fresh exclusive temporary file, file fsync, atomic rename and parent
directory fsync. PIN state has mode0600, a single link, bounded length, canonical
four-digit syntax, and a bounded app mask. Invalid or inaccessible state refuses
app launches rather than silently restoring defaults. Missing state selects the
initial default. The persistence implementation is trusted native code; physical
cache mounting and persistence still require coordinator hardware validation.

Wi-Fi shows current connection status without scanning or joining on entry.
Explicit Scan lists open and WPA/WPA2-PSK networks. WEP, enterprise, SAE-only and
escaped/binary SSIDs are omitted rather than misrepresented. The remote-driven
password keyboard supports up to63 printable characters; Red connects, Green
removes a character, Back cancels. The worker retains Home responsiveness. One
process-wide guard prevents concurrent operations after reopening Settings.
Credentials go only into mode0600 config files, never command-line arguments or
logs. Disconnect stops the identified wlan0 DHCP owner and removes only wlan0
addresses. The wired debug subnet/route is preserved. G13 Wi-Fi preparation has
not been qualified; the current preparation script is G06-specific.

Validation completed offline:

- `lake build TV.Settings TV.Proofs` passed.
- Twelve Rust tests passed as an ARM32 binary in QEMU, covering persistent policy,
  wrong/correct PINs, confirmation mismatch, disabling/re-enabling, Home/Back,
  corrupt/link state rejection, safe file replacement, and Wi-Fi config parsing.
  Evidence: `assets/build/settings-tests/qemu/serial.log`.
- The graphical ARM32 supervisor showed the Settings menu and app controls,
  rejected a wrong re-enable PIN, re-enabled using 0000, rejected mismatched
  confirmation, and persisted a changed PIN to the QEMU file. Screenshots:
  `assets/build/ui-audit/settings-finish/shots/`.

A repeatable graphical regression is `python3 tools/settings/check-ui.py --apps
assets/build/native-app-target/armv7-unknown-linux-musleabihf/release`. Build the
current native PID1 first with `python3 tools/pid1/build-repl.py`. The test uses
only local QEMU/fixture services, never the TV. It also checks a disabled app's
actual launch refusal and fail-closed behavior after corrupting only the VM's
settings file.

Final combined-image graphical regression passed all nine cases, including
actual disabled-app launch refusal, old-PIN rejection after a change, read-only
Wi-Fi entry, and corrupt-state launch refusal:
`assets/build/ui-audit/settings-check-1793562486003068000/result.json`.
The proof/capability page and inactive Wi-Fi page were visually inspected at
1280x720 with no clipped text. Two subsequent small UI fixes clear obsolete PIN
error notices on navigation and retry a deferred read-only Wi-Fi status request
after a prior page's connection operation completes; they do not change policy.

The saved app mask accepts six bits so existing files remain readable. A
missing settings file enables the five installed apps (mask 55). Retired ID 4
is denied even when its old bit is set; existing masks otherwise retain their
state. Re-enable an installed app with the current PIN if its bit is absent. Existing masks do not automatically enable City flight.
The QEMU demo and G06 native uploader stage all five apps. The native uploader
starts the pinned RPG server on the Mac at 192.168.77.1:8080 and stages its
converted scene pictures. The G13 runner retains its separately qualified app
set; its RPG hosting has not been qualified.

G06 hardware: the Settings screen opened on the native supervisor; the exact
cache filesystem mounted at /settings with nosuid,nodev,noexec, and its private
directory had root ownership and mode0700. A temporary ordinary settings file
was written, synced, read back and removed successfully. This does not replace
the pending user PIN/persistence and Wi-Fi playtests.
