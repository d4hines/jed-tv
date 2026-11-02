// Translated from FFmpeg 9.0.1 libavutil/parseutils.c
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
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn av_gettime() -> int64_t;
    fn strtod(
        _: *const ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
    fn strtol(
        _: *const ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn strtoul(
        _: *const ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    fn strtoll(
        _: *const ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_longlong;
    fn av_get_random_seed() -> uint32_t;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn strchr(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strspn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    fn av_expr_parse_and_eval(
        res: *mut ::core::ffi::c_double,
        s: *const ::core::ffi::c_char,
        const_names: *const *const ::core::ffi::c_char,
        const_values: *const ::core::ffi::c_double,
        func1_names: *const *const ::core::ffi::c_char,
        funcs1: *const Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_double,
            ) -> ::core::ffi::c_double,
        >,
        func2_names: *const *const ::core::ffi::c_char,
        funcs2: *const Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_double,
                ::core::ffi::c_double,
            ) -> ::core::ffi::c_double,
        >,
        opaque: *mut ::core::ffi::c_void,
        log_offset: ::core::ffi::c_int,
        log_ctx: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn mktime(_: *mut tm) -> time_t;
    fn bsearch(
        _: *const ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
        _: size_t,
        _: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    ) -> *mut ::core::ffi::c_void;
    fn gmtime_r(_: *const time_t, _: *mut tm) -> *mut tm;
    fn localtime_r(_: *const time_t, _: *mut tm) -> *mut tm;
    fn av_strlcpy(
        dst: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        size: size_t,
    ) -> size_t;
    fn av_reduce(
        dst_num: *mut ::core::ffi::c_int,
        dst_den: *mut ::core::ffi::c_int,
        num: int64_t,
        den: int64_t,
        max: int64_t,
    ) -> ::core::ffi::c_int;
    fn sscanf(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn av_d2q(d: ::core::ffi::c_double, max: ::core::ffi::c_int) -> AVRational;
    fn av_strcasecmp(
        a: *const ::core::ffi::c_char,
        b: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn av_strncasecmp(
        a: *const ::core::ffi::c_char,
        b: *const ::core::ffi::c_char,
        n: size_t,
    ) -> ::core::ffi::c_int;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
}
pub type size_t = ::core::ffi::c_ulong;
pub type time_t = ::core::ffi::c_long;
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
pub type int64_t = ::core::ffi::c_long;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint32_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: ::core::ffi::c_int,
    pub den: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoSizeAbbr {
    pub abbr: *const ::core::ffi::c_char,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoRateAbbr {
    pub abbr: *const ::core::ffi::c_char,
    pub rate: AVRational,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColorEntry {
    pub name: *const ::core::ffi::c_char,
    pub rgb_color: [uint8_t; 3],
}
#[no_mangle]
pub unsafe extern "C" fn av_parse_ratio(
    mut q: *mut AVRational,
    mut str: *const ::core::ffi::c_char,
    mut max: ::core::ffi::c_int,
    mut log_offset: ::core::ffi::c_int,
    mut log_ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_char = 0;
    let mut ret: ::core::ffi::c_int = 0;
    if sscanf(
        str,
        b"%d:%d%c\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*q).num,
        &raw mut (*q).den,
        &raw mut c,
    ) != 2 as ::core::ffi::c_int
    {
        let mut d: ::core::ffi::c_double = 0.;
        ret = av_expr_parse_and_eval(
            &raw mut d,
            str,
            ::core::ptr::null::<*const ::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_double>(),
            ::core::ptr::null::<*const ::core::ffi::c_char>(),
            ::core::ptr::null::<
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_double,
                    ) -> ::core::ffi::c_double,
                >,
            >(),
            ::core::ptr::null::<*const ::core::ffi::c_char>(),
            ::core::ptr::null::<
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_double,
                        ::core::ffi::c_double,
                    ) -> ::core::ffi::c_double,
                >,
            >(),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            log_offset,
            log_ctx,
        );
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        *q = av_d2q(d, max);
    } else {
        av_reduce(
            &raw mut (*q).num,
            &raw mut (*q).den,
            (*q).num as int64_t,
            (*q).den as int64_t,
            max as int64_t,
        );
    }
    return 0 as ::core::ffi::c_int;
}
static mut video_size_abbrs: [VideoSizeAbbr; 55] = [
    VideoSizeAbbr {
        abbr: b"ntsc\0" as *const u8 as *const ::core::ffi::c_char,
        width: 720 as ::core::ffi::c_int,
        height: 480 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"pal\0" as *const u8 as *const ::core::ffi::c_char,
        width: 720 as ::core::ffi::c_int,
        height: 576 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"qntsc\0" as *const u8 as *const ::core::ffi::c_char,
        width: 352 as ::core::ffi::c_int,
        height: 240 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"qpal\0" as *const u8 as *const ::core::ffi::c_char,
        width: 352 as ::core::ffi::c_int,
        height: 288 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"sntsc\0" as *const u8 as *const ::core::ffi::c_char,
        width: 640 as ::core::ffi::c_int,
        height: 480 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"spal\0" as *const u8 as *const ::core::ffi::c_char,
        width: 768 as ::core::ffi::c_int,
        height: 576 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"film\0" as *const u8 as *const ::core::ffi::c_char,
        width: 352 as ::core::ffi::c_int,
        height: 240 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"ntsc-film\0" as *const u8 as *const ::core::ffi::c_char,
        width: 352 as ::core::ffi::c_int,
        height: 240 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"sqcif\0" as *const u8 as *const ::core::ffi::c_char,
        width: 128 as ::core::ffi::c_int,
        height: 96 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"qcif\0" as *const u8 as *const ::core::ffi::c_char,
        width: 176 as ::core::ffi::c_int,
        height: 144 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"cif\0" as *const u8 as *const ::core::ffi::c_char,
        width: 352 as ::core::ffi::c_int,
        height: 288 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"4cif\0" as *const u8 as *const ::core::ffi::c_char,
        width: 704 as ::core::ffi::c_int,
        height: 576 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"16cif\0" as *const u8 as *const ::core::ffi::c_char,
        width: 1408 as ::core::ffi::c_int,
        height: 1152 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"qqvga\0" as *const u8 as *const ::core::ffi::c_char,
        width: 160 as ::core::ffi::c_int,
        height: 120 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"qvga\0" as *const u8 as *const ::core::ffi::c_char,
        width: 320 as ::core::ffi::c_int,
        height: 240 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"vga\0" as *const u8 as *const ::core::ffi::c_char,
        width: 640 as ::core::ffi::c_int,
        height: 480 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"svga\0" as *const u8 as *const ::core::ffi::c_char,
        width: 800 as ::core::ffi::c_int,
        height: 600 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"xga\0" as *const u8 as *const ::core::ffi::c_char,
        width: 1024 as ::core::ffi::c_int,
        height: 768 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"uxga\0" as *const u8 as *const ::core::ffi::c_char,
        width: 1600 as ::core::ffi::c_int,
        height: 1200 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"qxga\0" as *const u8 as *const ::core::ffi::c_char,
        width: 2048 as ::core::ffi::c_int,
        height: 1536 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"sxga\0" as *const u8 as *const ::core::ffi::c_char,
        width: 1280 as ::core::ffi::c_int,
        height: 1024 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"qsxga\0" as *const u8 as *const ::core::ffi::c_char,
        width: 2560 as ::core::ffi::c_int,
        height: 2048 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"hsxga\0" as *const u8 as *const ::core::ffi::c_char,
        width: 5120 as ::core::ffi::c_int,
        height: 4096 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"wvga\0" as *const u8 as *const ::core::ffi::c_char,
        width: 852 as ::core::ffi::c_int,
        height: 480 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"wxga\0" as *const u8 as *const ::core::ffi::c_char,
        width: 1366 as ::core::ffi::c_int,
        height: 768 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"wsxga\0" as *const u8 as *const ::core::ffi::c_char,
        width: 1600 as ::core::ffi::c_int,
        height: 1024 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"wuxga\0" as *const u8 as *const ::core::ffi::c_char,
        width: 1920 as ::core::ffi::c_int,
        height: 1200 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"woxga\0" as *const u8 as *const ::core::ffi::c_char,
        width: 2560 as ::core::ffi::c_int,
        height: 1600 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"wqhd\0" as *const u8 as *const ::core::ffi::c_char,
        width: 2560 as ::core::ffi::c_int,
        height: 1440 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"wqsxga\0" as *const u8 as *const ::core::ffi::c_char,
        width: 3200 as ::core::ffi::c_int,
        height: 2048 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"wquxga\0" as *const u8 as *const ::core::ffi::c_char,
        width: 3840 as ::core::ffi::c_int,
        height: 2400 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"whsxga\0" as *const u8 as *const ::core::ffi::c_char,
        width: 6400 as ::core::ffi::c_int,
        height: 4096 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"whuxga\0" as *const u8 as *const ::core::ffi::c_char,
        width: 7680 as ::core::ffi::c_int,
        height: 4800 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"cga\0" as *const u8 as *const ::core::ffi::c_char,
        width: 320 as ::core::ffi::c_int,
        height: 200 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"ega\0" as *const u8 as *const ::core::ffi::c_char,
        width: 640 as ::core::ffi::c_int,
        height: 350 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"hd480\0" as *const u8 as *const ::core::ffi::c_char,
        width: 852 as ::core::ffi::c_int,
        height: 480 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"hd720\0" as *const u8 as *const ::core::ffi::c_char,
        width: 1280 as ::core::ffi::c_int,
        height: 720 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"hd1080\0" as *const u8 as *const ::core::ffi::c_char,
        width: 1920 as ::core::ffi::c_int,
        height: 1080 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"quadhd\0" as *const u8 as *const ::core::ffi::c_char,
        width: 2560 as ::core::ffi::c_int,
        height: 1440 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"2k\0" as *const u8 as *const ::core::ffi::c_char,
        width: 2048 as ::core::ffi::c_int,
        height: 1080 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"2kdci\0" as *const u8 as *const ::core::ffi::c_char,
        width: 2048 as ::core::ffi::c_int,
        height: 1080 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"2kflat\0" as *const u8 as *const ::core::ffi::c_char,
        width: 1998 as ::core::ffi::c_int,
        height: 1080 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"2kscope\0" as *const u8 as *const ::core::ffi::c_char,
        width: 2048 as ::core::ffi::c_int,
        height: 858 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"4k\0" as *const u8 as *const ::core::ffi::c_char,
        width: 4096 as ::core::ffi::c_int,
        height: 2160 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"4kdci\0" as *const u8 as *const ::core::ffi::c_char,
        width: 4096 as ::core::ffi::c_int,
        height: 2160 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"4kflat\0" as *const u8 as *const ::core::ffi::c_char,
        width: 3996 as ::core::ffi::c_int,
        height: 2160 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"4kscope\0" as *const u8 as *const ::core::ffi::c_char,
        width: 4096 as ::core::ffi::c_int,
        height: 1716 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"nhd\0" as *const u8 as *const ::core::ffi::c_char,
        width: 640 as ::core::ffi::c_int,
        height: 360 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"hqvga\0" as *const u8 as *const ::core::ffi::c_char,
        width: 240 as ::core::ffi::c_int,
        height: 160 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"wqvga\0" as *const u8 as *const ::core::ffi::c_char,
        width: 400 as ::core::ffi::c_int,
        height: 240 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"fwqvga\0" as *const u8 as *const ::core::ffi::c_char,
        width: 432 as ::core::ffi::c_int,
        height: 240 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"hvga\0" as *const u8 as *const ::core::ffi::c_char,
        width: 480 as ::core::ffi::c_int,
        height: 320 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"qhd\0" as *const u8 as *const ::core::ffi::c_char,
        width: 960 as ::core::ffi::c_int,
        height: 540 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"uhd2160\0" as *const u8 as *const ::core::ffi::c_char,
        width: 3840 as ::core::ffi::c_int,
        height: 2160 as ::core::ffi::c_int,
    },
    VideoSizeAbbr {
        abbr: b"uhd4320\0" as *const u8 as *const ::core::ffi::c_char,
        width: 7680 as ::core::ffi::c_int,
        height: 4320 as ::core::ffi::c_int,
    },
];
static mut video_rate_abbrs: [VideoRateAbbr; 8] = [
    VideoRateAbbr {
        abbr: b"ntsc\0" as *const u8 as *const ::core::ffi::c_char,
        rate: AVRational {
            num: 30000 as ::core::ffi::c_int,
            den: 1001 as ::core::ffi::c_int,
        },
    },
    VideoRateAbbr {
        abbr: b"pal\0" as *const u8 as *const ::core::ffi::c_char,
        rate: AVRational {
            num: 25 as ::core::ffi::c_int,
            den: 1 as ::core::ffi::c_int,
        },
    },
    VideoRateAbbr {
        abbr: b"qntsc\0" as *const u8 as *const ::core::ffi::c_char,
        rate: AVRational {
            num: 30000 as ::core::ffi::c_int,
            den: 1001 as ::core::ffi::c_int,
        },
    },
    VideoRateAbbr {
        abbr: b"qpal\0" as *const u8 as *const ::core::ffi::c_char,
        rate: AVRational {
            num: 25 as ::core::ffi::c_int,
            den: 1 as ::core::ffi::c_int,
        },
    },
    VideoRateAbbr {
        abbr: b"sntsc\0" as *const u8 as *const ::core::ffi::c_char,
        rate: AVRational {
            num: 30000 as ::core::ffi::c_int,
            den: 1001 as ::core::ffi::c_int,
        },
    },
    VideoRateAbbr {
        abbr: b"spal\0" as *const u8 as *const ::core::ffi::c_char,
        rate: AVRational {
            num: 25 as ::core::ffi::c_int,
            den: 1 as ::core::ffi::c_int,
        },
    },
    VideoRateAbbr {
        abbr: b"film\0" as *const u8 as *const ::core::ffi::c_char,
        rate: AVRational {
            num: 24 as ::core::ffi::c_int,
            den: 1 as ::core::ffi::c_int,
        },
    },
    VideoRateAbbr {
        abbr: b"ntsc-film\0" as *const u8 as *const ::core::ffi::c_char,
        rate: AVRational {
            num: 24000 as ::core::ffi::c_int,
            den: 1001 as ::core::ffi::c_int,
        },
    },
];
static mut months: [*const ::core::ffi::c_char; 12] = [
    b"january\0" as *const u8 as *const ::core::ffi::c_char,
    b"february\0" as *const u8 as *const ::core::ffi::c_char,
    b"march\0" as *const u8 as *const ::core::ffi::c_char,
    b"april\0" as *const u8 as *const ::core::ffi::c_char,
    b"may\0" as *const u8 as *const ::core::ffi::c_char,
    b"june\0" as *const u8 as *const ::core::ffi::c_char,
    b"july\0" as *const u8 as *const ::core::ffi::c_char,
    b"august\0" as *const u8 as *const ::core::ffi::c_char,
    b"september\0" as *const u8 as *const ::core::ffi::c_char,
    b"october\0" as *const u8 as *const ::core::ffi::c_char,
    b"november\0" as *const u8 as *const ::core::ffi::c_char,
    b"december\0" as *const u8 as *const ::core::ffi::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn av_parse_video_size(
    mut width_ptr: *mut ::core::ffi::c_int,
    mut height_ptr: *mut ::core::ffi::c_int,
    mut str: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = (::core::mem::size_of::<[VideoSizeAbbr; 55]>()
        as usize)
        .wrapping_div(::core::mem::size_of::<VideoSizeAbbr>() as usize)
        as ::core::ffi::c_int;
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut width: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut height: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < n {
        if strcmp(video_size_abbrs[i as usize].abbr, str) == 0 {
            width = video_size_abbrs[i as usize].width;
            height = video_size_abbrs[i as usize].height;
            break;
        } else {
            i += 1;
        }
    }
    if i == n {
        width = strtol(
            str,
            &raw mut p as *mut ::core::ffi::c_void as *mut *mut ::core::ffi::c_char,
            10 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int;
        if *p != 0 {
            p = p.offset(1);
        }
        height = strtol(
            p,
            &raw mut p as *mut ::core::ffi::c_void as *mut *mut ::core::ffi::c_char,
            10 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int;
        if *p != 0 {
            return -(22 as ::core::ffi::c_int);
        }
    }
    if width <= 0 as ::core::ffi::c_int || height <= 0 as ::core::ffi::c_int {
        return -(22 as ::core::ffi::c_int);
    }
    *width_ptr = width;
    *height_ptr = height;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_parse_video_rate(
    mut rate: *mut AVRational,
    mut arg: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = (::core::mem::size_of::<[VideoRateAbbr; 8]>()
        as usize)
        .wrapping_div(::core::mem::size_of::<VideoRateAbbr>() as usize)
        as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < n {
        if strcmp(video_rate_abbrs[i as usize].abbr, arg) == 0 {
            *rate = video_rate_abbrs[i as usize].rate;
            return 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    ret = av_parse_ratio(
        rate,
        arg,
        1001000 as ::core::ffi::c_int,
        56 as ::core::ffi::c_int - -(8 as ::core::ffi::c_int),
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    if (*rate).num == 0 || (*rate).den == 0 {
        ret = av_parse_ratio(
            rate,
            arg,
            2147483647 as ::core::ffi::c_int,
            56 as ::core::ffi::c_int - -(8 as ::core::ffi::c_int),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
    }
    if (*rate).num <= 0 as ::core::ffi::c_int || (*rate).den <= 0 as ::core::ffi::c_int {
        return -(22 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn av_isdigit(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (c >= '0' as i32 && c <= '9' as i32) as ::core::ffi::c_int;
}
static mut color_table: [ColorEntry; 140] = [
    ColorEntry {
        name: b"AliceBlue\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xf0 as ::core::ffi::c_int as uint8_t,
            0xf8 as ::core::ffi::c_int as uint8_t,
            0xff as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"AntiqueWhite\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xfa as ::core::ffi::c_int as uint8_t,
            0xeb as ::core::ffi::c_int as uint8_t,
            0xd7 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Aqua\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0 as ::core::ffi::c_int as uint8_t,
            0xff as ::core::ffi::c_int as uint8_t,
            0xff as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Aquamarine\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x7f as ::core::ffi::c_int as uint8_t,
            0xff as ::core::ffi::c_int as uint8_t,
            0xd4 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Azure\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xf0 as ::core::ffi::c_int as uint8_t,
            0xff as ::core::ffi::c_int as uint8_t,
            0xff as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Beige\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xf5 as ::core::ffi::c_int as uint8_t,
            0xf5 as ::core::ffi::c_int as uint8_t,
            0xdc as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Bisque\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xff as ::core::ffi::c_int as uint8_t,
            0xe4 as ::core::ffi::c_int as uint8_t,
            0xc4 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Black\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"BlanchedAlmond\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xff as ::core::ffi::c_int as uint8_t,
            0xeb as ::core::ffi::c_int as uint8_t,
            0xcd as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Blue\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0xff as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"BlueViolet\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x8a as ::core::ffi::c_int as uint8_t,
            0x2b as ::core::ffi::c_int as uint8_t,
            0xe2 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Brown\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xa5 as ::core::ffi::c_int as uint8_t,
            0x2a as ::core::ffi::c_int as uint8_t,
            0x2a as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"BurlyWood\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xde as ::core::ffi::c_int as uint8_t,
            0xb8 as ::core::ffi::c_int as uint8_t,
            0x87 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"CadetBlue\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x5f as ::core::ffi::c_int as uint8_t,
            0x9e as ::core::ffi::c_int as uint8_t,
            0xa0 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Chartreuse\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x7f as ::core::ffi::c_int as uint8_t,
            0xff as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Chocolate\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xd2 as ::core::ffi::c_int as uint8_t,
            0x69 as ::core::ffi::c_int as uint8_t,
            0x1e as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Coral\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xff as ::core::ffi::c_int as uint8_t,
            0x7f as ::core::ffi::c_int as uint8_t,
            0x50 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"CornflowerBlue\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x64 as ::core::ffi::c_int as uint8_t,
            0x95 as ::core::ffi::c_int as uint8_t,
            0xed as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Cornsilk\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xff as ::core::ffi::c_int as uint8_t,
            0xf8 as ::core::ffi::c_int as uint8_t,
            0xdc as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Crimson\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xdc as ::core::ffi::c_int as uint8_t,
            0x14 as ::core::ffi::c_int as uint8_t,
            0x3c as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Cyan\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0 as ::core::ffi::c_int as uint8_t,
            0xff as ::core::ffi::c_int as uint8_t,
            0xff as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"DarkBlue\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0x8b as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"DarkCyan\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0 as ::core::ffi::c_int as uint8_t,
            0x8b as ::core::ffi::c_int as uint8_t,
            0x8b as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"DarkGoldenRod\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xb8 as ::core::ffi::c_int as uint8_t,
            0x86 as ::core::ffi::c_int as uint8_t,
            0xb as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"DarkGray\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xa9 as ::core::ffi::c_int as uint8_t,
            0xa9 as ::core::ffi::c_int as uint8_t,
            0xa9 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"DarkGreen\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0 as ::core::ffi::c_int as uint8_t,
            0x64 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"DarkKhaki\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xbd as ::core::ffi::c_int as uint8_t,
            0xb7 as ::core::ffi::c_int as uint8_t,
            0x6b as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"DarkMagenta\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x8b as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0x8b as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"DarkOliveGreen\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x55 as ::core::ffi::c_int as uint8_t,
            0x6b as ::core::ffi::c_int as uint8_t,
            0x2f as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Darkorange\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xff as ::core::ffi::c_int as uint8_t,
            0x8c as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"DarkOrchid\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x99 as ::core::ffi::c_int as uint8_t,
            0x32 as ::core::ffi::c_int as uint8_t,
            0xcc as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"DarkRed\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x8b as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"DarkSalmon\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xe9 as ::core::ffi::c_int as uint8_t,
            0x96 as ::core::ffi::c_int as uint8_t,
            0x7a as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"DarkSeaGreen\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x8f as ::core::ffi::c_int as uint8_t,
            0xbc as ::core::ffi::c_int as uint8_t,
            0x8f as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"DarkSlateBlue\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x48 as ::core::ffi::c_int as uint8_t,
            0x3d as ::core::ffi::c_int as uint8_t,
            0x8b as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"DarkSlateGray\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x2f as ::core::ffi::c_int as uint8_t,
            0x4f as ::core::ffi::c_int as uint8_t,
            0x4f as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"DarkTurquoise\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0 as ::core::ffi::c_int as uint8_t,
            0xce as ::core::ffi::c_int as uint8_t,
            0xd1 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"DarkViolet\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x94 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0xd3 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"DeepPink\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xff as ::core::ffi::c_int as uint8_t,
            0x14 as ::core::ffi::c_int as uint8_t,
            0x93 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"DeepSkyBlue\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0 as ::core::ffi::c_int as uint8_t,
            0xbf as ::core::ffi::c_int as uint8_t,
            0xff as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"DimGray\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x69 as ::core::ffi::c_int as uint8_t,
            0x69 as ::core::ffi::c_int as uint8_t,
            0x69 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"DodgerBlue\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x1e as ::core::ffi::c_int as uint8_t,
            0x90 as ::core::ffi::c_int as uint8_t,
            0xff as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"FireBrick\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xb2 as ::core::ffi::c_int as uint8_t,
            0x22 as ::core::ffi::c_int as uint8_t,
            0x22 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"FloralWhite\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xff as ::core::ffi::c_int as uint8_t,
            0xfa as ::core::ffi::c_int as uint8_t,
            0xf0 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"ForestGreen\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x22 as ::core::ffi::c_int as uint8_t,
            0x8b as ::core::ffi::c_int as uint8_t,
            0x22 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Fuchsia\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xff as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0xff as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Gainsboro\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xdc as ::core::ffi::c_int as uint8_t,
            0xdc as ::core::ffi::c_int as uint8_t,
            0xdc as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"GhostWhite\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xf8 as ::core::ffi::c_int as uint8_t,
            0xf8 as ::core::ffi::c_int as uint8_t,
            0xff as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Gold\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xff as ::core::ffi::c_int as uint8_t,
            0xd7 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"GoldenRod\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xda as ::core::ffi::c_int as uint8_t,
            0xa5 as ::core::ffi::c_int as uint8_t,
            0x20 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Gray\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x80 as ::core::ffi::c_int as uint8_t,
            0x80 as ::core::ffi::c_int as uint8_t,
            0x80 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Green\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0 as ::core::ffi::c_int as uint8_t,
            0x80 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"GreenYellow\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xad as ::core::ffi::c_int as uint8_t,
            0xff as ::core::ffi::c_int as uint8_t,
            0x2f as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"HoneyDew\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xf0 as ::core::ffi::c_int as uint8_t,
            0xff as ::core::ffi::c_int as uint8_t,
            0xf0 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"HotPink\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xff as ::core::ffi::c_int as uint8_t,
            0x69 as ::core::ffi::c_int as uint8_t,
            0xb4 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"IndianRed\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xcd as ::core::ffi::c_int as uint8_t,
            0x5c as ::core::ffi::c_int as uint8_t,
            0x5c as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Indigo\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x4b as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0x82 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Ivory\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xff as ::core::ffi::c_int as uint8_t,
            0xff as ::core::ffi::c_int as uint8_t,
            0xf0 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Khaki\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xf0 as ::core::ffi::c_int as uint8_t,
            0xe6 as ::core::ffi::c_int as uint8_t,
            0x8c as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Lavender\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xe6 as ::core::ffi::c_int as uint8_t,
            0xe6 as ::core::ffi::c_int as uint8_t,
            0xfa as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"LavenderBlush\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xff as ::core::ffi::c_int as uint8_t,
            0xf0 as ::core::ffi::c_int as uint8_t,
            0xf5 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"LawnGreen\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x7c as ::core::ffi::c_int as uint8_t,
            0xfc as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"LemonChiffon\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xff as ::core::ffi::c_int as uint8_t,
            0xfa as ::core::ffi::c_int as uint8_t,
            0xcd as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"LightBlue\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xad as ::core::ffi::c_int as uint8_t,
            0xd8 as ::core::ffi::c_int as uint8_t,
            0xe6 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"LightCoral\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xf0 as ::core::ffi::c_int as uint8_t,
            0x80 as ::core::ffi::c_int as uint8_t,
            0x80 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"LightCyan\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xe0 as ::core::ffi::c_int as uint8_t,
            0xff as ::core::ffi::c_int as uint8_t,
            0xff as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"LightGoldenRodYellow\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xfa as ::core::ffi::c_int as uint8_t,
            0xfa as ::core::ffi::c_int as uint8_t,
            0xd2 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"LightGreen\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x90 as ::core::ffi::c_int as uint8_t,
            0xee as ::core::ffi::c_int as uint8_t,
            0x90 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"LightGrey\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xd3 as ::core::ffi::c_int as uint8_t,
            0xd3 as ::core::ffi::c_int as uint8_t,
            0xd3 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"LightPink\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xff as ::core::ffi::c_int as uint8_t,
            0xb6 as ::core::ffi::c_int as uint8_t,
            0xc1 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"LightSalmon\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xff as ::core::ffi::c_int as uint8_t,
            0xa0 as ::core::ffi::c_int as uint8_t,
            0x7a as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"LightSeaGreen\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x20 as ::core::ffi::c_int as uint8_t,
            0xb2 as ::core::ffi::c_int as uint8_t,
            0xaa as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"LightSkyBlue\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x87 as ::core::ffi::c_int as uint8_t,
            0xce as ::core::ffi::c_int as uint8_t,
            0xfa as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"LightSlateGray\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x77 as ::core::ffi::c_int as uint8_t,
            0x88 as ::core::ffi::c_int as uint8_t,
            0x99 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"LightSteelBlue\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xb0 as ::core::ffi::c_int as uint8_t,
            0xc4 as ::core::ffi::c_int as uint8_t,
            0xde as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"LightYellow\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xff as ::core::ffi::c_int as uint8_t,
            0xff as ::core::ffi::c_int as uint8_t,
            0xe0 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Lime\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0 as ::core::ffi::c_int as uint8_t,
            0xff as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"LimeGreen\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x32 as ::core::ffi::c_int as uint8_t,
            0xcd as ::core::ffi::c_int as uint8_t,
            0x32 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Linen\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xfa as ::core::ffi::c_int as uint8_t,
            0xf0 as ::core::ffi::c_int as uint8_t,
            0xe6 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Magenta\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xff as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0xff as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Maroon\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x80 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"MediumAquaMarine\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x66 as ::core::ffi::c_int as uint8_t,
            0xcd as ::core::ffi::c_int as uint8_t,
            0xaa as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"MediumBlue\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0xcd as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"MediumOrchid\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xba as ::core::ffi::c_int as uint8_t,
            0x55 as ::core::ffi::c_int as uint8_t,
            0xd3 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"MediumPurple\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x93 as ::core::ffi::c_int as uint8_t,
            0x70 as ::core::ffi::c_int as uint8_t,
            0xd8 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"MediumSeaGreen\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x3c as ::core::ffi::c_int as uint8_t,
            0xb3 as ::core::ffi::c_int as uint8_t,
            0x71 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"MediumSlateBlue\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x7b as ::core::ffi::c_int as uint8_t,
            0x68 as ::core::ffi::c_int as uint8_t,
            0xee as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"MediumSpringGreen\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0 as ::core::ffi::c_int as uint8_t,
            0xfa as ::core::ffi::c_int as uint8_t,
            0x9a as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"MediumTurquoise\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x48 as ::core::ffi::c_int as uint8_t,
            0xd1 as ::core::ffi::c_int as uint8_t,
            0xcc as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"MediumVioletRed\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xc7 as ::core::ffi::c_int as uint8_t,
            0x15 as ::core::ffi::c_int as uint8_t,
            0x85 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"MidnightBlue\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x19 as ::core::ffi::c_int as uint8_t,
            0x19 as ::core::ffi::c_int as uint8_t,
            0x70 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"MintCream\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xf5 as ::core::ffi::c_int as uint8_t,
            0xff as ::core::ffi::c_int as uint8_t,
            0xfa as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"MistyRose\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xff as ::core::ffi::c_int as uint8_t,
            0xe4 as ::core::ffi::c_int as uint8_t,
            0xe1 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Moccasin\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xff as ::core::ffi::c_int as uint8_t,
            0xe4 as ::core::ffi::c_int as uint8_t,
            0xb5 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"NavajoWhite\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xff as ::core::ffi::c_int as uint8_t,
            0xde as ::core::ffi::c_int as uint8_t,
            0xad as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Navy\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0x80 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"OldLace\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xfd as ::core::ffi::c_int as uint8_t,
            0xf5 as ::core::ffi::c_int as uint8_t,
            0xe6 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Olive\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x80 as ::core::ffi::c_int as uint8_t,
            0x80 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"OliveDrab\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x6b as ::core::ffi::c_int as uint8_t,
            0x8e as ::core::ffi::c_int as uint8_t,
            0x23 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Orange\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xff as ::core::ffi::c_int as uint8_t,
            0xa5 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"OrangeRed\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xff as ::core::ffi::c_int as uint8_t,
            0x45 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Orchid\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xda as ::core::ffi::c_int as uint8_t,
            0x70 as ::core::ffi::c_int as uint8_t,
            0xd6 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"PaleGoldenRod\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xee as ::core::ffi::c_int as uint8_t,
            0xe8 as ::core::ffi::c_int as uint8_t,
            0xaa as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"PaleGreen\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x98 as ::core::ffi::c_int as uint8_t,
            0xfb as ::core::ffi::c_int as uint8_t,
            0x98 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"PaleTurquoise\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xaf as ::core::ffi::c_int as uint8_t,
            0xee as ::core::ffi::c_int as uint8_t,
            0xee as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"PaleVioletRed\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xd8 as ::core::ffi::c_int as uint8_t,
            0x70 as ::core::ffi::c_int as uint8_t,
            0x93 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"PapayaWhip\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xff as ::core::ffi::c_int as uint8_t,
            0xef as ::core::ffi::c_int as uint8_t,
            0xd5 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"PeachPuff\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xff as ::core::ffi::c_int as uint8_t,
            0xda as ::core::ffi::c_int as uint8_t,
            0xb9 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Peru\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xcd as ::core::ffi::c_int as uint8_t,
            0x85 as ::core::ffi::c_int as uint8_t,
            0x3f as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Pink\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xff as ::core::ffi::c_int as uint8_t,
            0xc0 as ::core::ffi::c_int as uint8_t,
            0xcb as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Plum\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xdd as ::core::ffi::c_int as uint8_t,
            0xa0 as ::core::ffi::c_int as uint8_t,
            0xdd as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"PowderBlue\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xb0 as ::core::ffi::c_int as uint8_t,
            0xe0 as ::core::ffi::c_int as uint8_t,
            0xe6 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Purple\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x80 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0x80 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Red\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xff as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"RosyBrown\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xbc as ::core::ffi::c_int as uint8_t,
            0x8f as ::core::ffi::c_int as uint8_t,
            0x8f as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"RoyalBlue\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x41 as ::core::ffi::c_int as uint8_t,
            0x69 as ::core::ffi::c_int as uint8_t,
            0xe1 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"SaddleBrown\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x8b as ::core::ffi::c_int as uint8_t,
            0x45 as ::core::ffi::c_int as uint8_t,
            0x13 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Salmon\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xfa as ::core::ffi::c_int as uint8_t,
            0x80 as ::core::ffi::c_int as uint8_t,
            0x72 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"SandyBrown\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xf4 as ::core::ffi::c_int as uint8_t,
            0xa4 as ::core::ffi::c_int as uint8_t,
            0x60 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"SeaGreen\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x2e as ::core::ffi::c_int as uint8_t,
            0x8b as ::core::ffi::c_int as uint8_t,
            0x57 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"SeaShell\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xff as ::core::ffi::c_int as uint8_t,
            0xf5 as ::core::ffi::c_int as uint8_t,
            0xee as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Sienna\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xa0 as ::core::ffi::c_int as uint8_t,
            0x52 as ::core::ffi::c_int as uint8_t,
            0x2d as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Silver\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xc0 as ::core::ffi::c_int as uint8_t,
            0xc0 as ::core::ffi::c_int as uint8_t,
            0xc0 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"SkyBlue\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x87 as ::core::ffi::c_int as uint8_t,
            0xce as ::core::ffi::c_int as uint8_t,
            0xeb as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"SlateBlue\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x6a as ::core::ffi::c_int as uint8_t,
            0x5a as ::core::ffi::c_int as uint8_t,
            0xcd as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"SlateGray\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x70 as ::core::ffi::c_int as uint8_t,
            0x80 as ::core::ffi::c_int as uint8_t,
            0x90 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Snow\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xff as ::core::ffi::c_int as uint8_t,
            0xfa as ::core::ffi::c_int as uint8_t,
            0xfa as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"SpringGreen\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0 as ::core::ffi::c_int as uint8_t,
            0xff as ::core::ffi::c_int as uint8_t,
            0x7f as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"SteelBlue\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x46 as ::core::ffi::c_int as uint8_t,
            0x82 as ::core::ffi::c_int as uint8_t,
            0xb4 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Tan\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xd2 as ::core::ffi::c_int as uint8_t,
            0xb4 as ::core::ffi::c_int as uint8_t,
            0x8c as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Teal\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0 as ::core::ffi::c_int as uint8_t,
            0x80 as ::core::ffi::c_int as uint8_t,
            0x80 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Thistle\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xd8 as ::core::ffi::c_int as uint8_t,
            0xbf as ::core::ffi::c_int as uint8_t,
            0xd8 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Tomato\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xff as ::core::ffi::c_int as uint8_t,
            0x63 as ::core::ffi::c_int as uint8_t,
            0x47 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Turquoise\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x40 as ::core::ffi::c_int as uint8_t,
            0xe0 as ::core::ffi::c_int as uint8_t,
            0xd0 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Violet\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xee as ::core::ffi::c_int as uint8_t,
            0x82 as ::core::ffi::c_int as uint8_t,
            0xee as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Wheat\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xf5 as ::core::ffi::c_int as uint8_t,
            0xde as ::core::ffi::c_int as uint8_t,
            0xb3 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"White\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xff as ::core::ffi::c_int as uint8_t,
            0xff as ::core::ffi::c_int as uint8_t,
            0xff as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"WhiteSmoke\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xf5 as ::core::ffi::c_int as uint8_t,
            0xf5 as ::core::ffi::c_int as uint8_t,
            0xf5 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"Yellow\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0xff as ::core::ffi::c_int as uint8_t,
            0xff as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
    },
    ColorEntry {
        name: b"YellowGreen\0" as *const u8 as *const ::core::ffi::c_char,
        rgb_color: [
            0x9a as ::core::ffi::c_int as uint8_t,
            0xcd as ::core::ffi::c_int as uint8_t,
            0x32 as ::core::ffi::c_int as uint8_t,
        ],
    },
];
#[inline]
unsafe extern "C" fn av_isspace(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (c == ' ' as i32 || c == '\u{c}' as i32 || c == '\n' as i32
        || c == '\r' as i32 || c == '\t' as i32 || c == '\u{b}' as i32)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn color_table_compare(
    mut lhs: *const ::core::ffi::c_void,
    mut rhs: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return av_strcasecmp(
        lhs as *const ::core::ffi::c_char,
        (*(rhs as *const ColorEntry)).name,
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_parse_color(
    mut rgba_color: *mut uint8_t,
    mut color_string: *const ::core::ffi::c_char,
    mut slen: ::core::ffi::c_int,
    mut log_ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut tail: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut color_string2: [::core::ffi::c_char; 128] = [0; 128];
    let mut entry: *const ColorEntry = ::core::ptr::null::<ColorEntry>();
    let mut len: ::core::ffi::c_int = 0;
    let mut hex_offset: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if *color_string.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == '#' as i32
    {
        hex_offset = 1 as ::core::ffi::c_int;
    } else if strncmp(
        color_string,
        b"0x\0" as *const u8 as *const ::core::ffi::c_char,
        2 as size_t,
    ) == 0
    {
        hex_offset = 2 as ::core::ffi::c_int;
    }
    if slen < 0 as ::core::ffi::c_int {
        slen = strlen(color_string) as ::core::ffi::c_int;
    }
    av_strlcpy(
        &raw mut color_string2 as *mut ::core::ffi::c_char,
        color_string.offset(hex_offset as isize),
        if (slen - hex_offset + 1 as ::core::ffi::c_int) as usize
            > ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as usize
        {
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t
        } else {
            (slen - hex_offset + 1 as ::core::ffi::c_int) as size_t
        },
    );
    tail = strchr(&raw mut color_string2 as *mut ::core::ffi::c_char, '@' as i32);
    if !tail.is_null() {
        let fresh0 = tail;
        tail = tail.offset(1);
        *fresh0 = 0 as ::core::ffi::c_char;
    }
    len = strlen(&raw mut color_string2 as *mut ::core::ffi::c_char)
        as ::core::ffi::c_int;
    *rgba_color.offset(3 as ::core::ffi::c_int as isize) = 255 as uint8_t;
    if av_strcasecmp(
        &raw mut color_string2 as *mut ::core::ffi::c_char,
        b"random\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
        || av_strcasecmp(
            &raw mut color_string2 as *mut ::core::ffi::c_char,
            b"bikeshed\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
    {
        let mut rgba: ::core::ffi::c_int = av_get_random_seed() as ::core::ffi::c_int;
        *rgba_color.offset(0 as ::core::ffi::c_int as isize) = (rgba
            >> 24 as ::core::ffi::c_int) as uint8_t;
        *rgba_color.offset(1 as ::core::ffi::c_int as isize) = (rgba
            >> 16 as ::core::ffi::c_int) as uint8_t;
        *rgba_color.offset(2 as ::core::ffi::c_int as isize) = (rgba
            >> 8 as ::core::ffi::c_int) as uint8_t;
        *rgba_color.offset(3 as ::core::ffi::c_int as isize) = rgba as uint8_t;
    } else if hex_offset != 0
        || strspn(
            &raw mut color_string2 as *mut ::core::ffi::c_char,
            b"0123456789ABCDEFabcdef\0" as *const u8 as *const ::core::ffi::c_char,
        ) == len as ::core::ffi::c_ulong
    {
        let mut tail_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut rgba_0: ::core::ffi::c_uint = strtoul(
            &raw mut color_string2 as *mut ::core::ffi::c_char,
            &raw mut tail_0,
            16 as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint;
        if *tail_0 as ::core::ffi::c_int != 0
            || len != 6 as ::core::ffi::c_int && len != 8 as ::core::ffi::c_int
        {
            av_log(
                log_ctx,
                16 as ::core::ffi::c_int,
                b"Invalid 0xRRGGBB[AA] color string: '%s'\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                &raw mut color_string2 as *mut ::core::ffi::c_char,
            );
            return -(22 as ::core::ffi::c_int);
        }
        if len == 8 as ::core::ffi::c_int {
            *rgba_color.offset(3 as ::core::ffi::c_int as isize) = rgba_0 as uint8_t;
            rgba_0 >>= 8 as ::core::ffi::c_int;
        }
        *rgba_color.offset(0 as ::core::ffi::c_int as isize) = (rgba_0
            >> 16 as ::core::ffi::c_int) as uint8_t;
        *rgba_color.offset(1 as ::core::ffi::c_int as isize) = (rgba_0
            >> 8 as ::core::ffi::c_int) as uint8_t;
        *rgba_color.offset(2 as ::core::ffi::c_int as isize) = rgba_0 as uint8_t;
    } else {
        entry = bsearch(
            &raw mut color_string2 as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            &raw const color_table as *const ColorEntry as *const ::core::ffi::c_void,
            (::core::mem::size_of::<[ColorEntry; 140]>() as size_t)
                .wrapping_div(::core::mem::size_of::<ColorEntry>() as size_t),
            ::core::mem::size_of::<ColorEntry>() as size_t,
            Some(
                color_table_compare
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        ) as *const ColorEntry;
        if entry.is_null() {
            av_log(
                log_ctx,
                16 as ::core::ffi::c_int,
                b"Cannot find color '%s'\n\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut color_string2 as *mut ::core::ffi::c_char,
            );
            return -(22 as ::core::ffi::c_int);
        }
        memcpy(
            rgba_color as *mut ::core::ffi::c_void,
            &raw const (*entry).rgb_color as *const uint8_t
                as *const ::core::ffi::c_void,
            3 as size_t,
        );
    }
    if !tail.is_null() {
        let mut alpha: ::core::ffi::c_double = 0.;
        let mut alpha_string: *const ::core::ffi::c_char = tail;
        if strncmp(
            alpha_string,
            b"0x\0" as *const u8 as *const ::core::ffi::c_char,
            2 as size_t,
        ) == 0
        {
            alpha = strtoul(alpha_string, &raw mut tail, 16 as ::core::ffi::c_int)
                as ::core::ffi::c_double;
        } else {
            let mut norm_alpha: ::core::ffi::c_double = strtod(
                alpha_string,
                &raw mut tail,
            );
            if norm_alpha < 0.0f64 || norm_alpha > 1.0f64 {
                alpha = 256 as ::core::ffi::c_int as ::core::ffi::c_double;
            } else {
                alpha = 255 as ::core::ffi::c_int as ::core::ffi::c_double * norm_alpha;
            }
        }
        if tail == alpha_string as *mut ::core::ffi::c_char
            || *tail as ::core::ffi::c_int != 0
            || alpha > 255 as ::core::ffi::c_int as ::core::ffi::c_double
            || alpha < 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            av_log(
                log_ctx,
                16 as ::core::ffi::c_int,
                b"Invalid alpha value specifier '%s' in '%s'\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                alpha_string,
                color_string,
            );
            return -(22 as ::core::ffi::c_int);
        }
        *rgba_color.offset(3 as ::core::ffi::c_int as isize) = alpha as uint8_t;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_get_known_color_name(
    mut color_idx: ::core::ffi::c_int,
    mut rgbp: *mut *const uint8_t,
) -> *const ::core::ffi::c_char {
    let mut color: *const ColorEntry = ::core::ptr::null::<ColorEntry>();
    if color_idx as ::core::ffi::c_uint as usize
        >= (::core::mem::size_of::<[ColorEntry; 140]>() as usize)
            .wrapping_div(::core::mem::size_of::<ColorEntry>() as usize)
    {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    color = (&raw const color_table as *const ColorEntry).offset(color_idx as isize)
        as *const ColorEntry;
    if !rgbp.is_null() {
        *rgbp = &raw const (*color).rgb_color as *const uint8_t;
    }
    return (*color).name;
}
unsafe extern "C" fn date_get_num(
    mut pp: *mut *const ::core::ffi::c_char,
    mut n_min: ::core::ffi::c_int,
    mut n_max: ::core::ffi::c_int,
    mut len_max: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut val: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    p = *pp;
    val = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < len_max {
        c = *p as ::core::ffi::c_int;
        if av_isdigit(c) == 0 {
            break;
        }
        val = val * 10 as ::core::ffi::c_int + c - '0' as i32;
        p = p.offset(1);
        i += 1;
    }
    if p == *pp {
        return -(1 as ::core::ffi::c_int);
    }
    if val < n_min || val > n_max {
        return -(1 as ::core::ffi::c_int);
    }
    *pp = p;
    return val;
}
unsafe extern "C" fn date_get_month(
    mut pp: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 12 as ::core::ffi::c_int {
        if av_strncasecmp(*pp, months[i as usize], 3 as size_t) == 0 {
            let mut mo_full: *const ::core::ffi::c_char = months[i as usize]
                .offset(3 as ::core::ffi::c_int as isize);
            let mut len: ::core::ffi::c_int = strlen(mo_full) as ::core::ffi::c_int;
            *pp = (*pp).offset(3 as ::core::ffi::c_int as isize);
            if len > 0 as ::core::ffi::c_int
                && av_strncasecmp(*pp, mo_full, len as size_t) == 0
            {
                *pp = (*pp).offset(len as isize);
            }
            return i;
        }
        i += 1;
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn av_small_strptime(
    mut p: *const ::core::ffi::c_char,
    mut fmt: *const ::core::ffi::c_char,
    mut dt: *mut tm,
) -> *mut ::core::ffi::c_char {
    let mut c: ::core::ffi::c_int = 0;
    let mut val: ::core::ffi::c_int = 0;
    loop {
        let fresh1 = fmt;
        fmt = fmt.offset(1);
        c = *fresh1 as ::core::ffi::c_int;
        if !(c != 0) {
            break;
        }
        if c != '%' as i32 {
            if av_isspace(c) != 0 {
                while *p as ::core::ffi::c_int != 0
                    && av_isspace(*p as ::core::ffi::c_int) != 0
                {
                    p = p.offset(1);
                }
            } else if *p as ::core::ffi::c_int != c {
                return ::core::ptr::null_mut::<::core::ffi::c_char>()
            } else {
                p = p.offset(1);
            }
        } else {
            let fresh2 = fmt;
            fmt = fmt.offset(1);
            c = *fresh2 as ::core::ffi::c_int;
            match c {
                72 | 74 => {
                    val = date_get_num(
                        &raw mut p,
                        0 as ::core::ffi::c_int,
                        if c == 'H' as i32 {
                            23 as ::core::ffi::c_int
                        } else {
                            2147483647 as ::core::ffi::c_int
                        },
                        if c == 'H' as i32 {
                            2 as ::core::ffi::c_int
                        } else {
                            4 as ::core::ffi::c_int
                        },
                    );
                    if val == -(1 as ::core::ffi::c_int) {
                        return ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                    (*dt).tm_hour = val;
                }
                77 => {
                    val = date_get_num(
                        &raw mut p,
                        0 as ::core::ffi::c_int,
                        59 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                    );
                    if val == -(1 as ::core::ffi::c_int) {
                        return ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                    (*dt).tm_min = val;
                }
                83 => {
                    val = date_get_num(
                        &raw mut p,
                        0 as ::core::ffi::c_int,
                        59 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                    );
                    if val == -(1 as ::core::ffi::c_int) {
                        return ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                    (*dt).tm_sec = val;
                }
                89 => {
                    val = date_get_num(
                        &raw mut p,
                        0 as ::core::ffi::c_int,
                        9999 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                    );
                    if val == -(1 as ::core::ffi::c_int) {
                        return ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                    (*dt).tm_year = val - 1900 as ::core::ffi::c_int;
                }
                109 => {
                    val = date_get_num(
                        &raw mut p,
                        1 as ::core::ffi::c_int,
                        12 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                    );
                    if val == -(1 as ::core::ffi::c_int) {
                        return ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                    (*dt).tm_mon = val - 1 as ::core::ffi::c_int;
                }
                100 => {
                    val = date_get_num(
                        &raw mut p,
                        1 as ::core::ffi::c_int,
                        31 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                    );
                    if val == -(1 as ::core::ffi::c_int) {
                        return ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                    (*dt).tm_mday = val;
                }
                84 => {
                    p = av_small_strptime(
                        p,
                        b"%H:%M:%S\0" as *const u8 as *const ::core::ffi::c_char,
                        dt,
                    );
                    if p.is_null() {
                        return ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                }
                98 | 66 | 104 => {
                    val = date_get_month(&raw mut p);
                    if val == -(1 as ::core::ffi::c_int) {
                        return ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                    (*dt).tm_mon = val;
                }
                37 => {
                    let fresh3 = p;
                    p = p.offset(1);
                    if *fresh3 as ::core::ffi::c_int != '%' as i32 {
                        return ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                }
                _ => return ::core::ptr::null_mut::<::core::ffi::c_char>(),
            }
        }
    }
    return p as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn av_timegm(mut tm: *mut tm) -> time_t {
    let mut t: time_t = 0;
    let mut y: ::core::ffi::c_int = (*tm).tm_year + 1900 as ::core::ffi::c_int;
    let mut m: ::core::ffi::c_int = (*tm).tm_mon + 1 as ::core::ffi::c_int;
    let mut d: ::core::ffi::c_int = (*tm).tm_mday;
    if m < 3 as ::core::ffi::c_int {
        m += 12 as ::core::ffi::c_int;
        y -= 1;
    }
    t = (86400 as ::core::ffi::c_longlong
        * (d
            + (153 as ::core::ffi::c_int * m - 457 as ::core::ffi::c_int)
                / 5 as ::core::ffi::c_int + 365 as ::core::ffi::c_int * y
            + y / 4 as ::core::ffi::c_int - y / 100 as ::core::ffi::c_int
            + y / 400 as ::core::ffi::c_int - 719469 as ::core::ffi::c_int)
            as ::core::ffi::c_longlong) as time_t;
    t
        += (3600 as ::core::ffi::c_int * (*tm).tm_hour
            + 60 as ::core::ffi::c_int * (*tm).tm_min + (*tm).tm_sec) as time_t;
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn av_parse_time(
    mut timeval: *mut int64_t,
    mut timestr: *const ::core::ffi::c_char,
    mut duration: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut q: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut t: int64_t = 0;
    let mut now64: int64_t = 0;
    let mut now: time_t = 0;
    let mut dt: tm = tm {
        tm_sec: 0 as ::core::ffi::c_int,
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
    let mut today: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut negative: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut microseconds: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut suffix: ::core::ffi::c_int = 1000000 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    static mut date_fmt: [*const ::core::ffi::c_char; 2] = [
        b"%Y - %m - %d\0" as *const u8 as *const ::core::ffi::c_char,
        b"%Y%m%d\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    static mut time_fmt: [*const ::core::ffi::c_char; 2] = [
        b"%H:%M:%S\0" as *const u8 as *const ::core::ffi::c_char,
        b"%H%M%S\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    static mut tz_fmt: [*const ::core::ffi::c_char; 3] = [
        b"%H:%M\0" as *const u8 as *const ::core::ffi::c_char,
        b"%H%M\0" as *const u8 as *const ::core::ffi::c_char,
        b"%H\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    p = timestr;
    q = ::core::ptr::null::<::core::ffi::c_char>();
    *timeval = (-(1 as ::core::ffi::c_int) as ::core::ffi::c_long
        - 0x7fffffffffffffff as ::core::ffi::c_long) as int64_t;
    if duration == 0 {
        now64 = av_gettime();
        now = (now64 / 1000000 as int64_t) as time_t;
        if av_strcasecmp(timestr, b"now\0" as *const u8 as *const ::core::ffi::c_char)
            == 0
        {
            *timeval = now64;
            return 0 as ::core::ffi::c_int;
        }
        i = 0 as ::core::ffi::c_int;
        while (i as usize)
            < (::core::mem::size_of::<[*const ::core::ffi::c_char; 2]>() as usize)
                .wrapping_div(
                    ::core::mem::size_of::<*const ::core::ffi::c_char>() as usize,
                )
        {
            q = av_small_strptime(p, date_fmt[i as usize], &raw mut dt);
            if !q.is_null() {
                break;
            }
            i += 1;
        }
        if q.is_null() {
            today = 1 as ::core::ffi::c_int;
            q = p;
        }
        p = q;
        if *p as ::core::ffi::c_int == 'T' as i32
            || *p as ::core::ffi::c_int == 't' as i32
        {
            p = p.offset(1);
        } else {
            while av_isspace(*p as ::core::ffi::c_int) != 0 {
                p = p.offset(1);
            }
        }
        i = 0 as ::core::ffi::c_int;
        while (i as usize)
            < (::core::mem::size_of::<[*const ::core::ffi::c_char; 2]>() as usize)
                .wrapping_div(
                    ::core::mem::size_of::<*const ::core::ffi::c_char>() as usize,
                )
        {
            q = av_small_strptime(p, time_fmt[i as usize], &raw mut dt);
            if !q.is_null() {
                break;
            }
            i += 1;
        }
    } else {
        if *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '-' as i32
        {
            negative = 1 as ::core::ffi::c_int;
            p = p.offset(1);
        }
        q = av_small_strptime(
            p,
            b"%J:%M:%S\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut dt,
        );
        if q.is_null() {
            q = av_small_strptime(
                p,
                b"%M:%S\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut dt,
            );
            dt.tm_hour = 0 as ::core::ffi::c_int;
        }
        if q.is_null() {
            let mut o: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            *__errno_location() = 0 as ::core::ffi::c_int;
            t = strtoll(p, &raw mut o, 10 as ::core::ffi::c_int) as int64_t;
            if o == p as *mut ::core::ffi::c_char {
                return -(22 as ::core::ffi::c_int);
            }
            if *__errno_location() == 34 as ::core::ffi::c_int {
                return -(34 as ::core::ffi::c_int);
            }
            q = o;
        } else {
            t = (dt.tm_hour * 3600 as ::core::ffi::c_int
                + dt.tm_min * 60 as ::core::ffi::c_int + dt.tm_sec) as int64_t;
        }
    }
    if q.is_null() {
        return -(22 as ::core::ffi::c_int);
    }
    if *q as ::core::ffi::c_int == '.' as i32 {
        let mut n: ::core::ffi::c_int = 0;
        q = q.offset(1);
        n = 100000 as ::core::ffi::c_int;
        while n >= 1 as ::core::ffi::c_int {
            if av_isdigit(*q as ::core::ffi::c_int) == 0 {
                break;
            }
            microseconds += n * (*q as ::core::ffi::c_int - '0' as i32);
            n /= 10 as ::core::ffi::c_int;
            q = q.offset(1);
        }
        while av_isdigit(*q as ::core::ffi::c_int) != 0 {
            q = q.offset(1);
        }
    }
    if duration != 0 {
        if *q.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'm' as i32
            && *q.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 's' as i32
        {
            suffix = 1000 as ::core::ffi::c_int;
            microseconds /= 1000 as ::core::ffi::c_int;
            q = q.offset(2 as ::core::ffi::c_int as isize);
        } else if *q.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'u' as i32
            && *q.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 's' as i32
        {
            suffix = 1 as ::core::ffi::c_int;
            microseconds = 0 as ::core::ffi::c_int;
            q = q.offset(2 as ::core::ffi::c_int as isize);
        } else if *q as ::core::ffi::c_int == 's' as i32 {
            q = q.offset(1);
        }
    } else {
        let mut is_utc: ::core::ffi::c_int = (*q as ::core::ffi::c_int == 'Z' as i32
            || *q as ::core::ffi::c_int == 'z' as i32) as ::core::ffi::c_int;
        let mut tzoffset: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        q = q.offset(is_utc as isize);
        if today == 0 && is_utc == 0
            && (*q as ::core::ffi::c_int == '+' as i32
                || *q as ::core::ffi::c_int == '-' as i32)
        {
            let mut tz: tm = tm {
                tm_sec: 0 as ::core::ffi::c_int,
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
            let mut sign: ::core::ffi::c_int = if *q as ::core::ffi::c_int == '+' as i32
            {
                -(1 as ::core::ffi::c_int)
            } else {
                1 as ::core::ffi::c_int
            };
            q = q.offset(1);
            p = q;
            i = 0 as ::core::ffi::c_int;
            while (i as usize)
                < (::core::mem::size_of::<[*const ::core::ffi::c_char; 3]>() as usize)
                    .wrapping_div(
                        ::core::mem::size_of::<*const ::core::ffi::c_char>() as usize,
                    )
            {
                q = av_small_strptime(p, tz_fmt[i as usize], &raw mut tz);
                if !q.is_null() {
                    break;
                }
                i += 1;
            }
            if q.is_null() {
                return -(22 as ::core::ffi::c_int);
            }
            tzoffset = sign * (tz.tm_hour * 60 as ::core::ffi::c_int + tz.tm_min)
                * 60 as ::core::ffi::c_int;
            is_utc = 1 as ::core::ffi::c_int;
        }
        if today != 0 {
            let mut dt2: tm = if is_utc != 0 {
                *gmtime_r(&raw mut now, &raw mut tmbuf)
            } else {
                *localtime_r(&raw mut now, &raw mut tmbuf)
            };
            dt2.tm_hour = dt.tm_hour;
            dt2.tm_min = dt.tm_min;
            dt2.tm_sec = dt.tm_sec;
            dt = dt2;
        }
        dt.tm_isdst = if is_utc != 0 {
            0 as ::core::ffi::c_int
        } else {
            -(1 as ::core::ffi::c_int)
        };
        t = (if is_utc != 0 { av_timegm(&raw mut dt) } else { mktime(&raw mut dt) })
            as int64_t;
        t += tzoffset as int64_t;
    }
    if *q != 0 {
        return -(22 as ::core::ffi::c_int);
    }
    if (0x7fffffffffffffff as ::core::ffi::c_long / suffix as ::core::ffi::c_long) < t
        || t
            < (-(1 as ::core::ffi::c_int) as ::core::ffi::c_long
                - 0x7fffffffffffffff as ::core::ffi::c_long)
                / suffix as ::core::ffi::c_long
    {
        return -(34 as ::core::ffi::c_int);
    }
    t *= suffix as int64_t;
    if (0x7fffffffffffffff as ::core::ffi::c_long - microseconds as ::core::ffi::c_long)
        < t
    {
        return -(34 as ::core::ffi::c_int);
    }
    t += microseconds as int64_t;
    if t
        == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long
            - 0x7fffffffffffffff as ::core::ffi::c_long && negative != 0
    {
        return -(34 as ::core::ffi::c_int);
    }
    *timeval = if negative != 0 { -t } else { t };
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_find_info_tag(
    mut arg: *mut ::core::ffi::c_char,
    mut arg_size: ::core::ffi::c_int,
    mut tag1: *const ::core::ffi::c_char,
    mut info: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut tag: [::core::ffi::c_char; 128] = [0; 128];
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    p = info;
    if *p as ::core::ffi::c_int == '?' as i32 {
        p = p.offset(1);
    }
    loop {
        q = &raw mut tag as *mut ::core::ffi::c_char;
        while *p as ::core::ffi::c_int != '\0' as i32
            && *p as ::core::ffi::c_int != '=' as i32
            && *p as ::core::ffi::c_int != '&' as i32
        {
            if (q.offset_from(&raw mut tag as *mut ::core::ffi::c_char)
                as ::core::ffi::c_long as usize)
                < (::core::mem::size_of::<[::core::ffi::c_char; 128]>() as usize)
                    .wrapping_sub(1 as usize)
            {
                let fresh4 = q;
                q = q.offset(1);
                *fresh4 = *p;
            }
            p = p.offset(1);
        }
        *q = '\0' as i32 as ::core::ffi::c_char;
        q = arg;
        if *p as ::core::ffi::c_int == '=' as i32 {
            p = p.offset(1);
            while *p as ::core::ffi::c_int != '&' as i32
                && *p as ::core::ffi::c_int != '\0' as i32
            {
                if (q.offset_from(arg) as ::core::ffi::c_long)
                    < (arg_size - 1 as ::core::ffi::c_int) as ::core::ffi::c_long
                {
                    if *p as ::core::ffi::c_int == '+' as i32 {
                        let fresh5 = q;
                        q = q.offset(1);
                        *fresh5 = ' ' as i32 as ::core::ffi::c_char;
                    } else {
                        let fresh6 = q;
                        q = q.offset(1);
                        *fresh6 = *p;
                    }
                }
                p = p.offset(1);
            }
        }
        *q = '\0' as i32 as ::core::ffi::c_char;
        if strcmp(&raw mut tag as *mut ::core::ffi::c_char, tag1) == 0 {
            return 1 as ::core::ffi::c_int;
        }
        if *p as ::core::ffi::c_int != '&' as i32 {
            break;
        }
        p = p.offset(1);
    }
    return 0 as ::core::ffi::c_int;
}
