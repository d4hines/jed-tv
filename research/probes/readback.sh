#!/vendor/bin/sh
# Writes only backup staging files in /tmp/tcl-readback (RAM).
set -eu
set -o pipefail
export PATH=/vendor/bin:/system/bin
stage=/tmp/tcl-readback
umask 077
case "${1:-}" in
  inspect)
    {
      id
      id -Z
      printf 'CMDLINE '
      cat /proc/cmdline || true
      cat /proc/partitions
      df -k /tmp
      for node in /dev/block/by-name/boot /dev/block/by-name/recovery /dev/block/by-name/MBOOT /dev/block/mmcblk0 /dev/block/mmcblk0boot0 /dev/block/mmcblk0boot1; do
        printf 'READ_TEST %s\n' "$node"
        if dd if="$node" of=/dev/null bs=512 count=1; then echo READ_OK; else echo READ_DENIED; fi
      done
      for sys in /sys/class/block/mmcblk0*; do
        printf 'BLOCK %s SIZE ' "${sys##*/}"
        cat "$sys/size" || true
        if [ -f "$sys/start" ]; then printf 'START '; cat "$sys/start" || true; fi
      done
      command -v nc || true
      command -v sha256sum || true
    } > "$stage/capabilities.txt" 2>&1
    chmod 0644 "$stage/capabilities.txt"
    ;;
  read|hash)
    [ "$#" -eq 5 ] || exit 2
    operation=$1
    source=$2
    skip=$3
    count=$4
    nonce=$5
    case "$source" in mmcblk0|mmcblk0boot0|mmcblk0boot1) ;; *) exit 2 ;; esac
    case "$skip:$count" in *[!0-9:]*|:*|*:) exit 2 ;; esac
    case "$nonce" in ''|*[!a-zA-Z0-9]*) exit 2 ;; esac
    [ "$count" -gt 0 ] || exit 2
    sectors=$(cat "/sys/class/block/$source/size")
    [ $(((skip + count) * 2048)) -le "$sectors" ] || exit 2
    if [ "$operation" = read ]; then [ "$count" -le 64 ] || exit 2; fi
    rm -f "$stage/status.txt" "$stage/chunk.bin" "$stage/digest.txt"
    if [ "$operation" = read ]; then
      if dd if="/dev/block/$source" of="$stage/chunk.bin" bs=1048576 skip="$skip" count="$count" 2> "$stage/dd.log"; then
        sha256sum "$stage/chunk.bin" > "$stage/digest.txt"
        chmod 0644 "$stage/chunk.bin" "$stage/digest.txt" "$stage/dd.log"
        printf '%s OK\n' "$nonce" > "$stage/status.txt"
      else
        printf '%s ERROR\n' "$nonce" > "$stage/status.txt"
      fi
    else
      if dd if="/dev/block/$source" bs=1048576 skip="$skip" count="$count" 2> "$stage/dd.log" | sha256sum > "$stage/digest.txt"; then
        chmod 0644 "$stage/digest.txt" "$stage/dd.log"
        printf '%s OK\n' "$nonce" > "$stage/status.txt"
      else
        printf '%s ERROR\n' "$nonce" > "$stage/status.txt"
      fi
    fi
    chmod 0644 "$stage/status.txt"
    ;;
  cleanup)
    rm -f "$stage/chunk.bin" "$stage/digest.txt" "$stage/dd.log" "$stage/status.txt"
    ;;
  *) exit 2 ;;
esac
