# V109 display ordering: offline findings

No hardware access occurred. This is evidence for a later coordinated test,
not confirmation of current scanout ordering.

## Configuration and MI API boundary

`assets/research/firmware-implications/tvconfig/config/OSD/OsdConfig.ini`
places layer1 at ZORDER0, layer0 at ZORDER1, layer2 at ZORDER2. Its comment says
larger ZORDER indices are above smaller ones. Layers0 and1 both use destination0.
Thus the independent fb1/layer1 experiment can be covered by compositor layer0.

`libmi3.so` exports WindowGetZOrder and WindowSetZOrder, not layer ordering APIs.
Those change windows within a layer. Layer attribute9 (BLENDING_POSITION)
selects placement around PQ processing; it is not z-order, and the exact kernel's
LayerGetAttr size dispatch does not support querying attribute9.

The MI kernel function `_MI_OSD_SetGopMux` at0x1ca9e4 constructs up to six
{GOP:u8,mux:u8} pairs from `_stOsdLayerSetting` and calls
MApi_GOP_GWIN_SetMux at0x1caa64. It skips this while the boot logo is active.
The MI API does not expose this helper as a standalone ioctl.

## Why blanket layer hide/show is not an exact snapshot

Exact MI wire calls exist:

- Layer handle:0xc0081456, payload {handle_out:u32,layer_id:u32}.
- Layer enabled:0xc008145d, payload {handle:u32,enabled_out:u8,padding[3]}.
- Hide:0xc0041406, payload {handle:u32}.
- Show:0xc0041405, payload {handle:u32}.

All use the existing16-byte MI outer envelope with payload size and u64 pointer.

Kernel MI_OSD_LayerHide at0x1d04dc iterates every window belonging to the layer,
switches its GOP, and disables each GWIN. MI_OSD_LayerShow at0x1d020c enables
every matching GWIN. They preserve layer/window/surface objects and buffers,
but can lose mixed enabled/disabled state among multiple windows.

MI_OSD_GetLayerIsEnabled at0x1d07ac queries each matching GWIN and overwrites a
single result, returning the last matching window's state. It does not capture
a per-window vector. LayerQueryWindowHandle at0x1d0b88 returns the first match;
it does not enumerate all windows. Consequently, a blanket hide/show lease is
exactly reversible only after proving there is one window, or proving all
window enable states match. The existing layer-enabled readback alone does not
prove this. No hide/show helper is installed automatically.

## Raw Utopia ordering route recovered, unsafe to probe yet

The following layouts are backed by exact V109 libutopia.so and utpa2k.ko:

1. Open `/proc/utopia` O_RDWR.
2. ioctl0xc00c5501 with12-byte {module:u32=5,version:u32,attribute_ptr:u32}.
   GOPRegisterToUtopia at0x2deb34 registers module5.
3. ioctl0xc0085503 with8-byte {command:u32,argument_ptr:u32}.
4. GOP command10 queries ordering; command9 sets ordering.
5. Both command arguments are12-byte {selector:u32=0,payload_ptr:u32,size:u32=124}.
6. The124-byte payload is {count:u32,gop[15]:u32,layer[15]:u32}.

Evidence:

- libutopia UtopiaOpen0x7edfc8, kernel transport0x7ee098..0x7ee0d4.
- libutopia UtopiaIoctl0x7ee254, kernel transport0x7ee2a0..0x7ee2b8.
- libutopia MApi_GOP_GWIN_GetLayer0x36e41c, command10 at0x36e4ec.
- libutopia MApi_GOP_GWIN_SetLayer0x36e294, command9 at0x36e3c4.
- utpa2k GOP_adp_Ioctl0x2fa774, commands9/10 select spt_GOP_LAYER at0x2fb13c.
- utpa2k Ioctl_GOP_GetLayer0x2de418 reads hardware destination/mux mapping.
- utpa2k Ioctl_GOP_SetLayer0x2de2cc applies each pair through
  MDrv_GOP_MapLayer2Mux and MDrv_GOP_GWIN_SetMux, then can trigger the mux latch.

Critical missing initialization: GOPOpen at0x2debf8 allocates per-instance
private memory and initializes/refcounts the GFLIP timer. It does not visibly
initialize the private GOP driver-context pointer at offset0x1b47.
Ioctl_GOP_GetLayer loads that pointer at0x2de488 and dereferences it immediately
at0x2de48c. GOPIoctl checks this pointer at0x2def6c, but a null value branches to
the regular command dispatcher rather than rejecting GetLayer. Therefore a
new raw GOP instance followed directly by GetLayer is not a safe read-only
probe; it may fault the kernel.

Before preparing an ordering helper, recover an exact context-attachment call
that reuses the initialized shared GOP hardware state without hardware reset,
and audit its close path. GOPClose calls Ioctl_GOP_AtExit, so arbitrary instance
reuse is also not interchangeable with a simple fd alias. Once attachment is
proven, the intended reversible operation is: pause competing writers; query
and validate the entire live mapping; save it; swap only the two relevant mux
positions; set and read back; run the bounded display trial; restore the exact
saved mapping and verify it before resuming writers. No hard-coded INI mapping
should be written over an unqueried current mapping.

Another route avoids reordering: create our own direct MI layer on a currently
unused higher slot (for example layer2 if live mapping confirms it). That needs
a native MI layer/window/surface allocator instead of the existing fb1 bridge,
whose configured layer ID is1. It remains a separate implementation task.

## Coordinated live follow-up

The coordinator's `assets/build/backend-live-window-inspect.txt` reports both
layer0 and the newly created layer1 have WINDOW capacity1. Their single window
handles are0x40000 and0x40001. This resolves the mixed-window visibility concern
for those exact observations, provided competing writers stay paused.
`mi_layer_control.rs` is an explicitly gated layer0-only show helper: it checks
identity, capacity1 and expected handles before changing visibility.

The first bounded direct trial allocated/mapped/presented successfully in
software, but its teardown exposed a vendor defect. `_MI_OSD_WindowDestroy`
at0x1d0158 contains an early check/log path and no actual destruction routine.
It prints literal count4 and the public wrapper reports error8. The reported4
is not a measured reference count. LayerDestroy reaches the same failing helper.
fbdev's FREE ioctl0x4697 ignores that failure, destroys surfaces, and frees the
allocation anyway; this left hidden layer1/window0x40001 with stale surfaces.
Do not show it or recreate fb1 blindly.

The revised direct_mi cleanup refuses to free the allocation unless MI getters
prove no referencing window exists. It deliberately retains memory when the
window remains or queries are inconclusive. This avoids stale DMA mappings;
it does not claim to fix the vendor's destruction implementation. A persistent
PID1-owned layer/allocation is the intended next iteration model.

The fbdev curTiming1920x1080 log is a getter/fallback result, not evidence that
it set panel timing. MI_OSD_LayerCreate does change GOP scale, stretch, destination,
and reapplies the whole configured mux permutation through `_MI_OSD_SetGopMux`
at0x1cefc0. This is a possible cross-layer effect. The helper is also used by
DisableBootLogo. Controller Open is conditional on GetController failing.

GetMuteStatus takes a video-window handle, not the controller handle. Feeding
0x36004354 caused status7 before reading mute state. Corrected read-only probe
`mi_output_inspect.rs` enumerates only the kernel-validated video-window handle
range0x36000000..0x3600000f and queries flags only for existing windows. It does
not call the named MI_DISP_GetHandle API or create windows. The output-timing
getter returned12 (E_MI_DISP_TIMING_1920X1080_60P) in the coordinator's capture.
