#!/system/bin/sh
# Sourced before the supervisor starts. Lean authenticated the entire selected
# image before publishing this DT property. Never choose using disk headers.
dande_mount_apps() {
    local property=/sys/firmware/devicetree/base/chosen/dande,apps-bank
    local bank device block offset mapped image=/dev/dande-app-image
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
    [ "$(blockdev --getro "$device")" = 1 ] || return 1
    offset=$((2049 + bank * 32768))
    mkdir -p "$image" "$base/root/bin" "$base/root/share" || return 1
    # A read-only dm slice works on this kernel; newly configured loop devices
    # fail reads in the native environment. Labels are embedded in EROFS.
    dmctl create dande-apps -ro linear 0 32767 "$device" "$offset" || return 1
    mapped=$(dmctl getpath dande-apps) || return 1
    if ! mount -t erofs -o ro,nosuid,nodev "$mapped" "$image"; then
        dmctl delete dande-apps
        return 1
    fi
    for executable in tvinit ytplay slides doomtv tvosd; do
        [ -x "$image/bin/$executable" ] || return 1
    done
    mount -o bind "$image/bin" "$base/root/bin" || return 1
    mount -o bind "$image/share" "$base/root/share" || return 1
    dande_boot_mark "APPS: authenticated image bank=$bank mounted read-only"
}
