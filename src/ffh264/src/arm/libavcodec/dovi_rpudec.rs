// Translated from FFmpeg 9.0.1 libavcodec/dovi_rpudec.c
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
    static ff_golomb_vlc_len: [uint8_t; 512];
    static ff_ue_golomb_vlc_code: [uint8_t; 512];
    static ff_log2_tab: [uint8_t; 256];
    fn av_crc_get_table(crc_id: AVCRCId) -> *const AVCRC;
    fn av_refstruct_alloc_ext_c(
        size: size_t,
        flags: ::core::ffi::c_uint,
        opaque: AVRefStructOpaque,
        free_cb: Option<
            unsafe extern "C" fn(AVRefStructOpaque, *mut ::core::ffi::c_void) -> (),
        >,
    ) -> *mut ::core::ffi::c_void;
    fn av_crc(
        ctx: *const AVCRC,
        crc: uint32_t,
        buffer: *const uint8_t,
        length: size_t,
    ) -> uint32_t;
    fn ff_dovi_ctx_unref(s: *mut DOVIContext);
    fn avpriv_request_sample(
        avc: *mut ::core::ffi::c_void,
        msg: *const ::core::ffi::c_char,
        ...
    );
    fn av_refstruct_unref(objp: *mut ::core::ffi::c_void);
    fn av_buffer_create(
        data: *mut uint8_t,
        size: size_t,
        free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint8_t) -> ()>,
        opaque: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_int,
    ) -> *mut AVBufferRef;
    fn av_buffer_unref(buf: *mut *mut AVBufferRef);
    fn ff_dovi_guess_profile_hevc(hdr: *const AVDOVIRpuDataHeader) -> ::core::ffi::c_int;
    static ff_dovi_color_default: AVDOVIColorMetadata;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn av_free(ptr: *mut ::core::ffi::c_void);
    fn av_dovi_metadata_alloc(size: *mut size_t) -> *mut AVDOVIMetadata;
    fn av_frame_new_side_data_from_buf(
        frame: *mut AVFrame,
        type_0: AVFrameSideDataType,
        buf: *mut AVBufferRef,
    ) -> *mut AVFrameSideData;
    fn av_fast_padded_malloc(
        ptr: *mut ::core::ffi::c_void,
        size: *mut ::core::ffi::c_uint,
        min_size: size_t,
    );
}
pub type size_t = ::core::ffi::c_uint;
pub type int16_t = ::core::ffi::c_short;
pub type int32_t = ::core::ffi::c_int;
pub type int64_t = ::core::ffi::c_longlong;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint16_t = ::core::ffi::c_ushort;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint64_t = ::core::ffi::c_ulonglong;
pub type AVCRC = uint32_t;
pub type AVCRCId = ::core::ffi::c_uint;
pub const AV_CRC_MAX: AVCRCId = 8;
pub const AV_CRC_8_EBU: AVCRCId = 7;
pub const AV_CRC_24_IEEE: AVCRCId = 6;
pub const AV_CRC_16_ANSI_LE: AVCRCId = 5;
pub const AV_CRC_32_IEEE_LE: AVCRCId = 4;
pub const AV_CRC_32_IEEE: AVCRCId = 3;
pub const AV_CRC_16_CCITT: AVCRCId = 2;
pub const AV_CRC_16_ANSI: AVCRCId = 1;
pub const AV_CRC_8_ATM: AVCRCId = 0;
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
    pub c2rust_unnamed: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
#[repr(C)]
pub union AVRefStructOpaque {
    pub nc: *mut ::core::ffi::c_void,
    pub c: *const ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub u32_0: uint32_t,
    pub f32_0: ::core::ffi::c_float,
}
pub const RPU_COEFF_FLOAT: C2RustUnnamed_3 = 1;
pub const RPU_COEFF_FIXED: C2RustUnnamed_3 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub u32_0: uint32_t,
    pub f32_0: ::core::ffi::c_float,
}
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
#[no_mangle]
pub unsafe extern "C" fn ff_dovi_get_metadata(
    mut s: *mut DOVIContext,
    mut out_metadata: *mut *mut AVDOVIMetadata,
) -> ::core::ffi::c_int {
    let mut dovi: *mut AVDOVIMetadata = ::core::ptr::null_mut::<AVDOVIMetadata>();
    let mut dovi_size: size_t = 0;
    if (*s).mapping.is_null() || (*s).color.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    dovi = av_dovi_metadata_alloc(&raw mut dovi_size);
    if dovi.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    memcpy(
        av_dovi_get_header(dovi) as *mut ::core::ffi::c_void,
        &raw mut (*s).header as *const ::core::ffi::c_void,
        (18 as size_t).wrapping_add(::core::mem::size_of::<uint8_t>() as size_t),
    );
    memcpy(
        av_dovi_get_mapping(dovi) as *mut ::core::ffi::c_void,
        (*s).mapping as *const ::core::ffi::c_void,
        (5136 as size_t).wrapping_add(::core::mem::size_of::<[uint16_t; 2]>() as size_t),
    );
    memcpy(
        av_dovi_get_color(dovi) as *mut ::core::ffi::c_void,
        (*s).color as *const ::core::ffi::c_void,
        (192 as size_t).wrapping_add(::core::mem::size_of::<uint16_t>() as size_t),
    );
    if !(*s).ext_blocks.is_null() {
        let mut ext: *const DOVIExt = (*s).ext_blocks;
        let mut ext_sz: size_t = if ::core::mem::size_of::<AVDOVIDmData>() as usize
            > (*dovi).ext_block_size as usize
        {
            (*dovi).ext_block_size
        } else {
            ::core::mem::size_of::<AVDOVIDmData>() as size_t
        };
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*ext).num_static {
            let fresh4 = (*dovi).num_ext_blocks;
            (*dovi).num_ext_blocks = (*dovi).num_ext_blocks + 1;
            memcpy(
                av_dovi_get_ext(dovi, fresh4) as *mut ::core::ffi::c_void,
                (&raw const (*ext).dm_static as *const AVDOVIDmData).offset(i as isize)
                    as *const AVDOVIDmData as *const ::core::ffi::c_void,
                ext_sz,
            );
            i += 1;
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < (*ext).num_dynamic {
            let fresh5 = (*dovi).num_ext_blocks;
            (*dovi).num_ext_blocks = (*dovi).num_ext_blocks + 1;
            memcpy(
                av_dovi_get_ext(dovi, fresh5) as *mut ::core::ffi::c_void,
                (&raw const (*ext).dm_dynamic as *const AVDOVIDmData)
                    .offset(i_0 as isize) as *const AVDOVIDmData
                    as *const ::core::ffi::c_void,
                ext_sz,
            );
            i_0 += 1;
        }
    }
    *out_metadata = dovi;
    return dovi_size as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn ff_log2_c(mut v: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if v & 0xffff0000 as ::core::ffi::c_uint != 0 {
        v >>= 16 as ::core::ffi::c_int;
        n += 16 as ::core::ffi::c_int;
    }
    if v & 0xff00 as ::core::ffi::c_uint != 0 {
        v >>= 8 as ::core::ffi::c_int;
        n += 8 as ::core::ffi::c_int;
    }
    n += ff_log2_tab[v as usize] as ::core::ffi::c_int;
    return n;
}
#[inline]
unsafe extern "C" fn get_ue_golomb(mut gb: *mut GetBitContext) -> ::core::ffi::c_int {
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
    if buf
        >= ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as ::core::ffi::c_uint
    {
        buf >>= 32 as ::core::ffi::c_int - 9 as ::core::ffi::c_int;
        re_index = if re_size_plus8
            > re_index
                .wrapping_add(ff_golomb_vlc_len[buf as usize] as ::core::ffi::c_uint)
        {
            re_index.wrapping_add(ff_golomb_vlc_len[buf as usize] as ::core::ffi::c_uint)
        } else {
            re_size_plus8
        };
        (*gb).index = re_index as ::core::ffi::c_int;
        return ff_ue_golomb_vlc_code[buf as usize] as ::core::ffi::c_int;
    } else {
        let mut log: ::core::ffi::c_int = 2 as ::core::ffi::c_int * ff_log2_c(buf)
            - 31 as ::core::ffi::c_int;
        re_index = if re_size_plus8
            > re_index
                .wrapping_add((32 as ::core::ffi::c_int - log) as ::core::ffi::c_uint)
        {
            re_index
                .wrapping_add((32 as ::core::ffi::c_int - log) as ::core::ffi::c_uint)
        } else {
            re_size_plus8
        };
        (*gb).index = re_index as ::core::ffi::c_int;
        if log < 7 as ::core::ffi::c_int {
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        buf >>= log;
        buf = buf.wrapping_sub(1);
        return buf as ::core::ffi::c_int;
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
#[no_mangle]
pub unsafe extern "C" fn ff_dovi_attach_side_data(
    mut s: *mut DOVIContext,
    mut frame: *mut AVFrame,
) -> ::core::ffi::c_int {
    let mut sd: *mut AVFrameSideData = ::core::ptr::null_mut::<AVFrameSideData>();
    let mut dovi: *mut AVDOVIMetadata = ::core::ptr::null_mut::<AVDOVIMetadata>();
    let mut buf: *mut AVBufferRef = ::core::ptr::null_mut::<AVBufferRef>();
    let mut size: ::core::ffi::c_int = 0;
    size = ff_dovi_get_metadata(s, &raw mut dovi);
    if size <= 0 as ::core::ffi::c_int {
        return size;
    }
    buf = av_buffer_create(
        dovi as *mut uint8_t,
        size as size_t,
        None,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        0 as ::core::ffi::c_int,
    );
    if buf.is_null() {
        av_free(dovi as *mut ::core::ffi::c_void);
        return -(12 as ::core::ffi::c_int);
    }
    sd = av_frame_new_side_data_from_buf(frame, AV_FRAME_DATA_DOVI_METADATA, buf);
    if sd.is_null() {
        av_buffer_unref(&raw mut buf);
        return -(12 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn av_make_q(
    mut num: ::core::ffi::c_int,
    mut den: ::core::ffi::c_int,
) -> AVRational {
    let mut r: AVRational = AVRational { num: num, den: den };
    return r;
}
#[inline]
unsafe extern "C" fn get_ue_coef(
    mut gb: *mut GetBitContext,
    mut hdr: *const AVDOVIRpuDataHeader,
) -> uint64_t {
    let mut ipart: uint64_t = 0;
    let mut fpart: C2RustUnnamed_1 = C2RustUnnamed_1 { u32_0: 0 };
    match (*hdr).coef_data_type as ::core::ffi::c_int {
        0 => {
            ipart = get_ue_golomb_long(gb) as uint64_t;
            fpart.u32_0 = get_bits_long(gb, (*hdr).coef_log2_denom as ::core::ffi::c_int)
                as uint32_t;
            return ipart << (*hdr).coef_log2_denom as ::core::ffi::c_int
                | fpart.u32_0 as uint64_t;
        }
        1 => {
            fpart.u32_0 = get_bits_long(gb, 32 as ::core::ffi::c_int) as uint32_t;
            return (fpart.f32_0
                * ((1 as ::core::ffi::c_longlong)
                    << (*hdr).coef_log2_denom as ::core::ffi::c_int)
                    as ::core::ffi::c_float) as uint64_t;
        }
        _ => {}
    }
    return 0 as uint64_t;
}
#[inline]
unsafe extern "C" fn av_refstruct_alloc_ext(
    mut size: size_t,
    mut flags: ::core::ffi::c_uint,
    mut opaque: *mut ::core::ffi::c_void,
    mut free_cb: Option<
        unsafe extern "C" fn(AVRefStructOpaque, *mut ::core::ffi::c_void) -> (),
    >,
) -> *mut ::core::ffi::c_void {
    return av_refstruct_alloc_ext_c(
        size,
        flags,
        AVRefStructOpaque { nc: opaque },
        free_cb,
    );
}
#[inline]
unsafe extern "C" fn av_refstruct_allocz(mut size: size_t) -> *mut ::core::ffi::c_void {
    return av_refstruct_alloc_ext(
        size,
        0 as ::core::ffi::c_uint,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        None,
    );
}
#[inline]
unsafe extern "C" fn get_ue_golomb_long(
    mut gb: *mut GetBitContext,
) -> ::core::ffi::c_uint {
    let mut buf: ::core::ffi::c_uint = 0;
    let mut log: ::core::ffi::c_uint = 0;
    buf = show_bits_long(gb, 32 as ::core::ffi::c_int);
    log = (31 as ::core::ffi::c_int - ff_log2_c(buf)) as ::core::ffi::c_uint;
    skip_bits_long(gb, log as ::core::ffi::c_int);
    return get_bits_long(
            gb,
            log.wrapping_add(1 as ::core::ffi::c_uint) as ::core::ffi::c_int,
        )
        .wrapping_sub(1 as ::core::ffi::c_uint);
}
#[inline]
unsafe extern "C" fn get_se_coef(
    mut gb: *mut GetBitContext,
    mut hdr: *const AVDOVIRpuDataHeader,
) -> int64_t {
    let mut ipart: int64_t = 0;
    let mut fpart: C2RustUnnamed_2 = C2RustUnnamed_2 { u32_0: 0 };
    match (*hdr).coef_data_type as ::core::ffi::c_int {
        0 => {
            ipart = get_se_golomb_long(gb) as int64_t;
            fpart.u32_0 = get_bits_long(gb, (*hdr).coef_log2_denom as ::core::ffi::c_int)
                as uint32_t;
            return ipart
                * ((1 as int64_t) << (*hdr).coef_log2_denom as ::core::ffi::c_int)
                | fpart.u32_0 as int64_t;
        }
        1 => {
            fpart.u32_0 = get_bits_long(gb, 32 as ::core::ffi::c_int) as uint32_t;
            return (fpart.f32_0
                * ((1 as ::core::ffi::c_longlong)
                    << (*hdr).coef_log2_denom as ::core::ffi::c_int)
                    as ::core::ffi::c_float) as int64_t;
        }
        _ => {}
    }
    return 0 as int64_t;
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
unsafe extern "C" fn get_variable_bits(
    mut gb: *mut GetBitContext,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    let mut value: ::core::ffi::c_uint = get_bits(gb, n);
    let mut read_more: ::core::ffi::c_int = get_bits1(gb) as ::core::ffi::c_int;
    while read_more != 0 {
        value = value.wrapping_add(1 as ::core::ffi::c_uint) << n;
        value |= get_bits(gb, n);
        read_more = get_bits1(gb) as ::core::ffi::c_int;
    }
    return value;
}
unsafe extern "C" fn parse_ext_v1(
    mut s: *mut DOVIContext,
    mut gb: *mut GetBitContext,
    mut dm: *mut AVDOVIDmData,
) -> ::core::ffi::c_int {
    match (*dm).level as ::core::ffi::c_int {
        1 => {
            (*dm).c2rust_unnamed.l1.min_pq = get_bits(gb, 12 as ::core::ffi::c_int)
                as uint16_t;
            (*dm).c2rust_unnamed.l1.max_pq = get_bits(gb, 12 as ::core::ffi::c_int)
                as uint16_t;
            (*dm).c2rust_unnamed.l1.avg_pq = get_bits(gb, 12 as ::core::ffi::c_int)
                as uint16_t;
        }
        2 => {
            (*dm).c2rust_unnamed.l2.target_max_pq = get_bits(
                gb,
                12 as ::core::ffi::c_int,
            ) as uint16_t;
            (*dm).c2rust_unnamed.l2.trim_slope = get_bits(gb, 12 as ::core::ffi::c_int)
                as uint16_t;
            (*dm).c2rust_unnamed.l2.trim_offset = get_bits(gb, 12 as ::core::ffi::c_int)
                as uint16_t;
            (*dm).c2rust_unnamed.l2.trim_power = get_bits(gb, 12 as ::core::ffi::c_int)
                as uint16_t;
            (*dm).c2rust_unnamed.l2.trim_chroma_weight = get_bits(
                gb,
                12 as ::core::ffi::c_int,
            ) as uint16_t;
            (*dm).c2rust_unnamed.l2.trim_saturation_gain = get_bits(
                gb,
                12 as ::core::ffi::c_int,
            ) as uint16_t;
            (*dm).c2rust_unnamed.l2.ms_weight = get_sbits(gb, 13 as ::core::ffi::c_int)
                as int16_t;
            if ((*dm).c2rust_unnamed.l2.ms_weight as ::core::ffi::c_int)
                < -(1 as ::core::ffi::c_int)
                || (*dm).c2rust_unnamed.l2.ms_weight as ::core::ffi::c_int
                    > 4095 as ::core::ffi::c_int
            {
                av_log(
                    (*s).logctx,
                    16 as ::core::ffi::c_int,
                    b"RPU validation failed: -1 <= dm->l2.ms_weight = %d <= 4095\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    (*dm).c2rust_unnamed.l2.ms_weight as ::core::ffi::c_int,
                );
                ff_dovi_ctx_unref(s);
                return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                    | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
            }
        }
        4 => {
            (*dm).c2rust_unnamed.l4.anchor_pq = get_bits(gb, 12 as ::core::ffi::c_int)
                as uint16_t;
            (*dm).c2rust_unnamed.l4.anchor_power = get_bits(gb, 12 as ::core::ffi::c_int)
                as uint16_t;
        }
        5 => {
            (*dm).c2rust_unnamed.l5.left_offset = get_bits(gb, 13 as ::core::ffi::c_int)
                as uint16_t;
            (*dm).c2rust_unnamed.l5.right_offset = get_bits(gb, 13 as ::core::ffi::c_int)
                as uint16_t;
            (*dm).c2rust_unnamed.l5.top_offset = get_bits(gb, 13 as ::core::ffi::c_int)
                as uint16_t;
            (*dm).c2rust_unnamed.l5.bottom_offset = get_bits(
                gb,
                13 as ::core::ffi::c_int,
            ) as uint16_t;
        }
        6 => {
            (*dm).c2rust_unnamed.l6.max_luminance = get_bits(
                gb,
                16 as ::core::ffi::c_int,
            ) as uint16_t;
            (*dm).c2rust_unnamed.l6.min_luminance = get_bits(
                gb,
                16 as ::core::ffi::c_int,
            ) as uint16_t;
            (*dm).c2rust_unnamed.l6.max_cll = get_bits(gb, 16 as ::core::ffi::c_int)
                as uint16_t;
            (*dm).c2rust_unnamed.l6.max_fall = get_bits(gb, 16 as ::core::ffi::c_int)
                as uint16_t;
        }
        255 => {
            (*dm).c2rust_unnamed.l255.dm_run_mode = get_bits(gb, 8 as ::core::ffi::c_int)
                as uint8_t;
            (*dm).c2rust_unnamed.l255.dm_run_version = get_bits(
                gb,
                8 as ::core::ffi::c_int,
            ) as uint8_t;
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < 4 as ::core::ffi::c_int {
                (*dm).c2rust_unnamed.l255.dm_debug[i as usize] = get_bits(
                    gb,
                    8 as ::core::ffi::c_int,
                ) as uint8_t;
                i += 1;
            }
        }
        _ => {
            avpriv_request_sample(
                (*s).logctx,
                b"Dolby Vision DM v1 level %u\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*dm).level as ::core::ffi::c_int,
            );
        }
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
#[inline]
unsafe extern "C" fn ff_dovi_rpu_extension_is_static(
    mut level: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    match level {
        6 | 10 | 32 | 254 | 255 => return 1 as ::core::ffi::c_int,
        _ => return 0 as ::core::ffi::c_int,
    };
}
unsafe extern "C" fn get_cie_xy(mut gb: *mut GetBitContext) -> AVCIExy {
    let mut xy: AVCIExy = AVCIExy {
        x: AVRational { num: 0, den: 0 },
        y: AVRational { num: 0, den: 0 },
    };
    let denom: ::core::ffi::c_int = 32767 as ::core::ffi::c_int;
    xy.x = av_make_q(get_sbits(gb, 16 as ::core::ffi::c_int), denom);
    xy.y = av_make_q(get_sbits(gb, 16 as ::core::ffi::c_int), denom);
    return xy;
}
unsafe extern "C" fn parse_ext_v2(
    mut s: *mut DOVIContext,
    mut gb: *mut GetBitContext,
    mut dm: *mut AVDOVIDmData,
    mut ext_block_length: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    match (*dm).level as ::core::ffi::c_int {
        3 => {
            (*dm).c2rust_unnamed.l3.min_pq_offset = get_bits(
                gb,
                12 as ::core::ffi::c_int,
            ) as uint16_t;
            (*dm).c2rust_unnamed.l3.max_pq_offset = get_bits(
                gb,
                12 as ::core::ffi::c_int,
            ) as uint16_t;
            (*dm).c2rust_unnamed.l3.avg_pq_offset = get_bits(
                gb,
                12 as ::core::ffi::c_int,
            ) as uint16_t;
        }
        8 => {
            (*dm).c2rust_unnamed.l8.target_display_index = get_bits(
                gb,
                8 as ::core::ffi::c_int,
            ) as uint8_t;
            (*dm).c2rust_unnamed.l8.trim_slope = get_bits(gb, 12 as ::core::ffi::c_int)
                as uint16_t;
            (*dm).c2rust_unnamed.l8.trim_offset = get_bits(gb, 12 as ::core::ffi::c_int)
                as uint16_t;
            (*dm).c2rust_unnamed.l8.trim_power = get_bits(gb, 12 as ::core::ffi::c_int)
                as uint16_t;
            (*dm).c2rust_unnamed.l8.trim_chroma_weight = get_bits(
                gb,
                12 as ::core::ffi::c_int,
            ) as uint16_t;
            (*dm).c2rust_unnamed.l8.trim_saturation_gain = get_bits(
                gb,
                12 as ::core::ffi::c_int,
            ) as uint16_t;
            (*dm).c2rust_unnamed.l8.ms_weight = get_bits(gb, 12 as ::core::ffi::c_int)
                as uint16_t;
            if !(ext_block_length < 12 as ::core::ffi::c_int) {
                (*dm).c2rust_unnamed.l8.target_mid_contrast = get_bits(
                    gb,
                    12 as ::core::ffi::c_int,
                ) as uint16_t;
                if !(ext_block_length < 13 as ::core::ffi::c_int) {
                    (*dm).c2rust_unnamed.l8.clip_trim = get_bits(
                        gb,
                        12 as ::core::ffi::c_int,
                    ) as uint16_t;
                    if !(ext_block_length < 19 as ::core::ffi::c_int) {
                        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i < 6 as ::core::ffi::c_int {
                            (*dm)
                                .c2rust_unnamed
                                .l8
                                .saturation_vector_field[i as usize] = get_bits(
                                gb,
                                8 as ::core::ffi::c_int,
                            ) as uint8_t;
                            i += 1;
                        }
                        if !(ext_block_length < 25 as ::core::ffi::c_int) {
                            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while i_0 < 6 as ::core::ffi::c_int {
                                (*dm).c2rust_unnamed.l8.hue_vector_field[i_0 as usize] = get_bits(
                                    gb,
                                    8 as ::core::ffi::c_int,
                                ) as uint8_t;
                                i_0 += 1;
                            }
                        }
                    }
                }
            }
        }
        9 => {
            (*dm).c2rust_unnamed.l9.source_primary_index = get_bits(
                gb,
                8 as ::core::ffi::c_int,
            ) as uint8_t;
            if !(ext_block_length < 17 as ::core::ffi::c_int) {
                (*dm).c2rust_unnamed.l9.source_display_primaries.prim.r = get_cie_xy(gb);
                (*dm).c2rust_unnamed.l9.source_display_primaries.prim.g = get_cie_xy(gb);
                (*dm).c2rust_unnamed.l9.source_display_primaries.prim.b = get_cie_xy(gb);
                (*dm).c2rust_unnamed.l9.source_display_primaries.wp = get_cie_xy(gb)
                    as AVWhitepointCoefficients;
            }
        }
        10 => {
            (*dm).c2rust_unnamed.l10.target_display_index = get_bits(
                gb,
                8 as ::core::ffi::c_int,
            ) as uint8_t;
            (*dm).c2rust_unnamed.l10.target_max_pq = get_bits(
                gb,
                12 as ::core::ffi::c_int,
            ) as uint16_t;
            (*dm).c2rust_unnamed.l10.target_min_pq = get_bits(
                gb,
                12 as ::core::ffi::c_int,
            ) as uint16_t;
            (*dm).c2rust_unnamed.l10.target_primary_index = get_bits(
                gb,
                8 as ::core::ffi::c_int,
            ) as uint8_t;
            if !(ext_block_length < 21 as ::core::ffi::c_int) {
                (*dm).c2rust_unnamed.l10.target_display_primaries.prim.r = get_cie_xy(
                    gb,
                );
                (*dm).c2rust_unnamed.l10.target_display_primaries.prim.g = get_cie_xy(
                    gb,
                );
                (*dm).c2rust_unnamed.l10.target_display_primaries.prim.b = get_cie_xy(
                    gb,
                );
                (*dm).c2rust_unnamed.l10.target_display_primaries.wp = get_cie_xy(gb)
                    as AVWhitepointCoefficients;
            }
        }
        11 => {
            (*dm).c2rust_unnamed.l11.content_type = get_bits(gb, 8 as ::core::ffi::c_int)
                as uint8_t;
            (*dm).c2rust_unnamed.l11.whitepoint = get_bits(gb, 4 as ::core::ffi::c_int)
                as uint8_t;
            (*dm).c2rust_unnamed.l11.reference_mode_flag = get_bits1(gb) as uint8_t;
            skip_bits(gb, 3 as ::core::ffi::c_int);
            skip_bits(gb, 8 as ::core::ffi::c_int);
            skip_bits(gb, 8 as ::core::ffi::c_int);
        }
        254 => {
            (*dm).c2rust_unnamed.l254.dm_mode = get_bits(gb, 8 as ::core::ffi::c_int)
                as uint8_t;
            (*dm).c2rust_unnamed.l254.dm_version_index = get_bits(
                gb,
                8 as ::core::ffi::c_int,
            ) as uint8_t;
        }
        _ => {
            avpriv_request_sample(
                (*s).logctx,
                b"Dolby Vision DM v2 level %u\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*dm).level as ::core::ffi::c_int,
            );
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn av_clip_uint16_c(mut a: ::core::ffi::c_int) -> uint16_t {
    if a & !(0xffff as ::core::ffi::c_int) != 0 {
        return (!a >> 31 as ::core::ffi::c_int) as uint16_t
    } else {
        return a as uint16_t
    };
}
#[inline]
unsafe extern "C" fn get_bits_count(mut s: *const GetBitContext) -> ::core::ffi::c_int {
    return (*s).index;
}
unsafe extern "C" fn parse_ext_blocks(
    mut s: *mut DOVIContext,
    mut gb: *mut GetBitContext,
    mut ver: ::core::ffi::c_int,
    mut compression: ::core::ffi::c_int,
    mut err_recognition: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut num_ext_blocks: ::core::ffi::c_int = 0;
    let mut ext_block_length: ::core::ffi::c_int = 0;
    let mut start_pos: ::core::ffi::c_int = 0;
    let mut parsed_bits: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut ext: *mut DOVIExt = (*s).ext_blocks;
    num_ext_blocks = get_ue_golomb_31(gb);
    align_get_bits(gb);
    if num_ext_blocks != 0 && ext.is_null() {
        (*s).ext_blocks = av_refstruct_allocz(
            ::core::mem::size_of::<DOVIExt>() as size_t,
        ) as *mut DOVIExt;
        ext = (*s).ext_blocks;
        if ext.is_null() {
            return -(12 as ::core::ffi::c_int);
        }
    }
    loop {
        let fresh1 = num_ext_blocks;
        num_ext_blocks = num_ext_blocks - 1;
        if !(fresh1 != 0) {
            break;
        }
        let mut current_block_36: u64;
        let mut dummy: AVDOVIDmData = AVDOVIDmData {
            level: 0,
            c2rust_unnamed: C2RustUnnamed_0 {
                l1: AVDOVIDmLevel1 {
                    min_pq: 0,
                    max_pq: 0,
                    avg_pq: 0,
                },
            },
        };
        let mut dm: *mut AVDOVIDmData = ::core::ptr::null_mut::<AVDOVIDmData>();
        let mut level: uint8_t = 0;
        ext_block_length = get_ue_golomb_31(gb);
        level = get_bits(gb, 8 as ::core::ffi::c_int) as uint8_t;
        start_pos = get_bits_count(gb);
        if ff_dovi_rpu_extension_is_static(level as ::core::ffi::c_int) != 0 {
            if compression != 0 {
                av_log(
                    (*s).logctx,
                    24 as ::core::ffi::c_int,
                    b"Compressed DM RPU contains static extension block level %d\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    level as ::core::ffi::c_int,
                );
                if err_recognition
                    & ((1 as ::core::ffi::c_int) << 18 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) != 0
                {
                    return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                        | ('D' as i32) << 16 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint
                        | ('A' as i32 as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                }
                dm = &raw mut dummy;
            } else {
                if (*ext).num_static as usize
                    >= (::core::mem::size_of::<[AVDOVIDmData; 7]>() as usize)
                        .wrapping_div(::core::mem::size_of::<AVDOVIDmData>() as usize)
                {
                    return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                        | ('D' as i32) << 16 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint
                        | ('A' as i32 as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                }
                let fresh2 = (*ext).num_static;
                (*ext).num_static = (*ext).num_static + 1;
                dm = (&raw mut (*ext).dm_static as *mut AVDOVIDmData)
                    .offset(fresh2 as isize) as *mut AVDOVIDmData;
            }
        } else {
            if (*ext).num_dynamic as usize
                >= (::core::mem::size_of::<[AVDOVIDmData; 25]>() as usize)
                    .wrapping_div(::core::mem::size_of::<AVDOVIDmData>() as usize)
            {
                return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                    | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
            }
            let fresh3 = (*ext).num_dynamic;
            (*ext).num_dynamic = (*ext).num_dynamic + 1;
            dm = (&raw mut (*ext).dm_dynamic as *mut AVDOVIDmData)
                .offset(fresh3 as isize) as *mut AVDOVIDmData;
        }
        memset(
            dm as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<AVDOVIDmData>() as size_t,
        );
        (*dm).level = level;
        match ver {
            1 => {
                ret = parse_ext_v1(s, gb, dm);
                current_block_36 = 15345278821338558188;
            }
            2 => {
                ret = parse_ext_v2(s, gb, dm, ext_block_length);
                current_block_36 = 15345278821338558188;
            }
            _ => {
                avpriv_request_sample(
                    (*s).logctx,
                    b"Dolby Vision DM v%d\0" as *const u8 as *const ::core::ffi::c_char,
                    ver,
                );
                current_block_36 = 14668983701164131159;
            }
        }
        match current_block_36 {
            15345278821338558188 => {
                if ret < 0 as ::core::ffi::c_int {
                    return ret;
                }
            }
            _ => {}
        }
        parsed_bits = get_bits_count(gb) - start_pos;
        if parsed_bits > ext_block_length * 8 as ::core::ffi::c_int {
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        skip_bits(gb, ext_block_length * 8 as ::core::ffi::c_int - parsed_bits);
    }
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn skip_bits_long(
    mut s: *mut GetBitContext,
    mut n: ::core::ffi::c_int,
) {
    (*s).index += av_clip_c(n, -(*s).index, (*s).size_in_bits_plus8 - (*s).index);
}
#[inline]
unsafe extern "C" fn get_se_golomb_long(
    mut gb: *mut GetBitContext,
) -> ::core::ffi::c_int {
    let mut buf: ::core::ffi::c_uint = get_ue_golomb_long(gb);
    let mut sign: ::core::ffi::c_int = (buf & 1 as ::core::ffi::c_uint)
        .wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    return (buf >> 1 as ::core::ffi::c_int ^ sign as ::core::ffi::c_uint)
        .wrapping_add(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn get_sbits(
    mut s: *mut GetBitContext,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut tmp: ::core::ffi::c_int = 0;
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
    tmp = (re_cache as int32_t >> 32 as ::core::ffi::c_int - n) as ::core::ffi::c_int;
    re_index = if re_size_plus8 > re_index.wrapping_add(n as ::core::ffi::c_uint) {
        re_index.wrapping_add(n as ::core::ffi::c_uint)
    } else {
        re_size_plus8
    };
    (*s).index = re_index as ::core::ffi::c_int;
    return tmp;
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
#[no_mangle]
pub unsafe extern "C" fn ff_dovi_rpu_parse(
    mut s: *mut DOVIContext,
    mut rpu: *const uint8_t,
    mut rpu_size: size_t,
    mut err_recognition: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut hdr: *mut AVDOVIRpuDataHeader = &raw mut (*s).header;
    let mut fresh0: GetBitContext = GetBitContext {
        buffer: ::core::ptr::null::<uint8_t>(),
        index: 0,
        size_in_bits: 0,
        size_in_bits_plus8: 0,
    };
    let mut gb: *mut GetBitContext = &raw mut fresh0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut rpu_type: uint8_t = 0;
    let mut vdr_seq_info_present: uint8_t = 0;
    let mut vdr_dm_metadata_present: uint8_t = 0;
    let mut dm_compression: uint8_t = 0 as uint8_t;
    let mut use_prev_vdr_rpu: uint8_t = 0;
    let mut use_nlq: uint8_t = 0;
    let mut profile: uint8_t = 0;
    let mut compression: uint8_t = (if (*s).cfg.dv_profile as ::core::ffi::c_int != 0 {
        (*s).cfg.dv_md_compression as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as uint8_t;
    if rpu_size < 5 as size_t {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if (*s).cfg.dv_profile as ::core::ffi::c_int == 10 as ::core::ffi::c_int {
        let header_magic: ::core::ffi::c_uint = 0x1be6841 as ::core::ffi::c_uint;
        let mut emdf_header: ::core::ffi::c_uint = 0;
        let mut emdf_payload_size: ::core::ffi::c_uint = 0;
        let mut emdf_protection: ::core::ffi::c_uint = 0;
        ret = init_get_bits8(gb, rpu, rpu_size as ::core::ffi::c_int);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        emdf_header = get_bits_long(gb, 27 as ::core::ffi::c_int);
        if emdf_header < header_magic || emdf_header > header_magic {
            av_log(
                (*s).logctx,
                16 as ::core::ffi::c_int,
                b"RPU validation failed: header_magic <= emdf_header = %d <= header_magic\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                emdf_header as ::core::ffi::c_int,
            );
            ff_dovi_ctx_unref(s);
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        emdf_payload_size = get_variable_bits(gb, 8 as ::core::ffi::c_int);
        if emdf_payload_size < 6 as ::core::ffi::c_uint
            || emdf_payload_size > 512 as ::core::ffi::c_uint
        {
            av_log(
                (*s).logctx,
                16 as ::core::ffi::c_int,
                b"RPU validation failed: 6 <= emdf_payload_size = %d <= 512\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                emdf_payload_size as ::core::ffi::c_int,
            );
            ff_dovi_ctx_unref(s);
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        if emdf_payload_size.wrapping_mul(8 as ::core::ffi::c_uint)
            > get_bits_left(gb) as ::core::ffi::c_uint
        {
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        av_fast_padded_malloc(
            &raw mut (*s).rpu_buf as *mut ::core::ffi::c_void,
            &raw mut (*s).rpu_buf_sz,
            emdf_payload_size as size_t,
        );
        if (*s).rpu_buf.is_null() {
            return -(12 as ::core::ffi::c_int);
        }
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while (i as ::core::ffi::c_uint) < emdf_payload_size {
            *(*s).rpu_buf.offset(i as isize) = get_bits(gb, 8 as ::core::ffi::c_int)
                as uint8_t;
            i += 1;
        }
        rpu = (*s).rpu_buf;
        rpu_size = emdf_payload_size as size_t;
        emdf_protection = get_bits(
            gb,
            5 as ::core::ffi::c_int + 12 as ::core::ffi::c_int,
        );
        if emdf_protection < 0x400 as ::core::ffi::c_uint
            || emdf_protection > 0x400 as ::core::ffi::c_uint
        {
            av_log(
                (*s).logctx,
                16 as ::core::ffi::c_int,
                b"RPU validation failed: 0x400 <= emdf_protection = %d <= 0x400\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                emdf_protection as ::core::ffi::c_int,
            );
            ff_dovi_ctx_unref(s);
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
    } else {
        if (*rpu.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            < 25 as ::core::ffi::c_int
            || *rpu.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                > 25 as ::core::ffi::c_int
        {
            av_log(
                (*s).logctx,
                16 as ::core::ffi::c_int,
                b"RPU validation failed: 25 <= rpu[0] = %d <= 25\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                *rpu.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            );
            ff_dovi_ctx_unref(s);
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        rpu = rpu.offset(1);
        rpu_size = rpu_size.wrapping_sub(1);
    }
    ret = init_get_bits8(gb, rpu, rpu_size as ::core::ffi::c_int);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    rpu_type = get_bits(gb, 6 as ::core::ffi::c_int) as uint8_t;
    if rpu_type as ::core::ffi::c_int != 2 as ::core::ffi::c_int {
        av_log(
            (*s).logctx,
            24 as ::core::ffi::c_int,
            b"Unrecognized RPU type %u, ignoring\n\0" as *const u8
                as *const ::core::ffi::c_char,
            rpu_type as ::core::ffi::c_int,
        );
        return 0 as ::core::ffi::c_int;
    }
    (*hdr).rpu_type = rpu_type;
    (*hdr).rpu_format = get_bits(gb, 11 as ::core::ffi::c_int) as uint16_t;
    (*hdr).vdr_rpu_profile = get_bits(gb, 4 as ::core::ffi::c_int) as uint8_t;
    (*hdr).vdr_rpu_level = get_bits(gb, 4 as ::core::ffi::c_int) as uint8_t;
    vdr_seq_info_present = get_bits1(gb) as uint8_t;
    if vdr_seq_info_present != 0 {
        (*hdr).chroma_resampling_explicit_filter_flag = get_bits1(gb) as uint8_t;
        (*hdr).coef_data_type = get_bits(gb, 2 as ::core::ffi::c_int) as uint8_t;
        if ((*hdr).coef_data_type as ::core::ffi::c_int)
            < RPU_COEFF_FIXED as ::core::ffi::c_int
            || (*hdr).coef_data_type as ::core::ffi::c_int
                > RPU_COEFF_FLOAT as ::core::ffi::c_int
        {
            av_log(
                (*s).logctx,
                16 as ::core::ffi::c_int,
                b"RPU validation failed: RPU_COEFF_FIXED <= hdr->coef_data_type = %d <= RPU_COEFF_FLOAT\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                (*hdr).coef_data_type as ::core::ffi::c_int,
            );
            ff_dovi_ctx_unref(s);
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        match (*hdr).coef_data_type as ::core::ffi::c_int {
            0 => {
                (*hdr).coef_log2_denom = get_ue_golomb(gb) as uint8_t;
                if ((*hdr).coef_log2_denom as ::core::ffi::c_int)
                    < 13 as ::core::ffi::c_int
                    || (*hdr).coef_log2_denom as ::core::ffi::c_int
                        > 32 as ::core::ffi::c_int
                {
                    av_log(
                        (*s).logctx,
                        16 as ::core::ffi::c_int,
                        b"RPU validation failed: 13 <= hdr->coef_log2_denom = %d <= 32\n\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        (*hdr).coef_log2_denom as ::core::ffi::c_int,
                    );
                    ff_dovi_ctx_unref(s);
                    return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                        | ('D' as i32) << 16 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint
                        | ('A' as i32 as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                }
            }
            1 => {
                (*hdr).coef_log2_denom = 32 as uint8_t;
            }
            _ => {}
        }
        (*hdr).vdr_rpu_normalized_idc = get_bits(gb, 2 as ::core::ffi::c_int) as uint8_t;
        (*hdr).bl_video_full_range_flag = get_bits1(gb) as uint8_t;
        if (*hdr).rpu_format as ::core::ffi::c_int & 0x700 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            let mut bl_bit_depth_minus8: ::core::ffi::c_int = get_ue_golomb_31(gb);
            let mut el_bit_depth_minus8: ::core::ffi::c_int = get_ue_golomb_long(gb)
                as ::core::ffi::c_int;
            let mut vdr_bit_depth_minus8: ::core::ffi::c_int = get_ue_golomb_31(gb);
            let mut ext_mapping_idc: ::core::ffi::c_int = el_bit_depth_minus8
                >> 8 as ::core::ffi::c_int;
            el_bit_depth_minus8 = el_bit_depth_minus8 & 0xff as ::core::ffi::c_int;
            if bl_bit_depth_minus8 < 0 as ::core::ffi::c_int
                || bl_bit_depth_minus8 > 8 as ::core::ffi::c_int
            {
                av_log(
                    (*s).logctx,
                    16 as ::core::ffi::c_int,
                    b"RPU validation failed: 0 <= bl_bit_depth_minus8 = %d <= 8\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    bl_bit_depth_minus8,
                );
                ff_dovi_ctx_unref(s);
                return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                    | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
            }
            if el_bit_depth_minus8 < 0 as ::core::ffi::c_int
                || el_bit_depth_minus8 > 8 as ::core::ffi::c_int
            {
                av_log(
                    (*s).logctx,
                    16 as ::core::ffi::c_int,
                    b"RPU validation failed: 0 <= el_bit_depth_minus8 = %d <= 8\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    el_bit_depth_minus8,
                );
                ff_dovi_ctx_unref(s);
                return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                    | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
            }
            if ext_mapping_idc < 0 as ::core::ffi::c_int
                || ext_mapping_idc > 0xff as ::core::ffi::c_int
            {
                av_log(
                    (*s).logctx,
                    16 as ::core::ffi::c_int,
                    b"RPU validation failed: 0 <= ext_mapping_idc = %d <= 0xFF\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    ext_mapping_idc,
                );
                ff_dovi_ctx_unref(s);
                return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                    | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
            }
            if vdr_bit_depth_minus8 < 0 as ::core::ffi::c_int
                || vdr_bit_depth_minus8 > 8 as ::core::ffi::c_int
            {
                av_log(
                    (*s).logctx,
                    16 as ::core::ffi::c_int,
                    b"RPU validation failed: 0 <= vdr_bit_depth_minus8 = %d <= 8\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    vdr_bit_depth_minus8,
                );
                ff_dovi_ctx_unref(s);
                return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                    | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
            }
            (*hdr).bl_bit_depth = (bl_bit_depth_minus8 + 8 as ::core::ffi::c_int)
                as uint8_t;
            (*hdr).el_bit_depth = (el_bit_depth_minus8 + 8 as ::core::ffi::c_int)
                as uint8_t;
            (*hdr).ext_mapping_idc_0_4 = (ext_mapping_idc & 0x1f as ::core::ffi::c_int)
                as uint8_t;
            (*hdr).ext_mapping_idc_5_7 = (ext_mapping_idc >> 5 as ::core::ffi::c_int)
                as uint8_t;
            (*hdr).vdr_bit_depth = (vdr_bit_depth_minus8 + 8 as ::core::ffi::c_int)
                as uint8_t;
            (*hdr).spatial_resampling_filter_flag = get_bits1(gb) as uint8_t;
            dm_compression = get_bits(gb, 3 as ::core::ffi::c_int) as uint8_t;
            (*hdr).el_spatial_resampling_filter_flag = get_bits1(gb) as uint8_t;
            (*hdr).disable_residual_flag = get_bits1(gb) as uint8_t;
        } else {
            avpriv_request_sample(
                (*s).logctx,
                b"Unsupported RPU format 0x%x\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*hdr).rpu_format as ::core::ffi::c_int,
            );
            ff_dovi_ctx_unref(s);
            return -((('P' as i32 | ('A' as i32) << 8 as ::core::ffi::c_int
                | ('W' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('E' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
    } else {
        avpriv_request_sample(
            (*s).logctx,
            b"Missing RPU VDR sequence info\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        ff_dovi_ctx_unref(s);
        return -((('P' as i32 | ('A' as i32) << 8 as ::core::ffi::c_int
            | ('W' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('E' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    vdr_dm_metadata_present = get_bits1(gb) as uint8_t;
    if dm_compression as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
        av_log(
            (*s).logctx,
            16 as ::core::ffi::c_int,
            b"Dynamic metadata compression is not yet implemented\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -((('P' as i32 | ('A' as i32) << 8 as ::core::ffi::c_int
            | ('W' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('E' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    } else if dm_compression as ::core::ffi::c_int != 0 && vdr_dm_metadata_present == 0 {
        av_log(
            (*s).logctx,
            16 as ::core::ffi::c_int,
            b"Nonzero DM metadata compression method but no DM metadata present\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    use_prev_vdr_rpu = get_bits1(gb) as uint8_t;
    use_nlq = ((*hdr).rpu_format as ::core::ffi::c_int & 0x700 as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int && (*hdr).disable_residual_flag == 0)
        as ::core::ffi::c_int as uint8_t;
    profile = (if (*s).cfg.dv_profile as ::core::ffi::c_int != 0 {
        (*s).cfg.dv_profile as ::core::ffi::c_int
    } else {
        ff_dovi_guess_profile_hevc(hdr)
    }) as uint8_t;
    if profile as ::core::ffi::c_int == 5 as ::core::ffi::c_int
        && use_nlq as ::core::ffi::c_int != 0
    {
        av_log(
            (*s).logctx,
            16 as ::core::ffi::c_int,
            b"Profile 5 RPUs should not use NLQ\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        ff_dovi_ctx_unref(s);
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if err_recognition
        & ((1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) != 0
    {
        if (profile as ::core::ffi::c_int) < 8 as ::core::ffi::c_int
            && compression as ::core::ffi::c_int != 0
        {
            av_log(
                (*s).logctx,
                16 as ::core::ffi::c_int,
                b"Profile %d RPUs should not use metadata compression.\0" as *const u8
                    as *const ::core::ffi::c_char,
                profile as ::core::ffi::c_int,
            );
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        if use_prev_vdr_rpu as ::core::ffi::c_int != 0 && compression == 0 {
            av_log(
                (*s).logctx,
                16 as ::core::ffi::c_int,
                b"Uncompressed RPUs should not have use_prev_vdr_rpu=1\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        if dm_compression as ::core::ffi::c_int != 0 && compression == 0 {
            av_log(
                (*s).logctx,
                16 as ::core::ffi::c_int,
                b"Uncompressed RPUs should not use dm_compression=%d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                dm_compression as ::core::ffi::c_int,
            );
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
    }
    if use_prev_vdr_rpu != 0 {
        let mut prev_vdr_rpu_id: ::core::ffi::c_int = get_ue_golomb_31(gb);
        if prev_vdr_rpu_id < 0 as ::core::ffi::c_int
            || prev_vdr_rpu_id > 15 as ::core::ffi::c_int
        {
            av_log(
                (*s).logctx,
                16 as ::core::ffi::c_int,
                b"RPU validation failed: 0 <= prev_vdr_rpu_id = %d <= DOVI_MAX_DM_ID\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                prev_vdr_rpu_id,
            );
            ff_dovi_ctx_unref(s);
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        if (*s).vdr[prev_vdr_rpu_id as usize].is_null() {
            prev_vdr_rpu_id = 0 as ::core::ffi::c_int;
        }
        if (*s).vdr[prev_vdr_rpu_id as usize].is_null() {
            av_log(
                (*s).logctx,
                16 as ::core::ffi::c_int,
                b"Unknown previous RPU ID: %u\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                prev_vdr_rpu_id,
            );
            ff_dovi_ctx_unref(s);
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        (*s).mapping = (*s).vdr[prev_vdr_rpu_id as usize];
    } else {
        let mut mapping: *mut AVDOVIDataMapping = ::core::ptr::null_mut::<
            AVDOVIDataMapping,
        >();
        let mut vdr_rpu_id: ::core::ffi::c_int = get_ue_golomb_31(gb);
        if vdr_rpu_id < 0 as ::core::ffi::c_int || vdr_rpu_id > 15 as ::core::ffi::c_int
        {
            av_log(
                (*s).logctx,
                16 as ::core::ffi::c_int,
                b"RPU validation failed: 0 <= vdr_rpu_id = %d <= DOVI_MAX_DM_ID\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                vdr_rpu_id,
            );
            ff_dovi_ctx_unref(s);
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        if (*s).vdr[vdr_rpu_id as usize].is_null() {
            (*s).vdr[vdr_rpu_id as usize] = av_refstruct_allocz(
                ::core::mem::size_of::<AVDOVIDataMapping>() as size_t,
            ) as *mut AVDOVIDataMapping;
            if (*s).vdr[vdr_rpu_id as usize].is_null() {
                ff_dovi_ctx_unref(s);
                return -(12 as ::core::ffi::c_int);
            }
        }
        mapping = (*s).vdr[vdr_rpu_id as usize];
        (*s).mapping = mapping;
        (*mapping).vdr_rpu_id = vdr_rpu_id as uint8_t;
        (*mapping).mapping_color_space = get_ue_golomb_31(gb) as uint8_t;
        (*mapping).mapping_chroma_format_idc = get_ue_golomb_31(gb) as uint8_t;
        let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while c < 3 as ::core::ffi::c_int {
            let mut curve: *mut AVDOVIReshapingCurve = (&raw mut (*mapping).curves
                as *mut AVDOVIReshapingCurve)
                .offset(c as isize) as *mut AVDOVIReshapingCurve;
            let mut num_pivots_minus_2: ::core::ffi::c_int = get_ue_golomb_31(gb);
            let mut pivot: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if num_pivots_minus_2 < 0 as ::core::ffi::c_int
                || num_pivots_minus_2 > 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
            {
                av_log(
                    (*s).logctx,
                    16 as ::core::ffi::c_int,
                    b"RPU validation failed: 0 <= num_pivots_minus_2 = %d <= AV_DOVI_MAX_PIECES - 1\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    num_pivots_minus_2,
                );
                ff_dovi_ctx_unref(s);
                return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                    | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
            }
            (*curve).num_pivots = (num_pivots_minus_2 + 2 as ::core::ffi::c_int)
                as uint8_t;
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0 < (*curve).num_pivots as ::core::ffi::c_int {
                pivot = (pivot as ::core::ffi::c_uint)
                    .wrapping_add(
                        get_bits(gb, (*hdr).bl_bit_depth as ::core::ffi::c_int),
                    ) as ::core::ffi::c_int as ::core::ffi::c_int;
                (*curve).pivots[i_0 as usize] = av_clip_uint16_c(pivot);
                i_0 += 1;
            }
            c += 1;
        }
        if use_nlq != 0 {
            let mut nlq_pivot: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            (*mapping).nlq_method_idc = get_bits(gb, 3 as ::core::ffi::c_int)
                as AVDOVINLQMethod;
            let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_1 < 2 as ::core::ffi::c_int {
                nlq_pivot = (nlq_pivot as ::core::ffi::c_uint)
                    .wrapping_add(
                        get_bits(gb, (*hdr).bl_bit_depth as ::core::ffi::c_int),
                    ) as ::core::ffi::c_int as ::core::ffi::c_int;
                (*mapping).nlq_pivots[i_1 as usize] = av_clip_uint16_c(nlq_pivot);
                i_1 += 1;
            }
            if ((*mapping).nlq_method_idc as ::core::ffi::c_int)
                < 0 as ::core::ffi::c_int
                || (*mapping).nlq_method_idc as ::core::ffi::c_int
                    > AV_DOVI_NLQ_LINEAR_DZ as ::core::ffi::c_int
            {
                av_log(
                    (*s).logctx,
                    16 as ::core::ffi::c_int,
                    b"RPU validation failed: 0 <= mapping->nlq_method_idc = %d <= AV_DOVI_NLQ_LINEAR_DZ\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    (*mapping).nlq_method_idc as ::core::ffi::c_int,
                );
                ff_dovi_ctx_unref(s);
                return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                    | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
            }
        } else {
            (*mapping).nlq_method_idc = AV_DOVI_NLQ_NONE;
        }
        (*mapping).num_x_partitions = get_ue_golomb_long(gb)
            .wrapping_add(1 as ::core::ffi::c_uint) as uint32_t;
        (*mapping).num_y_partitions = get_ue_golomb_long(gb)
            .wrapping_add(1 as ::core::ffi::c_uint) as uint32_t;
        if (*mapping).num_x_partitions < 1 as uint32_t
            || (*mapping).num_x_partitions > 0xffff as uint32_t
        {
            av_log(
                (*s).logctx,
                16 as ::core::ffi::c_int,
                b"RPU validation failed: 1 <= mapping->num_x_partitions = %d <= 0xFFFF\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                (*mapping).num_x_partitions as ::core::ffi::c_int,
            );
            ff_dovi_ctx_unref(s);
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        if (*mapping).num_y_partitions < 1 as uint32_t
            || (*mapping).num_y_partitions > 0xffff as uint32_t
        {
            av_log(
                (*s).logctx,
                16 as ::core::ffi::c_int,
                b"RPU validation failed: 1 <= mapping->num_y_partitions = %d <= 0xFFFF\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                (*mapping).num_y_partitions as ::core::ffi::c_int,
            );
            ff_dovi_ctx_unref(s);
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        let mut c_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while c_0 < 3 as ::core::ffi::c_int {
            let mut curve_0: *mut AVDOVIReshapingCurve = (&raw mut (*mapping).curves
                as *mut AVDOVIReshapingCurve)
                .offset(c_0 as isize) as *mut AVDOVIReshapingCurve;
            let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_2
                < (*curve_0).num_pivots as ::core::ffi::c_int - 1 as ::core::ffi::c_int
            {
                let mut mapping_idc: ::core::ffi::c_int = get_ue_golomb_31(gb);
                if mapping_idc < 0 as ::core::ffi::c_int
                    || mapping_idc > 1 as ::core::ffi::c_int
                {
                    av_log(
                        (*s).logctx,
                        16 as ::core::ffi::c_int,
                        b"RPU validation failed: 0 <= mapping_idc = %d <= 1\n\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        mapping_idc,
                    );
                    ff_dovi_ctx_unref(s);
                    return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                        | ('D' as i32) << 16 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint
                        | ('A' as i32 as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                }
                (*curve_0).mapping_idc[i_2 as usize] = mapping_idc
                    as AVDOVIMappingMethod;
                match mapping_idc {
                    0 => {
                        let mut poly_order_minus1: ::core::ffi::c_int = get_ue_golomb_31(
                            gb,
                        );
                        if poly_order_minus1 < 0 as ::core::ffi::c_int
                            || poly_order_minus1 > 1 as ::core::ffi::c_int
                        {
                            av_log(
                                (*s).logctx,
                                16 as ::core::ffi::c_int,
                                b"RPU validation failed: 0 <= poly_order_minus1 = %d <= 1\n\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                                poly_order_minus1,
                            );
                            ff_dovi_ctx_unref(s);
                            return -((('I' as i32
                                | ('N' as i32) << 8 as ::core::ffi::c_int
                                | ('D' as i32) << 16 as ::core::ffi::c_int)
                                as ::core::ffi::c_uint
                                | ('A' as i32 as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                        }
                        (*curve_0).poly_order[i_2 as usize] = (poly_order_minus1
                            + 1 as ::core::ffi::c_int) as uint8_t;
                        if poly_order_minus1 == 0 as ::core::ffi::c_int {
                            let mut linear_interp_flag: ::core::ffi::c_int = get_bits1(
                                gb,
                            ) as ::core::ffi::c_int;
                            if linear_interp_flag != 0 {
                                avpriv_request_sample(
                                    (*s).logctx,
                                    b"Dolby Vision linear interpolation\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                ff_dovi_ctx_unref(s);
                                return -((('P' as i32
                                    | ('A' as i32) << 8 as ::core::ffi::c_int
                                    | ('W' as i32) << 16 as ::core::ffi::c_int)
                                    as ::core::ffi::c_uint
                                    | ('E' as i32 as ::core::ffi::c_uint)
                                        << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                            }
                        }
                        let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while k
                            <= (*curve_0).poly_order[i_2 as usize] as ::core::ffi::c_int
                        {
                            (*curve_0).poly_coef[i_2 as usize][k as usize] = get_se_coef(
                                gb,
                                hdr,
                            );
                            k += 1;
                        }
                    }
                    1 => {
                        let mut mmr_order_minus1: ::core::ffi::c_int = get_bits(
                            gb,
                            2 as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int;
                        if mmr_order_minus1 < 0 as ::core::ffi::c_int
                            || mmr_order_minus1 > 2 as ::core::ffi::c_int
                        {
                            av_log(
                                (*s).logctx,
                                16 as ::core::ffi::c_int,
                                b"RPU validation failed: 0 <= mmr_order_minus1 = %d <= 2\n\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                                mmr_order_minus1,
                            );
                            ff_dovi_ctx_unref(s);
                            return -((('I' as i32
                                | ('N' as i32) << 8 as ::core::ffi::c_int
                                | ('D' as i32) << 16 as ::core::ffi::c_int)
                                as ::core::ffi::c_uint
                                | ('A' as i32 as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                        }
                        (*curve_0).mmr_order[i_2 as usize] = (mmr_order_minus1
                            + 1 as ::core::ffi::c_int) as uint8_t;
                        (*curve_0).mmr_constant[i_2 as usize] = get_se_coef(gb, hdr);
                        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while j
                            < (*curve_0).mmr_order[i_2 as usize] as ::core::ffi::c_int
                        {
                            let mut k_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while k_0 < 7 as ::core::ffi::c_int {
                                (*curve_0)
                                    .mmr_coef[i_2 as usize][j as usize][k_0 as usize] = get_se_coef(
                                    gb,
                                    hdr,
                                );
                                k_0 += 1;
                            }
                            j += 1;
                        }
                    }
                    _ => {}
                }
                i_2 += 1;
            }
            c_0 += 1;
        }
        if use_nlq != 0 {
            let mut c_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while c_1 < 3 as ::core::ffi::c_int {
                let mut nlq: *mut AVDOVINLQParams = (&raw mut (*mapping).nlq
                    as *mut AVDOVINLQParams)
                    .offset(c_1 as isize) as *mut AVDOVINLQParams;
                (*nlq).nlq_offset = get_bits(
                    gb,
                    (*hdr).el_bit_depth as ::core::ffi::c_int,
                ) as uint16_t;
                (*nlq).vdr_in_max = get_ue_coef(gb, hdr);
                match (*mapping).nlq_method_idc as ::core::ffi::c_int {
                    0 => {
                        (*nlq).linear_deadzone_slope = get_ue_coef(gb, hdr);
                        (*nlq).linear_deadzone_threshold = get_ue_coef(gb, hdr);
                    }
                    _ => {}
                }
                c_1 += 1;
            }
        }
    }
    if vdr_dm_metadata_present != 0 {
        let mut color: *mut AVDOVIColorMetadata = ::core::ptr::null_mut::<
            AVDOVIColorMetadata,
        >();
        let mut affected_dm_id: ::core::ffi::c_int = get_ue_golomb_31(gb);
        let mut current_dm_id: ::core::ffi::c_int = get_ue_golomb_31(gb);
        if affected_dm_id < 0 as ::core::ffi::c_int
            || affected_dm_id > 15 as ::core::ffi::c_int
        {
            av_log(
                (*s).logctx,
                16 as ::core::ffi::c_int,
                b"RPU validation failed: 0 <= affected_dm_id = %d <= DOVI_MAX_DM_ID\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                affected_dm_id,
            );
            ff_dovi_ctx_unref(s);
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        if current_dm_id < 0 as ::core::ffi::c_int
            || current_dm_id > 15 as ::core::ffi::c_int
        {
            av_log(
                (*s).logctx,
                16 as ::core::ffi::c_int,
                b"RPU validation failed: 0 <= current_dm_id = %d <= DOVI_MAX_DM_ID\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                current_dm_id,
            );
            ff_dovi_ctx_unref(s);
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        if affected_dm_id != current_dm_id {
            avpriv_request_sample(
                (*s).logctx,
                b"affected/current_dm_metadata_id mismatch? %u != %u\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                affected_dm_id,
                current_dm_id,
            );
            ff_dovi_ctx_unref(s);
            return -((('P' as i32 | ('A' as i32) << 8 as ::core::ffi::c_int
                | ('W' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('E' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        if (*s).dm.is_null() {
            (*s).dm = av_refstruct_allocz(
                ::core::mem::size_of::<AVDOVIColorMetadata>() as size_t,
            ) as *mut AVDOVIColorMetadata;
            if (*s).dm.is_null() {
                ff_dovi_ctx_unref(s);
                return -(12 as ::core::ffi::c_int);
            }
        }
        color = (*s).dm;
        (*s).color = color;
        (*color).dm_metadata_id = affected_dm_id as uint8_t;
        (*color).scene_refresh_flag = get_ue_golomb_31(gb) as uint8_t;
        if dm_compression == 0 {
            let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_3 < 9 as ::core::ffi::c_int {
                (*color).ycc_to_rgb_matrix[i_3 as usize] = av_make_q(
                    get_sbits(gb, 16 as ::core::ffi::c_int),
                    (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int,
                );
                i_3 += 1;
            }
            let mut i_4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_4 < 3 as ::core::ffi::c_int {
                let mut denom: ::core::ffi::c_int = if profile as ::core::ffi::c_int
                    == 4 as ::core::ffi::c_int
                {
                    (1 as ::core::ffi::c_int) << 30 as ::core::ffi::c_int
                } else {
                    (1 as ::core::ffi::c_int) << 28 as ::core::ffi::c_int
                };
                let mut offset: ::core::ffi::c_uint = get_bits_long(
                    gb,
                    32 as ::core::ffi::c_int,
                );
                if offset > 2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint {
                    offset >>= 1 as ::core::ffi::c_int;
                    denom >>= 1 as ::core::ffi::c_int;
                }
                (*color).ycc_to_rgb_offset[i_4 as usize] = av_make_q(
                    offset as ::core::ffi::c_int,
                    denom,
                );
                i_4 += 1;
            }
            let mut i_5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_5 < 9 as ::core::ffi::c_int {
                (*color).rgb_to_lms_matrix[i_5 as usize] = av_make_q(
                    get_sbits(gb, 16 as ::core::ffi::c_int),
                    (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int,
                );
                i_5 += 1;
            }
            (*color).signal_eotf = get_bits(gb, 16 as ::core::ffi::c_int) as uint16_t;
            (*color).signal_eotf_param0 = get_bits(gb, 16 as ::core::ffi::c_int)
                as uint16_t;
            (*color).signal_eotf_param1 = get_bits(gb, 16 as ::core::ffi::c_int)
                as uint16_t;
            (*color).signal_eotf_param2 = get_bits_long(gb, 32 as ::core::ffi::c_int)
                as uint32_t;
            (*color).signal_bit_depth = get_bits(gb, 5 as ::core::ffi::c_int) as uint8_t;
            if ((*color).signal_bit_depth as ::core::ffi::c_int)
                < 8 as ::core::ffi::c_int
                || (*color).signal_bit_depth as ::core::ffi::c_int
                    > 16 as ::core::ffi::c_int
            {
                av_log(
                    (*s).logctx,
                    16 as ::core::ffi::c_int,
                    b"RPU validation failed: 8 <= color->signal_bit_depth = %d <= 16\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    (*color).signal_bit_depth as ::core::ffi::c_int,
                );
                ff_dovi_ctx_unref(s);
                return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                    | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
            }
            (*color).signal_color_space = get_bits(gb, 2 as ::core::ffi::c_int)
                as uint8_t;
            (*color).signal_chroma_format = get_bits(gb, 2 as ::core::ffi::c_int)
                as uint8_t;
            (*color).signal_full_range_flag = get_bits(gb, 2 as ::core::ffi::c_int)
                as uint8_t;
            (*color).source_min_pq = get_bits(gb, 12 as ::core::ffi::c_int) as uint16_t;
            (*color).source_max_pq = get_bits(gb, 12 as ::core::ffi::c_int) as uint16_t;
            (*color).source_diagonal = get_bits(gb, 10 as ::core::ffi::c_int)
                as uint16_t;
        }
        if !(*s).ext_blocks.is_null() {
            let mut ext: *mut DOVIExt = (*s).ext_blocks;
            if dm_compression == 0 {
                (*ext).num_static = 0 as ::core::ffi::c_int;
            }
            (*ext).num_dynamic = 0 as ::core::ffi::c_int;
        }
        ret = parse_ext_blocks(
            s,
            gb,
            1 as ::core::ffi::c_int,
            dm_compression as ::core::ffi::c_int,
            err_recognition,
        );
        if ret < 0 as ::core::ffi::c_int {
            ff_dovi_ctx_unref(s);
            return ret;
        }
        if get_bits_left(gb) > 48 as ::core::ffi::c_int {
            ret = parse_ext_blocks(
                s,
                gb,
                2 as ::core::ffi::c_int,
                dm_compression as ::core::ffi::c_int,
                err_recognition,
            );
            if ret < 0 as ::core::ffi::c_int {
                ff_dovi_ctx_unref(s);
                return ret;
            }
        }
    } else {
        (*s).color = &raw const ff_dovi_color_default;
        av_refstruct_unref(&raw mut (*s).ext_blocks as *mut ::core::ffi::c_void);
    }
    align_get_bits(gb);
    skip_bits(gb, 32 as ::core::ffi::c_int);
    if get_bits(gb, 8 as ::core::ffi::c_int) != 0x80 as ::core::ffi::c_uint {
        avpriv_request_sample(
            (*s).logctx,
            b"Unexpected RPU format\0" as *const u8 as *const ::core::ffi::c_char,
        );
        ff_dovi_ctx_unref(s);
        return -((('P' as i32 | ('A' as i32) << 8 as ::core::ffi::c_int
            | ('W' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('E' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if err_recognition & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
        rpu_size = (get_bits_count(gb) / 8 as ::core::ffi::c_int) as size_t;
        let mut crc: uint32_t = av_bswap32(
            av_crc(
                av_crc_get_table(AV_CRC_32_IEEE),
                -(1 as ::core::ffi::c_int) as uint32_t,
                rpu,
                (rpu_size as size_t).wrapping_sub(1 as size_t),
            ),
        );
        if crc != 0 {
            av_log(
                (*s).logctx,
                16 as ::core::ffi::c_int,
                b"RPU CRC mismatch: %X\n\0" as *const u8 as *const ::core::ffi::c_char,
                crc,
            );
            if err_recognition & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                != 0
            {
                ff_dovi_ctx_unref(s);
                return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                    | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn av_dovi_get_header(
    mut data: *const AVDOVIMetadata,
) -> *mut AVDOVIRpuDataHeader {
    return (data as *mut uint8_t).offset((*data).header_offset as isize)
        as *mut AVDOVIRpuDataHeader;
}
#[inline(always)]
unsafe extern "C" fn av_dovi_get_mapping(
    mut data: *const AVDOVIMetadata,
) -> *mut AVDOVIDataMapping {
    return (data as *mut uint8_t).offset((*data).mapping_offset as isize)
        as *mut AVDOVIDataMapping;
}
#[inline]
unsafe extern "C" fn show_bits(
    mut s: *mut GetBitContext,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    let mut tmp: ::core::ffi::c_uint = 0;
    let mut re_index: ::core::ffi::c_uint = (*s).index as ::core::ffi::c_uint;
    let mut re_cache: ::core::ffi::c_uint = 0;
    re_cache = (av_bswap32(
        (*((*s).buffer.offset((re_index >> 3 as ::core::ffi::c_int) as isize)
            as *const unaligned_32))
            .l,
    ) << (re_index & 7 as ::core::ffi::c_uint)
        >> 32 as ::core::ffi::c_int - 32 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    tmp = (re_cache >> 32 as ::core::ffi::c_int - n) as ::core::ffi::c_uint;
    return tmp;
}
#[inline(always)]
unsafe extern "C" fn av_dovi_get_color(
    mut data: *const AVDOVIMetadata,
) -> *mut AVDOVIColorMetadata {
    return (data as *mut uint8_t).offset((*data).color_offset as isize)
        as *mut AVDOVIColorMetadata;
}
#[inline]
unsafe extern "C" fn skip_bits(mut s: *mut GetBitContext, mut n: ::core::ffi::c_int) {
    let mut re_index: ::core::ffi::c_uint = (*s).index as ::core::ffi::c_uint;
    let mut re_size_plus8: ::core::ffi::c_uint = (*s).size_in_bits_plus8
        as ::core::ffi::c_uint;
    re_index = if re_size_plus8 > re_index.wrapping_add(n as ::core::ffi::c_uint) {
        re_index.wrapping_add(n as ::core::ffi::c_uint)
    } else {
        re_size_plus8
    };
    (*s).index = re_index as ::core::ffi::c_int;
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
#[inline]
unsafe extern "C" fn get_bits_long(
    mut s: *mut GetBitContext,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    if n == 0 {
        return 0 as ::core::ffi::c_uint
    } else if (0 as ::core::ffi::c_int == 0 || 0 != 0) && n <= 25 as ::core::ffi::c_int {
        return get_bits(s, n)
    } else {
        let mut ret: ::core::ffi::c_uint = get_bits(s, 16 as ::core::ffi::c_int)
            << n - 16 as ::core::ffi::c_int;
        return ret | get_bits(s, n - 16 as ::core::ffi::c_int);
    };
}
#[inline]
unsafe extern "C" fn show_bits_long(
    mut s: *mut GetBitContext,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    if n <= 25 as ::core::ffi::c_int {
        return show_bits(s, n)
    } else {
        let mut gb: GetBitContext = *s;
        return get_bits_long(&raw mut gb, n);
    };
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
#[inline]
unsafe extern "C" fn align_get_bits(mut s: *mut GetBitContext) -> *const uint8_t {
    let mut n: ::core::ffi::c_int = -get_bits_count(s) & 7 as ::core::ffi::c_int;
    if n != 0 {
        skip_bits(s, n);
    }
    return (*s).buffer.offset(((*s).index >> 3 as ::core::ffi::c_int) as isize);
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
