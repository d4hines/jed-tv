#!/usr/bin/env python3
"""Specialize postprocessed c2rust H.264 to the player's 8-bit 4:2:0 contract.
Run after postprocess.py for each architecture. Works on the generated Rust,
not raw c2rust or assembly. Unsupported SPS values are rejected before DSP
initialization. High-depth template definitions and extern declarations are
removed only once all non-template references have disappeared.
"""
import json
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[2]
ARCH = {"armv7-unknown-linux-musleabihf": "arm", "aarch64-unknown-linux-musl": "aarch64"}


def mask(s):
    # c2rust output uses ordinary strings/comments (no Rust raw strings).
    # Preserve newlines and byte positions so ranges still address the source.
    pat = r'//[^\n]*|/\*.*?\*/|b?"(?:\\.|[^"\\])*"'
    return re.sub(pat, lambda m: re.sub(r'[^\n]', ' ', m.group()), s, flags=re.S)


def close_brace(s, at):
    assert s[at] == '{'
    depth = 1
    for i in range(at + 1, len(s)):
        depth += (s[i] == '{') - (s[i] == '}')
        if depth == 0:
            return i + 1
    raise ValueError("unclosed generated block")


def replace_once(s, old, new):
    n = s.count(old)
    if n != 1:
        raise ValueError(f"expected exactly one generated pattern, found {n}: {old[:70]}")
    return s.replace(old, new)


def specialize(s):
    # bit_depth defaults to 8; the named >8 match arms instantiate the bulk
    # of the generated prediction/interpolation/IDCT code.
    while True:
        text = mask(s)
        m = re.search(r'\bmatch bit_depth\s*\{', text)
        if not m:
            break
        start = text.index('{', m.start())
        end = close_brace(text, start)
        arms = text[start + 1:end - 1]
        keep = re.search(r'\b_\s*=>\s*\{', arms)
        if not keep:
            raise ValueError("bit_depth dispatch has no 8-bit default arm")
        at = start + 1 + keep.end() - 1
        arm_end = close_brace(text, at)
        # The retained body includes FFmpeg's existing <=8 assertion.
        s = s[:m.start()] + '{' + s[at + 1:arm_end - 1] + '}' + s[end:]
    # The two generic >8 initializers use 16-bit storage, not an explicit
    # 9/10/12/14 dispatch. Keep their original 8-bit else branch.
    pat = r'if bit_depth > 8 as ::core::ffi::c_int && bit_depth <= 16 as ::core::ffi::c_int\s*\{'
    while (m := re.search(pat, mask(s))):
        text = mask(s)
        high_end = close_brace(text, m.end() - 1)
        tail = re.match(r'\s*else\s*\{', text[high_end:])
        if not tail:
            raise ValueError("high-depth initialization has no 8-bit else")
        low_start = high_end + tail.end() - 1
        low_end = close_brace(text, low_start)
        s = s[:m.start()] + s[low_start:low_end] + s[low_end:]
    return s


def functions(s):
    text = mask(s)
    pat = r'(?m)^(?:#\[[^\n]*\]\n)*(?:pub )?(?:unsafe )?extern "C" fn (\w+)\s*\('
    # mask() blanks the extern "C" string; match the source headers instead.
    for m in re.finditer(pat, s):
        at = text.find('{', m.end())
        semi = text.find(';', m.end())
        if at < 0 or 0 <= semi < at:
            continue
        yield m.group(1), m.start(), close_brace(text, at)


def prune(tree):
    candidates = {}
    for p, s in tree.items():
        for name, a, b in functions(s):
            if re.search(r'_(?:9|10|12|14|16)(?:_c)?$', name):
                candidates[name] = (p, a, b)
    # Reachability from every retained definition, type and static. Some
    # template helpers have ordinary names; those remain conservatively.
    without = dict(tree)
    for p in tree:
        spans = sorted((a, b) for name, (q, a, b) in candidates.items() if q == p)
        for a, b in reversed(spans):
            without[p] = without[p][:a] + ' ' * (b-a) + without[p][b:]
        # Extern function declarations alone do not make a definition live.
        without[p] = re.sub(r'(?ms)^    fn \w+\(.*?;', '', without[p])
    live = set()
    refs = set(re.findall(r'\b\w+\b', '\n'.join(mask(s) for s in without.values())))
    pending = list(refs & candidates.keys())
    while pending:
        name = pending.pop()
        if name in live:
            continue
        live.add(name)
        p, a, b = candidates[name]
        pending.extend(set(re.findall(r'\b\w+\b', mask(tree[p][a:b]))) & candidates.keys() - live)
    dead = candidates.keys() - live
    for p in tree:
        s = tree[p]
        spans = sorted((a,b) for name,(q,a,b) in candidates.items() if name in dead and q==p)
        for a,b in reversed(spans):
            s = s[:a] + s[b:]
        s = re.sub(r'(?ms)^    fn (\w+)\(.*?;\n',
                   lambda m: '' if m.group(1) in dead else m.group(), s)
        tree[p] = s.rstrip() + '\n'
    return sorted(dead)


def main(target, output=None):
    out = output or ROOT / 'src/ffh264/src' / ARCH[target]
    paths = sorted(out.rglob('*.rs'))
    tree = {p: p.read_text() for p in paths}
    before = sum(s.count('\n') for s in tree.values())
    # Refuse to reprocess a specialized tree; regeneration supplies originals.
    marker = '// Specialized to 8-bit 4:2:0 by tools/ffh264/shrink.py.\n'
    if marker in tree[out/'libavcodec/h264_ps.rs']:
        print(f'{target}: already specialized')
        return
    p = out/'libavcodec/h264_ps.rs'
    s = replace_once(tree[p], '(*sps).chroma_format_idc as ::core::ffi::c_uint > 3 as ::core::ffi::c_uint',
                     '(*sps).chroma_format_idc != 1 as ::core::ffi::c_int')
    old = '''(*sps).bit_depth_luma < 8 as ::core::ffi::c_int
                            || (*sps).bit_depth_luma > 14 as ::core::ffi::c_int
                            || (*sps).bit_depth_chroma < 8 as ::core::ffi::c_int
                            || (*sps).bit_depth_chroma > 14 as ::core::ffi::c_int'''
    s = replace_once(s, old, '''(*sps).bit_depth_luma != 8 as ::core::ffi::c_int
                            || (*sps).bit_depth_chroma != 8 as ::core::ffi::c_int''')
    tree[p] = marker + s
    for p,s in tree.items():
        tree[p] = specialize(s)
    dead = prune(tree)
    after = sum(s.count('\n') for s in tree.values())
    for p,s in tree.items():
        if p.read_text() != s:
            p.write_text(s)
    report = {'target':target,'before_lines':before,'after_lines':after,'removed_lines':before-after,
              'removed_high_depth_functions':dead,'format':'8-bit YUV420P only; other SPS rejected'}
    path = ROOT/'assets/build/ffh264'/f'shrink-{target}.json'
    path.write_text(json.dumps(report,indent=2)+'\n')
    print(f'{target}: {before} -> {after} lines; {len(dead)} high-depth functions removed')

if __name__ == '__main__':
    for target in sys.argv[1:] or ARCH:
        main(target)
