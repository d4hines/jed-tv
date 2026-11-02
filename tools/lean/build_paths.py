#!/usr/bin/env python3
"""Read this repository's literal Lake buildDir overrides (or Lake's default)."""
from pathlib import Path
import json
import re
import sys


def build_dir(package):
    package = Path(package).resolve()
    for name in ('lakefile.lean', 'lakefile.toml'):
        config = package / name
        if not config.is_file():
            continue
        for line in config.read_text().splitlines():
            if line.strip().startswith('buildDir'):
                match = re.fullmatch(r'\s*buildDir\s*(?::=|=)\s*("[^"]+")\s*', line)
                if not match:
                    raise ValueError(f'{config}: buildDir must be a literal string')
                return (package / json.loads(match[1])).resolve()
    return package / '.lake/build'


if __name__ == '__main__':
    print(build_dir(sys.argv[1]))
