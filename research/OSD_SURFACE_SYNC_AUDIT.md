# External MI OSD surfaces and presentation synchronization

Offline exact-V109 audit, 2026-11-01. No device operations or main-worktree
changes. This supplements DIRECT_OSD_WITHOUT_FBDEV.md and DISPLAY_PAN_AUDIT.md.

## Decision for the replacement

Use AP-owned PHY_OS surfaces over a retained MMA DMA-BUF, and reproduce the
working immediate framebuffer path with three direct MI calls:

1. LayerBeginConfig, ioctl0xc0041416, inner u32 layer handle.
2. WindowFlipByExternSurface, ioctl0xc008142d, inner [window,surface].
3. LayerApplyConfig, ioctl0xc0041417, inner u32 layer handle.

All three use the usual16-byte MI outer envelope. Do not issue only the flip:
the current working fbdev mode1 brackets it with Begin/Apply. No fbdev node
is needed for this sequence. The layer defaults CONFIG_WAIT_SYNC=true.

Preserve the current TVF2 ownership protocol. The MI calls do not supply a
DMA-BUF retirement fence, and must not be treated as authorizing arbitrary
immediate reuse/free of a submitted page. Current warm animation is proven;
strict no-tearing/reliable retirement remains a separate requirement below.

## SurfaceCreate validation and backing ownership

Public MI_OSD_SurfaceCreate at0x1c91c8 calls private _MI_OSD_SurfaceCreate at
0x1c7610. The public function copies the40-byte input into a local structure,
takes the OSD mutex, and returns handle0x80000|id. Existing ioctl layout is
56 bytes: handle@0, surface information@8, cache byte@48.

Private checks relevant to ARGB8888 AP/PHY_OS:

- Obtain an available surface slot from the initialized surface table
  (0x1c7658..0x1c76e4); failure returns9. Thus OSD initialization must allocate
  the table before this constructor.
- Query format-specific alignment/capability at0x1c771c and0x1c7758. Invalid
  formats fail. Pixel-depth conversion at0x1c7800 accepts its supported values.
- Explicit pitch must satisfy the queried pitch alignment when rearrangement
  is off: divide/remainder check at0x1c79a4..0x1c79c0. Supplying pitch0 instead
  computes width*pixel-size and aligns it, but our path should use exact5120.
- AP ownership selects0x1c788c. Only memory type OS=1 takes the virtual pointer
  alignment/translation branch. PHY_OS=2 instead copies the supplied64-bit
  address, requires nonzero, and goes directly to surface bookkeeping
  (0x1c7a74..0x1c7a88). Address stored at surface record+0x18 via STRD0x1c7b60.
- The getter used by presentation, private _MI_OSD_GetSurfacePa at0x1c8b38,
  translates virtual storage only for OS=1. Other memory types return the
  saved64-bit address at0x1c8c00..0x1c8c10. PHY_OS does not truncate the IOVA.

The AP/PHY_OS branch does not take a DMA-BUF descriptor, import an allocation,
validate its extent, or acquire a backing-storage reference. It only knows an
address and geometry. A successful surface handle therefore does not validate
our allocation bounds or preserve its lifetime. Keep the MMA allocation fd
alive and check the complete [base,base+size) range in Rust before creating
both pages. Check width/height/pitch arithmetic independently; no overflow or
extent protection should be inferred from the vendor constructor.

Surface metadata stores ownership and memory type separately (record+0x34 and
+0x30). The first flip rejects non-PHY_OS surfaces at0x1dc3fc..0x1dc404.

No TEEC or DMA-BUF import call is present on this inspected PHY_OS path. It
can still only operate after the broader OSD/GOP hardware initialization.

## What the immediate flip actually does

MI_OSD_WindowFlipByExternSurface at0x1dc1b4 validates OSD-ready, handle classes,
and that the surface id exists; then holds the OSD mutex and calls its private
helper0x1dc398. The helper requires an active window and surface memory type2.
It gets/creates a GOP framebuffer object over that physical address, updates
geometry if necessary, and calls _MI_OSD_SetWindowFB at0x1c7e78. With the null
queue argument used by this API, normal2D presentation maps the GOP framebuffer
to its window via MApi_GOP_GWIN_Map32FB2Win at0x1c84fc. Queue-specific paths
instead reach Switch_GWIN_2_FB; they are deliberately outside this replacement.
After success it updates the surface's owning window and current-window
surface bookkeeping at0x1dc75c..0x1dc830.

There is no scanout-completion wait in this public/private flip sequence and
no retirement fence in its two-word ioctl ABI. The bookkeeping being updated
is not evidence that the old physical page is no longer being read.

## Begin/Apply and ACK behavior

LayerBeginConfig selects the layer's GOP and calls
MApi_GOP_GWIN_UpdateRegOnceByMask(mask,true,wait_sync) at0x1d4220.
LayerApplyConfig uses(mask,false,wait_sync) at0x1d449c. LayerCreate explicitly
initializes wait_sync to1 at0x1cef70..0x1cef7c. CONFIG_WAIT_SYNC attr10 gets or
sets the one-byte field at layer record+0x5d; do not confuse it with WAIT_SYNC
attr5, which has a different structure/purpose.

The Utopia wrapper passes this to GOP_UpdateCurrentOnce0x2dce54, which caches
the begin flag and on apply calls MDrv_GOP_GWIN_UpdateRegWithMaskSync
(0x2dcee0). That driver has several hardware paths. On its normal ACK path,
wait_sync=1 causes HAL_GOP_GetGOPACK polling and MsOS_DelayTask(1), up to
approximately40 iterations (0x2ea6dc..0x2ea724). If the timeout is reached it
forces register write-in at0x2ea728..0x2ea74c rather than returning a distinct
retirement-failure status. Other configured hardware paths can bypass that
particular ACK loop.

Crucially, GOP_UpdateCurrentOnce unconditionally returns1 at0x2dcef0 after
the driver call. The success value reported by MI Apply is therefore not a
strict proof of ACK or previous-buffer retirement. The sequence is the exact
working mode1 path and worth retaining, but cannot support a stronger guarantee.

MI_OSD_WaitRenderDone is not a substitute: it calls MApi_GFX_WaitForTAGID
at0x1e159c and waits for graphics-engine work, not panel scanout.

## Requirements and remaining gates

- Allocation and surfaces remain PID1-owned for the display lifetime. Do not
  free/close on uncertain window publication or on the known broken
  WindowDestroy path. Hide while retaining resources between app launches.
- Use checked geometry, page offsets,64-bit IOVAs and fd lengths. The kernel
  AP/PHY_OS surface constructor is not an allocation validator.
- Preserve uncached mapping and ensure CPU writes are ordered before the MI
  presentation calls. A release fence can order stores; it is not a substitute
  for cache maintenance if a cached mapping is introduced later.
- Implement Begin/Flip/Apply as a single serialized PID1 operation, with Apply
  attempted even if Flip fails after a successful Begin. Keep the published
  surface alive on any ambiguous result.
- Record exact hardware ACK/scanout behavior before promising strict retirement
  or tear-free presentation. A successful warm movie is practical evidence,
  not proof of the failure-case fence contract. The currently exposed API offers
  no explicit fence fd to transfer to an app.
- Minimal next hardware gate is a persistent allocation + two external surfaces
  + one external window under existing initialized OSD. Compare both pages and
  animation using the exact Begin/Flip/Apply calls, then separately test cold
  initialization without opening fbdev or publishing SYS_USER selector1.

Exact evidence (shared ignored artifacts):
assets/build/display-agent/osd-surface-{internal,validation,physical-get}.txt
assets/build/display-agent/osd-flip-{kernel,common-full,tail}.txt
assets/build/display-agent/osd-layer-config.txt
assets/build/display-agent/gop-update-mask-sync.txt
Full authoritative disassembly: assets/research/g06-display-offline/mik-full.txt
and assets/build/display-agent/utpa2k-full.txt.
