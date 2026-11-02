"""Verify an Android whole-file OTA signature against an extracted stock cert.

Requires asn1crypto and cryptography. Supports detached RSA SignApk signatures
without signed attributes; rejects other formats. Does not install anything.
"""
import argparse
import hashlib
import json
from pathlib import Path
import struct
import zipfile

from asn1crypto import cms
from cryptography import x509
from cryptography.hazmat.primitives import hashes, serialization
from cryptography.hazmat.primitives.asymmetric import padding, utils

p = argparse.ArgumentParser(description=__doc__)
p.add_argument('ota', type=Path)
p.add_argument('trusted_certificate', type=Path)
p.add_argument('output', type=Path)
a = p.parse_args()
size = a.ota.stat().st_size
with a.ota.open('rb') as f:
    f.seek(-6, 2)
    signature_start, marker, comment_size = struct.unpack('<HHH', f.read(6))
    assert marker == 0xffff and 6 < signature_start <= comment_size
    f.seek(-comment_size - 22, 2)
    eocd = f.read(comment_size + 22)
    assert eocd[:4] == b'PK\x05\x06' and b'PK\x05\x06' not in eocd[4:]
    assert struct.unpack_from('<H', eocd, 20)[0] == comment_size
    signature_der = eocd[len(eocd) - signature_start:-6]
signed = cms.ContentInfo.load(signature_der, strict=True)
assert signed['content_type'].native == 'signed_data'
signed = signed['content']
assert len(signed['signer_infos']) == 1
signer = signed['signer_infos'][0]
assert signer['signed_attrs'].native is None
assert signed['encap_content_info']['content'].native is None
assert signer['signature_algorithm']['algorithm'].native == 'rsassa_pkcs1v15'
algorithm = signer['digest_algorithm']['algorithm'].native
assert algorithm in ('sha1', 'sha256')
cert = x509.load_pem_x509_certificate(a.trusted_certificate.read_bytes())
public = cert.public_key()
signature = signer['signature'].native
signed_length = size - comment_size - 2
digest = hashlib.new(algorithm)
full_sha256, full_md5 = hashlib.sha256(), hashlib.md5()
with a.ota.open('rb') as f:
    offset = 0
    while chunk := f.read(4 * 1024 * 1024):
        full_sha256.update(chunk)
        full_md5.update(chunk)
        if offset < signed_length:
            digest.update(chunk[:signed_length - offset])
        offset += len(chunk)
hash_algorithm = hashes.SHA1() if algorithm == 'sha1' else hashes.SHA256()
public.verify(signature, digest.digest(), padding.PKCS1v15(), utils.Prehashed(hash_algorithm))
with zipfile.ZipFile(a.ota) as z:
    embedded = x509.load_pem_x509_certificate(z.read('META-INF/com/android/otacert'))
    embedded_matches = embedded.public_bytes(serialization.Encoding.DER) == cert.public_bytes(serialization.Encoding.DER)
    metadata = z.read('META-INF/com/android/metadata').decode()
    bad_entry = z.testzip()
assert embedded_matches and bad_entry is None
report = {'ota': str(a.ota), 'bytes': size, 'sha256': full_sha256.hexdigest(),
          'md5': full_md5.hexdigest(), 'signature_valid': True,
          'signature_digest': algorithm, 'signed_length': signed_length,
          'trusted_certificate': str(a.trusted_certificate),
          'trusted_certificate_sha256': cert.fingerprint(hashes.SHA256()).hex(),
          'embedded_certificate_matches': embedded_matches,
          'zip_entry_crc_checks_passed': True, 'metadata': metadata,
          'limit': 'Authenticity and archive integrity only. Does not establish recovery startup or successful installation.'}
a.output.write_text(json.dumps(report, indent=2) + '\n')
print(json.dumps(report, indent=2))
