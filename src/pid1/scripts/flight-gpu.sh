#!/bin/sh
# Start the city flight demo's GPU renderer in the native PID 1 session on the
# original G06: stock runtime mounted, the stock Mali module loaded, then
# /tmp/flightgpu (src/flightgpu) under the stock Android linker. It announces
# itself in /tmp/flight-gpu.ready; City flight started from the home screen
# then publishes its state instead of drawing, the GPU draws the picture into
# a shared frame, and City flight presents it through the display service.
# Nothing is written to the TV's storage; nothing persists across a reboot.
set -eu
grep -q 'androidboot.serialno=4C0490290FA05A0C0' /proc/cmdline
[ "$(id -u)" = 0 ]
mountpoint -q /system || /tmp/mount-stock-runtime.sh
if ! grep -q '^mali_kbase ' /proc/modules; then
    insmod /vendor/lib/modules/mali_kbase.ko
fi
test -c /dev/mali0
mkdir -p /tmp/gpu /run/flight-gpu
chmod 755 /run/flight-gpu
# One renderer at a time: a second one would be a second writer of the frame.
rm -f /tmp/flight-gpu.ready
if pidof flightgpu >/dev/null; then kill "$(pidof flightgpu)"; sleep 1; fi
pidof flightgpu >/dev/null && { echo 'a previous flightgpu is still running' >&2; exit 1; }
export ANDROID_ROOT=/system ANDROID_DATA=/data
export LD_LIBRARY_PATH=/system/lib:/vendor/lib:/vendor/lib/egl
chmod +x /tmp/flightgpu
# The confined app (uid 10006) must be able to map the record read-write.
nohup /tmp/flightgpu /run/flight-gpu/state > /tmp/gpu/flightgpu.log 2>&1 &
for _ in $(seq 1 300); do
    [ -f /tmp/flight-gpu.ready ] && break
    sleep 1
done
cat /tmp/gpu/flightgpu.log
test -f /tmp/flight-gpu.ready
chmod 666 /run/flight-gpu/state /run/flight-gpu/state.frame
echo "GPU renderer ready; start City flight from the home screen"
