# Milestone M1 on the TV: our app as PID 1 on the stock kernel (handoff)

agent1 (QEMU track), 2026-10-31. For the device track (Codex) and the owner.
Nothing here was run on the TV; this file only describes artifacts built
offline from the local V109 backup. Read `RECOVERY_NOTES.txt` and `AGENTS.md`
before any boot experiment.

## What M1 means
Linux (the TV's own V109 kernel, unmodified) with **every userspace process
ours**: `/init` is `tvinit` (Rust, 32-bit ARM), which configures the network,
loads kernel modules, and (later) runs the home screen and apps. Vendor
kernel modules count as kernel, not userspace.

## Artifacts (worktree /Users/d4hines/repos/dande-tv/agent1; assets/ is shared with main)
Built by `tools/make_tv_boot.py <stage>` into `assets/build/tv-boot/<stage>/`
(`boot.img`, `ramdisk.cpio.gz`, `manifest.json`).

| stage | tests | boot.img sha256 | ramdisk.gz |
|---|---|---|---|
| m1a | tvinit is PID 1 on the TV kernel and reachable over Ethernet; kernel log over the network | ef9e0ee7ac20ac0000fe4ced14c780ae44c925f425c023ae9536019935d12f2b | 286 KB |
| m1b | m1a + loads the 13 MStar display-stack modules in vendor `init.fusion.rc` order; does `/dev/fb0` appear? | 6d601194491f54bfd2abb894df0eded25739079b6e1513317205ce1d2c157477 | 9.6 MB |
| m1c | m1b + the home screen and `/bin/ytplay` (no hold). YouTube via the laptop's replay server | c02ea0c9296d0d02c48257e13574bb4820932af22d0f150499042dd80e4503ad | 11.6 MB |

- Stock input: `assets/backups/v109-stock-20261031/boot.img`, sha256
  d116dfa370d8e5dd5809f6f65266c86671b5e9bec9d62f95c0977bca0ff30085 (the same
  image as Codex's boot POC `original_sha256`).
- Kernel and DTB are byte-identical to stock (verified after repacking; kernel
  sha256 01e36bda...). Header v2 and all offsets are as in stock. Self-check:
  repacking the STOCK ramdisk with the same arguments reproduces the stock
  image's first 20,621,312 bytes exactly (everything before its AVB footer).
- Ramdisk: newc cpio, gzip -9 (kernel has CONFIG_RD_GZIP). Contents: `/init`
  (tvinit), `/dev/console`, `/etc/tvinit.conf`; m1b/m1c add
  `/lib/modules/*.ko` (extracted unmodified from the V109 vendor partition) and
  `/etc/modules.list`; m1c adds `/bin/ytplay` and `/share/tiles/*.rgb`.
- Command line = stock + `rdinit=/init tvinit.iface=eth0
  tvinit.ip=192.168.77.2/24 tvinit.log=192.168.77.1:5140 tvinit.status=7777`
  (+ `tvinit.hold=1` for m1a/m1b, `ytplay.server=http://192.168.77.1:8090`
  for m1c). The same settings are ALSO baked into `/etc/tvinit.conf`, so it
  works even if MBOOT rewrites the command line. (Codex's
  `androidboot.codex_poc=1` test answers whether it does.)

## How to run (the device track/owner decides; RAM-only)
1. On the laptop (192.168.77.1): `python3 tools/udplog.py --port 5140 --out /tmp/tv-udp.log`
   (from the agent1 worktree; macOS may ask to allow incoming connections).
   For m1c also start the replay server on the TV link:
   `python3 test/replay.py --host 192.168.77.1 --port 8090 --player ... --fixtures ...`.
2. Boot the image from RAM only: `fastboot boot assets/build/tv-boot/m1a/boot.img`.
   **Never `fastboot flash` these.**
3. Watch the UDP log, or pull the boot log at any time: `nc 192.168.77.2 7777`.
4. Back to Android: power-cycle. tvinit mounts only devtmpfs/proc/sysfs (in
   RAM) and writes nothing to eMMC; it never opens /dev/block.

## What success looks like (UDP log lines)
- m1a: `tvinit: network: eth0 = 192.168.77.2/24`, `logging to udp://...`,
  `status: boot log on tcp port 7777`, `hold: bring-up only`, plus `kmsg [...]`
  lines carrying the TV kernel's own log. `ping 192.168.77.2` answers.
- m1b: `module /lib/modules/<m>.ko loaded in N ms` for all 13, then
  `modules: 13 loaded, 0 failed`; the kmsg lines from utpa2k/mik/fbdev show
  whether `/dev/fb0` (Mstar-fb / mstar_fbdev_mi) registered.
- m1c: `home screen: ...` and the menu on the panel (if the display stack
  works without Android's userspace); remote keys arrive as evdev.

## Known unknowns / risks
- Whether MBOOT supports `fastboot boot` (RAM boot) on G06.
- Load addresses: the header puts the kernel at 0x20208000, the ramdisk at
  0x21208000 and the DTB at 0x22108000 (a 15 MB ramdisk window). The stock
  kernel blob is 19.8 MB, which on paper overlaps the ramdisk address, yet
  stock boots, so MBOOT must place images itself. Ramdisks larger than
  stock's 732 KB (m1b 9.6 MB, m1c 11.6 MB) are untested. m1a is the safe first
  step.
- MStar modules may expect files from /vendor or /tvconfig partitions
  (iniparser suggests kernel-side config parsing), or init done by Android
  userspace (midaemon/utopia, tvservice). The kmsg stream will show where
  m1b stops. The panel might also depend on MBOOT's logo/panel init.
- IR remote: the driver is built into the kernel (CONFIG_MSTAR_IR_REFACTOR=y,
  CONFIG_RC_CORE=y; `/dev/input/event2` "MStar Smart TV IR Receiver"), so it
  needs no module or Android service and works already in m1a. Expected
  Linux keycodes come from TCL's own layout file
  `/vendor/usr/keylayout/Vendor_3697_Product_0001.kl` (copy:
  `assets/research/tv-pid1/v109/keylayout/`; also readable straight from
  `TVBACKUP/TCL32Q3K-V109/super.img`) together with the IR table
  `/vendor/tvconfig/config/ir_config.ini` in tvconfig.img. The handset
  (TCL G0401-000303 family) has: Power=116, Mute=113, arrows 103/105/106/108,
  OK=28 (KEY_ENTER), Back=158, Home=102 (KEY_HOME), Vol -/+ = 114/115,
  Ch -/+ = 403/402, NETFLIX=566, prime video=752, YouTube=569. It has no
  number pad, colour keys or transport keys (no play/pause, fast-forward,
  rewind). Its Input/Settings/TCLtv+ buttons' codes are unconfirmed.
  `platform::remote` maps these to actions; in QEMU `demo/remote.py` sends
  the same codes to PID 1's virtual IR receiver (`remote.udp=PORT`).
  To confirm on the device (issue #7): `adb shell getevent -l /dev/input/event2`
  while pressing OK, Home, Back, YouTube, prime video and Ch +.
- The TV has no RTC time and no internet. m1c's YouTube uses the replay
  server only.

## QEMU evidence behind this
- The tvinit bring-up (static IP via ioctls, UDP log, kmsg forwarding, TCP
  boot log, module loading via finit_module incl. a failure case) passes on
  arm64 and 32-bit ARM; the 32-bit run is configured purely from
  /etc/tvinit.conf. The kernel module `test_module` printed "Hello, world"
  into the forwarded kmsg.
- The exact m1a `ramdisk.cpio.gz` boots on the 32-bit QEMU kernel: tvinit
  configures 192.168.77.2/24 from /etc/tvinit.conf and holds.
- Tools: `tools/make_tv_boot.py`, `tools/udplog.py`, `test/run.py --hostfwd`,
  tvinit options documented at the top of `src/tvinit/src/main.rs`.
