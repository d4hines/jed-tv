// Translated from FFmpeg 9.0.1 libavutil/hdr_dynamic_metadata.c
// (preprocessed for aarch64-unknown-linux-musl) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
//
// Copyright (c) 2018 Mohammad Izadi <moh.izadi at gmail.com>
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
    fn abort() -> !;
    fn av_malloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn av_mallocz(size: size_t) -> *mut ::core::ffi::c_void;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn av_free(ptr: *mut ::core::ffi::c_void);
    fn av_frame_new_side_data(
        frame: *mut AVFrame,
        type_0: AVFrameSideDataType,
        size: size_t,
    ) -> *mut AVFrameSideData;
}
pub type size_t = ::core::ffi::c_ulong;
pub type int32_t = ::core::ffi::c_int;
pub type int64_t = ::core::ffi::c_long;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint16_t = ::core::ffi::c_ushort;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint64_t = ::core::ffi::c_ulong;
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
pub struct AVRational {
    pub num: ::core::ffi::c_int,
    pub den: ::core::ffi::c_int,
}
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
pub type AVColorRange = ::core::ffi::c_uint;
pub const AVCOL_RANGE_NB: AVColorRange = 3;
pub const AVCOL_RANGE_JPEG: AVColorRange = 2;
pub const AVCOL_RANGE_MPEG: AVColorRange = 1;
pub const AVCOL_RANGE_UNSPECIFIED: AVColorRange = 0;
pub type AVChromaLocation = ::core::ffi::c_uint;
pub const AVCHROMA_LOC_NB: AVChromaLocation = 7;
pub const AVCHROMA_LOC_BOTTOM: AVChromaLocation = 6;
pub const AVCHROMA_LOC_BOTTOMLEFT: AVChromaLocation = 5;
pub const AVCHROMA_LOC_TOP: AVChromaLocation = 4;
pub const AVCHROMA_LOC_TOPLEFT: AVChromaLocation = 3;
pub const AVCHROMA_LOC_CENTER: AVChromaLocation = 2;
pub const AVCHROMA_LOC_LEFT: AVChromaLocation = 1;
pub const AVCHROMA_LOC_UNSPECIFIED: AVChromaLocation = 0;
pub type AVAlphaMode = ::core::ffi::c_uint;
pub const AVALPHA_MODE_NB: AVAlphaMode = 3;
pub const AVALPHA_MODE_STRAIGHT: AVAlphaMode = 2;
pub const AVALPHA_MODE_PREMULTIPLIED: AVAlphaMode = 1;
pub const AVALPHA_MODE_UNSPECIFIED: AVAlphaMode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBufferRef {
    pub buffer: *mut AVBuffer,
    pub data: *mut uint8_t,
    pub size: size_t,
}
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
pub type AVFrameSideDataType = ::core::ffi::c_uint;
pub const AV_FRAME_DATA_RAW_COLOR_PARAMS: AVFrameSideDataType = 36;
pub const AV_FRAME_DATA_IAMF_RECON_GAIN_INFO_PARAM: AVFrameSideDataType = 35;
pub const AV_FRAME_DATA_IAMF_DEMIXING_INFO_PARAM: AVFrameSideDataType = 34;
pub const AV_FRAME_DATA_IAMF_MIX_GAIN_PARAM: AVFrameSideDataType = 33;
pub const AV_FRAME_DATA_DYNAMIC_HDR_SMPTE_2094_APP5: AVFrameSideDataType = 32;
pub const AV_FRAME_DATA_EXIF: AVFrameSideDataType = 31;
pub const AV_FRAME_DATA_3D_REFERENCE_DISPLAYS: AVFrameSideDataType = 30;
pub const AV_FRAME_DATA_VIEW_ID: AVFrameSideDataType = 29;
pub const AV_FRAME_DATA_LCEVC: AVFrameSideDataType = 28;
pub const AV_FRAME_DATA_VIDEO_HINT: AVFrameSideDataType = 27;
pub const AV_FRAME_DATA_AMBIENT_VIEWING_ENVIRONMENT: AVFrameSideDataType = 26;
pub const AV_FRAME_DATA_DYNAMIC_HDR_VIVID: AVFrameSideDataType = 25;
pub const AV_FRAME_DATA_DOVI_METADATA: AVFrameSideDataType = 24;
pub const AV_FRAME_DATA_DOVI_RPU_BUFFER: AVFrameSideDataType = 23;
pub const AV_FRAME_DATA_DETECTION_BBOXES: AVFrameSideDataType = 22;
pub const AV_FRAME_DATA_FILM_GRAIN_PARAMS: AVFrameSideDataType = 21;
pub const AV_FRAME_DATA_SEI_UNREGISTERED: AVFrameSideDataType = 20;
pub const AV_FRAME_DATA_VIDEO_ENC_PARAMS: AVFrameSideDataType = 19;
pub const AV_FRAME_DATA_REGIONS_OF_INTEREST: AVFrameSideDataType = 18;
pub const AV_FRAME_DATA_DYNAMIC_HDR_PLUS: AVFrameSideDataType = 17;
pub const AV_FRAME_DATA_S12M_TIMECODE: AVFrameSideDataType = 16;
pub const AV_FRAME_DATA_ICC_PROFILE: AVFrameSideDataType = 15;
pub const AV_FRAME_DATA_CONTENT_LIGHT_LEVEL: AVFrameSideDataType = 14;
pub const AV_FRAME_DATA_SPHERICAL: AVFrameSideDataType = 13;
pub const AV_FRAME_DATA_GOP_TIMECODE: AVFrameSideDataType = 12;
pub const AV_FRAME_DATA_MASTERING_DISPLAY_METADATA: AVFrameSideDataType = 11;
pub const AV_FRAME_DATA_AUDIO_SERVICE_TYPE: AVFrameSideDataType = 10;
pub const AV_FRAME_DATA_SKIP_SAMPLES: AVFrameSideDataType = 9;
pub const AV_FRAME_DATA_MOTION_VECTORS: AVFrameSideDataType = 8;
pub const AV_FRAME_DATA_AFD: AVFrameSideDataType = 7;
pub const AV_FRAME_DATA_DISPLAYMATRIX: AVFrameSideDataType = 6;
pub const AV_FRAME_DATA_REPLAYGAIN: AVFrameSideDataType = 5;
pub const AV_FRAME_DATA_DOWNMIX_INFO: AVFrameSideDataType = 4;
pub const AV_FRAME_DATA_MATRIXENCODING: AVFrameSideDataType = 3;
pub const AV_FRAME_DATA_STEREO3D: AVFrameSideDataType = 2;
pub const AV_FRAME_DATA_A53_CC: AVFrameSideDataType = 1;
pub const AV_FRAME_DATA_PANSCAN: AVFrameSideDataType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFrameSideData {
    pub type_0: AVFrameSideDataType,
    pub data: *mut uint8_t,
    pub size: size_t,
    pub metadata: *mut AVDictionary,
    pub buf: *mut AVBufferRef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFrame {
    pub data: [*mut uint8_t; 8],
    pub linesize: [::core::ffi::c_int; 8],
    pub extended_data: *mut *mut uint8_t,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub nb_samples: ::core::ffi::c_int,
    pub format: ::core::ffi::c_int,
    pub pict_type: AVPictureType,
    pub sample_aspect_ratio: AVRational,
    pub pts: int64_t,
    pub pkt_dts: int64_t,
    pub time_base: AVRational,
    pub quality: ::core::ffi::c_int,
    pub opaque: *mut ::core::ffi::c_void,
    pub repeat_pict: ::core::ffi::c_int,
    pub sample_rate: ::core::ffi::c_int,
    pub buf: [*mut AVBufferRef; 8],
    pub extended_buf: *mut *mut AVBufferRef,
    pub nb_extended_buf: ::core::ffi::c_int,
    pub side_data: *mut *mut AVFrameSideData,
    pub nb_side_data: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub color_range: AVColorRange,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub colorspace: AVColorSpace,
    pub chroma_location: AVChromaLocation,
    pub best_effort_timestamp: int64_t,
    pub metadata: *mut AVDictionary,
    pub decode_error_flags: ::core::ffi::c_int,
    pub hw_frames_ctx: *mut AVBufferRef,
    pub opaque_ref: *mut AVBufferRef,
    pub crop_top: size_t,
    pub crop_bottom: size_t,
    pub crop_left: size_t,
    pub crop_right: size_t,
    pub private_ref: *mut ::core::ffi::c_void,
    pub ch_layout: AVChannelLayout,
    pub duration: int64_t,
    pub alpha_mode: AVAlphaMode,
}
pub type AVHDRPlusOverlapProcessOption = ::core::ffi::c_uint;
pub const AV_HDR_PLUS_OVERLAP_PROCESS_LAYERING: AVHDRPlusOverlapProcessOption = 1;
pub const AV_HDR_PLUS_OVERLAP_PROCESS_WEIGHTED_AVERAGING: AVHDRPlusOverlapProcessOption = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVHDRPlusPercentile {
    pub percentage: uint8_t,
    pub percentile: AVRational,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVHDRPlusColorTransformParams {
    pub window_upper_left_corner_x: AVRational,
    pub window_upper_left_corner_y: AVRational,
    pub window_lower_right_corner_x: AVRational,
    pub window_lower_right_corner_y: AVRational,
    pub center_of_ellipse_x: uint16_t,
    pub center_of_ellipse_y: uint16_t,
    pub rotation_angle: uint8_t,
    pub semimajor_axis_internal_ellipse: uint16_t,
    pub semimajor_axis_external_ellipse: uint16_t,
    pub semiminor_axis_external_ellipse: uint16_t,
    pub overlap_process_option: AVHDRPlusOverlapProcessOption,
    pub maxscl: [AVRational; 3],
    pub average_maxrgb: AVRational,
    pub num_distribution_maxrgb_percentiles: uint8_t,
    pub distribution_maxrgb: [AVHDRPlusPercentile; 15],
    pub fraction_bright_pixels: AVRational,
    pub tone_mapping_flag: uint8_t,
    pub knee_point_x: AVRational,
    pub knee_point_y: AVRational,
    pub num_bezier_curve_anchors: uint8_t,
    pub bezier_curve_anchors: [AVRational; 15],
    pub color_saturation_mapping_flag: uint8_t,
    pub color_saturation_weight: AVRational,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDynamicHDRPlus {
    pub itu_t_t35_country_code: uint8_t,
    pub application_version: uint8_t,
    pub num_windows: uint8_t,
    pub params: [AVHDRPlusColorTransformParams; 3],
    pub targeted_system_display_maximum_luminance: AVRational,
    pub targeted_system_display_actual_peak_luminance_flag: uint8_t,
    pub num_rows_targeted_system_display_actual_peak_luminance: uint8_t,
    pub num_cols_targeted_system_display_actual_peak_luminance: uint8_t,
    pub targeted_system_display_actual_peak_luminance: [[AVRational; 25]; 25],
    pub mastering_display_actual_peak_luminance_flag: uint8_t,
    pub num_rows_mastering_display_actual_peak_luminance: uint8_t,
    pub num_cols_mastering_display_actual_peak_luminance: uint8_t,
    pub mastering_display_actual_peak_luminance: [[AVRational; 25]; 25],
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
#[repr(C, packed)]
pub union unaligned_32 {
    pub l: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union unaligned_64 {
    pub l: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PutBitContext {
    pub bit_buf: BitBuf,
    pub bit_left: ::core::ffi::c_int,
    pub buf: *mut uint8_t,
    pub buf_ptr: *mut uint8_t,
    pub buf_end: *mut uint8_t,
}
pub type BitBuf = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDynamicHDRSmpte2094App5 {
    pub application_version: uint8_t,
    pub minimum_application_version: uint8_t,
    pub has_custom_hdr_reference_white_flag: uint8_t,
    pub has_adaptive_tone_map_flag: uint8_t,
    pub hdr_reference_white: uint16_t,
    pub baseline_hdr_headroom: uint16_t,
    pub use_reference_white_tone_mapping_flag: uint8_t,
    pub num_alternate_images: uint8_t,
    pub gain_application_space_chromaticities_flag: uint8_t,
    pub has_common_component_mix_params_flag: uint8_t,
    pub has_common_curve_params_flag: uint8_t,
    pub gain_application_space_chromaticities: [uint16_t; 8],
    pub alternate_hdr_headrooms: [uint16_t; 4],
    pub component_mixing_type: [uint8_t; 4],
    pub has_component_mixing_coefficient_flag: [[uint8_t; 6]; 4],
    pub component_mixing_coefficient: [[uint16_t; 6]; 4],
    pub gain_curve_num_control_points_minus_1: [uint8_t; 4],
    pub gain_curve_use_pchip_slope_flag: [uint8_t; 4],
    pub gain_curve_control_points_x: [[uint16_t; 32]; 4],
    pub gain_curve_control_points_y: [[uint16_t; 32]; 4],
    pub gain_curve_control_points_theta: [[uint16_t; 32]; 4],
}
static mut luminance_den: int64_t = 1 as int64_t;
static mut peak_luminance_den: int32_t = 15 as int32_t;
static mut rgb_den: int64_t = 100000 as int64_t;
static mut fraction_pixel_den: int32_t = 1000 as int32_t;
static mut knee_point_den: int32_t = 4095 as int32_t;
static mut bezier_anchor_den: int32_t = 1023 as int32_t;
static mut saturation_weight_den: int32_t = 8 as int32_t;
#[no_mangle]
pub unsafe extern "C" fn av_dynamic_hdr_plus_alloc(
    mut size: *mut size_t,
) -> *mut AVDynamicHDRPlus {
    let mut hdr_plus: *mut AVDynamicHDRPlus = av_mallocz(
        ::core::mem::size_of::<AVDynamicHDRPlus>() as size_t,
    ) as *mut AVDynamicHDRPlus;
    if !size.is_null() {
        *size = (if !hdr_plus.is_null() {
            ::core::mem::size_of::<AVDynamicHDRPlus>() as usize
        } else {
            0 as usize
        }) as size_t;
    }
    return hdr_plus;
}
#[no_mangle]
pub unsafe extern "C" fn av_dynamic_hdr_plus_create_side_data(
    mut frame: *mut AVFrame,
) -> *mut AVDynamicHDRPlus {
    let mut side_data: *mut AVFrameSideData = av_frame_new_side_data(
        frame,
        AV_FRAME_DATA_DYNAMIC_HDR_PLUS,
        ::core::mem::size_of::<AVDynamicHDRPlus>() as size_t,
    );
    if side_data.is_null() {
        return ::core::ptr::null_mut::<AVDynamicHDRPlus>();
    }
    memset(
        (*side_data).data as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<AVDynamicHDRPlus>() as size_t,
    );
    return (*side_data).data as *mut AVDynamicHDRPlus;
}
#[no_mangle]
pub unsafe extern "C" fn av_dynamic_hdr_plus_from_t35(
    mut s: *mut AVDynamicHDRPlus,
    mut data: *const uint8_t,
    mut size: size_t,
) -> ::core::ffi::c_int {
    let mut padded_buf: [uint8_t; 971] = [0; 971];
    let mut gbc: GetBitContext = GetBitContext {
        buffer: ::core::ptr::null::<uint8_t>(),
        index: 0,
        size_in_bits: 0,
        size_in_bits_plus8: 0,
    };
    let mut gb: *mut GetBitContext = &raw mut gbc;
    let mut ret: ::core::ffi::c_int = 0;
    if s.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    if size > 907 as size_t {
        return -(22 as ::core::ffi::c_int);
    }
    memcpy(
        &raw mut padded_buf as *mut uint8_t as *mut ::core::ffi::c_void,
        data as *const ::core::ffi::c_void,
        size,
    );
    memset(
        (&raw mut padded_buf as *mut uint8_t).offset(size as isize)
            as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        64 as size_t,
    );
    ret = init_get_bits8(
        gb,
        &raw mut padded_buf as *mut uint8_t,
        size as ::core::ffi::c_int,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    if get_bits_left(gb) < 10 as ::core::ffi::c_int {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    (*s).application_version = get_bits(gb, 8 as ::core::ffi::c_int) as uint8_t;
    (*s).num_windows = get_bits(gb, 2 as ::core::ffi::c_int) as uint8_t;
    if ((*s).num_windows as ::core::ffi::c_int) < 1 as ::core::ffi::c_int
        || (*s).num_windows as ::core::ffi::c_int > 3 as ::core::ffi::c_int
    {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if get_bits_left(gb)
        < (19 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            * ((*s).num_windows as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
    {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    let mut w: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while w < (*s).num_windows as ::core::ffi::c_int {
        let mut params: *mut AVHDRPlusColorTransformParams = (&raw mut (*s).params
            as *mut AVHDRPlusColorTransformParams)
            .offset(w as isize) as *mut AVHDRPlusColorTransformParams;
        (*params).window_upper_left_corner_x = AVRational {
            num: get_bits(gb, 16 as ::core::ffi::c_int) as ::core::ffi::c_int,
            den: 1 as ::core::ffi::c_int,
        };
        (*params).window_upper_left_corner_y = AVRational {
            num: get_bits(gb, 16 as ::core::ffi::c_int) as ::core::ffi::c_int,
            den: 1 as ::core::ffi::c_int,
        };
        (*params).window_lower_right_corner_x = AVRational {
            num: get_bits(gb, 16 as ::core::ffi::c_int) as ::core::ffi::c_int,
            den: 1 as ::core::ffi::c_int,
        };
        (*params).window_lower_right_corner_y = AVRational {
            num: get_bits(gb, 16 as ::core::ffi::c_int) as ::core::ffi::c_int,
            den: 1 as ::core::ffi::c_int,
        };
        (*params).center_of_ellipse_x = get_bits(gb, 16 as ::core::ffi::c_int)
            as uint16_t;
        (*params).center_of_ellipse_y = get_bits(gb, 16 as ::core::ffi::c_int)
            as uint16_t;
        (*params).rotation_angle = get_bits(gb, 8 as ::core::ffi::c_int) as uint8_t;
        (*params).semimajor_axis_internal_ellipse = get_bits(
            gb,
            16 as ::core::ffi::c_int,
        ) as uint16_t;
        (*params).semimajor_axis_external_ellipse = get_bits(
            gb,
            16 as ::core::ffi::c_int,
        ) as uint16_t;
        (*params).semiminor_axis_external_ellipse = get_bits(
            gb,
            16 as ::core::ffi::c_int,
        ) as uint16_t;
        (*params).overlap_process_option = get_bits1(gb)
            as AVHDRPlusOverlapProcessOption;
        w += 1;
    }
    if get_bits_left(gb) < 28 as ::core::ffi::c_int {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    (*s).targeted_system_display_maximum_luminance = AVRational {
        num: get_bits_long(gb, 27 as ::core::ffi::c_int) as ::core::ffi::c_int,
        den: luminance_den as ::core::ffi::c_int,
    };
    (*s).targeted_system_display_actual_peak_luminance_flag = get_bits1(gb) as uint8_t;
    if (*s).targeted_system_display_actual_peak_luminance_flag != 0 {
        let mut rows: ::core::ffi::c_int = 0;
        let mut cols: ::core::ffi::c_int = 0;
        if get_bits_left(gb) < 10 as ::core::ffi::c_int {
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        rows = get_bits(gb, 5 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cols = get_bits(gb, 5 as ::core::ffi::c_int) as ::core::ffi::c_int;
        if rows < 2 as ::core::ffi::c_int || rows > 25 as ::core::ffi::c_int
            || (cols < 2 as ::core::ffi::c_int || cols > 25 as ::core::ffi::c_int)
        {
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        (*s).num_rows_targeted_system_display_actual_peak_luminance = rows as uint8_t;
        (*s).num_cols_targeted_system_display_actual_peak_luminance = cols as uint8_t;
        if get_bits_left(gb) < rows * cols * 4 as ::core::ffi::c_int {
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < rows {
            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j < cols {
                (*s)
                    .targeted_system_display_actual_peak_luminance[i
                    as usize][j as usize] = AVRational {
                    num: get_bits(gb, 4 as ::core::ffi::c_int) as ::core::ffi::c_int,
                    den: peak_luminance_den as ::core::ffi::c_int,
                };
                j += 1;
            }
            i += 1;
        }
    }
    let mut w_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while w_0 < (*s).num_windows as ::core::ffi::c_int {
        let mut params_0: *mut AVHDRPlusColorTransformParams = (&raw mut (*s).params
            as *mut AVHDRPlusColorTransformParams)
            .offset(w_0 as isize) as *mut AVHDRPlusColorTransformParams;
        if get_bits_left(gb)
            < 3 as ::core::ffi::c_int * 17 as ::core::ffi::c_int
                + 17 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
        {
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < 3 as ::core::ffi::c_int {
            (*params_0).maxscl[i_0 as usize] = AVRational {
                num: get_bits(gb, 17 as ::core::ffi::c_int) as ::core::ffi::c_int,
                den: rgb_den as ::core::ffi::c_int,
            };
            i_0 += 1;
        }
        (*params_0).average_maxrgb = AVRational {
            num: get_bits(gb, 17 as ::core::ffi::c_int) as ::core::ffi::c_int,
            den: rgb_den as ::core::ffi::c_int,
        };
        (*params_0).num_distribution_maxrgb_percentiles = get_bits(
            gb,
            4 as ::core::ffi::c_int,
        ) as uint8_t;
        if get_bits_left(gb)
            < (*params_0).num_distribution_maxrgb_percentiles as ::core::ffi::c_int
                * 24 as ::core::ffi::c_int
        {
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1 < (*params_0).num_distribution_maxrgb_percentiles as ::core::ffi::c_int
        {
            (*params_0).distribution_maxrgb[i_1 as usize].percentage = get_bits(
                gb,
                7 as ::core::ffi::c_int,
            ) as uint8_t;
            (*params_0).distribution_maxrgb[i_1 as usize].percentile = AVRational {
                num: get_bits(gb, 17 as ::core::ffi::c_int) as ::core::ffi::c_int,
                den: rgb_den as ::core::ffi::c_int,
            };
            i_1 += 1;
        }
        if get_bits_left(gb) < 10 as ::core::ffi::c_int {
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        (*params_0).fraction_bright_pixels = AVRational {
            num: get_bits(gb, 10 as ::core::ffi::c_int) as ::core::ffi::c_int,
            den: fraction_pixel_den as ::core::ffi::c_int,
        };
        w_0 += 1;
    }
    if get_bits_left(gb) < 1 as ::core::ffi::c_int {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    (*s).mastering_display_actual_peak_luminance_flag = get_bits1(gb) as uint8_t;
    if (*s).mastering_display_actual_peak_luminance_flag != 0 {
        let mut rows_0: ::core::ffi::c_int = 0;
        let mut cols_0: ::core::ffi::c_int = 0;
        if get_bits_left(gb) < 10 as ::core::ffi::c_int {
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        rows_0 = get_bits(gb, 5 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cols_0 = get_bits(gb, 5 as ::core::ffi::c_int) as ::core::ffi::c_int;
        if rows_0 < 2 as ::core::ffi::c_int || rows_0 > 25 as ::core::ffi::c_int
            || (cols_0 < 2 as ::core::ffi::c_int || cols_0 > 25 as ::core::ffi::c_int)
        {
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        (*s).num_rows_mastering_display_actual_peak_luminance = rows_0 as uint8_t;
        (*s).num_cols_mastering_display_actual_peak_luminance = cols_0 as uint8_t;
        if get_bits_left(gb) < rows_0 * cols_0 * 4 as ::core::ffi::c_int {
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_2 < rows_0 {
            let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j_0 < cols_0 {
                (*s)
                    .mastering_display_actual_peak_luminance[i_2
                    as usize][j_0 as usize] = AVRational {
                    num: get_bits(gb, 4 as ::core::ffi::c_int) as ::core::ffi::c_int,
                    den: peak_luminance_den as ::core::ffi::c_int,
                };
                j_0 += 1;
            }
            i_2 += 1;
        }
    }
    let mut w_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while w_1 < (*s).num_windows as ::core::ffi::c_int {
        let mut params_1: *mut AVHDRPlusColorTransformParams = (&raw mut (*s).params
            as *mut AVHDRPlusColorTransformParams)
            .offset(w_1 as isize) as *mut AVHDRPlusColorTransformParams;
        if get_bits_left(gb) < 1 as ::core::ffi::c_int {
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        (*params_1).tone_mapping_flag = get_bits1(gb) as uint8_t;
        if (*params_1).tone_mapping_flag != 0 {
            if get_bits_left(gb) < 28 as ::core::ffi::c_int {
                return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                    | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
            }
            (*params_1).knee_point_x = AVRational {
                num: get_bits(gb, 12 as ::core::ffi::c_int) as ::core::ffi::c_int,
                den: knee_point_den as ::core::ffi::c_int,
            };
            (*params_1).knee_point_y = AVRational {
                num: get_bits(gb, 12 as ::core::ffi::c_int) as ::core::ffi::c_int,
                den: knee_point_den as ::core::ffi::c_int,
            };
            (*params_1).num_bezier_curve_anchors = get_bits(gb, 4 as ::core::ffi::c_int)
                as uint8_t;
            if get_bits_left(gb)
                < (*params_1).num_bezier_curve_anchors as ::core::ffi::c_int
                    * 10 as ::core::ffi::c_int
            {
                return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                    | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
            }
            let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_3 < (*params_1).num_bezier_curve_anchors as ::core::ffi::c_int {
                (*params_1).bezier_curve_anchors[i_3 as usize] = AVRational {
                    num: get_bits(gb, 10 as ::core::ffi::c_int) as ::core::ffi::c_int,
                    den: bezier_anchor_den as ::core::ffi::c_int,
                };
                i_3 += 1;
            }
        }
        if get_bits_left(gb) < 1 as ::core::ffi::c_int {
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        (*params_1).color_saturation_mapping_flag = get_bits1(gb) as uint8_t;
        if (*params_1).color_saturation_mapping_flag != 0 {
            if get_bits_left(gb) < 6 as ::core::ffi::c_int {
                return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                    | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
            }
            (*params_1).color_saturation_weight = AVRational {
                num: get_bits(gb, 6 as ::core::ffi::c_int) as ::core::ffi::c_int,
                den: saturation_weight_den as ::core::ffi::c_int,
            };
        }
        w_1 += 1;
    }
    return 0 as ::core::ffi::c_int;
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
unsafe extern "C" fn init_put_bits(
    mut s: *mut PutBitContext,
    mut buffer: *mut uint8_t,
    mut buffer_size: ::core::ffi::c_int,
) {
    if buffer_size < 0 as ::core::ffi::c_int {
        buffer_size = 0 as ::core::ffi::c_int;
        buffer = ::core::ptr::null_mut::<uint8_t>();
    }
    (*s).buf = buffer;
    (*s).buf_end = (*s).buf.offset(buffer_size as isize);
    (*s).buf_ptr = (*s).buf;
    (*s).bit_left = 32 as ::core::ffi::c_int;
    (*s).bit_buf = 0 as BitBuf;
}
#[inline]
unsafe extern "C" fn av_bswap64(mut x: uint64_t) -> uint64_t {
    return (av_bswap32(x as uint32_t) as uint64_t) << 32 as ::core::ffi::c_int
        | av_bswap32((x >> 32 as ::core::ffi::c_int) as uint32_t) as uint64_t;
}
#[inline]
unsafe extern "C" fn flush_put_bits(mut s: *mut PutBitContext) {
    if (*s).bit_left < 32 as ::core::ffi::c_int {
        (*s).bit_buf <<= (*s).bit_left;
    }
    while (*s).bit_left < 32 as ::core::ffi::c_int {
        if !((*s).buf_ptr < (*s).buf_end) {
            av_log(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                0 as ::core::ffi::c_int,
                b"Assertion %s failed at %s:%d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"s->buf_ptr < s->buf_end\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/libavcodec/put_bits.h\0" as *const u8
                    as *const ::core::ffi::c_char,
                160 as ::core::ffi::c_int,
            );
            abort();
        }
        let fresh0 = (*s).buf_ptr;
        (*s).buf_ptr = (*s).buf_ptr.offset(1);
        *fresh0 = ((*s).bit_buf >> 32 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
            as uint8_t;
        (*s).bit_buf <<= 8 as ::core::ffi::c_int;
        (*s).bit_left += 8 as ::core::ffi::c_int;
    }
    (*s).bit_left = 32 as ::core::ffi::c_int;
    (*s).bit_buf = 0 as BitBuf;
}
#[inline]
unsafe extern "C" fn put_bits_no_assert(
    mut s: *mut PutBitContext,
    mut n: ::core::ffi::c_int,
    mut value: BitBuf,
) {
    let mut bit_buf: BitBuf = 0;
    let mut bit_left: ::core::ffi::c_int = 0;
    bit_buf = (*s).bit_buf;
    bit_left = (*s).bit_left;
    if n < bit_left {
        bit_buf = bit_buf << n | value;
        bit_left -= n;
    } else {
        bit_buf <<= bit_left;
        bit_buf |= value >> n - bit_left;
        if (*s).buf_end.offset_from((*s).buf_ptr) as ::core::ffi::c_long as usize
            >= ::core::mem::size_of::<BitBuf>() as usize
        {
            (*((*s).buf_ptr as *mut unaligned_32)).l = av_bswap32(bit_buf as uint32_t);
            (*s).buf_ptr = (*s)
                .buf_ptr
                .offset(::core::mem::size_of::<BitBuf>() as usize as isize);
        } else {
            av_log(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                16 as ::core::ffi::c_int,
                b"Internal error, put_bits buffer too small\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        bit_left += 32 as ::core::ffi::c_int - n;
        bit_buf = value;
    }
    (*s).bit_buf = bit_buf;
    (*s).bit_left = bit_left;
}
#[no_mangle]
pub unsafe extern "C" fn av_dynamic_hdr_plus_to_t35(
    mut s: *const AVDynamicHDRPlus,
    mut data: *mut *mut uint8_t,
    mut size: *mut size_t,
) -> ::core::ffi::c_int {
    let mut buf: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut size_bits: size_t = 0;
    let mut size_bytes: size_t = 0;
    let mut pbc: PutBitContext = PutBitContext {
        bit_buf: 0,
        bit_left: 0,
        buf: ::core::ptr::null_mut::<uint8_t>(),
        buf_ptr: ::core::ptr::null_mut::<uint8_t>(),
        buf_end: ::core::ptr::null_mut::<uint8_t>(),
    };
    let mut pb: *mut PutBitContext = &raw mut pbc;
    if s.is_null() {
        return -(22 as ::core::ffi::c_int);
    }
    if (data.is_null() || !(*data).is_null()) && size.is_null() {
        return -(22 as ::core::ffi::c_int);
    }
    size_bits = 8 as size_t;
    size_bits = size_bits.wrapping_add(2 as size_t);
    let mut w: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while w < (*s).num_windows as ::core::ffi::c_int {
        size_bits = size_bits.wrapping_add(153 as size_t);
        w += 1;
    }
    size_bits = size_bits.wrapping_add(27 as size_t);
    size_bits = size_bits.wrapping_add(1 as size_t);
    if (*s).targeted_system_display_actual_peak_luminance_flag != 0 {
        size_bits = size_bits
            .wrapping_add(
                (10 as ::core::ffi::c_int
                    + (*s).num_rows_targeted_system_display_actual_peak_luminance
                        as ::core::ffi::c_int
                        * (*s).num_cols_targeted_system_display_actual_peak_luminance
                            as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as size_t,
            );
    }
    let mut w_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while w_0 < (*s).num_windows as ::core::ffi::c_int {
        size_bits = size_bits
            .wrapping_add(
                (72 as ::core::ffi::c_int
                    + (*s).params[w_0 as usize].num_distribution_maxrgb_percentiles
                        as ::core::ffi::c_int * 24 as ::core::ffi::c_int
                    + 10 as ::core::ffi::c_int) as size_t,
            );
        w_0 += 1;
    }
    size_bits = size_bits.wrapping_add(1 as size_t);
    if (*s).mastering_display_actual_peak_luminance_flag != 0 {
        size_bits = size_bits
            .wrapping_add(
                (10 as ::core::ffi::c_int
                    + (*s).num_rows_mastering_display_actual_peak_luminance
                        as ::core::ffi::c_int
                        * (*s).num_cols_mastering_display_actual_peak_luminance
                            as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as size_t,
            );
    }
    let mut w_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while w_1 < (*s).num_windows as ::core::ffi::c_int {
        size_bits = size_bits.wrapping_add(1 as size_t);
        if (*s).params[w_1 as usize].tone_mapping_flag != 0 {
            size_bits = size_bits
                .wrapping_add(
                    (28 as ::core::ffi::c_int
                        + (*s).params[w_1 as usize].num_bezier_curve_anchors
                            as ::core::ffi::c_int * 10 as ::core::ffi::c_int) as size_t,
                );
        }
        size_bits = size_bits.wrapping_add(1 as size_t);
        if (*s).params[w_1 as usize].color_saturation_mapping_flag != 0 {
            size_bits = size_bits.wrapping_add(6 as size_t);
        }
        w_1 += 1;
    }
    size_bytes = size_bits.wrapping_add(7 as size_t).wrapping_div(8 as size_t);
    if !(size_bytes <= 907 as size_t) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"size_bytes <= 907\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavutil/hdr_dynamic_metadata.c\0" as *const u8
                as *const ::core::ffi::c_char,
            298 as ::core::ffi::c_int,
        );
        abort();
    }
    if data.is_null() {
        *size = size_bytes;
        return 0 as ::core::ffi::c_int;
    } else if !(*data).is_null() {
        if *size < size_bytes {
            return -((('B' as i32 | ('U' as i32) << 8 as ::core::ffi::c_int
                | ('F' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('S' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        buf = *data;
    } else {
        buf = av_malloc(size_bytes) as *mut uint8_t;
        if buf.is_null() {
            return -(12 as ::core::ffi::c_int);
        }
    }
    init_put_bits(pb, buf, size_bytes as ::core::ffi::c_int);
    put_bits(pb, 8 as ::core::ffi::c_int, 1 as BitBuf);
    put_bits(pb, 2 as ::core::ffi::c_int, (*s).num_windows as BitBuf);
    let mut w_2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while w_2 < (*s).num_windows as ::core::ffi::c_int {
        put_bits(
            pb,
            16 as ::core::ffi::c_int,
            ((*s).params[w_2 as usize].window_upper_left_corner_x.num
                / (*s).params[w_2 as usize].window_upper_left_corner_x.den) as BitBuf,
        );
        put_bits(
            pb,
            16 as ::core::ffi::c_int,
            ((*s).params[w_2 as usize].window_upper_left_corner_y.num
                / (*s).params[w_2 as usize].window_upper_left_corner_y.den) as BitBuf,
        );
        put_bits(
            pb,
            16 as ::core::ffi::c_int,
            ((*s).params[w_2 as usize].window_lower_right_corner_x.num
                / (*s).params[w_2 as usize].window_lower_right_corner_x.den) as BitBuf,
        );
        put_bits(
            pb,
            16 as ::core::ffi::c_int,
            ((*s).params[w_2 as usize].window_lower_right_corner_y.num
                / (*s).params[w_2 as usize].window_lower_right_corner_y.den) as BitBuf,
        );
        put_bits(
            pb,
            16 as ::core::ffi::c_int,
            (*s).params[w_2 as usize].center_of_ellipse_x as BitBuf,
        );
        put_bits(
            pb,
            16 as ::core::ffi::c_int,
            (*s).params[w_2 as usize].center_of_ellipse_y as BitBuf,
        );
        put_bits(
            pb,
            8 as ::core::ffi::c_int,
            (*s).params[w_2 as usize].rotation_angle as BitBuf,
        );
        put_bits(
            pb,
            16 as ::core::ffi::c_int,
            (*s).params[w_2 as usize].semimajor_axis_internal_ellipse as BitBuf,
        );
        put_bits(
            pb,
            16 as ::core::ffi::c_int,
            (*s).params[w_2 as usize].semimajor_axis_external_ellipse as BitBuf,
        );
        put_bits(
            pb,
            16 as ::core::ffi::c_int,
            (*s).params[w_2 as usize].semiminor_axis_external_ellipse as BitBuf,
        );
        put_bits(
            pb,
            1 as ::core::ffi::c_int,
            (*s).params[w_2 as usize].overlap_process_option as BitBuf,
        );
        w_2 += 1;
    }
    put_bits(
        pb,
        27 as ::core::ffi::c_int,
        ((*s).targeted_system_display_maximum_luminance.num as int64_t * luminance_den
            / (*s).targeted_system_display_maximum_luminance.den as int64_t) as BitBuf,
    );
    put_bits(
        pb,
        1 as ::core::ffi::c_int,
        (*s).targeted_system_display_actual_peak_luminance_flag as BitBuf,
    );
    if (*s).targeted_system_display_actual_peak_luminance_flag != 0 {
        put_bits(
            pb,
            5 as ::core::ffi::c_int,
            (*s).num_rows_targeted_system_display_actual_peak_luminance as BitBuf,
        );
        put_bits(
            pb,
            5 as ::core::ffi::c_int,
            (*s).num_cols_targeted_system_display_actual_peak_luminance as BitBuf,
        );
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i
            < (*s).num_rows_targeted_system_display_actual_peak_luminance
                as ::core::ffi::c_int
        {
            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j
                < (*s).num_cols_targeted_system_display_actual_peak_luminance
                    as ::core::ffi::c_int
            {
                put_bits(
                    pb,
                    4 as ::core::ffi::c_int,
                    ((*s)
                        .targeted_system_display_actual_peak_luminance[i
                            as usize][j as usize]
                        .num * peak_luminance_den as ::core::ffi::c_int
                        / (*s)
                            .targeted_system_display_actual_peak_luminance[i
                                as usize][j as usize]
                            .den) as BitBuf,
                );
                j += 1;
            }
            i += 1;
        }
    }
    let mut w_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while w_3 < (*s).num_windows as ::core::ffi::c_int {
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < 3 as ::core::ffi::c_int {
            put_bits(
                pb,
                17 as ::core::ffi::c_int,
                ((*s).params[w_3 as usize].maxscl[i_0 as usize].num as int64_t * rgb_den
                    / (*s).params[w_3 as usize].maxscl[i_0 as usize].den as int64_t)
                    as BitBuf,
            );
            i_0 += 1;
        }
        put_bits(
            pb,
            17 as ::core::ffi::c_int,
            ((*s).params[w_3 as usize].average_maxrgb.num as int64_t * rgb_den
                / (*s).params[w_3 as usize].average_maxrgb.den as int64_t) as BitBuf,
        );
        put_bits(
            pb,
            4 as ::core::ffi::c_int,
            (*s).params[w_3 as usize].num_distribution_maxrgb_percentiles as BitBuf,
        );
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1
            < (*s).params[w_3 as usize].num_distribution_maxrgb_percentiles
                as ::core::ffi::c_int
        {
            put_bits(
                pb,
                7 as ::core::ffi::c_int,
                (*s).params[w_3 as usize].distribution_maxrgb[i_1 as usize].percentage
                    as BitBuf,
            );
            put_bits(
                pb,
                17 as ::core::ffi::c_int,
                ((*s)
                    .params[w_3 as usize]
                    .distribution_maxrgb[i_1 as usize]
                    .percentile
                    .num as int64_t * rgb_den
                    / (*s)
                        .params[w_3 as usize]
                        .distribution_maxrgb[i_1 as usize]
                        .percentile
                        .den as int64_t) as BitBuf,
            );
            i_1 += 1;
        }
        put_bits(
            pb,
            10 as ::core::ffi::c_int,
            ((*s).params[w_3 as usize].fraction_bright_pixels.num
                * fraction_pixel_den as ::core::ffi::c_int
                / (*s).params[w_3 as usize].fraction_bright_pixels.den) as BitBuf,
        );
        w_3 += 1;
    }
    put_bits(
        pb,
        1 as ::core::ffi::c_int,
        (*s).mastering_display_actual_peak_luminance_flag as BitBuf,
    );
    if (*s).mastering_display_actual_peak_luminance_flag != 0 {
        put_bits(
            pb,
            5 as ::core::ffi::c_int,
            (*s).num_rows_mastering_display_actual_peak_luminance as BitBuf,
        );
        put_bits(
            pb,
            5 as ::core::ffi::c_int,
            (*s).num_cols_mastering_display_actual_peak_luminance as BitBuf,
        );
        let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_2
            < (*s).num_rows_mastering_display_actual_peak_luminance as ::core::ffi::c_int
        {
            let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j_0
                < (*s).num_cols_mastering_display_actual_peak_luminance
                    as ::core::ffi::c_int
            {
                put_bits(
                    pb,
                    4 as ::core::ffi::c_int,
                    ((*s)
                        .mastering_display_actual_peak_luminance[i_2
                            as usize][j_0 as usize]
                        .num * peak_luminance_den as ::core::ffi::c_int
                        / (*s)
                            .mastering_display_actual_peak_luminance[i_2
                                as usize][j_0 as usize]
                            .den) as BitBuf,
                );
                j_0 += 1;
            }
            i_2 += 1;
        }
    }
    let mut w_4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while w_4 < (*s).num_windows as ::core::ffi::c_int {
        put_bits(
            pb,
            1 as ::core::ffi::c_int,
            (*s).params[w_4 as usize].tone_mapping_flag as BitBuf,
        );
        if (*s).params[w_4 as usize].tone_mapping_flag != 0 {
            put_bits(
                pb,
                12 as ::core::ffi::c_int,
                ((*s).params[w_4 as usize].knee_point_x.num
                    * knee_point_den as ::core::ffi::c_int
                    / (*s).params[w_4 as usize].knee_point_x.den) as BitBuf,
            );
            put_bits(
                pb,
                12 as ::core::ffi::c_int,
                ((*s).params[w_4 as usize].knee_point_y.num
                    * knee_point_den as ::core::ffi::c_int
                    / (*s).params[w_4 as usize].knee_point_y.den) as BitBuf,
            );
            put_bits(
                pb,
                4 as ::core::ffi::c_int,
                (*s).params[w_4 as usize].num_bezier_curve_anchors as BitBuf,
            );
            let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_3
                < (*s).params[w_4 as usize].num_bezier_curve_anchors
                    as ::core::ffi::c_int
            {
                put_bits(
                    pb,
                    10 as ::core::ffi::c_int,
                    ((*s).params[w_4 as usize].bezier_curve_anchors[i_3 as usize].num
                        * bezier_anchor_den as ::core::ffi::c_int
                        / (*s)
                            .params[w_4 as usize]
                            .bezier_curve_anchors[i_3 as usize]
                            .den) as BitBuf,
                );
                i_3 += 1;
            }
            put_bits(
                pb,
                1 as ::core::ffi::c_int,
                (*s).params[w_4 as usize].color_saturation_mapping_flag as BitBuf,
            );
            if (*s).params[w_4 as usize].color_saturation_mapping_flag != 0 {
                put_bits(
                    pb,
                    6 as ::core::ffi::c_int,
                    ((*s).params[w_4 as usize].color_saturation_weight.num
                        * saturation_weight_den as ::core::ffi::c_int
                        / (*s).params[w_4 as usize].color_saturation_weight.den)
                        as BitBuf,
                );
            }
        }
        w_4 += 1;
    }
    flush_put_bits(pb);
    *data = buf;
    if !size.is_null() {
        *size = size_bytes;
    }
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn get_bits_count(mut s: *const GetBitContext) -> ::core::ffi::c_int {
    return (*s).index;
}
#[inline]
unsafe extern "C" fn put_bits(
    mut s: *mut PutBitContext,
    mut n: ::core::ffi::c_int,
    mut value: BitBuf,
) {
    put_bits_no_assert(s, n, value);
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
#[no_mangle]
pub unsafe extern "C" fn av_dynamic_hdr_smpte2094_app5_alloc(
    mut size: *mut size_t,
) -> *mut AVDynamicHDRSmpte2094App5 {
    let mut smpte2094_app5: *mut AVDynamicHDRSmpte2094App5 = av_mallocz(
        ::core::mem::size_of::<AVDynamicHDRSmpte2094App5>() as size_t,
    ) as *mut AVDynamicHDRSmpte2094App5;
    if !size.is_null() {
        *size = (if !smpte2094_app5.is_null() {
            ::core::mem::size_of::<AVDynamicHDRSmpte2094App5>() as usize
        } else {
            0 as usize
        }) as size_t;
    }
    return smpte2094_app5;
}
#[no_mangle]
pub unsafe extern "C" fn av_dynamic_hdr_smpte2094_app5_create_side_data(
    mut frame: *mut AVFrame,
) -> *mut AVDynamicHDRSmpte2094App5 {
    let mut side_data: *mut AVFrameSideData = av_frame_new_side_data(
        frame,
        AV_FRAME_DATA_DYNAMIC_HDR_SMPTE_2094_APP5,
        ::core::mem::size_of::<AVDynamicHDRSmpte2094App5>() as size_t,
    );
    if side_data.is_null() {
        return ::core::ptr::null_mut::<AVDynamicHDRSmpte2094App5>();
    }
    memset(
        (*side_data).data as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<AVDynamicHDRSmpte2094App5>() as size_t,
    );
    return (*side_data).data as *mut AVDynamicHDRSmpte2094App5;
}
#[inline]
unsafe extern "C" fn get_bits_long(
    mut s: *mut GetBitContext,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    if n == 0 {
        return 0 as ::core::ffi::c_uint
    } else if (1 as ::core::ffi::c_int == 0 || 0 != 0) && n <= 25 as ::core::ffi::c_int {
        return get_bits(s, n)
    } else {
        let mut tmp: ::core::ffi::c_uint = 0;
        let mut re_index: ::core::ffi::c_uint = (*s).index as ::core::ffi::c_uint;
        let mut re_cache: ::core::ffi::c_uint = 0;
        let mut re_size_plus8: ::core::ffi::c_uint = (*s).size_in_bits_plus8
            as ::core::ffi::c_uint;
        re_cache = (av_bswap64(
            (*((*s).buffer.offset((re_index >> 3 as ::core::ffi::c_int) as isize)
                as *const unaligned_64))
                .l,
        ) << (re_index & 7 as ::core::ffi::c_uint)
            >> 64 as ::core::ffi::c_int - 32 as ::core::ffi::c_int)
            as ::core::ffi::c_uint;
        tmp = (re_cache >> 32 as ::core::ffi::c_int - n) as ::core::ffi::c_uint;
        re_index = if re_size_plus8 > re_index.wrapping_add(n as ::core::ffi::c_uint) {
            re_index.wrapping_add(n as ::core::ffi::c_uint)
        } else {
            re_size_plus8
        };
        (*s).index = re_index as ::core::ffi::c_int;
        return tmp;
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_dynamic_hdr_smpte2094_app5_from_t35(
    mut s: *mut AVDynamicHDRSmpte2094App5,
    mut data: *const uint8_t,
    mut size: size_t,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut gbc: GetBitContext = GetBitContext {
        buffer: ::core::ptr::null::<uint8_t>(),
        index: 0,
        size_in_bits: 0,
        size_in_bits_plus8: 0,
    };
    let mut gb: *mut GetBitContext = &raw mut gbc;
    let mut ret: ::core::ffi::c_int = 0;
    let mut reserved_zero: ::core::ffi::c_int = 0;
    let mut padded_size: size_t = size.wrapping_add(64 as size_t);
    if s.is_null() {
        return -(22 as ::core::ffi::c_int);
    }
    let mut padded_data: *mut uint8_t = av_mallocz(padded_size) as *mut uint8_t;
    if padded_data.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    memcpy(
        padded_data as *mut ::core::ffi::c_void,
        data as *const ::core::ffi::c_void,
        size,
    );
    ret = init_get_bits8(gb, padded_data, size as ::core::ffi::c_int);
    if !(ret < 0 as ::core::ffi::c_int) {
        if get_bits_left(gb) < 3 as ::core::ffi::c_int {
            ret = -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        } else {
            (*s).application_version = get_bits(gb, 3 as ::core::ffi::c_int) as uint8_t;
            if get_bits_left(gb) < 3 as ::core::ffi::c_int {
                ret = -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                    | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
            } else {
                (*s).minimum_application_version = get_bits(gb, 3 as ::core::ffi::c_int)
                    as uint8_t;
                if get_bits_left(gb) < 2 as ::core::ffi::c_int {
                    ret = -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                        | ('D' as i32) << 16 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint
                        | ('A' as i32 as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                } else {
                    reserved_zero = get_bits(gb, 2 as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                    if reserved_zero != 0 {
                        ret = -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                            | ('D' as i32) << 16 as ::core::ffi::c_int)
                            as ::core::ffi::c_uint
                            | ('A' as i32 as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                    } else if get_bits_left(gb) < 1 as ::core::ffi::c_int {
                        ret = -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                            | ('D' as i32) << 16 as ::core::ffi::c_int)
                            as ::core::ffi::c_uint
                            | ('A' as i32 as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                    } else {
                        (*s).has_custom_hdr_reference_white_flag = get_bits(
                            gb,
                            1 as ::core::ffi::c_int,
                        ) as uint8_t;
                        if get_bits_left(gb) < 1 as ::core::ffi::c_int {
                            ret = -((('I' as i32
                                | ('N' as i32) << 8 as ::core::ffi::c_int
                                | ('D' as i32) << 16 as ::core::ffi::c_int)
                                as ::core::ffi::c_uint
                                | ('A' as i32 as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                        } else {
                            (*s).has_adaptive_tone_map_flag = get_bits(
                                gb,
                                1 as ::core::ffi::c_int,
                            ) as uint8_t;
                            if get_bits_left(gb) < 6 as ::core::ffi::c_int {
                                ret = -((('I' as i32
                                    | ('N' as i32) << 8 as ::core::ffi::c_int
                                    | ('D' as i32) << 16 as ::core::ffi::c_int)
                                    as ::core::ffi::c_uint
                                    | ('A' as i32 as ::core::ffi::c_uint)
                                        << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                            } else {
                                reserved_zero = get_bits(gb, 6 as ::core::ffi::c_int)
                                    as ::core::ffi::c_int;
                                if reserved_zero != 0 {
                                    ret = -((('I' as i32
                                        | ('N' as i32) << 8 as ::core::ffi::c_int
                                        | ('D' as i32) << 16 as ::core::ffi::c_int)
                                        as ::core::ffi::c_uint
                                        | ('A' as i32 as ::core::ffi::c_uint)
                                            << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                                } else {
                                    if (*s).has_custom_hdr_reference_white_flag != 0 {
                                        if get_bits_left(gb) < 16 as ::core::ffi::c_int {
                                            ret = -((('I' as i32
                                                | ('N' as i32) << 8 as ::core::ffi::c_int
                                                | ('D' as i32) << 16 as ::core::ffi::c_int)
                                                as ::core::ffi::c_uint
                                                | ('A' as i32 as ::core::ffi::c_uint)
                                                    << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                                            current_block = 2761875490206857268;
                                        } else {
                                            (*s).hdr_reference_white = get_bits(
                                                gb,
                                                16 as ::core::ffi::c_int,
                                            ) as uint16_t;
                                            current_block = 3934796541983872331;
                                        }
                                    } else {
                                        current_block = 3934796541983872331;
                                    }
                                    match current_block {
                                        2761875490206857268 => {}
                                        _ => {
                                            if (*s).has_adaptive_tone_map_flag == 0 {
                                                ret = 0 as ::core::ffi::c_int;
                                            } else if get_bits_left(gb) < 16 as ::core::ffi::c_int {
                                                ret = -((('I' as i32
                                                    | ('N' as i32) << 8 as ::core::ffi::c_int
                                                    | ('D' as i32) << 16 as ::core::ffi::c_int)
                                                    as ::core::ffi::c_uint
                                                    | ('A' as i32 as ::core::ffi::c_uint)
                                                        << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                                            } else {
                                                (*s).baseline_hdr_headroom = get_bits(
                                                    gb,
                                                    16 as ::core::ffi::c_int,
                                                ) as uint16_t;
                                                if get_bits_left(gb) < 1 as ::core::ffi::c_int {
                                                    ret = -((('I' as i32
                                                        | ('N' as i32) << 8 as ::core::ffi::c_int
                                                        | ('D' as i32) << 16 as ::core::ffi::c_int)
                                                        as ::core::ffi::c_uint
                                                        | ('A' as i32 as ::core::ffi::c_uint)
                                                            << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                                                } else {
                                                    (*s).use_reference_white_tone_mapping_flag = get_bits(
                                                        gb,
                                                        1 as ::core::ffi::c_int,
                                                    ) as uint8_t;
                                                    if (*s).use_reference_white_tone_mapping_flag != 0 {
                                                        if get_bits_left(gb) < 7 as ::core::ffi::c_int {
                                                            ret = -((('I' as i32
                                                                | ('N' as i32) << 8 as ::core::ffi::c_int
                                                                | ('D' as i32) << 16 as ::core::ffi::c_int)
                                                                as ::core::ffi::c_uint
                                                                | ('A' as i32 as ::core::ffi::c_uint)
                                                                    << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                                                        } else {
                                                            reserved_zero = get_bits(gb, 7 as ::core::ffi::c_int)
                                                                as ::core::ffi::c_int;
                                                            ret = if reserved_zero != 0 {
                                                                -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                                                                    | ('D' as i32) << 16 as ::core::ffi::c_int)
                                                                    as ::core::ffi::c_uint
                                                                    | ('A' as i32 as ::core::ffi::c_uint)
                                                                        << 24 as ::core::ffi::c_int) as ::core::ffi::c_int)
                                                            } else {
                                                                0 as ::core::ffi::c_int
                                                            };
                                                        }
                                                    } else if get_bits_left(gb) < 3 as ::core::ffi::c_int {
                                                        ret = -((('I' as i32
                                                            | ('N' as i32) << 8 as ::core::ffi::c_int
                                                            | ('D' as i32) << 16 as ::core::ffi::c_int)
                                                            as ::core::ffi::c_uint
                                                            | ('A' as i32 as ::core::ffi::c_uint)
                                                                << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                                                    } else {
                                                        (*s).num_alternate_images = get_bits(
                                                            gb,
                                                            3 as ::core::ffi::c_int,
                                                        ) as uint8_t;
                                                        if (*s).num_alternate_images as ::core::ffi::c_int
                                                            > 4 as ::core::ffi::c_int
                                                        {
                                                            ret = -((('I' as i32
                                                                | ('N' as i32) << 8 as ::core::ffi::c_int
                                                                | ('D' as i32) << 16 as ::core::ffi::c_int)
                                                                as ::core::ffi::c_uint
                                                                | ('A' as i32 as ::core::ffi::c_uint)
                                                                    << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                                                        } else if get_bits_left(gb) < 2 as ::core::ffi::c_int {
                                                            ret = -((('I' as i32
                                                                | ('N' as i32) << 8 as ::core::ffi::c_int
                                                                | ('D' as i32) << 16 as ::core::ffi::c_int)
                                                                as ::core::ffi::c_uint
                                                                | ('A' as i32 as ::core::ffi::c_uint)
                                                                    << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                                                        } else {
                                                            (*s).gain_application_space_chromaticities_flag = get_bits(
                                                                gb,
                                                                2 as ::core::ffi::c_int,
                                                            ) as uint8_t;
                                                            if get_bits_left(gb) < 1 as ::core::ffi::c_int {
                                                                ret = -((('I' as i32
                                                                    | ('N' as i32) << 8 as ::core::ffi::c_int
                                                                    | ('D' as i32) << 16 as ::core::ffi::c_int)
                                                                    as ::core::ffi::c_uint
                                                                    | ('A' as i32 as ::core::ffi::c_uint)
                                                                        << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                                                            } else {
                                                                (*s).has_common_component_mix_params_flag = get_bits(
                                                                    gb,
                                                                    1 as ::core::ffi::c_int,
                                                                ) as uint8_t;
                                                                if get_bits_left(gb) < 1 as ::core::ffi::c_int {
                                                                    ret = -((('I' as i32
                                                                        | ('N' as i32) << 8 as ::core::ffi::c_int
                                                                        | ('D' as i32) << 16 as ::core::ffi::c_int)
                                                                        as ::core::ffi::c_uint
                                                                        | ('A' as i32 as ::core::ffi::c_uint)
                                                                            << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                                                                } else {
                                                                    (*s).has_common_curve_params_flag = get_bits(
                                                                        gb,
                                                                        1 as ::core::ffi::c_int,
                                                                    ) as uint8_t;
                                                                    if (*s).gain_application_space_chromaticities_flag
                                                                        as ::core::ffi::c_int == 3 as ::core::ffi::c_int
                                                                    {
                                                                        let mut r: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                                                        loop {
                                                                            if !(r < 8 as ::core::ffi::c_int) {
                                                                                current_block = 3879520548144599102;
                                                                                break;
                                                                            }
                                                                            if get_bits_left(gb) < 16 as ::core::ffi::c_int {
                                                                                ret = -((('I' as i32
                                                                                    | ('N' as i32) << 8 as ::core::ffi::c_int
                                                                                    | ('D' as i32) << 16 as ::core::ffi::c_int)
                                                                                    as ::core::ffi::c_uint
                                                                                    | ('A' as i32 as ::core::ffi::c_uint)
                                                                                        << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                                                                                current_block = 2761875490206857268;
                                                                                break;
                                                                            } else {
                                                                                (*s).gain_application_space_chromaticities[r as usize] = get_bits(
                                                                                    gb,
                                                                                    16 as ::core::ffi::c_int,
                                                                                ) as uint16_t;
                                                                                r += 1;
                                                                            }
                                                                        }
                                                                    } else {
                                                                        current_block = 3879520548144599102;
                                                                    }
                                                                    match current_block {
                                                                        2761875490206857268 => {}
                                                                        _ => {
                                                                            let mut a: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                                                            's_417: loop {
                                                                                if !(a < (*s).num_alternate_images as ::core::ffi::c_int) {
                                                                                    current_block = 13876735945283890807;
                                                                                    break;
                                                                                }
                                                                                if get_bits_left(gb) < 16 as ::core::ffi::c_int {
                                                                                    ret = -((('I' as i32
                                                                                        | ('N' as i32) << 8 as ::core::ffi::c_int
                                                                                        | ('D' as i32) << 16 as ::core::ffi::c_int)
                                                                                        as ::core::ffi::c_uint
                                                                                        | ('A' as i32 as ::core::ffi::c_uint)
                                                                                            << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                                                                                    current_block = 2761875490206857268;
                                                                                    break;
                                                                                } else {
                                                                                    (*s).alternate_hdr_headrooms[a as usize] = get_bits(
                                                                                        gb,
                                                                                        16 as ::core::ffi::c_int,
                                                                                    ) as uint16_t;
                                                                                    if a == 0 || (*s).has_common_component_mix_params_flag == 0
                                                                                    {
                                                                                        if get_bits_left(gb) < 2 as ::core::ffi::c_int {
                                                                                            ret = -((('I' as i32
                                                                                                | ('N' as i32) << 8 as ::core::ffi::c_int
                                                                                                | ('D' as i32) << 16 as ::core::ffi::c_int)
                                                                                                as ::core::ffi::c_uint
                                                                                                | ('A' as i32 as ::core::ffi::c_uint)
                                                                                                    << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                                                                                            current_block = 2761875490206857268;
                                                                                            break;
                                                                                        } else {
                                                                                            (*s).component_mixing_type[a as usize] = get_bits(
                                                                                                gb,
                                                                                                2 as ::core::ffi::c_int,
                                                                                            ) as uint8_t;
                                                                                            if (*s).component_mixing_type[a as usize]
                                                                                                as ::core::ffi::c_int != 3 as ::core::ffi::c_int
                                                                                            {
                                                                                                if get_bits_left(gb) < 6 as ::core::ffi::c_int {
                                                                                                    ret = -((('I' as i32
                                                                                                        | ('N' as i32) << 8 as ::core::ffi::c_int
                                                                                                        | ('D' as i32) << 16 as ::core::ffi::c_int)
                                                                                                        as ::core::ffi::c_uint
                                                                                                        | ('A' as i32 as ::core::ffi::c_uint)
                                                                                                            << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                                                                                                    current_block = 2761875490206857268;
                                                                                                    break;
                                                                                                } else {
                                                                                                    reserved_zero = get_bits(gb, 6 as ::core::ffi::c_int)
                                                                                                        as ::core::ffi::c_int;
                                                                                                    if reserved_zero != 0 {
                                                                                                        ret = -((('I' as i32
                                                                                                            | ('N' as i32) << 8 as ::core::ffi::c_int
                                                                                                            | ('D' as i32) << 16 as ::core::ffi::c_int)
                                                                                                            as ::core::ffi::c_uint
                                                                                                            | ('A' as i32 as ::core::ffi::c_uint)
                                                                                                                << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                                                                                                        current_block = 2761875490206857268;
                                                                                                        break;
                                                                                                    }
                                                                                                }
                                                                                            } else {
                                                                                                let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                                                                                while k < 6 as ::core::ffi::c_int {
                                                                                                    if get_bits_left(gb) < 1 as ::core::ffi::c_int {
                                                                                                        ret = -((('I' as i32
                                                                                                            | ('N' as i32) << 8 as ::core::ffi::c_int
                                                                                                            | ('D' as i32) << 16 as ::core::ffi::c_int)
                                                                                                            as ::core::ffi::c_uint
                                                                                                            | ('A' as i32 as ::core::ffi::c_uint)
                                                                                                                << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                                                                                                        current_block = 2761875490206857268;
                                                                                                        break 's_417;
                                                                                                    } else {
                                                                                                        (*s)
                                                                                                            .has_component_mixing_coefficient_flag[a
                                                                                                            as usize][k as usize] = get_bits(
                                                                                                            gb,
                                                                                                            1 as ::core::ffi::c_int,
                                                                                                        ) as uint8_t;
                                                                                                        k += 1;
                                                                                                    }
                                                                                                }
                                                                                                let mut k_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                                                                                while k_0 < 6 as ::core::ffi::c_int {
                                                                                                    if (*s)
                                                                                                        .has_component_mixing_coefficient_flag[a
                                                                                                        as usize][k_0 as usize] != 0
                                                                                                    {
                                                                                                        if get_bits_left(gb) < 16 as ::core::ffi::c_int {
                                                                                                            ret = -((('I' as i32
                                                                                                                | ('N' as i32) << 8 as ::core::ffi::c_int
                                                                                                                | ('D' as i32) << 16 as ::core::ffi::c_int)
                                                                                                                as ::core::ffi::c_uint
                                                                                                                | ('A' as i32 as ::core::ffi::c_uint)
                                                                                                                    << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                                                                                                            current_block = 2761875490206857268;
                                                                                                            break 's_417;
                                                                                                        } else {
                                                                                                            (*s)
                                                                                                                .component_mixing_coefficient[a as usize][k_0 as usize] = get_bits(
                                                                                                                gb,
                                                                                                                16 as ::core::ffi::c_int,
                                                                                                            ) as uint16_t;
                                                                                                        }
                                                                                                    } else {
                                                                                                        (*s)
                                                                                                            .component_mixing_coefficient[a as usize][k_0 as usize] = 0
                                                                                                            as uint16_t;
                                                                                                    }
                                                                                                    k_0 += 1;
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        (*s).component_mixing_type[a as usize] = (*s)
                                                                                            .component_mixing_type[0 as ::core::ffi::c_int as usize];
                                                                                        if (*s).component_mixing_type[a as usize]
                                                                                            as ::core::ffi::c_int == 3 as ::core::ffi::c_int
                                                                                        {
                                                                                            let mut k_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                                                                            while k_1 < 6 as ::core::ffi::c_int {
                                                                                                (*s)
                                                                                                    .has_component_mixing_coefficient_flag[a
                                                                                                    as usize][k_1 as usize] = (*s)
                                                                                                    .has_component_mixing_coefficient_flag[0
                                                                                                    as ::core::ffi::c_int as usize][k_1 as usize];
                                                                                                (*s)
                                                                                                    .component_mixing_coefficient[a as usize][k_1 as usize] = (*s)
                                                                                                    .component_mixing_coefficient[0 as ::core::ffi::c_int
                                                                                                    as usize][k_1 as usize];
                                                                                                k_1 += 1;
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    if a == 0 || (*s).has_common_curve_params_flag == 0 {
                                                                                        if get_bits_left(gb) < 5 as ::core::ffi::c_int {
                                                                                            ret = -((('I' as i32
                                                                                                | ('N' as i32) << 8 as ::core::ffi::c_int
                                                                                                | ('D' as i32) << 16 as ::core::ffi::c_int)
                                                                                                as ::core::ffi::c_uint
                                                                                                | ('A' as i32 as ::core::ffi::c_uint)
                                                                                                    << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                                                                                            current_block = 2761875490206857268;
                                                                                            break;
                                                                                        } else {
                                                                                            (*s).gain_curve_num_control_points_minus_1[a as usize] = get_bits(
                                                                                                gb,
                                                                                                5 as ::core::ffi::c_int,
                                                                                            ) as uint8_t;
                                                                                            if (*s).gain_curve_num_control_points_minus_1[a as usize]
                                                                                                as ::core::ffi::c_int > 31 as ::core::ffi::c_int
                                                                                            {
                                                                                                ret = -((('I' as i32
                                                                                                    | ('N' as i32) << 8 as ::core::ffi::c_int
                                                                                                    | ('D' as i32) << 16 as ::core::ffi::c_int)
                                                                                                    as ::core::ffi::c_uint
                                                                                                    | ('A' as i32 as ::core::ffi::c_uint)
                                                                                                        << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                                                                                                current_block = 2761875490206857268;
                                                                                                break;
                                                                                            } else if get_bits_left(gb) < 1 as ::core::ffi::c_int {
                                                                                                ret = -((('I' as i32
                                                                                                    | ('N' as i32) << 8 as ::core::ffi::c_int
                                                                                                    | ('D' as i32) << 16 as ::core::ffi::c_int)
                                                                                                    as ::core::ffi::c_uint
                                                                                                    | ('A' as i32 as ::core::ffi::c_uint)
                                                                                                        << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                                                                                                current_block = 2761875490206857268;
                                                                                                break;
                                                                                            } else {
                                                                                                (*s).gain_curve_use_pchip_slope_flag[a as usize] = get_bits(
                                                                                                    gb,
                                                                                                    1 as ::core::ffi::c_int,
                                                                                                ) as uint8_t;
                                                                                                if get_bits_left(gb) < 2 as ::core::ffi::c_int {
                                                                                                    ret = -((('I' as i32
                                                                                                        | ('N' as i32) << 8 as ::core::ffi::c_int
                                                                                                        | ('D' as i32) << 16 as ::core::ffi::c_int)
                                                                                                        as ::core::ffi::c_uint
                                                                                                        | ('A' as i32 as ::core::ffi::c_uint)
                                                                                                            << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                                                                                                    current_block = 2761875490206857268;
                                                                                                    break;
                                                                                                } else {
                                                                                                    reserved_zero = get_bits(gb, 2 as ::core::ffi::c_int)
                                                                                                        as ::core::ffi::c_int;
                                                                                                    if reserved_zero != 0 {
                                                                                                        ret = -((('I' as i32
                                                                                                            | ('N' as i32) << 8 as ::core::ffi::c_int
                                                                                                            | ('D' as i32) << 16 as ::core::ffi::c_int)
                                                                                                            as ::core::ffi::c_uint
                                                                                                            | ('A' as i32 as ::core::ffi::c_uint)
                                                                                                                << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                                                                                                        current_block = 2761875490206857268;
                                                                                                        break;
                                                                                                    } else {
                                                                                                        let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                                                                                        while c
                                                                                                            <= (*s).gain_curve_num_control_points_minus_1[a as usize]
                                                                                                                as ::core::ffi::c_int
                                                                                                        {
                                                                                                            if get_bits_left(gb) < 16 as ::core::ffi::c_int {
                                                                                                                ret = -((('I' as i32
                                                                                                                    | ('N' as i32) << 8 as ::core::ffi::c_int
                                                                                                                    | ('D' as i32) << 16 as ::core::ffi::c_int)
                                                                                                                    as ::core::ffi::c_uint
                                                                                                                    | ('A' as i32 as ::core::ffi::c_uint)
                                                                                                                        << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                                                                                                                current_block = 2761875490206857268;
                                                                                                                break 's_417;
                                                                                                            } else {
                                                                                                                (*s).gain_curve_control_points_x[a as usize][c as usize] = get_bits(
                                                                                                                    gb,
                                                                                                                    16 as ::core::ffi::c_int,
                                                                                                                ) as uint16_t;
                                                                                                                c += 1;
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        (*s).gain_curve_num_control_points_minus_1[a as usize] = (*s)
                                                                                            .gain_curve_num_control_points_minus_1[0
                                                                                            as ::core::ffi::c_int as usize];
                                                                                        (*s).gain_curve_use_pchip_slope_flag[a as usize] = (*s)
                                                                                            .gain_curve_use_pchip_slope_flag[0 as ::core::ffi::c_int
                                                                                            as usize];
                                                                                        let mut c_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                                                                        while c_0
                                                                                            <= (*s).gain_curve_num_control_points_minus_1[a as usize]
                                                                                                as ::core::ffi::c_int
                                                                                        {
                                                                                            (*s)
                                                                                                .gain_curve_control_points_x[a as usize][c_0 as usize] = (*s)
                                                                                                .gain_curve_control_points_x[0 as ::core::ffi::c_int
                                                                                                as usize][c_0 as usize];
                                                                                            c_0 += 1;
                                                                                        }
                                                                                    }
                                                                                    let mut c_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                                                                    while c_1
                                                                                        <= (*s).gain_curve_num_control_points_minus_1[a as usize]
                                                                                            as ::core::ffi::c_int
                                                                                    {
                                                                                        if get_bits_left(gb) < 16 as ::core::ffi::c_int {
                                                                                            ret = -((('I' as i32
                                                                                                | ('N' as i32) << 8 as ::core::ffi::c_int
                                                                                                | ('D' as i32) << 16 as ::core::ffi::c_int)
                                                                                                as ::core::ffi::c_uint
                                                                                                | ('A' as i32 as ::core::ffi::c_uint)
                                                                                                    << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                                                                                            current_block = 2761875490206857268;
                                                                                            break 's_417;
                                                                                        } else {
                                                                                            (*s)
                                                                                                .gain_curve_control_points_y[a as usize][c_1 as usize] = get_bits(
                                                                                                gb,
                                                                                                16 as ::core::ffi::c_int,
                                                                                            ) as uint16_t;
                                                                                            c_1 += 1;
                                                                                        }
                                                                                    }
                                                                                    if (*s).gain_curve_use_pchip_slope_flag[a as usize] == 0 {
                                                                                        let mut c_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                                                                        while c_2
                                                                                            <= (*s).gain_curve_num_control_points_minus_1[a as usize]
                                                                                                as ::core::ffi::c_int
                                                                                        {
                                                                                            if get_bits_left(gb) < 16 as ::core::ffi::c_int {
                                                                                                ret = -((('I' as i32
                                                                                                    | ('N' as i32) << 8 as ::core::ffi::c_int
                                                                                                    | ('D' as i32) << 16 as ::core::ffi::c_int)
                                                                                                    as ::core::ffi::c_uint
                                                                                                    | ('A' as i32 as ::core::ffi::c_uint)
                                                                                                        << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                                                                                                current_block = 2761875490206857268;
                                                                                                break 's_417;
                                                                                            } else {
                                                                                                (*s)
                                                                                                    .gain_curve_control_points_theta[a
                                                                                                    as usize][c_2 as usize] = get_bits(
                                                                                                    gb,
                                                                                                    16 as ::core::ffi::c_int,
                                                                                                ) as uint16_t;
                                                                                                c_2 += 1;
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    a += 1;
                                                                                }
                                                                            }
                                                                            match current_block {
                                                                                2761875490206857268 => {}
                                                                                _ => {
                                                                                    ret = 0 as ::core::ffi::c_int;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    av_free(padded_data as *mut ::core::ffi::c_void);
    return ret;
}
#[inline]
unsafe extern "C" fn init_get_bits(
    mut s: *mut GetBitContext,
    mut buffer: *const uint8_t,
    mut bit_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if bit_size
        >= 2147483647 as ::core::ffi::c_int
            - (if 7 as ::core::ffi::c_int
                > 64 as ::core::ffi::c_int * 8 as ::core::ffi::c_int
            {
                7 as ::core::ffi::c_int
            } else {
                64 as ::core::ffi::c_int * 8 as ::core::ffi::c_int
            }) || bit_size < 0 as ::core::ffi::c_int || buffer.is_null()
    {
        bit_size = 0 as ::core::ffi::c_int;
        buffer = ::core::ptr::null::<uint8_t>();
        ret = -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    (*s).buffer = buffer;
    (*s).size_in_bits = bit_size;
    (*s).size_in_bits_plus8 = bit_size + 8 as ::core::ffi::c_int;
    (*s).index = 0 as ::core::ffi::c_int;
    return ret;
}
#[inline]
unsafe extern "C" fn init_get_bits8(
    mut s: *mut GetBitContext,
    mut buffer: *const uint8_t,
    mut byte_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if byte_size > 2147483647 as ::core::ffi::c_int / 8 as ::core::ffi::c_int
        || byte_size < 0 as ::core::ffi::c_int
    {
        byte_size = -(1 as ::core::ffi::c_int);
    }
    return init_get_bits(s, buffer, byte_size * 8 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn av_dynamic_hdr_smpte2094_app5_to_t35(
    mut s: *const AVDynamicHDRSmpte2094App5,
    mut data: *mut *mut uint8_t,
    mut size: *mut size_t,
) -> ::core::ffi::c_int {
    let mut buf: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut size_bytes: size_t = 0;
    let mut size_bits: size_t = 0;
    let mut pbc: PutBitContext = PutBitContext {
        bit_buf: 0,
        bit_left: 0,
        buf: ::core::ptr::null_mut::<uint8_t>(),
        buf_ptr: ::core::ptr::null_mut::<uint8_t>(),
        buf_end: ::core::ptr::null_mut::<uint8_t>(),
    };
    let mut pb: *mut PutBitContext = &raw mut pbc;
    if s.is_null() {
        return -(22 as ::core::ffi::c_int);
    }
    if (data.is_null() || !(*data).is_null()) && size.is_null() {
        return -(22 as ::core::ffi::c_int);
    }
    if (*s).application_version as ::core::ffi::c_int >= 8 as ::core::ffi::c_int
        || (*s).minimum_application_version as ::core::ffi::c_int
            >= 3 as ::core::ffi::c_int
    {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    size_bits = 0 as size_t;
    size_bits = size_bits
        .wrapping_add(
            (3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                as size_t,
        );
    size_bits = size_bits
        .wrapping_add(
            (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 6 as ::core::ffi::c_int)
                as size_t,
        );
    if (*s).has_custom_hdr_reference_white_flag != 0 {
        size_bits = size_bits.wrapping_add(16 as size_t);
    }
    if (*s).has_adaptive_tone_map_flag != 0 {
        size_bits = size_bits
            .wrapping_add(
                (16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t,
            );
        if (*s).use_reference_white_tone_mapping_flag != 0 {
            size_bits = size_bits.wrapping_add(7 as size_t);
        } else {
            size_bits = size_bits
                .wrapping_add(
                    (3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t,
                );
            if (*s).gain_application_space_chromaticities_flag as ::core::ffi::c_int
                == 3 as ::core::ffi::c_int
            {
                size_bits = size_bits
                    .wrapping_add(
                        (16 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as size_t,
                    );
            }
            if (*s).num_alternate_images as ::core::ffi::c_int > 4 as ::core::ffi::c_int
            {
                return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                    | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
            }
            let mut a: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while a < (*s).num_alternate_images as ::core::ffi::c_int {
                size_bits = size_bits.wrapping_add(16 as size_t);
                if a == 0 || (*s).has_common_component_mix_params_flag == 0 {
                    size_bits = size_bits.wrapping_add(2 as size_t);
                    if (*s).component_mixing_type[a as usize] as ::core::ffi::c_int
                        != 3 as ::core::ffi::c_int
                    {
                        size_bits = size_bits.wrapping_add(6 as size_t);
                    } else {
                        size_bits = size_bits.wrapping_add(6 as size_t);
                        let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while k < 6 as ::core::ffi::c_int {
                            if (*s)
                                .has_component_mixing_coefficient_flag[a
                                as usize][k as usize] != 0
                            {
                                size_bits = size_bits.wrapping_add(16 as size_t);
                            }
                            k += 1;
                        }
                    }
                }
                if a == 0 || (*s).has_common_curve_params_flag == 0 {
                    size_bits = size_bits
                        .wrapping_add(
                            (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int) as size_t,
                        );
                    if (*s).gain_curve_num_control_points_minus_1[a as usize]
                        as ::core::ffi::c_int > 31 as ::core::ffi::c_int
                    {
                        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                            | ('D' as i32) << 16 as ::core::ffi::c_int)
                            as ::core::ffi::c_uint
                            | ('A' as i32 as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                    }
                    size_bits = size_bits
                        .wrapping_add(
                            (16 as ::core::ffi::c_int
                                * ((*s).gain_curve_num_control_points_minus_1[a as usize]
                                    as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as size_t,
                        );
                }
                size_bits = size_bits
                    .wrapping_add(
                        (16 as ::core::ffi::c_int
                            * ((*s).gain_curve_num_control_points_minus_1[a as usize]
                                as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as size_t,
                    );
                if (*s).gain_curve_use_pchip_slope_flag[a as usize] == 0 {
                    size_bits = size_bits
                        .wrapping_add(
                            (16 as ::core::ffi::c_int
                                * ((*s).gain_curve_num_control_points_minus_1[a as usize]
                                    as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as size_t,
                        );
                }
                a += 1;
            }
        }
    }
    if size_bits.wrapping_rem(8 as size_t) != 0 {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    size_bytes = size_bits >> 3 as ::core::ffi::c_int;
    if data.is_null() {
        *size = size_bytes;
        return 0 as ::core::ffi::c_int;
    } else if !(*data).is_null() {
        if *size < size_bytes {
            return -((('B' as i32 | ('U' as i32) << 8 as ::core::ffi::c_int
                | ('F' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('S' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        buf = *data;
    } else {
        buf = av_malloc(size_bytes) as *mut uint8_t;
        if buf.is_null() {
            return -(12 as ::core::ffi::c_int);
        }
    }
    init_put_bits(pb, buf, size_bytes as ::core::ffi::c_int);
    put_bits(pb, 3 as ::core::ffi::c_int, (*s).application_version as BitBuf);
    put_bits(pb, 3 as ::core::ffi::c_int, (*s).minimum_application_version as BitBuf);
    put_bits(pb, 2 as ::core::ffi::c_int, 0 as BitBuf);
    put_bits(
        pb,
        1 as ::core::ffi::c_int,
        (*s).has_custom_hdr_reference_white_flag as BitBuf,
    );
    put_bits(pb, 1 as ::core::ffi::c_int, (*s).has_adaptive_tone_map_flag as BitBuf);
    put_bits(pb, 6 as ::core::ffi::c_int, 0 as BitBuf);
    if (*s).has_custom_hdr_reference_white_flag != 0 {
        put_bits(pb, 16 as ::core::ffi::c_int, (*s).hdr_reference_white as BitBuf);
    }
    if (*s).has_adaptive_tone_map_flag != 0 {
        put_bits(pb, 16 as ::core::ffi::c_int, (*s).baseline_hdr_headroom as BitBuf);
        put_bits(
            pb,
            1 as ::core::ffi::c_int,
            (*s).use_reference_white_tone_mapping_flag as BitBuf,
        );
        if (*s).use_reference_white_tone_mapping_flag != 0 {
            put_bits(pb, 7 as ::core::ffi::c_int, 0 as BitBuf);
        } else {
            put_bits(pb, 3 as ::core::ffi::c_int, (*s).num_alternate_images as BitBuf);
            put_bits(
                pb,
                2 as ::core::ffi::c_int,
                (*s).gain_application_space_chromaticities_flag as BitBuf,
            );
            put_bits(
                pb,
                1 as ::core::ffi::c_int,
                (*s).has_common_component_mix_params_flag as BitBuf,
            );
            put_bits(
                pb,
                1 as ::core::ffi::c_int,
                (*s).has_common_curve_params_flag as BitBuf,
            );
            if (*s).gain_application_space_chromaticities_flag as ::core::ffi::c_int
                == 3 as ::core::ffi::c_int
            {
                let mut r: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while r < 8 as ::core::ffi::c_int {
                    put_bits(
                        pb,
                        16 as ::core::ffi::c_int,
                        (*s).gain_application_space_chromaticities[r as usize] as BitBuf,
                    );
                    r += 1;
                }
            }
            let mut a_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while a_0 < (*s).num_alternate_images as ::core::ffi::c_int {
                put_bits(
                    pb,
                    16 as ::core::ffi::c_int,
                    (*s).alternate_hdr_headrooms[a_0 as usize] as BitBuf,
                );
                if a_0 == 0 || (*s).has_common_component_mix_params_flag == 0 {
                    put_bits(
                        pb,
                        2 as ::core::ffi::c_int,
                        (*s).component_mixing_type[a_0 as usize] as BitBuf,
                    );
                    if (*s).component_mixing_type[a_0 as usize] as ::core::ffi::c_int
                        != 3 as ::core::ffi::c_int
                    {
                        put_bits(pb, 6 as ::core::ffi::c_int, 0 as BitBuf);
                    } else {
                        let mut k_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while k_0 < 6 as ::core::ffi::c_int {
                            put_bits(
                                pb,
                                1 as ::core::ffi::c_int,
                                (*s)
                                    .has_component_mixing_coefficient_flag[a_0
                                    as usize][k_0 as usize] as BitBuf,
                            );
                            k_0 += 1;
                        }
                        let mut k_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while k_1 < 6 as ::core::ffi::c_int {
                            if (*s)
                                .has_component_mixing_coefficient_flag[a_0
                                as usize][k_1 as usize] != 0
                            {
                                put_bits(
                                    pb,
                                    16 as ::core::ffi::c_int,
                                    (*s)
                                        .component_mixing_coefficient[a_0 as usize][k_1 as usize]
                                        as BitBuf,
                                );
                            }
                            k_1 += 1;
                        }
                    }
                }
                if a_0 == 0 || (*s).has_common_curve_params_flag == 0 {
                    put_bits(
                        pb,
                        5 as ::core::ffi::c_int,
                        (*s).gain_curve_num_control_points_minus_1[a_0 as usize]
                            as BitBuf,
                    );
                    put_bits(
                        pb,
                        1 as ::core::ffi::c_int,
                        (*s).gain_curve_use_pchip_slope_flag[a_0 as usize] as BitBuf,
                    );
                    put_bits(pb, 2 as ::core::ffi::c_int, 0 as BitBuf);
                    let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while c
                        <= (*s).gain_curve_num_control_points_minus_1[a_0 as usize]
                            as ::core::ffi::c_int
                    {
                        put_bits(
                            pb,
                            16 as ::core::ffi::c_int,
                            (*s).gain_curve_control_points_x[a_0 as usize][c as usize]
                                as BitBuf,
                        );
                        c += 1;
                    }
                }
                let mut c_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while c_0
                    <= (*s).gain_curve_num_control_points_minus_1[a_0 as usize]
                        as ::core::ffi::c_int
                {
                    put_bits(
                        pb,
                        16 as ::core::ffi::c_int,
                        (*s).gain_curve_control_points_y[a_0 as usize][c_0 as usize]
                            as BitBuf,
                    );
                    c_0 += 1;
                }
                if (*s).gain_curve_use_pchip_slope_flag[a_0 as usize] == 0 {
                    let mut c_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while c_1
                        <= (*s).gain_curve_num_control_points_minus_1[a_0 as usize]
                            as ::core::ffi::c_int
                    {
                        put_bits(
                            pb,
                            16 as ::core::ffi::c_int,
                            (*s)
                                .gain_curve_control_points_theta[a_0 as usize][c_1 as usize]
                                as BitBuf,
                        );
                        c_1 += 1;
                    }
                }
                a_0 += 1;
            }
        }
    }
    flush_put_bits(pb);
    *data = buf;
    if !size.is_null() {
        *size = size_bytes;
    }
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn get_bits_left(mut gb: *mut GetBitContext) -> ::core::ffi::c_int {
    return (*gb).size_in_bits - get_bits_count(gb);
}
#[repr(C)]
pub struct AVBuffer {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVDictionary {
    _opaque: [u8; 0],
}
