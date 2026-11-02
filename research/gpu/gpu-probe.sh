#!/bin/sh
# GPU probe in the native PID 1 session on the original G06: load the stock
# Mali kernel module and run /tmp/egl-probe (research/gpu/egl-probe.rs) under the
# stock Android linker with the system and vendor libraries mounted read-only.
# Nothing is written to the TV's storage; nothing persists across a reboot.
set -eu
grep -q 'androidboot.serialno=4C0490290FA05A0C0' /proc/cmdline
[ "$(id -u)" = 0 ]
mountpoint -q /system || /tmp/mount-stock-runtime.sh
# mali_kbase alone created /dev/mali0 in the earlier native session (tools/pid1/README.txt).
if ! grep -q '^mali_kbase ' /proc/modules; then
    insmod /vendor/lib/modules/mali_kbase.ko
fi
test -c /dev/mali0
mkdir -p /tmp/gpu
export ANDROID_ROOT=/system ANDROID_DATA=/data
# Android's libEGL picks /vendor/lib/egl/libGLES_mali.so; without the property
# service it falls back to scanning that directory.
export LD_LIBRARY_PATH=/system/lib:/vendor/lib:/vendor/lib/egl
chmod +x /tmp/egl-probe
timeout 120 /tmp/egl-probe "${1:-2000}" "${2:-60}" 2>&1 | tee /tmp/gpu/probe.log
grep -q GPU_PROBE_PASS /tmp/gpu/probe.log
