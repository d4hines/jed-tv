// Translated from FFmpeg 9.0.1 libavutil/container_fifo.c
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
    fn abort() -> !;
    fn av_fifo_alloc2(
        elems: size_t,
        elem_size: size_t,
        flags: ::core::ffi::c_uint,
    ) -> *mut AVFifo;
    fn av_fifo_can_read(f: *const AVFifo) -> size_t;
    fn av_refstruct_unref(objp: *mut ::core::ffi::c_void);
    fn av_mallocz(size: size_t) -> *mut ::core::ffi::c_void;
    fn av_fifo_write(
        f: *mut AVFifo,
        buf: *const ::core::ffi::c_void,
        nb_elems: size_t,
    ) -> ::core::ffi::c_int;
    fn av_fifo_read(
        f: *mut AVFifo,
        buf: *mut ::core::ffi::c_void,
        nb_elems: size_t,
    ) -> ::core::ffi::c_int;
    fn av_fifo_peek(
        f: *const AVFifo,
        buf: *mut ::core::ffi::c_void,
        nb_elems: size_t,
        offset: size_t,
    ) -> ::core::ffi::c_int;
    fn av_fifo_freep2(f: *mut *mut AVFifo);
    fn av_refstruct_pool_alloc_ext_c(
        size: size_t,
        flags: ::core::ffi::c_uint,
        opaque: AVRefStructOpaque,
        init_cb: Option<
            unsafe extern "C" fn(
                AVRefStructOpaque,
                *mut ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
        reset_cb: Option<
            unsafe extern "C" fn(AVRefStructOpaque, *mut ::core::ffi::c_void) -> (),
        >,
        free_entry_cb: Option<
            unsafe extern "C" fn(AVRefStructOpaque, *mut ::core::ffi::c_void) -> (),
        >,
        free_cb: Option<unsafe extern "C" fn(AVRefStructOpaque) -> ()>,
    ) -> *mut AVRefStructPool;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn av_refstruct_pool_get(pool: *mut AVRefStructPool) -> *mut ::core::ffi::c_void;
    fn av_freep(ptr: *mut ::core::ffi::c_void);
    fn av_frame_alloc() -> *mut AVFrame;
    fn av_frame_free(frame: *mut *mut AVFrame);
    fn av_frame_ref(dst: *mut AVFrame, src: *const AVFrame) -> ::core::ffi::c_int;
    fn av_frame_unref(frame: *mut AVFrame);
    fn av_frame_move_ref(dst: *mut AVFrame, src: *mut AVFrame);
}
pub type size_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVContainerFifo {
    pub fifo: *mut AVFifo,
    pub pool: *mut AVRefStructPool,
    pub opaque: *mut ::core::ffi::c_void,
    pub container_alloc: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
    >,
    pub container_reset: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> (),
    >,
    pub container_free: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> (),
    >,
    pub fifo_transfer: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_uint,
        ) -> ::core::ffi::c_int,
    >,
}
pub type AVContainerFifoFlags = ::core::ffi::c_uint;
pub const AV_CONTAINER_FIFO_FLAG_USER: AVContainerFifoFlags = 65536;
pub const AV_CONTAINER_FIFO_FLAG_REF: AVContainerFifoFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub union AVRefStructOpaque {
    pub nc: *mut ::core::ffi::c_void,
    pub c: *const ::core::ffi::c_void,
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
pub type AVAlphaMode = ::core::ffi::c_uint;
pub const AVALPHA_MODE_NB: AVAlphaMode = 3;
pub const AVALPHA_MODE_STRAIGHT: AVAlphaMode = 2;
pub const AVALPHA_MODE_PREMULTIPLIED: AVAlphaMode = 1;
pub const AVALPHA_MODE_UNSPECIFIED: AVAlphaMode = 0;
pub type int64_t = ::core::ffi::c_long;
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
pub struct AVChannelCustom {
    pub id: AVChannel,
    pub name: [::core::ffi::c_char; 16],
    pub opaque: *mut ::core::ffi::c_void,
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
pub type uint64_t = ::core::ffi::c_ulong;
pub type AVChannelOrder = ::core::ffi::c_uint;
pub const FF_CHANNEL_ORDER_NB: AVChannelOrder = 4;
pub const AV_CHANNEL_ORDER_AMBISONIC: AVChannelOrder = 3;
pub const AV_CHANNEL_ORDER_CUSTOM: AVChannelOrder = 2;
pub const AV_CHANNEL_ORDER_NATIVE: AVChannelOrder = 1;
pub const AV_CHANNEL_ORDER_UNSPEC: AVChannelOrder = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBufferRef {
    pub buffer: *mut AVBuffer,
    pub data: *mut uint8_t,
    pub size: size_t,
}
pub type uint8_t = ::core::ffi::c_uchar;
pub type AVChromaLocation = ::core::ffi::c_uint;
pub const AVCHROMA_LOC_NB: AVChromaLocation = 7;
pub const AVCHROMA_LOC_BOTTOM: AVChromaLocation = 6;
pub const AVCHROMA_LOC_BOTTOMLEFT: AVChromaLocation = 5;
pub const AVCHROMA_LOC_TOP: AVChromaLocation = 4;
pub const AVCHROMA_LOC_TOPLEFT: AVChromaLocation = 3;
pub const AVCHROMA_LOC_CENTER: AVChromaLocation = 2;
pub const AVCHROMA_LOC_LEFT: AVChromaLocation = 1;
pub const AVCHROMA_LOC_UNSPECIFIED: AVChromaLocation = 0;
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
pub type AVColorRange = ::core::ffi::c_uint;
pub const AVCOL_RANGE_NB: AVColorRange = 3;
pub const AVCOL_RANGE_JPEG: AVColorRange = 2;
pub const AVCOL_RANGE_MPEG: AVColorRange = 1;
pub const AVCOL_RANGE_UNSPECIFIED: AVColorRange = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFrameSideData {
    pub type_0: AVFrameSideDataType,
    pub data: *mut uint8_t,
    pub size: size_t,
    pub metadata: *mut AVDictionary,
    pub buf: *mut AVBufferRef,
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
pub struct AVRational {
    pub num: ::core::ffi::c_int,
    pub den: ::core::ffi::c_int,
}
pub type AVPictureType = ::core::ffi::c_uint;
pub const AV_PICTURE_TYPE_BI: AVPictureType = 7;
pub const AV_PICTURE_TYPE_SP: AVPictureType = 6;
pub const AV_PICTURE_TYPE_SI: AVPictureType = 5;
pub const AV_PICTURE_TYPE_S: AVPictureType = 4;
pub const AV_PICTURE_TYPE_B: AVPictureType = 3;
pub const AV_PICTURE_TYPE_P: AVPictureType = 2;
pub const AV_PICTURE_TYPE_I: AVPictureType = 1;
pub const AV_PICTURE_TYPE_NONE: AVPictureType = 0;
unsafe extern "C" fn container_fifo_init_entry(
    mut opaque: AVRefStructOpaque,
    mut obj: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut cf: *mut AVContainerFifo = opaque.nc as *mut AVContainerFifo;
    let mut pobj: *mut *mut ::core::ffi::c_void = obj as *mut *mut ::core::ffi::c_void;
    *pobj = (*cf).container_alloc.expect("non-null function pointer")((*cf).opaque);
    if (*pobj).is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn container_fifo_reset_entry(
    mut opaque: AVRefStructOpaque,
    mut obj: *mut ::core::ffi::c_void,
) {
    let mut cf: *mut AVContainerFifo = opaque.nc as *mut AVContainerFifo;
    (*cf)
        .container_reset
        .expect(
            "non-null function pointer",
        )((*cf).opaque, *(obj as *mut *mut ::core::ffi::c_void));
}
unsafe extern "C" fn container_fifo_free_entry(
    mut opaque: AVRefStructOpaque,
    mut obj: *mut ::core::ffi::c_void,
) {
    let mut cf: *mut AVContainerFifo = opaque.nc as *mut AVContainerFifo;
    (*cf)
        .container_free
        .expect(
            "non-null function pointer",
        )((*cf).opaque, *(obj as *mut *mut ::core::ffi::c_void));
}
#[no_mangle]
pub unsafe extern "C" fn av_container_fifo_alloc(
    mut opaque: *mut ::core::ffi::c_void,
    mut container_alloc: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
    >,
    mut container_reset: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> (),
    >,
    mut container_free: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> (),
    >,
    mut fifo_transfer: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_uint,
        ) -> ::core::ffi::c_int,
    >,
    mut flags: ::core::ffi::c_uint,
) -> *mut AVContainerFifo {
    let mut cf: *mut AVContainerFifo = ::core::ptr::null_mut::<AVContainerFifo>();
    cf = av_mallocz(::core::mem::size_of::<AVContainerFifo>() as size_t)
        as *mut AVContainerFifo;
    if cf.is_null() {
        return ::core::ptr::null_mut::<AVContainerFifo>();
    }
    (*cf).opaque = opaque;
    (*cf).container_alloc = container_alloc;
    (*cf).container_reset = container_reset;
    (*cf).container_free = container_free;
    (*cf).fifo_transfer = fifo_transfer;
    (*cf).fifo = av_fifo_alloc2(
        1 as size_t,
        ::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t,
        ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as ::core::ffi::c_uint,
    );
    if !(*cf).fifo.is_null() {
        (*cf).pool = av_refstruct_pool_alloc_ext(
            ::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t,
            0 as ::core::ffi::c_uint,
            cf as *mut ::core::ffi::c_void,
            Some(
                container_fifo_init_entry
                    as unsafe extern "C" fn(
                        AVRefStructOpaque,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                container_fifo_reset_entry
                    as unsafe extern "C" fn(
                        AVRefStructOpaque,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            ),
            Some(
                container_fifo_free_entry
                    as unsafe extern "C" fn(
                        AVRefStructOpaque,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            ),
            None,
        );
        if !(*cf).pool.is_null() {
            return cf;
        }
    }
    av_container_fifo_free(&raw mut cf);
    return ::core::ptr::null_mut::<AVContainerFifo>();
}
#[no_mangle]
pub unsafe extern "C" fn av_container_fifo_free(mut pcf: *mut *mut AVContainerFifo) {
    let mut cf: *mut AVContainerFifo = ::core::ptr::null_mut::<AVContainerFifo>();
    if (*pcf).is_null() {
        return;
    }
    cf = *pcf;
    if !(*cf).fifo.is_null() {
        let mut obj: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        while av_fifo_read(
            (*cf).fifo,
            &raw mut obj as *mut ::core::ffi::c_void,
            1 as size_t,
        ) >= 0 as ::core::ffi::c_int
        {
            av_refstruct_unref(&raw mut obj as *mut ::core::ffi::c_void);
        }
        av_fifo_freep2(&raw mut (*cf).fifo);
    }
    av_refstruct_pool_uninit(&raw mut (*cf).pool);
    av_freep(pcf as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn av_container_fifo_read(
    mut cf: *mut AVContainerFifo,
    mut obj: *mut ::core::ffi::c_void,
    mut flags: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut psrc: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        *mut ::core::ffi::c_void,
    >();
    let mut ret: ::core::ffi::c_int = 0;
    ret = av_fifo_read(
        (*cf).fifo,
        &raw mut psrc as *mut ::core::ffi::c_void,
        1 as size_t,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    ret = (*cf)
        .fifo_transfer
        .expect("non-null function pointer")((*cf).opaque, obj, *psrc, flags);
    av_refstruct_unref(&raw mut psrc as *mut ::core::ffi::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_container_fifo_peek(
    mut cf: *mut AVContainerFifo,
    mut pdst: *mut *mut ::core::ffi::c_void,
    mut offset: size_t,
) -> ::core::ffi::c_int {
    let mut pobj: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        *mut ::core::ffi::c_void,
    >();
    let mut ret: ::core::ffi::c_int = 0;
    ret = av_fifo_peek(
        (*cf).fifo,
        &raw mut pobj as *mut ::core::ffi::c_void,
        1 as size_t,
        offset,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    *pdst = *pobj;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_container_fifo_drain(
    mut cf: *mut AVContainerFifo,
    mut nb_elems: size_t,
) {
    if !(nb_elems <= av_fifo_can_read((*cf).fifo)) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"nb_elems <= av_fifo_can_read(cf->fifo)\0" as *const u8
                as *const ::core::ffi::c_char,
            b"src/libavutil/container_fifo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            153 as ::core::ffi::c_int,
        );
        abort();
    }
    loop {
        let fresh0 = nb_elems;
        nb_elems = nb_elems.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        let mut pobj: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            *mut ::core::ffi::c_void,
        >();
        let mut ret: ::core::ffi::c_int = av_fifo_read(
            (*cf).fifo,
            &raw mut pobj as *mut ::core::ffi::c_void,
            1 as size_t,
        );
        if !(ret >= 0 as ::core::ffi::c_int) {
            av_log(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                0 as ::core::ffi::c_int,
                b"Assertion %s failed at %s:%d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"ret >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/libavutil/container_fifo.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                157 as ::core::ffi::c_int,
            );
            abort();
        }
        av_refstruct_unref(&raw mut pobj as *mut ::core::ffi::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_container_fifo_write(
    mut cf: *mut AVContainerFifo,
    mut obj: *mut ::core::ffi::c_void,
    mut flags: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut pdst: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        *mut ::core::ffi::c_void,
    >();
    let mut ret: ::core::ffi::c_int = 0;
    pdst = av_refstruct_pool_get((*cf).pool) as *mut *mut ::core::ffi::c_void;
    if pdst.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    ret = (*cf)
        .fifo_transfer
        .expect("non-null function pointer")((*cf).opaque, *pdst, obj, flags);
    if !(ret < 0 as ::core::ffi::c_int) {
        ret = av_fifo_write(
            (*cf).fifo,
            &raw mut pdst as *const ::core::ffi::c_void,
            1 as size_t,
        );
        if !(ret < 0 as ::core::ffi::c_int) {
            return 0 as ::core::ffi::c_int;
        }
    }
    av_refstruct_unref(&raw mut pdst as *mut ::core::ffi::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_container_fifo_can_read(
    mut cf: *const AVContainerFifo,
) -> size_t {
    return av_fifo_can_read((*cf).fifo);
}
unsafe extern "C" fn frame_alloc(
    mut opaque: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    return av_frame_alloc() as *mut ::core::ffi::c_void;
}
unsafe extern "C" fn frame_reset(
    mut opaque: *mut ::core::ffi::c_void,
    mut obj: *mut ::core::ffi::c_void,
) {
    av_frame_unref(obj as *mut AVFrame);
}
unsafe extern "C" fn frame_free(
    mut opaque: *mut ::core::ffi::c_void,
    mut obj: *mut ::core::ffi::c_void,
) {
    let mut frame: *mut AVFrame = obj as *mut AVFrame;
    av_frame_free(&raw mut frame);
}
unsafe extern "C" fn frame_transfer(
    mut opaque: *mut ::core::ffi::c_void,
    mut dst: *mut ::core::ffi::c_void,
    mut src: *mut ::core::ffi::c_void,
    mut flags: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if flags & AV_CONTAINER_FIFO_FLAG_REF as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        return av_frame_ref(dst as *mut AVFrame, src as *const AVFrame);
    }
    av_frame_move_ref(dst as *mut AVFrame, src as *mut AVFrame);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_container_fifo_alloc_avframe(
    mut flags: ::core::ffi::c_uint,
) -> *mut AVContainerFifo {
    return av_container_fifo_alloc(
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        Some(
            frame_alloc
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                ) -> *mut ::core::ffi::c_void,
        ),
        Some(
            frame_reset
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        Some(
            frame_free
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        Some(
            frame_transfer
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_uint,
                ) -> ::core::ffi::c_int,
        ),
        0 as ::core::ffi::c_uint,
    );
}
#[inline]
unsafe extern "C" fn av_refstruct_pool_alloc_ext(
    mut size: size_t,
    mut flags: ::core::ffi::c_uint,
    mut opaque: *mut ::core::ffi::c_void,
    mut init_cb: Option<
        unsafe extern "C" fn(
            AVRefStructOpaque,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    mut reset_cb: Option<
        unsafe extern "C" fn(AVRefStructOpaque, *mut ::core::ffi::c_void) -> (),
    >,
    mut free_entry_cb: Option<
        unsafe extern "C" fn(AVRefStructOpaque, *mut ::core::ffi::c_void) -> (),
    >,
    mut free_cb: Option<unsafe extern "C" fn(AVRefStructOpaque) -> ()>,
) -> *mut AVRefStructPool {
    return av_refstruct_pool_alloc_ext_c(
        size,
        flags,
        AVRefStructOpaque { nc: opaque },
        init_cb,
        reset_cb,
        free_entry_cb,
        free_cb,
    );
}
#[inline]
unsafe extern "C" fn av_refstruct_pool_uninit(mut poolp: *mut *mut AVRefStructPool) {
    av_refstruct_unref(poolp as *mut ::core::ffi::c_void);
}
#[repr(C)]
pub struct AVRefStructPool {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVFifo {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVBuffer {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVDictionary {
    _opaque: [u8; 0],
}
