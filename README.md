# jed-tv

Lean TV userspace, bootloader experiments, proofs, host tools, and hardware
interoperability research, selected from the internal repository into one new
root commit.

Start with [the sharing review](SHARE_REVIEW.md) and
[third-party provenance](THIRD_PARTY.md). The exact source revision, file
selection, and edits are in [the manifest](sharing/source-manifest.json).

This copy retains 2,081 original files, including all 576 Lean files and 337
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
unchanged against a one-commit snapshot; its reference assembly is retained
under `src/ffh264/asm/arm`.

[README.txt](README.txt), [QEMU.md](QEMU.md), [RECOVERY_NOTES.txt](RECOVERY_NOTES.txt),
and research notes describe historical setup and results. They have not been
rerun for this selection and do not authorize new hardware operations.

Existing third-party licenses apply. No new blanket reuse license has been
chosen for the team's work. The sharing review identifies the remaining
permission questions, including original contributions from the Lean Doom
upstream. This independent TV project is not endorsed by the vendors or by
Lean FRO.
