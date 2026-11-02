# App API v1

This is the boundary for independently implementing Doom, YouTube and other
apps while the display/audio backends change. App code may be Rust or Lean.
The launcher still selects the app, applies its UID/seccomp policy, and owns
Home/Power. Do not open device nodes, Android services or bridge files in app
code. The current backend is transitional, not the final capability broker.

## Linking

The ABI is declared in `src/platform/include/tv_app_v1.h` and implemented in
Rust in `src/platform/src/app_api.rs`. Build with `./tools/native-app/build.sh`.
The ARM32 library is `assets/build/native-app-target/armv7-unknown-linux-musleabihf/release/libplatform.a`.
The Nix demo app output also includes `libplatform.a` and `include/tv_app_v1.h`.
Link native ARM32 clients with the same Zig musl target and
`-lplatform -lpthread -ldl -lm -lunwind` and `-Wl,--gc-sections` (the library path must precede those
system libraries). An independently compiled C client was linked and run on
the G06 to verify symbol and calling-convention compatibility.
Use a thin Lean FFI wrapper to call these C-compatible symbols; the wrapper
must respect handle lifetimes and cannot retain a frame pointer after release.
The library includes Rust's native runtime support; it does not start a VM.
Existing Rust app code can use `platform::display::Display` directly.

`tv_app_api_version()` returns `0x00010000`. Functions return zero on success
and negative errno on failure, except input polling as documented below.
Handles are opaque, owned by one app and serialized by that app. One display
writer is active per app; concurrent handles/writers are unsupported.

## Display

1. `tv_display_open(&display)` chooses the supplied backend.
2. `tv_display_acquire(display, &frame)` obtains an exclusive writable lease.
3. Draw directly into `frame.pixels`; there is no pixel serialization through IPC.
4. `tv_display_present(display)` publishes and releases the lease.

`tv_display_cancel` releases without publishing; close also cancels an open
lease. A second acquisition before release returns `-EBUSY`. Each frame
reports width, height, stride in **bytes**, three channel shifts and an opaque
alpha mask. Pixels are native-endian 32-bit words:

    opaque_alpha | (r << red_shift) | (g << green_shift) | (b << blue_shift)

Write only the reported rows/pixels. Redraw the complete frame unless you
track per-buffer contents yourself. Contents are not guaranteed to match the
previously displayed frame. The pointer expires at present/cancel/close.
Present means published, not physical scanout completed. Frame acquisition
can wait for a reader, with a two-second failure deadline in the shared backend.

The Rust form is:

    let mut display = platform::display::Display::open_default()?;
    let color = display.pixel(20, 40, 80);
    let mut frame = display.acquire()?;
    frame.pixels().px.fill(color);
    frame.present()?;

Dropping the Rust frame lease cancels it. The existing framebuffer helper
methods remain compatibility APIs for the launcher and current renderers.

## Audio

Open with sample rate (8000..96000) and channels (1 or 2). Submit interleaved
signed 16-bit PCM. The write count is **samples**, not bytes or frames; it must
be a multiple of channels. The maximum call is 524288 samples. Writes may
block under backpressure. `queued` returns queued **frames** for playback-clock
synchronization. Reset discards queued audio; drain waits for it to finish.
The backend owns device setup and startup priming. Volume is controlled by
the privileged runtime; apps must not write settings or amplify global gain.

## Input and lifecycle

Open input, then poll. Poll returns 1 and a Linux keycode/value, 0 when no event
is ready, or negative errno. Values are 0 release, 1 press, 2 repeat. Keep the
handle alive: it retains excess events between polls. The launcher supplies
input through its per-launch socket; apps never open the physical remote.
The standard mappings are in `src/platform/src/remote.rs`.
Home/Power are consumed by the privileged launcher. App exit returns home;
apps must tolerate termination and must not assume a framebuffer pointer is
valid after their process lifetime. Parent cleanup and full capability
revocation are separate policy work tracked in issue 12.

## Current shared-display transport (TVF2)

This is backend code, not part of the app ABI. A 4096-byte header precedes two
RGBA buffers. Header u32 words are magic `0x32465654`, width, height, stride,
published page and sequence. Linux byte-range locks enforce buffer ownership:

- Producer takes an exclusive lock over the chosen pixel buffer before writes.
- Producer publishes page/sequence under an exclusive header lock, then releases
  the pixel lock. The atomic sequence increment has release ordering.
- Consumer snapshots under a shared header lock, takes a shared pixel lock,
  and rechecks the snapshot. It copies only if still current, then releases.
- No reader can copy a pixel buffer while a cooperating writer changes it.
  Intermediate frames may be skipped; partially written frames may not be read.

Locks are released by the kernel when a process exits. They are advisory;
this is correctness for cooperating implementations, not isolation from a
malicious app with raw mapping access. Shared mappings remain an explicit
migration exception in our sandbox.

The Android adapter still polls at up to a 3 ms idle interval and copies into
a Bitmap, then posts a Canvas to SurfaceFlinger. This is not zero-copy scanout
and not a measured input-to-photon deadline. Its logs report copy/post timing.
A future event notification or DMA-buffer backend can replace this transport
without changing the app API. QEMU's bridge adapter uses the same read leases.

## Validation for this implementation

- Exact stock-kernel QEMU: API version, acquire/present, cancellation and an
  independently running consumer holding a buffer for 250 ms.
- Graphical QEMU: the same test plus full Doom/YouTube tours through both
  fbdev/ALSA and shared-buffer/PCM backends.
- The surrogate now enables FILE_LOCKING, which the stock kernel already had.
- Real G06: independent C-link smoke test; public PCM/input ABI smoke test;
  YouTube playback and Doom frame production through owned buffers.
- Observed Doom: 35 fps and approximately 0.05-0.08 ms publication work.
  Android copy and Canvas posting averaged approximately 2.1 ms and 5.8 ms
  during the sampled playback period. Neither measurement is scanout latency.
