# dande-tv

## A smart TV where every process is ours

Minimal YouTube on a TCL 32" TV, in Rust

Hackathon, October 2026

---

# The goal

- The smallest possible YouTube player for our TV
- Highly secure and minimal: know exactly what it needs
- Rust now; Lean and proofs next
- Shrink the stack until only the kernel isn't ours

---

# The TV

- TCL 32Q3K: MediaTek MT9221, 4x Cortex-A55
- 32-bit ARM Linux 4.19 kernel, Android 11 on top
- 1280x720 panel, 100 Mbit Ethernet, no internet
- Bootloader unlocked; firmware backed up bit for bit

---

# What YouTube really needs

No JavaScript engine, no tokens, no cookies, no login:

```
1. POST /youtubei/v1/player        -> "not a bot?" + visitorData
2. POST /youtubei/v1/player again  -> formats with direct URLs
3. GET  <url>  Range: bytes=a-b    -> 206 Partial Content
```

- H.264 720p + AAC, both in MP4

---

# Shrinking the stack

- Before: NewPipe on Java on Android on Linux
- After: our PID 1 (tvinit) runs our apps on Linux
- Each app is its own process; Back returns home
- Only foreign code left in charge: the kernel

---

# The player

- Own MP4 demuxer, checked against ffprobe on every sample
- FFmpeg's H.264 decoder with NEON, symphonia AAC, rustls TLS
- Bit-exact: 4431 of 4431 frames match FFmpeg
- Full film, 0 dropped frames, audio/video within 20 ms
- One static binary: 4.3 MB

---

# The exact API

- 33 syscalls, 11 ioctls for live 720p YouTube
- ioctls: 6 ALSA audio, 4 framebuffer, 1 input
- Enforced with seccomp: anything else is trapped

```
seccomp: blocked syscall 174
```

---

# QEMU first

- Linux 6.18 from tinyconfig: PID 1 in 60 ms
- A 32-bit profile that mirrors the TV's kernel
- Headless harness: screenshots, key presses, audio capture

---

# Doom

- The original GPL engine plus Freedoom data
- 35 fps at about 10% of one core
- Sound effects through the same ALSA code
- Launched from the home screen like any app

---

# On the real TV

- Stock kernel and device tree, byte for byte
- Our ramdisk: tvinit as PID 1
- Network first: logs stream to the laptop over UDP
- The IR remote is built into the kernel: no Android needed

---

# Next

- Everything in userspace in Rust plus assembly
- c2rust for the H.264 decoder and Doom; keep the NEON code
- Then Lean proofs, starting with the parser and the sandbox

---

# Thank you

Press Back to return to the home screen
