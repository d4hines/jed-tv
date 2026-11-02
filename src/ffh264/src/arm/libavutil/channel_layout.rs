// Translated from FFmpeg 9.0.1 libavutil/channel_layout.c
// (preprocessed for armv7-unknown-linux-musleabihf) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// Copyright (c) 2006 Michael Niedermayer <michaelni@gmx.at>
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
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strtol(
        _: *const ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn strtoull(
        _: *const ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulonglong;
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
    fn abort() -> !;
    fn strstr(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn av_strlcpy(
        dst: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        size: size_t,
    ) -> size_t;
    fn av_malloc_array(nmemb: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn av_bprint_init_for_buffer(
        buf: *mut AVBPrint,
        buffer: *mut ::core::ffi::c_char,
        size: ::core::ffi::c_uint,
    );
    fn av_calloc(nmemb: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn av_bprintf(buf: *mut AVBPrint, fmt: *const ::core::ffi::c_char, ...);
    fn av_bprint_chars(
        buf: *mut AVBPrint,
        c: ::core::ffi::c_char,
        n: ::core::ffi::c_uint,
    );
    fn floor(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn sqrt(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn av_free(ptr: *mut ::core::ffi::c_void);
    fn av_freep(ptr: *mut ::core::ffi::c_void);
    fn av_strdup(s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn av_sscanf(
        string: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn av_dynarray2_add(
        tab_ptr: *mut *mut ::core::ffi::c_void,
        nb_ptr: *mut ::core::ffi::c_int,
        elem_size: size_t,
        elem_data: *const uint8_t,
    ) -> *mut ::core::ffi::c_void;
    fn av_opt_get_key_value(
        ropts: *mut *const ::core::ffi::c_char,
        key_val_sep: *const ::core::ffi::c_char,
        pairs_sep: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_uint,
        rkey: *mut *mut ::core::ffi::c_char,
        rval: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
pub type uintptr_t = ::core::ffi::c_uint;
pub type int64_t = ::core::ffi::c_longlong;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint64_t = ::core::ffi::c_ulonglong;
pub type size_t = ::core::ffi::c_uint;
pub type AVChannel = ::core::ffi::c_int;
pub const AV_CHAN_AMBISONIC_END: AVChannel = 2047;
pub const AV_CHAN_AMBISONIC_BASE: AVChannel = 1024;
pub const AV_CHAN_UNKNOWN: AVChannel = 768;
pub const AV_CHAN_UNUSED: AVChannel = 512;
pub const AV_CHAN_BINAURAL_RIGHT: AVChannel = 62;
pub const AV_CHAN_BINAURAL_LEFT: AVChannel = 61;
pub const AV_CHAN_TOP_SURROUND_RIGHT: AVChannel = 44;
pub const AV_CHAN_TOP_SURROUND_LEFT: AVChannel = 43;
pub const AV_CHAN_SIDE_SURROUND_RIGHT: AVChannel = 42;
pub const AV_CHAN_SIDE_SURROUND_LEFT: AVChannel = 41;
pub const AV_CHAN_BOTTOM_FRONT_RIGHT: AVChannel = 40;
pub const AV_CHAN_BOTTOM_FRONT_LEFT: AVChannel = 39;
pub const AV_CHAN_BOTTOM_FRONT_CENTER: AVChannel = 38;
pub const AV_CHAN_TOP_SIDE_RIGHT: AVChannel = 37;
pub const AV_CHAN_TOP_SIDE_LEFT: AVChannel = 36;
pub const AV_CHAN_LOW_FREQUENCY_2: AVChannel = 35;
pub const AV_CHAN_SURROUND_DIRECT_RIGHT: AVChannel = 34;
pub const AV_CHAN_SURROUND_DIRECT_LEFT: AVChannel = 33;
pub const AV_CHAN_WIDE_RIGHT: AVChannel = 32;
pub const AV_CHAN_WIDE_LEFT: AVChannel = 31;
pub const AV_CHAN_STEREO_RIGHT: AVChannel = 30;
pub const AV_CHAN_STEREO_LEFT: AVChannel = 29;
pub const AV_CHAN_TOP_BACK_RIGHT: AVChannel = 17;
pub const AV_CHAN_TOP_BACK_CENTER: AVChannel = 16;
pub const AV_CHAN_TOP_BACK_LEFT: AVChannel = 15;
pub const AV_CHAN_TOP_FRONT_RIGHT: AVChannel = 14;
pub const AV_CHAN_TOP_FRONT_CENTER: AVChannel = 13;
pub const AV_CHAN_TOP_FRONT_LEFT: AVChannel = 12;
pub const AV_CHAN_TOP_CENTER: AVChannel = 11;
pub const AV_CHAN_SIDE_RIGHT: AVChannel = 10;
pub const AV_CHAN_SIDE_LEFT: AVChannel = 9;
pub const AV_CHAN_BACK_CENTER: AVChannel = 8;
pub const AV_CHAN_FRONT_RIGHT_OF_CENTER: AVChannel = 7;
pub const AV_CHAN_FRONT_LEFT_OF_CENTER: AVChannel = 6;
pub const AV_CHAN_BACK_RIGHT: AVChannel = 5;
pub const AV_CHAN_BACK_LEFT: AVChannel = 4;
pub const AV_CHAN_LOW_FREQUENCY: AVChannel = 3;
pub const AV_CHAN_FRONT_CENTER: AVChannel = 2;
pub const AV_CHAN_FRONT_RIGHT: AVChannel = 1;
pub const AV_CHAN_FRONT_LEFT: AVChannel = 0;
pub const AV_CHAN_NONE: AVChannel = -1;
pub type AVChannelOrder = ::core::ffi::c_uint;
pub const FF_CHANNEL_ORDER_NB: AVChannelOrder = 4;
pub const AV_CHANNEL_ORDER_AMBISONIC: AVChannelOrder = 3;
pub const AV_CHANNEL_ORDER_CUSTOM: AVChannelOrder = 2;
pub const AV_CHANNEL_ORDER_NATIVE: AVChannelOrder = 1;
pub const AV_CHANNEL_ORDER_UNSPEC: AVChannelOrder = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVChannelCustom {
    pub id: AVChannel,
    pub name: [::core::ffi::c_char; 16],
    pub opaque: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVChannelLayout {
    pub order: AVChannelOrder,
    pub nb_channels: ::core::ffi::c_int,
    pub u: C2RustUnnamed,
    pub opaque: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub mask: uint64_t,
    pub map: *mut AVChannelCustom,
}
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
pub struct channel_name {
    pub name: *const ::core::ffi::c_char,
    pub description: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct channel_layout_name {
    pub name: *const ::core::ffi::c_char,
    pub layout: AVChannelLayout,
}
pub const AV_OPT_FLAG_IMPLICIT_KEY: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
static mut channel_names: [channel_name; 63] = [
    channel_name {
        name: b"FL\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"front left\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"FR\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"front right\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"FC\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"front center\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"LFE\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"low frequency\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"BL\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"back left\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"BR\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"back right\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"FLC\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"front left-of-center\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"FRC\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"front right-of-center\0" as *const u8
            as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"BC\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"back center\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"SL\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"side left\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"SR\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"side right\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"TC\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"top center\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"TFL\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"top front left\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"TFC\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"top front center\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"TFR\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"top front right\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"TBL\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"top back left\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"TBC\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"top back center\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"TBR\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"top back right\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        description: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    channel_name {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        description: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    channel_name {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        description: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    channel_name {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        description: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    channel_name {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        description: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    channel_name {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        description: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    channel_name {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        description: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    channel_name {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        description: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    channel_name {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        description: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    channel_name {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        description: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    channel_name {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        description: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    channel_name {
        name: b"DL\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"downmix left\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"DR\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"downmix right\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"WL\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"wide left\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"WR\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"wide right\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"SDL\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"surround direct left\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"SDR\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"surround direct right\0" as *const u8
            as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"LFE2\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"low frequency 2\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"TSL\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"top side left\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"TSR\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"top side right\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"BFC\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"bottom front center\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"BFL\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"bottom front left\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"BFR\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"bottom front right\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"SSL\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"side surround left\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"SSR\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"side surround right\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"TTL\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"top surround left\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"TTR\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"top surround right\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        description: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    channel_name {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        description: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    channel_name {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        description: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    channel_name {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        description: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    channel_name {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        description: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    channel_name {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        description: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    channel_name {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        description: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    channel_name {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        description: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    channel_name {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        description: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    channel_name {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        description: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    channel_name {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        description: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    channel_name {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        description: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    channel_name {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        description: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    channel_name {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        description: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    channel_name {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        description: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    channel_name {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        description: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    channel_name {
        name: b"BIL\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"binaural left\0" as *const u8 as *const ::core::ffi::c_char,
    },
    channel_name {
        name: b"BIR\0" as *const u8 as *const ::core::ffi::c_char,
        description: b"binaural right\0" as *const u8 as *const ::core::ffi::c_char,
    },
];
#[no_mangle]
pub unsafe extern "C" fn av_channel_name_bprint(
    mut bp: *mut AVBPrint,
    mut channel_id: AVChannel,
) {
    if channel_id as ::core::ffi::c_int >= AV_CHAN_AMBISONIC_BASE as ::core::ffi::c_int
        && channel_id as ::core::ffi::c_int
            <= AV_CHAN_AMBISONIC_END as ::core::ffi::c_int
    {
        av_bprintf(
            bp,
            b"AMBI%d\0" as *const u8 as *const ::core::ffi::c_char,
            channel_id as ::core::ffi::c_int
                - AV_CHAN_AMBISONIC_BASE as ::core::ffi::c_int,
        );
    } else if (channel_id as usize)
        < (::core::mem::size_of::<[channel_name; 63]>() as usize)
            .wrapping_div(::core::mem::size_of::<channel_name>() as usize)
        && !channel_names[channel_id as usize].name.is_null()
    {
        av_bprintf(
            bp,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            channel_names[channel_id as usize].name,
        );
    } else if channel_id as ::core::ffi::c_int == AV_CHAN_NONE as ::core::ffi::c_int {
        av_bprintf(bp, b"NONE\0" as *const u8 as *const ::core::ffi::c_char);
    } else if channel_id as ::core::ffi::c_int == AV_CHAN_UNKNOWN as ::core::ffi::c_int {
        av_bprintf(bp, b"UNK\0" as *const u8 as *const ::core::ffi::c_char);
    } else if channel_id as ::core::ffi::c_int == AV_CHAN_UNUSED as ::core::ffi::c_int {
        av_bprintf(bp, b"UNSD\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        av_bprintf(
            bp,
            b"USR%d\0" as *const u8 as *const ::core::ffi::c_char,
            channel_id as ::core::ffi::c_int,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_channel_name(
    mut buf: *mut ::core::ffi::c_char,
    mut buf_size: size_t,
    mut channel_id: AVChannel,
) -> ::core::ffi::c_int {
    let mut bp: AVBPrint = AVBPrint {
        str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        len: 0,
        size: 0,
        size_max: 0,
        reserved_internal_buffer: [0; 1],
        reserved_padding: [0; 1004],
    };
    if buf.is_null() && buf_size != 0 {
        return -(22 as ::core::ffi::c_int);
    }
    av_bprint_init_for_buffer(&raw mut bp, buf, buf_size as ::core::ffi::c_uint);
    av_channel_name_bprint(&raw mut bp, channel_id);
    if bp.len >= 2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint {
        return -(34 as ::core::ffi::c_int);
    }
    return bp.len.wrapping_add(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_channel_description_bprint(
    mut bp: *mut AVBPrint,
    mut channel_id: AVChannel,
) {
    if channel_id as ::core::ffi::c_int >= AV_CHAN_AMBISONIC_BASE as ::core::ffi::c_int
        && channel_id as ::core::ffi::c_int
            <= AV_CHAN_AMBISONIC_END as ::core::ffi::c_int
    {
        av_bprintf(
            bp,
            b"ambisonic ACN %d\0" as *const u8 as *const ::core::ffi::c_char,
            channel_id as ::core::ffi::c_int
                - AV_CHAN_AMBISONIC_BASE as ::core::ffi::c_int,
        );
    } else if (channel_id as usize)
        < (::core::mem::size_of::<[channel_name; 63]>() as usize)
            .wrapping_div(::core::mem::size_of::<channel_name>() as usize)
        && !channel_names[channel_id as usize].description.is_null()
    {
        av_bprintf(
            bp,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            channel_names[channel_id as usize].description,
        );
    } else if channel_id as ::core::ffi::c_int == AV_CHAN_NONE as ::core::ffi::c_int {
        av_bprintf(bp, b"none\0" as *const u8 as *const ::core::ffi::c_char);
    } else if channel_id as ::core::ffi::c_int == AV_CHAN_UNKNOWN as ::core::ffi::c_int {
        av_bprintf(bp, b"unknown\0" as *const u8 as *const ::core::ffi::c_char);
    } else if channel_id as ::core::ffi::c_int == AV_CHAN_UNUSED as ::core::ffi::c_int {
        av_bprintf(bp, b"unused\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        av_bprintf(
            bp,
            b"user %d\0" as *const u8 as *const ::core::ffi::c_char,
            channel_id as ::core::ffi::c_int,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_channel_description(
    mut buf: *mut ::core::ffi::c_char,
    mut buf_size: size_t,
    mut channel_id: AVChannel,
) -> ::core::ffi::c_int {
    let mut bp: AVBPrint = AVBPrint {
        str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        len: 0,
        size: 0,
        size_max: 0,
        reserved_internal_buffer: [0; 1],
        reserved_padding: [0; 1004],
    };
    if buf.is_null() && buf_size != 0 {
        return -(22 as ::core::ffi::c_int);
    }
    av_bprint_init_for_buffer(&raw mut bp, buf, buf_size as ::core::ffi::c_uint);
    av_channel_description_bprint(&raw mut bp, channel_id);
    if bp.len >= 2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint {
        return -(34 as ::core::ffi::c_int);
    }
    return bp.len.wrapping_add(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_channel_from_string(
    mut str: *const ::core::ffi::c_char,
) -> AVChannel {
    let mut i: ::core::ffi::c_int = 0;
    let mut endptr: *mut ::core::ffi::c_char = str as *mut ::core::ffi::c_char;
    let mut id: AVChannel = AV_CHAN_NONE;
    if strncmp(str, b"AMBI\0" as *const u8 as *const ::core::ffi::c_char, 4 as size_t)
        == 0
    {
        i = strtol(
            str.offset(4 as ::core::ffi::c_int as isize),
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int;
        if i < 0 as ::core::ffi::c_int
            || i
                > AV_CHAN_AMBISONIC_END as ::core::ffi::c_int
                    - AV_CHAN_AMBISONIC_BASE as ::core::ffi::c_int
        {
            return AV_CHAN_NONE;
        }
        return (AV_CHAN_AMBISONIC_BASE as ::core::ffi::c_int + i) as AVChannel;
    }
    i = 0 as ::core::ffi::c_int;
    while (i as usize)
        < (::core::mem::size_of::<[channel_name; 63]>() as usize)
            .wrapping_div(::core::mem::size_of::<channel_name>() as usize)
    {
        if !channel_names[i as usize].name.is_null()
            && strcmp(str, channel_names[i as usize].name) == 0
        {
            return i as AVChannel;
        }
        i += 1;
    }
    if strcmp(str, b"UNK\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        return AV_CHAN_UNKNOWN;
    }
    if strcmp(str, b"UNSD\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        return AV_CHAN_UNUSED;
    }
    if strncmp(str, b"USR\0" as *const u8 as *const ::core::ffi::c_char, 3 as size_t)
        == 0
    {
        let mut p: *const ::core::ffi::c_char = str
            .offset(3 as ::core::ffi::c_int as isize);
        id = strtol(p, &raw mut endptr, 0 as ::core::ffi::c_int) as AVChannel;
    }
    if id as ::core::ffi::c_int >= 0 as ::core::ffi::c_int && *endptr == 0 {
        return id;
    }
    return AV_CHAN_NONE;
}
static mut channel_layout_map: [channel_layout_name; 40] = [
    channel_layout_name {
        name: b"mono\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 1 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_CENTER as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"stereo\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 2 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"2.1\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 3 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_LOW_FREQUENCY as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"3.0\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 3 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_CENTER as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"3.0(back)\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 3 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_CENTER as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"4.0\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 4 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_CENTER as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"quad\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 4 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_RIGHT as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"quad(side)\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 4 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_RIGHT as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"3.1\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 4 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_LOW_FREQUENCY as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"5.0\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 5 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_RIGHT as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"5.0(side)\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 5 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_RIGHT as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"4.1\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 5 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_LOW_FREQUENCY as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"5.1\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 6 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_LOW_FREQUENCY as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"5.1(side)\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 6 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_LOW_FREQUENCY as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"6.0\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 6 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_CENTER as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"6.0(front)\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 6 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_LEFT_OF_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT_OF_CENTER as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"3.1.2\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 6 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_LOW_FREQUENCY as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_FRONT_RIGHT as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"hexagonal\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 6 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_CENTER as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"6.1\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 7 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_LOW_FREQUENCY as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_CENTER as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"6.1(back)\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 7 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_LOW_FREQUENCY as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_CENTER as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"6.1(front)\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 7 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_LEFT_OF_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT_OF_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_LOW_FREQUENCY as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"7.0\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 7 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_RIGHT as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"7.0(front)\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 7 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_LEFT_OF_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT_OF_CENTER as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"7.1\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 8 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_LOW_FREQUENCY as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_RIGHT as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"7.1(wide)\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 8 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_LOW_FREQUENCY as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_LEFT_OF_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT_OF_CENTER as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"7.1(wide-side)\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 8 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_LOW_FREQUENCY as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_LEFT_OF_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT_OF_CENTER as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"5.1.2\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 8 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_LOW_FREQUENCY as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_FRONT_RIGHT as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"5.1.2(back)\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 8 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_LOW_FREQUENCY as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_FRONT_RIGHT as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"octagonal\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 8 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_RIGHT as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"cube\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 8 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_BACK_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_BACK_RIGHT as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"5.1.4\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 10 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_LOW_FREQUENCY as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_BACK_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_BACK_RIGHT as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"7.1.2\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 10 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_LOW_FREQUENCY as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_FRONT_RIGHT as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"7.1.4\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 12 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_LOW_FREQUENCY as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_BACK_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_BACK_RIGHT as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"7.2.3\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 12 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_LOW_FREQUENCY as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_BACK_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_LOW_FREQUENCY_2 as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"9.1.4\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 14 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_LOW_FREQUENCY as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_BACK_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_BACK_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_LEFT_OF_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT_OF_CENTER as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"9.1.6\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 16 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_LOW_FREQUENCY as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_BACK_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_BACK_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_LEFT_OF_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT_OF_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_SIDE_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_SIDE_RIGHT as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"hexadecagonal\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 16 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_WIDE_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_WIDE_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_BACK_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_BACK_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_BACK_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_FRONT_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_FRONT_RIGHT as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"binaural\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 2 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_BINAURAL_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BINAURAL_RIGHT as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"downmix\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 2 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_STEREO_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_STEREO_RIGHT as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
    channel_layout_name {
        name: b"22.2\0" as *const u8 as *const ::core::ffi::c_char,
        layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 24 as ::core::ffi::c_int,
            u: C2RustUnnamed {
                mask: (1 as ::core::ffi::c_ulonglong)
                    << AV_CHAN_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_SIDE_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_LOW_FREQUENCY as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_FRONT_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_BACK_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_BACK_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_LEFT_OF_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_FRONT_RIGHT_OF_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_SIDE_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_SIDE_RIGHT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BACK_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_LOW_FREQUENCY_2 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_FRONT_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_TOP_BACK_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BOTTOM_FRONT_CENTER as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BOTTOM_FRONT_LEFT as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_ulonglong)
                        << AV_CHAN_BOTTOM_FRONT_RIGHT as ::core::ffi::c_int,
            },
            opaque: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
        },
    },
];
#[no_mangle]
pub unsafe extern "C" fn av_channel_layout_custom_init(
    mut channel_layout: *mut AVChannelLayout,
    mut nb_channels: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut map: *mut AVChannelCustom = ::core::ptr::null_mut::<AVChannelCustom>();
    if nb_channels <= 0 as ::core::ffi::c_int {
        return -(22 as ::core::ffi::c_int);
    }
    map = av_calloc(
        nb_channels as size_t,
        ::core::mem::size_of::<AVChannelCustom>() as size_t,
    ) as *mut AVChannelCustom;
    if map.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < nb_channels {
        (*map.offset(i as isize)).id = AV_CHAN_UNKNOWN;
        i += 1;
    }
    (*channel_layout).order = AV_CHANNEL_ORDER_CUSTOM;
    (*channel_layout).nb_channels = nb_channels;
    (*channel_layout).u.map = map;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_channel_layout_from_mask(
    mut channel_layout: *mut AVChannelLayout,
    mut mask: uint64_t,
) -> ::core::ffi::c_int {
    if mask == 0 {
        return -(22 as ::core::ffi::c_int);
    }
    (*channel_layout).order = AV_CHANNEL_ORDER_NATIVE;
    (*channel_layout).nb_channels = av_popcount64_c(mask);
    (*channel_layout).u.mask = mask;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn parse_channel_list(
    mut ch_layout: *mut AVChannelLayout,
    mut str: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut nb_channels: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut map: *mut AVChannelCustom = ::core::ptr::null_mut::<AVChannelCustom>();
    let mut custom: AVChannelCustom = AVChannelCustom {
        id: AV_CHAN_FRONT_LEFT,
        name: [0; 16],
        opaque: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    while *str != 0 {
        let mut channel: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut chname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        ret = av_opt_get_key_value(
            &raw mut str,
            b"@\0" as *const u8 as *const ::core::ffi::c_char,
            b"+\0" as *const u8 as *const ::core::ffi::c_char,
            AV_OPT_FLAG_IMPLICIT_KEY as ::core::ffi::c_int as ::core::ffi::c_uint,
            &raw mut channel,
            &raw mut chname,
        );
        if ret < 0 as ::core::ffi::c_int {
            av_freep(&raw mut map as *mut ::core::ffi::c_void);
            return ret;
        }
        if *str != 0 {
            str = str.offset(1);
        }
        if channel.is_null() {
            channel = chname;
            chname = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        av_strlcpy(
            &raw mut custom.name as *mut ::core::ffi::c_char,
            if !chname.is_null() {
                chname as *const ::core::ffi::c_char
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as size_t,
        );
        custom.id = av_channel_from_string(channel);
        av_free(channel as *mut ::core::ffi::c_void);
        av_free(chname as *mut ::core::ffi::c_void);
        if custom.id as ::core::ffi::c_int == AV_CHAN_NONE as ::core::ffi::c_int {
            av_freep(&raw mut map as *mut ::core::ffi::c_void);
            return -(22 as ::core::ffi::c_int);
        }
        av_dynarray2_add(
            &raw mut map as *mut *mut ::core::ffi::c_void,
            &raw mut nb_channels,
            ::core::mem::size_of::<AVChannelCustom>() as size_t,
            &raw mut custom as *mut ::core::ffi::c_void as *const uint8_t,
        );
        if map.is_null() {
            return -(12 as ::core::ffi::c_int);
        }
    }
    if nb_channels == 0 {
        return -(22 as ::core::ffi::c_int);
    }
    (*ch_layout).order = AV_CHANNEL_ORDER_CUSTOM;
    (*ch_layout).u.map = map;
    (*ch_layout).nb_channels = nb_channels;
    ret = av_channel_layout_retype(
        ch_layout,
        AV_CHANNEL_ORDER_UNSPEC,
        (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
    );
    if !(ret == 0 as ::core::ffi::c_int) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"ret == 0\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavutil/channel_layout.c\0" as *const u8
                as *const ::core::ffi::c_char,
            308 as ::core::ffi::c_int,
        );
        abort();
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_channel_layout_from_string(
    mut channel_layout: *mut AVChannelLayout,
    mut str: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut matches: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut channels: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nb_channels: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut chlist: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut mask: uint64_t = 0 as uint64_t;
    i = 0 as ::core::ffi::c_int;
    while (i as usize)
        < (::core::mem::size_of::<[channel_layout_name; 40]>() as usize)
            .wrapping_div(::core::mem::size_of::<channel_layout_name>() as usize)
    {
        if !channel_layout_map[i as usize].name.is_null()
            && strcmp(str, channel_layout_map[i as usize].name) == 0
        {
            *channel_layout = channel_layout_map[i as usize].layout;
            return 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    memset(
        channel_layout as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<AVChannelLayout>() as size_t,
    );
    if strncmp(
        str,
        b"ambisonic \0" as *const u8 as *const ::core::ffi::c_char,
        10 as size_t,
    ) == 0
    {
        let mut p: *const ::core::ffi::c_char = str
            .offset(10 as ::core::ffi::c_int as isize);
        let mut endptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut extra: AVChannelLayout = AVChannelLayout {
            order: AV_CHANNEL_ORDER_UNSPEC,
            nb_channels: 0,
            u: C2RustUnnamed { mask: 0 },
            opaque: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        };
        let mut order: ::core::ffi::c_int = 0;
        order = strtol(p, &raw mut endptr, 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        if order < 0 as ::core::ffi::c_int
            || order + 1 as ::core::ffi::c_int
                > 2147483647 as ::core::ffi::c_int / (order + 1 as ::core::ffi::c_int)
            || *endptr as ::core::ffi::c_int != 0
                && *endptr as ::core::ffi::c_int != '+' as i32
        {
            return -(22 as ::core::ffi::c_int);
        }
        (*channel_layout).order = AV_CHANNEL_ORDER_AMBISONIC;
        (*channel_layout).nb_channels = (order + 1 as ::core::ffi::c_int)
            * (order + 1 as ::core::ffi::c_int);
        if *endptr != 0 {
            ret = av_channel_layout_from_string(
                &raw mut extra,
                endptr.offset(1 as ::core::ffi::c_int as isize),
            );
            if ret < 0 as ::core::ffi::c_int {
                return ret;
            }
            if extra.nb_channels
                >= 2147483647 as ::core::ffi::c_int - (*channel_layout).nb_channels
            {
                av_channel_layout_uninit(&raw mut extra);
                return -(22 as ::core::ffi::c_int);
            }
            if extra.order as ::core::ffi::c_uint
                == AV_CHANNEL_ORDER_NATIVE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*channel_layout).u.mask = extra.u.mask;
            } else {
                (*channel_layout).order = AV_CHANNEL_ORDER_CUSTOM;
                (*channel_layout).u.map = av_calloc(
                    ((*channel_layout).nb_channels + extra.nb_channels) as size_t,
                    ::core::mem::size_of::<AVChannelCustom>() as size_t,
                ) as *mut AVChannelCustom;
                if (*channel_layout).u.map.is_null() {
                    av_channel_layout_uninit(&raw mut extra);
                    return -(12 as ::core::ffi::c_int);
                }
                i = 0 as ::core::ffi::c_int;
                while i < (*channel_layout).nb_channels {
                    (*(*channel_layout).u.map.offset(i as isize)).id = (AV_CHAN_AMBISONIC_BASE
                        as ::core::ffi::c_int + i) as AVChannel;
                    i += 1;
                }
                i = 0 as ::core::ffi::c_int;
                while i < extra.nb_channels {
                    let mut ch: AVChannel = av_channel_layout_channel_from_index(
                        &raw mut extra,
                        i as ::core::ffi::c_uint,
                    );
                    if ch as ::core::ffi::c_int
                        >= AV_CHAN_AMBISONIC_BASE as ::core::ffi::c_int
                        && ch as ::core::ffi::c_int
                            <= AV_CHAN_AMBISONIC_END as ::core::ffi::c_int
                    {
                        av_channel_layout_uninit(channel_layout);
                        av_channel_layout_uninit(&raw mut extra);
                        return -(22 as ::core::ffi::c_int);
                    }
                    (*(*channel_layout)
                        .u
                        .map
                        .offset(((*channel_layout).nb_channels + i) as isize))
                        .id = ch;
                    if extra.order as ::core::ffi::c_uint
                        == AV_CHANNEL_ORDER_CUSTOM as ::core::ffi::c_int
                            as ::core::ffi::c_uint
                        && (*extra.u.map.offset(i as isize))
                            .name[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                            != 0
                    {
                        av_strlcpy(
                            &raw mut (*(*channel_layout)
                                .u
                                .map
                                .offset(((*channel_layout).nb_channels + i) as isize))
                                .name as *mut ::core::ffi::c_char,
                            &raw mut (*extra.u.map.offset(i as isize)).name
                                as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as size_t,
                        );
                    }
                    i += 1;
                }
            }
            (*channel_layout).nb_channels += extra.nb_channels;
            av_channel_layout_uninit(&raw mut extra);
        }
        return 0 as ::core::ffi::c_int;
    }
    chlist = av_strdup(str);
    if chlist.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    matches = av_sscanf(
        str,
        b"%d channels (%[^)]\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut nb_channels,
        chlist,
    );
    ret = parse_channel_list(channel_layout, chlist);
    av_freep(&raw mut chlist as *mut ::core::ffi::c_void);
    if ret < 0 as ::core::ffi::c_int && ret != -(22 as ::core::ffi::c_int) {
        return ret;
    }
    if ret >= 0 as ::core::ffi::c_int {
        end = strchr(str, ')' as i32);
        if matches == 2 as ::core::ffi::c_int
            && (nb_channels != (*channel_layout).nb_channels || end.is_null()
                || {
                    end = end.offset(1);
                    *end as ::core::ffi::c_int != 0
                })
        {
            av_channel_layout_uninit(channel_layout);
            return -(22 as ::core::ffi::c_int);
        }
        return 0 as ::core::ffi::c_int;
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    mask = strtoull(str, &raw mut end, 0 as ::core::ffi::c_int) as uint64_t;
    if *__errno_location() == 0 && *end == 0 && strchr(str, '-' as i32).is_null()
        && mask != 0
    {
        av_channel_layout_from_mask(channel_layout, mask);
        return 0 as ::core::ffi::c_int;
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    channels = strtol(str, &raw mut end, 10 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if *__errno_location() == 0
        && strcmp(end, b"c\0" as *const u8 as *const ::core::ffi::c_char) == 0
        && channels > 0 as ::core::ffi::c_int
    {
        av_channel_layout_default(channel_layout, channels);
        if (*channel_layout).order as ::core::ffi::c_uint
            == AV_CHANNEL_ORDER_NATIVE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    if *__errno_location() == 0
        && (strcmp(end, b"C\0" as *const u8 as *const ::core::ffi::c_char) == 0
            || strcmp(end, b" channels\0" as *const u8 as *const ::core::ffi::c_char)
                == 0) && channels > 0 as ::core::ffi::c_int
    {
        (*channel_layout).order = AV_CHANNEL_ORDER_UNSPEC;
        (*channel_layout).nb_channels = channels;
        return 0 as ::core::ffi::c_int;
    }
    return -(22 as ::core::ffi::c_int);
}
#[inline(always)]
unsafe extern "C" fn av_popcount_c(mut x: uint32_t) -> ::core::ffi::c_int {
    x = x
        .wrapping_sub(
            x >> 1 as ::core::ffi::c_int & 0x55555555 as ::core::ffi::c_int as uint32_t,
        );
    x = (x & 0x33333333 as ::core::ffi::c_int as uint32_t)
        .wrapping_add(
            x >> 2 as ::core::ffi::c_int & 0x33333333 as ::core::ffi::c_int as uint32_t,
        );
    x = x.wrapping_add(x >> 4 as ::core::ffi::c_int)
        & 0xf0f0f0f as ::core::ffi::c_int as uint32_t;
    x = x.wrapping_add(x >> 8 as ::core::ffi::c_int);
    return (x.wrapping_add(x >> 16 as ::core::ffi::c_int) & 0x3f as uint32_t)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_channel_layout_uninit(
    mut channel_layout: *mut AVChannelLayout,
) {
    if (*channel_layout).order as ::core::ffi::c_uint
        == AV_CHANNEL_ORDER_CUSTOM as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        av_freep(&raw mut (*channel_layout).u.map as *mut ::core::ffi::c_void);
    }
    memset(
        channel_layout as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<AVChannelLayout>() as size_t,
    );
}
#[inline(always)]
unsafe extern "C" fn av_popcount64_c(mut x: uint64_t) -> ::core::ffi::c_int {
    return av_popcount_c(x as uint32_t)
        + av_popcount_c((x >> 32 as ::core::ffi::c_int) as uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn av_channel_layout_copy(
    mut dst: *mut AVChannelLayout,
    mut src: *const AVChannelLayout,
) -> ::core::ffi::c_int {
    av_channel_layout_uninit(dst);
    *dst = *src;
    if (*src).order as ::core::ffi::c_uint
        == AV_CHANNEL_ORDER_CUSTOM as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*dst).u.map = av_malloc_array(
            (*src).nb_channels as size_t,
            ::core::mem::size_of::<AVChannelCustom>() as size_t,
        ) as *mut AVChannelCustom;
        if (*dst).u.map.is_null() {
            return -(12 as ::core::ffi::c_int);
        }
        memcpy(
            (*dst).u.map as *mut ::core::ffi::c_void,
            (*src).u.map as *const ::core::ffi::c_void,
            ((*src).nb_channels as size_t)
                .wrapping_mul(::core::mem::size_of::<AVChannelCustom>() as size_t),
        );
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn masked_description(
    mut channel_layout: *const AVChannelLayout,
    mut start_channel: ::core::ffi::c_int,
) -> int64_t {
    let mut mask: uint64_t = 0 as uint64_t;
    let mut i: ::core::ffi::c_int = start_channel;
    while i < (*channel_layout).nb_channels {
        let mut ch: AVChannel = (*(*channel_layout).u.map.offset(i as isize)).id;
        if ch as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
            && (ch as ::core::ffi::c_int) < 63 as ::core::ffi::c_int
            && mask < (1 as ::core::ffi::c_ulonglong) << ch as ::core::ffi::c_int
        {
            mask |= (1 as ::core::ffi::c_ulonglong) << ch as ::core::ffi::c_int;
        } else {
            return -(22 as ::core::ffi::c_int) as int64_t
        }
        i += 1;
    }
    return mask as int64_t;
}
unsafe extern "C" fn has_channel_names(
    mut channel_layout: *const AVChannelLayout,
) -> ::core::ffi::c_int {
    if (*channel_layout).order as ::core::ffi::c_uint
        != AV_CHANNEL_ORDER_CUSTOM as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*channel_layout).nb_channels {
        if (*(*channel_layout).u.map.offset(i as isize))
            .name[0 as ::core::ffi::c_int as usize] != 0
        {
            return 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_channel_layout_ambisonic_order(
    mut channel_layout: *const AVChannelLayout,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut highest_ambi: ::core::ffi::c_int = 0;
    let mut order: ::core::ffi::c_int = 0;
    if (*channel_layout).order as ::core::ffi::c_uint
        != AV_CHANNEL_ORDER_AMBISONIC as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*channel_layout).order as ::core::ffi::c_uint
            != AV_CHANNEL_ORDER_CUSTOM as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return -(22 as ::core::ffi::c_int);
    }
    highest_ambi = -(1 as ::core::ffi::c_int);
    if (*channel_layout).order as ::core::ffi::c_uint
        == AV_CHANNEL_ORDER_AMBISONIC as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        highest_ambi = (*channel_layout).nb_channels
            - av_popcount64_c((*channel_layout).u.mask) - 1 as ::core::ffi::c_int;
    } else {
        let mut map: *const AVChannelCustom = (*channel_layout).u.map;
        if !((*channel_layout).order as ::core::ffi::c_uint
            == AV_CHANNEL_ORDER_CUSTOM as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            av_log(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                0 as ::core::ffi::c_int,
                b"Assertion %s failed at %s:%d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"channel_layout->order == AV_CHANNEL_ORDER_CUSTOM\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"src/libavutil/channel_layout.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                499 as ::core::ffi::c_int,
            );
            abort();
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*channel_layout).nb_channels {
            let mut is_ambi: ::core::ffi::c_int = ((*map.offset(i as isize)).id
                as ::core::ffi::c_int >= AV_CHAN_AMBISONIC_BASE as ::core::ffi::c_int
                && (*map.offset(i as isize)).id as ::core::ffi::c_int
                    <= AV_CHAN_AMBISONIC_END as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            if i > 0 as ::core::ffi::c_int && is_ambi != 0
                && !((*map.offset((i - 1 as ::core::ffi::c_int) as isize)).id
                    as ::core::ffi::c_int >= AV_CHAN_AMBISONIC_BASE as ::core::ffi::c_int
                    && (*map.offset((i - 1 as ::core::ffi::c_int) as isize)).id
                        as ::core::ffi::c_int
                        <= AV_CHAN_AMBISONIC_END as ::core::ffi::c_int)
            {
                return -(22 as ::core::ffi::c_int);
            }
            if is_ambi != 0
                && (*map.offset(i as isize)).id as ::core::ffi::c_int
                    - AV_CHAN_AMBISONIC_BASE as ::core::ffi::c_int != i
            {
                return -(22 as ::core::ffi::c_int);
            }
            if (*map.offset(i as isize)).id as ::core::ffi::c_int
                >= AV_CHAN_AMBISONIC_BASE as ::core::ffi::c_int
                && (*map.offset(i as isize)).id as ::core::ffi::c_int
                    <= AV_CHAN_AMBISONIC_END as ::core::ffi::c_int
            {
                highest_ambi = i;
            }
            i += 1;
        }
    }
    if highest_ambi < 0 as ::core::ffi::c_int {
        return -(22 as ::core::ffi::c_int);
    }
    order = floor(sqrt(highest_ambi as ::core::ffi::c_double)) as ::core::ffi::c_int;
    if (order + 1 as ::core::ffi::c_int) * (order + 1 as ::core::ffi::c_int)
        != highest_ambi + 1 as ::core::ffi::c_int
    {
        return -(22 as ::core::ffi::c_int);
    }
    return order;
}
unsafe extern "C" fn canonical_order(
    mut channel_layout: *mut AVChannelLayout,
) -> AVChannelOrder {
    let mut has_known_channel: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut order: ::core::ffi::c_int = 0;
    if (*channel_layout).order as ::core::ffi::c_uint
        != AV_CHANNEL_ORDER_CUSTOM as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (*channel_layout).order;
    }
    if has_channel_names(channel_layout) != 0 {
        return AV_CHANNEL_ORDER_CUSTOM;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*channel_layout).nb_channels && has_known_channel == 0 {
        if (*(*channel_layout).u.map.offset(i as isize)).id as ::core::ffi::c_int
            != AV_CHAN_UNKNOWN as ::core::ffi::c_int
        {
            has_known_channel = 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    if has_known_channel == 0 {
        return AV_CHANNEL_ORDER_UNSPEC;
    }
    if masked_description(channel_layout, 0 as ::core::ffi::c_int) > 0 as int64_t {
        return AV_CHANNEL_ORDER_NATIVE;
    }
    order = av_channel_layout_ambisonic_order(channel_layout);
    if order >= 0 as ::core::ffi::c_int
        && masked_description(
            channel_layout,
            (order + 1 as ::core::ffi::c_int) * (order + 1 as ::core::ffi::c_int),
        ) >= 0 as int64_t
    {
        return AV_CHANNEL_ORDER_AMBISONIC;
    }
    return AV_CHANNEL_ORDER_CUSTOM;
}
unsafe extern "C" fn try_describe_ambisonic(
    mut bp: *mut AVBPrint,
    mut channel_layout: *const AVChannelLayout,
) -> ::core::ffi::c_int {
    let mut nb_ambi_channels: ::core::ffi::c_int = 0;
    let mut order: ::core::ffi::c_int = av_channel_layout_ambisonic_order(
        channel_layout,
    );
    if order < 0 as ::core::ffi::c_int {
        return order;
    }
    av_bprintf(bp, b"ambisonic %d\0" as *const u8 as *const ::core::ffi::c_char, order);
    nb_ambi_channels = (order + 1 as ::core::ffi::c_int)
        * (order + 1 as ::core::ffi::c_int);
    if nb_ambi_channels < (*channel_layout).nb_channels {
        let mut extra: AVChannelLayout = AVChannelLayout {
            order: AV_CHANNEL_ORDER_UNSPEC,
            nb_channels: 0,
            u: C2RustUnnamed { mask: 0 },
            opaque: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        };
        if (*channel_layout).order as ::core::ffi::c_uint
            == AV_CHANNEL_ORDER_AMBISONIC as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            extra.order = AV_CHANNEL_ORDER_NATIVE;
            extra.nb_channels = av_popcount64_c((*channel_layout).u.mask);
            extra.u.mask = (*channel_layout).u.mask;
        } else {
            let mut mask: int64_t = 0;
            if has_channel_names(channel_layout) == 0
                && {
                    mask = masked_description(channel_layout, nb_ambi_channels);
                    mask > 0 as int64_t
                }
            {
                extra.order = AV_CHANNEL_ORDER_NATIVE;
                extra.nb_channels = av_popcount64_c(mask as uint64_t);
                extra.u.mask = mask as uint64_t;
            } else {
                extra.order = AV_CHANNEL_ORDER_CUSTOM;
                extra.nb_channels = (*channel_layout).nb_channels - nb_ambi_channels;
                extra.u.map = (*channel_layout).u.map.offset(nb_ambi_channels as isize);
            }
        }
        av_bprint_chars(bp, '+' as i32 as ::core::ffi::c_char, 1 as ::core::ffi::c_uint);
        av_channel_layout_describe_bprint(&raw mut extra, bp);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_channel_layout_describe_bprint(
    mut channel_layout: *const AVChannelLayout,
    mut bp: *mut AVBPrint,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut current_block_32: u64;
    match (*channel_layout).order as ::core::ffi::c_uint {
        1 => {
            i = 0 as ::core::ffi::c_int;
            while (i as usize)
                < (::core::mem::size_of::<[channel_layout_name; 40]>() as usize)
                    .wrapping_div(::core::mem::size_of::<channel_layout_name>() as usize)
            {
                if (*channel_layout).u.mask
                    == channel_layout_map[i as usize].layout.u.mask
                {
                    av_bprintf(
                        bp,
                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                        channel_layout_map[i as usize].name,
                    );
                    return 0 as ::core::ffi::c_int;
                }
                i += 1;
            }
            current_block_32 = 9555331055768489306;
        }
        2 => {
            current_block_32 = 9555331055768489306;
        }
        0 => {
            current_block_32 = 1856543081689279749;
        }
        3 => return try_describe_ambisonic(bp, channel_layout),
        _ => return -(22 as ::core::ffi::c_int),
    }
    match current_block_32 {
        9555331055768489306 => {
            if (*channel_layout).order as ::core::ffi::c_uint
                == AV_CHANNEL_ORDER_CUSTOM as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                let mut mask: int64_t = 0;
                let mut res: ::core::ffi::c_int = try_describe_ambisonic(
                    bp,
                    channel_layout,
                );
                if res >= 0 as ::core::ffi::c_int {
                    return 0 as ::core::ffi::c_int;
                }
                if has_channel_names(channel_layout) == 0
                    && {
                        mask = masked_description(
                            channel_layout,
                            0 as ::core::ffi::c_int,
                        );
                        mask > 0 as int64_t
                    }
                {
                    let mut native: AVChannelLayout = AVChannelLayout {
                        order: AV_CHANNEL_ORDER_NATIVE,
                        nb_channels: av_popcount64_c(mask as uint64_t),
                        u: C2RustUnnamed {
                            mask: mask as uint64_t,
                        },
                        opaque: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    };
                    return av_channel_layout_describe_bprint(&raw mut native, bp);
                }
            }
            if (*channel_layout).nb_channels != 0 {
                av_bprintf(
                    bp,
                    b"%d channels (\0" as *const u8 as *const ::core::ffi::c_char,
                    (*channel_layout).nb_channels,
                );
            }
            i = 0 as ::core::ffi::c_int;
            while i < (*channel_layout).nb_channels {
                let mut ch: AVChannel = av_channel_layout_channel_from_index(
                    channel_layout,
                    i as ::core::ffi::c_uint,
                );
                if i != 0 {
                    av_bprintf(bp, b"+\0" as *const u8 as *const ::core::ffi::c_char);
                }
                av_channel_name_bprint(bp, ch);
                if (*channel_layout).order as ::core::ffi::c_uint
                    == AV_CHANNEL_ORDER_CUSTOM as ::core::ffi::c_int
                        as ::core::ffi::c_uint
                    && (*(*channel_layout).u.map.offset(i as isize))
                        .name[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                        != 0
                {
                    av_bprintf(
                        bp,
                        b"@%s\0" as *const u8 as *const ::core::ffi::c_char,
                        &raw mut (*(*channel_layout).u.map.offset(i as isize)).name
                            as *mut ::core::ffi::c_char,
                    );
                }
                i += 1;
            }
            if (*channel_layout).nb_channels != 0 {
                av_bprintf(bp, b")\0" as *const u8 as *const ::core::ffi::c_char);
                return 0 as ::core::ffi::c_int;
            }
        }
        _ => {}
    }
    av_bprintf(
        bp,
        b"%d channels\0" as *const u8 as *const ::core::ffi::c_char,
        (*channel_layout).nb_channels,
    );
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_channel_layout_describe(
    mut channel_layout: *const AVChannelLayout,
    mut buf: *mut ::core::ffi::c_char,
    mut buf_size: size_t,
) -> ::core::ffi::c_int {
    let mut bp: AVBPrint = AVBPrint {
        str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        len: 0,
        size: 0,
        size_max: 0,
        reserved_internal_buffer: [0; 1],
        reserved_padding: [0; 1004],
    };
    let mut ret: ::core::ffi::c_int = 0;
    if buf.is_null() && buf_size != 0 {
        return -(22 as ::core::ffi::c_int);
    }
    av_bprint_init_for_buffer(&raw mut bp, buf, buf_size as ::core::ffi::c_uint);
    ret = av_channel_layout_describe_bprint(channel_layout, &raw mut bp);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    if bp.len >= 2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint {
        return -(34 as ::core::ffi::c_int);
    }
    return bp.len.wrapping_add(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_channel_layout_channel_from_index(
    mut channel_layout: *const AVChannelLayout,
    mut idx: ::core::ffi::c_uint,
) -> AVChannel {
    let mut i: ::core::ffi::c_int = 0;
    if idx >= (*channel_layout).nb_channels as ::core::ffi::c_uint {
        return AV_CHAN_NONE;
    }
    let mut current_block_14: u64;
    match (*channel_layout).order as ::core::ffi::c_uint {
        2 => return (*(*channel_layout).u.map.offset(idx as isize)).id,
        3 => {
            let mut ambi_channels: ::core::ffi::c_int = (*channel_layout).nb_channels
                - av_popcount64_c((*channel_layout).u.mask);
            if idx < ambi_channels as ::core::ffi::c_uint {
                return (AV_CHAN_AMBISONIC_BASE as ::core::ffi::c_int
                    as ::core::ffi::c_uint)
                    .wrapping_add(idx) as AVChannel;
            }
            idx = idx.wrapping_sub(ambi_channels as ::core::ffi::c_uint);
            current_block_14 = 15433940732114890699;
        }
        1 => {
            current_block_14 = 15433940732114890699;
        }
        _ => {
            current_block_14 = 10370040694135445195;
        }
    }
    match current_block_14 {
        15433940732114890699 => {
            i = 0 as ::core::ffi::c_int;
            while i < 64 as ::core::ffi::c_int {
                if (1 as uint64_t) << i & (*channel_layout).u.mask != 0
                    && {
                        let fresh0 = idx;
                        idx = idx.wrapping_sub(1);
                        fresh0 == 0
                    }
                {
                    return i as AVChannel;
                }
                i += 1;
            }
        }
        _ => {}
    }
    return AV_CHAN_NONE;
}
#[no_mangle]
pub unsafe extern "C" fn av_channel_layout_channel_from_string(
    mut channel_layout: *const AVChannelLayout,
    mut str: *const ::core::ffi::c_char,
) -> AVChannel {
    let mut index: ::core::ffi::c_int = av_channel_layout_index_from_string(
        channel_layout,
        str,
    );
    if index < 0 as ::core::ffi::c_int {
        return AV_CHAN_NONE;
    }
    return av_channel_layout_channel_from_index(
        channel_layout,
        index as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_channel_layout_index_from_channel(
    mut channel_layout: *const AVChannelLayout,
    mut channel: AVChannel,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    if channel as ::core::ffi::c_int == AV_CHAN_NONE as ::core::ffi::c_int {
        return -(22 as ::core::ffi::c_int);
    }
    match (*channel_layout).order as ::core::ffi::c_uint {
        2 => {
            i = 0 as ::core::ffi::c_int;
            while i < (*channel_layout).nb_channels {
                if (*(*channel_layout).u.map.offset(i as isize)).id as ::core::ffi::c_int
                    == channel as ::core::ffi::c_int
                {
                    return i;
                }
                i += 1;
            }
            return -(22 as ::core::ffi::c_int);
        }
        3 | 1 => {
            let mut mask: uint64_t = (*channel_layout).u.mask;
            let mut ambi_channels: ::core::ffi::c_int = (*channel_layout).nb_channels
                - av_popcount64_c(mask);
            if (*channel_layout).order as ::core::ffi::c_uint
                == AV_CHANNEL_ORDER_AMBISONIC as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                && channel as ::core::ffi::c_int
                    >= AV_CHAN_AMBISONIC_BASE as ::core::ffi::c_int
            {
                if channel as ::core::ffi::c_int
                    - AV_CHAN_AMBISONIC_BASE as ::core::ffi::c_int >= ambi_channels
                {
                    return -(22 as ::core::ffi::c_int);
                }
                return channel as ::core::ffi::c_int
                    - AV_CHAN_AMBISONIC_BASE as ::core::ffi::c_int;
            }
            if channel as ::core::ffi::c_uint > 63 as ::core::ffi::c_uint
                || mask as ::core::ffi::c_ulonglong
                    & (1 as ::core::ffi::c_ulonglong) << channel as ::core::ffi::c_int
                    == 0
            {
                return -(22 as ::core::ffi::c_int);
            }
            mask
                &= ((1 as ::core::ffi::c_ulonglong) << channel as ::core::ffi::c_int)
                    .wrapping_sub(1 as ::core::ffi::c_ulonglong);
            return av_popcount64_c(mask) + ambi_channels;
        }
        _ => return -(22 as ::core::ffi::c_int),
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_channel_layout_index_from_string(
    mut channel_layout: *const AVChannelLayout,
    mut str: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut chname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut ch: AVChannel = AV_CHAN_NONE;
    's_90: {
        match (*channel_layout).order as ::core::ffi::c_uint {
            2 => {
                chname = strstr(str, b"@\0" as *const u8 as *const ::core::ffi::c_char);
                if !chname.is_null() {
                    let mut buf: [::core::ffi::c_char; 16] = [0; 16];
                    chname = chname.offset(1);
                    av_strlcpy(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        str,
                        if ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as usize
                            > chname.offset_from(str) as ::core::ffi::c_int as usize
                        {
                            chname.offset_from(str) as ::core::ffi::c_int as size_t
                        } else {
                            ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as size_t
                        },
                    );
                    if *chname == 0 {
                        chname = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                    ch = av_channel_from_string(
                        &raw mut buf as *mut ::core::ffi::c_char,
                    );
                    if ch as ::core::ffi::c_int == AV_CHAN_NONE as ::core::ffi::c_int
                        && *(&raw mut buf as *mut ::core::ffi::c_char)
                            as ::core::ffi::c_int != 0
                    {
                        return -(22 as ::core::ffi::c_int);
                    }
                }
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while !chname.is_null() && i < (*channel_layout).nb_channels {
                    if strcmp(
                        chname,
                        &raw mut (*(*channel_layout).u.map.offset(i as isize)).name
                            as *mut ::core::ffi::c_char,
                    ) == 0
                        && (ch as ::core::ffi::c_int
                            == AV_CHAN_NONE as ::core::ffi::c_int
                            || ch as ::core::ffi::c_int
                                == (*(*channel_layout).u.map.offset(i as isize)).id
                                    as ::core::ffi::c_int)
                    {
                        return i;
                    }
                    i += 1;
                }
            }
            3 | 1 => {}
            _ => {
                break 's_90;
            }
        }
        ch = av_channel_from_string(str);
        if ch as ::core::ffi::c_int == AV_CHAN_NONE as ::core::ffi::c_int {
            return -(22 as ::core::ffi::c_int);
        }
        return av_channel_layout_index_from_channel(channel_layout, ch);
    }
    return -(22 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn av_channel_layout_check(
    mut channel_layout: *const AVChannelLayout,
) -> ::core::ffi::c_int {
    if (*channel_layout).nb_channels <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    match (*channel_layout).order as ::core::ffi::c_uint {
        1 => {
            return (av_popcount64_c((*channel_layout).u.mask)
                == (*channel_layout).nb_channels) as ::core::ffi::c_int;
        }
        2 => {
            if (*channel_layout).u.map.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < (*channel_layout).nb_channels {
                if (*(*channel_layout).u.map.offset(i as isize)).id as ::core::ffi::c_int
                    == AV_CHAN_NONE as ::core::ffi::c_int
                {
                    return 0 as ::core::ffi::c_int;
                }
                i += 1;
            }
            return 1 as ::core::ffi::c_int;
        }
        3 => {
            return (av_popcount64_c((*channel_layout).u.mask)
                < (*channel_layout).nb_channels) as ::core::ffi::c_int;
        }
        0 => return 1 as ::core::ffi::c_int,
        _ => return 0 as ::core::ffi::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_channel_layout_compare(
    mut chl: *const AVChannelLayout,
    mut chl1: *const AVChannelLayout,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    if (*chl).nb_channels != (*chl1).nb_channels {
        return 1 as ::core::ffi::c_int;
    }
    if ((*chl).order as ::core::ffi::c_uint
        == AV_CHANNEL_ORDER_UNSPEC as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int
        != ((*chl1).order as ::core::ffi::c_uint
            == AV_CHANNEL_ORDER_UNSPEC as ::core::ffi::c_int as ::core::ffi::c_uint)
            as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int
    } else if (*chl).order as ::core::ffi::c_uint
        == AV_CHANNEL_ORDER_UNSPEC as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int
    }
    if ((*chl).order as ::core::ffi::c_uint
        == AV_CHANNEL_ORDER_NATIVE as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*chl).order as ::core::ffi::c_uint
            == AV_CHANNEL_ORDER_AMBISONIC as ::core::ffi::c_int as ::core::ffi::c_uint)
        && (*chl).order as ::core::ffi::c_uint == (*chl1).order as ::core::ffi::c_uint
    {
        return ((*chl).u.mask != (*chl1).u.mask) as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*chl).nb_channels {
        if av_channel_layout_channel_from_index(chl, i as ::core::ffi::c_uint)
            as ::core::ffi::c_int
            != av_channel_layout_channel_from_index(chl1, i as ::core::ffi::c_uint)
                as ::core::ffi::c_int
        {
            return 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_channel_layout_default(
    mut ch_layout: *mut AVChannelLayout,
    mut nb_channels: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while (i as usize)
        < (::core::mem::size_of::<[channel_layout_name; 40]>() as usize)
            .wrapping_div(::core::mem::size_of::<channel_layout_name>() as usize)
    {
        if nb_channels == channel_layout_map[i as usize].layout.nb_channels {
            *ch_layout = channel_layout_map[i as usize].layout;
            return;
        }
        i += 1;
    }
    (*ch_layout).order = AV_CHANNEL_ORDER_UNSPEC;
    (*ch_layout).nb_channels = nb_channels;
}
#[no_mangle]
pub unsafe extern "C" fn av_channel_layout_standard(
    mut opaque: *mut *mut ::core::ffi::c_void,
) -> *const AVChannelLayout {
    let mut i: uintptr_t = *opaque as uintptr_t;
    let mut ch_layout: *const AVChannelLayout = ::core::ptr::null::<AVChannelLayout>();
    if (i as usize)
        < (::core::mem::size_of::<[channel_layout_name; 40]>() as usize)
            .wrapping_div(::core::mem::size_of::<channel_layout_name>() as usize)
    {
        ch_layout = &raw const (*(&raw const channel_layout_map
            as *const channel_layout_name)
            .offset(i as isize))
            .layout;
        *opaque = i.wrapping_add(1 as uintptr_t) as *mut ::core::ffi::c_void;
    }
    return ch_layout;
}
#[no_mangle]
pub unsafe extern "C" fn av_channel_layout_subset(
    mut channel_layout: *const AVChannelLayout,
    mut mask: uint64_t,
) -> uint64_t {
    let mut ret: uint64_t = 0 as uint64_t;
    let mut i: ::core::ffi::c_int = 0;
    match (*channel_layout).order as ::core::ffi::c_uint {
        1 | 3 => return (*channel_layout).u.mask & mask,
        2 => {
            i = 0 as ::core::ffi::c_int;
            while i < 64 as ::core::ffi::c_int {
                if mask as ::core::ffi::c_ulonglong
                    & (1 as ::core::ffi::c_ulonglong) << i != 0
                    && av_channel_layout_index_from_channel(
                        channel_layout,
                        i as AVChannel,
                    ) >= 0 as ::core::ffi::c_int
                {
                    ret |= (1 as ::core::ffi::c_ulonglong) << i;
                }
                i += 1;
            }
        }
        _ => {}
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_channel_layout_retype(
    mut channel_layout: *mut AVChannelLayout,
    mut order: AVChannelOrder,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut allow_lossy: ::core::ffi::c_int = (flags
        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int == 0)
        as ::core::ffi::c_int;
    let mut lossy: ::core::ffi::c_int = 0;
    if av_channel_layout_check(channel_layout) == 0 {
        return -(22 as ::core::ffi::c_int);
    }
    if flags & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0 {
        order = canonical_order(channel_layout);
    }
    if (*channel_layout).order as ::core::ffi::c_uint == order as ::core::ffi::c_uint {
        return 0 as ::core::ffi::c_int;
    }
    match order as ::core::ffi::c_uint {
        0 => {
            let mut nb_channels: ::core::ffi::c_int = (*channel_layout).nb_channels;
            if (*channel_layout).order as ::core::ffi::c_uint
                == AV_CHANNEL_ORDER_CUSTOM as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                lossy = 0 as ::core::ffi::c_int;
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i < nb_channels {
                    if (*(*channel_layout).u.map.offset(i as isize)).id
                        as ::core::ffi::c_int != AV_CHAN_UNKNOWN as ::core::ffi::c_int
                        || (*(*channel_layout).u.map.offset(i as isize))
                            .name[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                            != 0
                    {
                        lossy = 1 as ::core::ffi::c_int;
                        break;
                    } else {
                        i += 1;
                    }
                }
            } else {
                lossy = 1 as ::core::ffi::c_int;
            }
            if lossy == 0 || allow_lossy != 0 {
                let mut opaque: *mut ::core::ffi::c_void = (*channel_layout).opaque;
                av_channel_layout_uninit(channel_layout);
                (*channel_layout).order = AV_CHANNEL_ORDER_UNSPEC;
                (*channel_layout).nb_channels = nb_channels;
                (*channel_layout).opaque = opaque;
                return lossy;
            }
            return -(38 as ::core::ffi::c_int);
        }
        1 => {
            if (*channel_layout).order as ::core::ffi::c_uint
                == AV_CHANNEL_ORDER_CUSTOM as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                let mut mask: int64_t = masked_description(
                    channel_layout,
                    0 as ::core::ffi::c_int,
                );
                if mask < 0 as int64_t {
                    return -(38 as ::core::ffi::c_int);
                }
                lossy = has_channel_names(channel_layout);
                if lossy == 0 || allow_lossy != 0 {
                    let mut opaque_0: *mut ::core::ffi::c_void = (*channel_layout)
                        .opaque;
                    av_channel_layout_uninit(channel_layout);
                    av_channel_layout_from_mask(channel_layout, mask as uint64_t);
                    (*channel_layout).opaque = opaque_0;
                    return lossy;
                }
            }
            return -(38 as ::core::ffi::c_int);
        }
        2 => {
            let mut custom: AVChannelLayout = AVChannelLayout {
                order: AV_CHANNEL_ORDER_UNSPEC,
                nb_channels: 0,
                u: C2RustUnnamed { mask: 0 },
                opaque: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            };
            let mut ret: ::core::ffi::c_int = av_channel_layout_custom_init(
                &raw mut custom,
                (*channel_layout).nb_channels,
            );
            let mut opaque_1: *mut ::core::ffi::c_void = (*channel_layout).opaque;
            if ret < 0 as ::core::ffi::c_int {
                return ret;
            }
            if (*channel_layout).order as ::core::ffi::c_uint
                != AV_CHANNEL_ORDER_UNSPEC as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_0 < (*channel_layout).nb_channels {
                    (*custom.u.map.offset(i_0 as isize)).id = av_channel_layout_channel_from_index(
                        channel_layout,
                        i_0 as ::core::ffi::c_uint,
                    );
                    i_0 += 1;
                }
            }
            av_channel_layout_uninit(channel_layout);
            *channel_layout = custom;
            (*channel_layout).opaque = opaque_1;
            return 0 as ::core::ffi::c_int;
        }
        3 => {
            if (*channel_layout).order as ::core::ffi::c_uint
                == AV_CHANNEL_ORDER_CUSTOM as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                let mut mask_0: int64_t = 0;
                let mut nb_channels_0: ::core::ffi::c_int = (*channel_layout)
                    .nb_channels;
                let mut amb_order: ::core::ffi::c_int = av_channel_layout_ambisonic_order(
                    channel_layout,
                );
                if amb_order < 0 as ::core::ffi::c_int {
                    return -(38 as ::core::ffi::c_int);
                }
                mask_0 = masked_description(
                    channel_layout,
                    (amb_order + 1 as ::core::ffi::c_int)
                        * (amb_order + 1 as ::core::ffi::c_int),
                );
                if mask_0 < 0 as int64_t {
                    return -(38 as ::core::ffi::c_int);
                }
                lossy = has_channel_names(channel_layout);
                if lossy == 0 || allow_lossy != 0 {
                    let mut opaque_2: *mut ::core::ffi::c_void = (*channel_layout)
                        .opaque;
                    av_channel_layout_uninit(channel_layout);
                    (*channel_layout).order = AV_CHANNEL_ORDER_AMBISONIC;
                    (*channel_layout).nb_channels = nb_channels_0;
                    (*channel_layout).u.mask = mask_0 as uint64_t;
                    (*channel_layout).opaque = opaque_2;
                    return lossy;
                }
            }
            return -(38 as ::core::ffi::c_int);
        }
        _ => return -(22 as ::core::ffi::c_int),
    };
}
