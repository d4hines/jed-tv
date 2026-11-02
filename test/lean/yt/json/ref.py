#!/usr/bin/env python3
"""Reference results for the Lean JSON test, from Python's json module.

For every fixture: (1) the canonical compact re-serialization of the whole
document (numbers kept as their source text via parse_int/parse_float,
lone surrogates as U+FFFD), and (2) the fields the app extracts, with the
rules of src/ytplay/src/yt.rs (serde_json as_u64, str::parse::<u64>,
max_by_key keeping the last maximum). Also the accept/reject verdicts and
canonical forms of hand-written edge cases, and the request bodies
(json.dumps with sorted keys, as serde_json's json! macro produces).

  python3 -I test/lean/yt/json/ref.py OUT_DIR FIXTURE.json...
"""
import base64
import json
import pathlib
import re
import sys


class Raw(str):
    pass


class Pairs(list):
    """An object with its members in order, duplicates kept."""


def loads(b, pairs=False):
    """Python's json.loads on the bytes, except that UTF-8 is decoded
    strictly: json.loads(bytes) uses 'surrogatepass', accepting UTF-8-encoded
    surrogates (ED A0 80), which are invalid UTF-8 and which the Lean parser
    rejects. A leading BOM is skipped, as json.loads(bytes) does."""
    def bad(c):
        raise ValueError("constant " + c)
    if b.startswith(b"\xef\xbb\xbf"):
        b = b[3:]
    return json.loads(b.decode("utf-8"), parse_int=Raw, parse_float=Raw, parse_constant=bad,
                      object_pairs_hook=Pairs if pairs else None)


def fix(s):
    # Lone surrogates -> U+FFFD (UTF-8 cannot carry them).
    return re.sub("[\ud800-\udfff]", "�", s)


def jstr(s):
    return json.dumps(fix(s), ensure_ascii=False)


def canon(v):
    if v is None:
        return "null"
    if v is True:
        return "true"
    if v is False:
        return "false"
    if isinstance(v, Raw):
        return str(v)
    if isinstance(v, str):
        return jstr(v)
    if isinstance(v, Pairs):
        return "{" + ",".join(jstr(k) + ":" + canon(x) for k, x in v) + "}"
    return "[" + ",".join(canon(x) for x in v) + "]"


def as_u64(v):
    if isinstance(v, Raw) and re.fullmatch(r"0|[1-9][0-9]*", v) and int(v) < 2**64:
        return int(v)
    return None


def parse_u64(s):  # Rust str::parse::<u64>
    if not isinstance(s, str) or isinstance(s, Raw):
        return None
    t = s[1:] if s.startswith("+") else s
    if not t or not all("0" <= c <= "9" for c in t) or int(t) >= 2**64:
        return None
    return int(t)


def get(v, k):
    return v.get(k) if isinstance(v, dict) else None


def as_str(v):
    return v if isinstance(v, str) and not isinstance(v, Raw) else None


def player(d):
    out = []
    ps = get(d, "playabilityStatus")
    out.append("status " + jstr(as_str(get(ps, "status")) or ""))
    out.append("reason " + jstr(as_str(get(ps, "reason")) or ""))
    vd = as_str(get(get(d, "responseContext"), "visitorData"))
    out.append("visitor " + (jstr(vd) if vd is not None else "none"))
    det = get(d, "videoDetails")
    out.append("title " + jstr(as_str(get(det, "title")) or ""))
    out.append("length %d" % (parse_u64(get(det, "lengthSeconds")) or 0))
    fmts = []
    af = get(get(d, "streamingData"), "adaptiveFormats")
    for f in af if isinstance(af, list) else []:
        itag = as_u64(get(f, "itag"))
        url = as_str(get(f, "url"))
        if itag is None or url is None:
            continue
        u = lambda k: (as_u64(get(f, k)) or 0) % 2**32
        rng = lambda r: ((parse_u64(get(r, "start")) or 0), (parse_u64(get(r, "end")) or 0))
        fmts.append(dict(itag=itag % 2**32, url=url, mime=as_str(get(f, "mimeType")) or "",
                         w=u("width"), h=u("height"), fps=u("fps"), bitrate=as_u64(get(f, "bitrate")) or 0,
                         clen=parse_u64(get(f, "contentLength")) or 0,
                         init=rng(get(f, "initRange")), index=rng(get(f, "indexRange"))))
    for f in fmts:
        out.append("format itag%d mime%s w%d h%d fps%d br%d clen%d init%d-%d index%d-%d url%s" % (
            f["itag"], jstr(f["mime"]), f["w"], f["h"], f["fps"], f["bitrate"], f["clen"],
            f["init"][0], f["init"][1], f["index"][0], f["index"][1], jstr(f["url"])))

    def best(cands, key):
        b = None
        for c in cands:
            if b is None or key(c) >= key(b):
                b = c
        return "none" if b is None else "%d" % b["itag"]
    for mh in (720, 480, 144, 100):
        v = [f for f in fmts if f["mime"].startswith("video/mp4") and "avc1" in f["mime"]
             and 0 < f["w"] <= 1280 and 0 < f["h"] <= mh and 0 < f["fps"] <= 30]
        out.append("pick-video %d %s" % (mh, best(v, lambda f: (f["h"], f["bitrate"]))))
    a = [f for f in fmts if f["mime"].startswith("audio/mp4") and "mp4a" in f["mime"]]
    out.append("pick-audio " + best(a, lambda f: f["bitrate"]))
    return out


def text_of(v):
    s = as_str(get(v, "simpleText"))
    if s is not None:
        return s
    r = get(v, "runs")
    if isinstance(r, list):
        return "".join(as_str(get(x, "text")) or "" for x in r)
    return ""


def collect(v, out, order):
    if isinstance(v, dict):
        r = v.get("videoRenderer")
        if r is not None:
            vid = as_str(get(r, "videoId"))
            if vid is not None:
                out.append((vid, text_of(get(r, "title")), text_of(get(r, "ownerText")),
                            text_of(get(r, "lengthText"))))
        for k in order(v):
            collect(v[k], out, order)
    elif isinstance(v, list):
        for x in v:
            collect(x, out, order)
    return out


def search(d, order=lambda m: list(m.keys())):
    items, seen = [], set()
    for i in collect(d, [], order):
        if i[3] and i[0] not in seen:
            seen.add(i[0])
            items.append(i)
    return items


CASES = [
    b'{}', b'[]', b' \t\r\n{ "a" : [ 1 , 2 ] } \n', b'"abc"', b'5', b'-0', b'true', b'false', b'null',
    b'{"a":1,"a":2}', b'[1,2,]', b'{"a":1,}', b'[1 2]', b'{"a" 1}', b'{1:2}', b'', b'   ', b'{} x',
    b'[', b']', b'{"a":', b'"\\"\\\\\\/\\b\\f\\n\\r\\t"', b'"\\u0000\\u001f\\u0041\\u00e9\\u20ac\\uFFFF"',
    b'"\\ud83d\\ude00"', b'"\\uD83D\\uDE00x"', b'"\\ud800"', b'"\\udc00"', b'"\\ud800\\u0041"',
    b'"\\ud800\\ud800\\udc00"', b'"\\ud800x"', b'"\\x"', b'"\\u12"', b'"\\u12g4"', b'"\\U0041"',
    b'"a\tb"', b'"a\nb"', b'"\x7f"', b'"unterminated', b'"\xc3\xa9\xe2\x82\xac\xf0\x9f\x98\x80"',
    b'"\xff"', b'"\xc3"', b'\xff', b'"\xed\xa0\x80"', b'\xef\xbb\xbf{"bom":1}',
    b'0', b'01', b'-', b'-01', b'1.', b'.5', b'1.5', b'1e', b'1e+', b'1E-5', b'-1.2e+3', b'1.0',
    b'123456789012345678901234567890', b'18446744073709551615', b'18446744073709551616', b'+1',
    b'NaN', b'Infinity', b'-Infinity', b'tru', b'nul', b'truex', b'[true,false,null]',
    b'{"k":{"n":{"d":[[],[{}],{"e":""}]}}}', b'[' * 900 + b']' * 900, b'{"a":"\\u0026\\u003c\\u003e"}',
    b'["\\"quoted\\" and \\\\ backslash"]', b'{"":""}', b'{"a":[1.5e300,-1e-300,0.0]}',
]


def main(out, *fixtures):
    sys.setrecursionlimit(100000)
    out = pathlib.Path(out)
    out.mkdir(parents=True, exist_ok=True)
    for f in fixtures:
        name = pathlib.Path(f).stem
        b = pathlib.Path(f).read_bytes()
        d = loads(b)
        (out / (name + ".canon")).write_text(canon(loads(b, pairs=True)) + "\n", encoding="utf-8")
        lines = player(d)
        doc_order = search(d)
        sorted_order = search(d, order=lambda m: sorted(m.keys()))
        lines.append("search %d (rust sorted-key walk: %s)" % (
            len(doc_order), "same order" if doc_order == sorted_order else "different order"))
        for i in doc_order:
            lines.append("item %s title%s channel%s length%s" % tuple(jstr(x) for x in i))
        (out / (name + ".expect")).write_text("\n".join(lines) + "\n", encoding="utf-8")
    cases, verdicts = [], []
    for c in CASES:
        cases.append(base64.b64encode(c).decode())
        try:
            verdicts.append("ok " + canon(loads(c, pairs=True)))
        except Exception:  # noqa: BLE001 - any rejection
            verdicts.append("err")
    (out / "cases.b64").write_text("\n".join(cases) + "\n")
    (out / "cases.expect").write_text("\n".join(verdicts) + "\n", encoding="utf-8")
    ua = ("Mozilla/5.0 (Macintosh; Intel Mac OS X 15_7_3) AppleWebKit/605.1.15 "
          "(KHTML, like Gecko) Version/26.0 Safari/605.1.15")
    client = {"clientName": "VISIONOS", "clientVersion": "1.02", "deviceMake": "Apple",
              "deviceModel": "RealityDevice17,1", "osName": "visionOS", "osVersion": "26.5.23O471",
              "hl": "en", "userAgent": ua}
    dump = lambda o: json.dumps(o, sort_keys=True, separators=(",", ":"), ensure_ascii=False)
    bodies = []
    for vid, vis in [("PVGeM40dABA", None), ("PVGeM40dABA", "CgtTWVg5bUIwa0VZWSjK7pvXBjIKCgJVUxIEGgAgJg%3D%3D")]:
        c = dict(client)
        if vis:
            c["visitorData"] = vis
        bodies.append(dump({"context": {"client": c}, "videoId": vid, "contentCheckOk": True,
                            "racyCheckOk": True}))
    for q in ["big buck bunny", 'say "hi" \\ back\tslash\n', "café 日本 \U0001f600", "\x01\x1f\x7f"]:
        bodies.append(dump({"context": {"client": {"clientName": "WEB", "clientVersion": "2.20260708.00.00",
                                                   "hl": "en", "gl": "US"}}, "query": q}))
    (out / "bodies.expect").write_text("\n".join(base64.b64encode(x.encode()).decode() for x in bodies) + "\n")


if __name__ == "__main__":
    main(*sys.argv[1:])
