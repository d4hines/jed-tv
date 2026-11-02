// Translated from FFmpeg 9.0.1 libavcodec/dovi_rpu.c
// (preprocessed for armv7-unknown-linux-musleabihf) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// Dolby Vision RPU decoder
//
// Copyright (C) 2021 Jan Ekström
// Copyright (C) 2021-2024 Niklas Haas
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
    fn av_refstruct_unref(objp: *mut ::core::ffi::c_void);
    fn av_refstruct_replace(
        dstp: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
    );
    fn av_free(ptr: *mut ::core::ffi::c_void);
}
pub type int16_t = ::core::ffi::c_short;
pub type int64_t = ::core::ffi::c_longlong;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint16_t = ::core::ffi::c_ushort;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint64_t = ::core::ffi::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: ::core::ffi::c_int,
    pub den: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCIExy {
    pub x: AVRational,
    pub y: AVRational,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPrimaryCoefficients {
    pub r: AVCIExy,
    pub g: AVCIExy,
    pub b: AVCIExy,
}
pub type AVWhitepointCoefficients = AVCIExy;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVColorPrimariesDesc {
    pub wp: AVWhitepointCoefficients,
    pub prim: AVPrimaryCoefficients,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIDecoderConfigurationRecord {
    pub dv_version_major: uint8_t,
    pub dv_version_minor: uint8_t,
    pub dv_profile: uint8_t,
    pub dv_level: uint8_t,
    pub rpu_present_flag: uint8_t,
    pub el_present_flag: uint8_t,
    pub bl_present_flag: uint8_t,
    pub dv_bl_signal_compatibility_id: uint8_t,
    pub dv_md_compression: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIRpuDataHeader {
    pub rpu_type: uint8_t,
    pub rpu_format: uint16_t,
    pub vdr_rpu_profile: uint8_t,
    pub vdr_rpu_level: uint8_t,
    pub chroma_resampling_explicit_filter_flag: uint8_t,
    pub coef_data_type: uint8_t,
    pub coef_log2_denom: uint8_t,
    pub vdr_rpu_normalized_idc: uint8_t,
    pub bl_video_full_range_flag: uint8_t,
    pub bl_bit_depth: uint8_t,
    pub el_bit_depth: uint8_t,
    pub vdr_bit_depth: uint8_t,
    pub spatial_resampling_filter_flag: uint8_t,
    pub el_spatial_resampling_filter_flag: uint8_t,
    pub disable_residual_flag: uint8_t,
    pub ext_mapping_idc_0_4: uint8_t,
    pub ext_mapping_idc_5_7: uint8_t,
}
pub type AVDOVIMappingMethod = ::core::ffi::c_uint;
pub const AV_DOVI_MAPPING_MMR: AVDOVIMappingMethod = 1;
pub const AV_DOVI_MAPPING_POLYNOMIAL: AVDOVIMappingMethod = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIReshapingCurve {
    pub num_pivots: uint8_t,
    pub pivots: [uint16_t; 9],
    pub mapping_idc: [AVDOVIMappingMethod; 8],
    pub poly_order: [uint8_t; 8],
    pub poly_coef: [[int64_t; 3]; 8],
    pub mmr_order: [uint8_t; 8],
    pub mmr_constant: [int64_t; 8],
    pub mmr_coef: [[[int64_t; 7]; 3]; 8],
}
pub type AVDOVINLQMethod = ::core::ffi::c_int;
pub const AV_DOVI_NLQ_LINEAR_DZ: AVDOVINLQMethod = 0;
pub const AV_DOVI_NLQ_NONE: AVDOVINLQMethod = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVINLQParams {
    pub nlq_offset: uint16_t,
    pub vdr_in_max: uint64_t,
    pub linear_deadzone_slope: uint64_t,
    pub linear_deadzone_threshold: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIDataMapping {
    pub vdr_rpu_id: uint8_t,
    pub mapping_color_space: uint8_t,
    pub mapping_chroma_format_idc: uint8_t,
    pub curves: [AVDOVIReshapingCurve; 3],
    pub nlq_method_idc: AVDOVINLQMethod,
    pub num_x_partitions: uint32_t,
    pub num_y_partitions: uint32_t,
    pub nlq: [AVDOVINLQParams; 3],
    pub nlq_pivots: [uint16_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIColorMetadata {
    pub dm_metadata_id: uint8_t,
    pub scene_refresh_flag: uint8_t,
    pub ycc_to_rgb_matrix: [AVRational; 9],
    pub ycc_to_rgb_offset: [AVRational; 3],
    pub rgb_to_lms_matrix: [AVRational; 9],
    pub signal_eotf: uint16_t,
    pub signal_eotf_param0: uint16_t,
    pub signal_eotf_param1: uint16_t,
    pub signal_eotf_param2: uint32_t,
    pub signal_bit_depth: uint8_t,
    pub signal_color_space: uint8_t,
    pub signal_chroma_format: uint8_t,
    pub signal_full_range_flag: uint8_t,
    pub source_min_pq: uint16_t,
    pub source_max_pq: uint16_t,
    pub source_diagonal: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIDmLevel1 {
    pub min_pq: uint16_t,
    pub max_pq: uint16_t,
    pub avg_pq: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIDmLevel2 {
    pub target_max_pq: uint16_t,
    pub trim_slope: uint16_t,
    pub trim_offset: uint16_t,
    pub trim_power: uint16_t,
    pub trim_chroma_weight: uint16_t,
    pub trim_saturation_gain: uint16_t,
    pub ms_weight: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIDmLevel3 {
    pub min_pq_offset: uint16_t,
    pub max_pq_offset: uint16_t,
    pub avg_pq_offset: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIDmLevel4 {
    pub anchor_pq: uint16_t,
    pub anchor_power: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIDmLevel5 {
    pub left_offset: uint16_t,
    pub right_offset: uint16_t,
    pub top_offset: uint16_t,
    pub bottom_offset: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIDmLevel6 {
    pub max_luminance: uint16_t,
    pub min_luminance: uint16_t,
    pub max_cll: uint16_t,
    pub max_fall: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIDmLevel8 {
    pub target_display_index: uint8_t,
    pub trim_slope: uint16_t,
    pub trim_offset: uint16_t,
    pub trim_power: uint16_t,
    pub trim_chroma_weight: uint16_t,
    pub trim_saturation_gain: uint16_t,
    pub ms_weight: uint16_t,
    pub target_mid_contrast: uint16_t,
    pub clip_trim: uint16_t,
    pub saturation_vector_field: [uint8_t; 6],
    pub hue_vector_field: [uint8_t; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIDmLevel9 {
    pub source_primary_index: uint8_t,
    pub source_display_primaries: AVColorPrimariesDesc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIDmLevel10 {
    pub target_display_index: uint8_t,
    pub target_max_pq: uint16_t,
    pub target_min_pq: uint16_t,
    pub target_primary_index: uint8_t,
    pub target_display_primaries: AVColorPrimariesDesc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIDmLevel11 {
    pub content_type: uint8_t,
    pub whitepoint: uint8_t,
    pub reference_mode_flag: uint8_t,
    pub sharpness: uint8_t,
    pub noise_reduction: uint8_t,
    pub mpeg_noise_reduction: uint8_t,
    pub frame_rate_conversion: uint8_t,
    pub brightness: uint8_t,
    pub color: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIDmLevel254 {
    pub dm_mode: uint8_t,
    pub dm_version_index: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIDmLevel255 {
    pub dm_run_mode: uint8_t,
    pub dm_run_version: uint8_t,
    pub dm_debug: [uint8_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIDmData {
    pub level: uint8_t,
    pub c2rust_unnamed: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub l1: AVDOVIDmLevel1,
    pub l2: AVDOVIDmLevel2,
    pub l3: AVDOVIDmLevel3,
    pub l4: AVDOVIDmLevel4,
    pub l5: AVDOVIDmLevel5,
    pub l6: AVDOVIDmLevel6,
    pub l8: AVDOVIDmLevel8,
    pub l9: AVDOVIDmLevel9,
    pub l10: AVDOVIDmLevel10,
    pub l11: AVDOVIDmLevel11,
    pub l254: AVDOVIDmLevel254,
    pub l255: AVDOVIDmLevel255,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DOVIExt {
    pub dm_static: [AVDOVIDmData; 7],
    pub dm_dynamic: [AVDOVIDmData; 25],
    pub num_static: ::core::ffi::c_int,
    pub num_dynamic: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DOVIContext {
    pub logctx: *mut ::core::ffi::c_void,
    pub enable: ::core::ffi::c_int,
    pub cfg: AVDOVIDecoderConfigurationRecord,
    pub header: AVDOVIRpuDataHeader,
    pub mapping: *const AVDOVIDataMapping,
    pub color: *const AVDOVIColorMetadata,
    pub ext_blocks: *mut DOVIExt,
    pub dm: *mut AVDOVIColorMetadata,
    pub vdr: [*mut AVDOVIDataMapping; 16],
    pub rpu_buf: *mut uint8_t,
    pub rpu_buf_sz: ::core::ffi::c_uint,
}
#[no_mangle]
pub unsafe extern "C" fn ff_dovi_ctx_unref(mut s: *mut DOVIContext) {
    av_refstruct_unref(&raw mut (*s).dm as *mut ::core::ffi::c_void);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (i as usize)
        < (::core::mem::size_of::<[*mut AVDOVIDataMapping; 16]>() as usize)
            .wrapping_div(::core::mem::size_of::<*mut AVDOVIDataMapping>() as usize)
    {
        av_refstruct_unref(
            (&raw mut (*s).vdr as *mut *mut AVDOVIDataMapping).offset(i as isize)
                as *mut *mut AVDOVIDataMapping as *mut ::core::ffi::c_void,
        );
        i += 1;
    }
    av_refstruct_unref(&raw mut (*s).ext_blocks as *mut ::core::ffi::c_void);
    av_free((*s).rpu_buf as *mut ::core::ffi::c_void);
    *s = DOVIContext {
        logctx: (*s).logctx,
        enable: 0,
        cfg: AVDOVIDecoderConfigurationRecord {
            dv_version_major: 0,
            dv_version_minor: 0,
            dv_profile: 0,
            dv_level: 0,
            rpu_present_flag: 0,
            el_present_flag: 0,
            bl_present_flag: 0,
            dv_bl_signal_compatibility_id: 0,
            dv_md_compression: 0,
        },
        header: AVDOVIRpuDataHeader {
            rpu_type: 0,
            rpu_format: 0,
            vdr_rpu_profile: 0,
            vdr_rpu_level: 0,
            chroma_resampling_explicit_filter_flag: 0,
            coef_data_type: 0,
            coef_log2_denom: 0,
            vdr_rpu_normalized_idc: 0,
            bl_video_full_range_flag: 0,
            bl_bit_depth: 0,
            el_bit_depth: 0,
            vdr_bit_depth: 0,
            spatial_resampling_filter_flag: 0,
            el_spatial_resampling_filter_flag: 0,
            disable_residual_flag: 0,
            ext_mapping_idc_0_4: 0,
            ext_mapping_idc_5_7: 0,
        },
        mapping: ::core::ptr::null::<AVDOVIDataMapping>(),
        color: ::core::ptr::null::<AVDOVIColorMetadata>(),
        ext_blocks: ::core::ptr::null_mut::<DOVIExt>(),
        dm: ::core::ptr::null_mut::<AVDOVIColorMetadata>(),
        vdr: [::core::ptr::null_mut::<AVDOVIDataMapping>(); 16],
        rpu_buf: ::core::ptr::null_mut::<uint8_t>(),
        rpu_buf_sz: 0,
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ff_dovi_ctx_flush(mut s: *mut DOVIContext) {
    av_refstruct_unref(&raw mut (*s).dm as *mut ::core::ffi::c_void);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (i as usize)
        < (::core::mem::size_of::<[*mut AVDOVIDataMapping; 16]>() as usize)
            .wrapping_div(::core::mem::size_of::<*mut AVDOVIDataMapping>() as usize)
    {
        av_refstruct_unref(
            (&raw mut (*s).vdr as *mut *mut AVDOVIDataMapping).offset(i as isize)
                as *mut *mut AVDOVIDataMapping as *mut ::core::ffi::c_void,
        );
        i += 1;
    }
    av_refstruct_unref(&raw mut (*s).ext_blocks as *mut ::core::ffi::c_void);
    *s = DOVIContext {
        logctx: (*s).logctx,
        enable: 0,
        cfg: (*s).cfg,
        header: AVDOVIRpuDataHeader {
            rpu_type: 0,
            rpu_format: 0,
            vdr_rpu_profile: 0,
            vdr_rpu_level: 0,
            chroma_resampling_explicit_filter_flag: 0,
            coef_data_type: 0,
            coef_log2_denom: 0,
            vdr_rpu_normalized_idc: 0,
            bl_video_full_range_flag: 0,
            bl_bit_depth: 0,
            el_bit_depth: 0,
            vdr_bit_depth: 0,
            spatial_resampling_filter_flag: 0,
            el_spatial_resampling_filter_flag: 0,
            disable_residual_flag: 0,
            ext_mapping_idc_0_4: 0,
            ext_mapping_idc_5_7: 0,
        },
        mapping: ::core::ptr::null::<AVDOVIDataMapping>(),
        color: ::core::ptr::null::<AVDOVIColorMetadata>(),
        ext_blocks: ::core::ptr::null_mut::<DOVIExt>(),
        dm: ::core::ptr::null_mut::<AVDOVIColorMetadata>(),
        vdr: [::core::ptr::null_mut::<AVDOVIDataMapping>(); 16],
        rpu_buf: (*s).rpu_buf,
        rpu_buf_sz: (*s).rpu_buf_sz,
    };
}
#[no_mangle]
pub unsafe extern "C" fn ff_dovi_ctx_replace(
    mut s: *mut DOVIContext,
    mut s0: *const DOVIContext,
) {
    (*s).logctx = (*s0).logctx;
    (*s).cfg = (*s0).cfg;
    (*s).header = (*s0).header;
    (*s).mapping = (*s0).mapping;
    (*s).color = (*s0).color;
    av_refstruct_replace(
        &raw mut (*s).dm as *mut ::core::ffi::c_void,
        (*s0).dm as *const ::core::ffi::c_void,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i <= 15 as ::core::ffi::c_int {
        av_refstruct_replace(
            (&raw mut (*s).vdr as *mut *mut AVDOVIDataMapping).offset(i as isize)
                as *mut *mut AVDOVIDataMapping as *mut ::core::ffi::c_void,
            (*s0).vdr[i as usize] as *const ::core::ffi::c_void,
        );
        i += 1;
    }
    av_refstruct_replace(
        &raw mut (*s).ext_blocks as *mut ::core::ffi::c_void,
        (*s0).ext_blocks as *const ::core::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ff_dovi_guess_profile_hevc(
    mut hdr: *const AVDOVIRpuDataHeader,
) -> ::core::ffi::c_int {
    match (*hdr).vdr_rpu_profile as ::core::ffi::c_int {
        0 => {
            if (*hdr).bl_video_full_range_flag != 0 {
                return 5 as ::core::ffi::c_int;
            }
        }
        1 => {
            if (*hdr).el_spatial_resampling_filter_flag as ::core::ffi::c_int != 0
                && (*hdr).disable_residual_flag == 0
            {
                if (*hdr).vdr_bit_depth as ::core::ffi::c_int == 12 as ::core::ffi::c_int
                {
                    return 7 as ::core::ffi::c_int
                } else {
                    return 4 as ::core::ffi::c_int
                }
            } else {
                return 8 as ::core::ffi::c_int
            }
        }
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub static mut ff_dovi_color_default: AVDOVIColorMetadata = AVDOVIColorMetadata {
    dm_metadata_id: 0 as uint8_t,
    scene_refresh_flag: 0 as uint8_t,
    ycc_to_rgb_matrix: [
        AVRational {
            num: 9575 as ::core::ffi::c_int,
            den: 8192 as ::core::ffi::c_int,
        },
        AVRational {
            num: 0 as ::core::ffi::c_int,
            den: 8192 as ::core::ffi::c_int,
        },
        AVRational {
            num: 14742 as ::core::ffi::c_int,
            den: 8192 as ::core::ffi::c_int,
        },
        AVRational {
            num: 9575 as ::core::ffi::c_int,
            den: 8192 as ::core::ffi::c_int,
        },
        AVRational {
            num: 1754 as ::core::ffi::c_int,
            den: 8192 as ::core::ffi::c_int,
        },
        AVRational {
            num: 4383 as ::core::ffi::c_int,
            den: 8192 as ::core::ffi::c_int,
        },
        AVRational {
            num: 9575 as ::core::ffi::c_int,
            den: 8192 as ::core::ffi::c_int,
        },
        AVRational {
            num: 17372 as ::core::ffi::c_int,
            den: 8192 as ::core::ffi::c_int,
        },
        AVRational {
            num: 0 as ::core::ffi::c_int,
            den: 8192 as ::core::ffi::c_int,
        },
    ],
    ycc_to_rgb_offset: [
        AVRational {
            num: 1 as ::core::ffi::c_int,
            den: 4 as ::core::ffi::c_int,
        },
        AVRational {
            num: 2 as ::core::ffi::c_int,
            den: 1 as ::core::ffi::c_int,
        },
        AVRational {
            num: 2 as ::core::ffi::c_int,
            den: 1 as ::core::ffi::c_int,
        },
    ],
    rgb_to_lms_matrix: [
        AVRational {
            num: 5845 as ::core::ffi::c_int,
            den: 16384 as ::core::ffi::c_int,
        },
        AVRational {
            num: 9702 as ::core::ffi::c_int,
            den: 16384 as ::core::ffi::c_int,
        },
        AVRational {
            num: 837 as ::core::ffi::c_int,
            den: 16384 as ::core::ffi::c_int,
        },
        AVRational {
            num: 2568 as ::core::ffi::c_int,
            den: 16384 as ::core::ffi::c_int,
        },
        AVRational {
            num: 12256 as ::core::ffi::c_int,
            den: 16384 as ::core::ffi::c_int,
        },
        AVRational {
            num: 1561 as ::core::ffi::c_int,
            den: 16384 as ::core::ffi::c_int,
        },
        AVRational {
            num: 0 as ::core::ffi::c_int,
            den: 16384 as ::core::ffi::c_int,
        },
        AVRational {
            num: 679 as ::core::ffi::c_int,
            den: 16384 as ::core::ffi::c_int,
        },
        AVRational {
            num: 15705 as ::core::ffi::c_int,
            den: 16384 as ::core::ffi::c_int,
        },
    ],
    signal_eotf: 39322 as uint16_t,
    signal_eotf_param0: 15867 as uint16_t,
    signal_eotf_param1: 228 as uint16_t,
    signal_eotf_param2: 1383604 as ::core::ffi::c_int as uint32_t,
    signal_bit_depth: 14 as uint8_t,
    signal_color_space: 0 as uint8_t,
    signal_chroma_format: 0 as uint8_t,
    signal_full_range_flag: 1 as uint8_t,
    source_min_pq: 62 as uint16_t,
    source_max_pq: 3696 as uint16_t,
    source_diagonal: 42 as uint16_t,
};
