Production G06 audio uses the cooperative Lean protocol, converter and device
ownership policy under src/pid1/lean/TV/Audio. Individual MI operations live in
src/pid1/audio/device-resources.rs, aout-route.rs and mi-pcm.rs.

The retired Rust direct controller and converter are retained under
research/probes/audio for standalone diagnostics and differential test oracles.
Diagnostic build: tools/audio/build-direct.sh
Diagnostic checks: tools/audio/check-direct.sh
Production adapter check: test/lean/tv/audio/native/README.md

The Rust protocol remains in src/pid1/audio/protocol.rs because the active G13
AAudio bridge includes it. It is not used by the G06 cooperative Lean executor.

The following is historical investigation, not current startup instructions.

G06 native audio investigation
==============================
Native graphics and Doom remain proven. Stock Android sound is physically confirmed.
A one-second rising jingle at media volume 30/100 and 30% PCM amplitude was heard
by the owner; stock media volume was restored to 10/100 afterward.
Native sound was physically confirmed after explicit amplifier unmute. The first
native 30% PCM jingle was too loud; a 3% PCM jingle was confirmed comfortable.
Do not repeat the 30% native test. Doom effects now use software PCM scaling with a 3% peak ceiling.

Historical initial tests:
No physical native sound had been heard yet.
Two-second 440 Hz AudioTrack tones at 10% and 30% PCM amplitude were submitted;
after the scheduling shim, playback head advanced by about 96000 frames.
Owner still heard silence at 30%. Do not increase volume without asking.

Stock audio children used:
/vendor/bin/hw/mstar.hardware.audio.service
/system/bin/audioserver
/system/bin/mediametrics
/system/bin/hw/android.system.suspend@1.0-service

Audio HAL LD_LIBRARY_PATH:
/vendor/lib:/apex/com.android.vndk.v30/lib:/system/lib:/apex/com.android.i18n/lib:/apex/com.android.runtime/lib
Audioserver LD_LIBRARY_PATH:
/system/lib:/apex/com.android.os.statsd/lib:/apex/com.android.i18n/lib:/vendor/lib:/apex/com.android.vndk.v30/lib
Audioserver LD_PRELOAD=/tmp/standalone-app-state.so.

The shim avoids ActivityManager UID observer, sensor-privacy listener,
PermissionController package enumeration, native package UID metadata, and
scheduling-policy waits. Sensor privacy remains enabled. Priority promotion
returns ENOSYS instead of waiting forever. Without these absent framework
services, audioserver blocks or its TimeCheck watchdog aborts it. The last
successful tone used research/hello/AudioTone.java, player bookkeeping via an in-process
IAudioService proxy, real native AudioTrack/AudioFlinger/HAL, 48000 Hz stereo.

The missing hardware step was physical amplifier startup and explicit unmute. Stock board.ini specifies
BOARD_AUDIO_AMP_DEVICE_ID=0 with comment TCL MW init AMP. Its amp INI specifies
DEVICE_TOTAL_NUMBER=0 with AMP move to TCL MW. The separate middleware lives
under /stock-vendor/tvos (alias /vendor/tvos), especially bin/sitatvservice.
It exports fpi_audio_amplifier_init at 0x6f4498 (ARM), config_init at 0x3b1130,
config_get_str (alias json_config_get) at 0x3b110c. The amplifier function reads
model.Drives.AMP_CHIPS. Do not call with an unverified model configuration.

middleware-probe successfully dlopens the unchanged stock executable as a
library. config_init() succeeds with F1_CONFIG=/tclconfig/json_sys.ini.
main_sys_config_init() is a LEGACY INI path and tried parsing that JSON as INI;
it should NOT be part of the JSON initialization sequence. It also invalidated
the subsequent configuration probe. Array nodes may return NULL with length=1; query the scalar
model.Drives.AMP_CHIPS.AMP_1.TYPE instead. It returns AD82111 after mounting
/dev/mmcblk0p24 at /impdata read-only with noload. This supplies project ID 17849
and HID 185. The correct EQ file is selected from that model configuration.

The helper now calls mif_handle_init, mif_system_init, mif_gpio_init and
mif_iic_init from stock libfpp before fpi_gpio_init and fpi_audio_amplifier_init(0).
Without the handle/system setup, MI_GPIO_Init reports uninitialized handles and
the amplifier reset fails. With it, GPIO and I2C return zero and stock logs say
fpi_amp_reset_pd is ok. The JSON no-GPIO-config warning occurs on stock too: this
board uses the lower-level legacy GPIO mapping. No invented GPIO numbers used. config_get_str really returns a pointer and takes
an int* output count/type; a key without a dot crashes its parser (strchr result
is blindly subtracted). A previous bad probe crashed only its own process.

Read-only extra mounts inspected: p17 linux_rootfs_a (squashfs), p18 3rd_a
(squashfs), p20 oem (ext4 ro,noload). Nothing useful for the amplifier there.
All eMMC remains read-only; userdata was never mounted. Shell directories
/userdata /cache /persist are RAM directories. /impdata now mounts stock p24
read-only/noload for the project ID; no configuration files were changed.

Safe native test: AudioTone 3 jingle, one second maximum. The native Java
helper explicitly initializes stream 3 to range 0..100 and sets speaker index 30
through AudioSystem before playback; it stops without playing if that fails.
This setting alone does NOT reproduce stock loudness. Explicit PCM/track gain
reduction to 3% is necessary in the current minimal environment.
This firmware exports public setStreamVolumeIndexAS and private
setStreamVolumeIndex. The reflection lookup accepts the three-int signature.

Earlier amplifier evidence: assets/research/audio-native/jingle-native-amp-initialized.log
and jingle-native-test-2.log. The latter reports 48000 playback frames but was inaudible.
The missing final step was fpi_amp_mute(0), which directly writes the AD82111
unmute register. fpi_set_mutes_for_device alone skips this before the middleware
audio-initialized flag is set. Owner confirmed jingle-explicit-unmute.log was
audible (too loud); jingle-native-quiet.log at 3% was comfortable. TV remains in native PID 1 with
root REPL and graphics/audio services; no firmware flash was needed.

Reproduce the stock baseline (only when already in Android):
  python3 tools/audio/stock-tone.py --percent 30 --jingle
This does NOT change Android media volume. The confirmed audible baseline used
media volume 30/100; save/restore volume separately and verify with dumpsys audio.

Native audio startup from an existing graphics session (no tone or reboot):
  python3 tools/audio/start-native.py
Start or stop Doom with quiet sound effects:
  python3 tools/legacy-doom/run-native.py --sound
  python3 tools/legacy-doom/run-native.py --stop
The runner initializes audio when --sound is requested. It uses a 48 kHz stereo
PCM FIFO between the static ARM engine and research/hello/AudioPcm.java. The Java consumer
sets track gain to unity and scales signed PCM in software, with input boost 4
and a hard output ceiling of 983/32767 (3%). This makes Doom effects audible
without exceeding the approved quiet signal ceiling. Music remains disabled. Freedoom's DS sound lumps are kept
in the separate sound WAD; the original silent WAD and engine remain available.

PHYSICALLY CONFIRMED at the hotel on 2026-11-01 UTC: the owner heard the
streaming jingle, then the one-second two-pistol-shot test, then confirmed actual
Doom gameplay sound ("dude it's working!"). Native PID 1, root shell, graphics,
remote controls, and Doom sound effects are all working together.

The earlier implementation applied 3% AudioTrack gain to a relatively quiet
Doom mix. Its PCM was nonzero, but the owner heard no game effects. The working
version applies explicit PCM scaling, boosts the game input fourfold, and caps
the result to 3%. We changed both scaling location and gain, so we have not
isolated their individual contributions.

Working evidence: hotel-audio-start.log, hotel-stream-jingle.log,
hotel-pistol-test.log, hotel-doom-launch.log, and hotel-doom-live.log beneath
assets/research/audio-native/. The live game reported 35 fps and 35 tics/s;
input peak 8192, output peak 983.

To reproduce from the installed stock-plus-loader Android session:
  python3 tools/pid1/boot-repl.py --execute --serial 192.168.77.2:5555
  python3 tools/pid1/graphics.py --start --seconds 1
  python3 tools/legacy-doom/run-native.py --sound
Use the USB serial instead if USB ADB is connected. The boot script always checks
the actual G06 serial/build separately from the selected ADB transport. If the
TV returns to standby during reboot, press Power once. No new flash is required.

Evidence, extracted firmware and generated artifacts: assets/research/audio-native/.

Music follow-up: E1M1 music is now physically confirmed audible too.
  python3 tools/legacy-doom/music.py
  python3 tools/legacy-doom/music.py --stop
This renders the original Freedoom E1M1 MIDI with FluidSynth and GeneralUser GS
on the laptop, uploads stereo PCM in bounded chunks, and loops it in a separate
AudioTrack on the TV. It does not implement the engine's MIDI backend or follow
menu/pause/map changes. The music PCM ceiling is 0.75%, below the effect ceiling.
The existing Doom process keeps running. run-native.py --stop also stops music.
Evidence: assets/research/audio-native/music-render.log and music-launch.log;
owner confirmed hearing the music.
