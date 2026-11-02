# Retained network helpers

On the G06, `wpa_supplicant` and BusyBox `udhcpc` are retained privileged
helpers alongside `tee-supplicant`. With Wi-Fi active, the steady-state design
is PID 1, the selected app, and these three helpers. It is not a two-process
system. Settings operations also run transient `wpa_cli`, shell and network
configuration commands.

`install.sh` puts the network executables, lease hook, configuration, PID files,
logs and WPA control/reply sockets below `/run/tv-network`, owned by root with
mode 0700. Executables and hooks have mode 0700; configuration and daemon state
are created with umask 077. The installer rejects symlink or writable roots,
symlink source files, non-root source ownership and source hard links.
Existing saved configurations are normalized to the private WPA socket path.
The persistent settings directory remains governed by the settings adapter.

Apps use PID 1's Settings effects; they receive no WPA control descriptor or
credentials. Linux UID permissions prevent app UIDs from reading or changing
the private state or reaching the Unix control socket. This assumes the
existing app UID/capability confinement. Other privileged code and the
development root shell/HTTP channel are outside that isolation claim.

This boundary does not authenticate DHCP servers or replace the WPA/DHCP
implementations. It changes no Ethernet configuration. When adopting it in a
running session, the coordinator must stop the previously staged network
helpers before starting these private-path versions; a fresh native boot has
no previous helper instance.

The G13 script is a separate Android path: its vendor supplicant drops to UID
1010 and uses a root/wifi directory. The G06 root-only qualification does not
establish the same property for that platform.

`test/network/check-private.py` runs the actual installer in ARM32 QEMU and
checks rejection of hostile staging plus denied access for app UIDs 10001,
10006 and 10008. It performs no Wi-Fi association or real network changes.
