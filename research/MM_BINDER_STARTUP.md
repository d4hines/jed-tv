# MM startup: Binder node and service-manager dependency

The unmodified midaemon MM startup needs a Binder context manager; it is not
just a self-hosted thread pool. For the exact MM path, the requested domain is
`/dev/vndbinder`. The missing `/dev/binder` error does not prove that ordinary
servicemanager is required: the linked ProcessState explicitly falls back to
`/dev/binder` when `/dev/vndbinder` fails its initial accessibility check.

## Exact call chain

`libmi.so:mi_ipc_Init` at0x1fec8 tests its weak `mi_ipc_StartMiMmService` symbol
at0x1fed8–0x1fee4. If present, it prints `Start MM service` and calls it at
0x1fef8. It next repeats this pattern for Injplay, Subtitle, Ttx and Voc.

`libmi_server.so:mi_ipc_StartMiMmService` at0x75eb8:

1. Constructs a `BnMiMmService` object.
2. Calls `ProcessState::initWithDriver("/dev/vndbinder")` at0x75f5c.
3. Calls `defaultServiceManager()` at0x75f88.
4. Starts the Binder thread pool at0x75fa8.
5. Calls the service manager's vtable+0x18 at0x76030 to register
   `mstar.MI.MmService`, allowIsolated=false, dumpPriority=8.

It logs a nonzero addService result and returns an error. The generic
`mi_ipc_Init` caller does not check the individual start-call return value,
but null/dead manager handling inside the linked Binder implementation can
still fail before reaching that return. There is no evidence supporting a
node-only startup as a complete working replacement.

The linked `libmdroid.so:ProcessState::initWithDriver` at0x80520 first checks
the requested driver with `access(..., R_OK)` at0x805cc. Failure branches to
0x805fc, logs that the requested driver is unavailable, replaces its argument
with `/dev/binder` at0x80618, then constructs the process state. This exactly
explains how a requested vndbinder can produce a missing binder error.
If a process state already exists with a different driver, it asserts at
0x80624 instead of switching domains.

The constructors before that explicit call do not establish an observed
ordinary Binder dependency: the MiIpcMmap client/server constructors only
initialize local object fields; the BBinder constructor delegates to the base
and initializes local fields. The fallback is a concrete explanation for the
reported path; it does not need a hypothetical earlier ProcessState call.

## Minimum supported reduced-vendor trial

Create the Binder nodes with the existing Rust `tv-binder` before starting
midaemon, then start the stock vndservicemanager on `/dev/vndbinder`.
Wait for the manager to be alive before midaemon begins. This is the exact
domain needed by the MM registration path. No ordinary servicemanager,
hwservicemanager, media services or Java runtime is established as necessary
by this chain. Other later code might add dependencies; that needs separate
logs rather than starting them speculatively.

The historical `midaemon-with-vndbinder.log` reached `Start MM service` then
segfaulted. A missing context manager is consistent with it, but the log has
no crash PC, so it does not prove the cause. The new missing-driver failure
and that historical segmentation fault are distinct pieces of evidence.

## Supported path that omits the memory service

`libmi.so` deliberately supports the absence of these services: all five
start symbols are undefined WEAK and guarded by null checks. Its full
DT_NEEDED closure contains17 libraries and neither libmi_server nor
libmdroid/Binder. This was checked against the complete extracted glibc tree;
all17 dependencies were found. An owned helper linked only to libmi can use
that supported no-server variant of `MI_SYS_Init`; it must avoid linking the
larger multimedia SDK/server libraries that reintroduce the symbols.

Unmodified midaemon cannot use this variant merely by changing an environment
variable or removing a file: its ELF DT_NEEDED explicitly includes
libmi_server.so. No configuration switch was found that skips `mi_ipc_Init`.
Removing a DT_NEEDED entry from a copied executable would be a separate binary
modification, not the supported weak-link helper path described above.

Our owned Rust direct-kernel sequence bypasses the user-library `MI_SYS_Init`
and `mi_ipc_Init` entirely: kernel SYS/FLASH setup, exact subsystem ioctls,
controller open, boot-logo handoff and direct OSD/MMA do not make a Binder
registration call. This is the intended final path, still needing cold
hardware validation. The optional libmi-only helper is an intermediate way to
retain vendor userspace initialization while excluding MM/Binder.

## Saved evidence

Under `assets/research/midaemon-replacement/`:

- `mm-server-start.txt`: exact annotated MM service registration function.
- `mm-process-state-driver.txt`: driver selection and explicit fallback.
- `mm-default-service-manager.txt`: linked default manager implementation.
- `libmi-without-server-closure.json`: all17 no-server library dependencies.

The original full libmi/libmi_server annotated dumps contain the weak-symbol
checks and constructors. Exact linked libmdroid SHA256 is
`a212066a522d87676c7b626a944980c0a119392a66784180779b8424909b4e87`.
No hardware calls or main-worktree changes were made in this audit.
