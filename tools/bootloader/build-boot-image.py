#!/usr/bin/env python3
"""Build the exact-G13 replaceable boot_a bundle. Never accesses the TV."""
import hashlib
import json
import struct
import zlib
import lz4.block
from pathlib import Path
from os_profile import profile, BACKUP, OUT


def word_hash(data):
    data += bytes(-len(data) % 4)
    value = 2166136261
    for word, in struct.iter_unpack('<I', data):
        value = ((value ^ word) * 16777619) & 0xffffffff
    return value


def build(gate=None, output=OUT):
    p = profile()
    assert struct.unpack_from('<I', p['init'], 12)[0] == 2262498
    assert struct.unpack_from('<I', p['vendor'], 24)[0] == 18128651
    gate = (OUT/'g13-gate.bin').read_bytes() if gate is None else gate
    assert 0 < len(gate) <= 0x40000
    stage0 = (OUT/'g13-stage0.bin').read_bytes()
    assert len(stage0) <= 0x2000
    packed_gate = lz4.block.compress(gate, mode='high_compression', store_size=False)
    assert lz4.block.decompress(packed_gate, uncompressed_size=len(gate)) == gate
    prefix_size = 0x20000
    original_prefix = p['boot'][:4096+prefix_size]
    compressed = lz4.block.compress(original_prefix, mode='high_compression', store_size=False)
    assert lz4.block.decompress(compressed, uncompressed_size=len(original_prefix)) == original_prefix
    bundle = bytearray(stage0 + bytes(0x3000-len(stage0)) + packed_gate)
    bundle += bytes(-len(bundle) % 4)
    original_offset = len(bundle)
    bundle += compressed
    bundle += bytes(-len(bundle) % 4)
    meta_offset = len(bundle)
    meta = p['vendor'][:4096] + p['vendor'][18132992:] + p['init'][:4096] + p['control']
    assert len(meta) == 56896
    packed_meta = lz4.block.compress(meta, mode='high_compression', store_size=False)
    assert lz4.block.decompress(packed_meta, uncompressed_size=len(meta)) == meta
    bundle += packed_meta
    data_end = len(bundle)
    assert data_end <= prefix_size
    bundle += bytes(prefix_size-len(bundle))
    bundle += p['boot'][4096+prefix_size:4096+15871916]
    assert len(bundle) == 15871916
    vendor = p['vendor'][4096:4096+18128651]
    generic = p['init'][4096:4096+2262498]
    header = bytearray(80)
    struct.pack_into('<19I', header, 0, 0x3142474c, 4, len(bundle), data_end,
                     original_offset, len(compressed), word_hash(compressed), word_hash(gate),
                     word_hash(p['boot']), meta_offset, len(packed_meta), word_hash(packed_meta),
                     word_hash(vendor+generic), len(vendor+generic), prefix_size,
                     0x3000, len(packed_gate), len(gate), word_hash(packed_gate))
    struct.pack_into('<I', header, 76, zlib.crc32(header[:76]))
    bundle[0x2000:0x2000+80] = header
    entries = {'stage0_bytes':len(stage0), 'compressed_gate': {'offset':0x3000,'bytes':len(packed_gate)},
               'compressed_boot': {'offset': original_offset, 'bytes': len(compressed)},
               'metadata': {'offset': meta_offset, 'bytes': len(packed_meta), 'expanded_bytes':len(meta)},
               'data_end': data_end, 'prefix_bytes': prefix_size, 'existing_ramdisk_bytes': len(vendor+generic)}
    original = (BACKUP/'boot_a.img').read_bytes()
    image = bytearray(original)
    assert image[:8] == b'ANDROID!' and struct.unpack_from('<I', image, 40)[0] == 4
    assert len(bundle) + 4096 < len(image) - 65536
    assert struct.unpack_from('<I', image, 8)[0] == len(bundle)
    image[4096:4096+len(bundle)] = bundle
    output.mkdir(parents=True,exist_ok=True)
    (output/'g13-boot-bundle.bin').write_bytes(bundle)
    (output/'g13-boot_a.img').write_bytes(image)
    (output/'g13-boot-ram.img').write_bytes(image[:len(p['boot'])])
    report = {'installed': False, 'target': '18ACC22D1C47 / G13 AU13 / slot a',
              'bundle_bytes': len(bundle), 'gate_bytes': len(gate),
              'entries': entries, 'image_bytes': len(image),
              'image_sha256': hashlib.sha256(image).hexdigest(),
              'bundle_sha256': hashlib.sha256(bundle).hexdigest(),
              'original_boot_sha256': hashlib.sha256(original).hexdigest(),
              'preserved': ['MBOOT', 'boot0', 'boot_b', 'vendor_boot_a', 'init_boot_a', 'vbmeta_a'],
              'avb': 'Original metadata retained; unlocked-device hash mismatch expected.'}
    (output/'boot-image.json').write_text(json.dumps(report, indent=2)+'\n')
    print(json.dumps(report, indent=2))


if __name__ == '__main__': build()
