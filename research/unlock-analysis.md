# T221T04 MBOOT unlock analysis

Offline analysis, 2026-10-30. No TV commands were issued for this analysis.

The analyzed binary is `assets/research/firmware/V8-T221T04-LF1V122/mboot.bin`, SHA-256
`467fc31270e642cc86ca088f4f2809e943a3fd0c457b7bed042668e4f2ac7c86`.
It identifies itself as bootloader **01.01.260322**. The live TV runs V109 and
bootloader **01.01.250118**; these findings establish the newer implementation,
not byte-for-byte equivalence with the installed bootloader.

Addresses below are ARM virtual addresses used by `inspect-mboot.py`.
File offset = virtual address - `0x273fdc00`; the ARM image begins at file
offset `0x2400`, mapped to `0x27400000`.

## Main conclusions

- In this implementation, `flashing get_unlock_ability` prints the **current
  lock state**, `lock` or `unlock`. It does not report a separate permission to
  unlock. The live TV's `INFO lock` is consistent with this implementation and
  is not evidence that unlocking is prohibited.
- The USB `flashing unlock` handler, when currently locked, immediately wipes
  userdata and writes the unlocked state. There is no OEM-setting check,
  password/token check, or on-device confirmation step in this handler.
- Unlock writes a recovery wipe request to `misc`, runs `mmc erase.p userdata`,
  then writes `devicestate=unlock` to the environment and updates AVB state in
  RPMB. It therefore has real persistent side effects, even without flashing
  any image. Relocking also invokes the wipe path.
- The handler sends `OKAY Done!!` without checking the state-write return code.
  A successful reply alone does not establish a successful persistent unlock.
  Requery `unlocked`, and later verify it after a reboot.
- No partition-readback or temporary `boot` command is registered in this USB
  Fastboot command table. UART/MBOOT remains the concrete backup avenue.
- Normal unlock and critical unlock are separate. The latter clears an
  in-memory guard protecting `MBOOT` and `rom_emmc_boot` writes; it is not needed
  simply to obtain the ordinary unlocked state.

## Command registration identifies the USB handler

Fastboot initialization starts at `0x274aa364` (file `0xac764`). It registers
variables and these commands at `0x274aa5bc` through `0x274aa6a8`:

```text
getvar:
reboot
reboot-bootloader
reboot-fastboot
reboot-recovery
flashall
download:
flash:
erase:
flashing
```

The `flashing` registration loads the function pointer at
`0x27680d9c + 0x330 = 0x276810cc`, which contains `0x274a8e1c`.
This distinguishes the relevant USB handler from a second handler at
`0x2749cfbc` used by another Fastboot implementation in the same binary.

No `fetch`, `upload`, `boot`, or `oem` command is registered in the inspected
USB initialization routine. Therefore arbitrary `fastboot oem ...` commands
are not an established way to reach the MBOOT shell, and `fastboot boot` is
not an established way to test a kernel on this build. The current live
readback tests, saved separately in `assets/research/device/fastboot-readback-capabilities.txt`,
also found no `max-fetch-size` support and the client declined standard fetch.

## `get_unlock_ability` really means current lock state

USB handler `0x274a8e1c` (file `0xab21c`) calls `0x2749cf68` to obtain a boolean
unlocked state. That helper calls the AVB read-state function `0x274c243c`,
where stored state **1 is locked** and **0 is unlocked**, and returns the
inverse as a boolean.

At `0x274a8e88`–`0x274a8edc`, the handler compares the subcommand with
`get_unlock_ability`, copies `lock` for boolean 0 or `unlock` otherwise, sends
that text with the INFO function `0x274aa114`, then sends `OKAY Done!!`.
There is no independent OEM permission variable on this path.

The registered `unlocked` callback is `0x274a9008`; it returns `yes` for this
same boolean and `no` otherwise. The registered `secure` callback at
`0x274a9070` returns its inverse. Consequently **`secure: yes` is not separate
evidence of a hardware fuse or verified-boot policy in this implementation**.

## Exact unlock path and wipe

The USB unlock branch begins at `0x274a8f68` (file `0xab368`):

```text
compare subcommand with "unlock"
if current unlocked boolean != 0: send OKAY Done!!
print "wipe user data"
call 0x2749c384
call 0x274c252c with argument 0
send OKAY Done!!
```

There are no calls to physical-input polling or a confirmation UI between
recognizing the command and performing these writes. This is a complete
inspection of this handler, not proof that older firmware uses identical code.

Wipe routine `0x2749c384` (file `0x9e784`):

1. Configures raw I/O for partition `misc`.
2. Calls `raw_write` at `0x27430f5c` with offset 0 and length `0x55` from
   address `0x2767ec15` (file `0x281015`). These 85 bytes are a bootloader
   control message beginning with `boot-recovery`, padding, then
   `recovery\n--wipe_data\n` at the recovery-field position.
3. Invokes command runner `0x274b009c` with `mmc erase.p userdata`.

The raw-write function's own debug labels identify it as `raw_write`; the
wipe helper's error message says `raw_read misc fail!`, which appears to be
a misleading copied error string. The call and arguments establish the write.

State writer `0x274c252c` (file `0xc492c`), identified by its error strings as
`write_device_state`, does the following:

1. Maps 0 to `unlock`, nonzero to `lock`.
2. Sets environment variable `devicestate` and invokes the environment-save
   routine at `0x274a9a04`.
3. Updates the AVB device-state structure and invokes RPMB write at
   `0x274c3144`.
4. Reloads state and checks the result, returning failure on error.

The caller at `0x274a8f98`–`0x274a8fa8` ignores this return value and sends
`Done!!`. State readback is therefore essential after an actual unlock.
The state reader combines the environment and RPMB state; editing only the
environment is not established as sufficient to unlock.

## Critical protection

At `0x274a8ee0`–`0x274a8f38`:

- `lock_critical` stores 1 at global `0x27687de0`.
- `unlock_critical` prints a warning and stores 0 at that global.
- Neither branch writes persistent storage in the inspected handler.

The flash handler checks this global at `0x274a87a4` and rejects writes to
`rom_emmc_boot` or `MBOOT` when it equals 1. Normal unlock does not clear it.
The effect is an in-memory guard; persistence/reset behavior would require
checking initialization or observing a reboot. Do not assume it removes ROM
signature enforcement or makes arbitrary MBOOT replacements bootable.

## Grounded next choices

For preserving the installed V109 bootloader, obtain UART first and inspect
`help`, `printenv`, `help mmc`, and `help emmcbin`. This binary contains the
MBOOT commands `mmc read.p`, `mmc read.boot`, `emmcbin`, `emmcbootbin`, and
`tcl_mmc_dump`, with help strings describing USB-disk backups. Their presence
does not establish that the live console exposes them or that an arbitrary
RAM address is safe. Select exact read commands after capturing live help,
partition layout, and RAM layout. Preserve both eMMC boot areas, MBOOT,
environment, boot/recovery/vbmeta, and device-specific partitions before
bootloader-image experiments.

If the owner chooses an unlock test before acquiring the original backup,
the concrete command is `fastboot flashing unlock`, followed by
`fastboot getvar unlocked`; **treat the first command as an immediate userdata
wipe and persistent state change**, without expecting a TV confirmation
screen. The live V109 behavior remains unverified until that authorized test.
Keep critical unlock and image writes as separate later experiments.

## Reproducing the analysis

The existing helper requires Python and Capstone. On this workspace:

```sh
nix shell --impure --expr 'with import <nixpkgs> {}; python3.withPackages (ps: [ ps.capstone ])' \
  -c python3 research/firmware/V8-T221T04-LF1V122/inspect-mboot.py \
  --start 0x274a8e1c --length 0x1ec
```

Other ranges useful for review: `0x2749c384 + 0xb4`,
`0x2749cf68 + 0x54`, `0x274c243c + 0x94`, `0x274c252c + 0x134`, and
`0x274aa364 + 0x358`. The helper annotates PC-relative string references;
literal pools are disassembled as if they were code, so only the actual
function intervals should be interpreted as instructions.
