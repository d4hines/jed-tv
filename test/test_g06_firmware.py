#!/usr/bin/env python3
"""Failure-oriented checks for firmware staging and module dependency auditing."""
import importlib.util
import pathlib
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('g06_firmware', ROOT / 'tools/g06_firmware.py')
fw = importlib.util.module_from_spec(spec)
spec.loader.exec_module(fw)


def module(name, imports=(), exports=(), depends=''):
    return {'file': name + '.ko', 'module_name': name, 'imports': list(imports),
            'exports': list(exports), 'modinfo': {'depends': depends}}


class FirmwareChecks(unittest.TestCase):
    def test_rejects_truncated_ramdisks(self):
        spec = importlib.util.spec_from_file_location('check_tv_boot', ROOT / 'tools/check_tv_boot.py')
        check = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(check)
        for data in [b'', b'070701' + b'0' * 104, b'not a cpio']:
            with self.assertRaises(ValueError):
                check.cpio(data)

    def test_rejects_non_elf_and_truncation(self):
        for data in [b'', b'not an ELF', b'\x7fELF\x01\x01\x01' + bytes(45)]:
            with self.assertRaises(ValueError):
                fw.elf(data)

    def test_rejects_parent_paths_and_debugfs_injection(self):
        # Mapping alone is tested; do not invoke any filesystem tool.
        files = object.__new__(fw.Files)
        for path in ['/vendor/../boot', '/vendor/x\nwrite', '/vendor/x"', 'relative/path']:
            with self.assertRaises(ValueError):
                files.map(path)

    def test_stock_lib_alias(self):
        files = object.__new__(fw.Files)
        image, source = files.map('/lib/ld-linux.so.3')
        self.assertEqual(image.name, 'tvservice.img')
        self.assertEqual(source, '/glibc/ld-linux.so.3')

    def test_resolves_shipped_symbols_despite_stale_split_module_names(self):
        modules = [module('utpa2k', exports=['MI_SYS_Init']),
                   module('mik', imports=['MI_SYS_Init', 'printk'], depends='utpa2k-system')]
        self.assertEqual(fw.module_order(modules, {'printk'}), [])

    def test_detects_actual_wrong_order(self):
        modules = [module('mik', imports=['MI_SYS_Init']), module('utpa2k', exports=['MI_SYS_Init'])]
        self.assertEqual(fw.module_order(modules, set())[0]['provider_not_loaded'], 'utpa2k')

    def test_detects_absent_symbol_even_with_plausible_metadata(self):
        modules = [module('utpa2k'), module('mik', imports=['MI_SYS_Init'], depends='utpa2k')]
        self.assertEqual(fw.module_order(modules, set())[0]['unresolved_symbol'], 'MI_SYS_Init')


if __name__ == '__main__':
    unittest.main()
