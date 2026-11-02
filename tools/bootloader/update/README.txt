Signed updates and network boot
===============================
Design, wire format and proof limits: src/bootloader/Update/README.txt.
Physical G13 evidence and installed image: research/G13_SIGNED_NETWORK_BOOT.txt.
New direct application-flash candidate and deployment: FLASHING.txt.
The new writer is local only; it has not been installed on the TV.

Fast network development on the installed G13
---------------------------------------------
From the repository root, with direct Ethernet configured as host
192.168.77.1/24 and TV 192.168.77.3, run:

  nix develop .#bootloader -c python3 tools/bootloader/update/try-network.py \
    assets/build/leanboot-network/network-example.bin \
    --key assets/private/network-boot/signing.pem --host 192.168.77.3 --reboot

This signs a temporary package, offers it over UDP port 7533 while the exact
G13 restarts through ADB, and waits for acceptance. UART prints LEANBOOT WAIT,
LEANBOOT NET, then NETWORK LEAN from the example. The example returns and the
resident gate continues to the native OS. Acceptance alone is not evidence
that a different payload ran successfully; inspect its UART output.

Replace the image with your flat ARM binary, position independent or linked
at 0x40600000. It receives r0=context and a private 64 KiB AAPCS stack. The
1 MiB limit, entry offset and required return-state contract are documented
in src/bootloader/Unsafe/CONTRACTS.txt. A non-returning payload can take over;
a returning payload must preserve resident code, RAM and CPU state. Repeated
signed versions are allowed for this volatile operation.

The existing third-start MBOOT-input escape deliberately skips Ethernet.
If that start is due, let it reach Home and run the command on the next start.
Without --reboot the tool waits for a manually initiated startup. A UART byte
also ends the receive window. That describes the installed older gate: the
new local candidate offers all starts and can flash signed application images.

The private deployment key is ignored by Git and stays on the host. Retain a
backup if you need to sign future payloads for this installed verifier. Test
keys used by local fixtures are public and are never resident release keys.

Build/check a network image locally with an explicit public key:

  nix develop .#bootloader -c python3 tools/bootloader/build.py
  nix develop .#bootloader -c python3 tools/bootloader/update/build-network.py \
    --public-key assets/private/network-boot/public.raw
  nix develop .#bootloader -c python3 tools/bootloader/update/audit-network.py
  nix develop .#bootloader -c python3 tools/bootloader/update/check-network.py \
    --key assets/private/network-boot/signing.pem
  nix develop .#bootloader -c python3 tools/bootloader/update/check-network-gate.py

These commands do not access the TV. Network artifacts are under
assets/build/leanboot-network/. install-network.py is an exact-state installer
for this experiment: it requires matching partition hashes, audited artifacts
and a successful same-image RAM boot. Its --apply writes the boot_a prefix.
verify-network-tv.py performs three real restarts; it is not a status query.

Local persistent-update model
-----------------------------
The commands below run locally and do not flash the TV.

Enter: nix develop .#bootloader

For real packaging, create/retain a private Ed25519 key outside Git:
  openssl genpkey -algorithm ED25519 -out /private/path/release.pem
  openssl pkey -in /private/path/release.pem -pubout -outform DER -out public.der
The last 32 bytes of that Ed25519 SubjectPublicKeyInfo are the raw public key.
Provide that explicit 32-byte file when building the verifier:
  python3 tools/bootloader/update/build.py --public-key public.raw
The output is a MODEL ELF, not a deployable TV boot image. The public key is
embedded in it; its private key is never consumed by the build.

Package a flat ARM second stage:
  python3 tools/bootloader/update/package.py IMAGE OUTPUT.lbu \
    --key /private/path/release.pem --version 2 --entry 0
The image must be linked for the eventual loader's agreed RAM address. The
manifest entry is an offset, not permission to load or jump to an arbitrary
physical address. No key rotation or arbitrary OS-partition target is supported. Component 2
application images use the separate fixed-bank writer; see FLASHING.txt.

Reproduce the local checks using a PUBLIC TEST KEY (never a release key):
  python3 -c "from pathlib import Path; p=Path('assets/build/leanboot-update/test-key.pub'); p.parent.mkdir(parents=True,exist_ok=True); p.write_bytes(bytes.fromhex('d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a'))"
  python3 tools/bootloader/update/build.py --public-key assets/build/leanboot-update/test-key.pub
  python3 tools/bootloader/update/audit.py
  python3 tools/bootloader/update/check.py
  python3 tools/bootloader/update/check-udp.py
  python3 tools/bootloader/prove.py

For an existing build with your own public key, check.py --key PATH signs the
local fixtures with the matching private key instead. It checks the embedded
public-key hash before running. It does not contact the TV or change the key
in the build. Keep private keys under ignored assets/private/ or outside Git.

The check writes packages and a two-bank disk snapshot under
assets/build/leanboot-update/. HACL* sources are fetched into that directory,
verified against the lock file, and linked without heap allocation or libc.
Source license notices and the upstream LICENSE are preserved there.

A localhost-only UDP server can use a pair of bank0.bin/bank1.bin files:
  python3 tools/bootloader/update/serve-model.py BANK_DIRECTORY --port 7533
  python3 tools/bootloader/update/send.py UPDATE.lbu --host 127.0.0.1
The server binds only loopback. Initialize one bank with an authenticated
release and the other with zeroes; model.bank_image constructs the local disk
layout. The checks' demo-banks already contain versions 1 and 2, so a further
update must have version 3 or higher. The model executes real ARM code, but
its storage failure semantics are intentionally finite and explicit.
