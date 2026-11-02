// Translated from FFmpeg 9.0.1 libavutil/timecode.c
// (preprocessed for aarch64-unknown-linux-musl) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// Copyright (c) 2006 Smartjog S.A.S, Baptiste Coudurier <baptiste.coudurier@gmail.com>
// Copyright (c) 2011-2012 Smartjog S.A.S, Clément Bœsch <clement.boesch@smartjog.com>
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
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn ff_timecode_set_smpte(
        drop_0: *mut ::core::ffi::c_uint,
        hh: *mut ::core::ffi::c_uint,
        mm: *mut ::core::ffi::c_uint,
        ss: *mut ::core::ffi::c_uint,
        ff: *mut ::core::ffi::c_uint,
        rate: AVRational,
        tcsmpte: uint32_t,
        prevent_df: ::core::ffi::c_int,
        skip_field: ::core::ffi::c_int,
    );
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sscanf(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
}
pub type size_t = ::core::ffi::c_ulong;
pub type int32_t = ::core::ffi::c_int;
pub type int64_t = ::core::ffi::c_long;
pub type uint32_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: ::core::ffi::c_int,
    pub den: ::core::ffi::c_int,
}
pub type AVTimecodeFlag = ::core::ffi::c_uint;
pub const AV_TIMECODE_FLAG_ALLOWNEGATIVE: AVTimecodeFlag = 4;
pub const AV_TIMECODE_FLAG_24HOURSMAX: AVTimecodeFlag = 2;
pub const AV_TIMECODE_FLAG_DROPFRAME: AVTimecodeFlag = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVTimecode {
    pub start: ::core::ffi::c_int,
    pub flags: uint32_t,
    pub rate: AVRational,
    pub fps: ::core::ffi::c_uint,
}
#[no_mangle]
pub unsafe extern "C" fn av_timecode_adjust_ntsc_framenum2(
    mut framenum: ::core::ffi::c_int,
    mut fps: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut drop_frames: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut d: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut frames_per_10mins: ::core::ffi::c_int = 0;
    if fps != 0 && fps % 30 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        drop_frames = fps / 30 as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
        frames_per_10mins = fps / 30 as ::core::ffi::c_int * 17982 as ::core::ffi::c_int;
    } else {
        return framenum
    }
    d = framenum / frames_per_10mins;
    m = framenum % frames_per_10mins;
    return (framenum as ::core::ffi::c_uint)
        .wrapping_add(
            (9 as ::core::ffi::c_uint)
                .wrapping_mul(drop_frames as ::core::ffi::c_uint)
                .wrapping_mul(d as ::core::ffi::c_uint),
        )
        .wrapping_add(
            (drop_frames
                * ((m - drop_frames) / (frames_per_10mins / 10 as ::core::ffi::c_int)))
                as ::core::ffi::c_uint,
        ) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_timecode_get_smpte_from_framenum(
    mut tc: *const AVTimecode,
    mut framenum: ::core::ffi::c_int,
) -> uint32_t {
    let mut fps: ::core::ffi::c_uint = (*tc).fps;
    let mut drop_0: ::core::ffi::c_int = ((*tc).flags
        & AV_TIMECODE_FLAG_DROPFRAME as ::core::ffi::c_int as uint32_t != 0)
        as ::core::ffi::c_int;
    let mut hh: ::core::ffi::c_int = 0;
    let mut mm: ::core::ffi::c_int = 0;
    let mut ss: ::core::ffi::c_int = 0;
    let mut ff: ::core::ffi::c_int = 0;
    framenum += (*tc).start;
    if drop_0 != 0 {
        framenum = av_timecode_adjust_ntsc_framenum2(
            framenum,
            (*tc).fps as ::core::ffi::c_int,
        );
    }
    ff = (framenum as ::core::ffi::c_uint).wrapping_rem(fps) as ::core::ffi::c_int;
    ss = (framenum as ::core::ffi::c_uint)
        .wrapping_div(fps)
        .wrapping_rem(60 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    mm = (framenum as ::core::ffi::c_longlong
        / (fps as ::core::ffi::c_longlong * 60 as ::core::ffi::c_longlong)
        % 60 as ::core::ffi::c_longlong) as ::core::ffi::c_int;
    hh = (framenum as ::core::ffi::c_longlong
        / (fps as ::core::ffi::c_longlong * 3600 as ::core::ffi::c_longlong)
        % 24 as ::core::ffi::c_longlong) as ::core::ffi::c_int;
    return av_timecode_get_smpte((*tc).rate, drop_0, hh, mm, ss, ff);
}
#[no_mangle]
pub unsafe extern "C" fn av_timecode_get_smpte(
    mut rate: AVRational,
    mut drop_0: ::core::ffi::c_int,
    mut hh: ::core::ffi::c_int,
    mut mm: ::core::ffi::c_int,
    mut ss: ::core::ffi::c_int,
    mut ff: ::core::ffi::c_int,
) -> uint32_t {
    let mut tc: uint32_t = 0 as uint32_t;
    if av_cmp_q(
        rate,
        AVRational {
            num: 30 as ::core::ffi::c_int,
            den: 1 as ::core::ffi::c_int,
        },
    ) == 1 as ::core::ffi::c_int
    {
        if ff % 2 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            if av_cmp_q(
                rate,
                AVRational {
                    num: 50 as ::core::ffi::c_int,
                    den: 1 as ::core::ffi::c_int,
                },
            ) == 0 as ::core::ffi::c_int
            {
                tc |= ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint32_t;
            } else {
                tc
                    |= ((1 as ::core::ffi::c_int) << 23 as ::core::ffi::c_int)
                        as uint32_t;
            }
        }
        ff /= 2 as ::core::ffi::c_int;
    }
    hh = hh % 24 as ::core::ffi::c_int;
    mm = av_clip_c(mm, 0 as ::core::ffi::c_int, 59 as ::core::ffi::c_int);
    ss = av_clip_c(ss, 0 as ::core::ffi::c_int, 59 as ::core::ffi::c_int);
    ff = ff % 40 as ::core::ffi::c_int;
    tc |= (drop_0 << 30 as ::core::ffi::c_int) as uint32_t;
    tc |= ((ff / 10 as ::core::ffi::c_int) << 28 as ::core::ffi::c_int) as uint32_t;
    tc |= ((ff % 10 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int) as uint32_t;
    tc |= ((ss / 10 as ::core::ffi::c_int) << 20 as ::core::ffi::c_int) as uint32_t;
    tc |= ((ss % 10 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as uint32_t;
    tc |= ((mm / 10 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int) as uint32_t;
    tc |= ((mm % 10 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint32_t;
    tc |= ((hh / 10 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint32_t;
    tc |= (hh % 10 as ::core::ffi::c_int) as uint32_t;
    return tc;
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
pub unsafe extern "C" fn av_timecode_make_string(
    mut tc: *const AVTimecode,
    mut buf: *mut ::core::ffi::c_char,
    mut framenum_arg: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut fps: ::core::ffi::c_uint = (*tc).fps;
    let mut drop_0: ::core::ffi::c_int = ((*tc).flags
        & AV_TIMECODE_FLAG_DROPFRAME as ::core::ffi::c_int as uint32_t)
        as ::core::ffi::c_int;
    let mut hh: ::core::ffi::c_int = 0;
    let mut mm: ::core::ffi::c_int = 0;
    let mut ss: ::core::ffi::c_int = 0;
    let mut ff: ::core::ffi::c_int = 0;
    let mut ff_len: ::core::ffi::c_int = 0;
    let mut neg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut framenum: int64_t = framenum_arg as int64_t;
    framenum += (*tc).start as int64_t;
    if drop_0 != 0 {
        framenum = av_timecode_adjust_ntsc_framenum2(
            framenum as ::core::ffi::c_int,
            fps as ::core::ffi::c_int,
        ) as int64_t;
    }
    if framenum < 0 as int64_t {
        framenum = -framenum;
        neg = ((*tc).flags
            & AV_TIMECODE_FLAG_ALLOWNEGATIVE as ::core::ffi::c_int as uint32_t)
            as ::core::ffi::c_int;
    }
    ff = (framenum % fps as int64_t) as ::core::ffi::c_int;
    ss = (framenum / fps as int64_t % 60 as int64_t) as ::core::ffi::c_int;
    mm = (framenum as ::core::ffi::c_longlong
        / (fps as ::core::ffi::c_longlong * 60 as ::core::ffi::c_longlong)
        % 60 as ::core::ffi::c_longlong) as ::core::ffi::c_int;
    hh = (framenum as ::core::ffi::c_longlong
        / (fps as ::core::ffi::c_longlong * 3600 as ::core::ffi::c_longlong))
        as ::core::ffi::c_int;
    if (*tc).flags & AV_TIMECODE_FLAG_24HOURSMAX as ::core::ffi::c_int as uint32_t != 0 {
        hh = hh % 24 as ::core::ffi::c_int;
    }
    ff_len = if fps > 10000 as ::core::ffi::c_uint {
        5 as ::core::ffi::c_int
    } else if fps > 1000 as ::core::ffi::c_uint {
        4 as ::core::ffi::c_int
    } else if fps > 100 as ::core::ffi::c_uint {
        3 as ::core::ffi::c_int
    } else if fps > 10 as ::core::ffi::c_uint {
        2 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    snprintf(
        buf,
        23 as size_t,
        b"%s%02d:%02d:%02d%c%0*d\0" as *const u8 as *const ::core::ffi::c_char,
        if neg != 0 {
            b"-\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        },
        hh,
        mm,
        ss,
        if drop_0 != 0 { ';' as i32 } else { ':' as i32 },
        ff_len,
        ff,
    );
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn av_timecode_make_smpte_tc_string2(
    mut buf: *mut ::core::ffi::c_char,
    mut rate: AVRational,
    mut tcsmpte: uint32_t,
    mut prevent_df: ::core::ffi::c_int,
    mut skip_field: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut hh: ::core::ffi::c_uint = 0;
    let mut mm: ::core::ffi::c_uint = 0;
    let mut ss: ::core::ffi::c_uint = 0;
    let mut ff: ::core::ffi::c_uint = 0;
    let mut drop_0: ::core::ffi::c_uint = 0;
    ff_timecode_set_smpte(
        &raw mut drop_0,
        &raw mut hh,
        &raw mut mm,
        &raw mut ss,
        &raw mut ff,
        rate,
        tcsmpte,
        prevent_df,
        skip_field,
    );
    snprintf(
        buf,
        23 as size_t,
        b"%02u:%02u:%02u%c%02u\0" as *const u8 as *const ::core::ffi::c_char,
        hh,
        mm,
        ss,
        if drop_0 != 0 { ';' as i32 } else { ':' as i32 },
        ff,
    );
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn av_timecode_make_smpte_tc_string(
    mut buf: *mut ::core::ffi::c_char,
    mut tcsmpte: uint32_t,
    mut prevent_df: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    return av_timecode_make_smpte_tc_string2(
        buf,
        AVRational {
            num: 30 as ::core::ffi::c_int,
            den: 1 as ::core::ffi::c_int,
        },
        tcsmpte,
        prevent_df,
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_timecode_make_mpeg_tc_string(
    mut buf: *mut ::core::ffi::c_char,
    mut tc25bit: uint32_t,
) -> *mut ::core::ffi::c_char {
    snprintf(
        buf,
        23 as size_t,
        b"%02u:%02u:%02u%c%02u\0" as *const u8 as *const ::core::ffi::c_char,
        tc25bit >> 19 as ::core::ffi::c_int & 0x1f as uint32_t,
        tc25bit >> 13 as ::core::ffi::c_int & 0x3f as uint32_t,
        tc25bit >> 6 as ::core::ffi::c_int & 0x3f as uint32_t,
        if tc25bit & ((1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int) as uint32_t
            != 0
        {
            ';' as i32
        } else {
            ':' as i32
        },
        tc25bit & 0x3f as uint32_t,
    );
    return buf;
}
unsafe extern "C" fn check_fps(mut fps: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    static mut supported_fps: [::core::ffi::c_int; 9] = [
        24 as ::core::ffi::c_int,
        25 as ::core::ffi::c_int,
        30 as ::core::ffi::c_int,
        48 as ::core::ffi::c_int,
        50 as ::core::ffi::c_int,
        60 as ::core::ffi::c_int,
        100 as ::core::ffi::c_int,
        120 as ::core::ffi::c_int,
        150 as ::core::ffi::c_int,
    ];
    i = 0 as ::core::ffi::c_int;
    while (i as usize)
        < (::core::mem::size_of::<[::core::ffi::c_int; 9]>() as usize)
            .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>() as usize)
    {
        if fps == supported_fps[i as usize] {
            return 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn check_timecode(
    mut log_ctx: *mut ::core::ffi::c_void,
    mut tc: *mut AVTimecode,
) -> ::core::ffi::c_int {
    if (*tc).fps as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
        av_log(
            log_ctx,
            16 as ::core::ffi::c_int,
            b"Valid timecode frame rate must be specified. Minimum value is 1\n\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return -(22 as ::core::ffi::c_int);
    }
    if (*tc).flags & AV_TIMECODE_FLAG_DROPFRAME as ::core::ffi::c_int as uint32_t != 0
        && (*tc).fps.wrapping_rem(30 as ::core::ffi::c_uint) != 0 as ::core::ffi::c_uint
    {
        av_log(
            log_ctx,
            16 as ::core::ffi::c_int,
            b"Drop frame is only allowed with multiples of 30000/1001 FPS\n\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return -(22 as ::core::ffi::c_int);
    }
    if check_fps((*tc).fps as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        av_log(
            log_ctx,
            24 as ::core::ffi::c_int,
            b"Using non-standard frame rate %d/%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            (*tc).rate.num,
            (*tc).rate.den,
        );
    }
    return 0 as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn av_clip_c(
    mut a: ::core::ffi::c_int,
    mut amin: ::core::ffi::c_int,
    mut amax: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if a < amin { return amin } else if a > amax { return amax } else { return a };
}
unsafe extern "C" fn fps_from_frame_rate(mut rate: AVRational) -> ::core::ffi::c_int {
    if rate.den == 0 || rate.num == 0 {
        return -(1 as ::core::ffi::c_int);
    }
    return ((rate.num as ::core::ffi::c_longlong
        + rate.den as ::core::ffi::c_longlong / 2 as ::core::ffi::c_longlong)
        / rate.den as ::core::ffi::c_longlong) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_timecode_check_frame_rate(
    mut rate: AVRational,
) -> ::core::ffi::c_int {
    return check_fps(fps_from_frame_rate(rate));
}
#[no_mangle]
pub unsafe extern "C" fn av_timecode_init(
    mut tc: *mut AVTimecode,
    mut rate: AVRational,
    mut flags: ::core::ffi::c_int,
    mut frame_start: ::core::ffi::c_int,
    mut log_ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    memset(
        tc as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<AVTimecode>() as size_t,
    );
    (*tc).start = frame_start;
    (*tc).flags = flags as uint32_t;
    (*tc).rate = rate;
    (*tc).fps = fps_from_frame_rate(rate) as ::core::ffi::c_uint;
    return check_timecode(log_ctx, tc);
}
#[no_mangle]
pub unsafe extern "C" fn av_timecode_init_from_components(
    mut tc: *mut AVTimecode,
    mut rate: AVRational,
    mut flags: ::core::ffi::c_int,
    mut hh: ::core::ffi::c_int,
    mut mm: ::core::ffi::c_int,
    mut ss: ::core::ffi::c_int,
    mut ff: ::core::ffi::c_int,
    mut log_ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut s: int64_t = 0;
    memset(
        tc as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<AVTimecode>() as size_t,
    );
    (*tc).flags = flags as uint32_t;
    (*tc).rate = rate;
    (*tc).fps = fps_from_frame_rate(rate) as ::core::ffi::c_uint;
    ret = check_timecode(log_ctx, tc);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    s = (hh as ::core::ffi::c_longlong * 3600 as ::core::ffi::c_longlong
        + mm as ::core::ffi::c_longlong * 60 as ::core::ffi::c_longlong
        + ss as ::core::ffi::c_longlong) as int64_t;
    if s != s as int32_t as int64_t {
        return -(22 as ::core::ffi::c_int);
    }
    s = s * (*tc).fps as int64_t + ff as int64_t;
    if s != s as int32_t as int64_t {
        return -(22 as ::core::ffi::c_int);
    }
    (*tc).start = s as ::core::ffi::c_int;
    if (*tc).flags & AV_TIMECODE_FLAG_DROPFRAME as ::core::ffi::c_int as uint32_t != 0 {
        let mut tmins: ::core::ffi::c_int = 60 as ::core::ffi::c_int * hh + mm;
        (*tc).start = ((*tc).start as ::core::ffi::c_uint)
            .wrapping_sub(
                (*tc)
                    .fps
                    .wrapping_div(30 as ::core::ffi::c_uint)
                    .wrapping_mul(2 as ::core::ffi::c_uint)
                    .wrapping_mul(
                        (tmins - tmins / 10 as ::core::ffi::c_int) as ::core::ffi::c_uint,
                    ),
            ) as ::core::ffi::c_int as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_timecode_init_from_string(
    mut tc: *mut AVTimecode,
    mut rate: AVRational,
    mut str: *const ::core::ffi::c_char,
    mut log_ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_char = 0;
    let mut hh: ::core::ffi::c_int = 0;
    let mut mm: ::core::ffi::c_int = 0;
    let mut ss: ::core::ffi::c_int = 0;
    let mut ff: ::core::ffi::c_int = 0;
    let mut flags: ::core::ffi::c_int = 0;
    if sscanf(
        str,
        b"%d:%d:%d%c%d\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut hh,
        &raw mut mm,
        &raw mut ss,
        &raw mut c,
        &raw mut ff,
    ) != 5 as ::core::ffi::c_int
    {
        av_log(
            log_ctx,
            16 as ::core::ffi::c_int,
            b"Unable to parse timecode, syntax: hh:mm:ss[:;.]ff\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    flags = if c as ::core::ffi::c_int != ':' as i32 {
        AV_TIMECODE_FLAG_DROPFRAME as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    return av_timecode_init_from_components(tc, rate, flags, hh, mm, ss, ff, log_ctx);
}
