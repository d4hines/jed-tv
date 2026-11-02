#!/system/bin/sh
# Sourced before the supervisor starts. Lean authenticated the entire selected
# image before publishing this DT property. Never choose using disk headers.
dande_mount_apps() {
    local property=/sys/firmware/devicetree/base/chosen/dande,apps-bank
    local bank device block offset loop image=/dev/dande-app-image
    [ -e "$property" ] || return 0 # Installed pre-update bootloader.
    bank=$(tr -d '\000' < "$property")
    case "$bank" in
        -) return 0 ;; # First provisioning: retain the installed app tree.
        0|1) ;;
        *) return 1 ;;
    esac
    [ "$(getprop ro.serialno)" = 18ACC22D1C47 ] || return 1
    [ "$(getprop ro.build.version.incremental)" = AU13 ] || return 1
    device=$(readlink -f /dev/block/by-name/upgrade) || return 1
    block=${device##*/}
    [ "$(cat /sys/class/block/"$block"/start)" = 3180544 ] || return 1
    [ "$(cat /sys/class/block/"$block"/size)" = 102400 ] || return 1
    offset=$((1048576 + bank * 16777216 + 512))
    mkdir -p "$image" "$base/root/bin" "$base/root/share" || return 1
    # Android 14 toybox: -s prints the chosen loop device; -S bounds it.
    loop=$(losetup -f -s -r -o "$offset" -S 16776704 "$device") || return 1
    if ! mount -t erofs -o ro,nosuid,nodev,context=u:object_r:shell_data_file:s0 "$loop" "$image"; then
        losetup -d "$loop"
        return 1
    fi
    for executable in tvinit ytplay slides doomtv tvosd; do
        [ -x "$image/bin/$executable" ] || return 1
    done
    mount -o bind "$image/bin" "$base/root/bin" || return 1
    mount -o bind "$image/share" "$base/root/share" || return 1
    dande_boot_mark "APPS: authenticated image bank=$bank mounted read-only"
}
dande_mount_apps
