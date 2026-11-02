// Translated from FFmpeg 9.0.1 libavutil/mathematics.c
// (preprocessed for aarch64-unknown-linux-musl) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// Copyright (c) 2005-2012 Michael Niedermayer <michaelni@gmx.at>
//
// This file is part of FFmpeg.
//
// FFmpeg is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// FFmpeg is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with FFmpeg; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
//

extern "C" {
    fn abort() -> !;
    fn llabs(_: ::core::ffi::c_longlong) -> ::core::ffi::c_longlong;
    fn av_mul_q(b: AVRational, c: AVRational) -> AVRational;
    fn exp(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn sqrt(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
}
pub type int64_t = ::core::ffi::c_long;
pub type uint64_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: ::core::ffi::c_int,
    pub den: ::core::ffi::c_int,
}
pub type AVRounding = ::core::ffi::c_uint;
pub const AV_ROUND_PASS_MINMAX: AVRounding = 8192;
pub const AV_ROUND_NEAR_INF: AVRounding = 5;
pub const AV_ROUND_UP: AVRounding = 3;
pub const AV_ROUND_DOWN: AVRounding = 2;
pub const AV_ROUND_INF: AVRounding = 1;
pub const AV_ROUND_ZERO: AVRounding = 0;
#[no_mangle]
pub unsafe extern "C" fn av_gcd(mut a: int64_t, mut b: int64_t) -> int64_t {
    let mut za: ::core::ffi::c_int = 0;
    let mut zb: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut u: int64_t = 0;
    let mut v: int64_t = 0;
    if a == 0 as int64_t {
        return b;
    }
    if b == 0 as int64_t {
        return a;
    }
    za = (a as ::core::ffi::c_ulonglong).trailing_zeros() as i32;
    zb = (b as ::core::ffi::c_ulonglong).trailing_zeros() as i32;
    k = if za > zb { zb } else { za };
    u = llabs((a >> za) as ::core::ffi::c_longlong) as int64_t;
    v = llabs((b >> zb) as ::core::ffi::c_longlong) as int64_t;
    while u != v {
        if u > v {
            let mut SWAP_tmp: int64_t = u;
            u = v;
            v = SWAP_tmp;
        }
        v -= u;
        v >>= (v as ::core::ffi::c_ulonglong).trailing_zeros() as i32;
    }
    return ((u as uint64_t) << k) as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn av_rescale_rnd(
    mut a: int64_t,
    mut b: int64_t,
    mut c: int64_t,
    mut rnd: AVRounding,
) -> int64_t {
    let mut r: int64_t = 0 as int64_t;
    if c <= 0 as int64_t || b < 0 as int64_t
        || !(rnd as ::core::ffi::c_uint
            & !(AV_ROUND_PASS_MINMAX as ::core::ffi::c_int) as ::core::ffi::c_uint
            <= 5 as ::core::ffi::c_uint
            && rnd as ::core::ffi::c_uint
                & !(AV_ROUND_PASS_MINMAX as ::core::ffi::c_int) as ::core::ffi::c_uint
                != 4 as ::core::ffi::c_uint)
    {
        return -(1 as ::core::ffi::c_int) as int64_t - 0x7fffffffffffffff as int64_t;
    }
    if rnd as ::core::ffi::c_uint
        & AV_ROUND_PASS_MINMAX as ::core::ffi::c_int as ::core::ffi::c_uint != 0
    {
        if a
            == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long
                - 0x7fffffffffffffff as ::core::ffi::c_long
            || a == 0x7fffffffffffffff as ::core::ffi::c_long
        {
            return a;
        }
        rnd = ::core::mem::transmute::<
            ::core::ffi::c_uint,
            AVRounding,
        >(
            (rnd as ::core::ffi::c_uint)
                .wrapping_sub(
                    AV_ROUND_PASS_MINMAX as ::core::ffi::c_int as ::core::ffi::c_uint,
                ),
        ) as AVRounding;
    }
    if a < 0 as int64_t {
        return (av_rescale_rnd(
            -if a > -(0x7fffffffffffffff as ::core::ffi::c_long) {
                a
            } else {
                -(0x7fffffffffffffff as int64_t)
            },
            b,
            c,
            (rnd as ::core::ffi::c_uint
                ^ rnd as ::core::ffi::c_uint >> 1 as ::core::ffi::c_int
                    & 1 as ::core::ffi::c_uint) as AVRounding,
        ) as uint64_t)
            .wrapping_neg() as int64_t;
    }
    if rnd as ::core::ffi::c_uint
        == AV_ROUND_NEAR_INF as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        r = c / 2 as int64_t;
    } else if rnd as ::core::ffi::c_uint & 1 as ::core::ffi::c_uint != 0 {
        r = c - 1 as int64_t;
    }
    if b <= 2147483647 as int64_t && c <= 2147483647 as int64_t {
        if a <= 2147483647 as int64_t {
            return (a * b + r) / c
        } else {
            let mut ad: int64_t = a / c;
            let mut a2: int64_t = (a % c * b + r) / c;
            if ad >= 0x7fffffff as int64_t && b != 0
                && ad > (0x7fffffffffffffff as int64_t - a2) / b
            {
                return -(1 as ::core::ffi::c_int) as int64_t
                    - 0x7fffffffffffffff as int64_t;
            }
            return ad * b + a2;
        }
    } else {
        let mut a0: uint64_t = (a & 0xffffffff as ::core::ffi::c_uint as int64_t)
            as uint64_t;
        let mut a1: uint64_t = (a >> 32 as ::core::ffi::c_int) as uint64_t;
        let mut b0: uint64_t = (b & 0xffffffff as ::core::ffi::c_uint as int64_t)
            as uint64_t;
        let mut b1: uint64_t = (b >> 32 as ::core::ffi::c_int) as uint64_t;
        let mut t1: uint64_t = a0.wrapping_mul(b1).wrapping_add(a1.wrapping_mul(b0));
        let mut t1a: uint64_t = t1 << 32 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0;
        a0 = a0.wrapping_mul(b0).wrapping_add(t1a);
        a1 = a1
            .wrapping_mul(b1)
            .wrapping_add(t1 >> 32 as ::core::ffi::c_int)
            .wrapping_add((a0 < t1a) as ::core::ffi::c_int as uint64_t);
        a0 = a0.wrapping_add(r as uint64_t);
        a1 = a1.wrapping_add((a0 < r as uint64_t) as ::core::ffi::c_int as uint64_t);
        i = 63 as ::core::ffi::c_int;
        while i >= 0 as ::core::ffi::c_int {
            a1 = a1.wrapping_add(a1.wrapping_add(a0 >> i & 1 as uint64_t));
            t1 = t1.wrapping_add(t1);
            if c as uint64_t <= a1 {
                a1 = a1.wrapping_sub(c as uint64_t);
                t1 = t1.wrapping_add(1);
            }
            i -= 1;
        }
        if t1 > 0x7fffffffffffffff as ::core::ffi::c_long as uint64_t {
            return -(1 as ::core::ffi::c_int) as int64_t - 0x7fffffffffffffff as int64_t;
        }
        return t1 as int64_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_rescale(
    mut a: int64_t,
    mut b: int64_t,
    mut c: int64_t,
) -> int64_t {
    return av_rescale_rnd(a, b, c, AV_ROUND_NEAR_INF);
}
#[no_mangle]
pub unsafe extern "C" fn av_rescale_q_rnd(
    mut a: int64_t,
    mut bq: AVRational,
    mut cq: AVRational,
    mut rnd: AVRounding,
) -> int64_t {
    let mut b: int64_t = bq.num as int64_t * cq.den as int64_t;
    let mut c: int64_t = cq.num as int64_t * bq.den as int64_t;
    return av_rescale_rnd(a, b, c, rnd);
}
#[no_mangle]
pub unsafe extern "C" fn av_rescale_q(
    mut a: int64_t,
    mut bq: AVRational,
    mut cq: AVRational,
) -> int64_t {
    return av_rescale_q_rnd(a, bq, cq, AV_ROUND_NEAR_INF);
}
#[no_mangle]
pub unsafe extern "C" fn av_compare_ts(
    mut ts_a: int64_t,
    mut tb_a: AVRational,
    mut ts_b: int64_t,
    mut tb_b: AVRational,
) -> ::core::ffi::c_int {
    let mut a: int64_t = tb_a.num as int64_t * tb_b.den as int64_t;
    let mut b: int64_t = tb_b.num as int64_t * tb_a.den as int64_t;
    if (if ts_a <= 0 as int64_t {
        (ts_a as uint64_t).wrapping_neg()
    } else {
        ts_a as uint64_t
    }) | a as uint64_t
        | (if ts_b <= 0 as int64_t {
            (ts_b as uint64_t).wrapping_neg()
        } else {
            ts_b as uint64_t
        }) | b as uint64_t <= 2147483647 as uint64_t
    {
        return (ts_a * a > ts_b * b) as ::core::ffi::c_int
            - (ts_a * a < ts_b * b) as ::core::ffi::c_int;
    }
    if av_rescale_rnd(ts_a, a, b, AV_ROUND_DOWN) < ts_b {
        return -(1 as ::core::ffi::c_int);
    }
    if av_rescale_rnd(ts_b, b, a, AV_ROUND_DOWN) < ts_a {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_compare_mod(
    mut a: uint64_t,
    mut b: uint64_t,
    mut mod_0: uint64_t,
) -> int64_t {
    let mut c: int64_t = (a.wrapping_sub(b) & mod_0.wrapping_sub(1 as uint64_t))
        as int64_t;
    if c as uint64_t > mod_0 >> 1 as ::core::ffi::c_int {
        c = (c as uint64_t).wrapping_sub(mod_0) as int64_t as int64_t;
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn av_rescale_delta(
    mut in_tb: AVRational,
    mut in_ts: int64_t,
    mut fs_tb: AVRational,
    mut duration: ::core::ffi::c_int,
    mut last: *mut int64_t,
    mut out_tb: AVRational,
) -> int64_t {
    let mut a: int64_t = 0;
    let mut b: int64_t = 0;
    let mut this: int64_t = 0;
    if !(in_ts != 0x8000000000000000 as ::core::ffi::c_ulong as int64_t) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"in_ts != ((int64_t)0x8000000000000000UL)\0" as *const u8
                as *const ::core::ffi::c_char,
            b"src/libavutil/mathematics.c\0" as *const u8 as *const ::core::ffi::c_char,
            171 as ::core::ffi::c_int,
        );
        abort();
    }
    if !(duration >= 0 as ::core::ffi::c_int) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"duration >= 0\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavutil/mathematics.c\0" as *const u8 as *const ::core::ffi::c_char,
            172 as ::core::ffi::c_int,
        );
        abort();
    }
    if !(*last == 0x8000000000000000 as ::core::ffi::c_ulong as int64_t || duration == 0
        || in_tb.num as int64_t * out_tb.den as int64_t
            <= out_tb.num as int64_t * in_tb.den as int64_t)
    {
        a = av_rescale_q_rnd(
            2 as int64_t * in_ts - 1 as int64_t,
            in_tb,
            fs_tb,
            AV_ROUND_DOWN,
        ) >> 1 as ::core::ffi::c_int;
        b = av_rescale_q_rnd(
            2 as int64_t * in_ts + 1 as int64_t,
            in_tb,
            fs_tb,
            AV_ROUND_UP,
        ) + 1 as int64_t >> 1 as ::core::ffi::c_int;
        if !(*last < 2 as int64_t * a - b || *last > 2 as int64_t * b - a) {
            this = av_clip64_c(*last, a, b);
            *last = this + duration as int64_t;
            return av_rescale_q(this, fs_tb, out_tb);
        }
    }
    *last = av_rescale_q(in_ts, in_tb, fs_tb) + duration as int64_t;
    return av_rescale_q(in_ts, in_tb, out_tb);
}
#[no_mangle]
pub unsafe extern "C" fn av_add_stable(
    mut ts_tb: AVRational,
    mut ts: int64_t,
    mut inc_tb: AVRational,
    mut inc: int64_t,
) -> int64_t {
    let mut m: int64_t = 0;
    let mut d: int64_t = 0;
    if inc != 1 as int64_t {
        inc_tb = av_mul_q(
            inc_tb,
            AVRational {
                num: inc as ::core::ffi::c_int,
                den: 1 as ::core::ffi::c_int,
            },
        );
    }
    m = inc_tb.num as int64_t * ts_tb.den as int64_t;
    d = inc_tb.den as int64_t * ts_tb.num as int64_t;
    if m % d == 0 as int64_t && ts <= 0x7fffffffffffffff as int64_t - m / d {
        return ts + m / d;
    }
    if m < d {
        return ts;
    }
    let mut old: int64_t = av_rescale_q(ts, ts_tb, inc_tb);
    let mut old_ts: int64_t = av_rescale_q(old, inc_tb, ts_tb);
    if old == 0x7fffffffffffffff as ::core::ffi::c_long
        || old == 0x8000000000000000 as ::core::ffi::c_ulong as int64_t
        || old_ts == 0x8000000000000000 as ::core::ffi::c_ulong as int64_t
    {
        return ts;
    }
    return av_sat_add64_c(av_rescale_q(old + 1 as int64_t, inc_tb, ts_tb), ts - old_ts);
}
#[inline(always)]
unsafe extern "C" fn av_clip64_c(
    mut a: int64_t,
    mut amin: int64_t,
    mut amax: int64_t,
) -> int64_t {
    if a < amin { return amin } else if a > amax { return amax } else { return a };
}
#[inline]
unsafe extern "C" fn eval_poly(
    mut coeff: *const ::core::ffi::c_double,
    mut size: ::core::ffi::c_int,
    mut x: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut sum: ::core::ffi::c_double = *coeff
        .offset((size - 1 as ::core::ffi::c_int) as isize);
    let mut i: ::core::ffi::c_int = 0;
    i = size - 2 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        sum *= x;
        sum += *coeff.offset(i as isize);
        i -= 1;
    }
    return sum;
}
#[no_mangle]
pub unsafe extern "C" fn av_bessel_i0(
    mut x: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    static mut p1: [::core::ffi::c_double; 15] = [
        -2.2335582639474375249e+15f64,
        -5.5050369673018427753e+14f64,
        -3.2940087627407749166e+13f64,
        -8.4925101247114157499e+11f64,
        -1.1912746104985237192e+10f64,
        -1.0313066708737980747e+08f64,
        -5.9545626019847898221e+05f64,
        -2.4125195876041896775e+03f64,
        -7.0935347449210549190e+00f64,
        -1.5453977791786851041e-02f64,
        -2.5172644670688975051e-05f64,
        -3.0517226450451067446e-08f64,
        -2.6843448573468483278e-11f64,
        -1.5982226675653184646e-14f64,
        -5.2487866627945699800e-18f64,
    ];
    static mut q1: [::core::ffi::c_double; 6] = [
        -2.2335582639474375245e+15f64,
        7.8858692566751002988e+12f64,
        -1.2207067397808979846e+10f64,
        1.0377081058062166144e+07f64,
        -4.8527560179962773045e+03f64,
        1.0f64,
    ];
    static mut p2: [::core::ffi::c_double; 7] = [
        -2.2210262233306573296e-04f64,
        1.3067392038106924055e-02f64,
        -4.4700805721174453923e-01f64,
        5.5674518371240761397e+00f64,
        -2.3517945679239481621e+01f64,
        3.1611322818701131207e+01f64,
        -9.6090021968656180000e+00f64,
    ];
    static mut q2: [::core::ffi::c_double; 8] = [
        -5.5194330231005480228e-04f64,
        3.2547697594819615062e-02f64,
        -1.1151759188741312645e+00f64,
        1.3982595353892851542e+01f64,
        -6.0228002066743340583e+01f64,
        8.5539563258012929600e+01f64,
        -3.1446690275135491500e+01f64,
        1.0f64,
    ];
    let mut y: ::core::ffi::c_double = 0.;
    let mut r: ::core::ffi::c_double = 0.;
    let mut factor: ::core::ffi::c_double = 0.;
    if x == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        return 1.0f64;
    }
    x = fabs(x);
    if x <= 15 as ::core::ffi::c_int as ::core::ffi::c_double {
        y = x * x;
        return eval_poly(
            &raw const p1 as *const ::core::ffi::c_double,
            (::core::mem::size_of::<[::core::ffi::c_double; 15]>() as usize)
                .wrapping_div(::core::mem::size_of::<::core::ffi::c_double>() as usize)
                as ::core::ffi::c_int,
            y,
        )
            / eval_poly(
                &raw const q1 as *const ::core::ffi::c_double,
                (::core::mem::size_of::<[::core::ffi::c_double; 6]>() as usize)
                    .wrapping_div(
                        ::core::mem::size_of::<::core::ffi::c_double>() as usize,
                    ) as ::core::ffi::c_int,
                y,
            );
    } else {
        y = 1 as ::core::ffi::c_int as ::core::ffi::c_double / x
            - 1.0f64 / 15 as ::core::ffi::c_int as ::core::ffi::c_double;
        r = eval_poly(
            &raw const p2 as *const ::core::ffi::c_double,
            (::core::mem::size_of::<[::core::ffi::c_double; 7]>() as usize)
                .wrapping_div(::core::mem::size_of::<::core::ffi::c_double>() as usize)
                as ::core::ffi::c_int,
            y,
        )
            / eval_poly(
                &raw const q2 as *const ::core::ffi::c_double,
                (::core::mem::size_of::<[::core::ffi::c_double; 8]>() as usize)
                    .wrapping_div(
                        ::core::mem::size_of::<::core::ffi::c_double>() as usize,
                    ) as ::core::ffi::c_int,
                y,
            );
        factor = exp(x) / sqrt(x);
        return factor * r;
    };
}
#[inline(always)]
unsafe extern "C" fn av_sat_add64_c(mut a: int64_t, mut b: int64_t) -> int64_t {
    let mut tmp: int64_t = 0;
    let (fresh0, fresh1) = a.overflowing_add(b);
    *&raw mut tmp = fresh0;
    return if !fresh1 {
        tmp
    } else if tmp < 0 as int64_t {
        0x7fffffffffffffff as int64_t
    } else {
        -(1 as ::core::ffi::c_int) as int64_t - 0x7fffffffffffffff as int64_t
    };
}
