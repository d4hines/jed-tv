// Translated from FFmpeg 9.0.1 libavutil/utils.c
// (preprocessed for armv7-unknown-linux-musleabihf) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
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
    fn strchr(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}
pub type AVMediaType = ::core::ffi::c_int;
pub const AVMEDIA_TYPE_NB: AVMediaType = 5;
pub const AVMEDIA_TYPE_ATTACHMENT: AVMediaType = 4;
pub const AVMEDIA_TYPE_SUBTITLE: AVMediaType = 3;
pub const AVMEDIA_TYPE_DATA: AVMediaType = 2;
pub const AVMEDIA_TYPE_AUDIO: AVMediaType = 1;
pub const AVMEDIA_TYPE_VIDEO: AVMediaType = 0;
pub const AVMEDIA_TYPE_UNKNOWN: AVMediaType = -1;
pub type AVPictureType = ::core::ffi::c_uint;
pub const AV_PICTURE_TYPE_BI: AVPictureType = 7;
pub const AV_PICTURE_TYPE_SP: AVPictureType = 6;
pub const AV_PICTURE_TYPE_SI: AVPictureType = 5;
pub const AV_PICTURE_TYPE_S: AVPictureType = 4;
pub const AV_PICTURE_TYPE_B: AVPictureType = 3;
pub const AV_PICTURE_TYPE_P: AVPictureType = 2;
pub const AV_PICTURE_TYPE_I: AVPictureType = 1;
pub const AV_PICTURE_TYPE_NONE: AVPictureType = 0;
pub type uint32_t = ::core::ffi::c_uint;
pub type size_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: ::core::ffi::c_int,
    pub den: ::core::ffi::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn av_get_media_type_string(
    mut media_type: AVMediaType,
) -> *const ::core::ffi::c_char {
    match media_type as ::core::ffi::c_int {
        0 => return b"video\0" as *const u8 as *const ::core::ffi::c_char,
        1 => return b"audio\0" as *const u8 as *const ::core::ffi::c_char,
        2 => return b"data\0" as *const u8 as *const ::core::ffi::c_char,
        3 => return b"subtitle\0" as *const u8 as *const ::core::ffi::c_char,
        4 => return b"attachment\0" as *const u8 as *const ::core::ffi::c_char,
        _ => return ::core::ptr::null::<::core::ffi::c_char>(),
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_get_picture_type_char(
    mut pict_type: AVPictureType,
) -> ::core::ffi::c_char {
    match pict_type as ::core::ffi::c_uint {
        1 => return 'I' as i32 as ::core::ffi::c_char,
        2 => return 'P' as i32 as ::core::ffi::c_char,
        3 => return 'B' as i32 as ::core::ffi::c_char,
        4 => return 'S' as i32 as ::core::ffi::c_char,
        5 => return 'i' as i32 as ::core::ffi::c_char,
        6 => return 'p' as i32 as ::core::ffi::c_char,
        7 => return 'b' as i32 as ::core::ffi::c_char,
        _ => return '?' as i32 as ::core::ffi::c_char,
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_fourcc_make_string(
    mut buf: *mut ::core::ffi::c_char,
    mut fourcc: uint32_t,
) -> *mut ::core::ffi::c_char {
    let mut i: ::core::ffi::c_int = 0;
    let mut orig_buf: *mut ::core::ffi::c_char = buf;
    let mut buf_size: size_t = 32 as size_t;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        let c: ::core::ffi::c_int = (fourcc & 0xff as uint32_t) as ::core::ffi::c_int;
        let print_chr: ::core::ffi::c_int = (c >= '0' as i32 && c <= '9' as i32
            || c >= 'a' as i32 && c <= 'z' as i32 || c >= 'A' as i32 && c <= 'Z' as i32
            || c != 0
                && !strchr(b". -_\0" as *const u8 as *const ::core::ffi::c_char, c)
                    .is_null()) as ::core::ffi::c_int;
        let len: ::core::ffi::c_int = snprintf(
            buf,
            buf_size as size_t,
            if print_chr != 0 {
                b"%c\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"[%d]\0" as *const u8 as *const ::core::ffi::c_char
            },
            c,
        ) as ::core::ffi::c_int;
        if len < 0 as ::core::ffi::c_int {
            break;
        }
        buf = buf.offset(len as isize);
        buf_size = if buf_size > len as size_t {
            buf_size.wrapping_sub(len as size_t)
        } else {
            0 as size_t
        };
        fourcc >>= 8 as ::core::ffi::c_int;
        i += 1;
    }
    return orig_buf;
}
#[no_mangle]
pub unsafe extern "C" fn av_get_time_base_q() -> AVRational {
    return AVRational {
        num: 1 as ::core::ffi::c_int,
        den: 1000000 as ::core::ffi::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_assert0_fpu() {}
