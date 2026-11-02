#!/usr/bin/env python3
"""Validate prepared ramdisk/policy files using ARM tools; never flash or load policy."""
import argparse
import datetime
import hashlib
import importlib.util
import json
from pathlib import Path
import shlex
import subprocess

ROOT = Path(__file__).resolve().parents[2]
SERIAL = "4C0490290FA05A0C0"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("image", type=Path, nargs="?", default=ROOT / "assets/build/stock-plus-root/result")
    args = parser.parse_args()
    manifest = json.loads((args.image / "manifest.json").read_text())
    assert hashlib.sha256((args.image / "boot.img").read_bytes()).hexdigest() == manifest["boot_sha256"]
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%S.%fZ")
    output = ROOT / "assets/research/stock-plus-root/validation" / stamp
    output.mkdir(parents=True)
    remote = "/data/local/tmp/tv-root-image-check-" + stamp
    base = ["adb", "-s", SERIAL]

    def shell(words, expected=0):
        result = subprocess.run(base + ["shell", shlex.join(map(str, words))], capture_output=True, timeout=45)
        with (output / "commands.jsonl").open("a") as log:
            log.write(json.dumps({"args": list(map(str, words)), "rc": result.returncode,
                                  "stdout": result.stdout.decode(errors="replace"),
                                  "stderr": result.stderr.decode(errors="replace")}) + "\n")
        if result.returncode != expected:
            raise RuntimeError(f"Unexpected exit {result.returncode}: {words}: {result.stderr.decode(errors='replace')}")
        return result.stdout

    identity = [shell(["getprop", key]).decode().strip() for key in
                ("ro.product.device", "ro.software.version_id", "sys.boot_completed")]
    assert identity == ["G06", "V8-T221T04-LF1V109", "1"], identity
    assert shell(["getenforce"]).strip() == b"Enforcing"
    shell(["mkdir", "-m", "700", remote])
    try:
        paths = {
            "magiskboot": args.image / "validation/magiskboot",
            "magiskpolicy": args.image / "validation/magiskpolicy",
            "patched.cpio": args.image / "validation/patched.cpio",
            "restore.cpio": args.image / "validation/patched.cpio",
            "stock.policy": ROOT / "assets/research/kexec/policy/precompiled",
            "setup.sh": ROOT / "src/runtime-scripts/root/adb-root.sh",
        }
        for name, path in paths.items():
            subprocess.run(base + ["push", str(path), remote + "/" + name], check=True, capture_output=True)
        shell(["chmod", "700", remote + "/magiskboot", remote + "/magiskpolicy"])
        shell(["chmod", "600", remote + "/restore.cpio"])
        shell(["/system/bin/sh", "-n", remote + "/setup.sh"])
        shell([remote + "/magiskboot", "cpio", remote + "/patched.cpio", "test"], expected=1)
        shell([remote + "/magiskboot", "cpio", remote + "/restore.cpio", "restore"])
        subprocess.run(base + ["pull", remote + "/restore.cpio", str(output / "restored.cpio")], check=True, capture_output=True)
        spec = importlib.util.spec_from_file_location("root_image", ROOT / "tools/make-stock-plus-root.py")
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        original = module.read_cpio((args.image / "validation/stock.cpio").read_bytes())
        restored = module.read_cpio((output / "restored.cpio").read_bytes())

        def semantic(entries):
            return {name: ([fields[index] for index in (1, 2, 3, 9, 10)], content)
                    for name, (fields, content) in entries.items()}

        assert semantic(original) == semantic(restored), "Upstream restore did not recover original ramdisk"
        # Explicit file input/output only. Never add --live or omit --load.
        shell([remote + "/magiskpolicy", "--load", remote + "/stock.policy", "--magisk",
               "--save", remote + "/patched.policy"])
        subprocess.run(base + ["pull", remote + "/patched.policy", str(output / "patched.policy")], check=True, capture_output=True)
        assert shell(["getenforce"]).strip() == b"Enforcing"
        assert shell(["getprop", "sys.boot_completed"]).strip() == b"1"
        report = {
            "boot_sha256": manifest["boot_sha256"], "identity": identity,
            "magisk_recognizes_patch": True, "upstream_restore_matches_original": True,
            "setup_shell_syntax": "passed", "offline_stock_policy_patch": "passed",
            "patched_policy_sha256": hashlib.sha256((output / "patched.policy").read_bytes()).hexdigest(),
            "policy_loaded": False, "partition_writes": False, "rebooted": False,
            "hardware_boot_verified": False,
        }
        (output / "result.json").write_text(json.dumps(report, indent=2) + "\n")
        print(json.dumps(report, indent=2))
    finally:
        shell(["rm", "-rf", remote])
    print("Evidence:", output)


if __name__ == "__main__":
    main()
