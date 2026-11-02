#!/usr/bin/env python3
import importlib.util
import pathlib
import unittest
spec = importlib.util.spec_from_file_location('checker', pathlib.Path(__file__).with_name('check_framemd5.py'))
checker = importlib.util.module_from_spec(spec)
spec.loader.exec_module(checker)

class Completeness(unittest.TestCase):
    reference = '\n'.join(f'0, 0, 0, 1, 6, {i:032x}' for i in range(3))
    lines = [f'framemd5 {i} {i} 2x2 {i:032x}' for i in range(3)]
    def test_full(self):
        self.assertTrue(checker.compare('\n'.join(self.lines), self.reference)[0])
    def test_truncation_is_not_success(self):
        self.assertFalse(checker.compare(self.lines[0], self.reference)[0])
    def test_explicit_prefix(self):
        self.assertTrue(checker.compare(self.lines[0], self.reference, 1)[0])
    def test_skipped_index(self):
        lines = self.lines.copy(); lines[1] = lines[1].replace('framemd5 1', 'framemd5 4')
        self.assertFalse(checker.compare('\n'.join(lines), self.reference)[0])
    def test_extra_frame(self):
        self.assertFalse(checker.compare('\n'.join(self.lines + [self.lines[0]]), self.reference)[0])
    def test_empty(self):
        self.assertFalse(checker.compare('', self.reference)[0])

if __name__ == '__main__':
    unittest.main()
