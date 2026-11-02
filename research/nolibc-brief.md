# Brief: replace musl (C) with Rust in our static binaries

Goal: "all userspace is Rust + assembly". Our static apps currently link
musl, the C library bundled with Rust's `*-linux-musl` targets. Find and
prototype a way to build them with a C library written in Rust (assembly is
fine, e.g. syscall stubs or the existing NEON code), and measure the cost.

## Where (worktree /Users/d4hines/repos/dande-tv/agent1, branch d4hines/agent1)
- Work in `nolibc/` at the worktree root, as its own Cargo workspace (NOT
  under `app/`, so the main workspace keeps building). Outputs go to
  `assets/build/nolibc/`. You may path-depend on crates in `app/crates/`, but
  don't edit them; if a crate needs a change, describe it in your report.
- Never create files under /Users/d4hines/repos/dande-tv/main. Keep Bash
  calls simple (the shell guard refuses complex compound commands); use
  absolute paths; put multi-step work in scripts.
- Context: `app/` (crates tvinit, slides, ytplay, doomtv, platform, mp4);
  `app/.cargo/config.toml` (rust-lld, zig cc wrappers); `app/rust-toolchain.toml`
  (Rust 1.92). The measured syscall surface is in
  `research/syscall-census-20261031.md`; `platform::seccomp` enforces it.
- QEMU testing: `python3 qemu/run.py fast|faithful --headless --bin <name>
  --at SEC:shot:NAME --at SEC:key:QCODE ...`. run.py builds from `app/`, so
  for your binaries copy the approach (initramfs via `qemu/mkinitramfs.py`,
  kernels in `assets/build/kernel/`) into `nolibc/run.py`, or add an
  additive option to run.py to boot a prebuilt binary.

## Options to evaluate (pick, justify, prototype)
- sunfishcode's `c-scape` / `origin` / `mustang` (libc and program startup in
  Rust on `rustix`). Check which targets they support (aarch64 AND 32-bit ARM
  matter; the TV runs a 32-bit kernel) and whether they need nightly or
  `-Zbuild-std`.
- A minimal libc subset in Rust sized to our census, providing only the
  symbols Rust std and our C pieces actually import.
- Anything else credible (e.g. relibc), briefly.

## Gates
1. `tvinit` and `slides` (no C besides musl) run in QEMU on both profiles
   without musl: home screen / slides screenshots, keys work, seccomp still
   installs and nothing is blocked. Look at your screenshots with the Read tool.
2. Data for the rest: list the exact undefined libc symbols that
   `assets/build/decoder/<target>/lib/*.a` (FFmpeg H.264), ring (in ytplay)
   and doomgeneric (`assets/build/doom/<target>/`) import, for example with
   `nm -u`. Say what providing them in Rust would take, ytplay included.
3. Numbers: binary sizes vs the musl builds; toolchain requirements
   (stable vs nightly); anything lost (e.g. DNS resolver, locale, threads).

## Report (concise)
What works with evidence paths; the chosen approach and why; sizes; the
symbol inventory and a plan/estimate for ytplay and doomtv; open problems.

## Hard constraints
No TV access. No git commit/push. Don't contact YouTube. No system-wide
installs on the Mac (nix shells are fine).
