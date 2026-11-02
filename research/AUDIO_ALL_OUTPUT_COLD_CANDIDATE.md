# V109 cold audio: the ALL output precedes the speaker

Offline investigation, 2026-11-01. No TV access or runtime changes by this
investigation. This is an evidence-backed candidate, not a confirmed fix.

The coordinator observed a silent cold-native stream despite live PCM queue
movement, a speaker route, disabled mute flags, and the amplifier's register02
equal to0x10. Previous audible direct playback followed Android initialization.

## A difference in the stock startup order

In the exact V109 `audio.primary.mt5867.so` Thumb disassembly:

- `mi_device_init` starts at0x365b4. Its first output-open callback is at0x365f8,
  with Android device mask0x5fffffff prepared at0x365f4.
- The mapping helper at0x36968 returns MI_AOUT path0 for that mask, at0x369b6.
  Speaker mask2 maps to path4 at0x369be. The stock initialization then opens
  speaker, headphones, ARC, SPDIF and HDMI paths as well.
- `mi_device_open` performs GetHandle, Init and Open. It does not issue an extra
  speaker-specific SetAttr after Open. The existing Rust88-byte Open payload
  agrees with its empty name, default table and null table pointer.

The Rust cold owner opened only path4. Path0 is not merely an unused endpoint:
the exact `mik.ko` `MI_AOUT_Open` has path0-specific effects.

- At0x79ca4..0x79cb4, path0 clears `_bIsSettingAllPathType` before setup.
- At0x7a474..0x7a490, successful path0 initialization sets that flag.
- If `_bSetSpeakerConfigByUser` is false, path0 additionally gets system config
  type0x10e at0x7a4d8. The exact libmi DWARF names this
  `E_MI_SYS_CONFIG_PARAM_TYPE_GET_AUDIO_DAP_SPEAKER_CONFIGURATION`.
- The returned speaker-config string can be sent through
  `MApi_AUDIO_SYSTEM_Control` at0x7a734. Other output paths bypass this block.
- Path0 Close clears the global flag at0x7bb54..0x7bb6c. Therefore opening and
  immediately closing path0 is not a neutral startup substitute.

These facts establish a startup difference and a plausible missing DSP speaker
configuration. They do not prove that it explains the silence. Opening path0
after path4 is also not the exact stock ordering.

## Candidate probe

`src/mi-bootstrap/src/bin/all-output-owner-probe.rs` makes one path0 Open with
an88-byte all-zero payload. It first requires the original G06, uid0, exactly
zero app gain, an existing speaker path4 with no input routes, and absent path0.
It never calls INIT, creates PCM, connects routes, writes volume or controls the
amplifier. It retains the file descriptor indefinitely after the Open attempt,
including an error result; SIGTERM/SIGINT do not close it. There is deliberately
no Close because that would reset the global flag.

Build:

```
CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER=rust-lld cargo +1.92.0 build --offline --locked --release --target armv7-unknown-linux-musleabihf --manifest-path src/mi-bootstrap/Cargo.toml --target-dir assets/build/all-output-probe --bin all-output-owner-probe
```

The coordinator should stop playback and preserve zero gain before any trial,
inspect the logged handles, then test bounded PCM through the existing owner.
The helper performs no automatic tone. A fresh boot remains the clean return
from its retained global state.

## Eliminated red herrings

The HAL's SetAttr0x1202 is HDMI capability, not an I2S format control. Audio
attributes0x4009 and0x400e are AC4 dialog gain and DTS DRC. Their presence in the
same initialization function does not justify replaying them for this PCM
speaker failure. Exact attribute names and values are available in libmi's
DWARF enums; guessed setter numbers should not be used.

Evidence files are the existing exact firmware disassembly and generated
`assets/research/audio-agent-direct/cold-aout-init-quick.txt`. No secure payload
or proprietary binary content was copied into this note.
