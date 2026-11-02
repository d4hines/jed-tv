#!/usr/bin/env python3
"""Create a permissive, RAM-only linker map for the native research shell."""
from pathlib import Path
names = ['com_android_art', 'com_android_i18n', 'com_android_conscrypt', 'com_android_runtime', 'sphal']
paths = ['/system/${LIB}', '/system_ext/${LIB}', '/apex/com.android.art/${LIB}', '/apex/com.android.runtime/${LIB}', '/apex/com.android.i18n/${LIB}', '/apex/com.android.conscrypt/${LIB}', '/apex/com.android.os.statsd/${LIB}', '/vendor/${LIB}', '/apex/com.android.vndk.v30/${LIB}']
lines = ['dir.system = /system/bin/', '[system]', 'additional.namespaces = ' + ','.join(names)]
for name in ['default'] + names:
    lines += [f'namespace.{name}.isolated = false', f'namespace.{name}.visible = true', f'namespace.{name}.search.paths = ' + ':'.join(paths)]
    if name != 'default':
        lines += [f'namespace.{name}.links = default', f'namespace.{name}.link.default.allow_all_shared_libs = true']
Path('assets/research/native-vendor-stack/java-ld.config.txt').write_text('\n'.join(lines) + '\n')
