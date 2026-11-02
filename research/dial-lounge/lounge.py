"""YouTube Lounge API, screen (TV) side, plus the remote side for self-tests.

Standard library only. Endpoints and parameters follow the screen
implementations in SmartTube's MediaServiceCore (lounge/, 2026-10-28) and
yt-cast-receiver 2.1.1 (2026-03-19); see README.md for the protocol notes.

Nothing here talks to the TV. It runs on the host against live YouTube.
"""

import codecs
import http.client
import json
import random
import ssl
import string
import time
import urllib.parse
import uuid

HOST = "www.youtube.com"
API = "/api/lounge"
_CTX = ssl.create_default_context()


# --------------------------------------------------------------------------
# HTTP


class HttpError(Exception):
    def __init__(self, status, reason, body):
        super().__init__(f"HTTP {status} {reason}: {body[:200]!r}")
        self.status, self.reason, self.body = status, reason, body


def _conn(timeout):
    return http.client.HTTPSConnection(HOST, context=_CTX, timeout=timeout)


def request(method, path, query=None, form=None, timeout=20):
    """One request on a fresh connection. Returns (status, reason, body str)."""
    if query:
        path = path + ("&" if "?" in path else "?") + urllib.parse.urlencode(query)
    body = urllib.parse.urlencode(form).encode() if form is not None else None
    headers = {"User-Agent": "dande-tv-lounge-prototype/0"}
    if body is not None:
        headers["Content-Type"] = "application/x-www-form-urlencoded"
    c = _conn(timeout)
    try:
        c.request(method, path, body=body, headers=headers)
        r = c.getresponse()
        data = r.read().decode("utf-8", "replace")
        return r.status, r.reason, data
    finally:
        c.close()


def stream(path, query, timeout):
    """Open a GET whose body arrives incrementally (http.client undoes chunking)."""
    path = path + "?" + urllib.parse.urlencode(query)
    c = _conn(timeout)
    c.request("GET", path, headers={"User-Agent": "dande-tv-lounge-prototype/0"})
    return c, c.getresponse()


def zx():
    return "".join(random.choice(string.ascii_lowercase + string.digits) for _ in range(12))


# --------------------------------------------------------------------------
# BrowserChannel framing: "<length>\n<JSON array>\n" repeated. The array holds
# [arrayId, [name, payload?]] entries. We find each array by bracket matching
# (string-aware) rather than trusting the length, and record which unit the
# length uses so the Lean port can rely on it.


class Message:
    __slots__ = ("aid", "name", "args")

    def __init__(self, aid, name, args):
        self.aid, self.name, self.args = aid, name, args

    @property
    def payload(self):
        return self.args[0] if self.args else None

    def __repr__(self):
        return f"[{self.aid}] {self.name} {json.dumps(self.args, ensure_ascii=False)}"


def _scan_array(buf, i):
    """Index just past the JSON array starting at buf[i] == '[', or None."""
    depth, in_str, esc = 0, False, False
    for j in range(i, len(buf)):
        ch = buf[j]
        if in_str:
            if esc:
                esc = False
            elif ch == "\\":
                esc = True
            elif ch == '"':
                in_str = False
        elif ch == '"':
            in_str = True
        elif ch == "[":
            depth += 1
        elif ch == "]":
            depth -= 1
            if depth == 0:
                return j + 1
    return None


class ChunkParser:
    def __init__(self, on_length_check=None):
        self.buf = ""
        self.dec = codecs.getincrementaldecoder("utf-8")()
        self.on_length_check = on_length_check

    def feed(self, data):
        self.buf += self.dec.decode(data)
        out = []
        while True:
            s = self.buf.lstrip()
            nl = s.find("\n")
            if nl < 0:
                self.buf = s
                break
            head = s[:nl].strip()
            if not head.isdigit():
                raise ValueError(f"bad chunk header {head[:40]!r}")
            start = nl + 1
            while start < len(s) and s[start] in " \r\n":
                start += 1
            if start >= len(s):
                self.buf = s
                break
            end = _scan_array(s, start)
            if end is None:
                self.buf = s
                break
            text = s[start:end]
            declared = int(head)
            if self.on_length_check:
                body = s[nl + 1:end] + "\n"  # length covers the trailing newline
                self.on_length_check(declared, {
                    "chars": len(body),
                    "utf16": len(body.encode("utf-16-le")) // 2,
                    "utf8": len(body.encode("utf-8")),
                })
            for entry in json.loads(text):
                aid, inner = entry[0], entry[1]
                out.append(Message(aid, inner[0], inner[1:]))
            self.buf = s[end:]
        return out


def parse_body(text):
    return ChunkParser().feed(text.encode())


# --------------------------------------------------------------------------
# Pairing endpoints


def generate_screen_id():
    st, reason, body = request("GET", API + "/pairing/generate_screen_id")
    if st != 200:
        raise HttpError(st, reason, body)
    return body.strip()


def get_lounge_token(screen_id):
    st, reason, body = request("POST", API + "/pairing/get_lounge_token_batch",
                               form={"screen_ids": screen_id})
    if st != 200:
        raise HttpError(st, reason, body)
    return json.loads(body)["screens"][0]


def get_pairing_code(lounge_token, screen_id, screen_name, app, device_id):
    """The 12-digit 'Link with TV code' for this screen (plain text reply)."""
    st, reason, body = request("POST", API + "/pairing/get_pairing_code?ctx=pair", form={
        "access_type": "permanent", "app": app, "lounge_token": lounge_token,
        "screen_id": screen_id, "screen_name": screen_name, "device_id": device_id})
    if st != 200:
        raise HttpError(st, reason, body)
    return body.strip()


def register_pairing_code(code, screen_id, screen_name, app, device_id):
    """Attach a sender-generated code (from a DIAL launch POST) to this screen."""
    st, reason, body = request("POST", API + "/pairing/register_pairing_code", form={
        "access_type": "permanent", "app": app, "pairing_code": code,
        "screen_id": screen_id, "screen_name": screen_name, "device_id": device_id})
    return st, reason, body


def get_screen(pairing_code):
    """Remote side: resolve a pairing code to {screenId, loungeToken, name, ...}."""
    st, reason, body = request("POST", API + "/pairing/get_screen",
                               form={"pairing_code": pairing_code})
    if st != 200:
        raise HttpError(st, reason, body)
    return json.loads(body)["screen"]


def format_code(code):
    return " ".join(code[i:i + 3] for i in range(0, len(code), 3))


# --------------------------------------------------------------------------
# Bind channel session (either role)


class SessionGone(Exception):
    """400 Unknown SID / 410 Gone: open a new session. 401: refresh the token."""

    def __init__(self, status, reason):
        super().__init__(f"{status} {reason}")
        self.status, self.reason = status, reason


class BindSession:
    """One BrowserChannel session on /api/lounge/bc/bind.

    role "screen": device=LOUNGE_SCREEN (what our TV is).
    role "remote": device=REMOTE_CONTROL (only used by fake_remote.py).
    """

    def __init__(self, role, device_id, name, lounge_token, app, theme="cl",
                 capabilities=None, extra=None, log=None):
        self.role, self.device_id, self.name = role, device_id, name
        self.lounge_token, self.app, self.theme = lounge_token, app, theme
        self.capabilities = capabilities
        self.extra = extra or {}
        self.log = log or (lambda *a, **k: None)
        self.sid = self.gsessionid = None
        self.rid = random.randint(10000, 89999)
        self.aid = -1  # highest array id received
        self.ofs = 0   # count of maps sent on this session

    def _common(self):
        q = {"device": "LOUNGE_SCREEN" if self.role == "screen" else "REMOTE_CONTROL",
             "id": self.device_id, "name": self.name, "app": self.app,
             "theme": self.theme, "loungeIdToken": self.lounge_token,
             "VER": "8", "v": "2", "zx": zx(), "t": "1"}
        if self.capabilities:
            q["capabilities"] = self.capabilities
        if self.role == "screen":
            q["mdxVersion"] = "2"
        q.update(self.extra)
        return q

    def open(self):
        """Initial POST (count=0). Returns the messages delivered with it."""
        q = self._common()
        q.update({"RID": str(self.rid), "CVER": "1"})
        if self.role == "remote":
            q["auth_failure_option"] = "send_error"
        self.rid += 1
        st, reason, body = request("POST", API + "/bc/bind", query=q, form={"count": "0"})
        self.log("bind-open", status=st, reason=reason, body=body)
        if st != 200:
            if st in (400, 401, 410):
                raise SessionGone(st, reason)
            raise HttpError(st, reason, body)
        msgs = parse_body(body)
        for m in msgs:
            self.aid = max(self.aid, m.aid)
            if m.name == "c":
                self.sid = m.args[0]
            elif m.name == "S":
                self.gsessionid = m.args[0]
        if not (self.sid and self.gsessionid):
            raise HttpError(st, "no SID/gsessionid in bind reply", body)
        return [m for m in msgs if m.name not in ("c", "S")]

    def send(self, maps):
        """Forward channel: maps is a list of (name, {key: value}). One POST."""
        if not maps:
            return
        form = {"count": str(len(maps)), "ofs": str(self.ofs)}
        for i, (name, fields) in enumerate(maps):
            form[f"req{i}__sc"] = name
            for k, v in fields.items():
                if v is None:
                    continue
                if isinstance(v, bool):
                    v = "true" if v else "false"
                form[f"req{i}_{k}"] = str(v)
        q = self._common()
        q.update({"SID": self.sid, "gsessionid": self.gsessionid,
                  "RID": str(self.rid), "AID": str(self.aid)})
        self.rid += 1
        self.ofs += len(maps)
        st, reason, body = request("POST", API + "/bc/bind", query=q, form=form)
        self.log("send", maps=maps, status=st, reason=reason, body=body[:300])
        if st != 200:
            if st in (400, 401, 410):
                raise SessionGone(st, reason)
            raise HttpError(st, reason, body)

    def poll(self, on_messages, timeout=90, should_stop=lambda: False):
        """Back channel: one long-poll GET, delivering messages as they arrive.

        Returns normally when the server ends the response (reconnect then).
        """
        q = self._common()
        q.update({"RID": "rpc", "SID": self.sid, "gsessionid": self.gsessionid,
                  "CI": "0", "AID": str(self.aid), "TYPE": "xmlhttp"})
        c, r = stream(API + "/bc/bind", q, timeout)
        try:
            if r.status != 200:
                body = r.read().decode("utf-8", "replace")
                self.log("poll-error", status=r.status, reason=r.reason, body=body[:300])
                if r.status in (400, 401, 410):
                    raise SessionGone(r.status, r.reason)
                raise HttpError(r.status, r.reason, body)
            parser = ChunkParser(lambda d, units: self.log("chunk-length", declared=d, **units))
            while not should_stop():
                data = r.read1(65536)
                if not data:
                    break
                self.log("raw", text=data.decode("utf-8", "replace"))
                msgs = parser.feed(data)
                for m in msgs:
                    self.aid = max(self.aid, m.aid)
                if msgs:
                    on_messages(msgs)
        finally:
            c.close()

    def terminate(self):
        """Tell the server this session is over (best effort)."""
        if not self.sid:
            return
        q = self._common()
        q.update({"SID": self.sid, "gsessionid": self.gsessionid, "RID": str(self.rid),
                  "AID": str(self.aid), "CVER": "1"})
        self.rid += 1
        try:
            st, reason, body = request("POST", API + "/bc/bind", query=q,
                                       form={"TYPE": "terminate", "ui": ""}, timeout=5)
            self.log("terminate", status=st, reason=reason)
        except OSError:
            pass


def new_device_id():
    return str(uuid.uuid4())


def now():
    return time.monotonic()
