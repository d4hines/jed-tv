#!/usr/bin/env python3
"""Measure static native service dependencies; this is not an ABI/HAL test."""
import json
import posixpath
import re
import subprocess
import sys

sys.dont_write_bytecode = True
from minimal import WORK


def main():
    donor = WORK / 'android16-extracted'
    directories = ('/apex/com.android.runtime/lib/bionic', '/system/lib', '/system/system_ext/lib')

    def host(name):
        if name.startswith('/apex/com.android.runtime/'):
            return WORK / 'runtime-apex' / name.removeprefix('/apex/com.android.runtime/')
        return donor / name.lstrip('/')

    def resolve(name):
        seen = set()
        while host(name).is_symlink():
            if name in seen:
                raise RuntimeError('Symlink cycle: ' + name)
            seen.add(name)
            target = str(host(name).readlink())
            name = posixpath.normpath(posixpath.join(posixpath.dirname(name), target))
        return name

    cache = {}
    report = {}
    for binary in ('surfaceflinger', 'audioserver', 'hwservicemanager'):
        pending, seen, missing = ['/system/bin/' + binary], {}, {}
        while pending:
            name = resolve(pending.pop())
            if name in seen:
                continue
            path = host(name)
            if name not in cache:
                cache[name] = subprocess.check_output(['llvm-readelf', '-d', str(path)], text=True)
            seen[name] = path.stat().st_size
            for lib in re.findall(r'Shared library: \[([^\]]+)\]', cache[name]):
                found = next((resolve(directory + '/' + lib) for directory in directories
                              if host(resolve(directory + '/' + lib)).is_file()), None)
                if found:
                    pending.append(found)
                else:
                    missing.setdefault(lib, []).append(name)
        report[binary] = {'known_file_count': len(seen), 'known_bytes': sum(seen.values()),
                          'files': seen, 'missing_libraries': missing}
    report['scope'] = ('DT_NEEDED lower bounds only; excludes dlopen, linker namespace rules, '
                       'HALs, configuration, interpreter, data and kernel ABI. No services executed.')
    (WORK / 'media-dependency-survey.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps({name: {k: v for k, v in item.items() if k != 'files'}
                      for name, item in report.items() if isinstance(item, dict)}, indent=2))


if __name__ == '__main__':
    main()
