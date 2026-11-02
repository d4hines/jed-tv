// Translated from FFmpeg 9.0.1 libavutil/cpu.c
// (preprocessed for aarch64-unknown-linux-musl) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
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
    fn getauxval(_: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn ff_get_cpu_flags_aarch64() -> ::core::ffi::c_int;
    fn ff_get_cpu_max_align_aarch64() -> size_t;
    fn __sched_cpucount(_: size_t, _: *const cpu_set_t) -> ::core::ffi::c_int;
    fn sched_getaffinity(_: pid_t, _: size_t, _: *mut cpu_set_t) -> ::core::ffi::c_int;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn av_default_item_name(ctx: *mut ::core::ffi::c_void) -> *const ::core::ffi::c_char;
    fn av_opt_eval_flags(
        obj: *mut ::core::ffi::c_void,
        o: *const AVOption,
        val: *const ::core::ffi::c_char,
        flags_out: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
pub type size_t = ::core::ffi::c_ulong;
pub type pid_t = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpu_set_t {
    pub __bits: [::core::ffi::c_ulong; 16],
}
pub type int64_t = ::core::ffi::c_long;
pub type memory_order = ::core::ffi::c_uint;
pub const memory_order_seq_cst: memory_order = 5;
pub const memory_order_acq_rel: memory_order = 4;
pub const memory_order_release: memory_order = 3;
pub const memory_order_acquire: memory_order = 2;
pub const memory_order_consume: memory_order = 1;
pub const memory_order_relaxed: memory_order = 0;
pub type atomic_int = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVOption {
    pub name: *const ::core::ffi::c_char,
    pub help: *const ::core::ffi::c_char,
    pub offset: ::core::ffi::c_int,
    pub type_0: AVOptionType,
    pub default_val: C2RustUnnamed,
    pub min: ::core::ffi::c_double,
    pub max: ::core::ffi::c_double,
    pub flags: ::core::ffi::c_int,
    pub unit: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub i64_0: int64_t,
    pub dbl: ::core::ffi::c_double,
    pub str_0: *const ::core::ffi::c_char,
    pub q: AVRational,
    pub arr: *const AVOptionArrayDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVOptionArrayDef {
    pub def: *const ::core::ffi::c_char,
    pub size_min: ::core::ffi::c_uint,
    pub size_max: ::core::ffi::c_uint,
    pub sep: ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: ::core::ffi::c_int,
    pub den: ::core::ffi::c_int,
}
pub type AVOptionType = ::core::ffi::c_uint;
pub const AV_OPT_TYPE_FLAG_ARRAY: AVOptionType = 65536;
pub const AV_OPT_TYPE_UINT: AVOptionType = 20;
pub const AV_OPT_TYPE_CHLAYOUT: AVOptionType = 19;
pub const AV_OPT_TYPE_BOOL: AVOptionType = 18;
pub const AV_OPT_TYPE_COLOR: AVOptionType = 17;
pub const AV_OPT_TYPE_DURATION: AVOptionType = 16;
pub const AV_OPT_TYPE_VIDEO_RATE: AVOptionType = 15;
pub const AV_OPT_TYPE_SAMPLE_FMT: AVOptionType = 14;
pub const AV_OPT_TYPE_PIXEL_FMT: AVOptionType = 13;
pub const AV_OPT_TYPE_IMAGE_SIZE: AVOptionType = 12;
pub const AV_OPT_TYPE_CONST: AVOptionType = 11;
pub const AV_OPT_TYPE_UINT64: AVOptionType = 10;
pub const AV_OPT_TYPE_DICT: AVOptionType = 9;
pub const AV_OPT_TYPE_BINARY: AVOptionType = 8;
pub const AV_OPT_TYPE_RATIONAL: AVOptionType = 7;
pub const AV_OPT_TYPE_STRING: AVOptionType = 6;
pub const AV_OPT_TYPE_FLOAT: AVOptionType = 5;
pub const AV_OPT_TYPE_DOUBLE: AVOptionType = 4;
pub const AV_OPT_TYPE_INT64: AVOptionType = 3;
pub const AV_OPT_TYPE_INT: AVOptionType = 2;
pub const AV_OPT_TYPE_FLAGS: AVOptionType = 1;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVOptionRanges {
    pub range: *mut *mut AVOptionRange,
    pub nb_ranges: ::core::ffi::c_int,
    pub nb_components: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVOptionRange {
    pub str_0: *const ::core::ffi::c_char,
    pub value_min: ::core::ffi::c_double,
    pub value_max: ::core::ffi::c_double,
    pub component_min: ::core::ffi::c_double,
    pub component_max: ::core::ffi::c_double,
    pub is_range: ::core::ffi::c_int,
}
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
static mut cpu_flags: atomic_int = -(1 as ::core::ffi::c_int);
static mut cpu_count: atomic_int = -(1 as ::core::ffi::c_int);
unsafe extern "C" fn get_cpu_flags() -> ::core::ffi::c_int {
    return ff_get_cpu_flags_aarch64();
}
#[no_mangle]
pub unsafe extern "C" fn av_force_cpu_flags(mut arg: ::core::ffi::c_int) {
    if 0 as ::core::ffi::c_int != 0
        && arg
            & (0x4 as ::core::ffi::c_int | 0x20 as ::core::ffi::c_int
                | 0x2 as ::core::ffi::c_int | 0x8 as ::core::ffi::c_int
                | 0x10 as ::core::ffi::c_int | 0x40000000 as ::core::ffi::c_int
                | 0x40 as ::core::ffi::c_int | 0x20000000 as ::core::ffi::c_int
                | 0x80 as ::core::ffi::c_int | 0x100 as ::core::ffi::c_int
                | 0x200 as ::core::ffi::c_int | 0x4000 as ::core::ffi::c_int
                | 0x8000000 as ::core::ffi::c_int | 0x400 as ::core::ffi::c_int
                | 0x10000 as ::core::ffi::c_int | 0x800 as ::core::ffi::c_int
                | 0x8000 as ::core::ffi::c_int | 0x100000 as ::core::ffi::c_int) != 0
        && arg & 0x1 as ::core::ffi::c_int == 0
    {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            24 as ::core::ffi::c_int,
            b"MMX implied by specified flags\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        arg |= 0x1 as ::core::ffi::c_int;
    }
    crate::rt::atomic_store_relaxed(&raw mut cpu_flags, arg);
}
#[no_mangle]
pub unsafe extern "C" fn av_get_cpu_flags() -> ::core::ffi::c_int {
    let mut flags: ::core::ffi::c_int = crate::rt::atomic_load_relaxed(
        &raw mut cpu_flags,
    );
    if flags == -(1 as ::core::ffi::c_int) {
        flags = get_cpu_flags();
        crate::rt::atomic_store_relaxed(&raw mut cpu_flags, flags);
    }
    return flags;
}
#[no_mangle]
pub unsafe extern "C" fn av_parse_cpu_caps(
    mut flags: *mut ::core::ffi::c_uint,
    mut s: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    static mut cpuflags_opts: [AVOption; 15] = [
        AVOption {
            name: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_FLAGS,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(1 as ::core::ffi::c_int) as ::core::ffi::c_long
                - 0x7fffffffffffffff as ::core::ffi::c_long) as ::core::ffi::c_double,
            max: 0x7fffffffffffffff as ::core::ffi::c_long as ::core::ffi::c_double,
            flags: 0,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"armv8\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as int64_t,
            },
            min: 0.,
            max: 0.,
            flags: 0,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"neon\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as int64_t,
            },
            min: 0.,
            max: 0.,
            flags: 0,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"vfp\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as int64_t,
            },
            min: 0.,
            max: 0.,
            flags: 0,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"dotprod\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as int64_t,
            },
            min: 0.,
            max: 0.,
            flags: 0,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"i8mm\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as int64_t,
            },
            min: 0.,
            max: 0.,
            flags: 0,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"sve\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int) as int64_t,
            },
            min: 0.,
            max: 0.,
            flags: 0,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"sve2\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int) as int64_t,
            },
            min: 0.,
            max: 0.,
            flags: 0,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"sme\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int) as int64_t,
            },
            min: 0.,
            max: 0.,
            flags: 0,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"crc\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as int64_t,
            },
            min: 0.,
            max: 0.,
            flags: 0,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"sme_i16i64\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as int64_t,
            },
            min: 0.,
            max: 0.,
            flags: 0,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"sme2\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int) as int64_t,
            },
            min: 0.,
            max: 0.,
            flags: 0,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"pmull\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int) as int64_t,
            },
            min: 0.,
            max: 0.,
            flags: 0,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"eor3\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 18 as ::core::ffi::c_int) as int64_t,
            },
            min: 0.,
            max: 0.,
            flags: 0,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0,
            type_0: 0 as AVOptionType,
            default_val: C2RustUnnamed { i64_0: 0 },
            min: 0.,
            max: 0.,
            flags: 0,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
    ];
    static mut class: AVClass = unsafe {
        AVClass {
            class_name: b"cpuflags\0" as *const u8 as *const ::core::ffi::c_char,
            item_name: Some(
                av_default_item_name
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                    ) -> *const ::core::ffi::c_char,
            ),
            option: &raw const cpuflags_opts as *const AVOption,
            version: (61 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                | 101 as ::core::ffi::c_int,
            log_level_offset_offset: 0,
            parent_log_context_offset: 0,
            category: AV_CLASS_CATEGORY_NA,
            get_category: None,
            query_ranges: None,
            child_next: None,
            child_class_iterate: None,
            state_flags_offset: 0,
        }
    };
    let mut pclass: *const AVClass = &raw const class;
    return av_opt_eval_flags(
        &raw mut pclass as *mut ::core::ffi::c_void,
        (&raw const cpuflags_opts as *const AVOption)
            .offset(0 as ::core::ffi::c_int as isize) as *const AVOption,
        s,
        flags as *mut ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_cpu_count() -> ::core::ffi::c_int {
    static mut printed: atomic_int = 0 as ::core::ffi::c_int;
    let mut nb_cpus: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut cpuset: cpu_set_t = cpu_set_t { __bits: [0; 16] };
    memset(
        &raw mut cpuset as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<cpu_set_t>() as size_t,
    );
    if sched_getaffinity(
        0 as pid_t,
        ::core::mem::size_of::<cpu_set_t>() as size_t,
        &raw mut cpuset,
    ) == 0
    {
        nb_cpus = __sched_cpucount(
            ::core::mem::size_of::<cpu_set_t>() as size_t,
            &raw mut cpuset,
        );
    }
    if crate::rt::atomic_xchg_relaxed(&raw mut printed, 1 as ::core::ffi::c_int)
        == 0
    {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            48 as ::core::ffi::c_int,
            b"detected %d logical cores\n\0" as *const u8 as *const ::core::ffi::c_char,
            nb_cpus,
        );
    }
    count = crate::rt::atomic_load_relaxed(&raw mut cpu_count);
    if count > 0 as ::core::ffi::c_int {
        nb_cpus = count;
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            48 as ::core::ffi::c_int,
            b"overriding to %d logical cores\n\0" as *const u8
                as *const ::core::ffi::c_char,
            nb_cpus,
        );
    }
    return nb_cpus;
}
#[no_mangle]
pub unsafe extern "C" fn av_cpu_force_count(mut count: ::core::ffi::c_int) {
    crate::rt::atomic_store_relaxed(&raw mut cpu_count, count);
}
#[no_mangle]
pub unsafe extern "C" fn av_cpu_max_align() -> size_t {
    return ff_get_cpu_max_align_aarch64();
}
#[no_mangle]
pub unsafe extern "C" fn ff_getauxval(
    mut type_0: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    return getauxval(type_0);
}
