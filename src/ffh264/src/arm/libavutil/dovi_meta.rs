// Translated from FFmpeg 9.0.1 libavutil/dovi_meta.c
// (preprocessed for armv7-unknown-linux-musleabihf) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// Copyright (c) 2020 Jun Zhao<barryjzhao@tencent.com>
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
    fn av_mallocz(size: size_t) -> *mut ::core::ffi::c_void;
}
pub type size_t = ::core::ffi::c_uint;
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
pub struct AVDOVIMetadata {
    pub header_offset: size_t,
    pub mapping_offset: size_t,
    pub color_offset: size_t,
    pub ext_block_offset: size_t,
    pub ext_block_size: size_t,
    pub num_ext_blocks: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIMetadataInternal {
    pub metadata: AVDOVIMetadata,
    pub header: AVDOVIRpuDataHeader,
    pub mapping: AVDOVIDataMapping,
    pub color: AVDOVIColorMetadata,
    pub ext_blocks: [AVDOVIDmData; 32],
}
#[no_mangle]
pub unsafe extern "C" fn av_dovi_alloc(
    mut size: *mut size_t,
) -> *mut AVDOVIDecoderConfigurationRecord {
    let mut dovi: *mut AVDOVIDecoderConfigurationRecord = av_mallocz(
        ::core::mem::size_of::<AVDOVIDecoderConfigurationRecord>() as size_t,
    ) as *mut AVDOVIDecoderConfigurationRecord;
    if !size.is_null() {
        *size = (if !dovi.is_null() {
            ::core::mem::size_of::<AVDOVIDecoderConfigurationRecord>() as usize
        } else {
            0 as usize
        }) as size_t;
    }
    return dovi;
}
#[no_mangle]
pub unsafe extern "C" fn av_dovi_metadata_alloc(
    mut size: *mut size_t,
) -> *mut AVDOVIMetadata {
    let mut dovi: *mut AVDOVIMetadataInternal = av_mallocz(
        ::core::mem::size_of::<AVDOVIMetadataInternal>() as size_t,
    ) as *mut AVDOVIMetadataInternal;
    if !size.is_null() {
        *size = (if !dovi.is_null() {
            ::core::mem::size_of::<AVDOVIMetadataInternal>() as usize
        } else {
            0 as usize
        }) as size_t;
    }
    if dovi.is_null() {
        return ::core::ptr::null_mut::<AVDOVIMetadata>();
    }
    (*dovi).metadata = AVDOVIMetadata {
        header_offset: 24 as size_t,
        mapping_offset: 48 as size_t,
        color_offset: 5192 as size_t,
        ext_block_offset: 5388 as size_t,
        ext_block_size: ::core::mem::size_of::<AVDOVIDmData>() as size_t,
        num_ext_blocks: 0,
    } as AVDOVIMetadata;
    return &raw mut (*dovi).metadata;
}
#[no_mangle]
pub unsafe extern "C" fn av_dovi_find_level(
    mut data: *const AVDOVIMetadata,
    mut level: uint8_t,
) -> *mut AVDOVIDmData {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*data).num_ext_blocks {
        let mut ext: *mut AVDOVIDmData = av_dovi_get_ext(data, i);
        if (*ext).level as ::core::ffi::c_int == level as ::core::ffi::c_int {
            return ext;
        }
        i += 1;
    }
    return ::core::ptr::null_mut::<AVDOVIDmData>();
}
#[inline(always)]
unsafe extern "C" fn av_dovi_get_ext(
    mut data: *const AVDOVIMetadata,
    mut index: ::core::ffi::c_int,
) -> *mut AVDOVIDmData {
    return (data as *mut uint8_t)
        .offset((*data).ext_block_offset as isize)
        .offset((*data).ext_block_size.wrapping_mul(index as size_t) as isize)
        as *mut AVDOVIDmData;
}
