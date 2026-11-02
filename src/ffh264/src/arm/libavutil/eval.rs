// Translated from FFmpeg 9.0.1 libavutil/eval.c
// (preprocessed for armv7-unknown-linux-musleabihf) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// Copyright (c) 2002-2006 Michael Niedermayer <michaelni@gmx.at>
// Copyright (c) 2006 Oded Shimon <ods15@ods15.dyndns.org>
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
    static ff_reverse: [uint8_t; 256];
    fn strtod(
        _: *const ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
    fn av_gettime() -> int64_t;
    fn strtoul(
        _: *const ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    fn strchr(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn __fpclassifyl(_: crate::rt::F128) -> ::core::ffi::c_int;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn av_malloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn av_mallocz(size: size_t) -> *mut ::core::ffi::c_void;
    fn acos(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn asin(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn atan(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn atan2(
        _: ::core::ffi::c_double,
        _: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn ceil(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn av_gcd(a: int64_t, b: int64_t) -> int64_t;
    fn cos(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn cosh(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn av_realloc(
        ptr: *mut ::core::ffi::c_void,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn exp(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn exp2(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn floor(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn hypot(
        _: ::core::ffi::c_double,
        _: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn log(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn pow(_: ::core::ffi::c_double, _: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn round(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sin(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sinh(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sqrt(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn tan(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn tanh(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn av_free(ptr: *mut ::core::ffi::c_void);
    fn trunc(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn av_default_item_name(ctx: *mut ::core::ffi::c_void) -> *const ::core::ffi::c_char;
    fn av_freep(ptr: *mut ::core::ffi::c_void);
}
pub type size_t = ::core::ffi::c_uint;
pub type AVClassCategory = ::core::ffi::c_uint;
pub const AV_CLASS_CATEGORY_NB: AVClassCategory = 46;
pub const AV_CLASS_CATEGORY_DEVICE_INPUT: AVClassCategory = 45;
pub const AV_CLASS_CATEGORY_DEVICE_OUTPUT: AVClassCategory = 44;
pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_INPUT: AVClassCategory = 43;
pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_OUTPUT: AVClassCategory = 42;
pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_INPUT: AVClassCategory = 41;
pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_OUTPUT: AVClassCategory = 40;
pub const AV_CLASS_CATEGORY_HWDEVICE: AVClassCategory = 11;
pub const AV_CLASS_CATEGORY_SWRESAMPLER: AVClassCategory = 10;
pub const AV_CLASS_CATEGORY_SWSCALER: AVClassCategory = 9;
pub const AV_CLASS_CATEGORY_BITSTREAM_FILTER: AVClassCategory = 8;
pub const AV_CLASS_CATEGORY_FILTER: AVClassCategory = 7;
pub const AV_CLASS_CATEGORY_DECODER: AVClassCategory = 6;
pub const AV_CLASS_CATEGORY_ENCODER: AVClassCategory = 5;
pub const AV_CLASS_CATEGORY_DEMUXER: AVClassCategory = 4;
pub const AV_CLASS_CATEGORY_MUXER: AVClassCategory = 3;
pub const AV_CLASS_CATEGORY_OUTPUT: AVClassCategory = 2;
pub const AV_CLASS_CATEGORY_INPUT: AVClassCategory = 1;
pub const AV_CLASS_CATEGORY_NA: AVClassCategory = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVClass {
    pub class_name: *const ::core::ffi::c_char,
    pub item_name: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const ::core::ffi::c_char,
    >,
    pub option: *const AVOption,
    pub version: ::core::ffi::c_int,
    pub log_level_offset_offset: ::core::ffi::c_int,
    pub parent_log_context_offset: ::core::ffi::c_int,
    pub category: AVClassCategory,
    pub get_category: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> AVClassCategory,
    >,
    pub query_ranges: Option<
        unsafe extern "C" fn(
            *mut *mut AVOptionRanges,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub child_next: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub child_class_iterate: Option<
        unsafe extern "C" fn(*mut *mut ::core::ffi::c_void) -> *const AVClass,
    >,
    pub state_flags_offset: ::core::ffi::c_int,
}
pub type int8_t = ::core::ffi::c_schar;
pub type int64_t = ::core::ffi::c_longlong;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint64_t = ::core::ffi::c_ulonglong;
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
pub struct AVExpr {
    pub type_0: ::core::ffi::c_uchar,
    pub root: ::core::ffi::c_uchar,
    pub depth: ::core::ffi::c_short,
    pub const_index: ::core::ffi::c_int,
    pub value: ::core::ffi::c_double,
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub param: [*mut AVExpr; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub func0: Option<
        unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
    >,
    pub func1: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_double,
        ) -> ::core::ffi::c_double,
    >,
    pub func2: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
        ) -> ::core::ffi::c_double,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFSFC64 {
    pub a: uint64_t,
    pub b: uint64_t,
    pub c: uint64_t,
    pub counter: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVExprRoot {
    pub avexpr: AVExpr,
    pub var: *mut ::core::ffi::c_double,
    pub prng_state: *mut FFSFC64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Parser {
    pub class: *const AVClass,
    pub stack_index: ::core::ffi::c_int,
    pub s: *mut ::core::ffi::c_char,
    pub const_values: *const ::core::ffi::c_double,
    pub const_names: *const *const ::core::ffi::c_char,
    pub funcs1: *const Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_double,
        ) -> ::core::ffi::c_double,
    >,
    pub func1_names: *const *const ::core::ffi::c_char,
    pub funcs2: *const Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
        ) -> ::core::ffi::c_double,
    >,
    pub func2_names: *const *const ::core::ffi::c_char,
    pub opaque: *mut ::core::ffi::c_void,
    pub log_offset: ::core::ffi::c_int,
    pub log_ctx: *mut ::core::ffi::c_void,
    pub var: *mut ::core::ffi::c_double,
    pub prng_state: *mut FFSFC64,
}
pub const e_bitor: C2RustUnnamed_4 = 40;
pub const e_bitand: C2RustUnnamed_4 = 39;
pub const e_atan2: C2RustUnnamed_4 = 43;
pub const e_hypot: C2RustUnnamed_4 = 34;
pub const e_st: C2RustUnnamed_4 = 23;
pub const e_last: C2RustUnnamed_4 = 22;
pub const e_add: C2RustUnnamed_4 = 21;
pub const e_div: C2RustUnnamed_4 = 20;
pub const e_mul: C2RustUnnamed_4 = 19;
pub const e_pow: C2RustUnnamed_4 = 18;
pub const e_lte: C2RustUnnamed_4 = 16;
pub const e_lt: C2RustUnnamed_4 = 17;
pub const e_gte: C2RustUnnamed_4 = 15;
pub const e_gt: C2RustUnnamed_4 = 14;
pub const e_eq: C2RustUnnamed_4 = 13;
pub const e_min: C2RustUnnamed_4 = 12;
pub const e_max: C2RustUnnamed_4 = 11;
pub const e_gcd: C2RustUnnamed_4 = 35;
pub const e_mod: C2RustUnnamed_4 = 10;
pub const e_root: C2RustUnnamed_4 = 26;
pub const e_taylor: C2RustUnnamed_4 = 25;
pub const e_while: C2RustUnnamed_4 = 24;
pub const e_randomi: C2RustUnnamed_4 = 46;
pub const e_random: C2RustUnnamed_4 = 33;
pub const e_print: C2RustUnnamed_4 = 38;
pub const e_lerp: C2RustUnnamed_4 = 44;
pub const e_between: C2RustUnnamed_4 = 41;
pub const e_clip: C2RustUnnamed_4 = 42;
pub const e_ifnot: C2RustUnnamed_4 = 37;
pub const e_if: C2RustUnnamed_4 = 36;
pub const e_not: C2RustUnnamed_4 = 32;
pub const e_sqrt: C2RustUnnamed_4 = 31;
pub const e_sgn: C2RustUnnamed_4 = 45;
pub const e_round: C2RustUnnamed_4 = 30;
pub const e_trunc: C2RustUnnamed_4 = 29;
pub const e_ceil: C2RustUnnamed_4 = 28;
pub const e_floor: C2RustUnnamed_4 = 27;
pub const e_isinf: C2RustUnnamed_4 = 9;
pub const e_isnan: C2RustUnnamed_4 = 8;
pub const e_ld: C2RustUnnamed_4 = 7;
pub const e_gauss: C2RustUnnamed_4 = 6;
pub const e_squish: C2RustUnnamed_4 = 5;
pub const e_func2: C2RustUnnamed_4 = 4;
pub const e_func1: C2RustUnnamed_4 = 3;
pub const e_func0: C2RustUnnamed_4 = 2;
pub const e_const: C2RustUnnamed_4 = 1;
pub const e_value: C2RustUnnamed_4 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub name: *const ::core::ffi::c_char,
    pub value: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub bin_val: ::core::ffi::c_double,
    pub dec_val: ::core::ffi::c_double,
    pub exp: int8_t,
}
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn ff_sfc64_get(mut s: *mut FFSFC64) -> uint64_t {
    let fresh1 = (*s).counter;
    (*s).counter = (*s).counter.wrapping_add(1);
    let mut tmp: uint64_t = (*s).a.wrapping_add((*s).b).wrapping_add(fresh1);
    (*s).a = (*s).b ^ (*s).b >> 11 as ::core::ffi::c_int;
    (*s).b = (*s).c.wrapping_add((*s).c << 3 as ::core::ffi::c_int);
    (*s).c = ((*s).c << 24 as ::core::ffi::c_int | (*s).c >> 40 as ::core::ffi::c_int)
        .wrapping_add(tmp);
    return tmp;
}
#[inline(always)]
unsafe extern "C" fn ff_exp10(mut x: ::core::ffi::c_double) -> ::core::ffi::c_double {
    return exp2(3.32192809488736234787f64 * x);
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
static mut eval_class: AVClass = unsafe {
    AVClass {
        class_name: b"Eval\0" as *const u8 as *const ::core::ffi::c_char,
        item_name: Some(
            av_default_item_name
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                ) -> *const ::core::ffi::c_char,
        ),
        option: ::core::ptr::null::<AVOption>(),
        version: (61 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            | 101 as ::core::ffi::c_int,
        log_level_offset_offset: 40 as ::core::ffi::c_uint as ::core::ffi::c_int,
        parent_log_context_offset: 44 as ::core::ffi::c_uint as ::core::ffi::c_int,
        category: AV_CLASS_CATEGORY_NA,
        get_category: None,
        query_ranges: None,
        child_next: None,
        child_class_iterate: None,
        state_flags_offset: 0,
    }
};
static mut si_prefixes: [C2RustUnnamed_3; 54] = [
    C2RustUnnamed_3 {
        bin_val: 1.152921504606847e18f64,
        dec_val: 1e18f64,
        exp: 18 as int8_t,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 1.073741824e9f64,
        dec_val: 1e9f64,
        exp: 9 as int8_t,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 1.024e3f64,
        dec_val: 1e3f64,
        exp: 3 as int8_t,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 1.048576e6f64,
        dec_val: 1e6f64,
        exp: 6 as int8_t,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 1.125899906842624e15f64,
        dec_val: 1e15f64,
        exp: 15 as int8_t,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 1.099511627776e12f64,
        dec_val: 1e12f64,
        exp: 12 as int8_t,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 1.2089258196146292e24f64,
        dec_val: 1e24f64,
        exp: 24 as int8_t,
    },
    C2RustUnnamed_3 {
        bin_val: 1.1805916207174113e21f64,
        dec_val: 1e21f64,
        exp: 21 as int8_t,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 8.6736173798840355e-19f64,
        dec_val: 1e-18f64,
        exp: -(18 as ::core::ffi::c_int) as int8_t,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 9.8431332023036951e-3f64,
        dec_val: 1e-2f64,
        exp: -(2 as ::core::ffi::c_int) as int8_t,
    },
    C2RustUnnamed_3 {
        bin_val: 9.921256574801246e-2f64,
        dec_val: 1e-1f64,
        exp: -(1 as ::core::ffi::c_int) as int8_t,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 8.8817841970012523e-16f64,
        dec_val: 1e-15f64,
        exp: -(15 as ::core::ffi::c_int) as int8_t,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 1.0159366732596479e2f64,
        dec_val: 1e2f64,
        exp: 2 as int8_t,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 1.024e3f64,
        dec_val: 1e3f64,
        exp: 3 as int8_t,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 9.765625e-4f64,
        dec_val: 1e-3f64,
        exp: -(3 as ::core::ffi::c_int) as int8_t,
    },
    C2RustUnnamed_3 {
        bin_val: 9.3132257461547852e-10f64,
        dec_val: 1e-9f64,
        exp: -(9 as ::core::ffi::c_int) as int8_t,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 9.0949470177292824e-13f64,
        dec_val: 1e-12f64,
        exp: -(12 as ::core::ffi::c_int) as int8_t,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 9.5367431640625e-7f64,
        dec_val: 1e-6f64,
        exp: -(6 as ::core::ffi::c_int) as int8_t,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 0.,
        dec_val: 0.,
        exp: 0,
    },
    C2RustUnnamed_3 {
        bin_val: 8.271806125530276749e-25f64,
        dec_val: 1e-24f64,
        exp: -(24 as ::core::ffi::c_int) as int8_t,
    },
    C2RustUnnamed_3 {
        bin_val: 8.4703294725430034e-22f64,
        dec_val: 1e-21f64,
        exp: -(21 as ::core::ffi::c_int) as int8_t,
    },
];
#[inline]
unsafe extern "C" fn ff_sfc64_init(
    mut s: *mut FFSFC64,
    mut seeda: uint64_t,
    mut seedb: uint64_t,
    mut seedc: uint64_t,
    mut rounds: ::core::ffi::c_int,
) {
    (*s).a = seeda;
    (*s).b = seedb;
    (*s).c = seedc;
    (*s).counter = 1 as uint64_t;
    loop {
        let fresh2 = rounds;
        rounds = rounds - 1;
        if !(fresh2 != 0) {
            break;
        }
        ff_sfc64_get(s);
    };
}
static mut constants: [C2RustUnnamed_2; 4] = [
    C2RustUnnamed_2 {
        name: b"E\0" as *const u8 as *const ::core::ffi::c_char,
        value: 2.7182818284590452354f64,
    },
    C2RustUnnamed_2 {
        name: b"PI\0" as *const u8 as *const ::core::ffi::c_char,
        value: 3.14159265358979323846f64,
    },
    C2RustUnnamed_2 {
        name: b"PHI\0" as *const u8 as *const ::core::ffi::c_char,
        value: 1.61803398874989484820f64,
    },
    C2RustUnnamed_2 {
        name: b"QP2LAMBDA\0" as *const u8 as *const ::core::ffi::c_char,
        value: 118 as ::core::ffi::c_int as ::core::ffi::c_double,
    },
];
#[no_mangle]
pub unsafe extern "C" fn av_strtod(
    mut numstr: *const ::core::ffi::c_char,
    mut tail: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_double {
    let mut d: ::core::ffi::c_double = 0.;
    let mut next: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if *numstr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == '0' as i32
        && *numstr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            | 0x20 as ::core::ffi::c_int == 'x' as i32
    {
        d = strtoul(numstr, &raw mut next, 16 as ::core::ffi::c_int)
            as ::core::ffi::c_double;
    } else {
        d = strtod(numstr, &raw mut next);
    }
    if next != numstr as *mut ::core::ffi::c_char {
        if *next.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'd' as i32
            && *next.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'B' as i32
        {
            d = ff_exp10(d / 20 as ::core::ffi::c_int as ::core::ffi::c_double);
            next = next.offset(2 as ::core::ffi::c_int as isize);
        } else if *next as ::core::ffi::c_int >= 'E' as i32
            && *next as ::core::ffi::c_int <= 'z' as i32
        {
            let mut e: ::core::ffi::c_int = si_prefixes[(*next as ::core::ffi::c_int
                    - 'E' as i32) as usize]
                .exp as ::core::ffi::c_int;
            if e != 0 {
                if *next.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'i' as i32
                {
                    d
                        *= si_prefixes[(*next as ::core::ffi::c_int - 'E' as i32)
                                as usize]
                            .bin_val;
                    next = next.offset(2 as ::core::ffi::c_int as isize);
                } else {
                    d
                        *= si_prefixes[(*next as ::core::ffi::c_int - 'E' as i32)
                                as usize]
                            .dec_val;
                    next = next.offset(1);
                }
            }
        }
        if *next as ::core::ffi::c_int == 'B' as i32 {
            d *= 8 as ::core::ffi::c_int as ::core::ffi::c_double;
            next = next.offset(1);
        }
    }
    if !tail.is_null() {
        *tail = next;
    }
    return d;
}
unsafe extern "C" fn strmatch(
    mut s: *const ::core::ffi::c_char,
    mut prefix: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while *prefix.offset(i as isize) != 0 {
        if *prefix.offset(i as isize) as ::core::ffi::c_int
            != *s.offset(i as isize) as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return !((*s.offset(i as isize) as ::core::ffi::c_int - '0' as i32)
        as ::core::ffi::c_uint <= 9 as ::core::ffi::c_uint
        || (*s.offset(i as isize) as ::core::ffi::c_int - 'a' as i32)
            as ::core::ffi::c_uint <= 25 as ::core::ffi::c_uint
        || (*s.offset(i as isize) as ::core::ffi::c_int - 'A' as i32)
            as ::core::ffi::c_uint <= 25 as ::core::ffi::c_uint
        || *s.offset(i as isize) as ::core::ffi::c_int == '_' as i32)
        as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn av_clip_c(
    mut a: ::core::ffi::c_int,
    mut amin: ::core::ffi::c_int,
    mut amax: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if a < amin { return amin } else if a > amax { return amax } else { return a };
}
unsafe extern "C" fn etime(mut v: ::core::ffi::c_double) -> ::core::ffi::c_double {
    return av_gettime() as ::core::ffi::c_double * 0.000001f64;
}
unsafe extern "C" fn eval_expr(
    mut p: *mut Parser,
    mut e: *mut AVExpr,
) -> ::core::ffi::c_double {
    match (*e).type_0 as ::core::ffi::c_int {
        0 => return (*e).value,
        1 => return (*e).value * *(*p).const_values.offset((*e).const_index as isize),
        2 => {
            return (*e).value
                * (*e)
                    .c2rust_unnamed
                    .func0
                    .expect(
                        "non-null function pointer",
                    )(eval_expr(p, (*e).param[0 as ::core::ffi::c_int as usize]));
        }
        3 => {
            return (*e).value
                * (*e)
                    .c2rust_unnamed
                    .func1
                    .expect(
                        "non-null function pointer",
                    )(
                    (*p).opaque,
                    eval_expr(p, (*e).param[0 as ::core::ffi::c_int as usize]),
                );
        }
        4 => {
            return (*e).value
                * (*e)
                    .c2rust_unnamed
                    .func2
                    .expect(
                        "non-null function pointer",
                    )(
                    (*p).opaque,
                    eval_expr(p, (*e).param[0 as ::core::ffi::c_int as usize]),
                    eval_expr(p, (*e).param[1 as ::core::ffi::c_int as usize]),
                );
        }
        5 => {
            return (*e).value
                / (1 as ::core::ffi::c_int as ::core::ffi::c_double
                    + exp(
                        4 as ::core::ffi::c_int as ::core::ffi::c_double
                            * eval_expr(p, (*e).param[0 as ::core::ffi::c_int as usize]),
                    ));
        }
        6 => {
            let mut d: ::core::ffi::c_double = eval_expr(
                p,
                (*e).param[0 as ::core::ffi::c_int as usize],
            );
            return (*e).value
                * exp(-d * d / 2 as ::core::ffi::c_int as ::core::ffi::c_double)
                / sqrt(
                    2 as ::core::ffi::c_int as ::core::ffi::c_double
                        * 3.14159265358979323846f64,
                );
        }
        7 => {
            return (*e).value
                * *(*p)
                    .var
                    .offset(
                        av_clip_c(
                            eval_expr(p, (*e).param[0 as ::core::ffi::c_int as usize])
                                as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
                        ) as isize,
                    );
        }
        8 => {
            return (*e).value
                * ((if ::core::mem::size_of::<::core::ffi::c_double>() as usize
                    == ::core::mem::size_of::<::core::ffi::c_float>() as usize
                {
                    (__FLOAT_BITS(
                        eval_expr(p, (*e).param[0 as ::core::ffi::c_int as usize])
                            as ::core::ffi::c_float,
                    ) & 0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint
                        > 0x7f800000 as ::core::ffi::c_int as ::core::ffi::c_uint)
                        as ::core::ffi::c_int
                } else {
                    (if ::core::mem::size_of::<::core::ffi::c_double>() as usize
                        == ::core::mem::size_of::<::core::ffi::c_double>() as usize
                    {
                        (__DOUBLE_BITS(
                            eval_expr(p, (*e).param[0 as ::core::ffi::c_int as usize]),
                        )
                            & (1 as ::core::ffi::c_ulonglong).wrapping_neg()
                                >> 1 as ::core::ffi::c_int
                            > (0x7ff as ::core::ffi::c_ulonglong)
                                << 52 as ::core::ffi::c_int) as ::core::ffi::c_int
                    } else {
                        (__fpclassifyl(
                            crate::rt::F128::new(
                                eval_expr(p, (*e).param[0 as ::core::ffi::c_int as usize]),
                            ),
                        ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                    })
                }) != 0) as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        9 => {
            return (*e).value
                * ((if ::core::mem::size_of::<::core::ffi::c_double>() as usize
                    == ::core::mem::size_of::<::core::ffi::c_float>() as usize
                {
                    (__FLOAT_BITS(
                        eval_expr(p, (*e).param[0 as ::core::ffi::c_int as usize])
                            as ::core::ffi::c_float,
                    ) & 0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint
                        == 0x7f800000 as ::core::ffi::c_int as ::core::ffi::c_uint)
                        as ::core::ffi::c_int
                } else {
                    (if ::core::mem::size_of::<::core::ffi::c_double>() as usize
                        == ::core::mem::size_of::<::core::ffi::c_double>() as usize
                    {
                        (__DOUBLE_BITS(
                            eval_expr(p, (*e).param[0 as ::core::ffi::c_int as usize]),
                        )
                            & (1 as ::core::ffi::c_ulonglong).wrapping_neg()
                                >> 1 as ::core::ffi::c_int
                            == (0x7ff as ::core::ffi::c_ulonglong)
                                << 52 as ::core::ffi::c_int) as ::core::ffi::c_int
                    } else {
                        (__fpclassifyl(
                            crate::rt::F128::new(
                                eval_expr(p, (*e).param[0 as ::core::ffi::c_int as usize]),
                            ),
                        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
                    })
                }) != 0) as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        27 => {
            return (*e).value
                * floor(eval_expr(p, (*e).param[0 as ::core::ffi::c_int as usize]));
        }
        28 => {
            return (*e).value
                * ceil(eval_expr(p, (*e).param[0 as ::core::ffi::c_int as usize]));
        }
        29 => {
            return (*e).value
                * trunc(eval_expr(p, (*e).param[0 as ::core::ffi::c_int as usize]));
        }
        30 => {
            return (*e).value
                * round(eval_expr(p, (*e).param[0 as ::core::ffi::c_int as usize]));
        }
        45 => {
            return (*e).value
                * ((eval_expr(p, (*e).param[0 as ::core::ffi::c_int as usize])
                    > 0 as ::core::ffi::c_int as ::core::ffi::c_double)
                    as ::core::ffi::c_int
                    - (eval_expr(p, (*e).param[0 as ::core::ffi::c_int as usize])
                        < 0 as ::core::ffi::c_int as ::core::ffi::c_double)
                        as ::core::ffi::c_int) as ::core::ffi::c_double;
        }
        31 => {
            return (*e).value
                * sqrt(eval_expr(p, (*e).param[0 as ::core::ffi::c_int as usize]));
        }
        32 => {
            return (*e).value
                * (eval_expr(p, (*e).param[0 as ::core::ffi::c_int as usize])
                    == 0 as ::core::ffi::c_int as ::core::ffi::c_double)
                    as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        36 => {
            return (*e).value
                * (if eval_expr(p, (*e).param[0 as ::core::ffi::c_int as usize]) != 0. {
                    eval_expr(p, (*e).param[1 as ::core::ffi::c_int as usize])
                } else {
                    (if !(*e).param[2 as ::core::ffi::c_int as usize].is_null() {
                        eval_expr(p, (*e).param[2 as ::core::ffi::c_int as usize])
                    } else {
                        0 as ::core::ffi::c_int as ::core::ffi::c_double
                    })
                });
        }
        37 => {
            return (*e).value
                * (if eval_expr(p, (*e).param[0 as ::core::ffi::c_int as usize]) == 0. {
                    eval_expr(p, (*e).param[1 as ::core::ffi::c_int as usize])
                } else {
                    (if !(*e).param[2 as ::core::ffi::c_int as usize].is_null() {
                        eval_expr(p, (*e).param[2 as ::core::ffi::c_int as usize])
                    } else {
                        0 as ::core::ffi::c_int as ::core::ffi::c_double
                    })
                });
        }
        42 => {
            let mut x: ::core::ffi::c_double = eval_expr(
                p,
                (*e).param[0 as ::core::ffi::c_int as usize],
            );
            let mut min: ::core::ffi::c_double = eval_expr(
                p,
                (*e).param[1 as ::core::ffi::c_int as usize],
            );
            let mut max: ::core::ffi::c_double = eval_expr(
                p,
                (*e).param[2 as ::core::ffi::c_int as usize],
            );
            if (if ::core::mem::size_of::<::core::ffi::c_double>() as usize
                == ::core::mem::size_of::<::core::ffi::c_float>() as usize
            {
                (__FLOAT_BITS(min as ::core::ffi::c_float)
                    & 0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint
                    > 0x7f800000 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    as ::core::ffi::c_int
            } else {
                (if ::core::mem::size_of::<::core::ffi::c_double>() as usize
                    == ::core::mem::size_of::<::core::ffi::c_double>() as usize
                {
                    (__DOUBLE_BITS(min)
                        & (1 as ::core::ffi::c_ulonglong).wrapping_neg()
                            >> 1 as ::core::ffi::c_int
                        > (0x7ff as ::core::ffi::c_ulonglong)
                            << 52 as ::core::ffi::c_int) as ::core::ffi::c_int
                } else {
                    (__fpclassifyl(crate::rt::F128::new(min)) == 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                })
            }) != 0
                || (if ::core::mem::size_of::<::core::ffi::c_double>() as usize
                    == ::core::mem::size_of::<::core::ffi::c_float>() as usize
                {
                    (__FLOAT_BITS(max as ::core::ffi::c_float)
                        & 0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint
                        > 0x7f800000 as ::core::ffi::c_int as ::core::ffi::c_uint)
                        as ::core::ffi::c_int
                } else {
                    (if ::core::mem::size_of::<::core::ffi::c_double>() as usize
                        == ::core::mem::size_of::<::core::ffi::c_double>() as usize
                    {
                        (__DOUBLE_BITS(max)
                            & (1 as ::core::ffi::c_ulonglong).wrapping_neg()
                                >> 1 as ::core::ffi::c_int
                            > (0x7ff as ::core::ffi::c_ulonglong)
                                << 52 as ::core::ffi::c_int) as ::core::ffi::c_int
                    } else {
                        (__fpclassifyl(crate::rt::F128::new(max))
                            == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                    })
                }) != 0
                || (if ::core::mem::size_of::<::core::ffi::c_double>() as usize
                    == ::core::mem::size_of::<::core::ffi::c_float>() as usize
                {
                    (__FLOAT_BITS(x as ::core::ffi::c_float)
                        & 0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint
                        > 0x7f800000 as ::core::ffi::c_int as ::core::ffi::c_uint)
                        as ::core::ffi::c_int
                } else {
                    (if ::core::mem::size_of::<::core::ffi::c_double>() as usize
                        == ::core::mem::size_of::<::core::ffi::c_double>() as usize
                    {
                        (__DOUBLE_BITS(x)
                            & (1 as ::core::ffi::c_ulonglong).wrapping_neg()
                                >> 1 as ::core::ffi::c_int
                            > (0x7ff as ::core::ffi::c_ulonglong)
                                << 52 as ::core::ffi::c_int) as ::core::ffi::c_int
                    } else {
                        (__fpclassifyl(crate::rt::F128::new(x)) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int
                    })
                }) != 0 || min > max
            {
                return ::core::f32::NAN as ::core::ffi::c_double;
            }
            return (*e).value
                * av_clipd_c(
                    eval_expr(p, (*e).param[0 as ::core::ffi::c_int as usize]),
                    min,
                    max,
                );
        }
        41 => {
            let mut d_0: ::core::ffi::c_double = eval_expr(
                p,
                (*e).param[0 as ::core::ffi::c_int as usize],
            );
            return (*e).value
                * (d_0 >= eval_expr(p, (*e).param[1 as ::core::ffi::c_int as usize])
                    && d_0 <= eval_expr(p, (*e).param[2 as ::core::ffi::c_int as usize]))
                    as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        44 => {
            let mut v0: ::core::ffi::c_double = eval_expr(
                p,
                (*e).param[0 as ::core::ffi::c_int as usize],
            );
            let mut v1: ::core::ffi::c_double = eval_expr(
                p,
                (*e).param[1 as ::core::ffi::c_int as usize],
            );
            let mut f: ::core::ffi::c_double = eval_expr(
                p,
                (*e).param[2 as ::core::ffi::c_int as usize],
            );
            return (*e).value * (v0 + (v1 - v0) * f);
        }
        38 => {
            let mut x_0: ::core::ffi::c_double = eval_expr(
                p,
                (*e).param[0 as ::core::ffi::c_int as usize],
            );
            let mut level: ::core::ffi::c_int = if !(*e)
                .param[1 as ::core::ffi::c_int as usize]
                .is_null()
            {
                av_clip_c(
                    eval_expr(p, (*e).param[1 as ::core::ffi::c_int as usize])
                        as ::core::ffi::c_int,
                    -(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int,
                    2147483647 as ::core::ffi::c_int,
                )
            } else {
                32 as ::core::ffi::c_int
            };
            av_log(
                p as *mut ::core::ffi::c_void,
                level,
                b"%f\n\0" as *const u8 as *const ::core::ffi::c_char,
                x_0,
            );
            return (*e).value * x_0;
        }
        33 => {
            let mut idx: ::core::ffi::c_int = av_clip_c(
                eval_expr(p, (*e).param[0 as ::core::ffi::c_int as usize])
                    as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
            );
            let mut s: *mut FFSFC64 = (*p).prng_state.offset(idx as isize);
            let mut r: uint64_t = 0;
            if (*s).counter == 0 {
                r = (if if ::core::mem::size_of::<::core::ffi::c_double>() as usize
                    == ::core::mem::size_of::<::core::ffi::c_float>() as usize
                {
                    (__FLOAT_BITS(*(*p).var.offset(idx as isize) as ::core::ffi::c_float)
                        & 0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint
                        > 0x7f800000 as ::core::ffi::c_int as ::core::ffi::c_uint)
                        as ::core::ffi::c_int
                } else if ::core::mem::size_of::<::core::ffi::c_double>() as usize
                    == ::core::mem::size_of::<::core::ffi::c_double>() as usize
                {
                    (__DOUBLE_BITS(*(*p).var.offset(idx as isize))
                        & (1 as ::core::ffi::c_ulonglong).wrapping_neg()
                            >> 1 as ::core::ffi::c_int
                        > (0x7ff as ::core::ffi::c_ulonglong)
                            << 52 as ::core::ffi::c_int) as ::core::ffi::c_int
                } else {
                    (__fpclassifyl(crate::rt::F128::new(*(*p).var.offset(idx as isize)))
                        == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                } != 0
                {
                    0 as ::core::ffi::c_int as ::core::ffi::c_double
                } else {
                    *(*p).var.offset(idx as isize)
                }) as uint64_t;
                ff_sfc64_init(s, r, r, r, 12 as ::core::ffi::c_int);
            }
            r = ff_sfc64_get(s);
            *(*p).var.offset(idx as isize) = r as ::core::ffi::c_double;
            return r as ::core::ffi::c_double
                * (1.0f64
                    / 0xffffffffffffffff as ::core::ffi::c_ulonglong
                        as ::core::ffi::c_double);
        }
        46 => {
            let mut min_0: ::core::ffi::c_double = eval_expr(
                p,
                (*e).param[1 as ::core::ffi::c_int as usize],
            );
            let mut max_0: ::core::ffi::c_double = eval_expr(
                p,
                (*e).param[2 as ::core::ffi::c_int as usize],
            );
            let mut idx_0: ::core::ffi::c_int = av_clip_c(
                eval_expr(p, (*e).param[0 as ::core::ffi::c_int as usize])
                    as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
            );
            let mut s_0: *mut FFSFC64 = (*p).prng_state.offset(idx_0 as isize);
            let mut r_0: uint64_t = 0;
            if (*s_0).counter == 0 {
                r_0 = (if if ::core::mem::size_of::<::core::ffi::c_double>() as usize
                    == ::core::mem::size_of::<::core::ffi::c_float>() as usize
                {
                    (__FLOAT_BITS(
                        *(*p).var.offset(idx_0 as isize) as ::core::ffi::c_float,
                    ) & 0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint
                        > 0x7f800000 as ::core::ffi::c_int as ::core::ffi::c_uint)
                        as ::core::ffi::c_int
                } else if ::core::mem::size_of::<::core::ffi::c_double>() as usize
                    == ::core::mem::size_of::<::core::ffi::c_double>() as usize
                {
                    (__DOUBLE_BITS(*(*p).var.offset(idx_0 as isize))
                        & (1 as ::core::ffi::c_ulonglong).wrapping_neg()
                            >> 1 as ::core::ffi::c_int
                        > (0x7ff as ::core::ffi::c_ulonglong)
                            << 52 as ::core::ffi::c_int) as ::core::ffi::c_int
                } else {
                    (__fpclassifyl(crate::rt::F128::new(*(*p).var.offset(idx_0 as isize)))
                        == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                } != 0
                {
                    0 as ::core::ffi::c_int as ::core::ffi::c_double
                } else {
                    *(*p).var.offset(idx_0 as isize)
                }) as uint64_t;
                ff_sfc64_init(s_0, r_0, r_0, r_0, 12 as ::core::ffi::c_int);
            }
            r_0 = ff_sfc64_get(s_0);
            *(*p).var.offset(idx_0 as isize) = r_0 as ::core::ffi::c_double;
            return min_0
                + (max_0 - min_0) * r_0 as ::core::ffi::c_double
                    / 0xffffffffffffffff as ::core::ffi::c_ulonglong
                        as ::core::ffi::c_double;
        }
        24 => {
            let mut d_1: ::core::ffi::c_double = ::core::f32::NAN
                as ::core::ffi::c_double;
            while eval_expr(p, (*e).param[0 as ::core::ffi::c_int as usize]) != 0. {
                d_1 = eval_expr(p, (*e).param[1 as ::core::ffi::c_int as usize]);
            }
            return d_1;
        }
        25 => {
            let mut t: ::core::ffi::c_double = 1 as ::core::ffi::c_int
                as ::core::ffi::c_double;
            let mut d_2: ::core::ffi::c_double = 0 as ::core::ffi::c_int
                as ::core::ffi::c_double;
            let mut v: ::core::ffi::c_double = 0.;
            let mut x_1: ::core::ffi::c_double = eval_expr(
                p,
                (*e).param[1 as ::core::ffi::c_int as usize],
            );
            let mut id: ::core::ffi::c_int = if !(*e)
                .param[2 as ::core::ffi::c_int as usize]
                .is_null()
            {
                av_clip_c(
                    eval_expr(p, (*e).param[2 as ::core::ffi::c_int as usize])
                        as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
                )
            } else {
                0 as ::core::ffi::c_int
            };
            let mut i: ::core::ffi::c_int = 0;
            let mut var0: ::core::ffi::c_double = *(*p).var.offset(id as isize);
            i = 0 as ::core::ffi::c_int;
            while i < 1000 as ::core::ffi::c_int {
                let mut ld: ::core::ffi::c_double = d_2;
                *(*p).var.offset(id as isize) = i as ::core::ffi::c_double;
                v = eval_expr(p, (*e).param[0 as ::core::ffi::c_int as usize]);
                d_2 += t * v;
                if ld == d_2 && v != 0. {
                    break;
                }
                t *= x_1 / (i + 1 as ::core::ffi::c_int) as ::core::ffi::c_double;
                i += 1;
            }
            *(*p).var.offset(id as isize) = var0;
            return d_2;
        }
        26 => {
            let mut i_0: ::core::ffi::c_int = 0;
            let mut j: ::core::ffi::c_int = 0;
            let mut low: ::core::ffi::c_double = -(1 as ::core::ffi::c_int)
                as ::core::ffi::c_double;
            let mut high: ::core::ffi::c_double = -(1 as ::core::ffi::c_int)
                as ::core::ffi::c_double;
            let mut v_0: ::core::ffi::c_double = 0.;
            let mut low_v: ::core::ffi::c_double = -1.7976931348623157e+308f64;
            let mut high_v: ::core::ffi::c_double = 1.7976931348623157e+308f64;
            let mut var0_0: ::core::ffi::c_double = *(*p)
                .var
                .offset(0 as ::core::ffi::c_int as isize);
            let mut x_max: ::core::ffi::c_double = eval_expr(
                p,
                (*e).param[1 as ::core::ffi::c_int as usize],
            );
            i_0 = -(1 as ::core::ffi::c_int);
            while i_0 < 1024 as ::core::ffi::c_int {
                if i_0 < 255 as ::core::ffi::c_int {
                    *(*p).var.offset(0 as ::core::ffi::c_int as isize) = ff_reverse[(i_0
                        & 255 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
                        as ::core::ffi::c_double * x_max
                        / 255 as ::core::ffi::c_int as ::core::ffi::c_double;
                } else {
                    *(*p).var.offset(0 as ::core::ffi::c_int as isize) = x_max
                        * pow(
                            0.9f64,
                            (i_0 - 255 as ::core::ffi::c_int) as ::core::ffi::c_double,
                        );
                    if i_0 & 1 as ::core::ffi::c_int != 0 {
                        *(*p).var.offset(0 as ::core::ffi::c_int as isize)
                            *= -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
                    }
                    if i_0 & 2 as ::core::ffi::c_int != 0 {
                        *(*p).var.offset(0 as ::core::ffi::c_int as isize) += low;
                    } else {
                        *(*p).var.offset(0 as ::core::ffi::c_int as isize) += high;
                    }
                }
                v_0 = eval_expr(p, (*e).param[0 as ::core::ffi::c_int as usize]);
                if v_0 <= 0 as ::core::ffi::c_int as ::core::ffi::c_double && v_0 > low_v
                {
                    low = *(*p).var.offset(0 as ::core::ffi::c_int as isize);
                    low_v = v_0;
                }
                if v_0 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    && v_0 < high_v
                {
                    high = *(*p).var.offset(0 as ::core::ffi::c_int as isize);
                    high_v = v_0;
                }
                if low >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    && high >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    j = 0 as ::core::ffi::c_int;
                    while j < 1000 as ::core::ffi::c_int {
                        *(*p).var.offset(0 as ::core::ffi::c_int as isize) = (low + high)
                            * 0.5f64;
                        if low == *(*p).var.offset(0 as ::core::ffi::c_int as isize)
                            || high == *(*p).var.offset(0 as ::core::ffi::c_int as isize)
                        {
                            break;
                        }
                        v_0 = eval_expr(p, (*e).param[0 as ::core::ffi::c_int as usize]);
                        if v_0 <= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            low = *(*p).var.offset(0 as ::core::ffi::c_int as isize);
                        }
                        if v_0 >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            high = *(*p).var.offset(0 as ::core::ffi::c_int as isize);
                        }
                        if if ::core::mem::size_of::<::core::ffi::c_double>() as usize
                            == ::core::mem::size_of::<::core::ffi::c_float>() as usize
                        {
                            (__FLOAT_BITS(v_0 as ::core::ffi::c_float)
                                & 0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint
                                > 0x7f800000 as ::core::ffi::c_int as ::core::ffi::c_uint)
                                as ::core::ffi::c_int
                        } else if ::core::mem::size_of::<::core::ffi::c_double>()
                            as usize
                            == ::core::mem::size_of::<::core::ffi::c_double>() as usize
                        {
                            (__DOUBLE_BITS(v_0)
                                & (1 as ::core::ffi::c_ulonglong).wrapping_neg()
                                    >> 1 as ::core::ffi::c_int
                                > (0x7ff as ::core::ffi::c_ulonglong)
                                    << 52 as ::core::ffi::c_int) as ::core::ffi::c_int
                        } else {
                            (__fpclassifyl(crate::rt::F128::new(v_0))
                                == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                        } != 0
                        {
                            high = v_0;
                            low = high;
                            break;
                        } else {
                            j += 1;
                        }
                    }
                    break;
                } else {
                    i_0 += 1;
                }
            }
            *(*p).var.offset(0 as ::core::ffi::c_int as isize) = var0_0;
            return if -low_v < high_v { low } else { high };
        }
        _ => {
            let mut d_3: ::core::ffi::c_double = eval_expr(
                p,
                (*e).param[0 as ::core::ffi::c_int as usize],
            );
            let mut d2: ::core::ffi::c_double = eval_expr(
                p,
                (*e).param[1 as ::core::ffi::c_int as usize],
            );
            match (*e).type_0 as ::core::ffi::c_int {
                10 => {
                    return (*e).value
                        * (d_3
                            - floor(
                                (if d2 != 0. {
                                    d_3 / d2
                                } else {
                                    d_3 * ::core::f32::INFINITY as ::core::ffi::c_double
                                }),
                            ) * d2);
                }
                35 => {
                    return (*e).value
                        * av_gcd(d_3 as int64_t, d2 as int64_t) as ::core::ffi::c_double;
                }
                11 => return (*e).value * (if d_3 > d2 { d_3 } else { d2 }),
                12 => return (*e).value * (if d_3 < d2 { d_3 } else { d2 }),
                13 => return (*e).value * (if d_3 == d2 { 1.0f64 } else { 0.0f64 }),
                14 => return (*e).value * (if d_3 > d2 { 1.0f64 } else { 0.0f64 }),
                15 => return (*e).value * (if d_3 >= d2 { 1.0f64 } else { 0.0f64 }),
                17 => return (*e).value * (if d_3 < d2 { 1.0f64 } else { 0.0f64 }),
                16 => return (*e).value * (if d_3 <= d2 { 1.0f64 } else { 0.0f64 }),
                18 => return (*e).value * pow(d_3, d2),
                19 => return (*e).value * (d_3 * d2),
                20 => {
                    return (*e).value
                        * (if d2 != 0. {
                            d_3 / d2
                        } else {
                            d_3 * ::core::f32::INFINITY as ::core::ffi::c_double
                        });
                }
                21 => return (*e).value * (d_3 + d2),
                22 => return (*e).value * d2,
                23 => {
                    let mut index: ::core::ffi::c_int = av_clip_c(
                        d_3 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
                    );
                    (*(*p).prng_state.offset(index as isize)).counter = 0 as uint64_t;
                    let ref mut fresh0 = *(*p).var.offset(index as isize);
                    *fresh0 = d2;
                    return (*e).value * *fresh0;
                }
                34 => return (*e).value * hypot(d_3, d2),
                43 => return (*e).value * atan2(d_3, d2),
                39 => {
                    return if (if ::core::mem::size_of::<::core::ffi::c_double>()
                        as usize
                        == ::core::mem::size_of::<::core::ffi::c_float>() as usize
                    {
                        (__FLOAT_BITS(d_3 as ::core::ffi::c_float)
                            & 0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint
                            > 0x7f800000 as ::core::ffi::c_int as ::core::ffi::c_uint)
                            as ::core::ffi::c_int
                    } else {
                        (if ::core::mem::size_of::<::core::ffi::c_double>() as usize
                            == ::core::mem::size_of::<::core::ffi::c_double>() as usize
                        {
                            (__DOUBLE_BITS(d_3)
                                & (1 as ::core::ffi::c_ulonglong).wrapping_neg()
                                    >> 1 as ::core::ffi::c_int
                                > (0x7ff as ::core::ffi::c_ulonglong)
                                    << 52 as ::core::ffi::c_int) as ::core::ffi::c_int
                        } else {
                            (__fpclassifyl(crate::rt::F128::new(d_3))
                                == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                        })
                    }) != 0
                        || (if ::core::mem::size_of::<::core::ffi::c_double>() as usize
                            == ::core::mem::size_of::<::core::ffi::c_float>() as usize
                        {
                            (__FLOAT_BITS(d2 as ::core::ffi::c_float)
                                & 0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint
                                > 0x7f800000 as ::core::ffi::c_int as ::core::ffi::c_uint)
                                as ::core::ffi::c_int
                        } else {
                            (if ::core::mem::size_of::<::core::ffi::c_double>() as usize
                                == ::core::mem::size_of::<::core::ffi::c_double>() as usize
                            {
                                (__DOUBLE_BITS(d2)
                                    & (1 as ::core::ffi::c_ulonglong).wrapping_neg()
                                        >> 1 as ::core::ffi::c_int
                                    > (0x7ff as ::core::ffi::c_ulonglong)
                                        << 52 as ::core::ffi::c_int) as ::core::ffi::c_int
                            } else {
                                (__fpclassifyl(crate::rt::F128::new(d2))
                                    == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                            })
                        }) != 0
                    {
                        ::core::f32::NAN as ::core::ffi::c_double
                    } else {
                        (*e).value
                            * (d_3 as ::core::ffi::c_long & d2 as ::core::ffi::c_long)
                                as ::core::ffi::c_double
                    };
                }
                40 => {
                    return if (if ::core::mem::size_of::<::core::ffi::c_double>()
                        as usize
                        == ::core::mem::size_of::<::core::ffi::c_float>() as usize
                    {
                        (__FLOAT_BITS(d_3 as ::core::ffi::c_float)
                            & 0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint
                            > 0x7f800000 as ::core::ffi::c_int as ::core::ffi::c_uint)
                            as ::core::ffi::c_int
                    } else {
                        (if ::core::mem::size_of::<::core::ffi::c_double>() as usize
                            == ::core::mem::size_of::<::core::ffi::c_double>() as usize
                        {
                            (__DOUBLE_BITS(d_3)
                                & (1 as ::core::ffi::c_ulonglong).wrapping_neg()
                                    >> 1 as ::core::ffi::c_int
                                > (0x7ff as ::core::ffi::c_ulonglong)
                                    << 52 as ::core::ffi::c_int) as ::core::ffi::c_int
                        } else {
                            (__fpclassifyl(crate::rt::F128::new(d_3))
                                == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                        })
                    }) != 0
                        || (if ::core::mem::size_of::<::core::ffi::c_double>() as usize
                            == ::core::mem::size_of::<::core::ffi::c_float>() as usize
                        {
                            (__FLOAT_BITS(d2 as ::core::ffi::c_float)
                                & 0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint
                                > 0x7f800000 as ::core::ffi::c_int as ::core::ffi::c_uint)
                                as ::core::ffi::c_int
                        } else {
                            (if ::core::mem::size_of::<::core::ffi::c_double>() as usize
                                == ::core::mem::size_of::<::core::ffi::c_double>() as usize
                            {
                                (__DOUBLE_BITS(d2)
                                    & (1 as ::core::ffi::c_ulonglong).wrapping_neg()
                                        >> 1 as ::core::ffi::c_int
                                    > (0x7ff as ::core::ffi::c_ulonglong)
                                        << 52 as ::core::ffi::c_int) as ::core::ffi::c_int
                            } else {
                                (__fpclassifyl(crate::rt::F128::new(d2))
                                    == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                            })
                        }) != 0
                    {
                        ::core::f32::NAN as ::core::ffi::c_double
                    } else {
                        (*e).value
                            * (d_3 as ::core::ffi::c_long | d2 as ::core::ffi::c_long)
                                as ::core::ffi::c_double
                    };
                }
                _ => {}
            }
        }
    }
    return ::core::f32::NAN as ::core::ffi::c_double;
}
#[inline]
unsafe extern "C" fn av_isspace(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (c == ' ' as i32 || c == '\u{c}' as i32 || c == '\n' as i32
        || c == '\r' as i32 || c == '\t' as i32 || c == '\u{b}' as i32)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_expr_free(mut e: *mut AVExpr) {
    if e.is_null() {
        return;
    }
    av_expr_free((*e).param[0 as ::core::ffi::c_int as usize]);
    av_expr_free((*e).param[1 as ::core::ffi::c_int as usize]);
    av_expr_free((*e).param[2 as ::core::ffi::c_int as usize]);
    if (*e).root != 0 {
        let mut r: *mut AVExprRoot = e as *mut AVExprRoot;
        av_freep(&raw mut (*r).var as *mut ::core::ffi::c_void);
        av_freep(&raw mut (*r).prng_state as *mut ::core::ffi::c_void);
    }
    av_freep(&raw mut e as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn parse_primary(
    mut e: *mut *mut AVExpr,
    mut p: *mut Parser,
) -> ::core::ffi::c_int {
    let mut d: *mut AVExpr = av_mallocz(::core::mem::size_of::<AVExpr>() as size_t)
        as *mut AVExpr;
    let mut next: *mut ::core::ffi::c_char = (*p).s;
    let mut s0: *mut ::core::ffi::c_char = (*p).s;
    let mut ret: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    if d.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    (*d).value = av_strtod((*p).s, &raw mut next);
    if next != (*p).s {
        (*d).type_0 = e_value as ::core::ffi::c_int as ::core::ffi::c_uchar;
        (*p).s = next;
        *e = d;
        return 0 as ::core::ffi::c_int;
    }
    (*d).value = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
    i = 0 as ::core::ffi::c_int;
    while !(*p).const_names.is_null()
        && !(*(*p).const_names.offset(i as isize)).is_null()
    {
        if strmatch((*p).s, *(*p).const_names.offset(i as isize)) != 0 {
            (*p).s = (*p)
                .s
                .offset(strlen(*(*p).const_names.offset(i as isize)) as isize);
            (*d).type_0 = e_const as ::core::ffi::c_int as ::core::ffi::c_uchar;
            (*d).const_index = i;
            *e = d;
            return 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while (i as usize)
        < (::core::mem::size_of::<[C2RustUnnamed_2; 4]>() as usize)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_2>() as usize)
    {
        if strmatch((*p).s, constants[i as usize].name) != 0 {
            (*p).s = (*p).s.offset(strlen(constants[i as usize].name) as isize);
            (*d).type_0 = e_value as ::core::ffi::c_int as ::core::ffi::c_uchar;
            (*d).value = constants[i as usize].value;
            *e = d;
            return 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    (*p).s = strchr((*p).s, '(' as i32);
    if (*p).s.is_null() {
        av_log(
            p as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"Undefined constant or missing '(' in '%s'\n\0" as *const u8
                as *const ::core::ffi::c_char,
            s0,
        );
        (*p).s = next;
        av_expr_free(d);
        return -(22 as ::core::ffi::c_int);
    }
    (*p).s = (*p).s.offset(1);
    if *next as ::core::ffi::c_int == '(' as i32 {
        av_freep(&raw mut d as *mut ::core::ffi::c_void);
        ret = parse_expr(&raw mut d, p);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        if *(*p).s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != ')' as i32
        {
            av_log(
                p as *mut ::core::ffi::c_void,
                16 as ::core::ffi::c_int,
                b"Missing ')' in '%s'\n\0" as *const u8 as *const ::core::ffi::c_char,
                s0,
            );
            av_expr_free(d);
            return -(22 as ::core::ffi::c_int);
        }
        (*p).s = (*p).s.offset(1);
        *e = d;
        return 0 as ::core::ffi::c_int;
    }
    ret = parse_expr(
        (&raw mut (*d).param as *mut *mut AVExpr)
            .offset(0 as ::core::ffi::c_int as isize) as *mut *mut AVExpr,
        p,
    );
    if ret < 0 as ::core::ffi::c_int {
        av_expr_free(d);
        return ret;
    }
    if *(*p).s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == ',' as i32
    {
        (*p).s = (*p).s.offset(1);
        parse_expr(
            (&raw mut (*d).param as *mut *mut AVExpr)
                .offset(1 as ::core::ffi::c_int as isize) as *mut *mut AVExpr,
            p,
        );
    }
    if *(*p).s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == ',' as i32
    {
        (*p).s = (*p).s.offset(1);
        parse_expr(
            (&raw mut (*d).param as *mut *mut AVExpr)
                .offset(2 as ::core::ffi::c_int as isize) as *mut *mut AVExpr,
            p,
        );
    }
    if *(*p).s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        != ')' as i32
    {
        av_log(
            p as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"Missing ')' or too many args in '%s'\n\0" as *const u8
                as *const ::core::ffi::c_char,
            s0,
        );
        av_expr_free(d);
        return -(22 as ::core::ffi::c_int);
    }
    (*p).s = (*p).s.offset(1);
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < 3 as ::core::ffi::c_int {
        if !(*d).param[i_0 as usize].is_null() {
            (*d).depth = (if (*d).depth as ::core::ffi::c_int
                > (*(*d).param[i_0 as usize]).depth as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int
            {
                (*d).depth as ::core::ffi::c_int
            } else {
                (*(*d).param[i_0 as usize]).depth as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int
            }) as ::core::ffi::c_short;
        }
        i_0 += 1;
    }
    if (*d).depth as ::core::ffi::c_int > 100 as ::core::ffi::c_int {
        av_expr_free(d);
        return -(22 as ::core::ffi::c_int);
    }
    (*d).type_0 = e_func0 as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if strmatch(next, b"sinh\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).c2rust_unnamed.func0 = Some(
            sinh as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
        )
            as Option<
                unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
            >;
    } else if strmatch(next, b"cosh\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).c2rust_unnamed.func0 = Some(
            cosh as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
        )
            as Option<
                unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
            >;
    } else if strmatch(next, b"tanh\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).c2rust_unnamed.func0 = Some(
            tanh as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
        )
            as Option<
                unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
            >;
    } else if strmatch(next, b"sin\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).c2rust_unnamed.func0 = Some(
            sin as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
        )
            as Option<
                unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
            >;
    } else if strmatch(next, b"cos\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).c2rust_unnamed.func0 = Some(
            cos as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
        )
            as Option<
                unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
            >;
    } else if strmatch(next, b"tan\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).c2rust_unnamed.func0 = Some(
            tan as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
        )
            as Option<
                unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
            >;
    } else if strmatch(next, b"atan\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).c2rust_unnamed.func0 = Some(
            atan as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
        )
            as Option<
                unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
            >;
    } else if strmatch(next, b"asin\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).c2rust_unnamed.func0 = Some(
            asin as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
        )
            as Option<
                unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
            >;
    } else if strmatch(next, b"acos\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).c2rust_unnamed.func0 = Some(
            acos as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
        )
            as Option<
                unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
            >;
    } else if strmatch(next, b"exp\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).c2rust_unnamed.func0 = Some(
            exp as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
        )
            as Option<
                unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
            >;
    } else if strmatch(next, b"log\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).c2rust_unnamed.func0 = Some(
            log as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
        )
            as Option<
                unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
            >;
    } else if strmatch(next, b"abs\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).c2rust_unnamed.func0 = Some(
            fabs as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
        )
            as Option<
                unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
            >;
    } else if strmatch(next, b"time\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).c2rust_unnamed.func0 = Some(
            etime as unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
        )
            as Option<
                unsafe extern "C" fn(::core::ffi::c_double) -> ::core::ffi::c_double,
            >;
    } else if strmatch(next, b"squish\0" as *const u8 as *const ::core::ffi::c_char) != 0
    {
        (*d).type_0 = e_squish as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"gauss\0" as *const u8 as *const ::core::ffi::c_char) != 0
    {
        (*d).type_0 = e_gauss as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"mod\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).type_0 = e_mod as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"max\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).type_0 = e_max as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"min\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).type_0 = e_min as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"eq\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).type_0 = e_eq as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"gte\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).type_0 = e_gte as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"gt\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).type_0 = e_gt as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"lte\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).type_0 = e_lte as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"lt\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).type_0 = e_lt as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"ld\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).type_0 = e_ld as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"isnan\0" as *const u8 as *const ::core::ffi::c_char) != 0
    {
        (*d).type_0 = e_isnan as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"isinf\0" as *const u8 as *const ::core::ffi::c_char) != 0
    {
        (*d).type_0 = e_isinf as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"st\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).type_0 = e_st as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"while\0" as *const u8 as *const ::core::ffi::c_char) != 0
    {
        (*d).type_0 = e_while as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"taylor\0" as *const u8 as *const ::core::ffi::c_char) != 0
    {
        (*d).type_0 = e_taylor as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"root\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).type_0 = e_root as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"floor\0" as *const u8 as *const ::core::ffi::c_char) != 0
    {
        (*d).type_0 = e_floor as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"ceil\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).type_0 = e_ceil as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"trunc\0" as *const u8 as *const ::core::ffi::c_char) != 0
    {
        (*d).type_0 = e_trunc as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"round\0" as *const u8 as *const ::core::ffi::c_char) != 0
    {
        (*d).type_0 = e_round as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"sqrt\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).type_0 = e_sqrt as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"not\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).type_0 = e_not as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"pow\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).type_0 = e_pow as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"print\0" as *const u8 as *const ::core::ffi::c_char) != 0
    {
        (*d).type_0 = e_print as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"random\0" as *const u8 as *const ::core::ffi::c_char) != 0
    {
        (*d).type_0 = e_random as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"randomi\0" as *const u8 as *const ::core::ffi::c_char)
        != 0
    {
        (*d).type_0 = e_randomi as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"hypot\0" as *const u8 as *const ::core::ffi::c_char) != 0
    {
        (*d).type_0 = e_hypot as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"gcd\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).type_0 = e_gcd as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"if\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).type_0 = e_if as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"ifnot\0" as *const u8 as *const ::core::ffi::c_char) != 0
    {
        (*d).type_0 = e_ifnot as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"bitand\0" as *const u8 as *const ::core::ffi::c_char) != 0
    {
        (*d).type_0 = e_bitand as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"bitor\0" as *const u8 as *const ::core::ffi::c_char) != 0
    {
        (*d).type_0 = e_bitor as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"between\0" as *const u8 as *const ::core::ffi::c_char)
        != 0
    {
        (*d).type_0 = e_between as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"clip\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).type_0 = e_clip as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"atan2\0" as *const u8 as *const ::core::ffi::c_char) != 0
    {
        (*d).type_0 = e_atan2 as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"lerp\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).type_0 = e_lerp as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else if strmatch(next, b"sgn\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*d).type_0 = e_sgn as ::core::ffi::c_int as ::core::ffi::c_uchar;
    } else {
        i = 0 as ::core::ffi::c_int;
        while !(*p).func1_names.is_null()
            && !(*(*p).func1_names.offset(i as isize)).is_null()
        {
            if strmatch(next, *(*p).func1_names.offset(i as isize)) != 0 {
                (*d).c2rust_unnamed.func1 = *(*p).funcs1.offset(i as isize);
                (*d).type_0 = e_func1 as ::core::ffi::c_int as ::core::ffi::c_uchar;
                (*d).const_index = i;
                *e = d;
                return 0 as ::core::ffi::c_int;
            }
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while !(*p).func2_names.is_null()
            && !(*(*p).func2_names.offset(i as isize)).is_null()
        {
            if strmatch(next, *(*p).func2_names.offset(i as isize)) != 0 {
                (*d).c2rust_unnamed.func2 = *(*p).funcs2.offset(i as isize);
                (*d).type_0 = e_func2 as ::core::ffi::c_int as ::core::ffi::c_uchar;
                (*d).const_index = i;
                *e = d;
                return 0 as ::core::ffi::c_int;
            }
            i += 1;
        }
        av_log(
            p as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"Unknown function in '%s'\n\0" as *const u8 as *const ::core::ffi::c_char,
            s0,
        );
        av_expr_free(d);
        return -(22 as ::core::ffi::c_int);
    }
    *e = d;
    return 0 as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn av_clipd_c(
    mut a: ::core::ffi::c_double,
    mut amin: ::core::ffi::c_double,
    mut amax: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    return if (if a > amin { a } else { amin }) > amax {
        amax
    } else if a > amin {
        a
    } else {
        amin
    };
}
unsafe extern "C" fn make_eval_expr(
    mut type_0: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
    mut p0: *mut AVExpr,
    mut p1: *mut AVExpr,
) -> *mut AVExpr {
    let mut depth: ::core::ffi::c_int = (if (*p0).depth as ::core::ffi::c_int
        > (*p1).depth as ::core::ffi::c_int
    {
        (*p0).depth as ::core::ffi::c_int
    } else {
        (*p1).depth as ::core::ffi::c_int
    }) + 1 as ::core::ffi::c_int;
    if depth > 100 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<AVExpr>();
    }
    let mut e: *mut AVExpr = av_mallocz(::core::mem::size_of::<AVExpr>() as size_t)
        as *mut AVExpr;
    if e.is_null() {
        return ::core::ptr::null_mut::<AVExpr>();
    }
    (*e).type_0 = type_0 as ::core::ffi::c_uchar;
    (*e).value = value as ::core::ffi::c_double;
    (*e).param[0 as ::core::ffi::c_int as usize] = p0 as *mut AVExpr;
    (*e).param[1 as ::core::ffi::c_int as usize] = p1 as *mut AVExpr;
    (*e).depth = depth as ::core::ffi::c_short;
    return e;
}
unsafe extern "C" fn parse_pow(
    mut e: *mut *mut AVExpr,
    mut p: *mut Parser,
    mut sign: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    *sign = (*(*p).s as ::core::ffi::c_int == '+' as i32) as ::core::ffi::c_int
        - (*(*p).s as ::core::ffi::c_int == '-' as i32) as ::core::ffi::c_int;
    (*p).s = (*p).s.offset((*sign & 1 as ::core::ffi::c_int) as isize);
    return parse_primary(e, p);
}
unsafe extern "C" fn parse_dB(
    mut e: *mut *mut AVExpr,
    mut p: *mut Parser,
    mut sign: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if *(*p).s as ::core::ffi::c_int == '-' as i32 {
        let mut next: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut ignored: ::core::ffi::c_double = strtod((*p).s, &raw mut next);
        if next != (*p).s
            && *next.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'd' as i32
            && *next.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'B' as i32
        {
            *sign = 0 as ::core::ffi::c_int;
            return parse_primary(e, p);
        }
    }
    return parse_pow(e, p, sign);
}
unsafe extern "C" fn parse_factor(
    mut e: *mut *mut AVExpr,
    mut p: *mut Parser,
) -> ::core::ffi::c_int {
    let mut sign: ::core::ffi::c_int = 0;
    let mut sign2: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut e0: *mut AVExpr = ::core::ptr::null_mut::<AVExpr>();
    let mut e1: *mut AVExpr = ::core::ptr::null_mut::<AVExpr>();
    let mut e2: *mut AVExpr = ::core::ptr::null_mut::<AVExpr>();
    ret = parse_dB(&raw mut e0, p, &raw mut sign);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    while *(*p).s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == '^' as i32
    {
        e1 = e0;
        (*p).s = (*p).s.offset(1);
        ret = parse_dB(&raw mut e2, p, &raw mut sign2);
        if ret < 0 as ::core::ffi::c_int {
            av_expr_free(e1);
            return ret;
        }
        e0 = make_eval_expr(
            e_pow as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            e1,
            e2,
        );
        if e0.is_null() {
            av_expr_free(e1);
            av_expr_free(e2);
            return -(12 as ::core::ffi::c_int);
        }
        if !(*e0).param[1 as ::core::ffi::c_int as usize].is_null() {
            (*(*e0).param[1 as ::core::ffi::c_int as usize]).value
                *= (sign2 | 1 as ::core::ffi::c_int) as ::core::ffi::c_double;
        }
    }
    if !e0.is_null() {
        (*e0).value *= (sign | 1 as ::core::ffi::c_int) as ::core::ffi::c_double;
    }
    *e = e0;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn parse_term(
    mut e: *mut *mut AVExpr,
    mut p: *mut Parser,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut e0: *mut AVExpr = ::core::ptr::null_mut::<AVExpr>();
    let mut e1: *mut AVExpr = ::core::ptr::null_mut::<AVExpr>();
    let mut e2: *mut AVExpr = ::core::ptr::null_mut::<AVExpr>();
    ret = parse_factor(&raw mut e0, p);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    while *(*p).s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == '*' as i32
        || *(*p).s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '/' as i32
    {
        let fresh6 = (*p).s;
        (*p).s = (*p).s.offset(1);
        let mut c: ::core::ffi::c_int = *fresh6 as ::core::ffi::c_int;
        e1 = e0;
        ret = parse_factor(&raw mut e2, p);
        if ret < 0 as ::core::ffi::c_int {
            av_expr_free(e1);
            return ret;
        }
        e0 = make_eval_expr(
            if c == '*' as i32 {
                e_mul as ::core::ffi::c_int
            } else {
                e_div as ::core::ffi::c_int
            },
            1 as ::core::ffi::c_int,
            e1,
            e2,
        );
        if e0.is_null() {
            av_expr_free(e1);
            av_expr_free(e2);
            return -(12 as ::core::ffi::c_int);
        }
    }
    *e = e0;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn parse_subexpr(
    mut e: *mut *mut AVExpr,
    mut p: *mut Parser,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut e0: *mut AVExpr = ::core::ptr::null_mut::<AVExpr>();
    let mut e1: *mut AVExpr = ::core::ptr::null_mut::<AVExpr>();
    let mut e2: *mut AVExpr = ::core::ptr::null_mut::<AVExpr>();
    ret = parse_term(&raw mut e0, p);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    while *(*p).s as ::core::ffi::c_int == '+' as i32
        || *(*p).s as ::core::ffi::c_int == '-' as i32
    {
        e1 = e0;
        ret = parse_term(&raw mut e2, p);
        if ret < 0 as ::core::ffi::c_int {
            av_expr_free(e1);
            return ret;
        }
        e0 = make_eval_expr(
            e_add as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            e1,
            e2,
        );
        if e0.is_null() {
            av_expr_free(e1);
            av_expr_free(e2);
            return -(12 as ::core::ffi::c_int);
        }
    }
    *e = e0;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn parse_expr(
    mut e: *mut *mut AVExpr,
    mut p: *mut Parser,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut e0: *mut AVExpr = ::core::ptr::null_mut::<AVExpr>();
    let mut e1: *mut AVExpr = ::core::ptr::null_mut::<AVExpr>();
    let mut e2: *mut AVExpr = ::core::ptr::null_mut::<AVExpr>();
    if (*p).stack_index <= 0 as ::core::ffi::c_int {
        return -(22 as ::core::ffi::c_int);
    }
    (*p).stack_index -= 1;
    ret = parse_subexpr(&raw mut e0, p);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    while *(*p).s as ::core::ffi::c_int == ';' as i32 {
        (*p).s = (*p).s.offset(1);
        e1 = e0;
        ret = parse_subexpr(&raw mut e2, p);
        if ret < 0 as ::core::ffi::c_int {
            av_expr_free(e1);
            return ret;
        }
        e0 = make_eval_expr(
            e_last as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            e1,
            e2,
        );
        if e0.is_null() {
            av_expr_free(e1);
            av_expr_free(e2);
            return -(12 as ::core::ffi::c_int);
        }
    }
    (*p).stack_index += 1;
    *e = e0;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn verify_expr(mut e: *mut AVExpr) -> ::core::ffi::c_int {
    if e.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    match (*e).type_0 as ::core::ffi::c_int {
        0 | 1 => return 1 as ::core::ffi::c_int,
        2 | 3 | 5 | 7 | 6 | 8 | 9 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 45 => {
            return (verify_expr((*e).param[0 as ::core::ffi::c_int as usize]) != 0
                && (*e).param[1 as ::core::ffi::c_int as usize].is_null())
                as ::core::ffi::c_int;
        }
        38 => {
            return (verify_expr((*e).param[0 as ::core::ffi::c_int as usize]) != 0
                && ((*e).param[1 as ::core::ffi::c_int as usize].is_null()
                    || verify_expr((*e).param[1 as ::core::ffi::c_int as usize]) != 0))
                as ::core::ffi::c_int;
        }
        36 | 37 | 25 => {
            return (verify_expr((*e).param[0 as ::core::ffi::c_int as usize]) != 0
                && verify_expr((*e).param[1 as ::core::ffi::c_int as usize]) != 0
                && ((*e).param[2 as ::core::ffi::c_int as usize].is_null()
                    || verify_expr((*e).param[2 as ::core::ffi::c_int as usize]) != 0))
                as ::core::ffi::c_int;
        }
        41 | 42 | 44 | 46 => {
            return (verify_expr((*e).param[0 as ::core::ffi::c_int as usize]) != 0
                && verify_expr((*e).param[1 as ::core::ffi::c_int as usize]) != 0
                && verify_expr((*e).param[2 as ::core::ffi::c_int as usize]) != 0)
                as ::core::ffi::c_int;
        }
        _ => {
            return (verify_expr((*e).param[0 as ::core::ffi::c_int as usize]) != 0
                && verify_expr((*e).param[1 as ::core::ffi::c_int as usize]) != 0
                && (*e).param[2 as ::core::ffi::c_int as usize].is_null())
                as ::core::ffi::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_expr_parse(
    mut expr: *mut *mut AVExpr,
    mut s: *const ::core::ffi::c_char,
    mut const_names: *const *const ::core::ffi::c_char,
    mut func1_names: *const *const ::core::ffi::c_char,
    mut funcs1: *const Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_double,
        ) -> ::core::ffi::c_double,
    >,
    mut func2_names: *const *const ::core::ffi::c_char,
    mut funcs2: *const Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
        ) -> ::core::ffi::c_double,
    >,
    mut log_offset: ::core::ffi::c_int,
    mut log_ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut r: *mut AVExprRoot = ::core::ptr::null_mut::<AVExprRoot>();
    let mut p: Parser = Parser {
        class: ::core::ptr::null::<AVClass>(),
        stack_index: 0,
        s: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        const_values: ::core::ptr::null::<::core::ffi::c_double>(),
        const_names: ::core::ptr::null::<*const ::core::ffi::c_char>(),
        funcs1: ::core::ptr::null::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_double,
                ) -> ::core::ffi::c_double,
            >,
        >(),
        func1_names: ::core::ptr::null::<*const ::core::ffi::c_char>(),
        funcs2: ::core::ptr::null::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> ::core::ffi::c_double,
            >,
        >(),
        func2_names: ::core::ptr::null::<*const ::core::ffi::c_char>(),
        opaque: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        log_offset: 0,
        log_ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        var: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        prng_state: ::core::ptr::null_mut::<FFSFC64>(),
    };
    let mut e: *mut AVExpr = ::core::ptr::null_mut::<AVExpr>();
    let mut w: *mut ::core::ffi::c_char = av_malloc(
        (strlen(s) as size_t).wrapping_add(1 as size_t),
    ) as *mut ::core::ffi::c_char;
    let mut wp: *mut ::core::ffi::c_char = w;
    let mut s0: *const ::core::ffi::c_char = s;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if w.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    while *s != 0 {
        let fresh3 = s;
        s = s.offset(1);
        if av_isspace(*fresh3 as ::core::ffi::c_int) == 0 {
            let fresh4 = wp;
            wp = wp.offset(1);
            *fresh4 = *s.offset(-(1 as ::core::ffi::c_int) as isize);
        }
    }
    let fresh5 = wp;
    wp = wp.offset(1);
    *fresh5 = 0 as ::core::ffi::c_char;
    p.class = &raw const eval_class;
    p.stack_index = 100 as ::core::ffi::c_int;
    p.s = w;
    p.const_names = const_names;
    p.funcs1 = funcs1;
    p.func1_names = func1_names;
    p.funcs2 = funcs2;
    p.func2_names = func2_names;
    p.log_offset = log_offset;
    p.log_ctx = log_ctx;
    ret = parse_expr(&raw mut e, &raw mut p);
    if !(ret < 0 as ::core::ffi::c_int) {
        if *p.s != 0 {
            av_log(
                &raw mut p as *mut ::core::ffi::c_void,
                16 as ::core::ffi::c_int,
                b"Invalid chars '%s' at the end of expression '%s'\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                p.s,
                s0,
            );
            ret = -(22 as ::core::ffi::c_int);
        } else if verify_expr(e) == 0 {
            ret = -(22 as ::core::ffi::c_int);
        } else {
            r = av_realloc(
                e as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<AVExprRoot>() as size_t,
            ) as *mut AVExprRoot;
            if r.is_null() {
                ret = -(12 as ::core::ffi::c_int);
            } else {
                e = r as *mut AVExpr;
                (*e).root = 1 as ::core::ffi::c_uchar;
                (*r).var = av_mallocz(
                    (::core::mem::size_of::<::core::ffi::c_double>() as size_t)
                        .wrapping_mul(10 as size_t),
                ) as *mut ::core::ffi::c_double;
                (*r).prng_state = av_mallocz(
                    (::core::mem::size_of::<FFSFC64>() as size_t)
                        .wrapping_mul(10 as size_t),
                ) as *mut FFSFC64;
                if (*r).var.is_null() || (*r).prng_state.is_null() {
                    ret = -(12 as ::core::ffi::c_int);
                } else {
                    *expr = e;
                    e = ::core::ptr::null_mut::<AVExpr>();
                }
            }
        }
    }
    av_expr_free(e);
    av_free(w as *mut ::core::ffi::c_void);
    return ret;
}
unsafe extern "C" fn expr_count(
    mut e: *mut AVExpr,
    mut counter: *mut ::core::ffi::c_uint,
    mut size: ::core::ffi::c_int,
    mut type_0: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    if e.is_null() || counter.is_null() || size == 0 {
        return -(22 as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    while (*e).type_0 as ::core::ffi::c_int != type_0 && i < 3 as ::core::ffi::c_int
        && !(*e).param[i as usize].is_null()
    {
        expr_count((*e).param[i as usize], counter, size, type_0);
        i += 1;
    }
    if (*e).type_0 as ::core::ffi::c_int == type_0 && (*e).const_index < size {
        let ref mut fresh7 = *counter.offset((*e).const_index as isize);
        *fresh7 = (*fresh7).wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_expr_count_vars(
    mut e: *mut AVExpr,
    mut counter: *mut ::core::ffi::c_uint,
    mut size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return expr_count(e, counter, size, e_const as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn av_expr_count_func(
    mut e: *mut AVExpr,
    mut counter: *mut ::core::ffi::c_uint,
    mut size: ::core::ffi::c_int,
    mut arg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return expr_count(
        e,
        counter,
        size,
        [
            e_const as ::core::ffi::c_int,
            e_func1 as ::core::ffi::c_int,
            e_func2 as ::core::ffi::c_int,
        ][arg as usize],
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_expr_eval(
    mut e: *mut AVExpr,
    mut const_values: *const ::core::ffi::c_double,
    mut opaque: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_double {
    let mut r: *mut AVExprRoot = e as *mut AVExprRoot;
    let mut p: Parser = Parser {
        class: &raw const eval_class,
        stack_index: 0,
        s: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        const_values: const_values,
        const_names: ::core::ptr::null::<*const ::core::ffi::c_char>(),
        funcs1: ::core::ptr::null::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_double,
                ) -> ::core::ffi::c_double,
            >,
        >(),
        func1_names: ::core::ptr::null::<*const ::core::ffi::c_char>(),
        funcs2: ::core::ptr::null::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> ::core::ffi::c_double,
            >,
        >(),
        func2_names: ::core::ptr::null::<*const ::core::ffi::c_char>(),
        opaque: opaque,
        log_offset: 0,
        log_ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        var: (*r).var,
        prng_state: (*r).prng_state,
    };
    return eval_expr(&raw mut p, e);
}
#[no_mangle]
pub unsafe extern "C" fn av_expr_parse_and_eval(
    mut d: *mut ::core::ffi::c_double,
    mut s: *const ::core::ffi::c_char,
    mut const_names: *const *const ::core::ffi::c_char,
    mut const_values: *const ::core::ffi::c_double,
    mut func1_names: *const *const ::core::ffi::c_char,
    mut funcs1: *const Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_double,
        ) -> ::core::ffi::c_double,
    >,
    mut func2_names: *const *const ::core::ffi::c_char,
    mut funcs2: *const Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
        ) -> ::core::ffi::c_double,
    >,
    mut opaque: *mut ::core::ffi::c_void,
    mut log_offset: ::core::ffi::c_int,
    mut log_ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut e: *mut AVExpr = ::core::ptr::null_mut::<AVExpr>();
    let mut ret: ::core::ffi::c_int = av_expr_parse(
        &raw mut e,
        s,
        const_names,
        func1_names,
        funcs1,
        func2_names,
        funcs2,
        log_offset,
        log_ctx,
    );
    if ret < 0 as ::core::ffi::c_int {
        *d = ::core::f32::NAN as ::core::ffi::c_double;
        return ret;
    }
    *d = av_expr_eval(e, const_values, opaque);
    av_expr_free(e);
    return if if ::core::mem::size_of::<::core::ffi::c_double>() as usize
        == ::core::mem::size_of::<::core::ffi::c_float>() as usize
    {
        (__FLOAT_BITS(*d as ::core::ffi::c_float)
            & 0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint
            > 0x7f800000 as ::core::ffi::c_int as ::core::ffi::c_uint)
            as ::core::ffi::c_int
    } else if ::core::mem::size_of::<::core::ffi::c_double>() as usize
        == ::core::mem::size_of::<::core::ffi::c_double>() as usize
    {
        (__DOUBLE_BITS(*d)
            & (1 as ::core::ffi::c_ulonglong).wrapping_neg() >> 1 as ::core::ffi::c_int
            > (0x7ff as ::core::ffi::c_ulonglong) << 52 as ::core::ffi::c_int)
            as ::core::ffi::c_int
    } else {
        (__fpclassifyl(crate::rt::F128::new(*d)) == 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int
    } != 0
    {
        -(22 as ::core::ffi::c_int)
    } else {
        0 as ::core::ffi::c_int
    };
}
#[repr(C)]
pub struct AVOptionRanges {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVOption {
    _opaque: [u8; 0],
}
