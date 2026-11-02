// Translated from FFmpeg 9.0.1 libavutil/log.c
// (preprocessed for armv7-unknown-linux-musleabihf) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// log functions
// Copyright (c) 2003 Michel Bardiaux
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
    fn av_gettime() -> int64_t;
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strstr(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn getenv(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    static stderr: *mut FILE;
    fn localtime_r(_: *const time_t, _: *mut tm) -> *mut tm;
    fn fputs(_: *const ::core::ffi::c_char, _: *mut FILE) -> ::core::ffi::c_int;
    fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn isatty(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn av_bprint_init(
        buf: *mut AVBPrint,
        size_init: ::core::ffi::c_uint,
        size_max: ::core::ffi::c_uint,
    );
    fn av_bprintf(buf: *mut AVBPrint, fmt: *const ::core::ffi::c_char, ...);
    fn av_vbprintf(
        buf: *mut AVBPrint,
        fmt: *const ::core::ffi::c_char,
        vl_arg: crate::rt::VaList,
    );
    fn av_bprint_strftime(
        buf: *mut AVBPrint,
        fmt: *const ::core::ffi::c_char,
        tm: *const tm,
    );
    fn av_bprint_finalize(
        buf: *mut AVBPrint,
        ret_str: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
pub type __builtin_va_list = __va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list {
    pub __ap: *mut ::core::ffi::c_void,
}
pub type size_t = ::core::ffi::c_uint;
pub type uintptr_t = ::core::ffi::c_uint;
pub type int64_t = ::core::ffi::c_longlong;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint32_t = ::core::ffi::c_uint;
pub type va_list = __builtin_va_list;
pub type memory_order = ::core::ffi::c_uint;
pub const memory_order_seq_cst: memory_order = 5;
pub const memory_order_acq_rel: memory_order = 4;
pub const memory_order_release: memory_order = 3;
pub const memory_order_acquire: memory_order = 2;
pub const memory_order_consume: memory_order = 1;
pub const memory_order_relaxed: memory_order = 0;
pub type atomic_int = ::core::ffi::c_int;
pub type atomic_uintptr_t = uintptr_t;
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBPrint {
    pub str_0: *mut ::core::ffi::c_char,
    pub len: ::core::ffi::c_uint,
    pub size: ::core::ffi::c_uint,
    pub size_max: ::core::ffi::c_uint,
    pub reserved_internal_buffer: [::core::ffi::c_char; 1],
    pub reserved_padding: [::core::ffi::c_char; 1004],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: ::core::ffi::c_int,
    pub tm_min: ::core::ffi::c_int,
    pub tm_hour: ::core::ffi::c_int,
    pub tm_mday: ::core::ffi::c_int,
    pub tm_mon: ::core::ffi::c_int,
    pub tm_year: ::core::ffi::c_int,
    pub tm_wday: ::core::ffi::c_int,
    pub tm_yday: ::core::ffi::c_int,
    pub tm_isdst: ::core::ffi::c_int,
    pub __tm_gmtoff: ::core::ffi::c_long,
    pub __tm_zone: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pthread_mutex_t {
    pub __u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __i: [::core::ffi::c_int; 6],
    pub __vi: [::core::ffi::c_int; 6],
    pub __p: [*mut ::core::ffi::c_void; 6],
}
pub type time_t = ::core::ffi::c_longlong;
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
static mut mutex: pthread_mutex_t = pthread_mutex_t {
    __u: C2RustUnnamed {
        __i: [0 as ::core::ffi::c_int; 6],
    },
};
static mut av_log_level: atomic_int = 32 as ::core::ffi::c_int;
static mut av_log_flags: atomic_int = 0 as ::core::ffi::c_int;
static mut color: [uint32_t; 62] = [
    ((52 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
        | (196 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        | 0x41 as ::core::ffi::c_int) as uint32_t,
    ((208 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int | 0x41 as ::core::ffi::c_int)
        as uint32_t,
    ((196 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int | 0x11 as ::core::ffi::c_int)
        as uint32_t,
    ((226 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int | 0x3 as ::core::ffi::c_int)
        as uint32_t,
    ((253 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int | 0x9 as ::core::ffi::c_int)
        as uint32_t,
    ((40 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int | 0x2 as ::core::ffi::c_int)
        as uint32_t,
    ((34 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int | 0x2 as ::core::ffi::c_int)
        as uint32_t,
    ((34 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int | 0x7 as ::core::ffi::c_int)
        as uint32_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    ((250 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int | 0x9 as ::core::ffi::c_int)
        as uint32_t,
    ((219 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int | 0x15 as ::core::ffi::c_int)
        as uint32_t,
    ((201 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int | 0x5 as ::core::ffi::c_int)
        as uint32_t,
    ((213 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int | 0x15 as ::core::ffi::c_int)
        as uint32_t,
    ((207 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int | 0x5 as ::core::ffi::c_int)
        as uint32_t,
    ((51 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int | 0x16 as ::core::ffi::c_int)
        as uint32_t,
    ((39 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int | 0x6 as ::core::ffi::c_int)
        as uint32_t,
    ((155 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int | 0x12 as ::core::ffi::c_int)
        as uint32_t,
    ((192 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int | 0x14 as ::core::ffi::c_int)
        as uint32_t,
    ((153 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int | 0x14 as ::core::ffi::c_int)
        as uint32_t,
    ((147 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int | 0x14 as ::core::ffi::c_int)
        as uint32_t,
    ((214 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int | 0x13 as ::core::ffi::c_int)
        as uint32_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    ((213 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int | 0x15 as ::core::ffi::c_int)
        as uint32_t,
    ((207 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int | 0x5 as ::core::ffi::c_int)
        as uint32_t,
    ((213 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int | 0x15 as ::core::ffi::c_int)
        as uint32_t,
    ((207 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int | 0x5 as ::core::ffi::c_int)
        as uint32_t,
    ((213 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int | 0x15 as ::core::ffi::c_int)
        as uint32_t,
    ((207 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int | 0x5 as ::core::ffi::c_int)
        as uint32_t,
];
static mut use_color: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
unsafe extern "C" fn check_color_terminal() {
    let mut term: *mut ::core::ffi::c_char = getenv(
        b"TERM\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if !getenv(b"AV_LOG_FORCE_NOCOLOR\0" as *const u8 as *const ::core::ffi::c_char)
        .is_null()
    {
        use_color = 0 as ::core::ffi::c_int;
    } else if !getenv(b"AV_LOG_FORCE_COLOR\0" as *const u8 as *const ::core::ffi::c_char)
        .is_null()
    {
        use_color = 1 as ::core::ffi::c_int;
    } else {
        use_color = (!term.is_null() && isatty(2 as ::core::ffi::c_int) != 0)
            as ::core::ffi::c_int;
    }
    if !getenv(b"AV_LOG_FORCE_256COLOR\0" as *const u8 as *const ::core::ffi::c_char)
        .is_null()
        || !term.is_null()
            && !strstr(term, b"256color\0" as *const u8 as *const ::core::ffi::c_char)
                .is_null()
    {
        use_color *= 256 as ::core::ffi::c_int;
    }
}
#[inline(always)]
unsafe extern "C" fn av_clip_c(
    mut a: ::core::ffi::c_int,
    mut amin: ::core::ffi::c_int,
    mut amax: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if a < amin { return amin } else if a > amax { return amax } else { return a };
}
unsafe extern "C" fn ansi_fputs(
    mut level: ::core::ffi::c_int,
    mut tint: ::core::ffi::c_int,
    mut str: *const ::core::ffi::c_char,
    mut local_use_color: ::core::ffi::c_int,
) {
    if local_use_color == 1 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"\x1B[%u;3%um%s\x1B[0m\0" as *const u8 as *const ::core::ffi::c_char,
            color[level as usize] >> 4 as ::core::ffi::c_int & 15 as uint32_t,
            color[level as usize] & 15 as uint32_t,
            str,
        );
    } else if tint != 0 && use_color == 256 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"\x1B[48;5;%um\x1B[38;5;%dm%s\x1B[0m\0" as *const u8
                as *const ::core::ffi::c_char,
            color[level as usize] >> 16 as ::core::ffi::c_int & 0xff as uint32_t,
            tint,
            str,
        );
    } else if local_use_color == 256 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"\x1B[48;5;%um\x1B[38;5;%um%s\x1B[0m\0" as *const u8
                as *const ::core::ffi::c_char,
            color[level as usize] >> 16 as ::core::ffi::c_int & 0xff as uint32_t,
            color[level as usize] >> 8 as ::core::ffi::c_int & 0xff as uint32_t,
            str,
        );
    } else {
        fputs(str, stderr);
    };
}
unsafe extern "C" fn colored_fputs(
    mut level: ::core::ffi::c_int,
    mut tint: ::core::ffi::c_int,
    mut str: *const ::core::ffi::c_char,
) {
    let mut local_use_color: ::core::ffi::c_int = 0;
    if *str == 0 {
        return;
    }
    if use_color < 0 as ::core::ffi::c_int {
        check_color_terminal();
    }
    if level == 32 as ::core::ffi::c_int / 8 as ::core::ffi::c_int {
        local_use_color = 0 as ::core::ffi::c_int;
    } else {
        local_use_color = use_color;
    }
    ansi_fputs(level, tint, str, local_use_color);
}
#[no_mangle]
pub unsafe extern "C" fn av_default_item_name(
    mut ptr: *mut ::core::ffi::c_void,
) -> *const ::core::ffi::c_char {
    return (**(ptr as *mut *mut AVClass)).class_name;
}
#[no_mangle]
pub unsafe extern "C" fn av_default_get_category(
    mut ptr: *mut ::core::ffi::c_void,
) -> AVClassCategory {
    return (**(ptr as *mut *mut AVClass)).category;
}
unsafe extern "C" fn sanitize(mut line: *mut uint8_t) {
    while *line != 0 {
        if (*line as ::core::ffi::c_int) < 0x8 as ::core::ffi::c_int
            || *line as ::core::ffi::c_int > 0xd as ::core::ffi::c_int
                && (*line as ::core::ffi::c_int) < 0x20 as ::core::ffi::c_int
        {
            *line = '?' as i32 as uint8_t;
        }
        line = line.offset(1);
    }
}
unsafe extern "C" fn get_category(
    mut ptr: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut avc: *mut AVClass = *(ptr as *mut *mut AVClass);
    if avc.is_null()
        || ((*avc).version & 0xff as ::core::ffi::c_int) < 100 as ::core::ffi::c_int
        || (*avc).version
            < (51 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
                | (59 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        || (*avc).category as ::core::ffi::c_uint
            >= AV_CLASS_CATEGORY_NB as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return AV_CLASS_CATEGORY_NA as ::core::ffi::c_int + 16 as ::core::ffi::c_int;
    }
    if (*avc).get_category.is_some() {
        return ((*avc).get_category.expect("non-null function pointer")(ptr)
            as ::core::ffi::c_uint)
            .wrapping_add(16 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    }
    return ((*avc).category as ::core::ffi::c_uint)
        .wrapping_add(16 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
unsafe extern "C" fn get_level_str(
    mut level: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    match level {
        -8 => return b"quiet\0" as *const u8 as *const ::core::ffi::c_char,
        48 => return b"debug\0" as *const u8 as *const ::core::ffi::c_char,
        56 => return b"trace\0" as *const u8 as *const ::core::ffi::c_char,
        40 => return b"verbose\0" as *const u8 as *const ::core::ffi::c_char,
        32 => return b"info\0" as *const u8 as *const ::core::ffi::c_char,
        24 => return b"warning\0" as *const u8 as *const ::core::ffi::c_char,
        16 => return b"error\0" as *const u8 as *const ::core::ffi::c_char,
        8 => return b"fatal\0" as *const u8 as *const ::core::ffi::c_char,
        0 => return b"panic\0" as *const u8 as *const ::core::ffi::c_char,
        _ => return b"\0" as *const u8 as *const ::core::ffi::c_char,
    };
}
unsafe extern "C" fn item_name(
    mut obj: *mut ::core::ffi::c_void,
    mut cls: *const AVClass,
) -> *const ::core::ffi::c_char {
    return if (*cls).item_name.is_some() {
        (*cls).item_name
            as Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                ) -> *const ::core::ffi::c_char,
            >
    } else {
        Some(
            av_default_item_name
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                ) -> *const ::core::ffi::c_char,
        )
    }
        .expect("non-null function pointer")(obj);
}
unsafe extern "C" fn format_date_now(
    mut bp_time: *mut AVBPrint,
    mut include_date: ::core::ffi::c_int,
) {
    let mut ptm: *mut tm = ::core::ptr::null_mut::<tm>();
    let mut tmbuf: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        __tm_gmtoff: 0,
        __tm_zone: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    let time_us: int64_t = av_gettime() as int64_t;
    let time_ms: int64_t = time_us / 1000 as int64_t;
    let time_s: time_t = time_ms as time_t / 1000 as time_t;
    let millisec: ::core::ffi::c_int = (time_ms as time_t - time_s * 1000 as time_t)
        as ::core::ffi::c_int;
    ptm = localtime_r(&raw const time_s, &raw mut tmbuf);
    if !ptm.is_null() {
        if include_date != 0 {
            av_bprint_strftime(
                bp_time,
                b"%Y-%m-%d \0" as *const u8 as *const ::core::ffi::c_char,
                ptm,
            );
        }
        av_bprint_strftime(
            bp_time,
            b"%H:%M:%S\0" as *const u8 as *const ::core::ffi::c_char,
            ptm,
        );
        av_bprintf(
            bp_time,
            b".%03d \0" as *const u8 as *const ::core::ffi::c_char,
            millisec,
        );
    }
}
unsafe extern "C" fn format_line(
    mut avcl: *mut ::core::ffi::c_void,
    mut level: ::core::ffi::c_int,
    mut fmt: *const ::core::ffi::c_char,
    mut vl: crate::rt::VaList,
    mut part: *mut AVBPrint,
    mut print_prefix: *mut ::core::ffi::c_int,
    mut type_0: *mut ::core::ffi::c_int,
) {
    let mut avc: *mut AVClass = if !avcl.is_null() {
        *(avcl as *mut *mut AVClass)
    } else {
        ::core::ptr::null_mut::<AVClass>()
    };
    av_bprint_init(
        part.offset(0 as ::core::ffi::c_int as isize),
        0 as ::core::ffi::c_uint,
        1 as ::core::ffi::c_uint,
    );
    av_bprint_init(
        part.offset(1 as ::core::ffi::c_int as isize),
        0 as ::core::ffi::c_uint,
        1 as ::core::ffi::c_uint,
    );
    av_bprint_init(
        part.offset(2 as ::core::ffi::c_int as isize),
        0 as ::core::ffi::c_uint,
        1 as ::core::ffi::c_uint,
    );
    av_bprint_init(
        part.offset(3 as ::core::ffi::c_int as isize),
        0 as ::core::ffi::c_uint,
        65536 as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    av_bprint_init(
        part.offset(4 as ::core::ffi::c_int as isize),
        0 as ::core::ffi::c_uint,
        1 as ::core::ffi::c_uint,
    );
    let mut flags: ::core::ffi::c_int = crate::rt::atomic_load_relaxed(
        &raw mut av_log_flags,
    );
    if !type_0.is_null() {
        let ref mut fresh0 = *type_0.offset(1 as ::core::ffi::c_int as isize);
        *fresh0 = AV_CLASS_CATEGORY_NA as ::core::ffi::c_int + 16 as ::core::ffi::c_int;
        *type_0.offset(0 as ::core::ffi::c_int as isize) = *fresh0;
    }
    if *print_prefix != 0 && !avc.is_null() {
        if (*avc).parent_log_context_offset != 0 {
            let mut parent: *mut *mut AVClass = *((avcl as *mut uint8_t)
                .offset((*avc).parent_log_context_offset as isize)
                as *mut *mut *mut AVClass);
            if !parent.is_null() && !(*parent).is_null() {
                av_bprintf(
                    part.offset(0 as ::core::ffi::c_int as isize),
                    b"[%s @ %p] \0" as *const u8 as *const ::core::ffi::c_char,
                    item_name(parent as *mut ::core::ffi::c_void, *parent),
                    parent,
                );
                if !type_0.is_null() {
                    *type_0.offset(0 as ::core::ffi::c_int as isize) = get_category(
                        parent as *mut ::core::ffi::c_void,
                    );
                }
            }
        }
        av_bprintf(
            part.offset(1 as ::core::ffi::c_int as isize),
            b"[%s @ %p] \0" as *const u8 as *const ::core::ffi::c_char,
            item_name(avcl, avc),
            avcl,
        );
        if !type_0.is_null() {
            *type_0.offset(1 as ::core::ffi::c_int as isize) = get_category(avcl);
        }
    }
    if *print_prefix != 0 && level > -(8 as ::core::ffi::c_int)
        && flags & (4 as ::core::ffi::c_int | 8 as ::core::ffi::c_int) != 0
    {
        format_date_now(
            part.offset(4 as ::core::ffi::c_int as isize) as *mut AVBPrint,
            flags & 8 as ::core::ffi::c_int,
        );
    }
    if *print_prefix != 0 && level > -(8 as ::core::ffi::c_int)
        && flags & 2 as ::core::ffi::c_int != 0
    {
        av_bprintf(
            part.offset(2 as ::core::ffi::c_int as isize),
            b"[%s] \0" as *const u8 as *const ::core::ffi::c_char,
            get_level_str(level),
        );
    }
    av_vbprintf(part.offset(3 as ::core::ffi::c_int as isize), fmt, vl.as_va_list());
    if *(*part.offset(0 as ::core::ffi::c_int as isize)).str_0 as ::core::ffi::c_int != 0
        || *(*part.offset(1 as ::core::ffi::c_int as isize)).str_0 as ::core::ffi::c_int
            != 0
        || *(*part.offset(2 as ::core::ffi::c_int as isize)).str_0 as ::core::ffi::c_int
            != 0
        || *(*part.offset(3 as ::core::ffi::c_int as isize)).str_0 as ::core::ffi::c_int
            != 0
    {
        let mut lastc: ::core::ffi::c_char = (if (*part
            .offset(3 as ::core::ffi::c_int as isize))
            .len != 0
            && (*part.offset(3 as ::core::ffi::c_int as isize)).len
                <= (*part.offset(3 as ::core::ffi::c_int as isize)).size
        {
            *(*part.offset(3 as ::core::ffi::c_int as isize))
                .str_0
                .offset(
                    (*part.offset(3 as ::core::ffi::c_int as isize))
                        .len
                        .wrapping_sub(1 as ::core::ffi::c_uint) as isize,
                ) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_char;
        *print_prefix = (lastc as ::core::ffi::c_int == '\n' as i32
            || lastc as ::core::ffi::c_int == '\r' as i32) as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn av_log_format_line(
    mut ptr: *mut ::core::ffi::c_void,
    mut level: ::core::ffi::c_int,
    mut fmt: *const ::core::ffi::c_char,
    mut vl: crate::rt::VaList,
    mut line: *mut ::core::ffi::c_char,
    mut line_size: ::core::ffi::c_int,
    mut print_prefix: *mut ::core::ffi::c_int,
) {
    av_log_format_line2(ptr, level, fmt, vl.as_va_list(), line, line_size, print_prefix);
}
#[no_mangle]
pub unsafe extern "C" fn av_log_format_line2(
    mut ptr: *mut ::core::ffi::c_void,
    mut level: ::core::ffi::c_int,
    mut fmt: *const ::core::ffi::c_char,
    mut vl: crate::rt::VaList,
    mut line: *mut ::core::ffi::c_char,
    mut line_size: ::core::ffi::c_int,
    mut print_prefix: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut part: [AVBPrint; 5] = [AVBPrint {
        str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        len: 0,
        size: 0,
        size_max: 0,
        reserved_internal_buffer: [0; 1],
        reserved_padding: [0; 1004],
    }; 5];
    let mut ret: ::core::ffi::c_int = 0;
    format_line(
        ptr,
        level,
        fmt,
        vl.as_va_list(),
        &raw mut part as *mut AVBPrint,
        print_prefix,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    ret = snprintf(
        line,
        line_size as size_t,
        b"%s%s%s%s\0" as *const u8 as *const ::core::ffi::c_char,
        part[0 as ::core::ffi::c_int as usize].str_0,
        part[1 as ::core::ffi::c_int as usize].str_0,
        part[2 as ::core::ffi::c_int as usize].str_0,
        part[3 as ::core::ffi::c_int as usize].str_0,
    );
    av_bprint_finalize(
        (&raw mut part as *mut AVBPrint).offset(3 as ::core::ffi::c_int as isize),
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_log_default_callback(
    mut ptr: *mut ::core::ffi::c_void,
    mut level: ::core::ffi::c_int,
    mut fmt: *const ::core::ffi::c_char,
    mut vl: crate::rt::VaList,
) {
    static mut print_prefix: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    static mut count: ::core::ffi::c_int = 0;
    static mut prev: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut part: [AVBPrint; 5] = [AVBPrint {
        str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        len: 0,
        size: 0,
        size_max: 0,
        reserved_internal_buffer: [0; 1],
        reserved_padding: [0; 1004],
    }; 5];
    let mut line: [::core::ffi::c_char; 1024] = [0; 1024];
    static mut is_atty: ::core::ffi::c_int = 0;
    let mut type_0: [::core::ffi::c_int; 2] = [0; 2];
    let mut tint: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    if level >= 0 as ::core::ffi::c_int {
        tint = (level & 0xff00 as ::core::ffi::c_int) as ::core::ffi::c_uint;
        level &= 0xff as ::core::ffi::c_int;
    }
    if level > crate::rt::atomic_load_relaxed(&raw mut av_log_level) {
        return;
    }
    pthread_mutex_lock(&raw mut mutex);
    format_line(
        ptr,
        level,
        fmt,
        vl.as_va_list(),
        &raw mut part as *mut AVBPrint,
        &raw mut print_prefix,
        &raw mut type_0 as *mut ::core::ffi::c_int,
    );
    snprintf(
        &raw mut line as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
        b"%s%s%s%s\0" as *const u8 as *const ::core::ffi::c_char,
        part[0 as ::core::ffi::c_int as usize].str_0,
        part[1 as ::core::ffi::c_int as usize].str_0,
        part[2 as ::core::ffi::c_int as usize].str_0,
        part[3 as ::core::ffi::c_int as usize].str_0,
    );
    if is_atty == 0 {
        is_atty = if isatty(2 as ::core::ffi::c_int) != 0 {
            1 as ::core::ffi::c_int
        } else {
            -(1 as ::core::ffi::c_int)
        };
    }
    if print_prefix != 0
        && crate::rt::atomic_load_relaxed(&raw mut av_log_flags)
            & 1 as ::core::ffi::c_int != 0
        && strcmp(
            &raw mut line as *mut ::core::ffi::c_char,
            &raw mut prev as *mut ::core::ffi::c_char,
        ) == 0 && *(&raw mut line as *mut ::core::ffi::c_char) as ::core::ffi::c_int != 0
        && line[strlen(&raw mut line as *mut ::core::ffi::c_char)
            .wrapping_sub(1 as size_t) as usize] as ::core::ffi::c_int != '\r' as i32
    {
        count += 1;
        if is_atty == 1 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"    Last message repeated %d times\r\0" as *const u8
                    as *const ::core::ffi::c_char,
                count,
            );
        }
    } else {
        if count > 0 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"    Last message repeated %d times\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                count,
            );
            count = 0 as ::core::ffi::c_int;
        }
        strcpy(
            &raw mut prev as *mut ::core::ffi::c_char,
            &raw mut line as *mut ::core::ffi::c_char,
        );
        sanitize(part[4 as ::core::ffi::c_int as usize].str_0 as *mut uint8_t);
        colored_fputs(
            7 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            part[4 as ::core::ffi::c_int as usize].str_0,
        );
        sanitize(part[0 as ::core::ffi::c_int as usize].str_0 as *mut uint8_t);
        colored_fputs(
            type_0[0 as ::core::ffi::c_int as usize],
            0 as ::core::ffi::c_int,
            part[0 as ::core::ffi::c_int as usize].str_0,
        );
        sanitize(part[1 as ::core::ffi::c_int as usize].str_0 as *mut uint8_t);
        colored_fputs(
            type_0[1 as ::core::ffi::c_int as usize],
            0 as ::core::ffi::c_int,
            part[1 as ::core::ffi::c_int as usize].str_0,
        );
        sanitize(part[2 as ::core::ffi::c_int as usize].str_0 as *mut uint8_t);
        colored_fputs(
            av_clip_c(
                level >> 3 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
            ),
            (tint >> 8 as ::core::ffi::c_int) as ::core::ffi::c_int,
            part[2 as ::core::ffi::c_int as usize].str_0,
        );
        sanitize(part[3 as ::core::ffi::c_int as usize].str_0 as *mut uint8_t);
        colored_fputs(
            av_clip_c(
                level >> 3 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
            ),
            (tint >> 8 as ::core::ffi::c_int) as ::core::ffi::c_int,
            part[3 as ::core::ffi::c_int as usize].str_0,
        );
    }
    av_bprint_finalize(
        (&raw mut part as *mut AVBPrint).offset(3 as ::core::ffi::c_int as isize),
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    pthread_mutex_unlock(&raw mut mutex);
}
extern "C" {
    // asm/varargs-arm.S: builds the va_list, calls av_log__va
    pub fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_log__va(
    mut avcl: *mut ::core::ffi::c_void,
    mut level: ::core::ffi::c_int,
    mut fmt: *const ::core::ffi::c_char,
    mut args: crate::rt::VaList,
) {
    let mut vl: crate::rt::VaList;
    vl = args.clone();
    av_vlog(avcl, level, fmt, vl.as_va_list());
}
extern "C" {
    // asm/varargs-arm.S: builds the va_list, calls av_log_once__va
    pub fn av_log_once(
        avcl: *mut ::core::ffi::c_void,
        initial_level: ::core::ffi::c_int,
        subsequent_level: ::core::ffi::c_int,
        state: *mut ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_log_once__va(
    mut avcl: *mut ::core::ffi::c_void,
    mut initial_level: ::core::ffi::c_int,
    mut subsequent_level: ::core::ffi::c_int,
    mut state: *mut ::core::ffi::c_int,
    mut fmt: *const ::core::ffi::c_char,
    mut args: crate::rt::VaList,
) {
    let mut vl: crate::rt::VaList;
    vl = args.clone();
    av_vlog(
        avcl,
        if *state != 0 { subsequent_level } else { initial_level },
        fmt,
        vl.as_va_list(),
    );
    *state = 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_vlog(
    mut avcl: *mut ::core::ffi::c_void,
    mut level: ::core::ffi::c_int,
    mut fmt: *const ::core::ffi::c_char,
    mut vl: crate::rt::VaList,
) {
    let mut avc: *mut AVClass = if !avcl.is_null() {
        *(avcl as *mut *mut AVClass)
    } else {
        ::core::ptr::null_mut::<AVClass>()
    };
    let mut log_callback: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            crate::rt::VaList,
        ) -> (),
    > = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                crate::rt::VaList,
            ) -> (),
        >,
    >(
        crate::rt::atomic_load_relaxed(&raw mut av_log_callback)
            as *mut ::core::ffi::c_void,
    );
    if !avc.is_null()
        && (*avc).version
            >= (50 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
                | (15 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                | 2 as ::core::ffi::c_int && (*avc).log_level_offset_offset != 0
        && level >= 8 as ::core::ffi::c_int
    {
        level
            += *((avcl as *mut uint8_t).offset((*avc).log_level_offset_offset as isize)
                as *mut ::core::ffi::c_int);
    }
    if log_callback.is_some() {
        log_callback
            .expect("non-null function pointer")(avcl, level, fmt, vl.as_va_list());
    }
}
#[no_mangle]
pub unsafe extern "C" fn av_log_get_level() -> ::core::ffi::c_int {
    return crate::rt::atomic_load_relaxed(&raw mut av_log_level);
}
#[no_mangle]
pub unsafe extern "C" fn av_log_set_level(mut level: ::core::ffi::c_int) {
    crate::rt::atomic_store_relaxed(&raw mut av_log_level, level);
}
#[no_mangle]
pub unsafe extern "C" fn av_log_set_flags(mut arg: ::core::ffi::c_int) {
    crate::rt::atomic_store_relaxed(&raw mut av_log_flags, arg);
}
#[no_mangle]
pub unsafe extern "C" fn av_log_get_flags() -> ::core::ffi::c_int {
    return crate::rt::atomic_load_relaxed(&raw mut av_log_flags);
}
#[no_mangle]
pub unsafe extern "C" fn av_log_set_callback(
    mut callback: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            crate::rt::VaList,
        ) -> (),
    >,
) {
    crate::rt::atomic_store_relaxed(
        &raw mut av_log_callback,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    crate::rt::VaList,
                ) -> (),
            >,
            *mut ::core::ffi::c_void,
        >(callback),
    );
}
unsafe extern "C" fn missing_feature_sample(
    mut sample: ::core::ffi::c_int,
    mut avc: *mut ::core::ffi::c_void,
    mut msg: *const ::core::ffi::c_char,
    mut argument_list: crate::rt::VaList,
) {
    av_vlog(avc, 24 as ::core::ffi::c_int, msg, argument_list.as_va_list());
    av_log(
        avc,
        24 as ::core::ffi::c_int,
        b" is not implemented. Update your FFmpeg version to the newest one from Git. If the problem still occurs, it means that your file has a feature which has not been implemented.\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    if sample != 0 {
        av_log(
            avc,
            24 as ::core::ffi::c_int,
            b"If you want to help, upload a sample of this file to https://streams.videolan.org/upload/ and contact the ffmpeg-devel mailing list. (ffmpeg-devel@ffmpeg.org)\n\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
}
extern "C" {
    // asm/varargs-arm.S: builds the va_list, calls avpriv_request_sample__va
    pub fn avpriv_request_sample(
        avc: *mut ::core::ffi::c_void,
        msg: *const ::core::ffi::c_char,
        ...
    );
}
#[no_mangle]
pub unsafe extern "C" fn avpriv_request_sample__va(
    mut avc: *mut ::core::ffi::c_void,
    mut msg: *const ::core::ffi::c_char,
    mut args: crate::rt::VaList,
) {
    let mut argument_list: crate::rt::VaList;
    argument_list = args.clone();
    missing_feature_sample(
        1 as ::core::ffi::c_int,
        avc,
        msg,
        argument_list.as_va_list(),
    );
}
extern "C" {
    // asm/varargs-arm.S: builds the va_list, calls avpriv_report_missing_feature__va
    pub fn avpriv_report_missing_feature(
        avc: *mut ::core::ffi::c_void,
        msg: *const ::core::ffi::c_char,
        ...
    );
}
#[no_mangle]
pub unsafe extern "C" fn avpriv_report_missing_feature__va(
    mut avc: *mut ::core::ffi::c_void,
    mut msg: *const ::core::ffi::c_char,
    mut args: crate::rt::VaList,
) {
    let mut argument_list: crate::rt::VaList;
    argument_list = args.clone();
    missing_feature_sample(
        0 as ::core::ffi::c_int,
        avc,
        msg,
        argument_list.as_va_list(),
    );
}
// c2rust: "Unsupported default initializer: Atomic". Holds a function
// pointer (uintptr_t in C); a pointer type keeps it a link-time constant.
static mut av_log_callback: *mut ::core::ffi::c_void = av_log_default_callback
    as unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_int,
        *const ::core::ffi::c_char, crate::rt::VaList)
    as *mut ::core::ffi::c_void;
#[repr(C)]
pub struct _IO_FILE {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVOptionRanges {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVOption {
    _opaque: [u8; 0],
}
