"""Keep the owner's free-space reserve before expanding build artifacts."""
from pathlib import Path
import shutil

ROOT = Path(__file__).resolve().parents[2]
WORK = ROOT / 'assets/build/g13-android16'
GIB = 1024 ** 3
RESERVE = 15 * GIB


def require_space(extra_gib):
    WORK.mkdir(parents=True, exist_ok=True)
    free = shutil.disk_usage(WORK).free
    required = RESERVE + int(extra_gib * GIB)
    if free < required:
        raise SystemExit(f'Preserving 15 GiB free: have {free / GIB:.1f} GiB, '
                         f'need {required / GIB:.1f} GiB before this step')
