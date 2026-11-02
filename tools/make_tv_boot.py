#!/usr/bin/env python3
"""Build a TV boot image: the STOCK G06/V109 kernel and DTB (byte-identical)
plus OUR ramdisk, whose /init is tvinit (32-bit). Offline preparation only;
G06 fastboot boot is not established and kexec execution failed.
This script never talks to the TV or establishes a usable boot transport.

  python3 tools/make_tv_boot.py m1a   # network bring-up only: tvinit + /etc/tvinit.conf
  python3 tools/make_tv_boot.py m1b   # + the MStar display-stack kernel modules
  python3 tools/make_tv_boot.py m1c   # + /bin/ytplay and tile art: the home screen (no hold);
                                     #   YouTube via the laptop's replay server (TV has no internet)

Output: assets/build/tv-boot/<stage>/{boot.img,ramdisk.cpio.gz,manifest.json}.
Self-check: repacking the stock ramdisk with the same arguments must reproduce
the stock image's bytes up to the end of its payload (before the AVB footer).
"""
import gzip
import hashlib
import json
import pathlib
import shutil
import subprocess
import sys
import tempfile

ROOT = pathlib.Path(__file__).resolve().parents[1]
STOCK = ROOT / "assets/backups/v109-stock-20261031/boot.img"
VENDOR = ROOT / "assets/research/tv-pid1/v109/super/vendor.img"
BIN = ROOT / "assets/build/app-target/armv7-unknown-linux-musleabihf/release"
STOCK_CMDLINE = ("quiet no_autorun_log=1 loglevel=1 dbg_mutex=0 "
                 "androidboot.boot_devices=mstar_mci.0 buildvariant=user")
TV_SETTINGS = {
    "tvinit.iface": "eth0",
    "tvinit.ip": "192.168.77.2/24",     # the TV's usual address; the laptop is .1
    "tvinit.log": "192.168.77.1:5140",  # tools/udplog.py on the laptop
    "tvinit.status": "7777",            # nc 192.168.77.2 7777 -> boot log so far
}
# Display stack, in the order vendor init.fusion.rc loads it (on early-boot);
# USB, Wi-Fi, Bluetooth, Mali GPU, DTV and kheaders are left out on purpose.
DISPLAY_MODULES = ["iniparser", "kdrv_platform", "kdrv_xc", "kdrv_alg", "libdmdbraslia",
                   "utpa2k", "hdcp1x", "hdcp2x", "xcker", "mwgifker", "misck", "mik",
                   "mstar_fbdev_mi"]
MKBOOT = ["--header_version", "2", "--os_version", "11.0.0", "--os_patch_level", "2024-11",
          "--pagesize", "0x00000800", "--base", "0x00000000",
          "--kernel_offset", "0x20208000", "--ramdisk_offset", "0x21208000",
          "--second_offset", "0x00000000", "--tags_offset", "0x20208100",
          "--dtb_offset", "0x0000000022108000", "--board", ""]


def sha256(p):
    return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()


def run(cmd, **kw):
    return subprocess.run([str(c) for c in cmd], check=True, capture_output=True, text=True, **kw)


def mkbootimg(work, ramdisk, cmdline, out):
    run(["mkbootimg", *MKBOOT, "--kernel", work / "kernel", "--ramdisk", ramdisk,
         "--dtb", work / "dtb", "--cmdline", cmdline, "--output", out])


def self_check(work):
    """Repack the stock ramdisk; the payload must match the stock image byte for byte."""
    out = work / "stock-repack.img"
    mkbootimg(work, work / "ramdisk", STOCK_CMDLINE, out)
    a, b = STOCK.read_bytes(), out.read_bytes()
    if a[:len(b)] != b:
        sys.exit("self-check FAILED: repacking the stock ramdisk does not reproduce stock bytes")
    return len(b)


def debugfs_dump(src, dst):
    run(["nix", "shell", "nixpkgs#e2fsprogs", "-c", "debugfs", "-R", f"dump {src} {dst}", VENDOR])
    if not pathlib.Path(dst).exists() or pathlib.Path(dst).stat().st_size == 0:
        sys.exit(f"could not extract {src} from {VENDOR}")


def main():
    stage = sys.argv[1] if len(sys.argv) > 1 else "m1a"
    if stage not in ("m1a", "m1b", "m1c"):
        sys.exit(__doc__)
    out = ROOT / "assets/build/tv-boot" / stage
    if out.exists():
        shutil.rmtree(out)
    out.mkdir(parents=True)
    with tempfile.TemporaryDirectory() as t:
        work = pathlib.Path(t)
        run(["unpack_bootimg", "--boot_img", STOCK, "--out", work])
        payload = self_check(work)

        settings = dict(TV_SETTINGS)
        if stage in ("m1a", "m1b"):
            settings["tvinit.hold"] = "1"
        else:
            # Replay server on the laptop: test/replay.py --host 192.168.77.1
            settings["ytplay.server"] = "http://192.168.77.1:8090"
        conf = work / "tvinit.conf"
        conf.write_text("# tvinit settings baked into the ramdisk (stage %s)\n" % stage
                        + "".join(f"{k}={v}\n" for k, v in settings.items()))
        extra = [f"{conf}:/etc/tvinit.conf"]
        modules = []
        if stage in ("m1b", "m1c"):
            # Audit exact firmware/module provenance and stage literal config
            # dependencies before packaging. Native daemon libraries stay out.
            run([sys.executable, ROOT / "tools/g06_firmware.py"])
            kit = ROOT / "assets/build/g06-platform"
            firmware = json.loads((kit / "manifest.json").read_text())
            for guest in sorted(firmware["files"]):
                if guest.startswith(("/vendor/tvconfig/", "/tclconfig/")):
                    extra.append(f"{kit / 'root' / guest.lstrip('/')}:{guest}")
            mods = work / "modules"
            mods.mkdir()
            for m in DISPLAY_MODULES:
                shutil.copyfile(kit / "root/vendor/lib/modules" / f"{m}.ko", mods / f"{m}.ko")
                extra.append(f"{mods / (m + '.ko')}:/lib/modules/{m}.ko")
                modules.append(m)
            lst = work / "modules.list"
            lst.write_text("# MStar display stack, vendor init.fusion.rc order\n"
                           + "".join(f"/lib/modules/{m}.ko\n" for m in modules))
            extra.append(f"{lst}:/etc/modules.list")
        if stage == "m1c":
            # Doom is left out: its 28 MB WAD does not fit next to the modules
            # below the DTB load address. Tiles without their app are hidden.
            extra.append(f"{BIN / 'ytplay'}:/bin/ytplay")
            for t in sorted((ROOT / "assets/build/tiles").glob("*.rgb")):
                extra.append(f"{t}:/share/tiles/{t.name}")

        cpio = work / "ramdisk.cpio"
        run([sys.executable, ROOT / "tools/mkinitramfs.py", BIN / "tvinit", cpio, *extra])
        gz = out / "ramdisk.cpio.gz"
        gz.write_bytes(gzip.compress(cpio.read_bytes(), compresslevel=9, mtime=0))
        cmdline = STOCK_CMDLINE + " rdinit=/init " + " ".join(f"{k}={v}" for k, v in settings.items())
        img = out / "boot.img"
        mkbootimg(work, gz, cmdline, img)

        # Verify the result: stock kernel and DTB, our ramdisk and command line.
        chk = work / "check"
        info = run(["unpack_bootimg", "--boot_img", img, "--out", chk]).stdout
        assert sha256(chk / "kernel") == sha256(work / "kernel"), "kernel changed"
        assert sha256(chk / "dtb") == sha256(work / "dtb"), "dtb changed"
        assert sha256(chk / "ramdisk") == sha256(gz), "ramdisk mismatch"
        manifest = {
            "stage": stage,
            "purpose": "Offline stock-kernel G06 PID-1 experiment; hardware boot transport unverified",
            "hardware_ready": False,
            "hardware_gates": ["MI initialization and framebuffer readiness", "speaker routing", "boot transport"],
            "platform_kit_manifest_sha256": sha256(kit / "manifest.json") if modules else None,
            "configuration_unresolved": firmware["configuration_unresolved"] if modules else [],
            "stock_boot_sha256": sha256(STOCK),
            "stock_payload_reproduced_bytes": payload,
            "boot_img_sha256": sha256(img), "boot_img_bytes": img.stat().st_size,
            "ramdisk_gz_bytes": gz.stat().st_size, "ramdisk_cpio_bytes": cpio.stat().st_size,
            "kernel_sha256": sha256(work / "kernel"), "dtb_sha256": sha256(work / "dtb"),
            "tvinit_sha256": sha256(BIN / "tvinit"),
            "cmdline": cmdline, "tvinit_conf": settings, "modules": modules,
            "header_check": [l for l in info.splitlines() if "command line" in l or "ramdisk size" in l],
        }
        (out / "manifest.json").write_text(json.dumps(manifest, indent=1) + "\n")
    print(json.dumps({k: manifest[k] for k in ("stage", "boot_img_bytes", "ramdisk_gz_bytes",
                                               "stock_payload_reproduced_bytes", "modules")}, indent=1))
    print(f"wrote {out}")


if __name__ == "__main__":
    main()
