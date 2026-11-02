#!/usr/bin/env python3
"""Offline ARM32 Settings regression: real embedded supervisor, no TV/network.

Build PID1 first, then pass the directory containing ARM32 slides/doomtv/ytplay:
  python3 tools/settings/check-ui.py --apps assets/build/native-app-target/armv7-unknown-linux-musleabihf/release
Evidence (screenshots, serial log, results) is saved under assets/build/ui-audit/.
"""
import argparse
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location("ui_session", ROOT / "tools/ui-audit/session.py")
session = importlib.util.module_from_spec(spec)
spec.loader.exec_module(session)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--apps", type=Path, required=True)
    parser.add_argument("--init", type=Path, default=ROOT / "assets/build/pid1-repl/target/armv7-unknown-linux-musleabihf/release/hello-pid1")
    args = parser.parse_args()
    name = "settings-check-" + str(time.time_ns())
    process = subprocess.Popen([sys.executable, str(ROOT / "tools/ui-audit/session.py"), "start", name,
                                "--apps", str(args.apps.resolve()), "--init", str(args.init.resolve()), "--production"],
                               cwd=ROOT, stdout=subprocess.DEVNULL, stderr=subprocess.STDOUT)
    out = session.BASE / name
    result = {}
    completed = False
    s = None
    try:
        deadline = time.monotonic() + 90
        while time.monotonic() < deadline:
            if process.poll() is not None:
                raise RuntimeError("QEMU session ended before startup")
            if (out / "session.json").exists() and (out / "serial.log").exists():
                s = session.Session(name)
                if "home screen:" in s.serial():
                    break
            time.sleep(.1)
        else:
            raise RuntimeError("QEMU launcher startup timeout")
        def keys(script):
            s.run(script)
        def state():
            text = s.shell("cat /tmp/dande-tv-settings/parental").strip()
            return next((line for line in text.splitlines() if line.startswith("DANDE1 ")), "")
        def check(key, passed):
            result[key] = bool(passed)
            print(("PASS " if passed else "FAIL ") + key, flush=True)
            if not passed:
                raise AssertionError(key)
        keys("down ret ret ret shot:disabled")
        check("disable-without-pin", state() == "DANDE1 0000 246")
        wrong_pin_notices = s.serial().count("Core notice kind=0")
        keys("ret 1 2 3 4 ret shot:wrong-pin")
        check("wrong-pin-cannot-enable", state() == "DANDE1 0000 246"
              and s.serial().count("Core notice kind=0") > wrong_pin_notices)
        # Leave Settings and try YouTube itself: no child can bypass the gate.
        keys("esc esc up")
        before = s.serial().count("launcher spawned app")
        denied = s.serial().count("Core notice kind=1 app=1")
        keys("ret sleep:0.5 shot:disabled-launch")
        check("disabled-app-launch-refused", s.serial().count("launcher spawned app") == before
              and s.serial().count("Core notice kind=1 app=1") > denied)
        keys("down ret ret ret 0 0 0 0 ret")
        check("correct-pin-enables", state() == "DANDE1 0000 247")
        keys("esc down ret 0 0 0 0 ret 1 1 1 1 ret 1 1 1 2 ret shot:confirmation-mismatch")
        check("confirmation-mismatch-does-not-save", state() == "DANDE1 0000 247")
        keys("2 2 2 2 ret 2 2 2 2 ret shot:pin-changed")
        check("matching-confirmation-persists", state() == "DANDE1 2222 247")
        # Menus collect the old/new/confirmation digits without seeing the
        # stored PIN. Core rejects the complete request, not the old-PIN page.
        wrong_pin_notices = s.serial().count("Core notice kind=0")
        keys("esc ret down ret 0 0 0 0 ret 3 3 3 3 ret 3 3 3 3 ret shot:old-pin-rejected")
        check("old-pin-no-longer-authorizes", state() == "DANDE1 2222 247"
              and s.serial().count("Core notice kind=0") > wrong_pin_notices)
        keys("down*3 ret down*12 shot:proofs-scrolled esc down*2 ret sleep:0.5 shot:wifi-inactive")
        check("wifi-page-read-only-on-entry", "wpa_supplicant" not in s.shell("ps"))
        keys("esc esc")
        s.shell("printf 'invalid\\n' > /tmp/dande-tv-settings/parental")
        # Core owns its policy in memory for a controller run. Restart the
        # controller to exercise its fail-closed storage load explicitly.
        homes = s.serial().count("home screen:")
        status, body = s.http("POST", "/launcher/reload")
        if status != 200:
            raise RuntimeError(f"controller reload failed: {status} {body}")
        deadline = time.monotonic() + 15
        while s.serial().count("home screen:") <= homes:
            if time.monotonic() >= deadline:
                raise RuntimeError("controller did not reload after corrupt storage")
            time.sleep(.1)
        keys("down ret ret shot:unavailable-store esc esc up")
        before = s.serial().count("launcher spawned app")
        denied = s.serial().count("Core notice kind=1 app=1")
        keys("ret sleep:0.5")
        check("corrupt-state-refuses-launch", s.serial().count("launcher spawned app") == before
              and "Core settings unavailable" in s.serial()
              and s.serial().count("Core notice kind=1 app=1") > denied)
        completed = True
        (out / "result.json").write_text(json.dumps({"passed": completed and all(result.values()), "cases": result}, indent=2) + "\n")
        print("PASS: Settings UI; evidence " + str(out), flush=True)
    finally:
        if s is not None:
            (out / "result.json").write_text(json.dumps({"passed": completed and bool(result) and all(result.values()), "cases": result}, indent=2) + "\n")
            if s.wire is not None:
                s.wire.close()
                s.sock.close()
        if (out / "qemu.pid").exists():
            try:
                os.kill(int((out / "qemu.pid").read_text()), 15)
            except ProcessLookupError:
                pass
        try:
            process.wait(timeout=10)
        except subprocess.TimeoutExpired:
            process.terminate()
            process.wait(timeout=5)


if __name__ == "__main__":
    main()
