# Direct external OSD constructor parity with working fbdev

2026-11-01, offline review of parent's src/display/osd_scanout.rs against the
exact V109 mstar_fbdev_mi.ko. No hardware access or main-file edits.

Parent applied the corrections below in main and reported17 host tests passed.
This review establishes source/ABI parity for the scoped constructor path,
not physical qualification of the new no-fbdev path.

## Required corrections found and applied by parent

The working driver initializes its LayerInfo with CUSTOM size7 at0x400.
It sets source dimensions from its1280x720 framebuffer, then
_fb_miosd_init reads actual output timing and writes destination width/height
at0x33f4/0x33f8. The supplied current live evidence reports1920x1080.
Thus this board/session's matching LayerCreate inner words are:

    [0, 1, 7, 0, 0, 1280, 720, 1920, 1080]

Using standard size2 omitted the explicit destination scaling used by the
working framebuffer.1920x1080 is the current display-controller timing, not
the physical panel resolution. A future change of output timing should obtain
and validate the new destination rather than assuming these constants.

The driver initializes WindowInfo.bPixelAlpha=true at0x43c and bIsFullFlip=false
at0x438. The original candidate used the opposite booleans. Corrected matching
external-window words are:

    [0, layer, 0, 0, 1280, 720, 1, 8, 2, 1280, 720, 0]

After creating its window and external surfaces, _fb_miosd_init makes three
additional calls explicitly, regardless of their default cached state:

| Call | Exact inner wire | ioctl |
| --- | --- | --- |
| LayerDisableTransparentColor | [layer] | 0xc004140c |
| WindowSetAlpha(enabled=true,alpha=255) | [window,0x0000ff01] | 0xc0081425 |
| LayerDisableGlobalAlpha | [layer] | 0xc0041415 |

Driver call sites are0x34a0,0x34b0,0x34bc. WindowSetAlpha's inner struct has
handle@0, enable byte@4, alpha byte@5, zero padding@6..7. Exact wrapper call is
libmi3 MI_OSD_WindowSetAlpha at0x7b22c. Disabling transparent color and global
alpha is likewise confirmed from wrapper instructions0x79406/0x7940c and
0x79f3e/0x79f44.

## No further mandatory constructor call found

The working path uses external AP-owned PHY_OS surfaces, as the new path does.
It creates the window before its surfaces; these are independent until the
first flip, so the candidate's surfaces-before-window order has no observed
kernel dependency requiring reversal. All published backing remains retained.

SurfaceClear occurs in fbdev before presenting and was not copied: the owned
MMA buffer is initialized and each complete page is written by Rust before its
first submission. Clearing after those writes would erase the submitted image.

No separate WindowShow is called by the working fbdev creation/pan path.
Its mstar_FB_EnableGwin(layer,true) calls MI_OSD_LayerShow at0x42e8, as the new
renderer does after presentation. LayerShow handles its windows. Explicit
WindowShow is not needed for parity with this observed working path.

The working immediate mode1 brackets WindowFlipByExternSurface with
LayerBeginConfig/LayerApplyConfig. This matches the candidate, including an
Apply attempt after a failed Flip to close an opened transaction. Existing
sync/lifetime caveats remain in OSD_SURFACE_SYNC_AUDIT.md.

The framebuffer driver also reads board mirror/configuration state, but no
additional mirror operation was required in the scoped working constructor
trace. Do not fabricate a mirror setting from a failed configuration read.
The cold bootstrap remains responsible for board configuration, panel/output
initialization, GOP routing and removal of the boot-logo overlay.

Evidence sources:
- assets/research/firmware-implications/mstar_fbdev_mi.ko.disassembly.txt
- assets/research/firmware-implications/lib/libmi3.so
- assets/build/direct-doom-kernel-scanout.txt (current timing and surfaces)
- assets/build/display-agent/osd-window-create-kernel.txt
- research/OSD_SURFACE_SYNC_AUDIT.md
