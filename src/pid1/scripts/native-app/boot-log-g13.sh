# Sourced by G13 startup scripts. Use kernel uptime: the RTC can be wrong.
# Diagnostics must never turn a successful startup into a failed one.
dande_boot_mark() {
    local dande_uptime dande_idle dande_message
    read -r dande_uptime dande_idle < /proc/uptime || dande_uptime=unknown
    dande_message="DANDE_BOOT +${dande_uptime}s $*"
    { printf '%s\n' "$dande_message" >> /dev/dande-boot.log; } 2>/dev/null || :
    { printf '<6>%s\n' "$dande_message" > /dev/kmsg; } 2>/dev/null || :
    # printk is deliberately quiet on this firmware. Write the console
    # directly instead of raising the global kernel console log level.
    { printf '\n%s\n' "$dande_message" > /dev/console; } 2>/dev/null || :
}
