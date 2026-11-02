GPU on the TV from our own Linux (native PID 1)
===============================================

Goal: draw with the Mali-G31 GPU from the native PID 1 image, so the city
flight demo (src/flighttv) can render on the GPU instead of the CPU.

What is known (research/g06-platform-capabilities.md, tools/pid1/README.txt):
- The GPU driver is ARM's closed stack: mali_kbase.ko in /vendor/lib/modules
  and /vendor/lib/egl/libGLES_mali.so behind Android's libEGL/libGLESv2.
- In an earlier native session, loading the stock mali_kbase module worked
  and created /dev/mali0. Nothing has drawn through it yet.
- Stock Android binaries run in the native session under the stock linker
  once mount-stock-runtime.sh exposes /system and /vendor (the audio work
  runs the vendor middleware this way).

Probe result on the G06 (2026-11-01): PASS. Mali-G31 drew the 2,000 boxes in
5.34 ms per frame; glReadPixels of a 1280x720 frame took 17.25 ms; no GL
error. Its direct /dev/fb0 presentation failed: in the native session the
display service owns the panel, so off-screen GPU rendering is confirmed and
presentation must go through the app's display path (see below).

The probe:
  research/gpu/egl-probe.rs           Android-ABI program: EGL pbuffer, GLES 2,
                                 2000 boxes, 60 frames, glReadPixels, then
                                 the frame on /dev/fb0. Prints GPU_PROBE lines.
  tools/gpu/build-probe.sh       builds it like the audio probes (Rust 1.92
                                 Android target, tools/audio/android-linker.sh,
                                 exact Bionic libraries from the firmware).
  research/gpu/gpu-probe.sh   on-TV steps: stock runtime mounted,
                                 mali_kbase loaded, probe run with the stock
                                 linker. Read-only; nothing persists.
  tools/gpu/run-probe.py         build, upload, run through the native console.

What it needs that this machine did not have:
- The TV, booted into the native PID 1 session, with the console reachable.
- assets/research/native-vendor-stack/bionic/{libc,libdl,libm,liblog}.so,
  extracted from the exact firmware, for linking.

What success looks like: GPU_PROBE gl renderer="Mali-G31", a frame time of a
few ms, a readback of a few ms, the boxes visible on the panel, GPU_PROBE_PASS.

The city flight demo on the GPU (NOT YET RUN ON THE TV):
  src/flightgpu/                 Android-ABI renderer: the same city (World
                                 seed 1337), GLES 2 into a pbuffer, readback,
                                 the HUD from flighttv, then the frame into a
                                 shared file. It reads the aircraft state
                                 flighttv publishes. It never touches /dev/fb0.
  src/flighttv/src/state.rs      the shared record and the shared frame
                                 (memory-mapped files with a sequence lock)
                                 and /tmp/flight-gpu.ready.
  src/flighttv (GPU mode)        when the ready file names a record, flighttv
                                 publishes a state per loop and presents the
                                 renderer's frames through its usual display
                                 path (fbdev in QEMU, the display service on
                                 the TV); the simulation, the Lean flight
                                 model and the filter are unchanged.
  tools/gpu/build-flightgpu.sh   builds flightgpu like the probe.
  src/pid1/scripts/flight-gpu.sh   on-TV: Mali loaded, renderer started
                                 under the stock linker, record announced.
  tools/gpu/run-flight-gpu.py    build, upload, start; then open City flight.

Two processes because one process cannot mix Bionic (the GL stack) with musl
(the Lean plant and everything else on the TV). The GPU draws the picture; the
simulation still runs in slow motion on the TV's CPU.

What to expect: flightgpu logs "GLES on ARM / Mali-G31", then the city build,
then "ready". City flight logs "GPU renderer announced; publishing state".
flightgpu then logs its frame rate and the GPU time per frame.

Known risks: no property service for Android's EGL loader (it should fall back
to scanning /vendor/lib/egl); pbuffer memory through kbase without gralloc;
SELinux is permissive in the native session, so no policy work is expected.
