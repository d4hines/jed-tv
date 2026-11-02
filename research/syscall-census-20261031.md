# Syscall census: what the player asks of the kernel (2026-10-31)

Measured with ytplay's built-in tracer: `ytplay.trace=1` forks the player
as a ptrace-traced child, follows all of its threads, and counts every syscall
and every ioctl request. Workload: 10 s of 720p H.264 + AAC playback from the
replay server (plain HTTP; live HTTPS adds no new syscalls beyond getrandom,
which is already listed). Reproduce:

```
python3 qemu/run.py fast|faithful --headless --mode play --frames 240 --extra "ytplay.trace=1"
python3 qemu/syscall_census.py assets/build/qemu-runs/<run>/serial.log [--markdown]
```

## Summary

- **30 distinct syscalls, 11 distinct ioctls** on both arm64 and 32-bit ARM.
- The ioctls ARE the platform API: 6 ALSA PCM, 4 fbdev, 1 evdev; plus 1
  terminal query (TIOCGWINSZ) from the Rust standard library's console handling.
- The rest is process plumbing: threads (clone, futex, sigaltstack,
  rt_sigprocmask, prctl, sched_getaffinity, exit), memory (mmap, munmap,
  mprotect, madvise, brk), files (open/openat, read, write, writev, close,
  fstat/statx, fcntl, getdents64), sockets (socket, connect, setsockopt,
  sendto, recvfrom), time (clock_gettime on 32-bit without a vDSO,
  nanosleep), and getrandom.
- 32-bit specifics for the TV's 4.19 kernel: musl calls `clock_gettime64`
  first. 4.19 lacks the *_time64 syscalls, so it gets ENOSYS and falls back to
  the 32-bit call: two syscalls per clock read, and still correct. `statx`
  exists since 4.11.
- Finding: per-frame thread spawning in the YUV->RGB conversion created 964
  threads in 10 s. A persistent worker pool cut that to 11 threads in total
  and about 5x fewer mmap/munmap calls. Remaining mmap/munmap churn comes from
  per-segment buffers (could be reused).

## Enforced: seccomp allowlist (2026-10-31 ~14:25 PDT)

`platform::seccomp` turns the census into a seccomp-bpf filter. ytplay
installs it by default before starting any threads (`ytplay.seccomp=0`
disables it). The filter checks the architecture, allows the listed syscall
numbers, and allows `ioctl` only for the platform's request codes (taken from
the platform crate's own constants, so the 32/64-bit struct-size differences
come out right). Anything else raises SIGSYS; the handler prints the blocked
syscall (and ioctl code) and exits with 159.

- Allowlist: 46 syscalls on arm64, 52 on 32-bit ARM, 11 ioctl requests. It is
  the census union over replay playback, live HTTPS playback (adds musl DNS:
  bind, getsockname, ppoll) and the self-test, plus a few reachable-but-unseen
  paths: mremap (realloc), tgkill/getpid/gettid (abort), sync/reboot (PID 1
  power-off), lseek, sched_yield, and the 32-bit/64-bit time variants musl
  falls back between on the TV's 4.19 kernel.
- Verified on both profiles: play, self-test, framemd5 and live YouTube run
  unchanged with the filter on. A deliberate getuid() is trapped:
  "seccomp: blocked syscall 174" (arm64) / "199" (getuid32, ARM).
- The TV kernel has CONFIG_SECCOMP_FILTER=y, so the same sandbox applies on
  the device.

## arm64 (fast profile)
census: arch aarch64 threads 11 exit 0: 30 distinct syscalls, 11 distinct ioctls

| syscall | nr | calls |
|---|---|---|
| futex | 98 | 2559 |
| nanosleep | 101 | 1439 |
| ioctl | 29 | 1134 |
| read | 63 | 515 |
| mmap | 222 | 508 |
| munmap | 215 | 495 |
| madvise | 233 | 59 |
| recvfrom | 207 | 58 |
| rt_sigprocmask | 135 | 44 |
| close | 57 | 17 |
| setsockopt | 208 | 16 |
| sigaltstack | 132 | 15 |
| mprotect | 226 | 15 |
| writev | 66 | 14 |
| openat | 56 | 10 |
| exit | 93 | 10 |
| sendto | 206 | 10 |
| clone | 220 | 10 |
| fcntl | 25 | 9 |
| socket | 198 | 8 |
| connect | 203 | 8 |
| write | 64 | 6 |
| fstat | 80 | 5 |
| prctl | 167 | 5 |
| brk | 214 | 3 |
| getdents64 | 61 | 2 |
| sched_getaffinity | 123 | 2 |
| newfstatat | 79 | 1 |
| exit_group | 94 | 1 |
| getrandom | 278 | 1 |

| ioctl | code | calls |
|---|---|---|
| SNDRV_PCM_IOCTL_WRITEI_FRAMES (dir W, 'A', 0x50, 24 B) | 0x40184150 | 442 |
| SNDRV_PCM_IOCTL_DELAY (dir R, 'A', 0x21, 8 B) | 0x80084121 | 442 |
| FBIOPAN_DISPLAY (dir none, 'F', 0x06, 0 B) | 0x00004606 | 240 |
| TIOCGWINSZ (dir none, 'T', 0x13, 0 B) | 0x00005413 | 2 |
| FBIO_WAITFORVSYNC (dir W, 'F', 0x20, 4 B) | 0x40044620 | 2 |
| SNDRV_PCM_IOCTL_PREPARE (dir none, 'A', 0x40, 0 B) | 0x00004140 | 1 |
| SNDRV_PCM_IOCTL_DRAIN (dir none, 'A', 0x44, 0 B) | 0x00004144 | 1 |
| FBIOGET_VSCREENINFO (dir none, 'F', 0x00, 0 B) | 0x00004600 | 1 |
| FBIOGET_FSCREENINFO (dir none, 'F', 0x02, 0 B) | 0x00004602 | 1 |
| EVIOCGNAME (dir R, 'E', 0x06, 128 B) | 0x80804506 | 1 |
| SNDRV_PCM_IOCTL_HW_PARAMS (dir RW, 'A', 0x11, 608 B) | 0xc2604111 | 1 |

## 32-bit ARM (faithful profile)
census: arch arm threads 11 exit 0: 30 distinct syscalls, 11 distinct ioctls

| syscall | nr | calls |
|---|---|---|
| futex | 240 | 2468 |
| clock_gettime64 | 403 | 1877 |
| ioctl | 54 | 1005 |
| read | 3 | 452 |
| nanosleep | 162 | 443 |
| mmap2 | 192 | 407 |
| munmap | 91 | 395 |
| madvise | 220 | 58 |
| rt_sigprocmask | 175 | 44 |
| recvfrom | 292 | 27 |
| close | 6 | 17 |
| setsockopt | 294 | 16 |
| mprotect | 125 | 15 |
| sigaltstack | 186 | 15 |
| writev | 146 | 14 |
| exit | 1 | 10 |
| open | 5 | 10 |
| clone | 120 | 10 |
| sendto | 290 | 10 |
| fcntl64 | 221 | 9 |
| socket | 281 | 8 |
| connect | 283 | 8 |
| write | 4 | 6 |
| statx | 397 | 6 |
| prctl | 172 | 5 |
| getdents64 | 217 | 2 |
| sched_getaffinity | 242 | 2 |
| brk | 45 | 1 |
| exit_group | 248 | 1 |
| getrandom | 384 | 1 |

| ioctl | code | calls |
|---|---|---|
| SNDRV_PCM_IOCTL_WRITEI_FRAMES (dir W, 'A', 0x50, 12 B) | 0x400c4150 | 445 |
| SNDRV_PCM_IOCTL_DELAY (dir R, 'A', 0x21, 4 B) | 0x80044121 | 445 |
| FBIOPAN_DISPLAY (dir none, 'F', 0x06, 0 B) | 0x00004606 | 105 |
| TIOCGWINSZ (dir none, 'T', 0x13, 0 B) | 0x00005413 | 2 |
| FBIO_WAITFORVSYNC (dir W, 'F', 0x20, 4 B) | 0x40044620 | 2 |
| SNDRV_PCM_IOCTL_PREPARE (dir none, 'A', 0x40, 0 B) | 0x00004140 | 1 |
| SNDRV_PCM_IOCTL_DRAIN (dir none, 'A', 0x44, 0 B) | 0x00004144 | 1 |
| FBIOGET_VSCREENINFO (dir none, 'F', 0x00, 0 B) | 0x00004600 | 1 |
| FBIOGET_FSCREENINFO (dir none, 'F', 0x02, 0 B) | 0x00004602 | 1 |
| EVIOCGNAME (dir R, 'E', 0x06, 128 B) | 0x80804506 | 1 |
| SNDRV_PCM_IOCTL_HW_PARAMS (dir RW, 'A', 0x11, 604 B) | 0xc25c4111 | 1 |
