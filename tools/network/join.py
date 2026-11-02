#!/usr/bin/env python3
"""Stage the private Lean Router preset and standalone Wi-Fi tools into TV RAM."""
from pathlib import Path
import sys
from stage import stage_network

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'tools/pid1'))
from graphics import console

def main():
    preset = ROOT / 'assets/private/lean-router.conf'
    if not preset.is_file():
        raise SystemExit('Private preset is missing: assets/private/lean-router.conf')
    stage_network(preset)
    console('sh /run/tv-network/join.sh')
    console('/run/tv-network/wpa_cli -p /run/tv-network/wpa -s /run/tv-network -i wlan0 status; ip -4 addr show wlan0; ip route')



if __name__ == '__main__': main()
