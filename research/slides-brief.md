# Brief: a Markdown slideshow app for the TV (built at compile time)

The team wants a third app for presenting at the hackathon, on the TV itself:
a slideshow generated from a Markdown file at BUILD time.

## Context (worktree /Users/d4hines/repos/dande-tv/agent1, branch d4hines/agent1)
- Work only in this worktree; `assets/` is a symlink to main/assets. Never
  create files under /Users/d4hines/repos/dande-tv/main. The shell guard
  refuses complex compound commands (loops with variables, `cd` chains):
  keep Bash calls simple, use absolute paths, put multi-step work in scripts.
- Read main's `QEMU.md` and this worktree's `research/syscall-census-20261031.md`
  for context. Look at how the other apps are built: `app/crates/doomtv`
  (a non-PID-1 child of the launcher that powers off only when PID 1) and
  `app/crates/tvinit` (home screen: 8x8 font, dark gradient, red accent, tiles).
- Platform crate `app/crates/platform`: `fb` (fbdev 1280x720 XRGB8888,
  double-buffered by panning; `Framebuffer::open`, `back_buffer`, `present`,
  `pixel`), `input` (evdev), `remote` (NEW: `remote::Action` + `remote::poll`;
  map remote/keyboard keys to Up/Down/Left/Right/Ok/Back/Home/Next/Previous/
  PlayPause/Digit(n)/...), `seccomp` (`base_syscalls`, `platform_ioctls`,
  `install`), `sys` (`is_pid1`, `init_mounts`, `power_off`, `cmdline_arg`).
- Run as PID 1 for testing: `python3 qemu/run.py fast|faithful --headless --bin slides
  --at 2:shot:1 --at 3:key:right --at 3.5:shot:2 ...` (see `--at` in run.py:
  `SEC:shot[:NAME]`, `SEC:key:QCODE[:HOLD_MS]`). Look at your screenshots with
  the Read tool. Under the launcher it will run as `/bin/slides`.
- Toolchain: Rust 1.92 pinned (app/rust-toolchain.toml), targets
  aarch64-unknown-linux-musl and armv7-unknown-linux-musleabihf, rust-lld;
  `zig cc` wrappers exist for C in crates (avoid C if you can).

## What to build: crate `app/crates/slides` (binary `slides`)
1. **Build time**: build.rs reads the deck (env `SLIDES_MD`, default
   `slides/hackathon.md` at the worktree root; resolve paths with the
   RUNTIME `std::env::var("CARGO_MANIFEST_DIR")`, not `env!`). It parses
   it and generates Rust (`OUT_DIR/slides.rs`) with the slide structure, so
   the binary contains no Markdown parser. Malformed input fails the build
   with a line number. Emit rerun-if-changed for the deck and for images.
2. **Markdown subset** (document it in the crate doc):
   - slides separated by a line `---`
   - `# ` title, `## ` subtitle
   - paragraphs; `- ` bullets (two-space nesting)
   - `1. ` numbered lists
   - fenced code blocks
   - inline `` `code` ``, `**bold**` and `*italic*` (colour or weight is fine)
   - lines starting `Note:` = speaker notes, not shown
   - stretch: `![caption](path.png)` converted at build time to raw RGB (e.g.
     via ffmpeg on the host, like `qemu/make_tiles.py`) and embedded
3. **Rendering**: anti-aliased text, readable from a couch, at 1280x720:
   - A pure-Rust rasterizer like `fontdue`, plus free fonts. DejaVu Sans and
     DejaVu Sans Mono from nixpkgs (`nix build nixpkgs#dejavu_fonts`), copied to
     `assets/third_party/fonts/` with version and sha256 recorded, embedded with
     include_bytes. Or an equivalent you justify.
   - Word wrap.
   - Visual language matching the home screen (dark gradient, red accent); the
     title slide styled differently.
   - Code on a darker panel in monospace.
   - Footer with slide number "3 / 12" and a progress bar.
   - Pre-render each slide into RAM once and blit on change. It must be
     instant even on the slow 32-bit profile (its fbdev writes are slow:
     ~80 ms per full frame; that's QEMU, not the TV).
4. **Controls** via `platform::remote::Action`:
   - Right / Down / Ok / Next / PlayPause = next slide
   - Left / Up / Previous = previous slide
   - Digit(n) = jump to slide n
   - Back / Exit = quit (exit 0, so the launcher shows the home screen)
   - The launcher handles Home itself.
5. **Behave like the other apps**: when PID 1 (standalone testing), mount the
   pseudo filesystems and power off on quit; when a child, just exit.
   Install a seccomp allowlist (`base_syscalls` + `platform_ioctls`; no
   network needed) before spawning any threads.
6. Size: report binary sizes for both targets.

## Files you own
`app/crates/slides/**`, `assets/third_party/fonts/**`, `assets/build/slides/**`.
You may fix formatting in `slides/hackathon.md` only if the parser forces
you to; the team owns its content. Additive changes only to `qemu/*.py`.
Don't edit `app/crates/tvinit` (agent1 will add the home-screen tile),
`app/crates/platform`, `ytplay`, `doomtv` or `ffh264`.

## Hard constraints
No TV access (no adb/fastboot/192.168.77.x). No git commit/push. Don't
contact YouTube. No system-wide installs on the Mac (nix shells are fine).

## Report (concise)
What works, with screenshot paths (title slide, a bullet slide, a code slide,
the last slide) checked with your own eyes on both profiles; the Markdown
subset; fonts used (version, sha256, licence); binary sizes; build/run
commands; open problems.
