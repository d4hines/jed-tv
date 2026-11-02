#!/usr/bin/env python3
"""Undefined symbols of a set of static archives that the set itself does not
define, i.e. what the C code imports from outside (libc, libm, compiler rt).

  python3 nolibc/tools/archive_imports.py NAME OUT.md ARCHIVE...

Writes a Markdown table: symbol, number of referencing members, examples.
Archive-level (every member), so it over-approximates what a given program
pulls in; tools/probe_undefined.py gives the link-level list.
"""
import collections
import pathlib
import subprocess
import sys

NM = "/opt/homebrew/opt/llvm/bin/llvm-nm"


def main(name, out, *archives):
    defined, refs = set(), collections.defaultdict(set)
    for a in archives:
        text = subprocess.run([NM, "-A", a], capture_output=True, text=True).stdout
        for line in text.splitlines():
            # "<archive>:<member>: <value> <type> <name>" or "...: <type> <name>"
            head, _, rest = line.rpartition(": ")
            parts = rest.split()
            if len(parts) < 2:
                continue
            kind, sym = parts[-2], parts[-1]
            member = head.split(":")[-1]
            if kind in ("U", "w", "v") and len(parts) == 2:
                refs[sym].add(f"{pathlib.Path(a).name}:{member}")
            elif kind not in ("U",):
                defined.add(sym)
    missing = sorted(s for s in refs if s not in defined)
    lines = [f"# {name}: {len(missing)} symbols imported from outside", "",
             "Archives: " + ", ".join(f"`{a}`" for a in archives), "",
             "| symbol | members | e.g. |", "|---|---|---|"]
    for s in missing:
        ex = sorted(refs[s])
        lines.append(f"| `{s}` | {len(ex)} | {', '.join(e.split(':')[-1] for e in ex[:3])} |")
    pathlib.Path(out).parent.mkdir(parents=True, exist_ok=True)
    pathlib.Path(out).write_text("\n".join(lines) + "\n")
    print(f"{out}: {len(missing)} imported symbols")
    print(" ".join(missing))


if __name__ == "__main__":
    main(*sys.argv[1:])
