G06 display tools
=================

The direct renderer and MI hardware tests are documented in
research/DISPLAY_BACKEND_HANDOFF.md and research/DISPLAY_ORDERING.md.
Do not repeat create/destroy trials: the stock WindowDestroy path fails.
Keep direct scanout resources alive and hide/resume the persistent renderer.

Backlight sleep helper
---------------------
Build:
  sh tools/toolchain/build-static-rust.sh research/probes/display/backlight.rs assets/build/display-agent/backlight

The helper accepts --hold-off STATE_DIRECTORY. It saves brightness/power if
not already recorded, then keeps brightness0 and bl_power4 without stopping
Android or the root shell. SIGUSR1 restores saved settings and exits; SIGTERM
exits while leaving the panel dark. Start only one keeper per TV.

Current Android deployment is /data/local/tmp/tv-backlight; state and PID are
in /data/local/tmp/tv-backlight-state. Restore after the owner wakes:
  adb -s 192.168.77.2:5555 shell '/debug_ramdisk/su -c "kill -USR1 $(cat /data/local/tmp/tv-backlight-state/pid)"'

Recheck TV identity and keeper PID first. The owner physically confirmed a
completely dark screen with this helper. Its saved normal setting was
brightness1 and bl_power0. Do not re-enable the panel while the owner sleeps.

Restoration limitation found in live testing: bl_power4 cuts the panel VCC
rail as well as its light. Restoring brightness/power sysfs values reported
1/0 but did not restore a visible image; normal Android reboot did. Do not
use this helper while iterating on graphics if a quick screen restore is
needed. It is currently a sleep blackout tool, not a validated panel power
manager. The eventual owned power path must include panel reinitialization.
