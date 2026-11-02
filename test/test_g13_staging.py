"""Check the G13 deployment catalog and rejection gates without device access."""
import hashlib
import importlib.util
import json
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]
with patch.object(sys, 'path', [str(ROOT / 'tools/native-app'), *sys.path]):
    spec = importlib.util.spec_from_file_location('g13_runner', ROOT / 'tools/native-app/run-g13.py')
    runner = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(runner)


class G13Staging(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.apps, self.media, self.out = [self.root / n for n in ('apps', 'media', 'out')]
        for directory in (self.apps / 'bin', self.media / 'doom', self.media / 'tiles', self.out):
            directory.mkdir(parents=True)
        self.names = ('tvinit', 'ytplay', 'slides', 'doomtv', 'tvosd', 'rpgtv', 'flighttv', 'snaketv')
        self.uids = [19001, 19002, 19003, 19005, 19006, 19007, 19008]
        (self.apps / 'app-uids.json').write_text(json.dumps(self.uids))
        for name in self.names:
            (self.apps / 'bin' / name).write_bytes(name.encode())
        for name in ('native-display', 'native-audio', 'android-client', 'adb-relay', 'audio-protocol-probe'):
            (self.out / name).write_bytes(name.encode())
        (self.media / 'doom/freedoom1.wad').write_bytes(b'wad')
        for name in ('demo-video', 'flight'):
            (self.media / 'tiles' / (name + '.rgb')).write_bytes(b'rgb')
        self.uploads = {}
        self.packages = 'ordinary.package 10001 rest\n'

    def shell(self, command):
        if command == 'cat /data/system/packages.list':
            return self.packages
        if command.startswith('sha256sum '):
            return ''.join(hashlib.sha256(self.uploads[name]).hexdigest() + '  ' + name + '\n'
                           for name in command.split()[1:])
        return ''

    def invoke(self):
        with patch.object(runner, 'OUT', self.out), \
             patch.object(runner, 'shell', side_effect=self.shell), \
             patch.object(runner, 'push', side_effect=lambda p, d: self.uploads.update({d: p.read_bytes()})), \
             patch.object(runner, 'stop') as stop, \
             patch.object(runner.subprocess, 'run'):
            self.stop = stop
            runner.stage({'apps': str(self.apps), 'media': str(self.media)})

    def test_current_catalog_and_tiles_are_uploaded_and_verified(self):
        self.invoke()
        self.stop.assert_called_once()
        for name in self.names:
            self.assertIn(f'{runner.BASE}/root/bin/{name}', self.uploads)
        self.assertIn(f'{runner.BASE}/root/share/tiles/flight.rgb', self.uploads)
        self.assertTrue((self.out / 'device-sha256.json').is_file())

    def test_new_app_uid_collision_is_rejected_before_stopping(self):
        self.packages = 'installed.package 19006 rest\n'
        with self.assertRaisesRegex(RuntimeError, 'collides'):
            self.invoke()
        self.stop.assert_not_called()
        self.assertFalse(self.uploads)

    def test_wrong_platform_catalog_is_rejected_before_stopping(self):
        (self.apps / 'app-uids.json').write_text(json.dumps([10001, 10002, 10003, 10005, 10006, 10007, 10008]))
        with self.assertRaisesRegex(RuntimeError, 'Rebuild'):
            self.invoke()
        self.stop.assert_not_called()
        self.assertFalse(self.uploads)


if __name__ == '__main__':
    unittest.main()
