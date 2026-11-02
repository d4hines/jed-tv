# Confinement plan: every process except PID 1, modelled in Core.lean

Implementation plan for the next agent. Written 2026-11-01 at the end of a
design session with the owner; revised after the PR 14 review of e77f033 and
the new issue 12 rule on parsing. Target: the **original G06 TV** (Linux
4.19.116, our own PID 1, no SELinux policy loaded). The G13 is only for side
experiments.

Start from branch `d4hines/pid1-proof` (PR 14). It has the connection broker
(section 2), which this plan partly redesigns (section 4.9). Read
`src/pid1/lean/TV/Core.lean` first: it is the spec this plan extends.

Everything below is a plan. No Lean for the new model exists yet: the
guarantees are prose until step 1 of section 6 compiles them.

## 1. Decisions

Confirmed by the owner:

1. **Harden every process except PID 1.** Subjects: the seven apps,
   `tee-supplicant`, `wpa_supplicant`, `udhcpc` (with its lease hook and the
   `ip`/`ifconfig` it runs). Nothing else; no scope creep.
2. **Everything goes in `src/pid1/lean/TV/Core.lean`.** One file is the whole
   audit; use sections and comments for readability. Proof bodies stay
   one-line methods defined in `TV/CoreProofs.lean` (statements readable by
   an OCaml/Haskell programmer, tactics elsewhere).
3. **The manifest stays hard-coded** and becomes the *ceiling*: the most a
   subject can ever get.
4. **High-level app capabilities are dynamic.** Settings shows them and can
   switch them off and on within the ceiling. Every Linux mechanism is
   computed from the effective set.
5. **A global "disable networking" switch.** While it is on, the TV is
   offline, with as strong a guarantee as we can get.
6. **Prove the previous app is fully closed before the next one starts.** If
   an app cannot be closed, show a modal; OK reboots the TV.
7. **Infinite RPG is offline only** (network `none`). Its multiplayer is
   intentionally broken. Keep the app.
8. **Guarantees are stated for a production build.** The development back
   doors (recovery HTTP `/exec`, the REPL console, ADB) bypass everything, so
   Core takes a `production | development` build parameter.
9. **All parsing is done in the safe fragment of PID 1** (issue 12, added
   2026-11-01). Native effects move bytes; Lean turns bytes into values.
   Section 3.9 lists every native parser PID 1 has today and where it goes.

Defaults I chose; the owner has not objected. Use them unless told otherwise:

- User-facing capabilities: **Internet**, **Local network**, **Saved data**.
- Factory grants: every declared capability starts **on**.
- Granting needs the parental PIN; revoking does not (same as app
  enable/disable). No per-host toggles; host lists stay in the manifest.
- Going online (switching offline off) needs the PIN; going offline does not.
- Joining or changing a Wi-Fi network needs the PIN (issue 12 asks for this).
- Kernel module loading is locked after boot. The Wi-Fi radio is switched
  off by unbinding its USB driver, not by unloading the module.
- Development builds keep the Ethernet debug link up even when "offline"; the
  offline guarantee is therefore stated for production only.
- "Parsing" means turning bytes from any source other than PID 1's own
  constants into structured values: app messages, network data, files,
  daemon output, `/proc` and `/sys` text, the kernel command line. Fixed
  binary structs filled in by the kernel itself (evdev events, `waitpid`
  status, `pollfd`) are decoded by native code as today. Ask the owner if
  this boundary is wrong.

## 2. Current state

**PR 14 (`d4hines/pid1-proof`): the connection broker.**

- YouTube is `network := .hosts youtubeHosts` (`www.youtube.com`,
  `i.ytimg.com`, `under "googlevideo.com"`, `192.168.77.1`). It has no
  `socket`/`connect`/`bind` (network class 2 in `TV/Apps.lean`; profile in
  `TV/Sandbox.lean`). It sends `connect <host> <port>` on fd 4; Core checks
  it (input tag 5; the host is read byte by byte through `requestByte`);
  native code connects on a worker thread and passes the socket back with
  `SCM_RIGHTS`. Files: `src/pid1/tvinit/src/broker.rs`,
  `src/platform/src/broker.rs`, `core_effects.rs`, `linux.c` (fd 4).
- RPG is `network := .none`. Doom is the only `.unbrokered` app.
- Theorem `connectionsGoOnlyToListedHosts`.
- Checked: Core builds with all theorems; `test/lean/tv/core-check.py`;
  `test/lean/tv/check-sandbox.py` (918k BPF runs); ARM32 `cargo check`.
- QEMU: PID 1 grants and passes every connection. The TV-bridge gate gets
  through playback. The plain tour (`--check`) fails: ytplay exits with
  `Permission denied (os error 13)` about 50 ms after its fifth connection,
  before audio starts. Undiagnosed; neither seccomp filter returns EACCES.
  Evidence: `assets/build/integration-qemu-1793575076292363000/serial.log`
  (pid1-proof worktree).
- Review findings against this code, all still open (fixes in 4.9):
  app requests are serviced before remote keys, so a flooding app can delay
  Home; broker workers are unbounded and replies carry no request id; the
  request parser is Rust (violates decision 9) and DNS is musl's (same); the
  TV-bridge PCM endpoint and the dev HTTP proxy do not fit the broker; with
  the proxy the broker only checks the first hop; the "YouTube (Lean)" tile
  still calls `socket`/`connect` itself.

**PR 17 (`d4hines/confinement-trace`): the traces.** `tee-trace` logs every
syscall with paths, descriptors and socket addresses; `tools/tee-trace/
summarize.py` summarizes a log. Commit 1 is mergeable; commit 2 is a
TEMPORARY boot-time tracer hook in `hardware.rs`, to be dropped. Results from
the physical G06 are in the PR 17 comment and under
`assets/research/confinement-trace/20261101T224930Z/` (main worktree).

**What the traces say** (one run each; anything a daemon does only in other
situations is absent):

| | tee-supplicant | wpa_supplicant | udhcpc + lease.sh |
|---|---|---|---|
| today | root, all caps, no seccomp | root (spawned by `settings_wifi.rs`) | root (spawned by `settings_wifi.rs`) |
| threads / processes | 4 / 0 | 0 / 0 | 0 / 20 (sh, id, stat, ip, ifconfig, mkdir) |
| devices | `tee0`, `teepriv0` rw (TEE ioctls 0xa4 nr 0–7); `mmcblk0rpmb` rw (0xc008b301 ×8); `mmcblk0` read, one 0xc048b300 (MMC_IOC_CMD) | `urandom`, `random` | none |
| files read | its binary, bionic linker + libs (`/system/lib`, `/vendor/lib`, `/stock-system/...`), `/config/optee/teefs_path.ini`, TAs under `/mnt/vendor/tvservice/glibc/optee_armtz`, `/dev/__properties__/*`, `/sys/.../mmc0:0001/cid`, `/proc/self/*` | `/run/tv-network/wifi.conf`, sysfs for `wlan0` | `/run/tv-network/lease.sh`, `/proc/mounts`, `/proc/filesystems`, busybox applets |
| executable mappings | `mmap2` with `PROT_READ\|PROT_EXEC` of bionic libc/libdl/libm, `libc++`, `libnetd_client`, `libteec` | none (static) | none beyond exec |
| files written | **none in this run**; its store is `/vendor/tvcertificate/tee` (from `teefs_path.ini`); `/data/tee`, `/data/vendor`, `/mnt/vendor/tmp` were never touched | `wpa.log`, control dir+socket under `/run/tv-network/wpa` (created, removed) | `udhcpc.pid`, **`/etc/resolv.conf`** |
| sockets | none (loads `libnetd_client` but never opens one) | inet dgram (ioctls), netlink route (groups 1, listen), unix dgram control, packet 0x888e (EAPOL) | packet ETH_P_IP, inet raw proto 255, udp bound `:68`, netlink route |
| distinct syscalls | 34 | 32 (three wireless ioctls returned EPERM *as root*: driver behaviour, not needed) | 55 |

## 3. Core.lean: the model

### 3.1 Build and subjects

```lean
inductive Build where | production | development
def build : Build                    -- from a generated TV/BuildConfig.lean
inductive Daemon  where | teeSupplicant | wifi | dhcp
inductive Subject where | app (a : App) | daemon (d : Daemon)
```

PID 1 is the trusted base, not a subject. Development back doors are not
subjects either: they are native code absent from production builds, and
every theorem that depends on that says `build = .production →`.

### 3.2 Capabilities: ceiling, grants, effective set

```lean
inductive Capability where | internet | localNetwork | savedData
-- Manifest gains:  declares : Capability → Bool     (the ceiling)
-- Policy gains:    grants : App → Capability → Bool,  offline : Bool
def effective (p : Policy) (a : App) (c : Capability) : Bool :=
  (manifest a).declares c && p.grants a c && !(p.offline && c.isNetwork)
```

Ceilings: YouTube declares internet (its hosts) and localNetwork (casting);
Doom declares localNetwork (LAN deathmatch) and savedData; RPG declares
savedData only; Slides, Flight, Axiom, Snake declare nothing.

`effective` is what the policy *allows*. What a running app actually *has*
is fixed when it starts (its sandbox cannot be widened or narrowed after
exec). Section 3.5 is how a change to `effective` reaches running apps.

### 3.3 Linux confinement as data

```lean
inductive Cap where | netAdmin | netRaw | netBindService | sysRawio
-- The ONLY Linux capabilities any subject can hold. sysAdmin, sysModule,
-- setuid, ptrace, dacOverride, mknod, chown ... do not exist in this type.
structure Limits where threads memoryBytes openFiles : Nat
structure Mount  where path source : String; writable exec : Bool
structure Device where path : String; major minor : Nat; writable : Bool; ioctls : List Nat
structure Net where                   -- independent parts, any combination
  broker : List Host                  -- PID 1 opens connections to these (empty: none)
  lan    : Bool                       -- own INET/INET6 sockets, private destinations only
  daemon : List SocketRule            -- wifi/dhcp only: exact families, types, protocols
inductive Exec where | none | loader (libs : List String) | busybox (programs : List String)
structure Confinement where
  uid gid : Nat; caps : List Cap; mounts : List Mount; devices : List Device
  syscalls : List Rule; limits : Limits; net : Net; exec : Exec
def confine (p : Policy) : Subject → Confinement
```

`Net` is a record, not a choice: YouTube with both grants has `broker =
youtubeHosts` and `lan = true` at once (casting needs the LAN while the
host list stays brokered). No grant at all gives an empty network namespace.

Rows. Apps: uid and gid 10000+id (no more shared gid 4000), no caps;
mounts = read-only program and runtime, a 4 KiB `/proc/cmdline` copy,
`/data/apps/<uid>` rw only if savedData is granted (else a fresh tmpfs);
devices = `null`, `zero`, `urandom`, plus `fb0`/`snd` only in the
direct-device configuration (on the TV apps use the shared frames file and
the PCM socket); `exec = .none`. Daemons from the trace table: tee has
`exec = .loader [its libs]` and cap `sysRawio` *until the hardware test in
4.7 shows it is not needed*; wifi has `netAdmin`, `netRaw`; dhcp has
`netAdmin`, `netRaw`, `netBindService` and `exec = .busybox [sh, id, stat,
ip, ifconfig, mkdir, lease.sh]`.

### 3.4 Apps closing fully; the stuck modal

- `State` gains `stuck : Option App`. On a tick past `deadline + stuckGrace`
  (5 s) after KILL with no `exited`, `stuck := some a` and `.tell (.stuck a)`
  is emitted; the menus show "X didn't close. Restart the TV?" with
  Restart / Keep waiting.
- `Choice.restart` produces `Output.reboot` only when `stuck` is set.
  `Choice.keepWaiting` clears the modal; launches stay refused.
- `exited a` clears both `stopping` and `stuck`.
- `exited a` is a **native attestation** (4.4): reaped main process, no task
  with a's uid, a's cgroup empty, **and no broker work left for that launch**.
  The Lean theorems are about decisions given truthful attestations; they do
  not claim Linux itself is proved.
- **Launch generations.** Every `start` gets a fresh `Launch` number
  (`State.nextLaunch`, incremented per start). `Output.start a launch`,
  `Input.ask a launch r`, `Output.reply a launch r ok`, `Input.exited a
  launch`. A request or exit tagged with any launch other than the current
  one is dropped by `step`. Theorems: `repliesGoOnlyToTheCurrentLaunch`
  (every granted reply names the launch that is alive and in front),
  `staleRequestsChangeNothing`.

### 3.5 Changes take effect only when they are done: requested vs established

The review's point: changing `effective`, or what future launches get, does
not take back an open socket or a writable mount; and emitting shutdown
effects does not prove they worked. So both kinds of change are two-step:

- **Revoking a capability.** Settings is only reachable from home, so the
  only app that can hold the capability is one still closing after Home.
  The revoke choice is accepted only when `stopping = none` (no app alive);
  otherwise the UI shows "Waiting for X to close" and offers the stuck
  modal path. With no app alive, no socket, mount or descriptor of the old
  grant exists, and the next launch is built from the new policy. Brokered
  sockets live in the app's descriptor table (PID 1 keeps no copy), so they
  are gone when the app is reaped, even though PID 1 created them.
- **Going offline.** `Policy.offline` is the *request*; `State.net :
  NetState := online | goingOffline | offline | failed` is what has been
  *established*. `Choice.goOffline` sets the request, emits the shutdown
  effects (3.6) and enters `goingOffline`. Native code then verifies each
  layer (4.5: link flags read back down, daemons reaped, Wi-Fi driver
  unbound, deny-all attached and queried) and reports `Input.netDown ok`.
  Only `ok = true` enters `offline`; `false` enters `failed`, which shows
  the restart modal (a reboot with offline saved never raises a link).
- **Operation ids.** Every network command carries a fresh `NetOp` number
  (`State.nextNetOp`): `Output.network op (.up | .down)`, `Input.netUp op ok`,
  `Input.netDown op ok`. `NetState` remembers the pending op
  (`goingOffline op`, `goingOnline op`). An attestation whose op is not the
  pending one, or that arrives with nothing pending, or a second time, is
  ignored. A new request while one is pending supersedes it (new op); the
  old op's reply is then stale.
  Settings shows "Going offline…" until established. Like `exited`, this is
  a native attestation, and the theorems say so.
- An app may be launched while `net ≠ online` only with no network
  capabilities; while `goingOffline` launches wait.

### 3.6 Network daemons and the offline switch in Core

- `Output.daemon d (.start | .stop)`; `Input.daemonExited d`. wifi and dhcp
  start only when `net = online` and Wi-Fi is configured.
- `Output.network op (.up | .down)`, `Input.netDown op ok`,
  `Input.netUp op ok` (operation ids, 3.5).
- `Choice.goOffline`, `Choice.goOnline typed`, `Choice.wifi conf typed`.
- At boot `main` emits `.network .up` only if the loaded policy is online.

### 3.7 Theorems (statements; proofs in CoreProofs)

The existing 30 theorems stay. New:

- `grantsNeverExceedTheManifest`, `grantNeedsPin`, `revokeNeedsNoPin`,
  `revokeOnlyWithNoAppAlive`, `fewerGrantsNeverMoreAccess` (confine is
  monotone in grants: fewer grants ⇒ subset of mounts, devices, syscalls,
  caps, broker hosts; `lan` no wider).
- `noSubjectIsRoot`, `subjectsHaveDistinctUids` (apps and daemons),
  `appsHoldNoCapabilities`, `dangerousCapabilitiesDoNotExist`,
  `appsCannotExec`, `onlyOwnDirectoryIsWritable`, `appsSeeNoProcOrSys`,
  `onlyTeeOpensTeeDevices`, `onlyTeeHoldsRawio`, `dhcpExecsOnlyItsHook`.
- `offlineMeansNoNetworkCapability`, `offlineGrantsNoConnections` (no grant
  while `offline` is requested, whatever `net` is),
  `offlineStartsNoNetworkDaemon`, `goingOfflineEmitsEveryShutdown`
  (`.network .down`, `.daemon wifi .stop`, `.daemon dhcp .stop`),
  `offlineEstablishedOnlyAfterAttestation` (`net = offline` only after
  `netDown op true` for the pending `op`), `staleAttestationsChangeNothing`
  (an attestation for any other op, or with nothing pending, leaves `State`
  unchanged), `onlineEstablishedOnlyAfterAttestation`, `goingOnlineNeedsPin`,
  `wifiChangeNeedsPin`.
- `bootOfflineRaisesNoLinkBeforeAuthorizedOnline`: for a run from
  `State.boot p` with `p.offline = true`, `.network .up` appears only after
  an input `.choose (.goOnline pin)` where `pin` was the current PIN. (Not
  "never": an authorized go-online is allowed.)
- `nextAppStartsOnlyAfterThePreviousExited` and `atMostOneAppAlive` (run
  induction, like `pinNeverChangesWithoutPin`), `rebootOnlyWhenTheUserConfirms`,
  `stuckOnlyAfterKill`.
- `seccompMatchesPolicy`: the BPF program compiled from a `Confinement`
  allows exactly the syscalls (with argument rules) the row permits. This is
  the one sizeable proof; ship the generator with `check-sandbox.py`
  coverage first and the proof second.

### 3.8 Native boundary (section 8 of Core.lean)

Keep scalar effects. New externs: `daemon (d : UInt8) (start : Bool)`,
`network (up : Bool)`, `reboot`, `load`/`save` gain grant bits and the
offline bit, `start id grantsBits` (the launcher gets the effective set,
never the policy), `tell` kind 4 = stuck modal. Input tags: 6 daemon exited,
7 network attestation (up/down, ok, op id in bits 32–63). Launch numbers
and op ids are 32-bit on the wire; Core compares them exactly. Byte-moving effects for parsing in
Lean (3.9): `readFile`, `requestBytes`, `dnsSend`/`dnsReceive`. The
launcher and seccomp generator read `Apps.lean`'s scalar copy as now;
`nativeAppTableIsTheManifest` grows to cover `declares` and `Net`.

### 3.9 Parsing in Lean (decision 9)

Native code may only move bytes (read a file, receive a datagram) and act
on values Lean returns. Every parser PID 1 has today, and its fate:

| Today (native) | What it parses | Move to |
|---|---|---|
| `tvinit/src/broker.rs` `parse` | app request datagrams | Lean: native hands the raw datagram (≤ 300 bytes) through `requestBytes`; Lean parses `connect <id> <host> <port>`, validates, decides; the reply effect carries id, port and the host as a byte range of the raw datagram, so native never re-parses |
| musl `getaddrinfo` in the broker worker | `resolv.conf`, DNS replies | Lean DNS stub: Lean builds the query, native sends/receives one UDP datagram, Lean parses the reply and returns an IPv4/IPv6 address as scalars; native only `connect`s to that address. Lean reads the DNS server from the dhcp user's file |
| `src/pid1/src/tee_boundary.rs` | sysfs `major:minor`, `/proc/<pid>/task/*/syscall` text | Lean parses the bytes `readFile` returns; or drop the syscall-text readiness check for a simpler signal |
| `settings_wifi.rs` | pid file, `/proc/<pid>/cmdline`, `wpa_state=` / `ip -o` output | pid files and cmdline checks disappear (Core daemons: PID 1 knows its children's pids); status text goes to `TV/Wifi.lean` like the scan text already does |
| `settings_store.rs` | `/proc/cmdline`, `/proc/bootconfig`, sysfs device numbers (hardware identity checks) | Lean predicates over the bytes; native keeps the checks' effects |
| `lib.rs` `arg`, `/etc/tvinit.conf`, `doom.*` words | kernel command line and config | one Lean parser producing a typed config, passed to native as scalars |
| `direct_services.rs` | `/proc` pids and `comm` (ownership check) | Lean over `readFile` bytes |
| `audio_service.rs`, `lib.rs` gain files | float text | Lean |
| `core_menu.rs:107` | menu config value | Lean |
| `main.rs` HTTP, `repl.rs` | requests, commands | development only; compiled out of production builds |

Already compliant: Wi-Fi scan text and configuration (`TV/Wifi.lean`), the
settings file (`TV/SettingsFile.lean`), the PCM protocol (`TV/Audio/`),
`waitpid` status decoding (`TV/Supervisor.lean`). Build-time constants
embedded in the binary (amplifier plan, property archive) are not runtime
input; leave them unless the owner says otherwise.

### 3.10 "Not proven" additions

Native code must: report `exited a` only when a's main process is reaped,
no process runs as a's uid, and a's cgroup is empty; report `netDown true`
only after verifying every layer in 4.5; start daemons only through
`confine`'s row; move bytes without interpreting them; and in production
builds contain no REPL, `/exec`, ADB or debug link. Plus: the kernel and
vendor drivers; Wi-Fi/Ethernet/TEE firmware below Linux; physical access;
DNS answers for an allowed name are trusted (the network can still lie).

## 4. Native implementation

### 4.1 Launch program (`src/pid1/lean/native/linux.c`, `launch.rs`)

Replace the fixed sequence with an interpreter of `Confinement` (read as
scalars via exported Lean functions like today's `tv_app_uid`). Order, each
step fatal on error:

1. dup the input socket to fd 3 and the request socket to fd 4 (only when
   `net.broker` is non-empty); close everything else (keep 1023 = the
   executable).
2. `unshare(CLONE_NEWNS | CLONE_NEWIPC | CLONE_NEWUTS)`, plus
   `CLONE_NEWNET` when `net.lan = false` and `net.daemon = []` (G06:
   NET_NS, UTS_NS, IPC_NS all `y`). No PID or user namespaces: the kernel
   has neither.
3. Build the view on an empty tmpfs root: bind each `Mount` (ro unless
   `writable`, always nosuid, noexec unless `exec`), a private tmpfs `/dev`
   with `mknod` of exactly the row's devices (no device cgroup on this
   kernel, so the private `/dev` is the device gate), the `/proc/cmdline`
   copy, `/tmp` as a small tmpfs. `pivot_root`, not `chroot`. Apps never get
   a live `/proc` or `/sys`; tee gets a private procfs and one bound sysfs
   file.
4. Join the subject's cgroup (4.3).
5. rlimits from `Limits`, plus `CORE=0`, `RTPRIO=0`, `MEMLOCK=0`,
   `MSGQUEUE=0`, `NPROC = threads`.
6. Bounding set to `caps`; `PR_SET_KEEPCAPS`; `setgroups(0)`, `setresgid`,
   `setresuid`; `capset` permitted+effective = `caps`; ambient = `caps` for
   rows whose exec chain must keep them (dhcp's hook); `PR_SET_NO_NEW_PRIVS`.
7. seccomp from the row's program (4.2), then `execveat` on fd 1023.

Theorems about the order (mounts private before binds, setgroups before
setresuid, no_new_privs before seccomp, seccomp last before exec, exec last
and of the row's own program) are stated over the Lean `launch` list the
interpreter follows.

### 4.2 Seccomp generator (`TV/Sandbox.lean` folds into Core)

Replace the index table with a compiler from a list of `Rule`s:
`.call n`, `.callArg n argIndex (eq | maskEq) value`, `.ioctl numbers`,
`.socket family type protocol` (rejects everything else), `.clone` (thread
flags only, as today), `.execveat` (fd 1023 only). Keep the ARM EABI /
AArch64 tables.

Executable memory, per row (the review caught a wrong blanket rule):

- Static apps (`exec = .none`): deny `PROT_EXEC` in `mmap2`/`mprotect`.
  The kernel maps the program during `execveat`, which seccomp does not
  filter, so static binaries never need it again.
- The tee helper (`exec = .loader`): its loader maps libraries
  `PROT_READ|PROT_EXEC` (seen in the trace), so allow that and deny only
  `PROT_WRITE|PROT_EXEC` together. File-backed executable mappings are
  further limited by mounts: only its library mounts are not `noexec`.
  Anonymous memory turned executable with `mprotect` after being writable
  remains possible for it; say so in the risks.
- busybox (`exec = .busybox`): static, same as apps, plus `execve` of its
  listed programs only (mounts make everything else `noexec`).

Daemon socket rules from the traces: netlink only `NETLINK_ROUTE`; no raw
inet for wpa; `setsockopt` only at `SOL_SOCKET` and the specific options the
raw logs show (extract `(level, optname)` pairs from
`assets/research/confinement-trace/.../{wpa,dhcp}-syscalls.jsonl`). This is
what stops `netAdmin` from rewriting firewall state. Coverage first via
`test/lean/tv/check-sandbox.py` over every subject × grant combination,
then `seccompMatchesPolicy`.

### 4.3 cgroups

Mount cgroup2 at `/sys/fs/cgroup` (4.19: memory, cpu with CFS_BANDWIDTH,
cgroup BPF; no pids or device controller, no v2 freezer). Groups:
`/tv/app-<id>`, `/tv/tee`, `/tv/wifi`, `/tv/dhcp`; PID 1 in the root.
`memory.max` from `Limits.memoryBytes`; `cpu.max` on apps so PID 1 always
keeps a share (this backs "Home always works" in practice). Theorem: the
sum of memory limits of everything that can run at once plus PID 1's
reserve ≤ RAM (1430 MB on the G06).

### 4.4 "Fully closed"

`tv_core_next` reports `exited a` only when: `waitpid` reaped the main pid,
`/tv/app-<id>/cgroup.procs` reads as empty (zero bytes: no parsing), and a
scan finds no task with uid `10000+id` (the uid check parses `/proc`
status, so it moves to Lean per 3.9, or is dropped in favour of the cgroup
check alone). If the checks disagree, SIGKILL whatever remains and wait.
PID 1 closes its ends of fds 3 and 4.

Broker work for the launch must also be finished before `exited` is
reported. Each broker job records its launch number. On Home/exit: mark the
launch closed, cancel its jobs (close the socket being connected, which
ends a blocking connect; DNS and proxy exchanges are non-blocking with a
deadline and check the closed flag between steps), close any connected
socket that was not delivered, and wait for the job count of that launch to
reach zero. A job that completes after its launch closed closes its socket
and sends nothing. The launch's request socket is closed only after that,
so no reply can reach a later launch through a recycled descriptor. A process stuck in a vendor driver
never dies: Core's stuck modal and `reboot` (sync + `RB_AUTOBOOT`) is the
way out.

### 4.5 Networking and the offline switch

Layers, strongest first; each holds even if the others fail. `netDown ok`
is reported only after every layer is verified:

1. **Policy:** no network capability in `effective`; the broker refuses;
   no daemon start. (Verified by construction.)
2. **Namespaces:** apps launched offline get an empty netns.
3. **Daemons:** wifi and dhcp stopped (TERM, KILL after 1.5 s). Verified:
   both reaped, their cgroups empty.
4. **Links:** `eth0` and `wlan0` down via `SIOCSIFFLAGS`; the Wi-Fi USB
   driver unbound (`/sys/bus/usb/drivers/<driver>/unbind`, device id from
   `/sys/class/net/wlan0/device`, `2-1:1.2` in the trace). Verified: flags
   read back down, `wlan0` gone. Going online rebinds; the module is never
   unloaded, so the module lock (4.6) stays.
5. **Packet filter:** a deny-all eBPF program on the root cgroup
   (`BPF_CGROUP_INET_EGRESS`/`INGRESS`, 4.19, `CGROUP_BPF=y`), so even PID
   1's own sockets (including broker sockets, which live in the root
   cgroup) cannot move a packet. Lean generates the bytecode. Verified:
   `BPF_PROG_QUERY` shows it attached. Development builds allow
   192.168.77.0/24 on `eth0`. If the vendor verifier rejects the program,
   fall back to a DROP ruleset built by PID 1 via `IPT_SO_SET_REPLACE`
   (`IP_NF_FILTER=y`, `IP6_NF_FILTER=y`; there is no iptables binary).
6. **Boot:** production builds no longer bring `eth0` up in the pre-Lean
   Rust stage (`main.rs` `network()`); Core emits `.network .up` after
   loading the policy, only when online.

Per-app LAN (`net.lan`) uses cgroup-BPF on `/tv/app-<id>`
(`INET4_CONNECT`, `INET6_CONNECT`, `UDP4/6_SENDMSG`, egress): allow only
10/8, 172.16/12, 192.168/16, 169.254/16, fe80::/10 and multicast; deny
loopback and everything else.

DNS: udhcpc's hook writes `/run/tv-network/dhcp/resolv.conf`, owned by the
dhcp user; Lean reads the server address from it (3.9). `/etc/resolv.conf`
is no longer written. `lease.sh`'s `id`/`stat` checks change to "running
as the dhcp user, directory owned by it".

### 4.6 Kernel one-way switches: one order

Boot order, with no exceptions: hardware staging loads **all** modules
(vendor profile plus the Wi-Fi set `ehci-hcd`, `phy-ms-usb`,
`firmware_class`, `cfg80211`, `8733bu`, moved here from `prepare.sh`) →
**lock** → tee-supplicant → display/audio init → launcher. If display or
audio init turns out to load a module, move that load into staging; check
on the TV that `/proc/modules` does not change after the lock.

The lock: `kernel/modules_disabled=1`, `kexec_load_disabled=1`,
`kptr_restrict=2`, `dmesg_restrict=1`, `perf_event_paranoid=3`,
`unprivileged_bpf_disabled=1`, `bpf_jit_harden=2`, `sysrq=0`,
`suid_dumpable=0`, `fs/protected_{symlinks,hardlinks}=1`,
`protected_{fifos,regular}=2`; ipv4/ipv6: no redirects accepted or sent, no
source routing, `rp_filter=1`, `icmp_echo_ignore_broadcasts=1`. Theorem:
in a boot run, every `.start` and `.daemon _ .start` (tee included) comes
after `.kernelLocked`.

### 4.7 Daemons under confinement

- **tee-supplicant** (`hardware.rs` `start_tee`): launched through the
  interpreter with its row; env unchanged. Storage: `/vendor/tvcertificate/
  tee` rw for the tee user; stop creating `/data/tee`, `/data/vendor`,
  `/mnt/vendor/tmp`. **Hardware gate:** run it without `sysRawio`; if the
  TEE reaches "waiting" and Doom has picture and sound, drop the cap. If
  not, keep it and record that it can send any MMC command.
- **wpa_supplicant / udhcpc** (`settings_wifi.rs`): stop spawning them with
  `Command`; they become `.daemon` outputs carried out by the interpreter.
  `/run/tv-network` becomes per-daemon directories owned by their users;
  `wpa_cli` runs as the wifi user.

### 4.8 Storage and Settings UI

- `SettingsFile.lean`: `Policy` gains `grants : UInt32` (bit `appIndex*3 +
  capIndex`) and `offline : Bool`; the text format gains two words; parse
  falls back to defaults for missing words.
- `SettingsView.lean`: per-app capability pages (On asks for the PIN); root
  page "Networking: On/Off" (On asks for the PIN; shows "Going offline…"
  until established); Wi-Fi changes ask for the PIN; "System and proofs"
  shows each app's effective set. The UI still never sees the PIN. New
  choice kinds in `decodeInput`: 5 grant, 6 revoke, 7 offline, 8 online, 9
  restart, 10 keep waiting, 11 wifi.
- `core_menu.rs`: the stuck/failed modal (kind 4 notice) with Restart /
  Keep waiting.

### 4.9 Broker fixes (code already in PR 14)

1. **Home first.** In `tv_core_next`, drain remote keys before app
   requests, and take at most one request per 20 ms tick, so no app can
   delay Home or Power.
2. **Bounded work.** At most 2 broker workers per app; more requests get
   `EAGAIN` straight away. Each worker has the 10 s connect timeout.
3. **Request ids and launch binding.** `connect <id> <host> <port>`; the
   reply datagram is `<id:u32><status:i32>` (+ the socket). The client
   discards replies with another id, so a late reply to a timed-out request
   can never be taken for the current one. On PID 1's side every job is
   bound to its launch number, cancelled and drained on close (4.4), and
   never delivers to another launch.
4. **Parsing and DNS in Lean** (3.9).
5. **Proxy at the broker, not in the app.** When a dev proxy is configured,
   PID 1 checks the *final* host against the manifest, connects to the
   proxy, sends `CONNECT host:port`, Lean parses the reply status, and only
   then passes the tunnelled socket. Then the host check covers the tunnel
   destination, not just the first hop. Apps stop reading
   `TV_HTTP_PROXY`.
6. **PCM is not a broker connection.** The TV-bridge audio endpoint is a
   local service: PID 1 connects it and passes the socket at launch (like
   fd 3), instead of the app asking the broker for an address that is
   refused as loopback.
7. **Lean YouTube.** A `recvmsg`+`SCM_RIGHTS` primitive in ZrTv and
   `Conn.open` asking the broker. Coordinate with the Lean YouTube session
   (AGENTS.md, issue 12) before touching `src/lean/zr`.
8. **The `--check` EACCES** (section 2): diagnose first, it may be one of
   the above.

## 5. Tests and gates

- **The QEMU kernel must match the G06's features first.**
  `demo/kernel/config` has `CGROUPS`, `BPF_SYSCALL`, `NET_NS`, `UTS_NS` and
  `NETFILTER` off. Rebuild with `CGROUPS`, `CGROUP_BPF`, `BPF_SYSCALL`,
  `MEMCG`, `CGROUP_SCHED`, `CFS_BANDWIDTH`, `NET_NS`, `UTS_NS`, `IPC_NS`,
  `NETFILTER`, `IP_NF_FILTER`, `IP6_NF_FILTER` (`tools/kernel/build.sh`,
  built on eduardo-vm), then commit the new `zImage.xz` and config.
- `test/lean/tv/core-check.py`: grants, revocation waiting for close,
  offline requested vs established (including a failed attestation), the
  stuck modal (inputs where `exited` never arrives), wifi PIN; requests and
  exits tagged with an old launch number are ignored; online/offline
  toggled quickly with completions reordered, duplicated and unsolicited,
  plus a failed shutdown, and only the matching successful attestation
  establishes the state.
- `test/lean/tv/check-sandbox.py`: every subject × grant combination;
  socket-argument and ioctl rules; the per-row executable-memory rules.
- `test/fixtures/confinement-init.rs` (QEMU): add daemon rows and a stand-in
  that tries forbidden things (open `/dev/tee0` as an app, `mknod`,
  `socket` without the grant, write outside its directory, exec).
- `demo/run.py` tour: revoke YouTube's Internet → `broker: refused`; grant
  with PIN → playback works; Doom with LAN revoked → no sockets; a flood of
  requests while pressing Home → Home still wins.
- Broker closure (QEMU, with a test server on the host that can stall DNS,
  the TCP accept, and the proxy `CONNECT` reply): press Home while each is
  pending and check `exited` waits for the job; relaunch YouTube, release
  the stalled server, and check the late completion delivers nothing to the
  new launch and leaves no descriptor open in PID 1
  (`ls /proc/1/fd` count back to baseline).
- **Offline gate:** `-object filter-dump,id=f0,netdev=n0,file=…pcap`;
  switch offline; wait for "established"; drive every app and PID 1's own
  network paths; assert zero frames from the guest after establishment;
  reboot with offline saved and assert zero frames from power-on.
- Hardware (owner present): tee without `sysRawio`; `/proc/modules`
  unchanged after the lock; offline on real Wi-Fi (unbind) and back online
  with the PIN; YouTube through the broker-side proxy.

## 6. Order of work

1. Core.lean types, rows, theorems as statements; proof macros in
   CoreProofs; `core-check.py` green. Everything else keys off this.
2. Move PID 1's parsers to Lean (3.9), starting with the broker's.
3. Broker fixes 4.9.1–4.9.6.
4. Storage + Settings UI + menus (grants, offline states, wifi PIN, modals).
5. QEMU kernel config (section 5), then the launcher interpreter: empty
   root, private `/dev`, netns, cgroups, per-row seccomp;
   `check-sandbox.py` and the QEMU fixture green.
6. Boot order and kernel lock (4.6).
7. tee-supplicant row (hardware gate for `sysRawio`).
8. wifi/dhcp rows as Core daemons; DNS file; `lease.sh`.
9. Offline switch end to end: cgroup BPF, unbind, attestation, pcap gate.
10. "Fully closed" checks, stuck modal end to end, reboot.
11. `seccompMatchesPolicy` and the eBPF theorem.
12. Lean YouTube tile on the broker (with the Lean YouTube session).

## 7. Risks

- tee-supplicant may need `CAP_SYS_RAWIO` for one MMC_IOC_CMD on `mmcblk0`
  (reading EXT_CSD to size RPMB). With it plus an open `mmcblk0`, a
  compromised helper can issue any MMC command. Only the hardware test says
  whether it is needed. It can also make writable anonymous memory
  executable (4.2).
- wifi and dhcp hold `netAdmin`. The socket rules close firewall access;
  the worst left is mis-routing or link flaps, and offline mode does not run
  them.
- The 4.19 eBPF verifier may reject what QEMU's kernel accepts; keep the
  programs tiny and try them on the TV early. Fallback in 4.5.
- Development builds cannot have the offline guarantee; say so in Settings.
- The Lean YouTube tile breaks under the brokered sandbox until 4.9.7.
- Not modelled, by decision: SELinux, Landlock (not in 4.19), per-host
  toggles, any subject beyond the four classes above.
