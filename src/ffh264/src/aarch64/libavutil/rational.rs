// Translated from FFmpeg 9.0.1 libavutil/rational.c
// (preprocessed for aarch64-unknown-linux-musl) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// rational numbers
// Copyright (c) 2003 Michael Niedermayer <michaelni@gmx.at>
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
    fn __fpclassifyl(_: crate::rt::F128) -> ::core::ffi::c_int;
    fn av_gcd(a: int64_t, b: int64_t) -> int64_t;
    fn av_rescale(a: int64_t, b: int64_t, c: int64_t) -> int64_t;
    fn av_rescale_rnd(a: int64_t, b: int64_t, c: int64_t, rnd: AVRounding) -> int64_t;
    fn fabs(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn floor(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn frexp(
        _: ::core::ffi::c_double,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
}
pub type int64_t = ::core::ffi::c_long;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint64_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __f: ::core::ffi::c_float,
    pub __i: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub __f: ::core::ffi::c_double,
    pub __i: ::core::ffi::c_ulonglong,
}
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
pub unsafe extern "C" fn av_reduce(
    mut dst_num: *mut ::core::ffi::c_int,
    mut dst_den: *mut ::core::ffi::c_int,
    mut num: int64_t,
    mut den: int64_t,
    mut max: int64_t,
) -> ::core::ffi::c_int {
    let mut a0: AVRational = AVRational {
        num: 0 as ::core::ffi::c_int,
        den: 1 as ::core::ffi::c_int,
    };
    let mut a1: AVRational = AVRational {
        num: 1 as ::core::ffi::c_int,
        den: 0 as ::core::ffi::c_int,
    };
    let mut sign: ::core::ffi::c_int = (num < 0 as int64_t) as ::core::ffi::c_int
        ^ (den < 0 as int64_t) as ::core::ffi::c_int;
    let mut gcd: int64_t = av_gcd(
        if num >= 0 as int64_t { num } else { -num },
        if den >= 0 as int64_t { den } else { -den },
    );
    if gcd != 0 {
        num = (if num >= 0 as int64_t { num } else { -num }) / gcd;
        den = (if den >= 0 as int64_t { den } else { -den }) / gcd;
    }
    if num <= max && den <= max {
        a1 = AVRational {
            num: num as ::core::ffi::c_int,
            den: den as ::core::ffi::c_int,
        };
        den = 0 as int64_t;
    }
    while den != 0 {
        let mut x: uint64_t = (num / den) as uint64_t;
        let mut next_den: int64_t = (num as uint64_t)
            .wrapping_sub((den as uint64_t).wrapping_mul(x)) as int64_t;
        let mut a2n: int64_t = x
            .wrapping_mul(a1.num as uint64_t)
            .wrapping_add(a0.num as uint64_t) as int64_t;
        let mut a2d: int64_t = x
            .wrapping_mul(a1.den as uint64_t)
            .wrapping_add(a0.den as uint64_t) as int64_t;
        if a2n > max || a2d > max {
            if a1.num != 0 {
                x = ((max - a0.num as int64_t) / a1.num as int64_t) as uint64_t;
            }
            if a1.den != 0 {
                x = if x > ((max - a0.den as int64_t) / a1.den as int64_t) as uint64_t {
                    ((max - a0.den as int64_t) / a1.den as int64_t) as uint64_t
                } else {
                    x
                };
            }
            if (den as uint64_t)
                .wrapping_mul(
                    (2 as uint64_t)
                        .wrapping_mul(x)
                        .wrapping_mul(a1.den as uint64_t)
                        .wrapping_add(a0.den as uint64_t),
                ) > (num * a1.den as int64_t) as uint64_t
            {
                a1 = AVRational {
                    num: x
                        .wrapping_mul(a1.num as uint64_t)
                        .wrapping_add(a0.num as uint64_t) as ::core::ffi::c_int,
                    den: x
                        .wrapping_mul(a1.den as uint64_t)
                        .wrapping_add(a0.den as uint64_t) as ::core::ffi::c_int,
                };
            }
            break;
        } else {
            a0 = a1;
            a1 = AVRational {
                num: a2n as ::core::ffi::c_int,
                den: a2d as ::core::ffi::c_int,
            };
            num = den;
            den = next_den;
        }
    }
    *dst_num = if sign != 0 { -a1.num } else { a1.num };
    *dst_den = a1.den;
    return (den == 0 as int64_t) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn __FLOAT_BITS(mut __f: ::core::ffi::c_float) -> ::core::ffi::c_uint {
    let mut __u: C2RustUnnamed = C2RustUnnamed { __f: 0. };
    __u.__f = __f;
    return __u.__i;
}
#[inline]
unsafe extern "C" fn __DOUBLE_BITS(
    mut __f: ::core::ffi::c_double,
) -> ::core::ffi::c_ulonglong {
    let mut __u: C2RustUnnamed_0 = C2RustUnnamed_0 { __f: 0. };
    __u.__f = __f;
    return __u.__i;
}
#[inline]
unsafe extern "C" fn av_make_q(
    mut num: ::core::ffi::c_int,
    mut den: ::core::ffi::c_int,
) -> AVRational {
    let mut r: AVRational = AVRational { num: num, den: den };
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn av_mul_q(mut b: AVRational, mut c: AVRational) -> AVRational {
    av_reduce(
        &raw mut b.num,
        &raw mut b.den,
        b.num as int64_t * c.num as int64_t,
        b.den as int64_t * c.den as int64_t,
        2147483647 as int64_t,
    );
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn av_div_q(mut b: AVRational, mut c: AVRational) -> AVRational {
    return av_mul_q(
        b,
        AVRational {
            num: c.den,
            den: c.num,
        },
    );
}
#[inline]
unsafe extern "C" fn av_cmp_q(
    mut a: AVRational,
    mut b: AVRational,
) -> ::core::ffi::c_int {
    let tmp: int64_t = a.num as int64_t * b.den as int64_t
        - b.num as int64_t * a.den as int64_t;
    if tmp != 0 {
        return ((tmp ^ a.den as int64_t ^ b.den as int64_t) >> 63 as ::core::ffi::c_int)
            as ::core::ffi::c_int | 1 as ::core::ffi::c_int
    } else if b.den != 0 && a.den != 0 {
        return 0 as ::core::ffi::c_int
    } else if a.num != 0 && b.num != 0 {
        return (a.num >> 31 as ::core::ffi::c_int) - (b.num >> 31 as ::core::ffi::c_int)
    } else {
        return -(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_add_q(mut b: AVRational, mut c: AVRational) -> AVRational {
    av_reduce(
        &raw mut b.num,
        &raw mut b.den,
        b.num as int64_t * c.den as int64_t + c.num as int64_t * b.den as int64_t,
        b.den as int64_t * c.den as int64_t,
        2147483647 as int64_t,
    );
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn av_sub_q(mut b: AVRational, mut c: AVRational) -> AVRational {
    av_reduce(
        &raw mut b.num,
        &raw mut b.den,
        b.num as int64_t * c.den as int64_t - c.num as int64_t * b.den as int64_t,
        b.den as int64_t * c.den as int64_t,
        2147483647 as int64_t,
    );
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn av_d2q(
    mut d: ::core::ffi::c_double,
    mut max: ::core::ffi::c_int,
) -> AVRational {
    let mut a: AVRational = AVRational { num: 0, den: 0 };
    let mut exponent: ::core::ffi::c_int = 0;
    let mut den: int64_t = 0;
    if if ::core::mem::size_of::<::core::ffi::c_double>() as usize
        == ::core::mem::size_of::<::core::ffi::c_float>() as usize
    {
        (__FLOAT_BITS(d as ::core::ffi::c_float)
            & 0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint
            > 0x7f800000 as ::core::ffi::c_int as ::core::ffi::c_uint)
            as ::core::ffi::c_int
    } else if ::core::mem::size_of::<::core::ffi::c_double>() as usize
        == ::core::mem::size_of::<::core::ffi::c_double>() as usize
    {
        (__DOUBLE_BITS(d)
            & (1 as ::core::ffi::c_ulonglong).wrapping_neg() >> 1 as ::core::ffi::c_int
            > (0x7ff as ::core::ffi::c_ulonglong) << 52 as ::core::ffi::c_int)
            as ::core::ffi::c_int
    } else {
        (__fpclassifyl(crate::rt::F128::new(d)) == 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int
    } != 0
    {
        return AVRational {
            num: 0 as ::core::ffi::c_int,
            den: 0 as ::core::ffi::c_int,
        };
    }
    if fabs(d)
        > (2147483647 as ::core::ffi::c_longlong + 3 as ::core::ffi::c_longlong)
            as ::core::ffi::c_double
    {
        return AVRational {
            num: if d < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                -(1 as ::core::ffi::c_int)
            } else {
                1 as ::core::ffi::c_int
            },
            den: 0 as ::core::ffi::c_int,
        };
    }
    frexp(d, &raw mut exponent);
    exponent = if exponent - 1 as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        exponent - 1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    den = ((1 as ::core::ffi::c_longlong) << 62 as ::core::ffi::c_int - exponent)
        as int64_t;
    av_reduce(
        &raw mut a.num,
        &raw mut a.den,
        floor(d * den as ::core::ffi::c_double + 0.5f64) as int64_t,
        den,
        max as int64_t,
    );
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn av_nearer_q(
    mut q: AVRational,
    mut q1: AVRational,
    mut q2: AVRational,
) -> ::core::ffi::c_int {
    let mut a: int64_t = q1.num as int64_t * q2.den as int64_t
        + q2.num as int64_t * q1.den as int64_t;
    let mut b: int64_t = 2 as int64_t * q1.den as int64_t * q2.den as int64_t;
    let mut x_up: int64_t = av_rescale_rnd(a, q.den as int64_t, b, AV_ROUND_UP);
    let mut x_down: int64_t = av_rescale_rnd(a, q.den as int64_t, b, AV_ROUND_DOWN);
    return ((x_up > q.num as int64_t) as ::core::ffi::c_int
        - (x_down < q.num as int64_t) as ::core::ffi::c_int) * av_cmp_q(q2, q1);
}
#[no_mangle]
pub unsafe extern "C" fn av_find_nearest_q_idx(
    mut q: AVRational,
    mut q_list: *const AVRational,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut nearest_q_idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while (*q_list.offset(i as isize)).den != 0 {
        if av_nearer_q(
            q,
            *q_list.offset(i as isize),
            *q_list.offset(nearest_q_idx as isize),
        ) > 0 as ::core::ffi::c_int
        {
            nearest_q_idx = i;
        }
        i += 1;
    }
    return nearest_q_idx;
}
#[no_mangle]
pub unsafe extern "C" fn av_q2intfloat(mut q: AVRational) -> uint32_t {
    let mut n: int64_t = 0;
    let mut shift: ::core::ffi::c_int = 0;
    let mut sign: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if q.den < 0 as ::core::ffi::c_int {
        q.den *= -(1 as ::core::ffi::c_int);
        q.num *= -(1 as ::core::ffi::c_int);
    }
    if q.num < 0 as ::core::ffi::c_int {
        q.num *= -(1 as ::core::ffi::c_int);
        sign = 1 as ::core::ffi::c_int;
    }
    if q.num == 0 && q.den == 0 {
        return 0xffc00000 as uint32_t;
    }
    if q.num == 0 {
        return 0 as uint32_t;
    }
    if q.den == 0 {
        return 0x7f800000 as ::core::ffi::c_int as uint32_t
            | q.num as uint32_t & 0x80000000 as uint32_t;
    }
    shift = 23 as ::core::ffi::c_int
        + (31 as ::core::ffi::c_int
            - ((q.den | 1 as ::core::ffi::c_int) as ::core::ffi::c_uint).leading_zeros()
                as i32)
        - (31 as ::core::ffi::c_int
            - ((q.num | 1 as ::core::ffi::c_int) as ::core::ffi::c_uint).leading_zeros()
                as i32);
    if shift >= 0 as ::core::ffi::c_int {
        n = av_rescale(
            q.num as int64_t,
            ((1 as ::core::ffi::c_longlong) << shift) as int64_t,
            q.den as int64_t,
        );
    } else {
        n = av_rescale(q.num as int64_t, 1 as int64_t, (q.den as int64_t) << -shift);
    }
    shift
        -= (n >= ((1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int) as int64_t)
            as ::core::ffi::c_int;
    shift
        += (n < ((1 as ::core::ffi::c_int) << 23 as ::core::ffi::c_int) as int64_t)
            as ::core::ffi::c_int;
    if shift >= 0 as ::core::ffi::c_int {
        n = av_rescale(
            q.num as int64_t,
            ((1 as ::core::ffi::c_longlong) << shift) as int64_t,
            q.den as int64_t,
        );
    } else {
        n = av_rescale(q.num as int64_t, 1 as int64_t, (q.den as int64_t) << -shift);
    }
    return ((sign << 31 as ::core::ffi::c_int
        | 150 as ::core::ffi::c_int - shift << 23 as ::core::ffi::c_int) as int64_t
        | n - ((1 as ::core::ffi::c_int) << 23 as ::core::ffi::c_int) as int64_t)
        as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn av_gcd_q(
    mut a: AVRational,
    mut b: AVRational,
    mut max_den: ::core::ffi::c_int,
    mut def: AVRational,
) -> AVRational {
    let mut gcd: int64_t = 0;
    let mut lcm: int64_t = 0;
    gcd = av_gcd(a.den as int64_t, b.den as int64_t);
    lcm = a.den as int64_t / gcd * b.den as int64_t;
    return if lcm < max_den as int64_t {
        av_make_q(
            av_gcd(a.num as int64_t, b.num as int64_t) as ::core::ffi::c_int,
            lcm as ::core::ffi::c_int,
        )
    } else {
        def
    };
}
