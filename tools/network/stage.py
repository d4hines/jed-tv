"""Stage the G06 retained network helpers behind their private root boundary."""
from pathlib import Path
import struct
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools/pid1"))
from graphics import console, upload


def arm_static(path):
    data = path.read_bytes()
    if data[:7] != b"\x7fELF\x01\x01\x01" or struct.unpack_from("<H", data, 18)[0] != 40:
        raise RuntimeError(f"Not an ARM32 executable: {path}")
    offset = struct.unpack_from("<I", data, 28)[0]
    size, count = struct.unpack_from("<HH", data, 42)
    if any(struct.unpack_from("<I", data, offset + i * size)[0] == 3 for i in range(count)):
        raise RuntimeError(f"Network helper requires a dynamic interpreter: {path}")


def stage_network(preset=None):
    subprocess.run(["sh", "tools/network/build.sh"], cwd=ROOT, check=True)
    package = ROOT / "assets/build/network/wpa-supplicant"
    for name, remote in [("wpa_supplicant", "lean-wpa-supplicant"), ("wpa_cli", "lean-wpa-cli")]:
        path = package / "bin" / name
        if not path.is_file():
            path = package / "sbin" / name
        arm_static(path)
        upload(path, remote)
    scripts = ROOT / "src/pid1/scripts/network"
    for source, remote in [("prepare.sh", "wifi-prepare.sh"), ("lease.sh", "lean-lease.sh"),
                           ("join.sh", "lean-join.sh")]:
        upload(scripts / source, remote)
    if preset is not None:
        upload(Path(preset), "lean-router.conf")
    upload(scripts / "install.sh", "network-install.sh")
    console("sh /tmp/network-install.sh")
