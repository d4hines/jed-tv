# Owned V109 cold-start bootstrap candidate

This is an offline replacement plan for the initialization part of
`src/runtime-scripts/pid1/start-vendor-display.sh`. It is not yet a working
replacement launcher. No module was loaded, service stopped, boot changed, or
TV contacted to prepare it. It keeps the exact stock kernel and vendor kernel
drivers; all new coordinating userspace is intended to be a library in the
single Rust/Lean PID 1.

## Prepared artifact and reproducibility

`tools/midaemon/prepare-cold-start.py` checks every original configuration
source against the prior offline manifest and writes a new, separate RAM-root
candidate. It refuses an existing output directory. From the repository root:

```sh
PYTHONDONTWRITEBYTECODE=1 python3 tools/midaemon/prepare-cold-start.py
```

The prepared default is `assets/research/midaemon-replacement/cold-start/`.
`root/` contains 344 files, 33,498,137 bytes: the conservative 331-file
configuration set and 13 exact V109 modules. `manifest.json` records every
source/output hash, the previous manifest hash, module order, and unresolved
stock paths. `/config` is a relative symlink to `vendor/tvconfig/config`.

This preserves the existing *candidate* configuration closure, not a proven
minimum. It deliberately does not copy glibc, midaemon, Android properties,
Binder services, SurfaceFlinger, graphics HALs or tee-supplicant. Their absence
from this candidate does not prove that all functions currently provided by
them have been replaced.

The only INI change from stock is `MI_TUNER_INIT=0`. All 35 INIT keys are
validated: SYS and FLASH are 1, every other INIT is 0. This differs from the
earlier `g06-display-offline/candidate-root` recipe, which enabled automatic
DISP and OSD initialization. Do not accidentally stage that older INI for the
explicit-init plan. No original firmware file is modified.

`src/mi-bootstrap/src/cold_start.rs` validates the explicit-init INI, rejects
duplicate keys, unknown enabled initializers, missing flags and changed
DISP/OSD parameters. It exports ordered module and subsystem lists. The CLI
can validate a staged file on the host without opening a device:

```sh
mi-bootstrap validate-cold-ini PATH/TO/mik_init.ini
```

Validation proves agreement with this recipe, not the bytes the kernel
actually read and not cold-boot provenance. The caller must retain a hash of
the exact staged INI and immutable configuration until module loading ends.

## What already initializes in the kernel

The exact `mik.ko` entry path calls `MI_DEV_SYS_Init`; it opens
`/vendor/tvconfig/config/mik_init.ini` at `.text+0x33390`. The SYS path calls
`MI_FLASH_Init` at `0x33590`, resolves the flash environment's `MODEL_ID` and
selected map at `0x33718/0x33750`, and calls `MI_SYS_Init` at `0x3361c`.
The latter is unconditional along the successful SYS path: setting
`MI_SYS_INIT=0` is not a supported way to bypass SYS setup. Keep both flags 1.

`MI_DEV_SYS_ModuleInit` then follows per-subsystem INI controls. Known display
examples are `MI_DISP_INIT` → `MI_DISP_Init` at `0x34674` and
`MI_OSD_INIT` → `MI_OSD_Init` at `0x34a80`. Their success paths publish
selectors 6 and 9, respectively; failures abort module initialization.
Stock sets only tuner additionally to 1. The explicit recipe disables tuner,
leaving DISP/DISPOUT/OSD/AUDIO/AOUT/PCM to the owned Rust adapter.

| Phase | Already performed / requested | Owned userspace action |
| --- | --- | --- |
| `mik` load | FLASH and SYS, plus driver registrations | Do not repeat SYS/FLASH INIT; require successful load and selectors 40/0/3 |
| Display | No INI auto-init in this recipe | DISP → DISPOUT → OSD through exact ioctl adapters |
| Audio, optional later | No INI auto-init in this recipe | AUDIO → AOUT → PCM; route and amplifier are separate |
| GPIO/IIC | No INI auto-init in this recipe | Still needs an audited owned cold-init/open path before amplifier use |
| Framebuffer | `mik` publishes selector40, not selector1 | Do not infer framebuffer readiness or open it yet |

Exact existing adapter payloads are DISP `{u32 6, u32 6, u32 0}` (12 bytes),
OSD twelve zero bytes (surface_count0 selects the default64), and one zero
byte for DISPOUT/AUDIO/AOUT/PCM. Each uses the already recovered 16-byte outer
ioctl envelope. See `MIDAEMON_REPLACEMENT.md` and the crate for exact request
values. DISP/DISPOUT/OSD requests are real initialization, not substitutes for
panel configuration, scanout allocation or layer presentation.

## Module and configuration dependencies

The proven ELF link closure is:

```
iniparser → kdrv_xc → utpa2k → hdcp1x/hdcp2x
          → xcker/mwgifker/misck → mik → mstar_fbdev_mi
```

The arrows indicate a valid loading order, not that every adjacent pair has
a direct symbol edge. Explicit providers/CRC evidence remains in
`research/g06-display/FINDINGS.txt` and the generated dependency JSON.
There are ten modules in this closure. The first trial retains the three
stock runtime registration modules as well. Ordered pre-display load list:

```
iniparser kdrv_platform kdrv_xc kdrv_alg libdmdbraslia utpa2k
hdcp1x hdcp2x xcker mwgifker misck mik
```

Defer `mstar_fbdev_mi` until the framebuffer readiness gate below is resolved.
`dtv_driver`, `devnode`, `mali_kbase` and Android services are outside this
CPU framebuffer candidate. Later remove the three retained extras only after
separate comparison tests; a lack of ELF imports does not establish that
their runtime registration is unnecessary.

Before loading `mik`, stage the exact hierarchy including:

- `/vendor/tvconfig/config/{mik_init.ini,sys.ini,board.ini,MMAP_MI.h}`.
- OSD/OsdConfig.ini, OSD/OsdDefine.ini and PQ_general/OSDPQConfig.ini.
- Both onebin/model_0 and onebin/model_1 memory maps selected by `MODEL_ID`.
- Customer_185_32TCON_B07-1_TL1P2.ini and its referenced panel/TCON/PQ files.
- Qos.bin and QosCustomer.bin, referenced by code outside the INI path graph.

Do not select a memory map from MemTotal or substitute QEMU's map. SYS starts
multiple configuration loader workers even when the subsystem INIT flags
are zero. The smaller eight-file bootstrap list is not a sufficient closure.
The manifest carries unresolved literal stock references verbatim; do not
invent replacements for them or assume each is relevant to this panel path.

## PID-1 integration order and stopping conditions

1. Enter the existing native boot path with the working root console/recovery
   plumbing. Claim `ColdBootProvenance` in PID 1 **before any userspace MI
   initializer/client runs**, and retain it. This history cannot be recreated
   from a snapshot, an empty FD list, or false status bits.
2. Preserve current block-device read-only protection. Stage the validated
   configuration tree in RAM, without shadowing files from another recipe.
   Retain manifests and complete module/error logs. No partition writes are
   part of this plan.
3. Load the ordered pre-display modules exactly once. Stop immediately on
   failure. Inspect complete kernel logs for missing configuration, chosen
   MODEL_ID/map and failed runtime registration. Check required device nodes;
   absence of a node is a failure, not permission to publish readiness.
4. Open `Device`, consume provenance with `permit_initialization`, and run
   `initialize(..., cold_start::DISPLAY_SEQUENCE)`. This requires selectors
   40/0/3 and rejects selector1 already ready. Keep exact return codes and
   metadata readbacks. No retries on a failed or partially attempted sequence.
5. Establish the narrower readiness contract required by the intended direct
   display backend: real initialized controller/panel, initialized OSD, valid
   configuration, and exclusive layer/scanout ownership. This remains a
   hardware gate. This implementation intentionally has no selector1 setter.
6. Only once that semantic gate is reviewed, publish readiness through a
   separately audited owned operation and initialize the framebuffer once,
   or use the direct MI backend if it does not require that flag. A zero
   return code alone is not a replacement for the full gate.
7. Preserve each created layer/window/allocation for the lifetime of the
   owner. The display agent found the exact WindowDestroy path effectively
   stubbed: framebuffer FREE can free backing memory despite failed window
   destruction, leaving stale scanout references. Hide/reuse allocations;
   do not rely on close/free cleanup during iteration. Let the display owner
   define the lifetime contract before integrating this phase.
8. Add audio separately using `AUDIO_SEQUENCE`, then audited GPIO/IIC setup,
   route ownership, and the amplifier sequence. Keep PCM stopped and amp
   muted until a bounded, explicitly scheduled sound test.

The Rust plan validator does not issue a `finit_module` syscall or implement a
new loader; the existing PID-1 loader should consume its ordered constants.
It also does not publish readiness, create scanout resources, configure GPIO,
or alter the working launcher. These gaps are explicit to prevent a partial
plan from silently becoming an unattended boot experiment.

## Minimal discriminating hardware checks

First run only through step4 in a fresh native boot, with the root console
alive throughout. Record INI hash, each load/INIT result, full kernel log,
selectors 40/0/3/1/6/7/9, and read-only display-controller/OSD queries.
Do not start or stop vendor services in that run; starting them would end the
exclusive cold phase. A failure is diagnostic evidence, not a reason to
retry INIT against a partially live subsystem.

If that passes, schedule a separate owned persistent display trial using the
display agent's tested presentation protocol and allocation lifetime. Only
after a visible frame passes should the pack be narrowed. Config removal,
the three optional module removals, audio initialization, and all vendor
process elimination are distinct comparisons.

Offline verification: 16 crate tests pass, including five recipe/order
checks. The static ARM build passes. The generated INI passes the actual
Rust validator. File hashes and copied contents were checked on the host.
These checks establish reproducible staging and policy validation; they do
not establish cold panel initialization or complete vendor replacement.
