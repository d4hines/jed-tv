prelude
import Init.System.IO
import Init.Data.Int.OfNat
import Init.Data.List.Lemmas
import Init.Omega
import TV.Apps
import TV.CoreProofs

/-!
# PID 1: the core

Everything PID 1 decides, and everything we claim about those decisions, is in
this file. PID 1 is a pure state machine:

    step : State → Input → State × List Output

It takes one `Input` (a key press, a clock tick, an app exiting, a request
from an app, a choice made in PID 1's own menus, a report from native code)
and returns its new `State` and a list of `Output`s (show a screen, start or
signal an app, save settings, take the network down, ...). The running PID 1
is `loop step` (section 10), where every output is carried out by one of a
short list of native functions. Every decision it makes is a call to `step`,
so the theorems in section 9 cover all of them.

Every other process on the TV runs confined. Section 4 says, as plain data,
what Linux lets each one do: its user, the files and devices it can see, its
network, its system calls. PID 1's launcher builds exactly that sandbox.

Sections:

1. Who runs     — the apps, the three helper daemons, the build
2. Capabilities — what each app may ever have, and what the user switches on
3. Policy       — the parental PIN, enabled apps, grants, offline
4. Confinement  — what Linux enforces for every process except PID 1
5. State        — what PID 1 remembers between inputs
6. Inputs       — everything PID 1 can observe
7. Outputs      — everything PID 1 can do
8. The machine  — `step`
9. Theorems     — the claims, in English and in Lean
10. Effects     — every native function PID 1 calls, and the loop
11. Not proven  — what the theorems rely on but do not cover

Each theorem ends in a one-word proof method (`by cases_on_step` and so on),
defined in `TV/CoreProofs.lean`. You do not need to read it: Lean checks
every proof whenever it builds this file.

## Reading Lean as a Haskell/OCaml programmer

- `inductive T where | a | b (x : Nat)` is `data T = A | B Nat`.
- `structure R where x : Nat` is a record; `{ r with x := 1 }` updates a field.
- `.home` is a constructor whose type Lean infers from context (`Screen.home`).
- `def f (x : A) : B := ...` is a function; `match x with | ... => ...` is `case`.
- `(p.1, p.2)` are the two halves of a pair.
- `theorem name : statement := proof` is a claim Lean has checked. Read the
  statement like a type: `∀` is "for all", `→` "implies", `∧` "and", `∨`
  "or", `∃` "there is", `∈` "is in the list", `≠` "is not".
-/

namespace TV.Core

/-! ## 1. Who runs

The installed apps are a closed type, so "every app" means exactly these seven.
Besides them, three helper programs run: the vendor TEE helper, and the two
Wi-Fi programs. Nothing else runs on the TV except PID 1 itself. -/

inductive App where
  | youtube | doom | slides | rpg | flight | axiom | snake
  deriving DecidableEq

def App.all : List App := [.youtube, .doom, .slides, .rpg, .flight, .axiom, .snake]

/-- The number each app has always had: in stored settings, in its UID, and
    in calls to native code. 4 belonged to a retired app. -/
def App.id : App → Nat
  | .youtube => 1 | .doom => 2 | .slides => 3 | .rpg => 5 | .flight => 6
  | .axiom => 7 | .snake => 8

def App.ofId : Nat → Option App
  | 1 => some .youtube | 2 => some .doom | 3 => some .slides
  | 5 => some .rpg     | 6 => some .flight
  | 7 => some .axiom   | 8 => some .snake | _ => none

/-- The helper programs. Their sandboxes are fixed (section 4); Settings
    cannot change them. -/
inductive Daemon where
  /-- The vendor TEE helper: answers the secure world's requests to load
      trusted apps and to use the eMMC's protected block (RPMB). -/
  | tee
  /-- wpa_supplicant: joins the saved Wi-Fi network. -/
  | wifi
  /-- udhcpc and its lease script: gets an address on that network. -/
  | dhcp
  deriving DecidableEq

def Daemon.all : List Daemon := [.tee, .wifi, .dhcp]

/-- Every process on the TV except PID 1. -/
inductive Subject where
  | app (a : App)
  | daemon (d : Daemon)
  deriving DecidableEq

/-- Development builds also contain a root console, a recovery HTTP endpoint
    and an Ethernet debug link. Those bypass everything in this file, so the
    guarantees are for production builds (section 11). -/
inductive Build where
  | production | development
  deriving DecidableEq

/-! ## 2. Capabilities

Each app has a manifest, fixed when the TV is built: the most it can ever
have. The user can switch its capabilities off and on again in Settings, but
never beyond the manifest. -/

/-- A host an app may reach. -/
inductive Host where
  /-- Exactly this name or address, e.g. `name "192.168.77.1"`. -/
  | name (n : String)
  /-- Any name under this domain, e.g. `under "googlevideo.com"` matches
      `r3---sn-abc.googlevideo.com` but not `googlevideo.com.evil`. -/
  | under (domain : String)
  deriving DecidableEq

def Host.matches : Host → String → Bool
  | .name n,       host => host == n
  | .under domain, host => host.endsWith ("." ++ domain)

/-- What the user can switch on and off for each app. -/
inductive Capability where
  /-- Connections to the app's hosts, which PID 1 opens for it. The app
      itself cannot open a socket. -/
  | internet
  /-- Its own sockets, to addresses on the home network only: casting,
      LAN games. -/
  | localNetwork
  /-- A data directory that survives the app closing. Without it the app
      gets a fresh, empty one at every launch. -/
  | savedData
  deriving DecidableEq

def Capability.all : List Capability := [.internet, .localNetwork, .savedData]

def Capability.isNetwork : Capability → Bool
  | .internet | .localNetwork => true
  | .savedData                => false

/-- On or off, for each capability. -/
structure Grants where
  internet     : Bool
  localNetwork : Bool
  savedData    : Bool
  deriving DecidableEq

def Grants.has (g : Grants) : Capability → Bool
  | .internet     => g.internet
  | .localNetwork => g.localNetwork
  | .savedData    => g.savedData

def Grants.set (g : Grants) (c : Capability) (on : Bool) : Grants :=
  match c with
  | .internet     => { g with internet := on }
  | .localNetwork => { g with localNetwork := on }
  | .savedData    => { g with savedData := on }

def Grants.everything : Grants := ⟨true, true, true⟩

/-- `g` gives nothing that `g'` does not. -/
def Grants.within (g g' : Grants) : Prop :=
  ∀ c, g.has c = true → g'.has c = true

structure Manifest where
  uid     : Nat
  threads : Nat
  /-- Where `internet` may reach. Empty: the app does not have `internet`. -/
  hosts   : List Host
  /-- The app may have `localNetwork`. -/
  lan     : Bool
  /-- The app may have `savedData`. -/
  saves   : Bool
  /-- The app may ask PID 1 for a TEE session. -/
  tee     : Bool

def Manifest.declares (m : Manifest) : Capability → Bool
  | .internet     => !m.hosts.isEmpty
  | .localNetwork => m.lan
  | .savedData    => m.saves

/-- Base of the app UIDs. The G13 build replaces 10000 with 19000; the
    theorems hold for any base. -/
def uidBase : Nat := 10000

/-- Where YouTube may connect: its API, thumbnails and video servers, plus the
    development host (192.168.77.1: the replay server in QEMU, the proxy on
    the TV). -/
def youtubeHosts : List Host :=
  [.name "www.youtube.com", .name "i.ytimg.com", .under "googlevideo.com", .name "192.168.77.1"]

def manifest : App → Manifest
  | .youtube => { uid := uidBase + 1, threads := 32, hosts := youtubeHosts, lan := true,  saves := false, tee := false }
  | .doom    => { uid := uidBase + 2, threads := 8,  hosts := [],           lan := true,  saves := true,  tee := false }
  | .slides  => { uid := uidBase + 3, threads := 1,  hosts := [],           lan := false, saves := false, tee := false }
  | .rpg     => { uid := uidBase + 5, threads := 1,  hosts := [],           lan := false, saves := true,  tee := false }
  | .flight  => { uid := uidBase + 6, threads := 4,  hosts := [],           lan := false, saves := false, tee := false }
  | .axiom   => { uid := uidBase + 7, threads := 4,  hosts := [],           lan := false, saves := false, tee := false }
  | .snake   => { uid := uidBase + 8, threads := 1,  hosts := [],           lan := false, saves := false, tee := false }

/-! ## 3. Policy

The parental policy and the user's settings. Only PID 1 holds them. Apps
never see them, and PID 1's own menus never see the PIN. -/

/-- A four-digit PIN. `Fin 10000` is the numbers 0 to 9999, so a PIN outside
    0000–9999 cannot exist. -/
abbrev Pin := Fin 10000

structure Policy where
  pin     : Pin
  enabled : App → Bool
  /-- What the user has left switched on, for each app. Only capabilities
      in the app's manifest count (`effective`). -/
  grants  : App → Grants
  /-- The user asked for the TV to be offline. -/
  offline : Bool

/-- Factory settings: PIN 0000, every app enabled, everything switched on,
    online. -/
def Policy.initial : Policy :=
  { pin := 0, enabled := fun _ => true, grants := fun _ => .everything, offline := false }

/-- Set one app's enabled flag. Nothing else changes. -/
def Policy.allow (p : Policy) (a : App) (on : Bool) : Policy :=
  { p with enabled := fun b => if b = a then on else p.enabled b }

/-- Switch one capability of one app on or off. Nothing else changes. -/
def Policy.grant (p : Policy) (a : App) (c : Capability) (on : Bool) : Policy :=
  { p with grants := fun b => if b = a then (p.grants b).set c on else p.grants b }

/-- What an app actually gets: in its manifest, switched on, and, for the two
    network capabilities, the TV not offline. -/
def effective (p : Policy) (a : App) : Grants :=
  let on (c : Capability) := (manifest a).declares c && (p.grants a).has c && !(p.offline && c.isNetwork)
  { internet := on .internet, localNetwork := on .localNetwork, savedData := on .savedData }

/-- Something an app asks PID 1 for, over its PID 1 socket. -/
inductive Request where
  /-- A TCP connection to `host`, opened by PID 1 and handed to the app. -/
  | connect (host : String) (port : Nat)
  | teeSession

/-- Whether the policy lets PID 1 grant this request to this app. -/
def Policy.allows (p : Policy) (a : App) : Request → Bool
  | .connect host _ => (effective p a).internet && (manifest a).hosts.any (·.matches host)
  | .teeSession     => (manifest a).tee

/-! ## 4. Confinement

What Linux enforces for every process except PID 1, as data. `confine` gives
each one's row. PID 1's launcher (native code, section 10) builds exactly
that sandbox before the program starts, in a fixed order: a private
filesystem view, its devices, its network, its limits, its user and Linux
capabilities, then its system-call filter, then the program. -/

/-- The Linux capabilities a process may hold. This type is the whole list:
    CAP_SYS_ADMIN, CAP_SYS_MODULE, CAP_SETUID, CAP_SYS_PTRACE and the rest
    cannot be written down here, so nothing can be given them. -/
inductive LinuxCap where
  /-- Configure network interfaces and routes. -/
  | netAdmin
  /-- Open packet and raw sockets. -/
  | netRaw
  /-- Bind ports below 1024. -/
  | netBindService
  /-- Send raw commands to the eMMC. -/
  | sysRawio
  deriving DecidableEq

/-- What a mount shows. -/
inductive Source where
  /-- This path from PID 1's filesystem. -/
  | outside (path : String)
  /-- A new, empty directory, gone when the process ends. -/
  | fresh
  /-- A read-only copy of the kernel command line, made at launch. -/
  | cmdline
  /-- The shared frame buffer PID 1 made for this launch. -/
  | frames
  /-- A process list that shows only the process itself. -/
  | ownProcesses
  deriving DecidableEq

/-- One thing a process can see. Everything not mounted does not exist for it. -/
structure Mount where
  path     : String
  source   : Source
  writable : Bool
  exec     : Bool
  deriving DecidableEq

/-- A device node it can open. No others exist in its `/dev`. -/
structure Device where
  path     : String
  writable : Bool
  deriving DecidableEq

/-- A kind of socket, by the numbers the kernel uses. -/
structure SocketKind where
  family   : Nat
  type     : Nat
  protocol : Nat
  deriving DecidableEq

/-- Its network. The parts are independent: YouTube can have both `broker`
    and `lan`. A process with no `lan` and no `sockets` gets its own empty
    network: no interfaces, nowhere to send a packet. -/
structure Net where
  /-- Hosts PID 1 connects to for it (`Request.connect`). -/
  broker  : List Host
  /-- Its own sockets, to home-network addresses only. -/
  lan     : Bool
  /-- For the network daemons: exactly these kinds of socket. -/
  sockets : List SocketKind

def Net.none : Net := ⟨[], false, []⟩

/-- What it may run after its own program starts. -/
inductive Exec where
  /-- Nothing. It cannot make any memory executable either. -/
  | none
  /-- Its program is dynamically linked: the loader may map these libraries
      executable, and nothing writable may ever be executable. -/
  | loader (libraries : List String)
  /-- It may run these programs, and nothing else. -/
  | programs (paths : List String)

structure Limits where
  threads   : Nat
  memoryMB  : Nat
  openFiles : Nat
  deriving DecidableEq

structure Confinement where
  uid     : Nat
  gid     : Nat
  caps    : List LinuxCap
  mounts  : List Mount
  devices : List Device
  /-- The only ioctl requests its filter allows. -/
  ioctls  : List Nat
  net     : Net
  exec    : Exec
  limits  : Limits

/-- How apps reach the screen and speakers: their own device nodes (QEMU), or
    a shared frame buffer and PID 1's audio socket (the TV). -/
inductive Display where
  | devices | sharedFrames
  deriving DecidableEq

/-- Where an app's data lives. -/
def dataDir (a : App) : String := "/data/apps/" ++ toString (manifest a).uid

def appMounts (a : App) (saves : Bool) (d : Display) : List Mount :=
  [ ⟨"/proc/cmdline", .cmdline, false, false⟩,
    ⟨"/share", .outside "/share", false, false⟩,
    ⟨"/tmp", .fresh, true, false⟩,
    ⟨dataDir a, if saves then .outside (dataDir a) else .fresh, true, false⟩ ] ++
  (if a = .doom then [⟨"/doom", .outside "/doom", false, false⟩] else []) ++
  (if d = .sharedFrames then [⟨"/run/frames", .frames, true, false⟩] else [])

def appDevices : Display → List Device
  | .devices      => [⟨"/dev/null", true⟩, ⟨"/dev/urandom", false⟩, ⟨"/dev/fb0", true⟩, ⟨"/dev/snd", true⟩]
  | .sharedFrames => [⟨"/dev/null", true⟩, ⟨"/dev/urandom", false⟩]

/-- The framebuffer and PCM ioctls of the platform library (32-bit ARM). -/
def displayIoctls : List Nat :=
  [0x4600, 0x4602, 0x4606, 0x40044620,                         -- framebuffer
   0xc25c4111, 0x4140, 0x4144, 0x4143, 0x80044121, 0x400c4150, -- PCM
   0x80804506, 0x5413]                                         -- device name, terminal size

/-- An app's sandbox. It applies the manifest again, so even grants beyond
    the manifest would give nothing more. -/
def appRow (a : App) (g : Grants) (d : Display) : Confinement :=
  let m := manifest a
  let on (c : Capability) := g.has c && m.declares c
  { uid := m.uid, gid := m.uid, caps := [],
    mounts  := appMounts a (on .savedData) d,
    devices := appDevices d,
    ioctls  := if d = .devices then displayIoctls else [0x5413],
    net     := { broker := if on .internet then m.hosts else [], lan := on .localNetwork, sockets := [] },
    exec    := .none,
    limits  := { threads := m.threads, memoryMB := 768, openFiles := 256 } }

/-- The TEE helper, from its trace on the G06 (PR 17). It needs no network and
    writes only its own store. `sysRawio` is for one eMMC command that sizes
    the RPMB block; drop it if the TV starts without it (section 11). -/
def teeRow : Confinement :=
  { uid := 9001, gid := 9001, caps := [.sysRawio],
    mounts := [ ⟨"/system", .outside "/system", false, true⟩,
                ⟨"/stock-system", .outside "/stock-system", false, true⟩,
                ⟨"/vendor", .outside "/vendor", false, true⟩,
                ⟨"/config", .outside "/config", false, false⟩,
                ⟨"/vendor/tvcertificate/tee", .outside "/vendor/tvcertificate/tee", true, false⟩,
                ⟨"/mnt/vendor/tvservice", .outside "/mnt/vendor/tvservice", false, false⟩,
                ⟨"/dev/__properties__", .outside "/dev/__properties__", false, false⟩,
                ⟨"/sys/class/mmc_host/mmc0/mmc0:0001/cid",
                  .outside "/sys/class/mmc_host/mmc0/mmc0:0001/cid", false, false⟩,
                ⟨"/proc", .ownProcesses, false, false⟩ ],
    devices := [ ⟨"/dev/tee0", true⟩, ⟨"/dev/teepriv0", true⟩, ⟨"/dev/mmcblk0rpmb", true⟩,
                 ⟨"/dev/mmcblk0", false⟩, ⟨"/dev/null", true⟩, ⟨"/dev/urandom", false⟩ ],
    ioctls := [ 0x800ca400, 0xc010a401, 0x8010a402, 0x8010a403, 0x8004a405,  -- TEE client
                0x8010a406, 0x8010a407,                                      -- TEE helper
                0xc008b301, 0xc048b300 ],                                    -- RPMB, one MMC command
    net := .none,
    exec := .loader [ "/stock-system/system/apex/com.android.runtime/lib/bionic/libc.so",
                      "/stock-system/system/apex/com.android.runtime/lib/bionic/libdl.so",
                      "/stock-system/system/apex/com.android.runtime/lib/bionic/libm.so",
                      "/system/lib/libc++.so", "/system/lib/libnetd_client.so",
                      "/vendor/lib/libteec.so" ],
    limits := { threads := 8, memoryMB := 64, openFiles := 64 } }

/-- wpa_supplicant, from its trace. -/
def wifiRow : Confinement :=
  { uid := 9002, gid := 9002, caps := [.netAdmin, .netRaw],
    mounts := [ ⟨"/run/tv-network/wifi", .outside "/run/tv-network/wifi", true, false⟩,
                ⟨"/sys", .outside "/sys", false, false⟩ ],
    devices := [ ⟨"/dev/null", true⟩, ⟨"/dev/urandom", false⟩, ⟨"/dev/random", false⟩ ],
    ioctls := [ 0x8913, 0x8914, 0x8915, 0x8927, 0x8933, 0x5411, 0x5413,          -- interfaces
                0x8b04, 0x8b06, 0x8b07, 0x8b0b, 0x8b14, 0x8b15, 0x8b16, 0x8b18, -- wireless
                0x8b19, 0x8b1a, 0x8b1b, 0x8b30, 0x8b32, 0x8b34, 0x8b36 ],
    net := { broker := [], lan := false,
             sockets := [ ⟨2, 2, 0⟩,          -- IPv4 datagram, for interface ioctls
                          ⟨16, 3, 0⟩,         -- route netlink
                          ⟨1, 2, 0⟩,          -- its control socket
                          ⟨17, 2, 0x8e88⟩ ] }, -- EAPOL packets
    exec := .none,
    limits := { threads := 1, memoryMB := 32, openFiles := 64 } }

/-- udhcpc and its lease script, from its trace. -/
def dhcpRow : Confinement :=
  { uid := 9003, gid := 9003, caps := [.netAdmin, .netRaw, .netBindService],
    mounts := [ ⟨"/run/tv-network/dhcp", .outside "/run/tv-network/dhcp", true, false⟩,
                ⟨"/run/tv-network/lease.sh", .outside "/run/tv-network/lease.sh", false, true⟩,
                ⟨"/bin", .outside "/bin", false, true⟩,
                ⟨"/proc", .ownProcesses, false, false⟩ ],
    devices := [ ⟨"/dev/null", true⟩, ⟨"/dev/urandom", false⟩ ],
    ioctls := [ 0x8913, 0x8914, 0x8916, 0x891c, 0x8927, 0x8933 ],
    net := { broker := [], lan := false,
             sockets := [ ⟨17, 2, 8⟩,         -- IP packets, before it has an address
                          ⟨2, 3, 255⟩,        -- raw IP
                          ⟨2, 2, 0⟩, ⟨2, 2, 17⟩, -- UDP, and interface ioctls
                          ⟨16, 3, 0⟩ ] },     -- route netlink
    exec := .programs ["/bin/busybox", "/run/tv-network/lease.sh"],
    limits := { threads := 1, memoryMB := 32, openFiles := 64 } }

def daemonRow : Daemon → Confinement
  | .tee  => teeRow
  | .wifi => wifiRow
  | .dhcp => dhcpRow

/-- Every process's sandbox, given the policy and how apps reach the screen. -/
def confine (p : Policy) (d : Display) : Subject → Confinement
  | .app a    => appRow a (effective p a) d
  | .daemon x => daemonRow x

/-- `c` gives nothing that `c'` does not. -/
def Confinement.within (c c' : Confinement) : Prop :=
  (∀ x ∈ c.caps, x ∈ c'.caps) ∧
  (∀ m ∈ c.mounts, m.source = .fresh ∨ m ∈ c'.mounts) ∧
  (∀ v ∈ c.devices, v ∈ c'.devices) ∧
  (∀ h ∈ c.net.broker, h ∈ c'.net.broker) ∧
  (c.net.lan = true → c'.net.lan = true) ∧
  (∀ k ∈ c.net.sockets, k ∈ c'.net.sockets)

/-- Kernel settings PID 1 makes before anything else runs, and that cannot be
    undone without a reboot: no more kernel modules, no kexec, no kernel
    addresses, logs or profiling for anyone else, hardened BPF, links and
    core dumps, and a quieter network stack. -/
def kernelLock : List (String × String) :=
  [ ("kernel/modules_disabled", "1"), ("kernel/kexec_load_disabled", "1"),
    ("kernel/kptr_restrict", "2"), ("kernel/dmesg_restrict", "1"),
    ("kernel/perf_event_paranoid", "3"), ("kernel/unprivileged_bpf_disabled", "1"),
    ("net/core/bpf_jit_harden", "2"), ("kernel/sysrq", "0"), ("fs/suid_dumpable", "0"),
    ("fs/protected_symlinks", "1"), ("fs/protected_hardlinks", "1"),
    ("fs/protected_fifos", "2"), ("fs/protected_regular", "2"),
    ("net/ipv4/conf/all/accept_redirects", "0"), ("net/ipv4/conf/all/send_redirects", "0"),
    ("net/ipv4/conf/all/accept_source_route", "0"), ("net/ipv4/conf/all/rp_filter", "1"),
    ("net/ipv4/icmp_echo_ignore_broadcasts", "1"),
    ("net/ipv6/conf/all/accept_redirects", "0"), ("net/ipv6/conf/all/accept_source_route", "0") ]

/-- How the TV starts, in order. Every kernel module is loaded first, then the
    kernel is locked, and only then does any other process start. -/
inductive BootStep where
  | loadModules | lockKernel | startTee | startMedia | startLauncher
  deriving DecidableEq

def bootPlan : List BootStep := [.loadModules, .lockKernel, .startTee, .startMedia, .startLauncher]

/-- `x` comes before `y` in the list. -/
def before {α} [DecidableEq α] (x y : α) : List α → Bool
  | []      => false
  | z :: zs => if z = x then zs.contains y else if z = y then false else before x y zs

/-! ## 5. State -/

/-- Monotonic time in nanoseconds. -/
abbrev Time := Nat

/-- How long an app has to exit after TERM before PID 1 sends KILL. -/
def grace : Time := 1500000000

/-- How long PID 1 waits after KILL before it decides the app is stuck. -/
def stuckGrace : Time := 5000000000

/-- Every app start gets a new number. Requests and exits name the start they
    belong to, so nothing from a closed launch can reach a later one. -/
abbrev Launch := Nat

/-- Every network change gets a new number, and its confirmation must name it. -/
abbrev NetOp := Nat

/-- Who has the picture and the remote. -/
inductive Screen where
  /-- PID 1's own menus: the launcher, Settings, About. -/
  | home
  /-- An app. It is shown and gets every key except Home and Power. -/
  | app (a : App)
  deriving DecidableEq

/-- How far closing an app has got. -/
inductive Closing where
  /-- TERM sent. -/
  | asked
  /-- KILL sent. -/
  | killed
  /-- KILL did not work. The menus ask: restart the TV, or keep waiting. -/
  | stuck
  /-- The user chose to keep waiting. No app can start until it exits. -/
  | waiting
  deriving DecidableEq

/-- An app that has been asked to stop and has not exited yet. -/
structure Stopping where
  app      : App
  launch   : Launch
  deadline : Time
  stage    : Closing

/-- The network as it actually is, which can lag behind what the policy asks
    for: a change is only done once native code confirms it. -/
inductive NetState where
  | online
  | goingOffline (op : NetOp)
  | offline
  | goingOnline (op : NetOp)
  /-- A change could not be confirmed. The menus offer a restart. -/
  | failed
  deriving DecidableEq

structure State where
  policy   : Policy
  screen   : Screen
  stopping : Option Stopping
  clock    : Time
  /-- The number of the latest app start. -/
  launch   : Launch
  net      : NetState
  /-- The number the next network change will get. -/
  nextOp   : NetOp

/-- The app that is running, if any: the one in front, or the one closing. -/
def State.alive (s : State) : Option App :=
  match s.screen with
  | .app a => some a
  | .home  => s.stopping.map (·.app)

/-- The restart question is showing: an app would not close, or a network
    change could not be confirmed. -/
def State.restartOffered (s : State) : Bool :=
  s.net == .failed || s.stopping.any (fun st => st.stage == .stuck || st.stage == .waiting)

/-- PID 1 is waiting for native code to confirm network change `op`, going up
    (`up`) or down. -/
def State.awaits (s : State) (op : NetOp) (up : Bool) : Bool :=
  match s.net with
  | .goingOffline o => o == op && !up
  | .goingOnline o  => o == op && up && !s.policy.offline
  | _               => false

/-- The state at startup, with the policy read from storage. If it says
    offline, the network was never brought up. -/
def State.boot (saved : Policy) : State :=
  { policy := saved, screen := .home, stopping := none, clock := 0, launch := 0,
    net := if saved.offline then .offline else .goingOnline 0, nextOp := 1 }

/-! ## 6. Inputs -/

inductive Key where
  | home
  | power
  /-- Any other remote key, by Linux key code. -/
  | other (code : Nat)

/-- What the user picked in PID 1's own menus. The menus turn key presses into
    these. They never see the PIN: choices carry the digits the user typed,
    and `step` compares them with the PIN. -/
inductive Choice where
  | launch (a : App)
  | changePin (typed : Pin) (new : Pin)
  | disable (a : App)
  | enable (a : App) (typed : Pin)
  /-- Switch a capability on for an app. -/
  | grant (a : App) (c : Capability) (typed : Pin)
  /-- Switch it off. -/
  | revoke (a : App) (c : Capability)
  | goOffline
  | goOnline (typed : Pin)
  /-- Save the Wi-Fi network the user entered in the menus. -/
  | joinWifi (typed : Pin)
  /-- Answer to the restart question. -/
  | restart
  | keepWaiting
  | powerOff

inductive Input where
  /-- A key press. `held` is true for auto-repeat while the key is held down. -/
  | key (k : Key) (held : Bool)
  /-- Key-up for an ordinary key. Home, Power and volume releases are ignored. -/
  | release (code : Nat)
  /-- A choice made in PID 1's own menus. -/
  | choose (c : Choice)
  /-- The clock. Arrives at least every 20 ms. -/
  | tick (now : Time)
  /-- Launch `l` of the app has ended completely: its process is reaped,
      nothing runs as its user, its cgroup is empty, and PID 1 has no
      connection work left for it. A failed start is reported as an exit. -/
  | exited (a : App) (l : Launch)
  /-- A request from launch `l` of the app. PID 1 knows the sender from the
      socket the request arrived on; the app does not get to say who it is. -/
  | ask (a : App) (l : Launch) (r : Request)
  /-- Native code reports on network change `op` (going up if `up`): every
      layer was checked and is in place (`ok`), or not. -/
  | netDone (op : NetOp) (up ok : Bool)

/-! ## 7. Outputs

Every operation PID 1 performs on the outside world is one of these. Section
10 carries them out; section 11 says what each must do for the theorems to
mean what they say. -/

inductive Signal where
  | term | kill

inductive Notice where
  | wrongPin
  | disabled (a : App)
  | stillClosing (a : App)
  | exited (a : App)
  /-- `a` did not close after KILL. The menus ask whether to restart. -/
  | stuck (a : App)
  /-- The network is changing, or has changed. -/
  | network (n : NetState)

inductive NetDaemon where
  | wifi | dhcp

inductive Output where
  /-- Give `s` the picture and the remote. -/
  | show (s : Screen)
  /-- Pass a key to PID 1's own menus. -/
  | toMenu (k : Key) (held : Bool)
  /-- Pass an ordinary key press or repeat to an app. -/
  | forward (a : App) (code : Nat) (held : Bool)
  /-- Key-up for an ordinary app key, preserving press/release pairs. -/
  | release (a : App) (code : Nat)
  /-- Start launch `l` of the app, in the sandbox `appRow a g`. -/
  | start (a : App) (l : Launch) (g : Grants)
  | signal (a : App) (s : Signal)
  /-- Answer a request from launch `l`. -/
  | reply (a : App) (l : Launch) (r : Request) (granted : Bool)
  /-- Write the policy to storage. -/
  | save (p : Policy)
  /-- Show a message in PID 1's menus. -/
  | tell (n : Notice)
  /-- Bring the network up or down, every layer (section 11), and report back
      with `netDone op`. -/
  | network (op : NetOp) (up : Bool)
  /-- Start or stop a Wi-Fi daemon, in its sandbox. -/
  | daemon (d : NetDaemon) (start : Bool)
  /-- Write the Wi-Fi network the menus hold. -/
  | saveWifi
  | reboot
  | powerOff

/-! ## 8. The machine -/

/-- Home: PID 1's screen comes to the front at once, then the app is asked to
    stop. In PID 1's own menus, Home goes back to the top of the launcher. -/
def onHome (s : State) (held : Bool) : State × List Output :=
  match s.screen with
  | .app a => ({ s with screen := .home, stopping := some ⟨a, s.launch, s.clock + grace, .asked⟩ },
               [.show .home, .signal a .term])
  | .home  => (s, [.toMenu .home held])

/-- Linux codes owned globally by PID 1, or values outside the native word. -/
def reservedKey (code : Nat) : Bool :=
  code > 0xffffffff || code == 102 || code == 172 || code == 116 ||
  code == 113 || code == 114 || code == 115

/-- Any ordinary key goes to whoever is in front. Reject reserved raw codes
    too: a misclassified `Key.other` must not bypass Home/Power ownership. -/
def onKey (s : State) (code : Nat) (held : Bool) : State × List Output :=
  if reservedKey code then (s, []) else
  match s.screen with
  | .app a => (s, [.forward a code held])
  | .home  => (s, [.toMenu (.other code) held])

/-- Releases reach only the foreground app; menus act on presses/repeats. -/
def onRelease (s : State) (code : Nat) : State × List Output :=
  if reservedKey code then (s, []) else
  match s.screen with
  | .app a => (s, [.release a code])
  | .home => (s, [])

/-- The clock: KILL an app that has not exited by its deadline, and if KILL
    does not work either, ask the user whether to restart. -/
def onTick (s : State) (now : Time) : State × List Output :=
  let s := { s with clock := now }
  match s.stopping with
  | some st =>
    if now > st.deadline then
      match st.stage with
      | .asked  => ({ s with stopping := some { st with stage := .killed, deadline := now + stuckGrace } },
                    [.signal st.app .kill])
      | .killed => ({ s with stopping := some { st with stage := .stuck } }, [.tell (.stuck st.app)])
      | _       => (s, [])
    else (s, [])
  | none => (s, [])

/-- Launch `l` of an app exited. If it was in front, PID 1's screen comes
    back. If it was closing, PID 1's screen is drawn again, in case the app
    drew over it while closing. An exit from an earlier launch changes
    nothing. -/
def onExit (s : State) (a : App) (l : Launch) : State × List Output :=
  if l ≠ s.launch then (s, []) else
  if s.screen = .app a then ({ s with screen := .home }, [.show .home, .tell (.exited a)])
  else match s.stopping with
    | some st => if st.app = a then ({ s with stopping := none }, [.show .home]) else (s, [])
    | none    => (s, [])

/-- Keep a new policy and write it to storage. -/
def commit (s : State) (p : Policy) : State × List Output :=
  ({ s with policy := p }, [.save p])

/-- A network change is in progress. -/
def NetState.changing : NetState → Bool
  | .goingOffline _ | .goingOnline _ => true
  | _ => false

/-- A choice from PID 1's menus. `step` only calls this while PID 1's screen
    is in front. -/
def onChoice (s : State) : Choice → State × List Output
  | .launch a =>
    if s.policy.enabled a = false then (s, [.tell (.disabled a)])
    else match s.stopping with
      | some st => (s, [.tell (.stillClosing st.app)])
      | none =>
        if s.net.changing then (s, [.tell (.network s.net)])
        else ({ s with screen := .app a, launch := s.launch + 1 },
              [.start a (s.launch + 1) (effective s.policy a), .show (.app a)])
  | .changePin typed new =>
    if typed = s.policy.pin then commit s { s.policy with pin := new }
    else (s, [.tell .wrongPin])
  | .disable a => commit s (s.policy.allow a false)
  | .enable a typed =>
    if typed = s.policy.pin then commit s (s.policy.allow a true)
    else (s, [.tell .wrongPin])
  | .grant a c typed =>
    if typed = s.policy.pin then commit s (s.policy.grant a c true)
    else (s, [.tell .wrongPin])
  | .revoke a c =>
    match s.stopping with
    | some st => (s, [.tell (.stillClosing st.app)])
    | none    => commit s (s.policy.grant a c false)
  | .goOffline =>
    if s.policy.offline then (s, []) else
    let p := { s.policy with offline := true }
    ({ s with policy := p, net := .goingOffline s.nextOp, nextOp := s.nextOp + 1 },
     [.save p, .daemon .wifi false, .daemon .dhcp false, .network s.nextOp false,
      .tell (.network (.goingOffline s.nextOp))])
  | .goOnline typed =>
    if typed ≠ s.policy.pin then (s, [.tell .wrongPin]) else
    if !s.policy.offline then (s, []) else
    let p := { s.policy with offline := false }
    ({ s with policy := p, net := .goingOnline s.nextOp, nextOp := s.nextOp + 1 },
     [.save p, .network s.nextOp true, .tell (.network (.goingOnline s.nextOp))])
  | .joinWifi typed =>
    if typed ≠ s.policy.pin then (s, [.tell .wrongPin]) else
    if !s.policy.offline && s.net == .online then
      (s, [.saveWifi, .daemon .wifi false, .daemon .dhcp false, .daemon .wifi true, .daemon .dhcp true])
    else (s, [.saveWifi])
  | .restart => if s.restartOffered then (s, [.reboot]) else (s, [])
  | .keepWaiting =>
    match s.stopping with
    | some st => if st.stage = .stuck then ({ s with stopping := some { st with stage := .waiting } }, [.show .home])
                 else (s, [])
    | none    => (s, [])
  | .powerOff => (s, [.powerOff])

/-- A request from an app: granted only if it comes from the current launch,
    the app is in front, and the policy allows it. -/
def onAsk (s : State) (a : App) (l : Launch) (r : Request) : State × List Output :=
  (s, [.reply a l r (decide (l = s.launch) && decide (s.screen = .app a) && s.policy.allows a r)])

/-- Native code reports on a network change. Only the change PID 1 is waiting
    for counts; anything else (an old change, a repeat, a report nobody asked
    for) changes nothing. -/
def onNetDone (s : State) (op : NetOp) (up ok : Bool) : State × List Output :=
  if !s.awaits op up then (s, []) else
  if !ok then ({ s with net := .failed }, [.tell (.network .failed)]) else
  if up then ({ s with net := .online }, [.daemon .wifi true, .daemon .dhcp true, .tell (.network .online)])
  else ({ s with net := .offline }, [.tell (.network .offline)])

def step (s : State) : Input → State × List Output
  | .key .power _           => (s, [.powerOff])
  | .key .home held         => onHome s held
  | .key (.other code) held => onKey s code held
  | .release code           => onRelease s code
  | .choose c               => if s.screen = .home then onChoice s c else (s, [])
  | .tick now               => onTick s now
  | .exited a l             => onExit s a l
  | .ask a l r              => onAsk s a l r
  | .netDone op up ok       => onNetDone s op up ok

/-- Feed a list of inputs through `step` and return the final state. -/
def run (s : State) : List Input → State
  | []      => s
  | i :: is => run (step s i).1 is

/-- Everything a run does, in order. -/
def runOutputs (s : State) : List Input → List Output
  | []      => []
  | i :: is => (step s i).2 ++ runOutputs (step s i).1 is

/-- What PID 1 does at startup, before the first input. -/
def bootOutputs (saved : Policy) : List Output :=
  if saved.offline then [] else [.network 0 true]

/-! ## 9. Theorems

Each theorem is about `step` or about the tables above. PID 1 changes only by
steps, so a theorem about every step holds for every run;
`pinNeverChangesWithoutPin` and `bootOfflineStaysOfflineUntilThePin` spell
this out. -/

/-! ### The remote -/

/-- Power always powers off, whatever is running. -/
theorem powerAlwaysPowersOff :
    ∀ (s : State) (held : Bool),
      .powerOff ∈ (step s (.key .power held)).2 := by
  cases_on_step

/-- Home always brings PID 1's screen to the front, whatever is running. -/
theorem homeAlwaysGoesHome :
    ∀ (s : State) (held : Bool),
      (step s (.key .home held)).1.screen = .home := by
  cases_on_step

/-- When an app is in front, Home first takes the picture and the remote away
    from it, and only then asks it to stop. -/
theorem homeHidesAppBeforeStoppingIt :
    ∀ (s : State) (a : App) (held : Bool),
      s.screen = .app a →
      (step s (.key .home held)).2 = [.show .home, .signal a .term] := by
  cases_on_step

/-- An app that ignores TERM gets KILL on the first tick after its deadline. -/
theorem stuckAppsAreKilled :
    ∀ (s : State) (st : Stopping) (now : Time),
      s.stopping = some st → st.stage = .asked → now > st.deadline →
      .signal st.app .kill ∈ (step s (.tick now)).2 := by
  cases_on_step

/-- An app gets a key only while it is in front, and only the key just
    pressed. (Home and Power cannot be forwarded at all.) -/
theorem keysReachOnlyTheFrontApp :
    ∀ (s : State) (i : Input) (a : App) (code : Nat) (held : Bool),
      .forward a code held ∈ (step s i).2 →
      s.screen = .app a ∧ i = .key (.other code) held := by
  cases_on_step

/-- A key-up reaches only the foreground app, for the released key. -/
theorem releasesReachOnlyTheFrontApp :
    ∀ (s : State) (i : Input) (a : App) (code : Nat),
      .release a code ∈ (step s i).2 →
      s.screen = .app a ∧ i = .release code := by
  cases_on_step

/-- Home, Power and volume cannot be forwarded as numeric app key presses. -/
theorem reservedPressesNeverReachApps :
    ∀ (s : State) (i : Input) (a : App) (code : Nat) (held : Bool),
      reservedKey code = true → .forward a code held ∉ (step s i).2 := by
  cases_on_step

/-- Their releases cannot reach apps either. -/
theorem reservedReleasesNeverReachApps :
    ∀ (s : State) (i : Input) (a : App) (code : Nat),
      reservedKey code = true → .release a code ∉ (step s i).2 := by
  cases_on_step

/-! ### Starting and closing apps -/

/-- An app starts only when the user launches it from PID 1's screen, it is
    enabled, no other app is still running, and the network is not changing. -/
theorem onlyTheUserStartsApps :
    ∀ (s : State) (i : Input) (a : App) (l : Launch) (g : Grants),
      .start a l g ∈ (step s i).2 →
      i = .choose (.launch a) ∧ s.screen = .home ∧ s.policy.enabled a = true ∧
      s.stopping = none ∧ s.net.changing = false := by
  cases_on_step

/-- A disabled app cannot start. -/
theorem disabledAppsCannotStart :
    ∀ (s : State) (i : Input) (a : App) (l : Launch) (g : Grants),
      s.policy.enabled a = false → .start a l g ∉ (step s i).2 := by
  cases_on_step

/-- Nothing else is running when an app starts. -/
theorem nothingRunsWhenAnAppStarts :
    ∀ (s : State) (i : Input) (a : App) (l : Launch) (g : Grants),
      .start a l g ∈ (step s i).2 → s.alive = none := by
  cases_on_step

/-- A running app stops counting as running only when native code reports
    that its launch has ended completely. Together with the last theorem: the
    previous app is fully closed before the next one starts. -/
theorem appsStopRunningOnlyByExiting :
    ∀ (s : State) (i : Input) (a : App),
      s.alive = some a → (step s i).1.alive = none → i = .exited a s.launch := by
  cases_on_step

/-- Every start gets a new launch number. -/
theorem launchNumbersAreNew :
    ∀ (s : State) (i : Input) (a : App) (l : Launch) (g : Grants),
      .start a l g ∈ (step s i).2 → l = s.launch + 1 ∧ (step s i).1.launch = l := by
  cases_on_step

/-- An exit from an earlier launch changes nothing. -/
theorem oldExitsChangeNothing :
    ∀ (s : State) (a : App) (l : Launch),
      l ≠ s.launch → step s (.exited a l) = (s, []) := by
  cases_on_step

/-- An app is in front only if it already was, or the user just launched it
    from PID 1's screen and it is enabled. -/
theorem onlyLaunchedAppsAreInFront :
    ∀ (s : State) (i : Input) (a : App),
      (step s i).1.screen = .app a →
      s.screen = .app a ∨
      (i = .choose (.launch a) ∧ s.screen = .home ∧ s.policy.enabled a = true) := by
  cases_on_step

/-- The restart question appears only after KILL failed to close an app. -/
theorem stuckOnlyAfterKill :
    ∀ (s : State) (i : Input) (st : Stopping),
      (step s i).1.stopping = some st → st.stage = .stuck →
      ∃ st', s.stopping = some st' ∧ (st'.stage = .killed ∨ st'.stage = .stuck) := by
  cases_on_step

/-- PID 1 restarts the TV only when the user answers yes to the restart
    question. -/
theorem rebootOnlyWhenTheUserConfirms :
    ∀ (s : State) (i : Input),
      .reboot ∈ (step s i).2 →
      i = .choose .restart ∧ s.screen = .home ∧ s.restartOffered = true := by
  cases_on_step

/-! ### Capabilities -/

/-- An app never gets a capability its manifest does not have. -/
theorem grantsNeverExceedTheManifest :
    ∀ (p : Policy) (a : App) (c : Capability),
      (effective p a).has c = true → (manifest a).declares c = true := by
  cases_on_policy

/-- Every app starts with exactly what the policy gives it at that moment. -/
theorem appsStartWithTheirEffectiveGrants :
    ∀ (s : State) (i : Input) (a : App) (l : Launch) (g : Grants),
      .start a l g ∈ (step s i).2 → g = effective s.policy a := by
  cases_on_step

/-- A capability is switched on only when the user typed the current PIN. -/
theorem grantNeedsPin :
    ∀ (s : State) (i : Input) (a : App) (c : Capability),
      (s.policy.grants a).has c = false → ((step s i).1.policy.grants a).has c = true →
      i = .choose (.grant a c s.policy.pin) := by
  cases_on_step

/-- A capability is switched off only while no app is running, so nothing the
    old grant allowed (an open socket, a writable directory) survives. -/
theorem revokeOnlyWhenNothingRuns :
    ∀ (s : State) (i : Input) (a : App) (c : Capability),
      (s.policy.grants a).has c = true → ((step s i).1.policy.grants a).has c = false →
      s.alive = none := by
  cases_on_step

/-- Fewer grants never give an app more: no extra capability, file, device,
    host or socket. -/
theorem fewerGrantsNeverMoreAccess :
    ∀ (a : App) (g g' : Grants) (d : Display),
      g.within g' → (appRow a g d).within (appRow a g' d) := by
  cases_on_rows

/-! ### Connections -/

/-- A request from an app changes nothing in PID 1's state. -/
theorem requestsChangeNothing :
    ∀ (s : State) (a : App) (l : Launch) (r : Request),
      (step s (.ask a l r)).1 = s := by
  cases_on_step

/-- PID 1 grants a request only to the current launch of the app in front,
    and only if the policy allows it. -/
theorem grantsFollowThePolicy :
    ∀ (s : State) (i : Input) (a : App) (l : Launch) (r : Request),
      .reply a l r true ∈ (step s i).2 →
      l = s.launch ∧ s.screen = .app a ∧ s.policy.allows a r = true := by
  cases_on_step

/-- PID 1 opens a connection for an app only to a host in its manifest, and
    only while its `internet` capability is in effect. -/
theorem connectionsGoOnlyToListedHosts :
    ∀ (s : State) (i : Input) (a : App) (l : Launch) (host : String) (port : Nat),
      .reply a l (.connect host port) true ∈ (step s i).2 →
      (effective s.policy a).internet = true ∧ (manifest a).hosts.any (·.matches host) = true := by
  cases_on_step

/-- No app gets a TEE session. -/
theorem noAppGetsTheTee :
    ∀ (s : State) (i : Input) (a : App) (l : Launch),
      .reply a l .teeSession true ∉ (step s i).2 := by
  cases_on_step

/-! ### Offline -/

/-- While the TV is offline, no app has a network capability. -/
theorem offlineMeansNoNetworkCapability :
    ∀ (p : Policy) (a : App),
      p.offline = true → (effective p a).internet = false ∧ (effective p a).localNetwork = false := by
  cases_on_policy

/-- While the TV is offline, apps have no network at all: no hosts, no LAN,
    no sockets. -/
theorem offlineAppsHaveNoNetwork :
    ∀ (p : Policy) (d : Display) (a : App),
      p.offline = true → (confine p d (.app a)).net = .none := by
  cases_on_policy

/-- While the TV is offline, PID 1 opens no connection for anyone. -/
theorem offlineGrantsNoConnections :
    ∀ (s : State) (i : Input) (a : App) (l : Launch) (host : String) (port : Nat),
      s.policy.offline = true → .reply a l (.connect host port) true ∉ (step s i).2 := by
  cases_on_step

/-- While the TV is offline, PID 1 starts no Wi-Fi daemon. -/
theorem offlineStartsNoNetworkDaemon :
    ∀ (s : State) (i : Input) (d : NetDaemon),
      s.policy.offline = true → .daemon d true ∉ (step s i).2 := by
  cases_on_step

/-- Going offline stops both Wi-Fi daemons and takes the network down, and
    PID 1 then waits for native code to confirm it. -/
theorem goingOfflineTakesEverythingDown :
    ∀ (s : State),
      s.screen = .home → s.policy.offline = false →
      let (s', out) := step s (.choose .goOffline)
      s'.policy.offline = true ∧ s'.net = .goingOffline s.nextOp ∧
      .daemon .wifi false ∈ out ∧ .daemon .dhcp false ∈ out ∧ .network s.nextOp false ∈ out := by
  cases_on_step

/-- The TV counts as offline only after native code confirmed the change PID 1
    was waiting for. -/
theorem offlineOnlyOnceConfirmed :
    ∀ (s : State) (i : Input),
      (step s i).1.net = .offline → s.net ≠ .offline →
      ∃ op, s.net = .goingOffline op ∧ i = .netDone op false true := by
  cases_on_step

/-- The same for online. -/
theorem onlineOnlyOnceConfirmed :
    ∀ (s : State) (i : Input),
      (step s i).1.net = .online → s.net ≠ .online →
      ∃ op, s.net = .goingOnline op ∧ i = .netDone op true true := by
  cases_on_step

/-- A report on any change other than the one PID 1 is waiting for (an old
    one, a repeat, one nobody asked for) changes nothing. -/
theorem staleReportsChangeNothing :
    ∀ (s : State) (op : NetOp) (up ok : Bool),
      s.awaits op up = false → step s (.netDone op up ok) = (s, []) := by
  cases_on_step

/-- Going online needs the PIN. -/
theorem goingOnlineNeedsPin :
    ∀ (s : State) (i : Input),
      s.policy.offline = true → (step s i).1.policy.offline = false →
      i = .choose (.goOnline s.policy.pin) := by
  cases_on_step

/-- PID 1 asks for the network to come up only when the user went online with
    the PIN. -/
theorem linkUpOnlyWithPin :
    ∀ (s : State) (i : Input) (op : NetOp),
      .network op true ∈ (step s i).2 → i = .choose (.goOnline s.policy.pin) := by
  cases_on_step

/-- If the TV starts offline, the network stays down until, at some point,
    the user goes online with the PIN that was current then. -/
theorem bootOfflineStaysOfflineUntilThePin :
    ∀ (saved : Policy) (inputs : List Input) (op : NetOp),
      saved.offline = true →
      .network op true ∈ bootOutputs saved ++ runOutputs (State.boot saved) inputs →
      ∃ before after,
        inputs = before ++ .choose (.goOnline (run (State.boot saved) before).policy.pin) :: after := by
  induction_on_boot

/-- Saving a Wi-Fi network needs the PIN. -/
theorem wifiChangeNeedsPin :
    ∀ (s : State) (i : Input),
      .saveWifi ∈ (step s i).2 → i = .choose (.joinWifi s.policy.pin) := by
  cases_on_step

/-! ### Parental controls -/

/-- The PIN starts as 0000. -/
theorem pinStartsAs0000 :
    Policy.initial.pin = 0 :=
  rfl

/-- The policy changes only while PID 1's own screen is in front. -/
theorem policyChangesOnlyAtHome :
    ∀ (s : State) (i : Input),
      (step s i).1.policy ≠ s.policy → s.screen = .home := by
  cases_on_step

/-- The PIN changes only when the user typed the current PIN. -/
theorem pinChangeNeedsPin :
    ∀ (s : State) (i : Input),
      (step s i).1.policy.pin ≠ s.policy.pin →
      ∃ new, i = .choose (.changePin s.policy.pin new) := by
  cases_on_step

/-- A disabled app is enabled only when the user typed the current PIN. -/
theorem enableNeedsPin :
    ∀ (s : State) (i : Input) (a : App),
      s.policy.enabled a = false → (step s i).1.policy.enabled a = true →
      i = .choose (.enable a s.policy.pin) := by
  cases_on_step

/-- What PID 1 writes to storage is exactly its new policy. -/
theorem savesAreThePolicy :
    ∀ (s : State) (i : Input) (p : Policy),
      .save p ∈ (step s i).2 → p = (step s i).1.policy := by
  cases_on_step

/-- Over any run, if the PIN at the end differs from the PIN at the start,
    then at some point the user typed the PIN that was current at that point. -/
theorem pinNeverChangesWithoutPin :
    ∀ (s : State) (inputs : List Input),
      (run s inputs).policy.pin ≠ s.policy.pin →
      ∃ before after new,
        inputs = before ++ .choose (.changePin (run s before).policy.pin new) :: after := by
  induction_on_run

/-! ### The sandboxes -/

/-- No process except PID 1 runs as root, in any user or group. -/
theorem noSubjectIsRoot :
    ∀ (p : Policy) (d : Display) (x : Subject),
      (confine p d x).uid ≠ 0 ∧ (confine p d x).gid ≠ 0 := by
  cases_on_rows

/-- Every process runs as its own Linux user. -/
theorem subjectsHaveTheirOwnUsers :
    ∀ (p : Policy) (d : Display) (x y : Subject),
      (confine p d x).uid = (confine p d y).uid → x = y := by
  cases_on_rows

/-- Apps hold no Linux capabilities. -/
theorem appsHoldNoCapabilities :
    ∀ (p : Policy) (d : Display) (a : App),
      (confine p d (.app a)).caps = [] := by
  cases_on_rows

/-- Only the TEE helper can send raw eMMC commands. -/
theorem onlyTheTeeHelperHoldsRawio :
    ∀ (p : Policy) (d : Display) (x : Subject),
      .sysRawio ∈ (confine p d x).caps → x = .daemon .tee := by
  cases_on_rows

/-- Only the TEE helper can open the TEE or the eMMC. -/
theorem onlyTheTeeHelperSeesTheTee :
    ∀ (p : Policy) (d : Display) (x : Subject) (v : Device),
      v ∈ (confine p d x).devices →
      v.path ∈ ["/dev/tee0", "/dev/teepriv0", "/dev/mmcblk0rpmb", "/dev/mmcblk0"] →
      x = .daemon .tee := by
  cases_on_rows

/-- Apps run nothing but their own program, and cannot make memory executable. -/
theorem appsCannotExec :
    ∀ (p : Policy) (d : Display) (a : App),
      (confine p d (.app a)).exec = .none := by
  cases_on_rows

/-- An app can write only fresh directories, its own data directory, and its
    frame buffer. -/
theorem appsWriteOnlyTheirOwn :
    ∀ (p : Policy) (d : Display) (a : App) (m : Mount),
      m ∈ (confine p d (.app a)).mounts → m.writable = true →
      m.source = .fresh ∨ m.source = .outside (dataDir a) ∨ m.source = .frames := by
  cases_on_rows

/-- No process can see another app's data. -/
theorem nobodySeesAnotherAppsData :
    ∀ (p : Policy) (d : Display) (x : Subject) (a : App) (m : Mount),
      m ∈ (confine p d x).mounts → m.source = .outside (dataDir a) → x = .app a := by
  cases_on_rows

/-- Apps see no process list and nothing of `/sys`. -/
theorem appsSeeNoProcessesOrKernelFiles :
    ∀ (p : Policy) (d : Display) (a : App) (m : Mount),
      m ∈ (confine p d (.app a)).mounts →
      m.source ≠ .ownProcesses ∧ m.source ≠ .outside "/sys" := by
  cases_on_rows

/-- Only the DHCP client runs other programs, and only busybox and its lease
    script. -/
theorem onlyDhcpRunsPrograms :
    ∀ (p : Policy) (d : Display) (x : Subject) (paths : List String),
      (confine p d x).exec = .programs paths →
      x = .daemon .dhcp ∧ paths = ["/bin/busybox", "/run/tv-network/lease.sh"] := by
  cases_on_rows

/-- Only the Wi-Fi daemons open their own sockets beyond the home network. -/
theorem onlyNetworkDaemonsHaveRawSockets :
    ∀ (p : Policy) (d : Display) (x : Subject),
      (confine p d x).net.sockets ≠ [] → x = .daemon .wifi ∨ x = .daemon .dhcp := by
  cases_on_rows

/-- The kernel is locked after every module is loaded and before any other
    process starts. -/
theorem kernelLockedBeforeAnythingRuns :
    before .loadModules .lockKernel bootPlan = true ∧
    before .lockKernel .startTee bootPlan = true ∧
    before .lockKernel .startMedia bootPlan = true ∧
    before .lockKernel .startLauncher bootPlan = true := by
  decide

/-! ## 10. Effects

All of PID 1's native code, seen from Lean. Each `Native` function below is a
Rust function (`src/pid1/tvinit/src/core_effects.rs`), and the launcher's Rust
is nothing but these functions. Lean calls them only from `nextInput`,
`perform` and `main`. To replace a piece of Rust, reimplement one function.

Arguments and results are plain numbers, so the native side never touches
Lean's object layout. Apps are passed by `App.id`. Native code moves bytes;
everything that turns bytes into values (here, an app's request) is Lean. -/

namespace Native

/-- Wait at most 20 ms for something to happen and describe it as a word
    (see `decodeInput`). Choices made in the menus come back through here. -/
@[extern "tv_core_next"] opaque next : IO UInt64

/-- The monotonic clock, in nanoseconds. -/
@[extern "tv_core_clock"] opaque clock : IO UInt64

/-- Read the stored policy (see `decodePolicy`). -/
@[extern "tv_core_load"] opaque load : IO UInt64

/-- Write the policy to storage atomically. -/
@[extern "tv_core_save"] opaque save (pin enabled grants : UInt32) (offline : Bool) : IO Unit

/-- Draw PID 1's menus, and present no app frames. -/
@[extern "tv_core_show_home"] opaque showHome : IO Unit

/-- Stop drawing PID 1's menus: app `id` has the screen. -/
@[extern "tv_core_show_app"] opaque showApp (id : UInt32) : IO Unit

/-- Give a key to the menus: `kind` is 0 for Home, 1 for Power, 2 for any
    other key `code`. The menus may queue a choice for `next`. -/
@[extern "tv_core_menu_key"] opaque menuKey (kind : UInt8) (code : UInt32) (held : Bool) : IO Unit

/-- Tell the menus which apps are enabled, what each has switched on, and
    whether the TV is offline. The menus are never told the PIN. -/
@[extern "tv_core_menu_policy"] opaque menuPolicy (enabled grants : UInt32) (offline : Bool) : IO Unit

/-- Show a message in the menus (see `Notice.kind`). -/
@[extern "tv_core_tell"] opaque tell (kind : UInt8) (id : UInt32) : IO Unit

/-- Start launch `launch` of app `id` in the sandbox `appRow` gives for these
    grants (bits of `Grants.bits`). A failed start is reported through `next`
    as the launch exiting. -/
@[extern "tv_core_start"] opaque start (id launch grants : UInt32) : IO Unit

/-- Send app `id` SIGTERM, or SIGKILL if `kill`. -/
@[extern "tv_core_signal"] opaque signal (id : UInt32) (kill : Bool) : IO Unit

/-- Send a key event to app `id`: value 0 releases, 1 presses, 2 repeats. -/
@[extern "tv_core_forward"] opaque forward (id code : UInt32) (value : UInt8) : IO Unit

/-- Byte `index` of the request `next` just reported, or 256 past its end. -/
@[extern "tv_core_request_byte"] opaque requestByte (index : UInt32) : IO UInt64

/-- Answer launch `launch` of app `id`'s pending request. If `granted`, open
    a connection to the host in bytes `hostStart` to `hostStart + hostLength`
    of the request, on `port`, and pass the socket to that launch only,
    never blocking this loop; otherwise tell it no. The reply starts with
    the request's first 4 bytes, copied unread. -/
@[extern "tv_core_reply"] opaque reply (id launch : UInt32) (granted : Bool)
    (hostStart hostLength port : UInt32) : IO Unit

/-- Bring the network up or down, every layer, and report through `next`
    as change `op`. -/
@[extern "tv_core_network"] opaque network (op : UInt32) (up : Bool) : IO Unit

/-- Start or stop a Wi-Fi daemon (0 wpa_supplicant, 1 udhcpc) in its sandbox. -/
@[extern "tv_core_daemon"] opaque daemon (d : UInt8) (start : Bool) : IO Unit

/-- Write the Wi-Fi network the menus hold. -/
@[extern "tv_core_save_wifi"] opaque saveWifi : IO Unit

@[extern "tv_core_reboot"] opaque reboot : IO Unit

@[extern "tv_core_power_off"] opaque powerOff : IO Unit

/-- How native functions return values to Lean. -/
@[export tv_core_io_word] def ioWord (w : UInt64) : IO UInt64 := pure w
@[export tv_core_io_unit] def ioUnit (_ : UInt32) : IO Unit := pure ()
/-- A native effect must fail the IO action instead of pretending a write succeeded. -/
@[export tv_core_io_error] def ioError (errno : UInt32) : IO Unit :=
  throw (IO.userError s!"native core effect failed (errno {errno})")

end Native

/-- Bit `id - 1` is set for each enabled app. -/
def enabledBits (enabled : App → Bool) : UInt32 :=
  App.all.foldl (fun bits a => if enabled a then bits ||| (1 <<< (a.id - 1).toUInt32) else bits) 0

def Capability.index : Capability → Nat
  | .internet => 0 | .localNetwork => 1 | .savedData => 2

/-- One app's grants as bits: 1 internet, 2 local network, 4 saved data. -/
def Grants.bits (g : Grants) : UInt32 :=
  (if g.internet then 1 else 0) ||| (if g.localNetwork then 2 else 0) ||| (if g.savedData then 4 else 0)

def Grants.ofBits (w : Nat) : Grants :=
  ⟨w % 2 == 1, (w / 2) % 2 == 1, (w / 4) % 2 == 1⟩

/-- Every app's grants: three bits at `3 * (id - 1)`. -/
def grantBits (grants : App → Grants) : UInt32 :=
  App.all.foldl (fun bits a => bits ||| ((grants a).bits <<< (3 * (a.id - 1)).toUInt32)) 0

def Pin.ofNat? (n : Nat) : Option Pin :=
  if h : n < 10000 then some ⟨n, h⟩ else none

/-- What PID 1 runs with if stored settings cannot be read: no app enabled,
    nothing switched on, offline. The menus then offer no parental controls. -/
def Policy.unreadable : Policy :=
  { pin := 0, enabled := fun _ => false, grants := fun _ => ⟨false, false, false⟩, offline := true }

/-- The word from `Native.load`: PIN in bits 0–15, enabled apps in bits 16–23,
    bit 32 set if storage could not be read, bit 33 offline, and the grants
    (`grantBits`) in bits 40–63. -/
def decodePolicy (w : UInt64) : Policy :=
  match Pin.ofNat? (w &&& 0xffff).toNat with
  | some pin =>
    if (w >>> 32) &&& 1 == 0 then
      { pin,
        enabled := fun a => (w >>> (16 + a.id - 1).toUInt64) &&& 1 == 1,
        grants  := fun a => Grants.ofBits ((w >>> (40 + 3 * (a.id - 1)).toUInt64) &&& 7).toNat,
        offline := (w >>> 33) &&& 1 == 1 }
    else Policy.unreadable
  | none => Policy.unreadable

/-- A host name PID 1 will look up: 1 to 253 letters, digits, `.` and `-`. -/
def validHost (h : String) : Bool :=
  0 < h.length && h.length ≤ 253 && h.all (fun c => c.isAlphanum || c == '.' || c == '-')

/-- Digits to a number, if they are all digits and there are 1 to 5 of them. -/
def parsePort (digits : List Char) : Option Nat :=
  if digits.isEmpty || digits.length > 5 || !digits.all Char.isDigit then none
  else
    let n := digits.foldl (fun n c => n * 10 + (c.toNat - '0'.toNat)) 0
    if 0 < n && n < 65536 then some n else none

/-- Split at every space. -/
def words : List Char → List (List Char)
  | []        => [[]]
  | ' ' :: cs => [] :: words cs
  | c :: cs   => match words cs with
    | w :: ws => (c :: w) :: ws
    | []      => [[c]]

/-- An app's request: `connect <host> <port>`, after the 4-byte request number
    that native code copies into the reply without reading. -/
def parseRequest (bytes : List UInt8) : Option Request :=
  match words ((bytes.drop 4).map (fun b => Char.ofNat b.toNat)) with
  | [command, host, port] =>
    if String.mk command == "connect" then
      match parsePort port with
      | some p => if validHost (String.mk host) then some (.connect (String.mk host) p) else none
      | none   => none
    else none
  | _ => none

/-- The word from `Native.next`. The tag is in bits 0–7:

    - 0: a clock tick.
    - 1: a key. Kind in bits 8–15 (0 Home, 1 Power, 2 other), held in bit 16,
      key code in bits 32–63.
    - 2: a menu choice. Kind in bits 8–15 (0 launch, 1 change PIN, 2 disable,
      3 enable, 4 power off, 5 grant, 6 revoke, 7 go offline, 8 go online,
      9 join Wi-Fi, 10 restart, 11 keep waiting), app id in bits 16–23,
      capability in bits 24–31, typed PIN in bits 32–47, new PIN in bits 48–63.
    - 3: an app's launch exited. App id in bits 8–15, launch in bits 40–63.
    - 4: an ordinary key release. Key code in bits 32–63.
    - 5: a request from an app. App id in bits 8–15, launch in bits 40–63.
      The request itself is `request` (read through `requestByte`).
    - 6: a network report. Up in bit 8, ok in bit 9, change in bits 32–63.

    Anything else is dropped. (255 is handled by `nextInput`.) -/
def decodeInput (w : UInt64) (now : Time) (request : Option Request := none) : Option Input :=
  let field (pos width : Nat) : Nat := ((w >>> pos.toUInt64) &&& ((1 <<< width.toUInt64) - 1)).toNat
  let app := App.ofId (field 16 8)
  let cap := match field 24 8 with
    | 0 => some Capability.internet | 1 => some .localNetwork | 2 => some .savedData | _ => none
  let typed := Pin.ofNat? (field 32 16)
  let new := Pin.ofNat? (field 48 16)
  match field 0 8 with
  | 0 => some (.tick now)
  | 1 =>
    let held := field 16 1 == 1
    match field 8 8 with
    | 0 => some (.key .home held)
    | 1 => some (.key .power held)
    | 2 => some (.key (.other (field 32 32)) held)
    | _ => none
  | 2 =>
    match field 8 8, app, cap, typed, new with
    | 0, some a, _, _, _           => some (.choose (.launch a))
    | 1, _, _, some t, some n      => some (.choose (.changePin t n))
    | 2, some a, _, _, _           => some (.choose (.disable a))
    | 3, some a, _, some t, _      => some (.choose (.enable a t))
    | 4, _, _, _, _                => some (.choose .powerOff)
    | 5, some a, some c, some t, _ => some (.choose (.grant a c t))
    | 6, some a, some c, _, _      => some (.choose (.revoke a c))
    | 7, _, _, _, _                => some (.choose .goOffline)
    | 8, _, _, some t, _           => some (.choose (.goOnline t))
    | 9, _, _, some t, _           => some (.choose (.joinWifi t))
    | 10, _, _, _, _               => some (.choose .restart)
    | 11, _, _, _, _               => some (.choose .keepWaiting)
    | _, _, _, _, _                => none
  | 3 => (App.ofId (field 8 8)).map (.exited · (field 40 24))
  | 4 => some (.release (field 32 32))
  | 5 =>
    match App.ofId (field 8 8), request with
    | some a, some r => some (.ask a (field 40 24) r)
    | _, _           => none
  | 6 => some (.netDone (field 32 32) (field 8 1 == 1) (field 9 1 == 1))
  | _ => none

/-- Read the pending request's bytes, at most 300. -/
def readRequest : Nat → List UInt8 → IO (List UInt8)
  | 0,          bytes => pure bytes.reverse
  | budget + 1, bytes => do
    let b ← Native.requestByte bytes.length.toUInt32
    if b > 255 then pure bytes.reverse else readRequest budget (b.toUInt8 :: bytes)

/-- The next input. Tag 255 means the development console is stopping the
    launcher: the loop ends with an error, and native code may start `main`
    again later. -/
partial def nextInput : IO Input := do
  let w ← Native.next
  if w &&& 0xff == 255 then throw (IO.userError "launcher stopped")
  let request ← if w &&& 0xff == 5 then parseRequest <$> readRequest 300 [] else pure none
  match decodeInput w (← Native.clock).toNat request with
  | some i => pure i
  | none   =>
    -- A malformed request still gets an answer, so the app is not left waiting.
    if w &&& 0xff == 5 then
      Native.reply ((w >>> 8) &&& 0xff).toUInt32 (w >>> 40).toUInt32 false 0 0 0
    nextInput

def Key.kind : Key → UInt8
  | .home => 0 | .power => 1 | .other _ => 2

def Key.code : Key → UInt32
  | .other c => c.toUInt32 | _ => 0

def NetState.kind : NetState → UInt8
  | .online => 0 | .goingOffline _ => 1 | .offline => 2 | .goingOnline _ => 3 | .failed => 4

def Notice.kind : Notice → UInt8
  | .wrongPin => 0 | .disabled _ => 1 | .stillClosing _ => 2 | .exited _ => 3 | .stuck _ => 4
  | .network n => 5 + n.kind

def Notice.app : Notice → UInt32
  | .wrongPin | .network _ => 0
  | .disabled a | .stillClosing a | .exited a | .stuck a => a.id.toUInt32

/-- Where a parsed `connect` request's host is in its bytes: after the
    4-byte number and `connect `. -/
def Request.hostRange : Request → Nat × Nat
  | .connect host _ => (12, host.length) | .teeSession => (0, 0)

def Request.port : Request → Nat
  | .connect _ p => p | .teeSession => 0

/-- Carry out one output. -/
def perform : Output → IO Unit
  | .show .home       => Native.showHome
  | .show (.app a)    => Native.showApp a.id.toUInt32
  | .toMenu k held    => Native.menuKey k.kind k.code held
  | .forward a c held => Native.forward a.id.toUInt32 c.toUInt32 (if held then 2 else 1)
  | .release a c      => Native.forward a.id.toUInt32 c.toUInt32 0
  | .start a l g      => Native.start a.id.toUInt32 l.toUInt32 g.bits
  | .signal a s       => Native.signal a.id.toUInt32 (s matches .kill)
  | .reply a l r ok   =>
    Native.reply a.id.toUInt32 l.toUInt32 ok r.hostRange.1.toUInt32 r.hostRange.2.toUInt32 r.port.toUInt32
  | .save p           => do
    Native.save p.pin.val.toUInt32 (enabledBits p.enabled) (grantBits p.grants) p.offline
    Native.menuPolicy (enabledBits p.enabled) (grantBits p.grants) p.offline
  | .tell n           => Native.tell n.kind n.app
  | .network op up    => Native.network op.toUInt32 up
  | .daemon d go      => Native.daemon (match d with | .wifi => 0 | .dhcp => 1) go
  | .saveWifi         => Native.saveWifi
  | .reboot           => Native.reboot
  | .powerOff         => Native.powerOff

/-- Read an input, step, carry out the outputs, repeat. -/
partial def loop (next : IO Input) (perform : Output → IO Unit) (s : State) : IO Unit := do
  let (s', outputs) := step s (← next)
  for o in outputs do perform o
  loop next perform s'

/-- PID 1's launcher. Runs until the development console stops it. -/
@[export tv_core_main]
def main : IO Unit := do
  let policy := decodePolicy (← Native.load)
  Native.menuPolicy (enabledBits policy.enabled) (grantBits policy.grants) policy.offline
  Native.showHome
  for o in bootOutputs policy do perform o
  loop nextInput perform (State.boot policy)

/-! ### The native app table

The C launcher and the seccomp generator read a copy of `manifest` and
`appRow` as plain numbers (`TV/Apps.lean`), because they run without the Lean
runtime. -/

/-- `TV/Apps.lean` agrees with `manifest` and `appRow` for every app and every
    grant: the user, the thread limit, and the two parts of its network
    (bit 1: its own sockets; bit 2: connections from PID 1). -/
theorem nativeAppTableIsTheManifest :
    ∀ (a : App) (g : Grants) (d : Display),
      TV.Apps.uid a.id.toUInt32 = (manifest a).uid.toUInt32 ∧
      TV.Apps.threads a.id.toUInt32 = (manifest a).threads.toUInt32 ∧
      TV.Apps.network a.id.toUInt32 g.bits =
        ((if (appRow a g d).net.lan then 1 else 0) ||| (if (appRow a g d).net.broker.isEmpty then 0 else 2)) := by
  cases_on_apps

/-- Every app's id decodes back to that app. -/
theorem appIdsRoundTrip :
    ∀ (a : App), App.ofId a.id = some a := by
  cases_on_apps

/-- Every request PID 1 accepts names a valid host and a real port. -/
theorem parsedRequestsAreValid :
    ∀ (bytes : List UInt8) (host : String) (port : Nat),
      parseRequest bytes = some (.connect host port) →
      validHost host = true ∧ 0 < port ∧ port < 65536 := by
  cases_on_parse

/-! ## 11. Not proven

The theorems are about `step` and the tables. They mean what they say about
the TV only if native code does what section 10 says. In particular:

- **Exits are complete.** `next` reports `exited a l` only when launch `l`'s
  main process is reaped, no process runs as the app's user, its cgroup is
  empty, and every connection job PID 1 started for it is finished or
  cancelled, with any socket it opened closed and none delivered later.
  Once per launch.
- **Sandboxes are built as written.** `start` builds `appRow a g` exactly,
  and the daemons are started only in `daemonRow`'s sandboxes, in the order
  of section 4: private mounts, private `/dev` with only the listed nodes,
  network namespace, cgroup, limits, user and capabilities, no new
  privileges, the system-call filter last, then the program. The filter
  compiler (`TV/Sandbox.lean`) is checked exhaustively by
  `test/lean/tv/check-sandbox.py`, not yet proved. The `lan` limit (home
  addresses only) is a cgroup BPF program attached by native code.
- **The network reports are true.** `next` reports `netDone op false true`
  only after checking every layer: both Wi-Fi daemons reaped; `eth0` and
  `wlan0` read back down; the Wi-Fi driver unbound; a drop-everything packet
  filter attached to the root cgroup. `netDone op true true` only after the
  links are up. Production builds bring no link up before `main`.
- **Connections.** `reply` opens a connection only for the request `next`
  just reported, to that host and port, for that launch, on a worker that
  never blocks the loop. It never connects to the TV itself (loopback). The
  host check is on the name the app asked for; DNS answers for an allowed
  name are trusted.
- **The kernel is locked.** Native start-up follows `bootPlan` and applies
  every setting in `kernelLock` before starting the TEE helper.
- **The screen.** After `showHome`, no frame from an app reaches the panel.
  An app that writes the framebuffer directly can draw over the menus until
  it exits; PID 1 draws its screen again when it does.
- **The remote.** `next` reports Home and Power as `Key.home` and
  `Key.power`, and reports them before any app request. Volume keys are
  handled by PID 1's volume service and never reach `step`.
- **The menus** pass on the digits the user typed and are never given the
  PIN. If storage cannot be read, PID 1 runs with no app enabled and offline.
- **Storage.** `save` replaces the stored policy atomically, and `load`
  returns what was last saved. A failed save fails the IO action; `loop`
  then stops and the native supervisor shows a safe home state.
- **No back doors.** Development builds also contain a root console, a
  recovery HTTP endpoint, ADB and an Ethernet debug link, which bypass all
  of this. The guarantees are for production builds, which contain none.
- **The TEE helper's raw eMMC access.** `teeRow` holds `sysRawio` for one
  command that reads the eMMC's registers. With it, a compromised helper
  could send any eMMC command. Remove it if the TV starts without it.
- **The platform.** The Lean compiler and runtime, the Linux kernel, the
  vendor drivers, and the firmware below Linux.
- **Transitional network.** Until `appRow`'s `lan` limit is enforced, an app
  with `localNetwork` can reach any address.
-/

end TV.Core
