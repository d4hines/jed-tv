// Host-side sin/cos for JsbExact's jx_* externs (lean/JsbExact/Scalar.lean), linked only into the `jsbdiff`
// test executable (lean/lakefile.lean). Upstream's jxmath.c uses Darwin's combined __sincos_stret for the
// jx_sincos_* pair, which can differ from sin/cos in the last bit; for the differential test it does not
// matter which, because both models call these same four functions. The TV build writes its own copy of this
// file in demo/flight.nix.
#include <math.h>
double jx_sin(double x) { return sin(x); }
double jx_cos(double x) { return cos(x); }
double jx_sincos_sin(double x) { return sin(x); }
double jx_sincos_cos(double x) { return cos(x); }
