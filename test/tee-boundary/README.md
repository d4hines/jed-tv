# TEE boundary parsers

Run `python3 test/tee-boundary/check.py --arm` from the repository root.
This compiles `TV.Hardware.TeeBoundary` and its six proofs, then links the
actual `tee_boundary.rs` adapter into host and ARM32 tests without the Lean
runtime. The ARM tests run under the existing QEMU kernel, not on the TV.

The tests compare the previous Rust implementation on kernel-format ASCII:
15,200 readiness records and 1,350 device identities, including signs, hex,
overflow, delimiters, incomplete records and extra syscall arguments. They
also test device-number matching, 64 KiB inputs and eight concurrent callers.

The parser deliberately accepts only ASCII kernel records. Non-ASCII input
fails closed instead of accepting Rust's broader Unicode whitespace syntax.
No device permissions, readiness IOCTL number or Linux operation changes.
The proofs describe parser acceptance and rejection, not the trustworthiness
of `/proc`, `/sys`, the native byte transfer, or the vendor TEE implementation.
