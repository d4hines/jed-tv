#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
nix build .#wifi-supplicant --out-link assets/build/network/wpa-supplicant
