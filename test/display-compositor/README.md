# Scalar Lean compositor check

`TV.Display` owns letterboxing, nearest-neighbor sampling, background clearing
and overlay placement. Its three explicit `BaseIO` memory effects are generated
from `ZrAbi.DisplayMemory`; the Rust caller supplies live nonoverlapping slices.
The compiler erases the world token and uses only scalar arguments/results.
No initialization, allocator or Lean runtime is linked into this path, so it
can run on the existing display worker without entering the Core/audio executor.

Run the byte-exact oracle and guard/padding/concurrency checks:

```sh
python3 test/display-compositor/check.py
```

The oracle is the prior Rust algorithm, retained only in this test. It covers
2,084 valid cases plus malformed dimensions and overlays. The script also
cross-builds the same executable for ARM32 and boots it as `/init` under the
checked-in demo kernel (`--kernel PATH` overrides it). It also runs the display
unit tests, including every OSD state/failure combination, on ARM32. The native
test is offline and does not open display devices.

The kernel must support `CONFIG_FILE_LOCKING`, as the TV does. An older minimal
surrogate kernel lacks it and cannot run the existing shared-frame lease tests.

The final executable's symbol table must have no `lean_*`, `initialize_*`, or
`*___boxed` symbols. Unreachable compiler-generated wrappers are discarded by
the linker. This checks the final reachable call path, not every unused symbol
in the intermediate object.
