# JED TV

> A smart tv Linux Distro in Lean.

Our main proof results are around the init system and boot loader.

Additional highlights of the full app:
- Verified settings protocol
- A native Lean Youtube client, including an ffmpeg replacement (forgive us Claude)
- Obligatory DOOM in Lean
- Apps from other contestants
- Working Wifi (works on our hand-made router, which also runs 100% Lean on both CPU and FPGA)

## Architecture

We bought a TV loaded with GoogleTV (i.e laoded with gigabytes of proprietary firmware blobs and bloat ).

Our new distro runs exactly the following:

- Our own Lean bootloader (on the TV we were able to open and hack. The other one runs stock bootloader)
- Linux (Linux 5.15.167 and Android 14 for one, Linux 4.19.116 and Android 11 on the other)
- Userspace runs:
  - Our init application, called `pid1`, which runs the home screen as well
  - Several daemons we didn't have time to reverse engineer/reimplement (`tee-suplicant`, `wpa-supplicant`, and `udhcpc`)
  - Exactly one app at a time.

We then prove several important properties about the init system and boot loader:

## Init system

It's actually decently readable. All main types, functions, and theorem statements are in
`src/pid1/lean/TV/Core.lean`. If you know haskell you'll be fine.

Here's what we prove at a high level (w.r.t our very course and probably wrong model of Linux):

- PID1 app is the only app with all the priveleges, and each app gets exactly the priveleges it needs.
- There's a protocol for talking to pid1 that brokers many priveleges and apps must use the protocol correctly.
- Key settings (parental controls, wifi) are controlled by a parental PIN
- UX doesn't suck: when you press the home button, you always go home.
- You can sleep at night: when you press Power, it fully shuts down

All the unsafe bits are expressed as monadic effects, so you can see those declared as well.

## Bootloader

Because we didn't want to brick the TV harder than we already did, the bootloader runs 
after MBOOT, where the kernel would normally run.

The goal was to implement a bootloader that can load new images from the network, but 
only when signed properly, and provably so. We accomplsih this by proving that a failed
signature check is equivalent to returning an error code.

There are a bunch more properties proven - ask your local LLM.

## About this copy

This repository is a reviewed selection of the team's internal repository:
Lean TV userspace, bootloader experiments, proofs, host tools, and hardware
interoperability research. Start with [the sharing review](SHARE_REVIEW.md) and
[third-party provenance](THIRD_PARTY.md). The exact source revision, file
selection, and edits are in [the manifest](sharing/source-manifest.json).

It retains 2,098 original files, including all 585 Lean files and 338
research files, plus an archived historical agent log. The team-owned Zr
runtime/DSL, PID1, bootloader, proofs, applications, tests, and generators are
included. City Flight is included with the author's public redistribution permission,
reported by the owner; upstream JSBSim notices are included separately.

Backup, root, recovery, and image-patching tools operate on locally supplied
inputs. They do not include vendor firmware, proprietary libraries, APKs,
decompiled vendor sources, private credentials, or commercial game data.
The two OTA-query scripts require an authorized `TCL_OTA_APP_KEY` supplied by
the caller. Three factory-token replay helpers are held for focused review.

This is a source/research distribution, not a complete runnable TV image.
YouTube-logo media builds and
prebuilt-kernel QEMU flows still need their omitted inputs resolved.
`test/lean/zr/ffmpeg/AsmCheck.lean` expects internal Git history and cannot run
unchanged against this selected history; its reference assembly is retained
under `src/ffh264/asm/arm`.

[QEMU.md](QEMU.md), [RECOVERY_NOTES.txt](RECOVERY_NOTES.txt),
and research notes describe historical setup and results. They have not been
rerun for this selection and do not authorize new hardware operations.

Existing third-party licenses apply. No new blanket reuse license has been
chosen for the team's work. The sharing review identifies the remaining
permission questions, including original contributions from the Lean Doom
upstream. This independent TV project is not endorsed by the vendors or by
Lean FRO.
