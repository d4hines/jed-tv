# Lean YouTube: status and plan

## Integration alongside the working TV applications

This track is retained as a separate build. `nix build .#lean-youtube` produces
`result/bin/ytplay-lean`; it does not replace the default YouTube or Doom
applications, change the launcher, or change the PID1 runtime. The default
YouTube player remains Rust and Doom remains the working Rust/C version.

The branch history and verification reports below describe the original Lean
track, including its earlier proposed default-app swap. They do not establish
that this merged build has been deployed or tested on the TV.


Original handoff was prepared on branch `d4hines/qemu-main` (worktree `../qemu-main`: the
work of `d4hines/lean-youtube`, `d4hines/lean-h264-app` and the review branch
`d4hines/qemu`, merged with main); coordinating session `main-c6`.
Architecture and milestones: GitHub issue 12. Rules for code on our runtime:
`RULES.md`. Runtime and tools: `../zr/README.md`.

Provenance: the app was ported from the Rust player, and docstrings name its
files (`yt.rs`, `http.rs`, `hub.rs`, `browse.rs`, `yuv.rs` in
`src/ytplay/src`; `src/mp4`). They were removed on that development branch in commit 83ea4ca, but are retained
in the integrated default application;
read them with `git show 83ea4ca~1:src/ytplay/src/yt.rs` and the like.
FFmpeg's original assembly (`src/ffh264/asm/arm`) is in the same commit's
parent; `test/lean/zr/ffmpeg/AsmCheck.lean` and `tools/Fixtures.lean` (mp4)
take what they need from there.

## Historical Lean-track decisions (2026-11-01)

- As much in Lean as possible, in any form (low-level Lean and the assembly
  DSL included). AAC and H.264 are written in Lean; H.264 uses FFmpeg's NEON
  assembly as written in our DSL (`../zr/ZrAsm`).
- The swap (approved 2026-11-01): the Lean app is the YouTube tile
  (`/bin/ytplay`); the Rust player (`src/ytplay`, `src/mp4`) and FFmpeg's C
  decoder (`src/ffh264`, `src/ytnative`, `tools/ffh264`) are removed.
- Runtime: dande-router's Lean-written runtime, copied into `../zr` (not
  pinned). Not upstream's libleanrt.
- Keep other agents informed: milestones go to issue 12; the pointer in
  `AGENTS.md` on main names this track.

## Done and verified (QEMU, except where noted)

- Hardware: another session ran the runtime's conformance binary (from
  8405dda) on the physical G06 as a child process; stdout and panic output
  matched upstream Lean byte for byte (research/LEAN_RUNTIME_HARDWARE.txt).

- Runtime on 32-bit ARM Linux: conformance transcript byte-identical to
  upstream Lean (`zr-build conformance`, a gate).
- `ZrLean.Low.Str` (strings etc.) and `ZrLean.Low.InitShim` (verified
  standard-library shim) in Lean; `zrlow-check` passes.
- Tools: `zr-build` (`tools/lean/zr/ZrBuild.lean`: the runtime, apps, link-check,
  ARM transcripts), `test/demo-app.py`.
- Platform for apps (`../zr/ZrTv`, `Yt/Platform.lean`): system calls,
  argv/env, tv_app_v1 (display lease + fill/RGB blit/alpha mask, audio,
  input), the H.264 kernels (`ZrTv.H264Kernels`: DSL NEON and low-level
  Lean) and the YUV blit of Lean pictures (`ZrTv.Video.blitYuv`, Lean over
  DSL routines).
- `Yt.App.Hello` under the real launcher in the graphical QEMU demo: confined
  app 1, display, remote keys (with the TVAPP_REMOTE fix), antialiased text.
- Text: `tools/make-font.py` -> `Yt/Ui/FontData.lean` (generated) +
  `Yt/Ui/Text.lean`.
- The app: `Yt.App.Model` (pure update; `test/app` transcript identical on
  host and ARM32), `Yt.App.View`, `Yt.App.Main` (entry `yt_app_main`). In
  the QEMU demo it runs as the YouTube tile (`ytplay.mode=app`): home
  screen, search keyboard typing, GO -> search effect. Network effects are
  stubbed. Build: `lake -d tools/lean/zr exe zr-build app src/lean/yt Yt.App.Main yt_app_main OUT`; run:
  `test/lean/yt/demo-app.py OUT --launch down,ret --tvinit <local tvinit> ...`.
- Networking: `Yt.Net.Conn` (nonblocking TCP over poll, SO_ERROR on
  connect). From the confined app in QEMU: POST to the replay server's
  player API -> "HTTP/1.1 200 OK" (temporary probe on Play).

- Merged `d4hines/lean-neon` (f83ab64): ffh264's ARM32 assembly written in
  the Lean DSL (`../zr/ZrAsm/Ffmpeg`), 139/139 functions byte-identical
  (`tools/ffh264/asm-check.py`), framemd5 4431/4431; NEON YUV kernel
  (`../zr/ZrAsm/Yuv.lean`) used by ytplay and by `ZrTv.Video.blit`. DSL API
  notes: `ldr`/`str` take an `Addr`; never use `<<<` on a `Reg` (Fin 16).

- Merged d4hines/lean-tls (63ab2e6): `Yt.Crypto` (SHA-2, HMAC/HKDF,
  AES-GCM, ChaCha20-Poly1305, X25519, P-256/P-384 ECDSA, RSA), `Yt.X509`
  (GTS Root R1/R4 only), `Yt.Tls` (TLS 1.3 client, pure: `start`/`feed`/
  `send`/`close`, `sigCache` for repeat connections). RFC 8448 byte-exact,
  OpenSSL interop 18/18, live handshakes to all three host types. Open: about
  21 ms per handshake on the Mac (10 ms cached), so reuse connections; AES/GHASH
  are table-based (not constant-time).
- Merged d4hines/lean-formats (4a1ceb0): `Yt.Json` (index parser,
  `Yt.Json.YouTube`: player/search bodies and extraction, pickVideo/Audio),
  `Yt.Http` (Url, request, incremental `Parser`, CONNECT `Tunnel`),
  `Yt.Mp4` (identical to src/mp4), `Yt.Jpeg` (exact to libjpeg-turbo,
  `scaleNearest`). Concern: JPEG may be 10-20x slower on the TV.

- Merged d4hines/lean-aac (0066f3c): integer-only AAC-LC (`Yt.Aac`), within
  1 LSB of FFmpeg on all of 140.m4a; the runtime has no Float arithmetic.
- The app end to end in the QEMU demo (offline server `test/server.py`):
  search (JSON + JPEG thumbnails), player API with visitor retry, playback
  (`Yt.App.Player`: MP4 fragments -> FFmpeg -> blit; AAC -> tv_app_v1 audio;
  video follows the audio clock), pause, seek, overlay.
- ffh264 fix: dangling `hw_configs` pointer from c2rust (760fdc5), latent in
  main's ytplay too (told main-69).
- Live YouTube in QEMU (`demo-app.py --live`, ba3c4be): resolve, TLS streaming
  from googlevideo through a CONNECT proxy, video + Lean AAC audio.

- No C in the runtime or app entry (399a56d): `zr_sys6` and `main` in the DSL
  (ZrAsm.Linux, `zr-asm-gen app`), heap/fatal/panic in Lean (ZrLean.Low.Linux).
- Casting from the YouTube phone app (5094691): `Yt.Lounge` ("Link with TV
  code", by the research subagent, 74-check transcript) wired into the app;
  TV code on the home screen; 8/8 relayed commands with a scripted phone
  against live YouTube in QEMU. Real phone: not yet tried. Try it with
  `demo-app.py ... --live --window` (enter the code shown on screen).
- The app side of tv_app_v1 in Lean (63203f6): `ZrTv.Tv` opens shared
  frames (TV_FB_PATH, TVF2 header, region locks via fcntl64) or fbdev, PCM
  over ALSA ioctls or the TCP adapter (TV_AUDIO_ADDR), and the remote's input
  socket. The Lean app no longer links `src/platform`. Verified on the demo
  backends (fbdev/ALSA) and through `bridge-init` (shared frames + TCP PCM)
  in QEMU. The bridge needs the repo's current kernel (`demo/kernel`, with
  FILE_LOCKING); older cached media kernels time out on the region locks.
- The YUV blit in Lean (`ZrTv.Video.blit`): rows through the DSL NEON
  kernel, nearest-neighbour scaling through a new DSL routine
  (`zr_u32_scale_row`, exact `x * sw / w` without division, checked in the
  ARM model: `test/lean/zr/yuv/ScaleRow.lean`, 13/13), repeated rows copied.
  Both DSL routines are in the runtime's `tv.S`. `src/ytnative` is now only
  ffh264. Verified: offline 320x180 and 640x360 (fbdev and bridge/RGBA),
  live 1280x536 and 854x480 (tail block) with the 8/8 phone-cast checks.

## Path to 100% Lean (`tools/Composition.lean` on the linked app)

2026-11-01, after the swap and without libc: Lean 99.4%, DSL 0.6% (FFmpeg's
NEON H.264 kernels, the YUV blit, `_start`/main/memcpy/memmove/64-bit
division), linker tables 717 bytes; app 3.23 MB. Before the swap: Lean
69.9%, FFmpeg c2rust 22.7%, musl 3.5%, linker tables 2.5%, DSL 0.7%, Rust
core 0.6% (3.11 MB).

1. Done: our C glue; the app side of tv_app_v1 in Lean; the YUV blit in
   Lean + DSL; the runtime build's C passes, module closure and ARM
   conformance driver in Lean (`zr-cgen`, `zr-asm-gen`).
2. Done (2026-11-01): the Lean H.264 decoder in the app (bit-exact with
   FFmpeg on every frame of the YouTube fixture and the libx264 feature
   clips; NEON kernels from the DSL, scalar hot spots as low-level Lean
   kernels); FFmpeg, its Rust wrapper and the Rust player removed.
3. Done (2026-11-01): no C library in apps. `ZrAsm.Libc` (with each app's
   main, `zr-asm-gen app`): `_start`, NEON memcpy/memmove,
   `__aeabi_uldivmod` (checked against `Nat` division in the ARM model,
   test/lean/zr/libc/Uldivmod.lean); `zr-build app` links `-nostdlib` and
   compiles without unwind tables. The runtime conformance binary links the
   same way and stays byte-identical to upstream Lean.
4. Next: the decoder's speed on the TV (see below); Lean PID 1 (issue 12)
   with DIAL; NEON semantics in the DSL for proofs.

## Lean H.264 decoder (in progress, started 2026-11-01)

Goal (reached 2026-11-01: the app plays through it): replace FFmpeg with a Lean decoder, bit-exact
with FFmpeg on `assets/yt-stage0/record-PVGeM40dABA-visionos-20261031T200633Z/
fixtures/136.mp4` (1280x536 High, CABAC, 8x8 transform, B-frames, spatial
direct, 3 refs, POC 0, deblock, no weighted pred/scaling/MMCO). FFmpeg stays
in the app until the Lean decoder is bit-exact and benchmarked on the TV.
Modules (`Yt/H264/`, LGPL-2.1+ as derived from FFmpeg 9.0.1, whose source is
`/nix/store/cizg7bxjfvcska1fjnpa68cf7l5ysapf-ffmpeg`):
- Done: `Tables` (generated by `tools/H264Tables.lean` from FFmpeg's C),
  `Bits` (NAL unescape, header bits), `Params` (SPS/PPS, scaling lists),
  `Slice` (slice header), `Cabac` (engine; transIdx checked against FFmpeg).
  `test/h264` (`h264dec FILE N`): parses the fixture's headers correctly.
- Subagents (own worktrees, merge when done): `Intra` + `Transform`
  (branch d4hines/lean-h264-intra), `Inter` (qpel/chroma MC, weights) +
  `Filter` (loop filter edges) (branch d4hines/lean-h264-inter); verified
  against FFmpeg's C templates.
- Done (coordinator, committed fe7fea2 and after): `State` (scan8 caches),
  `Caches` (neighbours, MV prediction, P_Skip), `MbCabac` (all syntax,
  residuals, I_PCM), `Direct` (spatial and temporal), `SliceData`, `Decoder`
  (POC 0/2, ref lists + modifications, marking, weighted-pred tables, output
  in POC order). Parse check: all 4431 samples of 136.mp4, every slice ends
  exactly at its data (<= 7 bits after end_of_slice_flag).
- Done: `Recon` (intra, MC with explicit/implicit weights, residuals with
  FFmpeg's DC-only shortcuts; the picture is deblocked afterwards, so intra
  sees unfiltered neighbours) and `Loop` (fill_filter_caches, check_mv with
  DPB-index picture identity, filter_mb_dir with per-slice parameters).
- BIT-EXACT with FFmpeg: all 4431 frames of 136.mp4 (deblocked), and the
  libx264 feature clips (`lake exe h264clips` in test/h264: weighted pred, temporal
  direct, multiple slices, JVT cqm, b-pyramid, no 8x8, odd size, motion).
  `lake exe h264compare FIXTURE N [--no-deblock]` compares the Lean decoder
  with ffmpeg on exactly the same samples, frame by frame (test tools in
  Lean; `h264fixture` writes the transcript fixture).
- Our runtime: `zr-build transcript test/h264` (transcript of 80 frame hashes,
  host vs 32-bit ARM in QEMU) PASS; link-check of all of Yt PASS (after
  the Int runtime merge, d4hines/lean-int).
- Speed (first 16 samples of 136.mp4, QEMU -icount instruction counts on
  32-bit ARM, `zr-build timed test/h264 h264_test_bench 0,1,2 -- -icount
  shift=0`; where they go:
  `zr-build profile test/h264 h264_test_bench 2`): 506 M -> 42 M
  instructions/frame for the full decode (entropy ~25 M, reconstruction
  ~10 M, deblocking ~7 M, including the clip's parsing and output crop).
  What it took, in order of gain: positions in USize instead of Nat (Nat
  arithmetic costs overflow checks and boxing per operation; Yt.Bytes
  u8/u16/u32 read with one size check); planes/caches/tables taken out of
  the state before writing; state updates through small branch-free
  @[noinline] setters (an update after a branch or borrowed call retains
  and releases all 36 fields; inlined ones also had projections hoisted);
  the state split (`Dec.sl`, `Dec.pic`); CABAC bin loops as recursions;
  edge emulation only for the samples a filter reads; FFmpeg's NEON MC,
  loop filters and inverse transforms (DSL) and Low-style kernels for the
  scalar hot spots (fills/copies, boundary strengths, intra copy
  prediction, bS = 4 filters) through `Yt.H264.Kernels`, the Lean code
  being each kernel's host specification. RULES.md "Performance pitfalls"
  has the general lessons.
- Baseline: FFmpeg (ffh264, one thread, its NEON) decodes the same 16
  samples in 158 M instructions, ~9.9 M/frame (same output), measured with
  the same QEMU -icount method (harness: a C main over ytdec_* linked with
  libytnative, kept under assets/build/ffh264-bench, not committed). The
  Lean decoder is at ~42 M/frame, ~4.3x FFmpeg.
- Estimate for the TV: at an IPC of 0.7-1.0 on a 1.5 GHz A55, 42 M is
  28-60 ms/frame for 1280x536 (17-35 frames/s; 720p ~1.35x slower).
  The TV benchmark (owner's go-ahead) would replace the estimate.
- Next for speed: entropy decoding is the largest stage (macroblock-type
  and motion decoding, cache fills, CABAC); intra 4x4 directional modes and
  the residual passes still compute in Nat.
- Not implemented: CAVLC (tables generated; YouTube's H.264 is CABAC High),
  interlace/MBAFF, 4:2:2/4:4:4, >8-bit, error concealment (FFmpeg's ER).
- Conventions: planes are ByteArrays with a stride and 256 bytes of slack
  (`State.planeSlack`); coefficients int16 in a ByteArray, raster order,
  dequantized (FFmpeg's qmul formulation: (level*qmul+32)>>6,
  qmul = normAdjust*scale << (qp/6+2) for 4x4, << qp/6 for 8x8); engine
  state structures updated in place with results in fields (RULES.md).

## Next (coordinator)

1. The TV (needs the owner's go-ahead): run the Lean app through the
   existing native-app runtime on the G06 (`tools/native-app/build.sh` now
   builds it as `ytplay`); measure decode/JPEG/TLS cost.
2. Done (2026-11-01, branch d4hines/lean-h264-app): the Lean app is the
   YouTube tile. `demo/package.nix`, `test/run.py`, `test/tv.sh`,
   `tools/make_tv_boot.py` and `tools/native-app/build.sh` build it with
   `zr-build app`; `test/check.py`'s YouTube gates check the Lean app on
   virt32 (the aarch64 "fast" profile cannot run 32-bit code on Apple CPUs)
   and the Lean decoder on the host and in QEMU. The Rust player's own
   framemd5/bench/selftest/seccomp modes went with it; the launcher's
   seccomp program (`TV/Sandbox.lean`) still confines the app. The app now
   implements play mode (`ytplay.video=ID`, as the launcher's video tiles
   start it: that video, then exit) and shows late pictures instead of
   dropping them, skipping non-reference pictures while behind (in QEMU the
   decoder runs below real time at 1280x536; dropping every late picture
   showed one frame in 76 s). `test/run.py` boots tvinit by default (the app
   runs under it) and virt32 uses the repo's kernel (`demo/kernel`, with the
   AF_UNIX sockets the launcher's remote needs; the old pinned surrogate
   predates them).
3. Merged with main on `d4hines/qemu-main` (2026-11-01): main's Lean Slides
   and Lean Doom, and the launcher in the native supervisor. `test/check.py`'s
   Doom gate now opens Doom from the home screen (the Lean Doom cannot run as
   /init) and the Slides gate uses the repo's virt32 kernel (AF_UNIX); the
   check that the DSL's FFmpeg assembly assembles to FFmpeg's objects is back
   (`test/lean/zr/ffmpeg/AsmCheck.lean`, a gate).
4. Keep shrinking non-Lean code (path above). Ported to Lean on
   2026-11-01: the H.264 test tools (h264compare/h264clips/h264fixture),
   the ARM test runner (zr-qemu32, with its own initramfs writer), the
   generators of HuffData.lean, JpegTest/Fixtures.lean, Rfc8448.lean and
   LoungeTest/Fixtures.lean, and tools/Composition.lean. Remaining
   non-Lean in this track, by kind (about 1.2k lines, all host-side):
   - Independent oracles, deliberately not Lean: test/json/ref.py (Python's
     json), test/jpeg/ref.py (libjpeg-turbo), test/crypto/gen_vectors.py
     (Python crypto), test/http/capture.py (its requests mirror the old Rust
     client's, its TLS is Python's, and its response decoding is what the
     Lean HTTP parser is checked against), OpenSSL (test/tls/Interop.lean
     drives it; a gate), and FFmpeg itself for H.264/AAC.
   - Host drivers: test/server.py and test/demo-app.py extend main's Python
     QEMU harness (test/replay.py, QMP), so they move with it;
     test/lounge/live.py performs the Lean lounge session's requests on the
     host, with research/dial-lounge's phone stand-in (the app on the TV does
     that IO in Lean); tools/make-font.py rasterizes with FreeType's hinting
     (a Lean rasterizer would change how the text looks).
   - The profiler's QEMU TCG plugin (tools/lean/zr/profile/plugin.c: host code
     against QEMU's C plugin API).
   Lean since 2026-11-01: the fixtures around the oracles (tools/Fixtures.lean
   replaced five gen-fixtures scripts and the AAC one's embedded Python); the
   TLS interop driver (Interop.lean replaced interop.sh); and `zr-build`
   (`tools/lean/zr/ZrBuild.lean`): the app and runtime build (it replaced
   build-app.sh, rt-build.sh, gen.sh, link-check.sh, with byte-identical
   output), the ARM transcript runner, the instruction counts, the profile
   and the runtime conformance check (they replaced arm32-transcript.sh and
   the bench, profile and conformance scripts, whose C mains are now DSL
   entries: `ZrAsm.Linux.transcriptEntry`, `timedEntry`; symbols come from
   the ELF symbol table, not llvm-nm).

## Known issues

- YouTube refuses many videos to the app's VISIONOS client with
  `UNPLAYABLE "This video is not available"`, notably most uploads of the
  big studio and children's channels (all official Bluey and Minions
  results on 2026-11-01); live streams cannot play either (no `sidx`).
  Search therefore checks each result (up to 32, four at a time) with the
  same /player request playing would use and shows only those that pass,
  in YouTube's order; the status line counts the hidden ones. Transport or
  parse failures of a check show the result rather than hide it.

- QEMU's fbdev costs ~150 us per 4 KiB page written per frame; frame rates
  there are not meaningful (the TV uses the shared-buffer path).
- The blit is single-threaded (the runtime is); the Rust wrapper it replaced
  used a 4-thread pool. In QEMU (emulated NEON, fbdev) 120 frames now take
  ~14 s of media instead of ~10 s. Estimated ~1-2 ms per 720p frame on one
  A55 core, but unmeasured: part of the TV benchmark.
- Network apps still hold socket/connect (the PID 1 broker is issue-12
  stage 4). The broker will also need UDP for Doom's LAN games (port 2342,
  broadcast) and possibly multicast/listening for phone casting.
