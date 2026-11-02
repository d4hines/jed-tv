# Why cold FLASH status3 can be false after successful initialization

The live reduced bootstrap reported MIKO40=true, SYS0=true and FLASH3=false
after inserting mik with stock SYS/FLASH flags enabled. Exact V109 code
explains this: kernel SYS initialization overwrites the status table after
FLASH has initialized and published its bit. FLASH3 is not a valid prerequisite
for the later owned cold sequence.

The sequence in `mik.ko` is:

1. `MI_DEV_SYS_Init` reads the `MI_FLASH:MI_FLASH_INIT` setting at0x33408.
   If it equals1, it calls `MI_FLASH_Init` at0x3356c.
2. Only return0 takes the success branch: it constructs selector3/ready1 and
   calls `MI_SYS_SetAttr` at0x33590. A nonzero return takes the error path.
3. It resolves the flash environment/memory-map configuration and calls
   `MI_SYS_Init` at0x3361c.
4. `MI_SYS_Init` clears both words of `_stModuleInitTable` at0x2680b8/bc.
   More decisively, its successful initialization tail stores literal64-bit1
   into the whole table at0x269dac–0x269dbc: SYS bit0=true, every other bit=false.
5. Later module initialization publishes MIKO40 and whichever subsystem
   bits it initializes. It does not restore the earlier FLASH3 publication.

Thus the observed40/0=true,3=false is compatible with a successful FLASH init.
It is not permission to issue FLASH INIT again. The kernel FLASH routine has
its own `_gbIsFlashModuleInit` state, separate from `_stModuleInitTable`.

`assets/build/pid1-repl/reduced-bootstrap/kernel.log` records the expected
onebin model1 memory-map path and successful SYS progression. It also records
`MI_FLASH_StoreEnv: ENV write back fail`, consistent with the current protected
flash environment; the log should be preserved rather than treating every
message as harmless. No partition write was issued by this investigation.

## Corrected owned prerequisite

The shared Rust prerequisite is now selectors40 and0, in both general cold
initialization and amplifier-handle authorization. All other restrictions
remain: original G06, root PID1, exclusive fresh-boot provenance, existing
eMMC read-only protection, rejection of SYS_USER alreadyready, and at most
one attempt per subsystem. The provenance contract explicitly requires the
verified MI_FLASH_INIT=1 profile to have been staged before loading mik and
retained throughout kernel initialization. This is historical evidence,
not something a warm metadata snapshot can reconstruct.

FLASH3 remains available as a diagnostic query. Nothing sets it to make the
new checks pass. New tests verify that missing40 or0 still blocks all INITs,
while the exact false3 cold case can initialize only the requested display
module and leaves3 false. Existing warm-provenance refusal tests remain.

## Evidence and integration

- `sys-kernel-init.txt`: annotated MI_DEV_SYS_Init flag/call/publication order.
- `flash-sys-status-reset.txt`: annotated kernel FLASH and SYS implementations.
- `flash-status-prerequisite.patch`: minimal patch prepared against then-current
  main, covering `lib.rs` and `amp_handles.rs` only.

These generated files are under `assets/research/midaemon-replacement/`.
The updated isolated crate passes29 tests and the static ARM build.
No new TV call or state mutation was performed.
