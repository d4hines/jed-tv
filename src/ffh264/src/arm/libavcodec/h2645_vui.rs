// Translated from FFmpeg 9.0.1 libavcodec/h2645_vui.c
// (preprocessed for armv7-unknown-linux-musleabihf) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// Common H.264/HEVC VUI Parameter decoding
//
// Copyright (c) 2003 Michael Niedermayer <michaelni@gmx.at>
// Copyright (C) 2012 - 2013 Guillaume Martres
// Copyright (C) 2012 - 2013 Mickael Raulet
// Copyright (C) 2012 - 2013 Gildas Cocherel
// Copyright (C) 2013 Vittorio Giovara
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
    static ff_h2645_pixel_aspect: [AVRational; 17];
    static ff_golomb_vlc_len: [uint8_t; 512];
    static ff_ue_golomb_vlc_code: [uint8_t; 512];
    fn av_color_primaries_name(
        primaries: AVColorPrimaries,
    ) -> *const ::core::ffi::c_char;
    fn av_color_transfer_name(
        transfer: AVColorTransferCharacteristic,
    ) -> *const ::core::ffi::c_char;
    fn av_color_space_name(space: AVColorSpace) -> *const ::core::ffi::c_char;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
}
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint32_t = ::core::ffi::c_uint;
pub type AVColorPrimaries = ::core::ffi::c_uint;
pub const AVCOL_PRI_EXT_NB: AVColorPrimaries = 257;
pub const AVCOL_PRI_V_GAMUT: AVColorPrimaries = 256;
pub const AVCOL_PRI_EXT_BASE: AVColorPrimaries = 256;
pub const AVCOL_PRI_NB: AVColorPrimaries = 23;
pub const AVCOL_PRI_JEDEC_P22: AVColorPrimaries = 22;
pub const AVCOL_PRI_EBU3213: AVColorPrimaries = 22;
pub const AVCOL_PRI_SMPTE432: AVColorPrimaries = 12;
pub const AVCOL_PRI_SMPTE431: AVColorPrimaries = 11;
pub const AVCOL_PRI_SMPTEST428_1: AVColorPrimaries = 10;
pub const AVCOL_PRI_SMPTE428: AVColorPrimaries = 10;
pub const AVCOL_PRI_BT2020: AVColorPrimaries = 9;
pub const AVCOL_PRI_FILM: AVColorPrimaries = 8;
pub const AVCOL_PRI_SMPTE240M: AVColorPrimaries = 7;
pub const AVCOL_PRI_SMPTE170M: AVColorPrimaries = 6;
pub const AVCOL_PRI_BT470BG: AVColorPrimaries = 5;
pub const AVCOL_PRI_BT470M: AVColorPrimaries = 4;
pub const AVCOL_PRI_RESERVED: AVColorPrimaries = 3;
pub const AVCOL_PRI_UNSPECIFIED: AVColorPrimaries = 2;
pub const AVCOL_PRI_BT709: AVColorPrimaries = 1;
pub const AVCOL_PRI_RESERVED0: AVColorPrimaries = 0;
pub type AVColorTransferCharacteristic = ::core::ffi::c_uint;
pub const AVCOL_TRC_EXT_NB: AVColorTransferCharacteristic = 257;
pub const AVCOL_TRC_V_LOG: AVColorTransferCharacteristic = 256;
pub const AVCOL_TRC_EXT_BASE: AVColorTransferCharacteristic = 256;
pub const AVCOL_TRC_NB: AVColorTransferCharacteristic = 19;
pub const AVCOL_TRC_ARIB_STD_B67: AVColorTransferCharacteristic = 18;
pub const AVCOL_TRC_SMPTEST428_1: AVColorTransferCharacteristic = 17;
pub const AVCOL_TRC_SMPTE428: AVColorTransferCharacteristic = 17;
pub const AVCOL_TRC_SMPTEST2084: AVColorTransferCharacteristic = 16;
pub const AVCOL_TRC_SMPTE2084: AVColorTransferCharacteristic = 16;
pub const AVCOL_TRC_BT2020_12: AVColorTransferCharacteristic = 15;
pub const AVCOL_TRC_BT2020_10: AVColorTransferCharacteristic = 14;
pub const AVCOL_TRC_IEC61966_2_1: AVColorTransferCharacteristic = 13;
pub const AVCOL_TRC_BT1361_ECG: AVColorTransferCharacteristic = 12;
pub const AVCOL_TRC_IEC61966_2_4: AVColorTransferCharacteristic = 11;
pub const AVCOL_TRC_LOG_SQRT: AVColorTransferCharacteristic = 10;
pub const AVCOL_TRC_LOG: AVColorTransferCharacteristic = 9;
pub const AVCOL_TRC_LINEAR: AVColorTransferCharacteristic = 8;
pub const AVCOL_TRC_SMPTE240M: AVColorTransferCharacteristic = 7;
pub const AVCOL_TRC_SMPTE170M: AVColorTransferCharacteristic = 6;
pub const AVCOL_TRC_GAMMA28: AVColorTransferCharacteristic = 5;
pub const AVCOL_TRC_GAMMA22: AVColorTransferCharacteristic = 4;
pub const AVCOL_TRC_RESERVED: AVColorTransferCharacteristic = 3;
pub const AVCOL_TRC_UNSPECIFIED: AVColorTransferCharacteristic = 2;
pub const AVCOL_TRC_BT709: AVColorTransferCharacteristic = 1;
pub const AVCOL_TRC_RESERVED0: AVColorTransferCharacteristic = 0;
pub type AVColorSpace = ::core::ffi::c_uint;
pub const AVCOL_SPC_NB: AVColorSpace = 18;
pub const AVCOL_SPC_YCGCO_RO: AVColorSpace = 17;
pub const AVCOL_SPC_YCGCO_RE: AVColorSpace = 16;
pub const AVCOL_SPC_IPT_C2: AVColorSpace = 15;
pub const AVCOL_SPC_ICTCP: AVColorSpace = 14;
pub const AVCOL_SPC_CHROMA_DERIVED_CL: AVColorSpace = 13;
pub const AVCOL_SPC_CHROMA_DERIVED_NCL: AVColorSpace = 12;
pub const AVCOL_SPC_SMPTE2085: AVColorSpace = 11;
pub const AVCOL_SPC_BT2020_CL: AVColorSpace = 10;
pub const AVCOL_SPC_BT2020_NCL: AVColorSpace = 9;
pub const AVCOL_SPC_YCOCG: AVColorSpace = 8;
pub const AVCOL_SPC_YCGCO: AVColorSpace = 8;
pub const AVCOL_SPC_SMPTE240M: AVColorSpace = 7;
pub const AVCOL_SPC_SMPTE170M: AVColorSpace = 6;
pub const AVCOL_SPC_BT470BG: AVColorSpace = 5;
pub const AVCOL_SPC_FCC: AVColorSpace = 4;
pub const AVCOL_SPC_RESERVED: AVColorSpace = 3;
pub const AVCOL_SPC_UNSPECIFIED: AVColorSpace = 2;
pub const AVCOL_SPC_BT709: AVColorSpace = 1;
pub const AVCOL_SPC_RGB: AVColorSpace = 0;
pub type AVChromaLocation = ::core::ffi::c_uint;
pub const AVCHROMA_LOC_NB: AVChromaLocation = 7;
pub const AVCHROMA_LOC_BOTTOM: AVChromaLocation = 6;
pub const AVCHROMA_LOC_BOTTOMLEFT: AVChromaLocation = 5;
pub const AVCHROMA_LOC_TOP: AVChromaLocation = 4;
pub const AVCHROMA_LOC_TOPLEFT: AVChromaLocation = 3;
pub const AVCHROMA_LOC_CENTER: AVChromaLocation = 2;
pub const AVCHROMA_LOC_LEFT: AVChromaLocation = 1;
pub const AVCHROMA_LOC_UNSPECIFIED: AVChromaLocation = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: ::core::ffi::c_int,
    pub den: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union unaligned_32 {
    pub l: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GetBitContext {
    pub buffer: *const uint8_t,
    pub index: ::core::ffi::c_int,
    pub size_in_bits: ::core::ffi::c_int,
    pub size_in_bits_plus8: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2645VUI {
    pub sar: AVRational,
    pub aspect_ratio_idc: ::core::ffi::c_int,
    pub aspect_ratio_info_present_flag: ::core::ffi::c_int,
    pub overscan_info_present_flag: ::core::ffi::c_int,
    pub overscan_appropriate_flag: ::core::ffi::c_int,
    pub video_signal_type_present_flag: ::core::ffi::c_int,
    pub video_format: ::core::ffi::c_int,
    pub video_full_range_flag: ::core::ffi::c_int,
    pub colour_description_present_flag: ::core::ffi::c_int,
    pub colour_primaries: AVColorPrimaries,
    pub transfer_characteristics: AVColorTransferCharacteristic,
    pub matrix_coeffs: AVColorSpace,
    pub chroma_loc_info_present_flag: ::core::ffi::c_int,
    pub chroma_sample_loc_type_top_field: ::core::ffi::c_int,
    pub chroma_sample_loc_type_bottom_field: ::core::ffi::c_int,
    pub chroma_location: AVChromaLocation,
}
#[no_mangle]
pub unsafe extern "C" fn ff_h2645_decode_common_vui_params(
    mut gb: *mut GetBitContext,
    mut vui: *mut H2645VUI,
    mut logctx: *mut ::core::ffi::c_void,
) {
    av_log(
        logctx,
        48 as ::core::ffi::c_int,
        b"Decoding VUI\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*vui).aspect_ratio_info_present_flag = get_bits1(gb) as ::core::ffi::c_int;
    if (*vui).aspect_ratio_info_present_flag != 0 {
        (*vui).aspect_ratio_idc = get_bits(gb, 8 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        if ((*vui).aspect_ratio_idc as usize)
            < (::core::mem::size_of::<[AVRational; 17]>() as usize)
                .wrapping_div(::core::mem::size_of::<AVRational>() as usize)
        {
            (*vui).sar = ff_h2645_pixel_aspect[(*vui).aspect_ratio_idc as usize];
        } else if (*vui).aspect_ratio_idc == 255 as ::core::ffi::c_int {
            (*vui).sar.num = get_bits(gb, 16 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            (*vui).sar.den = get_bits(gb, 16 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            if (*vui).sar.num == 0 || (*vui).sar.den == 0 {
                av_log(
                    logctx,
                    24 as ::core::ffi::c_int,
                    b"Invalid SAR %d/%d in bitstream, treating as unspecified.\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    (*vui).sar.num,
                    (*vui).sar.den,
                );
                (*vui).sar = AVRational {
                    num: 0 as ::core::ffi::c_int,
                    den: 1 as ::core::ffi::c_int,
                };
            }
        } else {
            av_log(
                logctx,
                24 as ::core::ffi::c_int,
                b"Unknown SAR index: %u.\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*vui).aspect_ratio_idc,
            );
        }
    } else {
        (*vui).sar = AVRational {
            num: 0 as ::core::ffi::c_int,
            den: 1 as ::core::ffi::c_int,
        };
    }
    (*vui).overscan_info_present_flag = get_bits1(gb) as ::core::ffi::c_int;
    if (*vui).overscan_info_present_flag != 0 {
        (*vui).overscan_appropriate_flag = get_bits1(gb) as ::core::ffi::c_int;
    }
    (*vui).video_signal_type_present_flag = get_bits1(gb) as ::core::ffi::c_int;
    if (*vui).video_signal_type_present_flag != 0 {
        (*vui).video_format = get_bits(gb, 3 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        (*vui).video_full_range_flag = get_bits1(gb) as ::core::ffi::c_int;
        (*vui).colour_description_present_flag = get_bits1(gb) as ::core::ffi::c_int;
        if (*vui).colour_description_present_flag != 0 {
            (*vui).colour_primaries = get_bits(gb, 8 as ::core::ffi::c_int)
                as AVColorPrimaries;
            (*vui).transfer_characteristics = get_bits(gb, 8 as ::core::ffi::c_int)
                as AVColorTransferCharacteristic;
            (*vui).matrix_coeffs = get_bits(gb, 8 as ::core::ffi::c_int) as AVColorSpace;
            if (*vui).colour_primaries as ::core::ffi::c_uint
                == AVCOL_PRI_RESERVED0 as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*vui).colour_primaries as ::core::ffi::c_uint
                    == AVCOL_PRI_RESERVED as ::core::ffi::c_int as ::core::ffi::c_uint
                || av_color_primaries_name((*vui).colour_primaries).is_null()
            {
                (*vui).colour_primaries = AVCOL_PRI_UNSPECIFIED;
            }
            if (*vui).transfer_characteristics as ::core::ffi::c_uint
                == AVCOL_TRC_RESERVED0 as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*vui).transfer_characteristics as ::core::ffi::c_uint
                    == AVCOL_TRC_RESERVED as ::core::ffi::c_int as ::core::ffi::c_uint
                || av_color_transfer_name((*vui).transfer_characteristics).is_null()
            {
                (*vui).transfer_characteristics = AVCOL_TRC_UNSPECIFIED;
            }
            if (*vui).matrix_coeffs as ::core::ffi::c_uint
                == AVCOL_SPC_RESERVED as ::core::ffi::c_int as ::core::ffi::c_uint
                || av_color_space_name((*vui).matrix_coeffs).is_null()
            {
                (*vui).matrix_coeffs = AVCOL_SPC_UNSPECIFIED;
            }
        }
    }
    (*vui).chroma_loc_info_present_flag = get_bits1(gb) as ::core::ffi::c_int;
    if (*vui).chroma_loc_info_present_flag != 0 {
        (*vui).chroma_sample_loc_type_top_field = get_ue_golomb_31(gb);
        (*vui).chroma_sample_loc_type_bottom_field = get_ue_golomb_31(gb);
        if (*vui).chroma_sample_loc_type_top_field as ::core::ffi::c_uint
            <= 5 as ::core::ffi::c_uint
        {
            (*vui).chroma_location = ((*vui).chroma_sample_loc_type_top_field
                + 1 as ::core::ffi::c_int) as AVChromaLocation;
        } else {
            (*vui).chroma_location = AVCHROMA_LOC_UNSPECIFIED;
        }
    } else {
        (*vui).chroma_location = AVCHROMA_LOC_LEFT;
    };
}
#[inline(always)]
unsafe extern "C" fn av_bswap32(mut x: uint32_t) -> uint32_t {
    return (x << 8 as ::core::ffi::c_int & 0xff00 as uint32_t
        | x >> 8 as ::core::ffi::c_int & 0xff as uint32_t) << 16 as ::core::ffi::c_int
        | ((x >> 16 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            & 0xff00 as uint32_t
            | x >> 16 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
                & 0xff as uint32_t);
}
#[inline]
unsafe extern "C" fn get_ue_golomb_31(mut gb: *mut GetBitContext) -> ::core::ffi::c_int {
    let mut buf: ::core::ffi::c_uint = 0;
    let mut re_index: ::core::ffi::c_uint = (*gb).index as ::core::ffi::c_uint;
    let mut re_cache: ::core::ffi::c_uint = 0;
    let mut re_size_plus8: ::core::ffi::c_uint = (*gb).size_in_bits_plus8
        as ::core::ffi::c_uint;
    re_cache = (av_bswap32(
        (*((*gb).buffer.offset((re_index >> 3 as ::core::ffi::c_int) as isize)
            as *const unaligned_32))
            .l,
    ) << (re_index & 7 as ::core::ffi::c_uint)
        >> 32 as ::core::ffi::c_int - 32 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    buf = re_cache as ::core::ffi::c_uint;
    buf >>= 32 as ::core::ffi::c_int - 9 as ::core::ffi::c_int;
    re_index = if re_size_plus8
        > re_index.wrapping_add(ff_golomb_vlc_len[buf as usize] as ::core::ffi::c_uint)
    {
        re_index.wrapping_add(ff_golomb_vlc_len[buf as usize] as ::core::ffi::c_uint)
    } else {
        re_size_plus8
    };
    (*gb).index = re_index as ::core::ffi::c_int;
    return ff_ue_golomb_vlc_code[buf as usize] as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn get_bits(
    mut s: *mut GetBitContext,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    let mut tmp: ::core::ffi::c_uint = 0;
    let mut re_index: ::core::ffi::c_uint = (*s).index as ::core::ffi::c_uint;
    let mut re_cache: ::core::ffi::c_uint = 0;
    let mut re_size_plus8: ::core::ffi::c_uint = (*s).size_in_bits_plus8
        as ::core::ffi::c_uint;
    re_cache = (av_bswap32(
        (*((*s).buffer.offset((re_index >> 3 as ::core::ffi::c_int) as isize)
            as *const unaligned_32))
            .l,
    ) << (re_index & 7 as ::core::ffi::c_uint)
        >> 32 as ::core::ffi::c_int - 32 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    tmp = (re_cache >> 32 as ::core::ffi::c_int - n) as ::core::ffi::c_uint;
    re_index = if re_size_plus8 > re_index.wrapping_add(n as ::core::ffi::c_uint) {
        re_index.wrapping_add(n as ::core::ffi::c_uint)
    } else {
        re_size_plus8
    };
    (*s).index = re_index as ::core::ffi::c_int;
    return tmp;
}
#[inline]
unsafe extern "C" fn get_bits1(mut s: *mut GetBitContext) -> ::core::ffi::c_uint {
    let mut index: ::core::ffi::c_uint = (*s).index as ::core::ffi::c_uint;
    let mut result: uint8_t = *(*s)
        .buffer
        .offset((index >> 3 as ::core::ffi::c_int) as isize);
    result = ((result as ::core::ffi::c_int) << (index & 7 as ::core::ffi::c_uint))
        as uint8_t;
    result = (result as ::core::ffi::c_int
        >> 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as uint8_t;
    if (*s).index < (*s).size_in_bits_plus8 {
        index = index.wrapping_add(1);
    }
    (*s).index = index as ::core::ffi::c_int;
    return result as ::core::ffi::c_uint;
}
