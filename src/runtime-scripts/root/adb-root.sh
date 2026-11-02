#!/system/bin/sh
umask 077
exec > /dev/tv-adb-root.log 2>&1
setprop sys.tvroot.ready 0
"$1" --sqlite "BEGIN; INSERT OR REPLACE INTO settings (key,value) VALUES ('root_access',2); INSERT OR REPLACE INTO policies (uid,policy,until,logging,notification) VALUES (2000,2,0,0,0); COMMIT;"
policy_status=$?
printf 'policy_status=%s\n' "$policy_status"
if [ "$policy_status" -eq 0 ]; then
    id
    cat /proc/self/attr/current
    getenforce
    /vendor/bin/kexec --version
    printf 'kexec_loaded='
    cat /sys/kernel/kexec_loaded
    printf 'kexec_load_disabled='
    cat /proc/sys/kernel/kexec_load_disabled
    if cat /proc/iomem && [ "$(id -u)" = 0 ] &&
       [ "$(cat /proc/sys/kernel/kexec_load_disabled)" = 0 ]; then
        setprop sys.tvroot.ready 1
    fi
fi
chmod 0644 /dev/tv-adb-root.log
