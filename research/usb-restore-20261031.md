# V109 firmware backup restored from USB to laptop

Laptop collection time: 2026-10-31 12:30-12:36 PDT (19:30-19:36Z).
Operator: Daniel; executed by agent at Daniel's request ("move the TV
firmware from the USB stick to the laptop").

## What was transferred

The G06 / `V8-T221T04-LF1V109.000732` stock readback captured 2026-10-30,
33 images totaling 4,741,660,672 bytes (4522 MiB), plus the capture-time
manifests and notes. This is the stored firmware backup of the original TV,
not an OTA installer and not a full physical clone (userdata, RPMB and fuses
are excluded; see the dump's own `README.txt`).

Source: FAT32 volume labeled `TVBACKUP`, mounted read-write at
`/Volumes/TVBACKUP`, directory `TCL32Q3K-V109/`.

Destination (git-ignored via `.gitignore:1` `/assets/`, confirmed with
`git check-ignore -v`):

  assets/backups/v109-stock-20261031/              33 images + 10 metadata files
  assets/backups/v109-stock-20261031-usb-records/  the stick's own README.txt,
                                                   SHA256SUMS, COPY_MANIFEST.json

The destination path restores the original layout documented in `README.txt`
and recorded as the source path in the stick's `COPY_MANIFEST.json`.

## Why copy and not move

Live observation, 2026-10-31 12:31 PDT: before the transfer the laptop held
no copy of this backup. `assets/` did not exist, and a depth-6 search under
`/Users/d4hines` for `super.img`, `TCL32Q3K-V109` and `v109-stock-*` returned
nothing. The stick's `COPY_MANIFEST.json` records the originals as
`/home/eduardo/tv_hack/assets/backups/v109-stock-20261031/` — a different
host, not reachable or verifiable from this laptop.

The stick was therefore the only locally accountable copy, so it was copied
and left intact. Nothing was deleted from `TVBACKUP`. Erasing it remains a
separate decision for Daniel and has not been authorized or performed.

## Verification (reproduce)

    rsync -rt --no-perms /Volumes/TVBACKUP/TCL32Q3K-V109/ \
      assets/backups/v109-stock-20261031/          # exit 0, 43 files transferred

    cd assets/backups/v109-stock-20261031
    shasum -a 256 -c SHA256SUMS                    # 33/33 OK, 0 FAILED

Images were also checked against the independent stick-level list, which
covers the metadata files the inner list omits:

    grep '  TCL32Q3K-V109/' <usb-records>/SHA256SUMS \
      | sed 's|  TCL32Q3K-V109/|  |' > /tmp/tv-fw-inner-check.txt
    cd assets/backups/v109-stock-20261031
    shasum -a 256 -c /tmp/tv-fw-inner-check.txt     # 43/43 OK, 0 FAILED

Result: every file matches the capture-time hashes, so the laptop copy agrees
with the original read from the TV, not merely with the stick. Summed `.img`
size on the laptop is 4,741,660,672 bytes, equal to `image_bytes` in
`COPY_MANIFEST.json` and `bytes` in `host-verification.json`. The copied
stick-level `SHA256SUMS` is byte-identical to the one on the volume (`diff -q`).

Tools: rsync 3.4.1, /usr/bin/shasum (macOS), darwin 27.0.0.

## Post-copy state

FAT32 carries no Unix modes, so the backup's documented protections were
reapplied: directories `0700`, all files `0400` (the dump's `README.txt`
states originals were mode `0400` in a `0700` directory, and notes the
calibration and certificate images are unit-specific and should stay private).

Free space after: 50 GiB of 460 GiB (87% used).

## Limitations and open items

- Verified: bit-exact transfer. NOT verified: that these images restore a
  working TV. No restoration has been attempted, by this transfer or earlier.
  A verified backup is not proof of recovery.
- Whether the `/home/eduardo/tv_hack/` originals still exist is unknown from
  this laptop. Until that is confirmed, treat the stick and this laptop copy
  as the two known copies and do not erase the stick.
- `AGENTS.md` still directs readers to `docs/research-log.md` and describes
  an LG TV under a read-only, post-factory-reset scope. Neither `docs/` nor
  that log exists in this repo, and the tracked work here is TCL G06/G13.
  That file appears stale relative to this workspace; it was not edited as
  part of this transfer.
