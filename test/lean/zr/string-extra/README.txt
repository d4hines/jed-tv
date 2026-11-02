Run from the repository root:

  python3 test/lean/zr/string-extra/check.py --kernel /path/to/arm/zImage

This test checks the pinned upstream Lean IR ownership signatures, runs the
same Rust oracle against upstream Lean on the host and Lean-owned Zr on ARM32,
and writes evidence beneath assets/build/string-extra/.

Coverage: String.intercalate with empty/shared lists and separator aliasing;
borrowed Int.repr across small integers and signed128 boundaries, including
the minimum value; borrowed UTF-8 get/next over every Unicode scalar and the
terminal NUL. Every borrowed source is reused after the operation. The C file
is a test-only ABI adapter; no new C is part of the TV runtime.
