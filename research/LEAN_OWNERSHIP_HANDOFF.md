# Scalar Lean ownership guard

Isolated branch `codex/lean-ownership`, based on `52cade9`.
No hardware, root-checkout edit, merge or push was performed.

`TV.Hardware.Ownership` owns exact G06 serial-token recognition, unsigned-u32
/proc filename syntax, seven known media-owner comm names, first `State:` line
recognition and the T/t stopped decision. Its scalar entry points contain no
boxed Lean values or initializer calls. Rust supplies raw bytes and PID, keeps
filesystem failures/iteration, and formats existing recognized-owner errors.
Driver startup and display/audio loops are unchanged; Core is untouched.

All kernel input follows an explicit ASCII grammar. Non-ASCII cmdline/status
records fail recognition; malformed comm records return a distinct invalid
sentinel and deny startup instead of being mistaken for unrelated owners.
Non-numeric /proc names still skip normally. Numeric syntax retains leading
plus and arbitrarily many leading zeroes, with strict u32 overflow rejection.

Kernel-checked Lean theorems cover PID1 identity, stopped-state necessity for
recognized owners, invalid-owner refusal and absorbing invalid parser states.
The actual Rust/compiled-Lean adapter is tested on the host and ARM32 without
linking or initializing the Lean runtime, including a worker-thread call.
Host and ARM32 pass 40,245 cases with matching transcript
`eda3109d917a6367`; the full native ARM32 PID1 build also passes.
See `test/ownership/README.md` and the worktree's
`assets/build/ownership-check/result.json` and `assets/build/ownership-pid1.log`
for evidence.

Integration touches only direct_services.rs::verify_ownership, its new
ownership.rs adapter, the Lean module, and the build module list. Reconcile
`Hardware/Ownership` with concurrent module-list additions; no Core or broker
interface changes are required.
