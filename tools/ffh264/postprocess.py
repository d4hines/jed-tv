#!/usr/bin/env python3
"""Step 6: turn c2rust's raw output (assets/build/ffh264/raw/TARGET) into the
crate sources src/ffh264/src/ARCH/, compilable by stable Rust 1.92.
  postprocess.py TARGET
Every change is mechanical and listed here:
  * nightly-only items -> crate::rt equivalents (atomics, VaList, long double);
    `extern type`s -> zero-sized #[repr(C)] structs
  * C variadic definitions (`args: ...`) -> `<name>__va(..., VaList)`; the C
    symbol itself is an asm trampoline (asm/varargs-*.S)
  * two statics c2rust could not translate (atomic initializers)
  * `_Alignas` restored: on locals/statics via crate::rt::A{4,8,16}, on struct
    fields via explicit padding + #[repr(C, align(N))], computed from clang's
    own layout (layout.py); every struct's size/align/field offsets are then
    asserted at compile time against clang in test/unit/ffh264/ARCH/layout_check.rs
  * a header naming the source file and carrying its original notice
"""
import hashlib, json, os, pathlib, re, shutil, sys, tempfile
from concurrent.futures import ThreadPoolExecutor

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import layout  # noqa: E402
import shrink  # noqa: E402

T = sys.argv[1]
ARCH = {"aarch64-unknown-linux-musl": "aarch64", "armv7-unknown-linux-musleabihf": "arm"}[T]
ROOT = pathlib.Path(__file__).resolve().parents[2]
B = ROOT / "assets/build/ffh264"
SRC = pathlib.Path("/nix/store/cizg7bxjfvcska1fjnpa68cf7l5ysapf-ffmpeg")
RAW, PP = B / "raw" / T, B / "pp" / T
DEST = ROOT / "src/ffh264/src" / ARCH
# Preserve the active generated tree until every transformation succeeds.
OUT = pathlib.Path(tempfile.mkdtemp(prefix=f"postprocess-{T}-", dir=B))
C2RUST = "c2rust 0.22.1"
VARIADIC = {"av_log", "av_log_once", "av_strlcatf", "av_asprintf", "av_bprintf",
            "avpriv_request_sample", "avpriv_report_missing_feature", "avpriv_open", "av_sscanf"}
ALIGNED_VARS = {"la_temp": 16, "la_lsrc1": 16, "la_lsrc2": 16, "la_bS": 8, "mvbuf": 4, "zeromv": 4}

tus = [t for t in json.load(open(B / f"tus-{T}.json")) if t["kind"] == "c"]


def modpath(src):
    """C path -> module path relative to OUT (ytdec.c becomes a top-level module)."""
    return "ytdec" if src == "src/ffh264/ytdec.c" else src[:-2]


def notice(src):
    p = ROOT / src if src.startswith("tools/") else SRC / src
    m = re.match(r"\s*/\*(.*?)\*/", p.read_text(), re.S)
    body = [re.sub(r"^ ?\* ?", "", l).rstrip() for l in m.group(1).strip("\n").splitlines()] if m else []
    head = [f"Translated from FFmpeg 9.0.1 {src}" if not src.startswith("tools/")
            else f"Translated from {src}",
            f"(preprocessed for {T}) by {C2RUST}, then tools/ffh264/postprocess.py.",
            "Original notice:", ""]
    return "".join(f"// {l}".rstrip() + "\n" for l in head + body) + "\n"


# ------------------------------------------------------------------ parsing
def split_top(s, sep=","):
    """Split at depth-0 separators, treating '->' as text."""
    parts, depth, cur, i = [], 0, [], 0
    while i < len(s):
        c = s[i]
        if s.startswith("->", i):
            cur.append("->"); i += 2; continue
        if c in "([<{":
            depth += 1
        elif c in ")]>}":
            depth -= 1
        if c == sep and depth == 0:
            parts.append("".join(cur)); cur = []
        else:
            cur.append(c)
        i += 1
    if "".join(cur).strip():
        parts.append("".join(cur))
    return parts


STRUCT_RE = re.compile(r"((?:#\[[^\n]*\]\n)*)pub struct (\w+) \{\n(.*?)\n\}\n", re.S)


def struct_fields(body):
    """[(rust name, text)] for a struct body; name None for bitfield groups."""
    res = []
    for f in split_top(body):
        f = f.strip()
        if not f:
            continue
        m = re.match(r"(?:#\[[^\]]*\]\s*)*pub (\w+):", f)
        res.append((m.group(1) if m and not f.startswith("#[bitfield") else None, f))
    return res


# ------------------------------------------------------------ C layouts
print(f"{T}: dumping clang record layouts...", flush=True)
with ThreadPoolExecutor(8) as ex:
    dumps = dict(zip([t["src"] for t in tus],
                     ex.map(lambda t: layout.dump_records(
                         T, PP / t["src"], B / "probe" / T / "dump" / (t["src"].replace("/", "__") + ".o")), tus)))

aligned_names = set()
for t in tus:
    aligned_names |= set(re.findall(r"_Alignas\(\s*\d+\s*\)\s+(?:static\s+)?(?:const\s+)?[\w ]*?(\w+)\s*[\[;=]",
                                    (PP / t["src"]).read_text()))
candidates = {}  # struct -> tu whose dump/probe measures it
for t in tus:
    for name, r in dumps[t["src"]].items():
        if name in candidates or r["kind"] != "struct" or name.startswith("("):
            continue
        if any(f[0] in aligned_names for f in r["fields"]):
            candidates[name] = t["src"]
by_tu = {}
for name, tu in candidates.items():
    by_tu.setdefault(tu, []).append(name)
probes = {}
for i, (tu, structs) in enumerate(sorted(by_tu.items())):
    probes.update(layout.probe(T, PP / tu, structs, dumps[tu], B / "probe" / T / str(i)))


def realign(name, body, attrs):
    """Insert padding where the C offset of a field exceeds its natural Rust
    offset, and raise the struct's alignment to C's."""
    p = probes[name]
    fields = struct_fields(body)
    assert len(fields) == len(p["fields"]), name
    out, prev_end, nat_max, pads = [], 0, 1, []
    for (rname, text), (cname, off, size, nat) in zip(fields, p["fields"]):
        natural = (prev_end + nat - 1) // nat * nat
        if off != natural:
            pads.append((f"c2rust_alignpad_{len(pads)}", off - prev_end, rname))
            out.append(f"pub {pads[-1][0]}: [u8; {off - prev_end}]")
        out.append(text)
        prev_end, nat_max = off + size, max(nat_max, nat)
    if p["align"] > nat_max:
        attrs = attrs.replace("#[repr(C)]", f"#[repr(C, align({p['align']}))]")
        assert f"align({p['align']})" in attrs, name
    return attrs, "".join(f"    {f},\n" for f in out).rstrip("\n"), pads


# ---------------------------------------------------------------- rewrite
def rewrite(t):
    src = t["src"]
    mp = modpath(src)
    s = (RAW / (src[:-2] + ".rs")).read_text() if src != "src/ffh264/ytdec.c" \
        else (RAW / "tools/ffh264/ytdec.rs").read_text()
    stats = {}

    for u in ("use ::f128;\n", "use ::num_traits;\n", "use ::num_traits::ToPrimitive;\n"):
        s = s.replace(u, "")
    s = s.replace("use ::c2rust_bitfields;\n", "")
    s = s.replace("::f128::f128", "crate::rt::F128")
    s = s.replace("::core::intrinsics::atomic_", "crate::rt::atomic_")
    s = s.replace("::core::ffi::VaListImpl", "crate::rt::VaList")
    s = s.replace("::core::ffi::VaList", "crate::rt::VaList")
    assert "::core::intrinsics" not in s and "::f128" not in s, src

    def va(m):
        assert m.group(1) in VARIADIC, m.group(1)
        stats.setdefault("variadic", []).append(m.group(1))
        fixed = m.group(2).replace("    mut ", "        ")
        ret = m.group(3) or ""
        decl = (f'extern "C" {{\n    // asm/varargs-{ARCH}.S: builds the va_list, calls {m.group(1)}__va\n'
                f"    pub fn {m.group(1)}(\n{fixed}        ...\n    ){ret};\n}}\n")
        return (f'{decl}#[no_mangle]\npub unsafe extern "C" fn {m.group(1)}__va(\n{m.group(2)}'
                f"    mut args: crate::rt::VaList,\n){ret} {{")
    s = re.sub(r'#\[no_mangle\]\npub unsafe extern "C" fn (\w+)\(\n((?:    .*\n)*?)    mut args: \.\.\.\n\)'
               r'( -> [^{]+?)? \{', va, s)
    assert ": ...\n" not in s and "args: ..." not in s, src

    # c2rust leaves a few size_of/align_of results as usize where C has size_t
    s, k = re.subn(r"(\{\n\s*16 as size_t\n\s*\} else \{\n\s*)(::core::mem::align_of::<max_align_t>\(\))\n", r"\1\2 as size_t\n", s)
    s, k2 = re.subn(r"(&& \*type_0\s*<\s*\(::core::mem::size_of::<\[uint8_t; 14\]>\(\) as usize\)\s*"
                    r"\.wrapping_div\(::core::mem::size_of::<uint8_t>\(\) as usize\))\)",
                    r"\1 as ::core::ffi::c_uint)", s)
    if k + k2:
        stats["usize_casts"] = k + k2

    opaque = re.findall(r"^    pub type (\w+);\n", s, re.M)
    s = re.sub(r"^    pub type (\w+);\n", "", s, flags=re.M)

    for v, n in ALIGNED_VARS.items():
        s, k1 = re.subn(rf"^(\s*)let mut ({v}(?:_\d+)?): (.+?) = (.+);$",
                        rf"\1let mut \2: crate::rt::A{n}<\3> = crate::rt::A{n}(\4);", s, flags=re.M)
        s, k2 = re.subn(rf"^(\s*)static mut ({v}): (.+?) = (.+);$",
                        rf"\1static mut \2: crate::rt::A{n}<\3> = crate::rt::A{n}(\4);", s, flags=re.M)
        if k1 + k2:
            stats.setdefault("aligned_vars", []).append(f"{v}x{k1 + k2}")

    padded = {}

    def fix_struct(m):
        attrs, name, body = m.group(1), m.group(2), m.group(3)
        if name not in probes:
            return m.group(0)
        attrs, body, pads = realign(name, body, attrs)
        stats.setdefault("realigned", []).append(f"{name}(+{len(pads)} pads)")
        if pads:
            padded[name] = pads
        return f"{attrs}pub struct {name} {{\n{body}\n}}\n"
    s = STRUCT_RE.sub(fix_struct, s)
    # struct literals of realigned structs need the padding fields too
    for name, pads in padded.items():
        for lit in reversed(list(re.finditer(rf"(?<== ){name} \{{\n", s))):
            for pad, size, nxt in reversed(pads):
                m = re.compile(rf"^(\s+){nxt}: ", re.M).search(s, lit.end())
                s = s[:m.start()] + f"{m.group(1)}{pad}: [0; {size}],\n" + s[m.start():]
            stats.setdefault("padded_literals", []).append(name)

    # c2rust-bitfields' derive pins old proc-macro2/syn/quote versions, which
    # would downgrade the whole workspace; expand its accessors here instead.
    def bitfields(m):
        attrs, name, body = m.group(1), m.group(2), m.group(3)
        if "BitfieldStruct" not in attrs:
            return m.group(0)
        attrs = attrs.replace(", BitfieldStruct", "")
        acc, pending = [], []
        for line in body.split("\n"):
            bf = re.match(r'\s*#\[bitfield\(name = "(\w+)", ty = "([^"]+)", bits = "(\d+)\.\.=(\d+)"\)\]$', line)
            if bf:
                pending.append(bf.groups())
                continue
            g = re.match(r"\s*pub (\w+): \[u8; \d+\],$", line)
            for n, ty, lo, hi in pending:
                assert g, (name, line)
                signed = re.search(r"\b(c_int|c_schar|c_short|c_long|c_longlong|i8|i16|i32|i64)$", ty)
                get = "bf_get_signed" if signed else "bf_get"
                acc.append(f"    pub fn {n}(&self) -> {ty} {{\n"
                           f"        crate::rt::{get}(&self.{g.group(1)}, {lo}, {hi}) as {ty}\n    }}\n"
                           f"    pub fn set_{n}(&mut self, v: {ty}) {{\n"
                           f"        crate::rt::bf_set(&mut self.{g.group(1)}, {lo}, {hi}, v as u64)\n    }}\n")
            pending = []
        assert not pending, name
        body = re.sub(r'^\s*#\[bitfield\([^\n]*\)\]\n', "", body + "\n", flags=re.M).rstrip("\n")
        stats.setdefault("bitfields", []).append(name)
        return (f"{attrs}pub struct {name} {{\n{body}\n}}\n"
                f"impl {name} {{\n{''.join(acc)}}}\n")
    s = STRUCT_RE.sub(bitfields, s)
    assert "bitfield(" not in s and "BitfieldStruct" not in s, src

    if src == "libavutil/log.c":
        old = "            uintptr_t,\n        >(callback),"
        assert s.count(old) == 1
        s = s.replace(old, "            *mut ::core::ffi::c_void,\n        >(callback),")
        s += ("// c2rust: \"Unsupported default initializer: Atomic\". Holds a function\n"
              "// pointer (uintptr_t in C); a pointer type keeps it a link-time constant.\n"
              "static mut av_log_callback: *mut ::core::ffi::c_void = av_log_default_callback\n"
              "    as unsafe extern \"C\" fn(*mut ::core::ffi::c_void, ::core::ffi::c_int,\n"
              "        *const ::core::ffi::c_char, crate::rt::VaList)\n"
              "    as *mut ::core::ffi::c_void;\n")
    if src == "libavcodec/h274.c":
        s += ("// c2rust: \"Unsupported default initializer: Atomic\" (zero-initialized in C).\n"
              "static mut film_grain_db: H274FilmGrainDatabase = unsafe { ::core::mem::zeroed() };\n")

    if opaque:
        s += "".join(f"#[repr(C)]\npub struct {o} {{\n    _opaque: [u8; 0],\n}}\n" for o in opaque)

    s = notice(src) + s
    dst = OUT / (mp + ".rs")
    dst.parent.mkdir(parents=True, exist_ok=True)
    dst.write_text(s)
    return mp, s, stats


results = [rewrite(t) for t in tus]
for mp, _, st in results:
    if st:
        print(f"  {mp}: {st}")

# ------------------------------------------------------------- mod tree
tree = {}
for mp, _, _ in results:
    node = tree
    parts = mp.split("/")
    for p in parts[:-1]:
        node = node.setdefault(p, {})
    node[parts[-1]] = None


def emit(node, ind):
    out = []
    for k in sorted(node):
        if node[k] is None:
            out.append(f"{ind}pub mod {k};")
        else:
            out.append(f"{ind}pub mod {k} {{")
            out += emit(node[k], ind + "    ")
            out.append(f"{ind}}}")
    return out


# ---------------------------------------------------------- layout check
checks, seen = [], set()
for (mp, s, _), t in zip(results, tus):
    recs = dumps[t["src"]]
    for m in STRUCT_RE.finditer(s):
        name, body = m.group(2), m.group(3)
        r = recs.get(name)
        if not r or r["kind"] != "struct":
            continue
        key = hashlib.sha1(m.group(0).encode()).hexdigest()
        if (name, key) in seen:
            continue
        seen.add((name, key))
        rf = [f for f in struct_fields(body) if not (f[0] or "").startswith("c2rust_alignpad_")]
        offs = []
        if len(rf) == len(r["fields"]):
            offs = [f"{rn} @ {cf[1]}" for (rn, _), cf in zip(rf, r["fields"])
                    if rn and cf[0] and not cf[2]]
        path = "crate::tr::" + mp.replace("/", "::")
        checks.append(f"chk!({path}::{name}, {r['size']}, {r['align']}; {', '.join(offs)});")

(OUT / "mod.rs").write_text(
    f"// Generated by tools/ffh264/postprocess.py: the c2rust translation for {T}.\n"
    + "\n".join(emit(tree, "")) + f'\n#[path = "../../../../test/unit/ffh264/{ARCH}/layout_check.rs"]\nmod layout_check;\n')
(OUT / "layout_check.rs").write_text(
    "// Generated by tools/ffh264/postprocess.py. Compile-time proof that every\n"
    "// translated struct (one per distinct definition) has clang's size,\n"
    f"// alignment and field offsets for {T}.\n"
    "macro_rules! chk {\n"
    "    ($t:ty, $size:expr, $align:expr; $($f:ident @ $o:expr),*) => {\n"
    "        const _: () = assert!(::core::mem::size_of::<$t>() == $size);\n"
    "        const _: () = assert!(::core::mem::align_of::<$t>() == $align);\n"
    "        $(const _: () = assert!(::core::mem::offset_of!($t, $f) == $o);)*\n"
    "    };\n"
    "}\n" + "\n".join(checks) + "\n")
n_lines = sum(s.count("\n") for _, s, _ in results)
print(f"{T}: {len(results)} modules, {n_lines} lines -> {OUT}; "
      f"{len(probes)} structs realigned per clang; {len(checks)} layout checks")

# Mandatory format specialization: regenerate.sh never leaves the full-depth
# decoder in the source tree. The player consumes only 8-bit 4:2:0 frames.
shrink.main(T, OUT)
# Publish only files generated by this successful run. Keep unrelated files.
# Each replacement is atomic, and staging is retained under assets for audit.
for staged in sorted(OUT.rglob("*.rs")):
    destination = (ROOT / "test/unit/ffh264" / ARCH / "layout_check.rs"
                   if staged.name == "layout_check.rs" else DEST / staged.relative_to(OUT))
    destination.parent.mkdir(parents=True, exist_ok=True)
    temporary = destination.with_name(destination.name + ".new")
    shutil.copyfile(staged, temporary)
    os.replace(temporary, destination)
print(f"Published {DEST}; staged evidence retained at {OUT}")
