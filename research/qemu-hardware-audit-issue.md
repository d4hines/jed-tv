`faithful` boots the **exact dumped G06/V109 Linux 4.19.116+ kernel**, unchanged. `virt32` and `fast` are explicit media-test surrogates.

Kernel SHA-256: `01e36bda0554aed46ad3aa1d7fdf688174f46241117f62b4b95f02e25c47c11e`. QEMU checks pass for four CPUs, PID-1 threads/timers/fork/wait, execution of real `tvinit`, and a short H.264 decode/conversion test. This establishes kernel/ABI compatibility, not complete hardware emulation.

## Remaining differences, incorporating physical evidence

| Area | What is established / what remains |
|---|---|
| Display / DMA (#3) | Physical fb0 is **1920×1080**, stride 7680, one page; MI fb1 is **1280×720**, stride 5120, two pages (7,372,800 bytes). MI initialization, allocation, uncached MMA fd mapping and drawing succeeded, but **no visible native output** was observed. Legacy `smem_start` truncates 64-bit IOVAs; the first allocation at `0x200000000` makes legacy mmap fail with ENOMEM. Positive MMA import fds are success. QEMU still lacks this API/scanout model. |
| Networking (#6) | **Native Ethernet control works without Android:** HTTP 8765, PTY 8766, uploads, child execution, concurrent shells, persistent cwd/environment and Ctrl-C. QEMU still lacks a PHY/DMA packet backend. DNS/TLS, throughput, loss and reconnect remain unverified. |
| Watchdog / power / storage | **Stock watchdog servicing is physically verified**, including two 90-second observations. The exact clear is byte `1` at `0x1f006000`; QEMU now counts these writes. No expiration interval is assumed; reset/standby timing is unknown. eMMC, SDIO, PMIC, thermal sensing and firmware power-off remain unmodeled. |
| Vendor services / security | Observed blockers include missing tee-supplicant, Binder devices, explicit `/dev/vndbinder` selection, selinuxfs/status and policy classes. Missing hardware properties selected a failing fallback composer. `ro.hardware=mt5867` is a candidate, **not a measured value**. Driver/MI success cannot substitute for these prerequisites. |
| Entropy / clock (#9) | QEMU preserves early entropy-not-ready behavior. One minimal physical boot logged CRNG initialization at **120.412019 s**; this is neither a deadline nor a direct getrandom test. Validate readiness and realtime before TLS. |
| Audio / remote (#7, #8) | Speaker/DSP routing and physical IR event behavior remain unverified. Doom's OK/fire mapping is fixed; test real key release/repeat, simultaneous controls and an accessible Use button. |
| Performance (#4) | QEMU timing and immediate MIU completion cannot establish physical decoder capacity, DMA coherence or A/V drift. Measure sustained TV playback. |

The five explicitly skipped initcalls remain `mstar_mci_init`, `mmc_sdio_init`, `mtktv_regulator_init`, `mtktv_thermal_init`, `ir_dynamic_config_init`. Most RIU registers are backing cells; **`devices_ready=false` remains correct**. Watchdog counters are observability, not a timeout model. No display-success shim has been introduced.

## Evidence and reproduction

- Hardware handoff: `research/ISSUE_11_HARDWARE_DELTA.txt`; machine-readable contract: `research/g06-hardware-contract.json`. All **ten source hashes verified**. New QEMU runs attach the verified observations to `fidelity.json`, separately from emulator results.
- Main checkout: `python3 research/qemu-g06/build.py`; `python3 research/qemu-g06/check-watchdog.py`; `python3 research/qemu-g06/run.py --smoke --timeout 60`.
- App checkout: `python3 test/run.py faithful --stock-smoke --no-build`. Full run artifacts remain under `assets/build/qemu-runs/g06-stock-*/`.

Changes remain local/uncommitted. This update used recorded evidence only; no TV access. Scope is G06/V109, not G13. The earlier kexec DTB-placement mismatch remains a separate hardware-boot investigation (#1).
