// Translated from FFmpeg 9.0.1 libavcodec/h264data.c
// (preprocessed for armv7-unknown-linux-musleabihf) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// H.26L/H.264/AVC/JVT/14496-10/... encoder/decoder
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

pub type uint8_t = ::core::ffi::c_uchar;
pub type uint16_t = ::core::ffi::c_ushort;
pub type AVPictureType = ::core::ffi::c_uint;
pub const AV_PICTURE_TYPE_BI: AVPictureType = 7;
pub const AV_PICTURE_TYPE_SP: AVPictureType = 6;
pub const AV_PICTURE_TYPE_SI: AVPictureType = 5;
pub const AV_PICTURE_TYPE_S: AVPictureType = 4;
pub const AV_PICTURE_TYPE_B: AVPictureType = 3;
pub const AV_PICTURE_TYPE_P: AVPictureType = 2;
pub const AV_PICTURE_TYPE_I: AVPictureType = 1;
pub const AV_PICTURE_TYPE_NONE: AVPictureType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IMbInfo {
    pub type_0: uint16_t,
    pub pred_mode: uint8_t,
    pub cbp: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PMbInfo {
    pub type_0: uint16_t,
    pub partition_count: uint8_t,
}
#[no_mangle]
pub static mut ff_h264_golomb_to_pict_type: [uint8_t; 5] = [
    AV_PICTURE_TYPE_P as ::core::ffi::c_int as uint8_t,
    AV_PICTURE_TYPE_B as ::core::ffi::c_int as uint8_t,
    AV_PICTURE_TYPE_I as ::core::ffi::c_int as uint8_t,
    AV_PICTURE_TYPE_SP as ::core::ffi::c_int as uint8_t,
    AV_PICTURE_TYPE_SI as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
pub static mut ff_h264_golomb_to_intra4x4_cbp: [uint8_t; 48] = [
    47 as ::core::ffi::c_int as uint8_t,
    31 as ::core::ffi::c_int as uint8_t,
    15 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    23 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    29 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    7 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    13 as ::core::ffi::c_int as uint8_t,
    14 as ::core::ffi::c_int as uint8_t,
    39 as ::core::ffi::c_int as uint8_t,
    43 as ::core::ffi::c_int as uint8_t,
    45 as ::core::ffi::c_int as uint8_t,
    46 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    10 as ::core::ffi::c_int as uint8_t,
    12 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    21 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    28 as ::core::ffi::c_int as uint8_t,
    35 as ::core::ffi::c_int as uint8_t,
    37 as ::core::ffi::c_int as uint8_t,
    42 as ::core::ffi::c_int as uint8_t,
    44 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    8 as ::core::ffi::c_int as uint8_t,
    17 as ::core::ffi::c_int as uint8_t,
    18 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    25 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    33 as ::core::ffi::c_int as uint8_t,
    34 as ::core::ffi::c_int as uint8_t,
    36 as ::core::ffi::c_int as uint8_t,
    40 as ::core::ffi::c_int as uint8_t,
    38 as ::core::ffi::c_int as uint8_t,
    41 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
pub static mut ff_h264_golomb_to_inter_cbp: [uint8_t; 48] = [
    0 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    8 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    10 as ::core::ffi::c_int as uint8_t,
    12 as ::core::ffi::c_int as uint8_t,
    15 as ::core::ffi::c_int as uint8_t,
    47 as ::core::ffi::c_int as uint8_t,
    7 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    13 as ::core::ffi::c_int as uint8_t,
    14 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    31 as ::core::ffi::c_int as uint8_t,
    35 as ::core::ffi::c_int as uint8_t,
    37 as ::core::ffi::c_int as uint8_t,
    42 as ::core::ffi::c_int as uint8_t,
    44 as ::core::ffi::c_int as uint8_t,
    33 as ::core::ffi::c_int as uint8_t,
    34 as ::core::ffi::c_int as uint8_t,
    36 as ::core::ffi::c_int as uint8_t,
    40 as ::core::ffi::c_int as uint8_t,
    39 as ::core::ffi::c_int as uint8_t,
    43 as ::core::ffi::c_int as uint8_t,
    45 as ::core::ffi::c_int as uint8_t,
    46 as ::core::ffi::c_int as uint8_t,
    17 as ::core::ffi::c_int as uint8_t,
    18 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    21 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    28 as ::core::ffi::c_int as uint8_t,
    23 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    29 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    25 as ::core::ffi::c_int as uint8_t,
    38 as ::core::ffi::c_int as uint8_t,
    41 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
pub static mut ff_h264_chroma_dc_scan: [uint8_t; 4] = [
    ((0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
        * 16 as ::core::ffi::c_int) as uint8_t,
    ((1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
        * 16 as ::core::ffi::c_int) as uint8_t,
    ((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
        * 16 as ::core::ffi::c_int) as uint8_t,
    ((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
        * 16 as ::core::ffi::c_int) as uint8_t,
];
#[no_mangle]
pub static mut ff_h264_chroma422_dc_scan: [uint8_t; 8] = [
    ((0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
        * 16 as ::core::ffi::c_int) as uint8_t,
    ((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
        * 16 as ::core::ffi::c_int) as uint8_t,
    ((1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
        * 16 as ::core::ffi::c_int) as uint8_t,
    ((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
        * 16 as ::core::ffi::c_int) as uint8_t,
    ((0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
        * 16 as ::core::ffi::c_int) as uint8_t,
    ((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
        * 16 as ::core::ffi::c_int) as uint8_t,
    ((1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
        * 16 as ::core::ffi::c_int) as uint8_t,
    ((1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
        * 16 as ::core::ffi::c_int) as uint8_t,
];
#[no_mangle]
pub static mut ff_h264_i_mb_type_info: [IMbInfo; 26] = [
    IMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as uint16_t,
        pred_mode: -(1 as ::core::ffi::c_int) as uint8_t,
        cbp: -(1 as ::core::ffi::c_int) as uint8_t,
    },
    IMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint16_t,
        pred_mode: 2 as uint8_t,
        cbp: 0 as uint8_t,
    },
    IMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint16_t,
        pred_mode: 1 as uint8_t,
        cbp: 0 as uint8_t,
    },
    IMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint16_t,
        pred_mode: 0 as uint8_t,
        cbp: 0 as uint8_t,
    },
    IMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint16_t,
        pred_mode: 3 as uint8_t,
        cbp: 0 as uint8_t,
    },
    IMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint16_t,
        pred_mode: 2 as uint8_t,
        cbp: 16 as uint8_t,
    },
    IMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint16_t,
        pred_mode: 1 as uint8_t,
        cbp: 16 as uint8_t,
    },
    IMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint16_t,
        pred_mode: 0 as uint8_t,
        cbp: 16 as uint8_t,
    },
    IMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint16_t,
        pred_mode: 3 as uint8_t,
        cbp: 16 as uint8_t,
    },
    IMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint16_t,
        pred_mode: 2 as uint8_t,
        cbp: 32 as uint8_t,
    },
    IMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint16_t,
        pred_mode: 1 as uint8_t,
        cbp: 32 as uint8_t,
    },
    IMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint16_t,
        pred_mode: 0 as uint8_t,
        cbp: 32 as uint8_t,
    },
    IMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint16_t,
        pred_mode: 3 as uint8_t,
        cbp: 32 as uint8_t,
    },
    IMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint16_t,
        pred_mode: 2 as uint8_t,
        cbp: (15 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as uint8_t,
    },
    IMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint16_t,
        pred_mode: 1 as uint8_t,
        cbp: (15 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as uint8_t,
    },
    IMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint16_t,
        pred_mode: 0 as uint8_t,
        cbp: (15 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as uint8_t,
    },
    IMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint16_t,
        pred_mode: 3 as uint8_t,
        cbp: (15 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as uint8_t,
    },
    IMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint16_t,
        pred_mode: 2 as uint8_t,
        cbp: (15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as uint8_t,
    },
    IMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint16_t,
        pred_mode: 1 as uint8_t,
        cbp: (15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as uint8_t,
    },
    IMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint16_t,
        pred_mode: 0 as uint8_t,
        cbp: (15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as uint8_t,
    },
    IMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint16_t,
        pred_mode: 3 as uint8_t,
        cbp: (15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as uint8_t,
    },
    IMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint16_t,
        pred_mode: 2 as uint8_t,
        cbp: (15 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as uint8_t,
    },
    IMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint16_t,
        pred_mode: 1 as uint8_t,
        cbp: (15 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as uint8_t,
    },
    IMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint16_t,
        pred_mode: 0 as uint8_t,
        cbp: (15 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as uint8_t,
    },
    IMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint16_t,
        pred_mode: 3 as uint8_t,
        cbp: (15 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as uint8_t,
    },
    IMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as uint16_t,
        pred_mode: -(1 as ::core::ffi::c_int) as uint8_t,
        cbp: -(1 as ::core::ffi::c_int) as uint8_t,
    },
];
#[no_mangle]
pub static mut ff_h264_p_mb_type_info: [PMbInfo; 5] = [
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int) as uint16_t,
        partition_count: 1 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as uint16_t,
        partition_count: 2 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as uint16_t,
        partition_count: 2 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as uint16_t,
        partition_count: 4 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as uint16_t,
        partition_count: 4 as uint8_t,
    },
];
#[no_mangle]
pub static mut ff_h264_p_sub_mb_type_info: [PMbInfo; 4] = [
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int) as uint16_t,
        partition_count: 1 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int) as uint16_t,
        partition_count: 2 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int) as uint16_t,
        partition_count: 2 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int) as uint16_t,
        partition_count: 4 as uint8_t,
    },
];
#[no_mangle]
pub static mut ff_h264_b_mb_type_info: [PMbInfo; 23] = [
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)))
            as uint16_t,
        partition_count: 1 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int) as uint16_t,
        partition_count: 1 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int) as uint16_t,
        partition_count: 1 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int) as uint16_t,
        partition_count: 1 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as uint16_t,
        partition_count: 2 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as uint16_t,
        partition_count: 2 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as uint16_t,
        partition_count: 2 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as uint16_t,
        partition_count: 2 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as uint16_t,
        partition_count: 2 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as uint16_t,
        partition_count: 2 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as uint16_t,
        partition_count: 2 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as uint16_t,
        partition_count: 2 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as uint16_t,
        partition_count: 2 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as uint16_t,
        partition_count: 2 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as uint16_t,
        partition_count: 2 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as uint16_t,
        partition_count: 2 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as uint16_t,
        partition_count: 2 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as uint16_t,
        partition_count: 2 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as uint16_t,
        partition_count: 2 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as uint16_t,
        partition_count: 2 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as uint16_t,
        partition_count: 2 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as uint16_t,
        partition_count: 2 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as uint16_t,
        partition_count: 4 as uint8_t,
    },
];
#[no_mangle]
pub static mut ff_h264_b_sub_mb_type_info: [PMbInfo; 13] = [
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint16_t,
        partition_count: 1 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int) as uint16_t,
        partition_count: 1 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int) as uint16_t,
        partition_count: 1 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int) as uint16_t,
        partition_count: 1 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as uint16_t,
        partition_count: 2 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as uint16_t,
        partition_count: 2 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as uint16_t,
        partition_count: 2 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as uint16_t,
        partition_count: 2 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as uint16_t,
        partition_count: 2 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as uint16_t,
        partition_count: 2 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as uint16_t,
        partition_count: 4 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as uint16_t,
        partition_count: 4 as uint8_t,
    },
    PMbInfo {
        type_0: ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as uint16_t,
        partition_count: 4 as uint8_t,
    },
];
#[no_mangle]
pub static mut ff_h264_dequant4_coeff_init: [[uint8_t; 3]; 6] = [
    [
        10 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        16 as ::core::ffi::c_int as uint8_t,
    ],
    [
        11 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
        18 as ::core::ffi::c_int as uint8_t,
    ],
    [
        13 as ::core::ffi::c_int as uint8_t,
        16 as ::core::ffi::c_int as uint8_t,
        20 as ::core::ffi::c_int as uint8_t,
    ],
    [
        14 as ::core::ffi::c_int as uint8_t,
        18 as ::core::ffi::c_int as uint8_t,
        23 as ::core::ffi::c_int as uint8_t,
    ],
    [
        16 as ::core::ffi::c_int as uint8_t,
        20 as ::core::ffi::c_int as uint8_t,
        25 as ::core::ffi::c_int as uint8_t,
    ],
    [
        18 as ::core::ffi::c_int as uint8_t,
        23 as ::core::ffi::c_int as uint8_t,
        29 as ::core::ffi::c_int as uint8_t,
    ],
];
#[no_mangle]
pub static mut ff_h264_dequant8_coeff_init_scan: [uint8_t; 16] = [
    0 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
pub static mut ff_h264_dequant8_coeff_init: [[uint8_t; 6]; 6] = [
    [
        20 as ::core::ffi::c_int as uint8_t,
        18 as ::core::ffi::c_int as uint8_t,
        32 as ::core::ffi::c_int as uint8_t,
        19 as ::core::ffi::c_int as uint8_t,
        25 as ::core::ffi::c_int as uint8_t,
        24 as ::core::ffi::c_int as uint8_t,
    ],
    [
        22 as ::core::ffi::c_int as uint8_t,
        19 as ::core::ffi::c_int as uint8_t,
        35 as ::core::ffi::c_int as uint8_t,
        21 as ::core::ffi::c_int as uint8_t,
        28 as ::core::ffi::c_int as uint8_t,
        26 as ::core::ffi::c_int as uint8_t,
    ],
    [
        26 as ::core::ffi::c_int as uint8_t,
        23 as ::core::ffi::c_int as uint8_t,
        42 as ::core::ffi::c_int as uint8_t,
        24 as ::core::ffi::c_int as uint8_t,
        33 as ::core::ffi::c_int as uint8_t,
        31 as ::core::ffi::c_int as uint8_t,
    ],
    [
        28 as ::core::ffi::c_int as uint8_t,
        25 as ::core::ffi::c_int as uint8_t,
        45 as ::core::ffi::c_int as uint8_t,
        26 as ::core::ffi::c_int as uint8_t,
        35 as ::core::ffi::c_int as uint8_t,
        33 as ::core::ffi::c_int as uint8_t,
    ],
    [
        32 as ::core::ffi::c_int as uint8_t,
        28 as ::core::ffi::c_int as uint8_t,
        51 as ::core::ffi::c_int as uint8_t,
        30 as ::core::ffi::c_int as uint8_t,
        40 as ::core::ffi::c_int as uint8_t,
        38 as ::core::ffi::c_int as uint8_t,
    ],
    [
        36 as ::core::ffi::c_int as uint8_t,
        32 as ::core::ffi::c_int as uint8_t,
        58 as ::core::ffi::c_int as uint8_t,
        34 as ::core::ffi::c_int as uint8_t,
        46 as ::core::ffi::c_int as uint8_t,
        43 as ::core::ffi::c_int as uint8_t,
    ],
];
#[no_mangle]
pub static mut ff_h264_quant_rem6: [uint8_t; 88] = [
    0 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
pub static mut ff_h264_quant_div6: [uint8_t; 88] = [
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    7 as ::core::ffi::c_int as uint8_t,
    7 as ::core::ffi::c_int as uint8_t,
    7 as ::core::ffi::c_int as uint8_t,
    7 as ::core::ffi::c_int as uint8_t,
    7 as ::core::ffi::c_int as uint8_t,
    7 as ::core::ffi::c_int as uint8_t,
    8 as ::core::ffi::c_int as uint8_t,
    8 as ::core::ffi::c_int as uint8_t,
    8 as ::core::ffi::c_int as uint8_t,
    8 as ::core::ffi::c_int as uint8_t,
    8 as ::core::ffi::c_int as uint8_t,
    8 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    10 as ::core::ffi::c_int as uint8_t,
    10 as ::core::ffi::c_int as uint8_t,
    10 as ::core::ffi::c_int as uint8_t,
    10 as ::core::ffi::c_int as uint8_t,
    10 as ::core::ffi::c_int as uint8_t,
    10 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    12 as ::core::ffi::c_int as uint8_t,
    12 as ::core::ffi::c_int as uint8_t,
    12 as ::core::ffi::c_int as uint8_t,
    12 as ::core::ffi::c_int as uint8_t,
    12 as ::core::ffi::c_int as uint8_t,
    12 as ::core::ffi::c_int as uint8_t,
    13 as ::core::ffi::c_int as uint8_t,
    13 as ::core::ffi::c_int as uint8_t,
    13 as ::core::ffi::c_int as uint8_t,
    13 as ::core::ffi::c_int as uint8_t,
    13 as ::core::ffi::c_int as uint8_t,
    13 as ::core::ffi::c_int as uint8_t,
    14 as ::core::ffi::c_int as uint8_t,
    14 as ::core::ffi::c_int as uint8_t,
    14 as ::core::ffi::c_int as uint8_t,
    14 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
pub static mut ff_h264_chroma_qp: [[uint8_t; 88]; 7] = [
    [
        (0 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (1 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (2 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (3 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (4 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (5 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (6 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (7 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (8 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (9 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (10 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (11 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (12 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (13 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (14 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (15 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (16 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (17 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (18 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (19 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (20 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (21 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (22 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (23 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (24 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (25 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (26 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (27 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (28 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (29 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (29 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (30 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (31 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (32 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (32 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (33 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (34 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (34 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (35 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (35 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (36 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (36 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (37 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (37 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (37 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (38 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (38 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (38 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (39 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (39 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (39 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (39 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
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
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    [
        0 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        (0 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (1 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (2 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (3 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (4 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (5 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (6 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (7 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (8 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (9 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (10 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (11 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (12 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (13 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (14 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (15 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (16 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (17 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (18 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (19 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (20 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (21 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (22 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (23 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (24 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (25 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (26 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (27 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (28 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (29 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (29 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (30 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (31 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (32 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (32 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (33 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (34 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (34 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (35 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (35 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (36 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (36 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (37 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (37 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (37 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (38 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (38 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (38 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (39 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (39 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (39 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (39 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (9 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
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
        0,
        0,
    ],
    [
        0 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        (0 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (1 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (2 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (3 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (4 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (5 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (6 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (7 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (8 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (9 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (10 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (11 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (12 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (13 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (14 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (15 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (16 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (17 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (18 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (19 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (20 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (21 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (22 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (23 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (24 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (25 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (26 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (27 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (28 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (29 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (29 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (30 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (31 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (32 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (32 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (33 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (34 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (34 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (35 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (35 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (36 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (36 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (37 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (37 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (37 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (38 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (38 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (38 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (39 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (39 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (39 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (39 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
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
    ],
    [
        0 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        16 as ::core::ffi::c_int as uint8_t,
        17 as ::core::ffi::c_int as uint8_t,
        (0 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (1 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (2 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (3 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (4 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (5 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (6 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (7 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (8 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (9 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (10 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (11 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (12 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (13 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (14 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (15 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (16 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (17 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (18 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (19 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (20 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (21 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (22 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (23 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (24 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (25 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (26 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (27 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (28 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (29 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (29 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (30 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (31 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (32 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (32 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (33 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (34 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (34 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (35 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (35 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (36 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (36 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (37 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (37 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (37 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (38 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (38 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (38 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (39 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (39 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (39 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (39 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (11 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
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
    ],
    [
        0 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        16 as ::core::ffi::c_int as uint8_t,
        17 as ::core::ffi::c_int as uint8_t,
        18 as ::core::ffi::c_int as uint8_t,
        19 as ::core::ffi::c_int as uint8_t,
        20 as ::core::ffi::c_int as uint8_t,
        21 as ::core::ffi::c_int as uint8_t,
        22 as ::core::ffi::c_int as uint8_t,
        23 as ::core::ffi::c_int as uint8_t,
        (0 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (1 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (2 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (3 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (4 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (5 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (6 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (7 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (8 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (9 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (10 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (11 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (12 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (13 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (14 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (15 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (16 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (17 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (18 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (19 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (20 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (21 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (22 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (23 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (24 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (25 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (26 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (27 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (28 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (29 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (29 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (30 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (31 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (32 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (32 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (33 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (34 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (34 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (35 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (35 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (36 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (36 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (37 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (37 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (37 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (38 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (38 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (38 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (39 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (39 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (39 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (39 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (12 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
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
    ],
    [
        0 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        16 as ::core::ffi::c_int as uint8_t,
        17 as ::core::ffi::c_int as uint8_t,
        18 as ::core::ffi::c_int as uint8_t,
        19 as ::core::ffi::c_int as uint8_t,
        20 as ::core::ffi::c_int as uint8_t,
        21 as ::core::ffi::c_int as uint8_t,
        22 as ::core::ffi::c_int as uint8_t,
        23 as ::core::ffi::c_int as uint8_t,
        24 as ::core::ffi::c_int as uint8_t,
        25 as ::core::ffi::c_int as uint8_t,
        26 as ::core::ffi::c_int as uint8_t,
        27 as ::core::ffi::c_int as uint8_t,
        28 as ::core::ffi::c_int as uint8_t,
        29 as ::core::ffi::c_int as uint8_t,
        (0 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (1 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (2 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (3 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (4 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (5 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (6 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (7 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (8 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (9 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (10 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (11 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (12 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (13 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (14 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (15 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (16 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (17 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (18 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (19 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (20 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (21 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (22 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (23 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (24 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (25 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (26 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (27 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (28 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (29 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (29 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (30 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (31 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (32 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (32 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (33 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (34 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (34 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (35 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (35 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (36 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (36 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (37 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (37 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (37 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (38 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (38 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (38 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (39 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (39 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (39 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (39 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (13 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    [
        0 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        16 as ::core::ffi::c_int as uint8_t,
        17 as ::core::ffi::c_int as uint8_t,
        18 as ::core::ffi::c_int as uint8_t,
        19 as ::core::ffi::c_int as uint8_t,
        20 as ::core::ffi::c_int as uint8_t,
        21 as ::core::ffi::c_int as uint8_t,
        22 as ::core::ffi::c_int as uint8_t,
        23 as ::core::ffi::c_int as uint8_t,
        24 as ::core::ffi::c_int as uint8_t,
        25 as ::core::ffi::c_int as uint8_t,
        26 as ::core::ffi::c_int as uint8_t,
        27 as ::core::ffi::c_int as uint8_t,
        28 as ::core::ffi::c_int as uint8_t,
        29 as ::core::ffi::c_int as uint8_t,
        30 as ::core::ffi::c_int as uint8_t,
        31 as ::core::ffi::c_int as uint8_t,
        32 as ::core::ffi::c_int as uint8_t,
        33 as ::core::ffi::c_int as uint8_t,
        34 as ::core::ffi::c_int as uint8_t,
        35 as ::core::ffi::c_int as uint8_t,
        (0 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (1 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (2 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (3 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (4 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (5 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (6 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (7 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (8 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (9 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (10 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (11 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (12 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (13 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (14 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (15 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (16 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (17 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (18 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (19 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (20 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (21 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (22 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (23 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (24 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (25 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (26 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (27 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (28 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (29 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (29 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (30 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (31 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (32 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (32 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (33 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (34 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (34 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (35 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (35 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (36 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (36 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (37 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (37 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (37 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (38 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (38 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (38 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (39 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (39 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (39 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
        (39 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)) as uint8_t,
    ],
];
