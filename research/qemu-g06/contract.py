"""Verify the hardware handoff before attaching its observations to a run."""
import hashlib
import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[2]
PATH = pathlib.Path("research/g06-hardware-contract.json")


def verified_snapshot(root=ROOT):
    raw = (root / PATH).read_bytes()
    contract = json.loads(raw)
    if contract["schema"] != 1 or contract["target"]["device"] != "G06":
        raise ValueError("unexpected hardware contract")
    if contract["target"]["kernel_sha256"] != "01e36bda0554aed46ad3aa1d7fdf688174f46241117f62b4b95f02e25c47c11e":
        raise ValueError("hardware observations target a different kernel")
    for evidence in contract["evidence"]:
        path = root / evidence["path"]
        if hashlib.sha256(path.read_bytes()).hexdigest() != evidence["sha256"]:
            raise ValueError(f"hardware evidence changed: {evidence['path']}")
    # Deliberately omit the device serial from run reports and issue text.
    return {
        "source": str(PATH),
        "sha256": hashlib.sha256(raw).hexdigest(),
        "evidence_files_verified": len(contract["evidence"]),
        "native_runtime": contract["native_runtime"],
        "watchdog": contract["watchdog"],
        "display": contract["display"],
        "vendor_dependencies": contract["vendor_dependencies"],
        "entropy": contract["entropy"],
    }
