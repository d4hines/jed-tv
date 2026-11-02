# Original G06 / V109 AD82111 amplifier replacement

An owned Rust adapter now replaces the amplifier-specific vendor calls at the
register/GPIO level: `src/mi-bootstrap/src/amplifier.rs`. It uses MI kernel
ioctls directly and contains no `dlopen`, `libfpp`, `sitatvservice` or Android
service calls. It requires existing initialized IIC/GPIO handles and exclusive
PID-1 ownership. It has not been run on the TV.

The next hardware step is **read-only** `mi-bootstrap amp-status`: retrieve the
existing MI IIC handle for port9 and GPIO handle for pad30. Unlike PCM GetHandle,
these two exact kernel GetHandle implementations only search existing records;
their full disassemblies contain no allocation or open calls. The probe performs
no GPIO level changes and no I2C transaction. All hardware remains coordinated
by the root agent; no TV calls were made in this investigation.

## Exact board selection

The existing stock capture selected project17849, HID185. Its active model is
`/tclconfig/hardware/HID_32_185_2K_NA_S5400A.ini`, JSON despite the extension:

```
AMP_CHIPS[0].AMP_1.TYPE = AD82111
AMP_CHIPS[0].AMP_1.IIC = 1
AMP_CHIPS[0].AMP_1.ADDR = 96 (0x60)
AMP_CHIPS[0].AMP_1.EQ = /tclconfig/hardware/param/sound/EQ/PEQ__12V_6o_8W_32S65A_S356.ini
```

The vendor-format address0x60 is passed unchanged to MI IIC. Do not convert it
to0x30 unless switching to a different I2C API with explicitly different address
conventions. `board.ini` identifies the audio IIC device0x465 as MI port9
(`E_MI_IIC_PORT1_1`) and slave0x60. FPP bus1 selects its first hardware bus
handle; its format conversion does not shift the slave address.

The physical power-down/reset line is **pad30, invert0**. This is not the
misleading legacy `GPIO_AUDIO_AMP_RESET` entry (pad26, invert1):

```
fpi logical GPIO19
  -> sitatvservice fallback table entry18 = FPP function0x2003
  -> libfpp fpp_gpio_set adds0x3c0
  -> board function0x23c3 (E_MIF_GPIO_EXTERNAL_NAME_AMP_PD)
  -> BOARD_GPIO_20_PAD_NO=30, BOARD_GPIO_20_INVERTOR=0
```

`fpi_amp_reset_pd` (`sitatvservice`0x6e0a2c) configures logical19 as output,
sets it low, sleeps6000 microseconds, sets it high, then sleeps20000
microseconds. Optional logical20 is conditional on another configured GPIO;
the selected HID has one amplifier and no JSON GPIO declaration. The adapter
implements the proven primary AMP_PD route only.

There is a further encoding trap: FPP converts its boolean output to
`MI_GPIO_LEVEL_ON=0` / `MI_GPIO_LEVEL_OFF=1`. With invert0, MI level0 calls
`mdrv_gpio_set_high` and level1 calls `mdrv_gpio_set_low`. The Rust adapter
therefore sends1 for physical low and0 for physical high. It does not pass a
Rust boolean directly as the MI level.

## Exact register sequence

`sitatvservice::bsp_amp_ad82111_init` at0xa22b54 writes78 register/value pairs
from its table at virtual address0x11967d1. It sleeps8ms after pair0 and20ms
after pair1. Each write has one register byte and one value byte. It then calls
`bsp_amp_ad82111_parse_data`, which applies:

1. The selected EQ file's `DRC_TABLE.data`:17 register/value pairs.
2. `VOL_GAIN_DATA.data1`: two pairs, register04=17 and05=17 (hex).
3. `VOL_GAIN_DATA.data` is retained for later volume operations, not immediately
   emitted by this init routine.

The dedicated mute routine reads two-byte data symbols:

| Operation | Register | Value | Delay after write |
| --- | --- | --- | --- |
| Mute (`sound_off_tb82111`) | 0x02 | 0x1e | 2ms |
| Unmute (`sound_on_tb82111`) | 0x02 | 0x10 | 2ms |

The Rust sequence follows those init writes and then explicitly emits the
known mute command. Unmute is a separate method, rejected unless initialization
completed. An IIC failure stops the sequence and prevents unmute/retry on that
object. Physical loudness is not set here: the caller must keep PCM stopped
and establish a quiet bounded software gain before a later explicit unmute.

Register data stays in ignored firmware assets. Reproduce the small plan with:

```
PYTHONDONTWRITEBYTECODE=1 python3 tools/midaemon/prepare-amplifier.py
```

The extractor pins the firmware, active HID and selected EQ hashes and produces
`assets/research/midaemon-replacement/amplifier/ad82111.plan` plus a manifest.
The firmware SHA256 is
`2a561d62ec29eff19ad9c47921831b5437faf581f4f16b6877a96850dcca930b`.
The plan SHA256 is
`1ad71cd9a32912430b9cc1c2564338009a96c9fa279c772f86d972d106b7091e`.
The plan parser validates its route, version and table dimensions; the staging
caller must also check the manifest hash. It does not authenticate arbitrary
untrusted plan files itself.

## Direct MI ABI

All requests retain the mandatory16-byte outer envelope from the display/PCM
analysis: u16 inner bytes, six zero bytes, u64 inner address.

| Request | Inner layout |
| --- | --- |
| IIC GetHandle0xc0081d04 | 8 bytes: i32 handle_out@0, u32 MI port@4 |
| GPIO GetHandle0xc0082904 | 8 bytes: i32 handle_out@0, u16 pad@4, zero pad@6 |
| GPIO SetInvert0xc0082905 | 8 bytes: i32 handle@0, u8 invert@4, zero pad3 |
| GPIO SetOutput0xc0082907 | 8 bytes: i32 handle@0, u32 MI level@4 |
| GPIO GetInput0xc00c2908 | 12 bytes: i32 handle@0, u32 MI level_out@4, u8 invert_out@8, zero pad3 |
| IIC Write0xc0201d06 | 32 bytes: i32 handle@0,u16 bus/slave@4,u8 address_count@6,pad@7,u64 register_ptr@8,u16 data_count@16,pad6@18,u64 data_ptr@24 |
| IIC Read0xc0281d05 | 40 bytes: same first32 bytes as Write, u32 read_mode@32, zero pad4@36 |

The adapter uses address_count1, data_count1, slave0x60. Every pointer remains
live across the synchronous ioctl. The low-level adapter does not initialize
IIC/GPIO, open new MI logical handles or guess which bus is available. Missing
handles are an explicit error. Its unsafe adoption entry point checks root
PID1 and original TV identity, then requires the caller to guarantee that the
amplifier is exclusively owned and audio is quiescent. There is no CLI mutation
mode.

The bounded `amp-read-status` CLI performs the two existing-handle lookups,
then GPIO GetInput and one IIC Read of register0x02. It uses read_mode0
(`E_MI_IIC_READ_MODE_DEFAULT`), matching the stock `mif_iic_read` implementation
at Thumb0x22a34c. Its address_count and data_count are both1; the register and
output byte remain live during the synchronous ioctl. It does not write a
new register value, reset the amplifier, set GPIO direction/inversion, change
bus speed, initialize a module, or retry. The IIC transaction does select
register0x02 in its address phase. The GPIO kernel getter at0x194bcc only reads
the inversion mapping and physical input; decoding is
`physical_high = (MI_level == 0) XOR invert`. The new read is compiled and
offline-tested, but not yet hardware-validated. Known stock mute commands are
0x1e (mute) and0x10 (unmute); no undocumented bit meanings are assumed.

## Evidence and remaining validation

Generated disassemblies and type extracts are under
`assets/research/midaemon-replacement/amplifier/`:

* `ad82111.txt`, `amp-reset.txt`, `gpio-set.txt`: register loop and reset calls.
* `fpp-to-mif-iic.txt`, `mif-iic-write-thumb.txt`, `iic-api.txt`, `iic-types.txt`:
  bus/address conversion and exact kernel payload.
* `mif-gpio-init-thumb.txt`, `fpp-gpio-set-thumb.txt`, `gpio-api.txt`,
  `gpio-setoutput-kernel.txt`: board mapping and inverted MI boolean encoding.
* `gpio-gethandle-kernel.txt` and `iic-gethandle-kernel.txt`: audited lookup-only
  queries, unlike the allocating PCM getter discovered during this session.
* `gpio-getinput-kernel.txt`: input sampling and the MI level/inversion encoding.

Offline tests cover reset/register delay ordering, failure blocking unmute and
retry, rejecting the wrong pad/inversion route, and MI level encoding. The
crate builds as static ARM32 with no new dependency. The GPIO/IIC transport,
end-to-end muted initialization, audible output, and removal of middleware
remain hardware validation gates.

When hardware work resumes: run the lookup-only status probe; then during an
explicit quiet, exclusive test initialize the exact plan while PCM is stopped
and stay muted. A later bounded jingle/unmute test establishes equivalence.
Do not stop or unload vendor services as part of the first register trial.
The final cold-start path must separately provide initialized GPIO/IIC modules
and logical handles; this adapter intentionally avoids reinitializing a warm
shared subsystem based on misleading metadata bits.
