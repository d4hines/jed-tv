#!/usr/bin/env python3
"""Prototype YouTube Lounge screen ("Link with TV code"), run on the host.

  python3 screen.py                 # show a pairing code, print commands
  python3 screen.py --dial          # also answer DIAL/SSDP on the home LAN
  python3 screen.py --dial --ip 127.0.0.1   # DIAL on loopback only (self-test)

It holds a persistent screen ID (state file under assets/), opens the bind
channel, prints a 12-digit code to enter in the phone's YouTube app
(Settings > Watch on TV > Enter TV code) and then prints every command it
receives. Playback is simulated: a clock that advances while "playing", so
the phone sees plausible nowPlaying / onStateChange / onVolumeChanged replies.

Ctrl-C ends it (it sends loungeScreenDisconnected and stops any DIAL server).
"""

import argparse
import datetime
import json
import os
import pathlib
import queue
import signal
import sys
import threading
import time

import lounge

HERE = pathlib.Path(__file__).resolve().parent
ASSETS = HERE.parents[1] / "assets" / "research" / "dial-lounge"

# YouTube IFrame player state numbers, which is what real screens report
# (pyytlounge's State enum, observed from TVs): -1 unstarted, 0 ended,
# 1 playing, 2 paused, 3 buffering, 5 cued.
UNSTARTED, ENDED, PLAYING, PAUSED, BUFFERING = -1, 0, 1, 2, 3
STATE_NAMES = {-1: "unstarted", 0: "ended", 1: "playing", 2: "paused", 3: "buffering", 5: "cued"}


def utc():
    return datetime.datetime.now(datetime.UTC).strftime("%Y-%m-%dT%H:%M:%S.%fZ")


class Transcript:
    """JSONL log of every exchange (lounge tokens shortened)."""

    def __init__(self, path):
        self.path = path
        path.parent.mkdir(parents=True, exist_ok=True)
        self.f = open(path, "a", buffering=1)
        self.lock = threading.Lock()
        self.secrets = []

    def __call__(self, kind, **fields):
        rec = {"t": utc(), "kind": kind, **fields}
        line = json.dumps(rec, ensure_ascii=False, default=str)
        for s in self.secrets:
            line = line.replace(s, s[:8] + "...")
        with self.lock:
            self.f.write(line + "\n")


def say(*parts):
    print(time.strftime("%H:%M:%S"), *parts, flush=True)


class FakePlayer:
    """Just enough player state to answer the phone convincingly."""

    def __init__(self, duration):
        self.default_duration = duration
        self.video_id = None
        self.list_id = None
        self.video_ids = []
        self.index = 0
        self.ctt = None
        self.params = None
        self.state = UNSTARTED
        self.base = 0.0           # position when the clock was last set
        self.since = None         # monotonic time playback (re)started
        self.duration = 0.0
        self.volume, self.muted = 50, False
        self.cpn = lounge.zx()[:16]
        self.pending_play_at = None  # buffering -> playing transition time

    def position(self):
        p = self.base
        if self.state == PLAYING and self.since is not None:
            p += time.monotonic() - self.since
        return min(p, self.duration) if self.duration else p

    def set_position(self, t):
        self.base, self.since = float(t), (time.monotonic() if self.state == PLAYING else None)

    def load(self, video_id, start):
        self.video_id = video_id
        self.duration = float(self.default_duration)
        self.cpn = lounge.zx()[:16]
        self.state = BUFFERING
        self.set_position(start)
        self.pending_play_at = time.monotonic() + 1.0

    def play(self):
        self.base = self.position()
        self.state, self.since = PLAYING, time.monotonic()
        self.pending_play_at = None

    def pause(self):
        self.base = self.position()
        self.state, self.since = PAUSED, None

    def stop(self):
        self.state, self.since, self.base = UNSTARTED, None, 0.0
        self.pending_play_at = None

    # -- outgoing maps --------------------------------------------------------
    def now_playing(self):
        if not self.video_id:
            return ("nowPlaying", {})
        pos = self.position()
        return ("nowPlaying", {
            "videoId": self.video_id, "currentTime": f"{pos:.3f}", "state": self.state,
            "duration": f"{self.duration:.3f}", "loadedTime": f"{self.duration:.3f}",
            "seekableStartTime": "0", "seekableEndTime": f"{self.duration:.3f}",
            "cpn": self.cpn, "listId": self.list_id,
            "currentIndex": self.index if self.list_id else None, "ctt": self.ctt,
            "params": self.params})

    def state_change(self):
        pos = self.position()
        return ("onStateChange", {
            "state": self.state, "currentTime": f"{pos:.3f}",
            "duration": f"{self.duration:.3f}", "loadedTime": f"{self.duration:.3f}",
            "seekableStartTime": "0", "seekableEndTime": f"{self.duration:.3f}",
            "cpn": self.cpn})

    def volume_changed(self):
        return ("onVolumeChanged", {"volume": self.volume, "muted": self.muted})

    def prev_next(self):
        has = bool(self.video_ids)
        return ("onHasPreviousNextChanged", {
            "hasPrevious": has and self.index > 0,
            "hasNext": has and self.index < len(self.video_ids) - 1})


class Screen:
    def __init__(self, args, log):
        self.args, self.log = args, log
        self.player = FakePlayer(args.duration)
        self.inbox = queue.Queue()
        self.stopping = threading.Event()
        self.session = None
        self.session_gen = 0
        self.code = None
        self.code_at = 0.0
        self.remotes = {}  # id -> name
        self.state_path = pathlib.Path(args.state)
        self.load_state()

    # -- identity -------------------------------------------------------------
    def load_state(self):
        st = {}
        if self.state_path.exists():
            st = json.loads(self.state_path.read_text())
        self.screen_id = st.get("screen_id")
        self.device_id = st.get("device_id") or lounge.new_device_id()
        if not self.screen_id:
            self.screen_id = lounge.generate_screen_id()
            say("generated screen ID", self.screen_id)
            self.log("generate_screen_id", screen_id=self.screen_id)
        self.save_state()

    def save_state(self):
        self.state_path.parent.mkdir(parents=True, exist_ok=True)
        self.state_path.write_text(json.dumps(
            {"screen_id": self.screen_id, "device_id": self.device_id}, indent=1) + "\n")

    def refresh_token(self):
        tok = lounge.get_lounge_token(self.screen_id)
        self.token = tok["loungeToken"]
        self.log.secrets.append(self.token)
        self.log("get_lounge_token_batch", **{k: v for k, v in tok.items() if k != "loungeToken"})
        say(f"lounge token ok (lifespan {tok.get('loungeTokenLifespanMs', 0) // 86400000} days)")

    # -- session --------------------------------------------------------------
    def device_info_params(self):
        """deviceInfo on bind. The --additional-data DIAL shape needs it, with
        "supported" (kodi-yt-caster 2.0.14); SmartTube's shape sends none."""
        a = self.args
        if not (a.device_info or a.additional_data):
            return None
        info = {"brand": "TCL", "model": "32Q3K", "year": 0, "os": "Linux",
                "osVersion": "", "chipset": "", "clientName": "TVHTML5",
                "dialAdditionalDataSupportLevel": "supported" if a.additional_data else "unsupported",
                "mdxDialServerType": "MDX_DIAL_SERVER_TYPE_UNKNOWN"}
        return {"deviceInfo": json.dumps(info, separators=(",", ":")),
                "obfuscatedGaiaId": "", "cst": "m"}

    def dial_additional_data(self):
        """Full additionalData set served in the --additional-data DIAL shape."""
        return {"brand": "TCL", "model": "32Q3K", "screenId": self.screen_id, "theme": "cl",
                "deviceId": self.device_id, "loungeToken": self.token,
                "loungeTokenRefreshIntervalMs": 1500000}

    def open_session(self):
        a = self.args
        self.session = lounge.BindSession(
            "screen", self.device_id, a.name, self.token, a.app, theme="cl",
            capabilities=a.capabilities, log=self.log,
            extra=self.device_info_params())
        msgs = self.session.open()
        self.session_gen += 1
        say(f"bind channel open: SID {self.session.sid}, gsessionid {self.session.gsessionid[:8]}...")
        for m in msgs:
            self.inbox.put((self.session_gen, m))

    def reader(self):
        """Back-channel thread: keep a long poll open, reconnecting as needed."""
        backoff = 1
        while not self.stopping.is_set():
            gen, sess = self.session_gen, self.session
            try:
                sess.poll(lambda ms: [self.inbox.put((gen, m)) for m in ms],
                          timeout=self.args.poll_timeout, should_stop=self.stopping.is_set)
                self.log("poll-ended")
                backoff = 1
            except lounge.SessionGone as e:
                say(f"session ended by server ({e}); reopening")
                self.inbox.put((gen, ("session-gone", e.status)))
                while self.session_gen == gen and not self.stopping.is_set():
                    time.sleep(0.2)
            except Exception as e:  # network trouble: retry with backoff
                if self.stopping.is_set():
                    break
                say(f"poll error {type(e).__name__}: {e}; retry in {backoff}s")
                self.log("poll-exception", error=repr(e))
                time.sleep(backoff)
                backoff = min(backoff * 2, 60)

    def send(self, *maps):
        maps = [m for m in maps if m]
        if not maps:
            return
        try:
            self.session.send(list(maps))
            say("  -> sent", ", ".join(n for n, _ in maps))
        except Exception as e:
            say(f"  !! send failed: {e}")

    # -- pairing code ---------------------------------------------------------
    def show_code(self):
        a = self.args
        self.code = lounge.get_pairing_code(self.token, self.screen_id, a.name, a.app, self.device_id)
        self.code_at = time.monotonic()
        self.log("get_pairing_code", code=self.code)
        print()
        print("=" * 60)
        print(f"  Link with TV code:   {lounge.format_code(self.code)}")
        print(f"  Screen name:         {a.name}")
        print("  Phone: YouTube > You > Settings > Watch on TV > Enter TV code")
        print("=" * 60, flush=True)
        if a.code_file:
            pathlib.Path(a.code_file).write_text(self.code + "\n")

    # -- command handling -----------------------------------------------------
    def handle(self, m):
        p, pl = m.payload if isinstance(m.payload, dict) else {}, self.player
        name = m.name
        if name == "noop":
            return
        say(f"<- {name}", json.dumps(p, ensure_ascii=False) if p else "")
        self.log("command", aid=m.aid, name=name, args=m.args)

        if name == "loungeStatus":
            try:
                devices = json.loads(p.get("devices", "[]"))
            except ValueError:
                devices = []
            remotes = {d.get("id"): d.get("name") for d in devices if d.get("type") == "REMOTE_CONTROL"}
            for rid, rn in remotes.items():
                if rid not in self.remotes:
                    say(f"   remote present: {rn}")
            self.remotes = remotes
            if remotes:
                self.send(pl.prev_next(), pl.now_playing(),
                          ("onAutoplayModeChanged", {"autoplayMode": "UNSUPPORTED"}))
        elif name == "remoteConnected":
            self.remotes[p.get("id")] = p.get("name")
            say(f"   REMOTE CONNECTED: {p.get('name')} (app {p.get('app')}, theme {p.get('theme')})")
            self.send(pl.prev_next(), pl.now_playing(), pl.state_change() if pl.video_id else None,
                      pl.volume_changed(), ("onAutoplayModeChanged", {"autoplayMode": "UNSUPPORTED"}))
        elif name == "remoteDisconnected":
            self.remotes.pop(p.get("id"), None)
            say(f"   remote disconnected: {p.get('name')}")
        elif name == "getNowPlaying":
            self.send(pl.now_playing())
        elif name in ("setPlaylist", "setVideo"):
            vid = p.get("videoId")
            ids = [v for v in (p.get("videoIds") or vid or "").split(",") if v]
            pl.list_id, pl.video_ids = p.get("listId"), ids
            pl.ctt, pl.params = p.get("ctt"), p.get("params")
            try:
                pl.index = int(p.get("currentIndex", ids.index(vid) if vid in ids else 0))
            except ValueError:
                pl.index = 0
            start = float(p.get("currentTime") or 0)
            say(f"   >>> PLAY VIDEO {vid} from {start:.0f}s   https://youtu.be/{vid}"
                + (f"  (queue of {len(ids)}, list {pl.list_id})" if len(ids) > 1 else ""))
            pl.load(vid, start)
            self.send(pl.now_playing(), pl.state_change(), pl.prev_next())
        elif name == "updatePlaylist":
            ids = [v for v in (p.get("videoIds") or "").split(",") if v]
            pl.list_id, pl.video_ids = p.get("listId") or pl.list_id, ids
            if pl.video_id in ids:
                pl.index = ids.index(pl.video_id)
            say(f"   queue now {len(ids)} videos")
            self.send(pl.now_playing(), pl.prev_next())
        elif name == "play":
            pl.play()
            say(f"   >>> RESUME at {pl.position():.1f}s")
            self.send(pl.state_change())
        elif name == "pause":
            pl.pause()
            say(f"   >>> PAUSE at {pl.position():.1f}s")
            self.send(pl.state_change())
        elif name == "seekTo":
            t = float(p.get("newTime", 0))
            prev = pl.state
            pl.set_position(t)
            if prev == PLAYING:
                pl.play()
            say(f"   >>> SEEK to {t:.1f}s")
            self.send(pl.state_change())
        elif name == "stopVideo":
            pl.stop()
            say("   >>> STOP")
            self.send(pl.state_change(), pl.now_playing())
        elif name in ("next", "previous"):
            step = 1 if name == "next" else -1
            if pl.video_ids and 0 <= pl.index + step < len(pl.video_ids):
                pl.index += step
                vid = pl.video_ids[pl.index]
                say(f"   >>> PLAY VIDEO {vid} (queue {pl.index + 1}/{len(pl.video_ids)})")
                pl.load(vid, 0)
                self.send(pl.now_playing(), pl.state_change(), pl.prev_next())
        elif name == "setVolume":
            try:
                pl.volume = int(float(p.get("volume", pl.volume)))
            except ValueError:
                pass
            if "muted" in p:
                pl.muted = str(p["muted"]).lower() == "true"
            say(f"   >>> VOLUME {pl.volume}{' (muted)' if pl.muted else ''}")
            self.send(pl.volume_changed())
        elif name == "getVolume":
            self.send(pl.volume_changed())
        elif name == "setAutoplayMode":
            self.send(("onAutoplayModeChanged", {"autoplayMode": "UNSUPPORTED"}))
        elif name == "getSubtitlesTrack":
            self.send(("onSubtitlesTrackChanged", {"videoId": pl.video_id}))
        elif name == "getDiscoveryDeviceId":
            # Field names from SmartTube's CommandParams (defined there, unused).
            # Our DIAL UDN is the same UUID, so a phone can match the two.
            self.send(("setDiscoveryDeviceId", {"discoveryDeviceId": self.device_id,
                                                "loungeDeviceId": self.device_id}))
        elif name == "loungeScreenDisconnected":
            pass
        # Anything else (dpadCommand, voiceCommand, setSubtitlesTrack,
        # onUserActivity, ...) is printed above and otherwise ignored.

    def tick(self):
        pl = self.player
        if pl.pending_play_at and time.monotonic() >= pl.pending_play_at:
            pl.play()
            say(f"   (simulated load done: playing {pl.video_id})")
            self.send(pl.state_change(), pl.now_playing())
        if pl.state == PLAYING and pl.duration and pl.position() >= pl.duration:
            pl.base, pl.state, pl.since = pl.duration, ENDED, None
            say(f"   (simulated end of {pl.video_id})")
            self.send(pl.state_change())
        if (not self.remotes and not self.args.no_code and self.args.code_minutes > 0
                and time.monotonic() - self.code_at > self.args.code_minutes * 60):
            try:
                self.show_code()
            except Exception as e:
                say(f"pairing code refresh failed: {e}")

    # -- DIAL hook ------------------------------------------------------------
    def dial_launch(self, fields):
        """Called by the DIAL server (its own thread) on POST /apps/YouTube."""
        code = fields.get("pairingCode")
        if not code:
            say("   DIAL launch without pairingCode; nothing to register")
            return
        a = self.args
        st, reason, body = lounge.register_pairing_code(code, self.screen_id, a.name, a.app, self.device_id)
        self.log("register_pairing_code", code=code, status=st, reason=reason, body=body[:200])
        say(f"   register_pairing_code -> HTTP {st} {reason} {body[:80]!r}")

    def run(self, dial_server=None):
        self.refresh_token()
        self.open_session()
        if dial_server:
            dial_server.start()
        threading.Thread(target=self.reader, daemon=True).start()
        if not self.args.no_code:
            self.show_code()
        deadline = time.monotonic() + self.args.seconds if self.args.seconds else None
        while not self.stopping.is_set():
            if deadline and time.monotonic() > deadline:
                say(f"--seconds {self.args.seconds} elapsed; exiting")
                break
            try:
                gen, item = self.inbox.get(timeout=0.25)
            except queue.Empty:
                self.tick()
                continue
            if isinstance(item, tuple) and item[0] == "session-gone":
                if gen != self.session_gen:
                    continue
                if item[1] == 401:
                    self.refresh_token()
                time.sleep(1)
                self.open_session()
                continue
            if gen != self.session_gen:
                continue
            self.handle(item)
            self.tick()

    def shutdown(self):
        self.stopping.set()
        if self.session and self.session.sid:
            try:
                self.session.send([("loungeScreenDisconnected", {})])
            except Exception:
                pass
            self.session.terminate()


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--name", default="Dande TV (prototype)", help="screen name shown on the phone")
    ap.add_argument("--app", default="lb-v4",
                    help="screen app id sent to YouTube (SmartTube uses lb-v4, yt-cast-receiver ytcr)")
    ap.add_argument("--capabilities", default="dsp,mic,dpa")
    ap.add_argument("--device-info", action="store_true", help="also send a deviceInfo JSON on bind")
    ap.add_argument("--state", default=str(ASSETS / "screen-state.json"),
                    help="persistent screen ID / device ID file")
    ap.add_argument("--log-dir", default=str(ASSETS / "runs"))
    ap.add_argument("--duration", type=float, default=600, help="simulated video length (s)")
    ap.add_argument("--code-minutes", type=float, default=5,
                    help="refresh the code this often while no phone is connected (0: never)")
    ap.add_argument("--no-code", action="store_true", help="do not request a pairing code")
    ap.add_argument("--code-file", help="also write the current code to this file")
    ap.add_argument("--poll-timeout", type=float, default=90)
    ap.add_argument("--seconds", type=float, default=0, help="exit after this long (0: run until Ctrl-C)")
    ap.add_argument("--dial", action="store_true", help="also run the DIAL/SSDP server")
    ap.add_argument("--ip", help="DIAL: LAN address to serve on (default: the default-route address)")
    ap.add_argument("--port", type=int, default=56790, help="DIAL: HTTP port (8090 is taken by the TV proxy on this Mac)")
    ap.add_argument("--additional-data", action="store_true",
                    help="DIAL: alternative shape (kodi-yt-caster 2.0.14): always running, full "
                         "additionalData incl. loungeToken, deviceInfo on bind")
    args = ap.parse_args()

    run_dir = pathlib.Path(args.log_dir) / time.strftime("%Y%m%dT%H%M%SZ", time.gmtime())
    log = Transcript(run_dir / "transcript.jsonl")
    say(f"transcript: {run_dir / 'transcript.jsonl'}")
    screen = Screen(args, log)
    log("start", argv=sys.argv, screen_id=screen.screen_id, device_id=screen.device_id, name=args.name, app=args.app)

    dial_server = None
    if args.dial:
        import dial
        dial_server = dial.DialServer(
            ip=args.ip, port=args.port, friendly_name=args.name, uuid_seed=screen.device_id,
            on_launch=screen.dial_launch, log=log, say=say,
            additional_data=screen.dial_additional_data if args.additional_data else None)

    def on_signal(*_):
        raise KeyboardInterrupt
    signal.signal(signal.SIGTERM, on_signal)
    try:
        screen.run(dial_server)
    except KeyboardInterrupt:
        print()
        say("stopping")
    finally:
        if dial_server:
            dial_server.stop()
        screen.shutdown()
        log("stop")
        say("stopped")


if __name__ == "__main__":
    main()
