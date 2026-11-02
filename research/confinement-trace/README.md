# G06 helper confinement measurements

`network-g06.sh` is the one-off on-TV probe used for PR17. It requires that
PR's smoke-tested syscall tracer and the private G06 helpers installed under
`/run/tv-network`. It traces association, DHCP bind/renew/release, a scan and
normal termination. It preserves the Ethernet debug route and stores raw logs
only in RAM. It does not flash firmware. Run only from the native G06 root
console; the script checks the exact device serial.

The physical results and complete metadata summaries were posted to
https://github.com/d4hines/dande-tv/pull/17#issuecomment-6565243318.
Private raw logs are under `assets/research/confinement-trace/20261101T224930Z/`.
Raw traces include registers and TEE scalar values and should stay private.
The temporary cold-TEE startup hook is not part of main.
