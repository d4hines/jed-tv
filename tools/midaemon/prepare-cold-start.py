#!/usr/bin/env python3
"""Stage an offline V109 config/module candidate. Never connects to a TV."""
import argparse
import hashlib
import json
import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parents[2]
EVIDENCE = ROOT / "assets/research/g06-display-offline"
MODULES = ["iniparser", "kdrv_platform", "kdrv_xc", "kdrv_alg", "libdmdbraslia",
           "utpa2k", "hdcp1x", "hdcp2x", "xcker", "mwgifker", "misck", "mik"]


def digest(data):
    return hashlib.sha256(data).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=pathlib.Path,
                        default=ROOT / "assets/research/midaemon-replacement/cold-start")
    args = parser.parse_args()
    output = args.output.resolve()
    if not output.is_relative_to((ROOT / "assets").resolve()):
        raise RuntimeError("Output must be under ignored assets/")
    if output.exists():
        raise RuntimeError("Output already exists; preserve it or choose a new directory")
    source_manifest = EVIDENCE / "candidate-manifest.json"
    candidate = json.loads(source_manifest.read_text())
    prepared = []
    seen = set()
    for item in candidate["files"]:
        target = pathlib.PurePosixPath(item["target"])
        if not target.is_absolute() or ".." in target.parts or str(target) in seen:
            raise RuntimeError(f"Invalid/duplicate target {target}")
        seen.add(str(target))
        source = ROOT / item["source"]
        data = source.read_bytes()
        if digest(data) != item["sha256"]:
            raise RuntimeError(f"Source hash changed: {source}")
        if str(target) == "/vendor/tvconfig/config/mik_init.ini":
            text = data.decode()
            # Start from the pinned STOCK file, not the old auto-display candidate.
            text, count = re.subn(r"(?m)^(MI_TUNER_INIT\s*=\s*)1(?=\s*;)", r"\g<1>0", text)
            if count != 1:
                raise RuntimeError("Expected exactly one stock MI_TUNER_INIT=1")
            flags = re.findall(r"(?m)^(MI_\w+_INIT)\s*=\s*(\d+)", text)
            if len(flags) != 35 or len(dict(flags)) != len(flags):
                raise RuntimeError("Unexpected V109 INIT keys")
            for key, value in flags:
                expected = "1" if key in ("MI_SYS_INIT", "MI_FLASH_INIT") else "0"
                if value != expected:
                    raise RuntimeError(f"Unexpected explicit-init value {key}={value}")
            data = text.encode()
        prepared.append((str(target), source, data, item.get("tier", "referenced")))
    for name in MODULES + ["mstar_fbdev_mi"]:
        source = EVIDENCE / "vendor/lib/modules" / (name + ".ko")
        prepared.append(("/vendor/lib/modules/" + source.name, source, source.read_bytes(), "module"))
    # All sources and the selected profile are checked before creating output.
    target_root = output / "root"
    target_root.mkdir(parents=True)
    manifest = []
    for target, source, data, tier in prepared:
        destination = target_root / target.lstrip("/")
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_bytes(data)
        destination.chmod(0o444)
        manifest.append({"target": target, "source": str(source.relative_to(ROOT)),
                         "source_sha256": digest(source.read_bytes()),
                         "sha256": digest(data), "bytes": len(data), "tier": tier})
    (target_root / "config").symlink_to("vendor/tvconfig/config")
    report = {"hardware_tested": False,
              "status": "Conservative explicit-init candidate, not runtime/minimality proof",
              "candidate_manifest_sha256": digest(source_manifest.read_bytes()),
              "pre_display_module_order": MODULES,
              "deferred_framebuffer_module": "mstar_fbdev_mi",
              "kernel_auto_init": ["FLASH", "SYS"],
              "rust_display_init_order": ["DISP", "DISPOUT", "OSD"],
              "rust_audio_init_order_optional": ["AUDIO", "AOUT", "PCM"],
              "ini_edits_from_stock": {"MI_TUNER_INIT": 0},
              "aliases": {"/config": "vendor/tvconfig/config"},
              "unresolved_stock_references": candidate["missing_or_outside_extracted_trees"],
              "files": manifest}
    (output / "manifest.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({"output": str(output), "files": len(manifest),
                      "bytes": sum(item["bytes"] for item in manifest),
                      "hardware_tested": False}, indent=2))


if __name__ == "__main__":
    main()
