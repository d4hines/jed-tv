"""Minimal DIAL server (SSDP responder + HTTP) for the YouTube app.

Modelled on SmartTube's DialServer (commit e7c6626, 2026-10-28), which its
author tested with the stock YouTube Android app 21.40 on 2026-10-12, plus the
DIAL 2.x conventions used by peer-dial (yt-cast-receiver) and plaincast.

Used by `screen.py --dial`. It binds only while that process runs:
  * UDP 1900 (SO_REUSEADDR/SO_REUSEPORT), joined to 239.255.255.250 on the one
    LAN interface chosen (never the 192.168.77.0/24 TV link);
  * TCP <port> on that interface's address.
With --ip 127.0.0.1 it stays on loopback (no multicast join) for self-tests.

Every request it sees (SSDP and HTTP, with all headers and the launch body)
goes to the transcript, so a run with the owner's phone records exactly what
the YouTube app sends.
"""

import http.server
import ipaddress
import random
import socket
import struct
import threading
import time
import urllib.parse
from xml.sax.saxutils import escape

SSDP_ADDR, SSDP_PORT = "239.255.255.250", 1900
DIAL_ST = "urn:dial-multiscreen-org:service:dial:1"
ANSWER_ST = {DIAL_ST, "ssdp:all", "urn:dial-multiscreen-org:device:dial:1"}
APP = "YouTube"
TV_LINK = ipaddress.ip_network("192.168.77.0/24")


def default_route_ip():
    """Source address the OS would use for the internet (no packet is sent)."""
    s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    try:
        s.connect(("192.0.2.1", 9))
        return s.getsockname()[0]
    finally:
        s.close()


def parse_headers(text):
    lines = text.split("\r\n")
    out = {}
    for line in lines[1:]:
        if ":" in line:
            k, v = line.split(":", 1)
            out[k.strip().upper()] = v.strip()
    return lines[0], out


class DialServer:
    def __init__(self, ip, port, friendly_name, uuid_seed, on_launch, log, say,
                 additional_data=None):
        self.ip = ip or default_route_ip()
        addr = ipaddress.ip_address(self.ip)
        if addr in TV_LINK:
            raise SystemExit(f"refusing to serve DIAL on the TV link address {self.ip}; pass --ip")
        self.loopback = addr.is_loopback
        self.port, self.name, self.udn = port, friendly_name, uuid_seed
        self.on_launch, self.log, self.say = on_launch, log, say
        self.additional_data = additional_data  # callable -> dict, or None
        self.running_app = False
        self.stop_flag = threading.Event()
        self.ssdp = None
        self.httpd = None

    # -- descriptions ---------------------------------------------------------
    @property
    def base(self):
        return f"http://{self.ip}:{self.port}"

    def device_xml(self):
        return (
            '<?xml version="1.0"?>\n'
            '<root xmlns="urn:schemas-upnp-org:device-1-0">\n'
            '  <specVersion><major>1</major><minor>0</minor></specVersion>\n'
            '  <device>\n'
            '    <deviceType>urn:dial-multiscreen-org:device:dial:1</deviceType>\n'
            f'    <friendlyName>{escape(self.name)}</friendlyName>\n'
            '    <manufacturer>TCL</manufacturer>\n'
            '    <modelName>32Q3K</modelName>\n'
            f'    <UDN>uuid:{self.udn}</UDN>\n'
            '    <serviceList>\n'
            '      <service>\n'
            f'        <serviceType>{DIAL_ST}</serviceType>\n'
            '        <serviceId>urn:dial-multiscreen-org:serviceId:dial</serviceId>\n'
            '        <controlURL>/ssdp/notfound</controlURL>\n'
            '        <eventSubURL>/ssdp/notfound</eventSubURL>\n'
            '        <SCPDURL>/ssdp/notfound</SCPDURL>\n'
            '      </service>\n'
            '    </serviceList>\n'
            '  </device>\n'
            '</root>\n')

    def app_xml(self):
        if not self.additional_data:
            # SmartTube's shape (stock YouTube 21.40 cast to it, 2026-10-12):
            # no additionalData, so the phone POSTs a pairingCode to launch.
            state = "running" if self.running_app else "stopped"
            return (
                '<?xml version="1.0" encoding="UTF-8"?>\n'
                '<service xmlns="urn:dial-multiscreen-org:schemas:dial" dialVer="1.7">\n'
                f'  <name>{APP}</name>\n'
                '  <options allowStop="false"/>\n'
                f'  <state>{state}</state>\n'
                '</service>\n')
        # kodi-yt-caster 2.0.14's shape (verified by its author 2026-10-29):
        # always running, with the full additionalData set. A partial set
        # (screenId only) made the phone poll forever and then drop the
        # session with receiverIdentityMatchStatus=DOES_NOT_MATCH_RECEIVER.
        fields = self.additional_data()
        body = "".join(f"    <{k}>{escape(str(v))}</{k}>\n" for k, v in fields.items() if v)
        return (
            '<?xml version="1.0" encoding="UTF-8"?>\n'
            '<service xmlns="urn:dial-multiscreen-org:schemas:dial" dialVer="2.1">\n'
            f'  <name>{APP}</name>\n'
            '  <options allowStop="true"/>\n'
            '  <state>running</state>\n'
            '  <link rel="run" href="run"/>\n'
            f'  <additionalData>\n{body}  </additionalData>\n'
            '</service>\n')

    # -- SSDP -----------------------------------------------------------------
    def ssdp_reply(self, st):
        return (
            "HTTP/1.1 200 OK\r\n"
            "CACHE-CONTROL: max-age=1800\r\n"
            "EXT:\r\n"
            f"LOCATION: {self.base}/dd.xml\r\n"
            "SERVER: Linux/4.19 UPnP/1.1 dande-tv/0\r\n"
            f"ST: {st}\r\n"
            f"USN: uuid:{self.udn}::{st}\r\n"
            "BOOTID.UPNP.ORG: 1\r\n"
            "CONFIGID.UPNP.ORG: 1\r\n"
            "\r\n").encode()

    def notify(self, nts):
        if self.loopback:
            return
        msg = (
            "NOTIFY * HTTP/1.1\r\n"
            f"HOST: {SSDP_ADDR}:{SSDP_PORT}\r\n"
            "CACHE-CONTROL: max-age=1800\r\n"
            f"LOCATION: {self.base}/dd.xml\r\n"
            f"NT: {DIAL_ST}\r\n"
            f"NTS: {nts}\r\n"
            "SERVER: Linux/4.19 UPnP/1.1 dande-tv/0\r\n"
            f"USN: uuid:{self.udn}::{DIAL_ST}\r\n"
            "BOOTID.UPNP.ORG: 1\r\n"
            "CONFIGID.UPNP.ORG: 1\r\n"
            "\r\n").encode()
        try:
            self.ssdp.sendto(msg, (SSDP_ADDR, SSDP_PORT))
        except OSError as e:
            self.say(f"   SSDP NOTIFY failed: {e}")

    def ssdp_loop(self):
        while not self.stop_flag.is_set():
            try:
                data, src = self.ssdp.recvfrom(4096)
            except socket.timeout:
                continue
            except OSError:
                break
            text = data.decode("utf-8", "replace")
            if not text.startswith("M-SEARCH"):
                continue
            src_ip = ipaddress.ip_address(src[0])
            _, hdr = parse_headers(text)
            st = hdr.get("ST", "")
            answer = st in ANSWER_ST and src_ip not in TV_LINK and (self.loopback == src_ip.is_loopback)
            self.log("ssdp-msearch", src=f"{src[0]}:{src[1]}", headers=hdr, answered=answer)
            if st in ANSWER_ST or "dial" in st.lower():
                self.say(f"   SSDP M-SEARCH from {src[0]}:{src[1]} ST={st} UA={hdr.get('USER-AGENT', '-')}"
                         + ("" if answer else " (ignored)"))
            if not answer:
                continue
            try:
                mx = max(0, min(int(hdr.get("MX", "1")), 5))
            except ValueError:
                mx = 1
            time.sleep(random.uniform(0, min(mx, 1) * 0.5))
            self.ssdp.sendto(self.ssdp_reply(st if st != "ssdp:all" else DIAL_ST), src)

    def open_ssdp(self):
        s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM, socket.IPPROTO_UDP)
        s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        if hasattr(socket, "SO_REUSEPORT"):
            s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEPORT, 1)
        if self.loopback:
            s.bind((self.ip, SSDP_PORT))  # unicast M-SEARCH only; nothing leaves the host
        else:
            s.bind(("", SSDP_PORT))
            mreq = struct.pack("4s4s", socket.inet_aton(SSDP_ADDR), socket.inet_aton(self.ip))
            s.setsockopt(socket.IPPROTO_IP, socket.IP_ADD_MEMBERSHIP, mreq)
            s.setsockopt(socket.IPPROTO_IP, socket.IP_MULTICAST_IF, socket.inet_aton(self.ip))
            s.setsockopt(socket.IPPROTO_IP, socket.IP_MULTICAST_TTL, 2)
        s.settimeout(0.5)
        return s

    # -- HTTP -----------------------------------------------------------------
    def make_handler(self):
        server = self

        class Handler(http.server.BaseHTTPRequestHandler):
            protocol_version = "HTTP/1.1"
            server_version = "dande-tv-dial/0"

            def log_message(self, *a):
                pass

            def _record(self, body=b""):
                server.log("dial-http", src=self.client_address[0], method=self.command,
                           path=self.path, headers=dict(self.headers.items()),
                           body=body.decode("utf-8", "replace"))
                server.say(f"   DIAL {self.command} {self.path} from {self.client_address[0]}"
                           f" Origin={self.headers.get('Origin', '-')}"
                           f" UA={self.headers.get('User-Agent', '-')}")

            def _send(self, code, body=b"", ctype=None, headers=()):
                self.send_response(code)
                for k, v in headers:
                    self.send_header(k, v)
                if ctype:
                    self.send_header("Content-Type", ctype)
                self.send_header("Content-Length", str(len(body)))
                self.end_headers()
                if body and self.command != "HEAD":
                    self.wfile.write(body)

            def _path(self):
                return urllib.parse.urlsplit(self.path).path.rstrip("/")

            def do_GET(self):
                self._record()
                p = self._path()
                if p == "/dd.xml":
                    self._send(200, server.device_xml().encode(), "text/xml; charset=utf-8",
                               [("Application-URL", f"{server.base}/apps/")])
                elif p == f"/apps/{APP}":
                    self._send(200, server.app_xml().encode(), "text/xml; charset=utf-8")
                else:
                    self._send(404)

            def do_POST(self):
                n = int(self.headers.get("Content-Length") or 0)
                body = self.rfile.read(min(n, 65536)) if n else b""
                self._record(body)
                p = self._path()
                if p == f"/apps/{APP}":
                    fields = dict(urllib.parse.parse_qsl(body.decode("utf-8", "replace")))
                    server.say(f"   DIAL LAUNCH body: {body.decode('utf-8', 'replace')}")
                    was_running = server.running_app
                    server.running_app = True
                    self._send(200 if was_running else 201,
                               headers=[("Location", f"{server.base}/apps/{APP}/run")])
                    threading.Thread(target=server.on_launch, args=(fields,), daemon=True).start()
                elif p == f"/apps/{APP}/dial_data":
                    self._send(200)
                else:
                    self._send(404)

            def do_DELETE(self):
                self._record()
                if self._path() == f"/apps/{APP}/run":
                    server.running_app = False
                    self._send(200)
                else:
                    self._send(404)

            def do_OPTIONS(self):
                self._record()
                self._send(204, headers=[("Access-Control-Allow-Methods", "GET, POST, DELETE, OPTIONS")])

        return Handler

    # -- lifecycle ------------------------------------------------------------
    def start(self):
        self.httpd = http.server.ThreadingHTTPServer((self.ip, self.port), self.make_handler())
        self.httpd.daemon_threads = True
        threading.Thread(target=self.httpd.serve_forever, daemon=True).start()
        self.ssdp = self.open_ssdp()
        threading.Thread(target=self.ssdp_loop, daemon=True).start()
        self.log("dial-start", ip=self.ip, port=self.port, udn=self.udn, loopback=self.loopback)
        self.say(f"DIAL: SSDP on udp/{SSDP_PORT}"
                 + (" (loopback only)" if self.loopback else f", multicast joined on {self.ip}")
                 + f"; device description {self.base}/dd.xml")
        for _ in range(2):
            self.notify("ssdp:alive")

    def stop(self):
        self.stop_flag.set()
        if self.ssdp:
            self.notify("ssdp:byebye")
            self.ssdp.close()
        if self.httpd:
            self.httpd.shutdown()
            self.httpd.server_close()
        self.log("dial-stop")
        self.say("DIAL server stopped (sockets closed)")
