"""C struct layouts, straight from clang (zig cc), for postprocess.py.

c2rust drops `_Alignas` on struct fields, so structs like H264SliceContext
come out with the wrong field offsets. We ask the same compiler that built
FFmpeg for the truth: `-fdump-record-layouts-complete` for every record's
size/align/top-level field offsets (used for compile-time asserts), and a
small probe object (symbol sizes = offsetof/sizeof/natural alignof) to compute
the padding to insert."""
import pathlib, re, subprocess, time

ZIG_TARGET = {"aarch64-unknown-linux-musl": ("aarch64-linux-musl", "cortex_a55"),
              "armv7-unknown-linux-musleabihf": ("arm-linux-musleabihf", "cortex_a55")}


def zcc(target):
    zt, cpu = ZIG_TARGET[target]
    return ["zig", "cc", "-target", zt, f"-mcpu={cpu}", "-std=c17", "-w"]


def dump_records(target, c_file, obj):
    """{name: {"kind", "size", "align", "fields": [(name|None, offset, is_bitfield, type)]}}.
    Anonymous records are keyed by their "(unnamed at FILE:L:C)" location.
    `obj`: scratch object path (zig cc mishandles -fsyntax-only)."""
    pathlib.Path(obj).parent.mkdir(parents=True, exist_ok=True)
    # zig caches compilations; a nonce define forces clang to run (and dump) again.
    nonce = f"-DFFH264_LAYOUT_NONCE={time.time_ns()}"
    out = subprocess.run(zcc(target) + [nonce, "-O0", "-c", "-o", str(obj), "-Xclang",
                                        "-fdump-record-layouts-complete", str(c_file)],
                         capture_output=True, text=True, check=True).stdout
    recs, cur = {}, None
    for line in out.splitlines():
        if line.startswith("*** Dumping AST Record Layout"):
            cur = "header"
            continue
        m = re.match(r"^\s*(\d+)(?::(\d+)-(\d+))? \|( +)(.*?)\s*$", line)
        if cur == "header" and m:
            hdr = m.group(5)
            mm = re.match(r"^(struct|union) (\w+)$", hdr)
            ma = re.search(r"\((?:unnamed|anonymous) (at [^)]*)\)", hdr)
            key = mm.group(2) if mm else "(" + ma.group(1) + ")" if ma else None
            kind = hdr.split()[0]
            cur = None
            if key and key not in recs:
                cur = recs[key] = {"kind": kind, "fields": []}
            continue
        if isinstance(cur, dict):
            if m and len(m.group(4)) == 3:  # top-level field: "|   type name"
                text = m.group(5)
                anon = text.endswith(")")
                name = None if anon else text.split()[-1]
                typ = text if anon else text[: -len(name)].strip()
                cur["fields"].append((name, int(m.group(1)), m.group(2) is not None, typ))
            sm = re.search(r"\[sizeof=(\d+), (?:dsize=\d+, )?align=(\d+)", line)
            if sm:
                cur["size"], cur["align"] = int(sm.group(1)), int(sm.group(2))
                cur = None
    return {k: v for k, v in recs.items() if "size" in v}


def probe(target, c_file, structs, recs, workdir):
    """For each struct name: exact (name, offset, size, natural alignment) of
    every top-level field, plus the struct's size and alignment. Named fields
    are measured by a probe object; anonymous members come from the dump."""
    lines, keys = [], []
    for s in structs:
        t = f"struct {s}"
        keys.append((s, None))
        lines.append(f"char __ffp_{len(keys)}_s[sizeof({t}) + 1];")
        lines.append(f"char __ffp_{len(keys)}_a[_Alignof({t}) + 1];")
        lines.append(f"char __ffp_{len(keys)}_o[1];")
        for f, _, bit, _ in recs[s]["fields"]:
            assert not bit, f"bitfield in {s}"
            if f is None:
                continue
            keys.append((s, f))
            e = f"((({t} *)0)->{f})"
            lines.append(f"char __ffp_{len(keys)}_o[__builtin_offsetof({t}, {f}) + 1];")
            lines.append(f"char __ffp_{len(keys)}_s[sizeof{e} + 1];")
            lines.append(f"char __ffp_{len(keys)}_a[_Alignof(__typeof__{e}) + 1];")
    workdir = pathlib.Path(workdir)
    workdir.mkdir(parents=True, exist_ok=True)
    src = workdir / "probe.c"
    src.write_text(pathlib.Path(c_file).read_text() + "\n" + "\n".join(lines) + "\n")
    obj = workdir / "probe.o"
    subprocess.run(zcc(target) + ["-fno-common", "-c", "-o", str(obj), str(src)], check=True)
    vals = {}
    for line in subprocess.run(["nm", "-S", str(obj)], capture_output=True, text=True,
                               check=True).stdout.splitlines():
        m = re.match(r"^[0-9a-f]+ ([0-9a-f]+) \w __ffp_(\d+)_([soa])$", line)
        if m:
            vals[(int(m.group(2)), m.group(3))] = int(m.group(1), 16) - 1
    measured = {}
    for i, (s, f) in enumerate(keys, 1):
        measured[(s, f)] = {k: vals[(i, k)] for k in "osa"}
    res = {}
    for s in structs:
        top = measured[(s, None)]
        fields = []
        for f, off, _, typ in recs[s]["fields"]:
            if f is None:
                inner = recs["(" + re.search(r"\((?:unnamed|anonymous) (at [^)]*)\)", typ).group(1) + ")"]
                fields.append((None, off, inner["size"], inner["align"]))
            else:
                v = measured[(s, f)]
                assert v["o"] == off, (s, f)
                fields.append((f, off, v["s"], v["a"]))
        res[s] = {"size": top["s"], "align": top["a"], "fields": fields}
    return res
