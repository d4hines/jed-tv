#!/system/bin/sh
# Run by root through unshare -m. All mounts are private to this launcher.
set -eu
umask 022
base=/data/local/tmp/dande-g13
root=$base/root
[ "$(id -u)" = 0 ]
[ "$(getprop ro.serialno)" = 18ACC22D1C47 ]
[ "$(getprop ro.product.device)" = G13 ]
mount -o rprivate none /
# The app launcher changes propagation on / in each child namespace. Make
# this chroot root a mount point so that operation also works under Android.
mount -o bind "$root" "$root"
# Keep parental controls outside the transient app root, available only to PID1.
settings=/metadata/dande-tv/settings
mkdir -p "$settings" "$root/settings/dande-tv"
chmod 700 "$settings"
mount -o bind "$settings" "$root/settings/dande-tv"
mount -t tmpfs -o mode=755 tmpfs "$root/dev"
mkdir -p "$root/dev/input"
for node in null zero random urandom; do
    major=$(stat -c %t "/dev/$node")
    minor=$(stat -c %T "/dev/$node")
    mknod -m 666 "$root/dev/$node" c "$((0x$major))" "$((0x$minor))"
done
for node in /dev/input/event*; do
    major=$(stat -c %t "$node")
    minor=$(stat -c %T "$node")
    mknod -m 600 "$root/dev/input/${node##*/}" c "$((0x$major))" "$((0x$minor))"
done
mount -t proc -o ro,nosuid,nodev,noexec proc "$root/proc"
export TV_FB_PATH=/tmp/app.frames TV_AUDIO_ADDR=127.0.0.1:8877
export TV_HTTP_PROXY=127.0.0.1:8899 TV_INPUT_EXCLUSIVE=1
exec chroot "$root" /bin/tvinit ytplay.server=http://127.0.0.1:8090
