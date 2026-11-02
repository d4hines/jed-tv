## Direct OSD hardware submission (2026-11-01)

The new PID1-owned path created its OSD layer/window/surfaces directly. A
120-frame local video run completed with no dropped frames or audio underruns.
PID1 held /dev/mik/osd and its DMA-BUF and had no framebuffer descriptor open.
Stock vendor initialization was still used first. Physical panel confirmation
for this exact implementation is pending; do not equate submission with pixels.
Evidence: assets/build/pid1-repl/external-osd-live/.

## Allocation proof on the G06

The direct MMA allocation/map probe passed on V109: 7,372,800 private bytes,
full-width IOVA 0x20276a000, and readback at both ends of both pages. It made no
OSD or framebuffer calls; the existing display and audio processes stayed alive.
The probe unmapped and closed its DMA-BUF. Kernel allocator accounting was not
available at the checked debug paths, so release is not independently measured.
Source: src/display/mma.rs and src/display/mma_probe.rs. Evidence:
assets/build/display-agent/mma-allocation-proof.json. This does not validate
external surface creation or scanout.

# Replace fbdev scanout with direct MI OSD and MMA

Offline ABI recovery, 2026-11-01. This is a proposed replacement for the current
Scanout implementation, not a second app API or a runtime backend flag. No TV
ioctls were issued by this agent. The current physically proven route still
uses fbdev to create its OSD resources.

The reason to replace fbdev is its open-time SYS_USER selector-1 gate and
additional driver initialization/teardown behavior. MI_OSD_LayerCreate and
MI_OSD_WindowCreate check OSD's own initialization, not that broad gate.
The bootstrap agent verified those checks at mik 0x1cde70 and 0x1d85ec.

## Allocation

Exact libutopia.so `mma_alloc_internal` at 0x7eb0b0 opens /dev/mma and sends
ioctl 0xc0304d03 with this 48-byte direct argument (no MI outer envelope):

| offset | width | field |
| --- | --- | --- |
| 0 | 16 | NUL-terminated allocation tag |
| 16 | 8 | zero/reserved |
| 24 | 8 | returned IOVA |
| 32 | 4 | requested bytes |
| 36 | 4 | returned DMA-BUF fd |
| 40 | 4 | allocation flags; zero for normal nonsecure allocation |
| 44 | 4 | zero/reserved |

Evidence: stores at 0x7eb14c (size), 0x7eb168 (normal/secure flag), call at
0x7eb184, reads at 0x7eb190..0x7eb1a4. The public MsOS_MMA_Alloc veneer
sets the flag to zero at 0x7e4900. Use the existing framebuffer allocation's
`fbdev` tag first: it is already proven as a normal accepted board allocation.
Validate both full-width IOVA and returned fd before using either. Initialize
fd to -1; immediately adopt successful returned fd into owned File so later
errors do not leak it before any hardware consumer exists.

Select uncached CPU mapping with the already-proven 40-byte MMA operation:
0x40284d06, fd at word0, cache selector at word6 = 0. mmap the DMA-BUF at
offset zero, length checked against fd metadata. Never mmap a truncated IOVA.
A double page allocation is 1280*720*4*2 = 7372800 bytes. Both page IOVAs and
CPU offsets must remain within this one allocation.

## MI transport

Every /dev/mik/osd ioctl below uses the existing 16-byte outer envelope:
u16 inner length at offset0, padding zero, u64 pointer at offset8.
All inner structs have fixed 32-bit handles/enums and explicit padding for
64-bit addresses; do not serialize host pointers as a vendor address.

## Layer creation

ioctl 0xc0241403, inner 36 bytes:

    handle_out:u32 @0
    layer_id:u32 @4              1 for independent trial, 0 for owned cold boot
    layer_size_enum:u32 @8       7 = CUSTOM
    custom_rect_and_dest:[u32;6] @12 = 0,0,1280,720,1920,1080

That is MI_DEV_OSD_LayerCreateParams_t with nested LayerInfo. Exact wrapper
libmi3 0x78b6c; kernel 0x1cdde4. Require the selected layer to be absent before
creation. LayerCreate changes GOP routing/scale/mux; it is a visible mutation,
even before showing a window. Do not create a second layer on a live trial
without parent coordination and an explicit owner for existing resources.

## External surface creation

ioctl 0xc038142f, inner 56 bytes:

    handle_out:u32 @0; padding:u32 @4
    width:u32 @8 =1280; height:u32 @12 =720; pitch:u32 @16 =5120
    format:u32 @20 =8            ARGB8888, BGRA byte order in memory
    owner:u32 @24 =0             AP owns backing allocation
    memory_type:u32 @28 =2       PHY_OS, physical address supplied
    address:u64 @32              full IOVA including high word
    rearrange:u8 @40 =0; padding[7] @41
    is_cache:u8 @48 =0; padding[7] @49

Create one MI surface per page, address = allocation IOVA + checked page offset.
These are MI handles over our allocation, not separate allocations. Format 8
and AP/PHY_OS enum values are from exact libmi DWARF. SurfaceCreate wrapper
0x7b858 skips userspace virtual-to-physical translation for this memory type.
Its final ioctl is 0x7b97c. The matching kernel SurfaceCreate still needs a
complete follow-up audit of PHY_OS buffer validation and mapping; do not yet
claim this new constructor is hardware-proven.

## Window and presentation

Window create ioctl 0xc030141b, inner48 bytes:

    window_out:u32 @0
    layer_handle:u32 @4
    x:u32 @8 =0; y:u32 @12 =0; width:u32 @16 =1280; height:u32 @20 =720
    pixel_alpha:u8 @24 =1; padding[3] @25
    format:u32 @28 =8
    buffer_type:u32 @32 =2       EXTERNAL, not auto-allocated double buffers
    surface_width:u32 @36 =1280; surface_height:u32 @40 =720
    is_full_flip:u8 @44 =0; padding[3] @45

Wrapper 0x7a4a0, kernel WindowCreate 0x1d859c. Verify enum and struct sizes with
compile-time assertions and byte-offset tests on ARM32 as well as host.

Present with ioctl 0xc008142d, inner [window_handle:u32, surface_handle:u32].
That is MI_OSD_WindowFlipByExternSurface (wrapper0x7ae4c). The existing fbdev
pan wrapper also calls this exact API at0x470c. Visibility uses proven LayerShow/Hide 0xc0041405/0xc0041406. Window-level
visibility is available but not needed for this single-window replacement.
Do not derive synchronization guarantees from a successful return: audit the
kernel queue/wait behavior before permitting immediate page reuse.

## Ownership and failure behavior

PID1 should own the MI handles, allocation fd and CPU map for its lifetime.
The TVF2 shared producer/reader lease, compositing, and APIv1 boundary do not
change. Apps never get MI handles or a hardware allocation fd.

V109's WindowDestroy is broken (see DISPLAY_ORDERING.md). Once a window may
reference an allocation, a failure/drop path must not close/free its last
backing reference or destroy its surfaces and leave stale window state.
Hide and retain bounded resources on ordinary stop; use a clean kernel boot
to reset unrecoverable partial state. Before window creation, ordinary failed
allocation/map/surface setup can release resources only after proving no
published kernel consumer exists. A successfully allocated fd plus uncertain
window creation result needs conservative retention and explicit diagnostics.

Next implementation sequence: pure wire constructors and offset tests;
allocation/map-only probe (no OSD changes); kernel external-surface and flip
queue audit; parent-controlled persistent OSD trial; then replace the old
Scanout implementation after physical confirmation. Do not add a permanent
runtime switch between the two implementations.

Generated evidence:
assets/build/display-agent/{mma-alloc-internal,mma-alloc-wrapper,
iommu-allocate-wrapper,osd-layer-create-wrapper,osd-window-create-wrapper,
osd-window-flip-wrapper}.txt. Struct/enum source:
assets/research/firmware-implications/libmi.dwarf.txt, MI_OSD_* and
MI_DEV_OSD_* types. This plan does not establish cold panel initialization.

The current constructor matches the working fbdev custom-layer timing space:
1280x720 source to1920x1080 destination. After WindowCreate, it disables color
keying, enables per-pixel alpha with value255, and disables layer-global alpha.
These are explicit fbdev calls, not assumptions about kernel defaults.
