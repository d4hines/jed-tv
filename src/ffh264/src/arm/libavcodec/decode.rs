// Translated from FFmpeg 9.0.1 libavcodec/decode.c
// (preprocessed for armv7-unknown-linux-musleabihf) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// generic decoding-related code
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
    fn iconv_open(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> iconv_t;
    fn iconv(
        _: iconv_t,
        _: *mut *mut ::core::ffi::c_char,
        _: *mut size_t,
        _: *mut *mut ::core::ffi::c_char,
        _: *mut size_t,
    ) -> size_t;
    fn iconv_close(_: iconv_t) -> ::core::ffi::c_int;
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
    static ff_sd_global_map: [SideDataMap; 0];
    fn abort() -> !;
    fn ff_thread_get_buffer(
        avctx: *mut AVCodecContext,
        f: *mut AVFrame,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ff_thread_progress_init(
        pro: *mut ThreadProgress,
        init_mode: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ff_thread_progress_destroy(pro: *mut ThreadProgress);
    fn ff_thread_progress_report(pro: *mut ThreadProgress, progress: ::core::ffi::c_int);
    fn av_refstruct_alloc_ext_c(
        size: size_t,
        flags: ::core::ffi::c_uint,
        opaque: AVRefStructOpaque,
        free_cb: Option<
            unsafe extern "C" fn(AVRefStructOpaque, *mut ::core::ffi::c_void) -> (),
        >,
    ) -> *mut ::core::ffi::c_void;
    fn av_mastering_display_metadata_alloc_size(
        size: *mut size_t,
    ) -> *mut AVMasteringDisplayMetadata;
    fn ff_thread_progress_await(
        pro: *const ThreadProgress,
        progress: ::core::ffi::c_int,
    );
    fn av_mastering_display_metadata_create_side_data(
        frame: *mut AVFrame,
    ) -> *mut AVMasteringDisplayMetadata;
    fn ff_thread_receive_frame(
        avctx: *mut AVCodecContext,
        frame: *mut AVFrame,
        flags: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn av_strerror(
        errnum: ::core::ffi::c_int,
        errbuf: *mut ::core::ffi::c_char,
        errbuf_size: size_t,
    ) -> ::core::ffi::c_int;
    fn ff_set_dimensions(
        s: *mut AVCodecContext,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ff_thread_get_packet(
        avctx: *mut AVCodecContext,
        pkt: *mut AVPacket,
    ) -> ::core::ffi::c_int;
    fn av_refstruct_unref(objp: *mut ::core::ffi::c_void);
    fn av_content_light_metadata_alloc(size: *mut size_t) -> *mut AVContentLightMetadata;
    fn av_refstruct_ref(obj: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    fn av_mallocz(size: size_t) -> *mut ::core::ffi::c_void;
    fn av_buffer_create(
        data: *mut uint8_t,
        size: size_t,
        free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint8_t) -> ()>,
        opaque: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_int,
    ) -> *mut AVBufferRef;
    fn av_exif_get_tag_id(name: *const ::core::ffi::c_char) -> int32_t;
    fn av_content_light_metadata_create_side_data(
        frame: *mut AVFrame,
    ) -> *mut AVContentLightMetadata;
    fn av_buffer_unref(buf: *mut *mut AVBufferRef);
    fn av_exif_get_entry(
        logctx: *mut ::core::ffi::c_void,
        ifd: *mut AVExifMetadata,
        id: uint16_t,
        flags: ::core::ffi::c_int,
        value: *mut *mut AVExifEntry,
    ) -> ::core::ffi::c_int;
    fn av_bsf_init(ctx: *mut AVBSFContext) -> ::core::ffi::c_int;
    fn av_image_copy(
        dst_data: *const *mut uint8_t,
        dst_linesizes: *const ::core::ffi::c_int,
        src_data: *const *const uint8_t,
        src_linesizes: *const ::core::ffi::c_int,
        pix_fmt: AVPixelFormat,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    );
    fn av_exif_remove_entry(
        logctx: *mut ::core::ffi::c_void,
        ifd: *mut AVExifMetadata,
        id: uint16_t,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn av_buffer_make_writable(buf: *mut *mut AVBufferRef) -> ::core::ffi::c_int;
    fn av_pix_fmt_desc_get(pix_fmt: AVPixelFormat) -> *const AVPixFmtDescriptor;
    fn av_exif_parse_buffer(
        logctx: *mut ::core::ffi::c_void,
        data: *const uint8_t,
        size: size_t,
        ifd: *mut AVExifMetadata,
        header_mode: AVExifHeaderMode,
    ) -> ::core::ffi::c_int;
    fn av_bsf_send_packet(
        ctx: *mut AVBSFContext,
        pkt: *mut AVPacket,
    ) -> ::core::ffi::c_int;
    fn av_exif_write(
        logctx: *mut ::core::ffi::c_void,
        ifd: *const AVExifMetadata,
        buffer: *mut *mut AVBufferRef,
        header_mode: AVExifHeaderMode,
    ) -> ::core::ffi::c_int;
    fn av_rescale_q(a: int64_t, bq: AVRational, cq: AVRational) -> int64_t;
    fn av_exif_free(ifd: *mut AVExifMetadata);
    fn av_buffer_replace(
        dst: *mut *mut AVBufferRef,
        src: *const AVBufferRef,
    ) -> ::core::ffi::c_int;
    fn av_exif_ifd_to_dict(
        logctx: *mut ::core::ffi::c_void,
        ifd: *const AVExifMetadata,
        metadata: *mut *mut AVDictionary,
    ) -> ::core::ffi::c_int;
    fn av_pix_fmt_count_planes(pix_fmt: AVPixelFormat) -> ::core::ffi::c_int;
    fn av_bsf_receive_packet(
        ctx: *mut AVBSFContext,
        pkt: *mut AVPacket,
    ) -> ::core::ffi::c_int;
    fn av_exif_clone_ifd(ifd: *const AVExifMetadata) -> *mut AVExifMetadata;
    fn av_bsf_flush(ctx: *mut AVBSFContext);
    fn av_bsf_free(ctx: *mut *mut AVBSFContext);
    fn av_hwdevice_get_type_name(type_0: AVHWDeviceType) -> *const ::core::ffi::c_char;
    fn av_exif_orientation_to_matrix(
        matrix: *mut int32_t,
        orientation: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
    fn av_samples_copy(
        dst: *const *mut uint8_t,
        src: *const *mut uint8_t,
        dst_offset: ::core::ffi::c_int,
        src_offset: ::core::ffi::c_int,
        nb_samples: ::core::ffi::c_int,
        nb_channels: ::core::ffi::c_int,
        sample_fmt: AVSampleFormat,
    ) -> ::core::ffi::c_int;
    fn av_codec_is_decoder(codec: *const AVCodec) -> ::core::ffi::c_int;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn av_refstruct_pool_get(pool: *mut AVRefStructPool) -> *mut ::core::ffi::c_void;
    fn av_image_check_size2(
        w: ::core::ffi::c_uint,
        h: ::core::ffi::c_uint,
        max_pixels: int64_t,
        pix_fmt: AVPixelFormat,
        log_offset: ::core::ffi::c_int,
        log_ctx: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn av_image_check_sar(
        w: ::core::ffi::c_uint,
        h: ::core::ffi::c_uint,
        sar: AVRational,
    ) -> ::core::ffi::c_int;
    fn av_bsf_list_parse_str(
        str: *const ::core::ffi::c_char,
        bsf: *mut *mut AVBSFContext,
    ) -> ::core::ffi::c_int;
    fn av_get_pix_fmt_name(pix_fmt: AVPixelFormat) -> *const ::core::ffi::c_char;
    fn avcodec_get_hw_config(
        codec: *const AVCodec,
        index: ::core::ffi::c_int,
    ) -> *const AVCodecHWConfig;
    fn av_hwframe_ctx_alloc(device_ctx: *mut AVBufferRef) -> *mut AVBufferRef;
    fn av_free(ptr: *mut ::core::ffi::c_void);
    fn av_hwframe_ctx_init(ref_0: *mut AVBufferRef) -> ::core::ffi::c_int;
    fn av_freep(ptr: *mut ::core::ffi::c_void);
    fn av_memdup(
        p: *const ::core::ffi::c_void,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn av_channel_layout_copy(
        dst: *mut AVChannelLayout,
        src: *const AVChannelLayout,
    ) -> ::core::ffi::c_int;
    fn av_channel_layout_check(
        channel_layout: *const AVChannelLayout,
    ) -> ::core::ffi::c_int;
    fn av_packet_alloc() -> *mut AVPacket;
    fn av_new_packet(pkt: *mut AVPacket, size: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn av_packet_get_side_data(
        pkt: *const AVPacket,
        type_0: AVPacketSideDataType,
        size: *mut size_t,
    ) -> *mut uint8_t;
    fn av_frame_alloc() -> *mut AVFrame;
    fn av_packet_unpack_dictionary(
        data: *const uint8_t,
        size: size_t,
        dict: *mut *mut AVDictionary,
    ) -> ::core::ffi::c_int;
    fn av_frame_free(frame: *mut *mut AVFrame);
    fn av_packet_ref(dst: *mut AVPacket, src: *const AVPacket) -> ::core::ffi::c_int;
    fn av_packet_unref(pkt: *mut AVPacket);
    fn av_frame_unref(frame: *mut AVFrame);
    fn av_packet_copy_props(
        dst: *mut AVPacket,
        src: *const AVPacket,
    ) -> ::core::ffi::c_int;
    fn av_frame_move_ref(dst: *mut AVFrame, src: *mut AVFrame);
    fn av_frame_is_writable(frame: *mut AVFrame) -> ::core::ffi::c_int;
    fn av_frame_new_side_data(
        frame: *mut AVFrame,
        type_0: AVFrameSideDataType,
        size: size_t,
    ) -> *mut AVFrameSideData;
    fn av_frame_get_side_data(
        frame: *const AVFrame,
        type_0: AVFrameSideDataType,
    ) -> *mut AVFrameSideData;
    fn av_frame_remove_side_data(frame: *mut AVFrame, type_0: AVFrameSideDataType);
    fn av_frame_apply_cropping(
        frame: *mut AVFrame,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn av_frame_side_data_free(
        sd: *mut *mut *mut AVFrameSideData,
        nb_sd: *mut ::core::ffi::c_int,
    );
    fn av_frame_side_data_new(
        sd: *mut *mut *mut AVFrameSideData,
        nb_sd: *mut ::core::ffi::c_int,
        type_0: AVFrameSideDataType,
        size: size_t,
        flags: ::core::ffi::c_uint,
    ) -> *mut AVFrameSideData;
    fn av_frame_side_data_add(
        sd: *mut *mut *mut AVFrameSideData,
        nb_sd: *mut ::core::ffi::c_int,
        type_0: AVFrameSideDataType,
        buf: *mut *mut AVBufferRef,
        flags: ::core::ffi::c_uint,
    ) -> *mut AVFrameSideData;
    fn av_frame_side_data_clone(
        sd: *mut *mut *mut AVFrameSideData,
        nb_sd: *mut ::core::ffi::c_int,
        src: *const AVFrameSideData,
        flags: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn av_frame_side_data_get_c(
        sd: *const *const AVFrameSideData,
        nb_sd: ::core::ffi::c_int,
        type_0: AVFrameSideDataType,
    ) -> *const AVFrameSideData;
    fn av_frame_side_data_remove(
        sd: *mut *mut *mut AVFrameSideData,
        nb_sd: *mut ::core::ffi::c_int,
        type_0: AVFrameSideDataType,
    );
    fn avcodec_parameters_from_context(
        par: *mut AVCodecParameters,
        codec: *const AVCodecContext,
    ) -> ::core::ffi::c_int;
    fn avsubtitle_free(sub: *mut AVSubtitle);
    fn avcodec_is_open(s: *mut AVCodecContext) -> ::core::ffi::c_int;
}
pub type int8_t = ::core::ffi::c_schar;
pub type int32_t = ::core::ffi::c_int;
pub type int64_t = ::core::ffi::c_longlong;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint16_t = ::core::ffi::c_ushort;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint64_t = ::core::ffi::c_ulonglong;
pub type size_t = ::core::ffi::c_uint;
pub type iconv_t = *mut ::core::ffi::c_void;
pub type AVClassCategory = ::core::ffi::c_uint;
pub const AV_CLASS_CATEGORY_NB: AVClassCategory = 46;
pub const AV_CLASS_CATEGORY_DEVICE_INPUT: AVClassCategory = 45;
pub const AV_CLASS_CATEGORY_DEVICE_OUTPUT: AVClassCategory = 44;
pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_INPUT: AVClassCategory = 43;
pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_OUTPUT: AVClassCategory = 42;
pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_INPUT: AVClassCategory = 41;
pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_OUTPUT: AVClassCategory = 40;
pub const AV_CLASS_CATEGORY_HWDEVICE: AVClassCategory = 11;
pub const AV_CLASS_CATEGORY_SWRESAMPLER: AVClassCategory = 10;
pub const AV_CLASS_CATEGORY_SWSCALER: AVClassCategory = 9;
pub const AV_CLASS_CATEGORY_BITSTREAM_FILTER: AVClassCategory = 8;
pub const AV_CLASS_CATEGORY_FILTER: AVClassCategory = 7;
pub const AV_CLASS_CATEGORY_DECODER: AVClassCategory = 6;
pub const AV_CLASS_CATEGORY_ENCODER: AVClassCategory = 5;
pub const AV_CLASS_CATEGORY_DEMUXER: AVClassCategory = 4;
pub const AV_CLASS_CATEGORY_MUXER: AVClassCategory = 3;
pub const AV_CLASS_CATEGORY_OUTPUT: AVClassCategory = 2;
pub const AV_CLASS_CATEGORY_INPUT: AVClassCategory = 1;
pub const AV_CLASS_CATEGORY_NA: AVClassCategory = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVClass {
    pub class_name: *const ::core::ffi::c_char,
    pub item_name: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const ::core::ffi::c_char,
    >,
    pub option: *const AVOption,
    pub version: ::core::ffi::c_int,
    pub log_level_offset_offset: ::core::ffi::c_int,
    pub parent_log_context_offset: ::core::ffi::c_int,
    pub category: AVClassCategory,
    pub get_category: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> AVClassCategory,
    >,
    pub query_ranges: Option<
        unsafe extern "C" fn(
            *mut *mut AVOptionRanges,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub child_next: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub child_class_iterate: Option<
        unsafe extern "C" fn(*mut *mut ::core::ffi::c_void) -> *const AVClass,
    >,
    pub state_flags_offset: ::core::ffi::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: ::core::ffi::c_int,
    pub den: ::core::ffi::c_int,
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
pub type AVPixelFormat = ::core::ffi::c_int;
pub const AV_PIX_FMT_NB: AVPixelFormat = 267;
pub const AV_PIX_FMT_OHCODEC: AVPixelFormat = 266;
pub const AV_PIX_FMT_GBRP12MSBLE: AVPixelFormat = 265;
pub const AV_PIX_FMT_GBRP12MSBBE: AVPixelFormat = 264;
pub const AV_PIX_FMT_GBRP10MSBLE: AVPixelFormat = 263;
pub const AV_PIX_FMT_GBRP10MSBBE: AVPixelFormat = 262;
pub const AV_PIX_FMT_YUV444P12MSBLE: AVPixelFormat = 261;
pub const AV_PIX_FMT_YUV444P12MSBBE: AVPixelFormat = 260;
pub const AV_PIX_FMT_YUV444P10MSBLE: AVPixelFormat = 259;
pub const AV_PIX_FMT_YUV444P10MSBBE: AVPixelFormat = 258;
pub const AV_PIX_FMT_GBRAP32LE: AVPixelFormat = 257;
pub const AV_PIX_FMT_GBRAP32BE: AVPixelFormat = 256;
pub const AV_PIX_FMT_YAF16LE: AVPixelFormat = 255;
pub const AV_PIX_FMT_YAF16BE: AVPixelFormat = 254;
pub const AV_PIX_FMT_YAF32LE: AVPixelFormat = 253;
pub const AV_PIX_FMT_YAF32BE: AVPixelFormat = 252;
pub const AV_PIX_FMT_GRAY32LE: AVPixelFormat = 251;
pub const AV_PIX_FMT_GRAY32BE: AVPixelFormat = 250;
pub const AV_PIX_FMT_AMF_SURFACE: AVPixelFormat = 249;
pub const AV_PIX_FMT_GRAYF16LE: AVPixelFormat = 248;
pub const AV_PIX_FMT_GRAYF16BE: AVPixelFormat = 247;
pub const AV_PIX_FMT_GBRAPF16LE: AVPixelFormat = 246;
pub const AV_PIX_FMT_GBRAPF16BE: AVPixelFormat = 245;
pub const AV_PIX_FMT_GBRPF16LE: AVPixelFormat = 244;
pub const AV_PIX_FMT_GBRPF16BE: AVPixelFormat = 243;
pub const AV_PIX_FMT_XV48LE: AVPixelFormat = 242;
pub const AV_PIX_FMT_XV48BE: AVPixelFormat = 241;
pub const AV_PIX_FMT_Y216LE: AVPixelFormat = 240;
pub const AV_PIX_FMT_Y216BE: AVPixelFormat = 239;
pub const AV_PIX_FMT_RGB96LE: AVPixelFormat = 238;
pub const AV_PIX_FMT_RGB96BE: AVPixelFormat = 237;
pub const AV_PIX_FMT_RGBA128LE: AVPixelFormat = 236;
pub const AV_PIX_FMT_RGBA128BE: AVPixelFormat = 235;
pub const AV_PIX_FMT_RGBF16LE: AVPixelFormat = 234;
pub const AV_PIX_FMT_RGBF16BE: AVPixelFormat = 233;
pub const AV_PIX_FMT_V30XLE: AVPixelFormat = 232;
pub const AV_PIX_FMT_V30XBE: AVPixelFormat = 231;
pub const AV_PIX_FMT_VYU444: AVPixelFormat = 230;
pub const AV_PIX_FMT_UYVA: AVPixelFormat = 229;
pub const AV_PIX_FMT_AYUV: AVPixelFormat = 228;
pub const AV_PIX_FMT_D3D12: AVPixelFormat = 227;
pub const AV_PIX_FMT_GBRAP14LE: AVPixelFormat = 226;
pub const AV_PIX_FMT_GBRAP14BE: AVPixelFormat = 225;
pub const AV_PIX_FMT_P412LE: AVPixelFormat = 224;
pub const AV_PIX_FMT_P412BE: AVPixelFormat = 223;
pub const AV_PIX_FMT_P212LE: AVPixelFormat = 222;
pub const AV_PIX_FMT_P212BE: AVPixelFormat = 221;
pub const AV_PIX_FMT_RGBAF32LE: AVPixelFormat = 220;
pub const AV_PIX_FMT_RGBAF32BE: AVPixelFormat = 219;
pub const AV_PIX_FMT_RGBF32LE: AVPixelFormat = 218;
pub const AV_PIX_FMT_RGBF32BE: AVPixelFormat = 217;
pub const AV_PIX_FMT_XV36LE: AVPixelFormat = 216;
pub const AV_PIX_FMT_XV36BE: AVPixelFormat = 215;
pub const AV_PIX_FMT_XV30LE: AVPixelFormat = 214;
pub const AV_PIX_FMT_XV30BE: AVPixelFormat = 213;
pub const AV_PIX_FMT_Y212LE: AVPixelFormat = 212;
pub const AV_PIX_FMT_Y212BE: AVPixelFormat = 211;
pub const AV_PIX_FMT_P012BE: AVPixelFormat = 210;
pub const AV_PIX_FMT_P012LE: AVPixelFormat = 209;
pub const AV_PIX_FMT_VUYX: AVPixelFormat = 208;
pub const AV_PIX_FMT_RGBAF16LE: AVPixelFormat = 207;
pub const AV_PIX_FMT_RGBAF16BE: AVPixelFormat = 206;
pub const AV_PIX_FMT_VUYA: AVPixelFormat = 205;
pub const AV_PIX_FMT_P416LE: AVPixelFormat = 204;
pub const AV_PIX_FMT_P416BE: AVPixelFormat = 203;
pub const AV_PIX_FMT_P216LE: AVPixelFormat = 202;
pub const AV_PIX_FMT_P216BE: AVPixelFormat = 201;
pub const AV_PIX_FMT_P410LE: AVPixelFormat = 200;
pub const AV_PIX_FMT_P410BE: AVPixelFormat = 199;
pub const AV_PIX_FMT_P210LE: AVPixelFormat = 198;
pub const AV_PIX_FMT_P210BE: AVPixelFormat = 197;
pub const AV_PIX_FMT_X2BGR10BE: AVPixelFormat = 196;
pub const AV_PIX_FMT_X2BGR10LE: AVPixelFormat = 195;
pub const AV_PIX_FMT_X2RGB10BE: AVPixelFormat = 194;
pub const AV_PIX_FMT_X2RGB10LE: AVPixelFormat = 193;
pub const AV_PIX_FMT_Y210LE: AVPixelFormat = 192;
pub const AV_PIX_FMT_Y210BE: AVPixelFormat = 191;
pub const AV_PIX_FMT_VULKAN: AVPixelFormat = 190;
pub const AV_PIX_FMT_NV42: AVPixelFormat = 189;
pub const AV_PIX_FMT_NV24: AVPixelFormat = 188;
pub const AV_PIX_FMT_YUVA444P12LE: AVPixelFormat = 187;
pub const AV_PIX_FMT_YUVA444P12BE: AVPixelFormat = 186;
pub const AV_PIX_FMT_YUVA422P12LE: AVPixelFormat = 185;
pub const AV_PIX_FMT_YUVA422P12BE: AVPixelFormat = 184;
pub const AV_PIX_FMT_GRAYF32LE: AVPixelFormat = 183;
pub const AV_PIX_FMT_GRAYF32BE: AVPixelFormat = 182;
pub const AV_PIX_FMT_GRAY14LE: AVPixelFormat = 181;
pub const AV_PIX_FMT_GRAY14BE: AVPixelFormat = 180;
pub const AV_PIX_FMT_OPENCL: AVPixelFormat = 179;
pub const AV_PIX_FMT_DRM_PRIME: AVPixelFormat = 178;
pub const AV_PIX_FMT_GBRAPF32LE: AVPixelFormat = 177;
pub const AV_PIX_FMT_GBRAPF32BE: AVPixelFormat = 176;
pub const AV_PIX_FMT_GBRPF32LE: AVPixelFormat = 175;
pub const AV_PIX_FMT_GBRPF32BE: AVPixelFormat = 174;
pub const AV_PIX_FMT_GRAY9LE: AVPixelFormat = 173;
pub const AV_PIX_FMT_GRAY9BE: AVPixelFormat = 172;
pub const AV_PIX_FMT_D3D11: AVPixelFormat = 171;
pub const AV_PIX_FMT_P016BE: AVPixelFormat = 170;
pub const AV_PIX_FMT_P016LE: AVPixelFormat = 169;
pub const AV_PIX_FMT_GRAY10LE: AVPixelFormat = 168;
pub const AV_PIX_FMT_GRAY10BE: AVPixelFormat = 167;
pub const AV_PIX_FMT_GRAY12LE: AVPixelFormat = 166;
pub const AV_PIX_FMT_GRAY12BE: AVPixelFormat = 165;
pub const AV_PIX_FMT_MEDIACODEC: AVPixelFormat = 164;
pub const AV_PIX_FMT_GBRAP10LE: AVPixelFormat = 163;
pub const AV_PIX_FMT_GBRAP10BE: AVPixelFormat = 162;
pub const AV_PIX_FMT_GBRAP12LE: AVPixelFormat = 161;
pub const AV_PIX_FMT_GBRAP12BE: AVPixelFormat = 160;
pub const AV_PIX_FMT_P010BE: AVPixelFormat = 159;
pub const AV_PIX_FMT_P010LE: AVPixelFormat = 158;
pub const AV_PIX_FMT_VIDEOTOOLBOX: AVPixelFormat = 157;
pub const AV_PIX_FMT_AYUV64BE: AVPixelFormat = 156;
pub const AV_PIX_FMT_AYUV64LE: AVPixelFormat = 155;
pub const AV_PIX_FMT_YUV440P12BE: AVPixelFormat = 154;
pub const AV_PIX_FMT_YUV440P12LE: AVPixelFormat = 153;
pub const AV_PIX_FMT_YUV440P10BE: AVPixelFormat = 152;
pub const AV_PIX_FMT_YUV440P10LE: AVPixelFormat = 151;
pub const AV_PIX_FMT_BAYER_GRBG16BE: AVPixelFormat = 150;
pub const AV_PIX_FMT_BAYER_GRBG16LE: AVPixelFormat = 149;
pub const AV_PIX_FMT_BAYER_GBRG16BE: AVPixelFormat = 148;
pub const AV_PIX_FMT_BAYER_GBRG16LE: AVPixelFormat = 147;
pub const AV_PIX_FMT_BAYER_RGGB16BE: AVPixelFormat = 146;
pub const AV_PIX_FMT_BAYER_RGGB16LE: AVPixelFormat = 145;
pub const AV_PIX_FMT_BAYER_BGGR16BE: AVPixelFormat = 144;
pub const AV_PIX_FMT_BAYER_BGGR16LE: AVPixelFormat = 143;
pub const AV_PIX_FMT_BAYER_GRBG8: AVPixelFormat = 142;
pub const AV_PIX_FMT_BAYER_GBRG8: AVPixelFormat = 141;
pub const AV_PIX_FMT_BAYER_RGGB8: AVPixelFormat = 140;
pub const AV_PIX_FMT_BAYER_BGGR8: AVPixelFormat = 139;
pub const AV_PIX_FMT_YUVJ411P: AVPixelFormat = 138;
pub const AV_PIX_FMT_GBRP14LE: AVPixelFormat = 137;
pub const AV_PIX_FMT_GBRP14BE: AVPixelFormat = 136;
pub const AV_PIX_FMT_GBRP12LE: AVPixelFormat = 135;
pub const AV_PIX_FMT_GBRP12BE: AVPixelFormat = 134;
pub const AV_PIX_FMT_YUV444P14LE: AVPixelFormat = 133;
pub const AV_PIX_FMT_YUV444P14BE: AVPixelFormat = 132;
pub const AV_PIX_FMT_YUV444P12LE: AVPixelFormat = 131;
pub const AV_PIX_FMT_YUV444P12BE: AVPixelFormat = 130;
pub const AV_PIX_FMT_YUV422P14LE: AVPixelFormat = 129;
pub const AV_PIX_FMT_YUV422P14BE: AVPixelFormat = 128;
pub const AV_PIX_FMT_YUV422P12LE: AVPixelFormat = 127;
pub const AV_PIX_FMT_YUV422P12BE: AVPixelFormat = 126;
pub const AV_PIX_FMT_YUV420P14LE: AVPixelFormat = 125;
pub const AV_PIX_FMT_YUV420P14BE: AVPixelFormat = 124;
pub const AV_PIX_FMT_YUV420P12LE: AVPixelFormat = 123;
pub const AV_PIX_FMT_YUV420P12BE: AVPixelFormat = 122;
pub const AV_PIX_FMT_BGR0: AVPixelFormat = 121;
pub const AV_PIX_FMT_0BGR: AVPixelFormat = 120;
pub const AV_PIX_FMT_RGB0: AVPixelFormat = 119;
pub const AV_PIX_FMT_0RGB: AVPixelFormat = 118;
pub const AV_PIX_FMT_CUDA: AVPixelFormat = 117;
pub const AV_PIX_FMT_D3D11VA_VLD: AVPixelFormat = 116;
pub const AV_PIX_FMT_MMAL: AVPixelFormat = 115;
pub const AV_PIX_FMT_QSV: AVPixelFormat = 114;
pub const AV_PIX_FMT_GBRAP16LE: AVPixelFormat = 113;
pub const AV_PIX_FMT_GBRAP16BE: AVPixelFormat = 112;
pub const AV_PIX_FMT_GBRAP: AVPixelFormat = 111;
pub const AV_PIX_FMT_YA16LE: AVPixelFormat = 110;
pub const AV_PIX_FMT_YA16BE: AVPixelFormat = 109;
pub const AV_PIX_FMT_YVYU422: AVPixelFormat = 108;
pub const AV_PIX_FMT_BGRA64LE: AVPixelFormat = 107;
pub const AV_PIX_FMT_BGRA64BE: AVPixelFormat = 106;
pub const AV_PIX_FMT_RGBA64LE: AVPixelFormat = 105;
pub const AV_PIX_FMT_RGBA64BE: AVPixelFormat = 104;
pub const AV_PIX_FMT_NV20BE: AVPixelFormat = 103;
pub const AV_PIX_FMT_NV20LE: AVPixelFormat = 102;
pub const AV_PIX_FMT_NV16: AVPixelFormat = 101;
pub const AV_PIX_FMT_XYZ12BE: AVPixelFormat = 100;
pub const AV_PIX_FMT_XYZ12LE: AVPixelFormat = 99;
pub const AV_PIX_FMT_VDPAU: AVPixelFormat = 98;
pub const AV_PIX_FMT_YUVA444P16LE: AVPixelFormat = 97;
pub const AV_PIX_FMT_YUVA444P16BE: AVPixelFormat = 96;
pub const AV_PIX_FMT_YUVA422P16LE: AVPixelFormat = 95;
pub const AV_PIX_FMT_YUVA422P16BE: AVPixelFormat = 94;
pub const AV_PIX_FMT_YUVA420P16LE: AVPixelFormat = 93;
pub const AV_PIX_FMT_YUVA420P16BE: AVPixelFormat = 92;
pub const AV_PIX_FMT_YUVA444P10LE: AVPixelFormat = 91;
pub const AV_PIX_FMT_YUVA444P10BE: AVPixelFormat = 90;
pub const AV_PIX_FMT_YUVA422P10LE: AVPixelFormat = 89;
pub const AV_PIX_FMT_YUVA422P10BE: AVPixelFormat = 88;
pub const AV_PIX_FMT_YUVA420P10LE: AVPixelFormat = 87;
pub const AV_PIX_FMT_YUVA420P10BE: AVPixelFormat = 86;
pub const AV_PIX_FMT_YUVA444P9LE: AVPixelFormat = 85;
pub const AV_PIX_FMT_YUVA444P9BE: AVPixelFormat = 84;
pub const AV_PIX_FMT_YUVA422P9LE: AVPixelFormat = 83;
pub const AV_PIX_FMT_YUVA422P9BE: AVPixelFormat = 82;
pub const AV_PIX_FMT_YUVA420P9LE: AVPixelFormat = 81;
pub const AV_PIX_FMT_YUVA420P9BE: AVPixelFormat = 80;
pub const AV_PIX_FMT_YUVA444P: AVPixelFormat = 79;
pub const AV_PIX_FMT_YUVA422P: AVPixelFormat = 78;
pub const AV_PIX_FMT_GBRP16LE: AVPixelFormat = 77;
pub const AV_PIX_FMT_GBRP16BE: AVPixelFormat = 76;
pub const AV_PIX_FMT_GBRP10LE: AVPixelFormat = 75;
pub const AV_PIX_FMT_GBRP10BE: AVPixelFormat = 74;
pub const AV_PIX_FMT_GBRP9LE: AVPixelFormat = 73;
pub const AV_PIX_FMT_GBRP9BE: AVPixelFormat = 72;
pub const AV_PIX_FMT_GBR24P: AVPixelFormat = 71;
pub const AV_PIX_FMT_GBRP: AVPixelFormat = 71;
pub const AV_PIX_FMT_YUV422P9LE: AVPixelFormat = 70;
pub const AV_PIX_FMT_YUV422P9BE: AVPixelFormat = 69;
pub const AV_PIX_FMT_YUV444P10LE: AVPixelFormat = 68;
pub const AV_PIX_FMT_YUV444P10BE: AVPixelFormat = 67;
pub const AV_PIX_FMT_YUV444P9LE: AVPixelFormat = 66;
pub const AV_PIX_FMT_YUV444P9BE: AVPixelFormat = 65;
pub const AV_PIX_FMT_YUV422P10LE: AVPixelFormat = 64;
pub const AV_PIX_FMT_YUV422P10BE: AVPixelFormat = 63;
pub const AV_PIX_FMT_YUV420P10LE: AVPixelFormat = 62;
pub const AV_PIX_FMT_YUV420P10BE: AVPixelFormat = 61;
pub const AV_PIX_FMT_YUV420P9LE: AVPixelFormat = 60;
pub const AV_PIX_FMT_YUV420P9BE: AVPixelFormat = 59;
pub const AV_PIX_FMT_BGR48LE: AVPixelFormat = 58;
pub const AV_PIX_FMT_BGR48BE: AVPixelFormat = 57;
pub const AV_PIX_FMT_GRAY8A: AVPixelFormat = 56;
pub const AV_PIX_FMT_Y400A: AVPixelFormat = 56;
pub const AV_PIX_FMT_YA8: AVPixelFormat = 56;
pub const AV_PIX_FMT_BGR444BE: AVPixelFormat = 55;
pub const AV_PIX_FMT_BGR444LE: AVPixelFormat = 54;
pub const AV_PIX_FMT_RGB444BE: AVPixelFormat = 53;
pub const AV_PIX_FMT_RGB444LE: AVPixelFormat = 52;
pub const AV_PIX_FMT_DXVA2_VLD: AVPixelFormat = 51;
pub const AV_PIX_FMT_YUV444P16BE: AVPixelFormat = 50;
pub const AV_PIX_FMT_YUV444P16LE: AVPixelFormat = 49;
pub const AV_PIX_FMT_YUV422P16BE: AVPixelFormat = 48;
pub const AV_PIX_FMT_YUV422P16LE: AVPixelFormat = 47;
pub const AV_PIX_FMT_YUV420P16BE: AVPixelFormat = 46;
pub const AV_PIX_FMT_YUV420P16LE: AVPixelFormat = 45;
pub const AV_PIX_FMT_VAAPI: AVPixelFormat = 44;
pub const AV_PIX_FMT_BGR555LE: AVPixelFormat = 43;
pub const AV_PIX_FMT_BGR555BE: AVPixelFormat = 42;
pub const AV_PIX_FMT_BGR565LE: AVPixelFormat = 41;
pub const AV_PIX_FMT_BGR565BE: AVPixelFormat = 40;
pub const AV_PIX_FMT_RGB555LE: AVPixelFormat = 39;
pub const AV_PIX_FMT_RGB555BE: AVPixelFormat = 38;
pub const AV_PIX_FMT_RGB565LE: AVPixelFormat = 37;
pub const AV_PIX_FMT_RGB565BE: AVPixelFormat = 36;
pub const AV_PIX_FMT_RGB48LE: AVPixelFormat = 35;
pub const AV_PIX_FMT_RGB48BE: AVPixelFormat = 34;
pub const AV_PIX_FMT_YUVA420P: AVPixelFormat = 33;
pub const AV_PIX_FMT_YUVJ440P: AVPixelFormat = 32;
pub const AV_PIX_FMT_YUV440P: AVPixelFormat = 31;
pub const AV_PIX_FMT_GRAY16LE: AVPixelFormat = 30;
pub const AV_PIX_FMT_GRAY16BE: AVPixelFormat = 29;
pub const AV_PIX_FMT_BGRA: AVPixelFormat = 28;
pub const AV_PIX_FMT_ABGR: AVPixelFormat = 27;
pub const AV_PIX_FMT_RGBA: AVPixelFormat = 26;
pub const AV_PIX_FMT_ARGB: AVPixelFormat = 25;
pub const AV_PIX_FMT_NV21: AVPixelFormat = 24;
pub const AV_PIX_FMT_NV12: AVPixelFormat = 23;
pub const AV_PIX_FMT_RGB4_BYTE: AVPixelFormat = 22;
pub const AV_PIX_FMT_RGB4: AVPixelFormat = 21;
pub const AV_PIX_FMT_RGB8: AVPixelFormat = 20;
pub const AV_PIX_FMT_BGR4_BYTE: AVPixelFormat = 19;
pub const AV_PIX_FMT_BGR4: AVPixelFormat = 18;
pub const AV_PIX_FMT_BGR8: AVPixelFormat = 17;
pub const AV_PIX_FMT_UYYVYY411: AVPixelFormat = 16;
pub const AV_PIX_FMT_UYVY422: AVPixelFormat = 15;
pub const AV_PIX_FMT_YUVJ444P: AVPixelFormat = 14;
pub const AV_PIX_FMT_YUVJ422P: AVPixelFormat = 13;
pub const AV_PIX_FMT_YUVJ420P: AVPixelFormat = 12;
pub const AV_PIX_FMT_PAL8: AVPixelFormat = 11;
pub const AV_PIX_FMT_MONOBLACK: AVPixelFormat = 10;
pub const AV_PIX_FMT_MONOWHITE: AVPixelFormat = 9;
pub const AV_PIX_FMT_GRAY8: AVPixelFormat = 8;
pub const AV_PIX_FMT_YUV411P: AVPixelFormat = 7;
pub const AV_PIX_FMT_YUV410P: AVPixelFormat = 6;
pub const AV_PIX_FMT_YUV444P: AVPixelFormat = 5;
pub const AV_PIX_FMT_YUV422P: AVPixelFormat = 4;
pub const AV_PIX_FMT_BGR24: AVPixelFormat = 3;
pub const AV_PIX_FMT_RGB24: AVPixelFormat = 2;
pub const AV_PIX_FMT_YUYV422: AVPixelFormat = 1;
pub const AV_PIX_FMT_YUV420P: AVPixelFormat = 0;
pub const AV_PIX_FMT_NONE: AVPixelFormat = -1;
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
pub type AVSampleFormat = ::core::ffi::c_int;
pub const AV_SAMPLE_FMT_NB: AVSampleFormat = 12;
pub const AV_SAMPLE_FMT_S64P: AVSampleFormat = 11;
pub const AV_SAMPLE_FMT_S64: AVSampleFormat = 10;
pub const AV_SAMPLE_FMT_DBLP: AVSampleFormat = 9;
pub const AV_SAMPLE_FMT_FLTP: AVSampleFormat = 8;
pub const AV_SAMPLE_FMT_S32P: AVSampleFormat = 7;
pub const AV_SAMPLE_FMT_S16P: AVSampleFormat = 6;
pub const AV_SAMPLE_FMT_U8P: AVSampleFormat = 5;
pub const AV_SAMPLE_FMT_DBL: AVSampleFormat = 4;
pub const AV_SAMPLE_FMT_FLT: AVSampleFormat = 3;
pub const AV_SAMPLE_FMT_S32: AVSampleFormat = 2;
pub const AV_SAMPLE_FMT_S16: AVSampleFormat = 1;
pub const AV_SAMPLE_FMT_U8: AVSampleFormat = 0;
pub const AV_SAMPLE_FMT_NONE: AVSampleFormat = -1;
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
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const AV_FRAME_CROP_UNALIGNED: C2RustUnnamed_0 = 1;
pub type AVHWDeviceType = ::core::ffi::c_uint;
pub const AV_HWDEVICE_TYPE_OHCODEC: AVHWDeviceType = 14;
pub const AV_HWDEVICE_TYPE_AMF: AVHWDeviceType = 13;
pub const AV_HWDEVICE_TYPE_D3D12VA: AVHWDeviceType = 12;
pub const AV_HWDEVICE_TYPE_VULKAN: AVHWDeviceType = 11;
pub const AV_HWDEVICE_TYPE_MEDIACODEC: AVHWDeviceType = 10;
pub const AV_HWDEVICE_TYPE_OPENCL: AVHWDeviceType = 9;
pub const AV_HWDEVICE_TYPE_DRM: AVHWDeviceType = 8;
pub const AV_HWDEVICE_TYPE_D3D11VA: AVHWDeviceType = 7;
pub const AV_HWDEVICE_TYPE_VIDEOTOOLBOX: AVHWDeviceType = 6;
pub const AV_HWDEVICE_TYPE_QSV: AVHWDeviceType = 5;
pub const AV_HWDEVICE_TYPE_DXVA2: AVHWDeviceType = 4;
pub const AV_HWDEVICE_TYPE_VAAPI: AVHWDeviceType = 3;
pub const AV_HWDEVICE_TYPE_CUDA: AVHWDeviceType = 2;
pub const AV_HWDEVICE_TYPE_VDPAU: AVHWDeviceType = 1;
pub const AV_HWDEVICE_TYPE_NONE: AVHWDeviceType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVHWDeviceContext {
    pub av_class: *const AVClass,
    pub type_0: AVHWDeviceType,
    pub hwctx: *mut ::core::ffi::c_void,
    pub free: Option<unsafe extern "C" fn(*mut AVHWDeviceContext) -> ()>,
    pub user_opaque: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVHWFramesContext {
    pub av_class: *const AVClass,
    pub device_ref: *mut AVBufferRef,
    pub device_ctx: *mut AVHWDeviceContext,
    pub hwctx: *mut ::core::ffi::c_void,
    pub free: Option<unsafe extern "C" fn(*mut AVHWFramesContext) -> ()>,
    pub user_opaque: *mut ::core::ffi::c_void,
    pub pool: *mut AVBufferPool,
    pub initial_pool_size: ::core::ffi::c_int,
    pub format: AVPixelFormat,
    pub sw_format: AVPixelFormat,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVComponentDescriptor {
    pub plane: ::core::ffi::c_int,
    pub step: ::core::ffi::c_int,
    pub offset: ::core::ffi::c_int,
    pub shift: ::core::ffi::c_int,
    pub depth: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPixFmtDescriptor {
    pub name: *const ::core::ffi::c_char,
    pub nb_components: uint8_t,
    pub log2_chroma_w: uint8_t,
    pub log2_chroma_h: uint8_t,
    pub flags: uint64_t,
    pub comp: [AVComponentDescriptor; 4],
    pub alias: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVMasteringDisplayMetadata {
    pub display_primaries: [[AVRational; 2]; 3],
    pub white_point: [AVRational; 2],
    pub min_luminance: AVRational,
    pub max_luminance: AVRational,
    pub has_primaries: ::core::ffi::c_int,
    pub has_luminance: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVContentLightMetadata {
    pub MaxCLL: ::core::ffi::c_uint,
    pub MaxFALL: ::core::ffi::c_uint,
}
pub type AVStereo3DType = ::core::ffi::c_uint;
pub const AV_STEREO3D_UNSPEC: AVStereo3DType = 8;
pub const AV_STEREO3D_COLUMNS: AVStereo3DType = 7;
pub const AV_STEREO3D_LINES: AVStereo3DType = 6;
pub const AV_STEREO3D_SIDEBYSIDE_QUINCUNX: AVStereo3DType = 5;
pub const AV_STEREO3D_CHECKERBOARD: AVStereo3DType = 4;
pub const AV_STEREO3D_FRAMESEQUENCE: AVStereo3DType = 3;
pub const AV_STEREO3D_TOPBOTTOM: AVStereo3DType = 2;
pub const AV_STEREO3D_SIDEBYSIDE: AVStereo3DType = 1;
pub const AV_STEREO3D_2D: AVStereo3DType = 0;
pub type AVStereo3DView = ::core::ffi::c_uint;
pub const AV_STEREO3D_VIEW_UNSPEC: AVStereo3DView = 3;
pub const AV_STEREO3D_VIEW_RIGHT: AVStereo3DView = 2;
pub const AV_STEREO3D_VIEW_LEFT: AVStereo3DView = 1;
pub const AV_STEREO3D_VIEW_PACKED: AVStereo3DView = 0;
pub type AVStereo3DPrimaryEye = ::core::ffi::c_uint;
pub const AV_PRIMARY_EYE_RIGHT: AVStereo3DPrimaryEye = 2;
pub const AV_PRIMARY_EYE_LEFT: AVStereo3DPrimaryEye = 1;
pub const AV_PRIMARY_EYE_NONE: AVStereo3DPrimaryEye = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVStereo3D {
    pub type_0: AVStereo3DType,
    pub flags: ::core::ffi::c_int,
    pub view: AVStereo3DView,
    pub primary_eye: AVStereo3DPrimaryEye,
    pub baseline: uint32_t,
    pub horizontal_disparity_adjustment: AVRational,
    pub horizontal_field_of_view: AVRational,
}
pub type AVCodecID = ::core::ffi::c_uint;
pub const AV_CODEC_ID_ANULL: AVCodecID = 135171;
pub const AV_CODEC_ID_VNULL: AVCodecID = 135170;
pub const AV_CODEC_ID_WRAPPED_AVFRAME: AVCodecID = 135169;
pub const AV_CODEC_ID_FFMETADATA: AVCodecID = 135168;
pub const AV_CODEC_ID_MPEG4SYSTEMS: AVCodecID = 131073;
pub const AV_CODEC_ID_MPEG2TS: AVCodecID = 131072;
pub const AV_CODEC_ID_PROBE: AVCodecID = 102400;
pub const AV_CODEC_ID_SMPTE_436M_ANC: AVCodecID = 98317;
pub const AV_CODEC_ID_LCEVC: AVCodecID = 98316;
pub const AV_CODEC_ID_SMPTE_2038: AVCodecID = 98315;
pub const AV_CODEC_ID_BIN_DATA: AVCodecID = 98314;
pub const AV_CODEC_ID_TIMED_ID3: AVCodecID = 98313;
pub const AV_CODEC_ID_DVD_NAV: AVCodecID = 98312;
pub const AV_CODEC_ID_SMPTE_KLV: AVCodecID = 98311;
pub const AV_CODEC_ID_OTF: AVCodecID = 98310;
pub const AV_CODEC_ID_IDF: AVCodecID = 98309;
pub const AV_CODEC_ID_XBIN: AVCodecID = 98308;
pub const AV_CODEC_ID_BINTEXT: AVCodecID = 98307;
pub const AV_CODEC_ID_EPG: AVCodecID = 98306;
pub const AV_CODEC_ID_SCTE_35: AVCodecID = 98305;
pub const AV_CODEC_ID_TTF: AVCodecID = 98304;
pub const AV_CODEC_ID_FIRST_UNKNOWN: AVCodecID = 98304;
pub const AV_CODEC_ID_IVTV_VBI: AVCodecID = 94234;
pub const AV_CODEC_ID_ARIB_CAPTION: AVCodecID = 94233;
pub const AV_CODEC_ID_TTML: AVCodecID = 94232;
pub const AV_CODEC_ID_HDMV_TEXT_SUBTITLE: AVCodecID = 94231;
pub const AV_CODEC_ID_ASS: AVCodecID = 94230;
pub const AV_CODEC_ID_PJS: AVCodecID = 94229;
pub const AV_CODEC_ID_VPLAYER: AVCodecID = 94228;
pub const AV_CODEC_ID_MPL2: AVCodecID = 94227;
pub const AV_CODEC_ID_WEBVTT: AVCodecID = 94226;
pub const AV_CODEC_ID_SUBRIP: AVCodecID = 94225;
pub const AV_CODEC_ID_SUBVIEWER: AVCodecID = 94224;
pub const AV_CODEC_ID_SUBVIEWER1: AVCodecID = 94223;
pub const AV_CODEC_ID_STL: AVCodecID = 94222;
pub const AV_CODEC_ID_REALTEXT: AVCodecID = 94221;
pub const AV_CODEC_ID_SAMI: AVCodecID = 94220;
pub const AV_CODEC_ID_JACOSUB: AVCodecID = 94219;
pub const AV_CODEC_ID_EIA_608: AVCodecID = 94218;
pub const AV_CODEC_ID_MICRODVD: AVCodecID = 94217;
pub const AV_CODEC_ID_SRT: AVCodecID = 94216;
pub const AV_CODEC_ID_DVB_TELETEXT: AVCodecID = 94215;
pub const AV_CODEC_ID_HDMV_PGS_SUBTITLE: AVCodecID = 94214;
pub const AV_CODEC_ID_MOV_TEXT: AVCodecID = 94213;
pub const AV_CODEC_ID_SSA: AVCodecID = 94212;
pub const AV_CODEC_ID_XSUB: AVCodecID = 94211;
pub const AV_CODEC_ID_TEXT: AVCodecID = 94210;
pub const AV_CODEC_ID_DVB_SUBTITLE: AVCodecID = 94209;
pub const AV_CODEC_ID_DVD_SUBTITLE: AVCodecID = 94208;
pub const AV_CODEC_ID_FIRST_SUBTITLE: AVCodecID = 94208;
pub const AV_CODEC_ID_APPLE_APAC: AVCodecID = 86125;
pub const AV_CODEC_ID_AHX: AVCodecID = 86124;
pub const AV_CODEC_ID_G728: AVCodecID = 86123;
pub const AV_CODEC_ID_LC3: AVCodecID = 86122;
pub const AV_CODEC_ID_QOA: AVCodecID = 86121;
pub const AV_CODEC_ID_OSQ: AVCodecID = 86120;
pub const AV_CODEC_ID_AC4: AVCodecID = 86119;
pub const AV_CODEC_ID_RKA: AVCodecID = 86118;
pub const AV_CODEC_ID_WAVARC: AVCodecID = 86117;
pub const AV_CODEC_ID_FTR: AVCodecID = 86116;
pub const AV_CODEC_ID_APAC: AVCodecID = 86115;
pub const AV_CODEC_ID_MISC4: AVCodecID = 86114;
pub const AV_CODEC_ID_BONK: AVCodecID = 86113;
pub const AV_CODEC_ID_DFPWM: AVCodecID = 86112;
pub const AV_CODEC_ID_MSNSIREN: AVCodecID = 86111;
pub const AV_CODEC_ID_FASTAUDIO: AVCodecID = 86110;
pub const AV_CODEC_ID_HCA: AVCodecID = 86109;
pub const AV_CODEC_ID_SIREN: AVCodecID = 86108;
pub const AV_CODEC_ID_MPEGH_3D_AUDIO: AVCodecID = 86107;
pub const AV_CODEC_ID_ACELP_KELVIN: AVCodecID = 86106;
pub const AV_CODEC_ID_HCOM: AVCodecID = 86105;
pub const AV_CODEC_ID_ATRAC9: AVCodecID = 86104;
pub const AV_CODEC_ID_SBC: AVCodecID = 86103;
pub const AV_CODEC_ID_APTX_HD: AVCodecID = 86102;
pub const AV_CODEC_ID_APTX: AVCodecID = 86101;
pub const AV_CODEC_ID_DOLBY_E: AVCodecID = 86100;
pub const AV_CODEC_ID_ATRAC3PAL: AVCodecID = 86099;
pub const AV_CODEC_ID_ATRAC3AL: AVCodecID = 86098;
pub const AV_CODEC_ID_DST: AVCodecID = 86097;
pub const AV_CODEC_ID_XMA2: AVCodecID = 86096;
pub const AV_CODEC_ID_XMA1: AVCodecID = 86095;
pub const AV_CODEC_ID_INTERPLAY_ACM: AVCodecID = 86094;
pub const AV_CODEC_ID_4GV: AVCodecID = 86093;
pub const AV_CODEC_ID_DSD_MSBF_PLANAR: AVCodecID = 86092;
pub const AV_CODEC_ID_DSD_LSBF_PLANAR: AVCodecID = 86091;
pub const AV_CODEC_ID_DSD_MSBF: AVCodecID = 86090;
pub const AV_CODEC_ID_DSD_LSBF: AVCodecID = 86089;
pub const AV_CODEC_ID_SMV: AVCodecID = 86088;
pub const AV_CODEC_ID_EVRC: AVCodecID = 86087;
pub const AV_CODEC_ID_SONIC_LS: AVCodecID = 86086;
pub const AV_CODEC_ID_SONIC: AVCodecID = 86085;
pub const AV_CODEC_ID_FFWAVESYNTH: AVCodecID = 86084;
pub const AV_CODEC_ID_CODEC2: AVCodecID = 86083;
pub const AV_CODEC_ID_DSS_SP: AVCodecID = 86082;
pub const AV_CODEC_ID_ON2AVC: AVCodecID = 86081;
pub const AV_CODEC_ID_PAF_AUDIO: AVCodecID = 86080;
pub const AV_CODEC_ID_METASOUND: AVCodecID = 86079;
pub const AV_CODEC_ID_TAK: AVCodecID = 86078;
pub const AV_CODEC_ID_COMFORT_NOISE: AVCodecID = 86077;
pub const AV_CODEC_ID_OPUS: AVCodecID = 86076;
pub const AV_CODEC_ID_ILBC: AVCodecID = 86075;
pub const AV_CODEC_ID_IAC: AVCodecID = 86074;
pub const AV_CODEC_ID_RALF: AVCodecID = 86073;
pub const AV_CODEC_ID_BMV_AUDIO: AVCodecID = 86072;
pub const AV_CODEC_ID_8SVX_FIB: AVCodecID = 86071;
pub const AV_CODEC_ID_8SVX_EXP: AVCodecID = 86070;
pub const AV_CODEC_ID_G729: AVCodecID = 86069;
pub const AV_CODEC_ID_G723_1: AVCodecID = 86068;
pub const AV_CODEC_ID_CELT: AVCodecID = 86067;
pub const AV_CODEC_ID_QDMC: AVCodecID = 86066;
pub const AV_CODEC_ID_AAC_LATM: AVCodecID = 86065;
pub const AV_CODEC_ID_BINKAUDIO_DCT: AVCodecID = 86064;
pub const AV_CODEC_ID_BINKAUDIO_RDFT: AVCodecID = 86063;
pub const AV_CODEC_ID_ATRAC1: AVCodecID = 86062;
pub const AV_CODEC_ID_MP4ALS: AVCodecID = 86061;
pub const AV_CODEC_ID_TRUEHD: AVCodecID = 86060;
pub const AV_CODEC_ID_TWINVQ: AVCodecID = 86059;
pub const AV_CODEC_ID_MP1: AVCodecID = 86058;
pub const AV_CODEC_ID_SIPR: AVCodecID = 86057;
pub const AV_CODEC_ID_EAC3: AVCodecID = 86056;
pub const AV_CODEC_ID_ATRAC3P: AVCodecID = 86055;
pub const AV_CODEC_ID_WMALOSSLESS: AVCodecID = 86054;
pub const AV_CODEC_ID_WMAPRO: AVCodecID = 86053;
pub const AV_CODEC_ID_WMAVOICE: AVCodecID = 86052;
pub const AV_CODEC_ID_SPEEX: AVCodecID = 86051;
pub const AV_CODEC_ID_MUSEPACK8: AVCodecID = 86050;
pub const AV_CODEC_ID_NELLYMOSER: AVCodecID = 86049;
pub const AV_CODEC_ID_APE: AVCodecID = 86048;
pub const AV_CODEC_ID_ATRAC3: AVCodecID = 86047;
pub const AV_CODEC_ID_GSM_MS: AVCodecID = 86046;
pub const AV_CODEC_ID_MLP: AVCodecID = 86045;
pub const AV_CODEC_ID_MUSEPACK7: AVCodecID = 86044;
pub const AV_CODEC_ID_IMC: AVCodecID = 86043;
pub const AV_CODEC_ID_DSICINAUDIO: AVCodecID = 86042;
pub const AV_CODEC_ID_WAVPACK: AVCodecID = 86041;
pub const AV_CODEC_ID_QCELP: AVCodecID = 86040;
pub const AV_CODEC_ID_SMACKAUDIO: AVCodecID = 86039;
pub const AV_CODEC_ID_TTA: AVCodecID = 86038;
pub const AV_CODEC_ID_TRUESPEECH: AVCodecID = 86037;
pub const AV_CODEC_ID_COOK: AVCodecID = 86036;
pub const AV_CODEC_ID_QDM2: AVCodecID = 86035;
pub const AV_CODEC_ID_GSM: AVCodecID = 86034;
pub const AV_CODEC_ID_WESTWOOD_SND1: AVCodecID = 86033;
pub const AV_CODEC_ID_ALAC: AVCodecID = 86032;
pub const AV_CODEC_ID_SHORTEN: AVCodecID = 86031;
pub const AV_CODEC_ID_MP3ON4: AVCodecID = 86030;
pub const AV_CODEC_ID_MP3ADU: AVCodecID = 86029;
pub const AV_CODEC_ID_FLAC: AVCodecID = 86028;
pub const AV_CODEC_ID_VMDAUDIO: AVCodecID = 86027;
pub const AV_CODEC_ID_MACE6: AVCodecID = 86026;
pub const AV_CODEC_ID_MACE3: AVCodecID = 86025;
pub const AV_CODEC_ID_WMAV2: AVCodecID = 86024;
pub const AV_CODEC_ID_WMAV1: AVCodecID = 86023;
pub const AV_CODEC_ID_DVAUDIO: AVCodecID = 86022;
pub const AV_CODEC_ID_VORBIS: AVCodecID = 86021;
pub const AV_CODEC_ID_DTS: AVCodecID = 86020;
pub const AV_CODEC_ID_AC3: AVCodecID = 86019;
pub const AV_CODEC_ID_AAC: AVCodecID = 86018;
pub const AV_CODEC_ID_MP3: AVCodecID = 86017;
pub const AV_CODEC_ID_MP2: AVCodecID = 86016;
pub const AV_CODEC_ID_CBD2_DPCM: AVCodecID = 81928;
pub const AV_CODEC_ID_WADY_DPCM: AVCodecID = 81927;
pub const AV_CODEC_ID_DERF_DPCM: AVCodecID = 81926;
pub const AV_CODEC_ID_GREMLIN_DPCM: AVCodecID = 81925;
pub const AV_CODEC_ID_SDX2_DPCM: AVCodecID = 81924;
pub const AV_CODEC_ID_SOL_DPCM: AVCodecID = 81923;
pub const AV_CODEC_ID_XAN_DPCM: AVCodecID = 81922;
pub const AV_CODEC_ID_INTERPLAY_DPCM: AVCodecID = 81921;
pub const AV_CODEC_ID_ROQ_DPCM: AVCodecID = 81920;
pub const AV_CODEC_ID_RA_288: AVCodecID = 77825;
pub const AV_CODEC_ID_RA_144: AVCodecID = 77824;
pub const AV_CODEC_ID_AMR_WB: AVCodecID = 73729;
pub const AV_CODEC_ID_AMR_NB: AVCodecID = 73728;
pub const AV_CODEC_ID_ADPCM_IMA_ESCAPE: AVCodecID = 69693;
pub const AV_CODEC_ID_ADPCM_CIRCUS: AVCodecID = 69692;
pub const AV_CODEC_ID_ADPCM_PSXC: AVCodecID = 69691;
pub const AV_CODEC_ID_ADPCM_IMA_MAGIX: AVCodecID = 69690;
pub const AV_CODEC_ID_ADPCM_IMA_HVQM2: AVCodecID = 69689;
pub const AV_CODEC_ID_ADPCM_N64: AVCodecID = 69688;
pub const AV_CODEC_ID_ADPCM_IMA_PDA: AVCodecID = 69687;
pub const AV_CODEC_ID_ADPCM_IMA_HVQM4: AVCodecID = 69686;
pub const AV_CODEC_ID_ADPCM_SANYO: AVCodecID = 69685;
pub const AV_CODEC_ID_ADPCM_IMA_XBOX: AVCodecID = 69684;
pub const AV_CODEC_ID_ADPCM_XMD: AVCodecID = 69683;
pub const AV_CODEC_ID_ADPCM_IMA_ACORN: AVCodecID = 69682;
pub const AV_CODEC_ID_ADPCM_IMA_MOFLEX: AVCodecID = 69681;
pub const AV_CODEC_ID_ADPCM_IMA_CUNNING: AVCodecID = 69680;
pub const AV_CODEC_ID_ADPCM_IMA_MTF: AVCodecID = 69679;
pub const AV_CODEC_ID_ADPCM_IMA_ALP: AVCodecID = 69678;
pub const AV_CODEC_ID_ADPCM_IMA_APM: AVCodecID = 69677;
pub const AV_CODEC_ID_ADPCM_ZORK: AVCodecID = 69676;
pub const AV_CODEC_ID_ADPCM_IMA_SSI: AVCodecID = 69675;
pub const AV_CODEC_ID_ADPCM_ARGO: AVCodecID = 69674;
pub const AV_CODEC_ID_ADPCM_AGM: AVCodecID = 69673;
pub const AV_CODEC_ID_ADPCM_MTAF: AVCodecID = 69672;
pub const AV_CODEC_ID_ADPCM_IMA_DAT4: AVCodecID = 69671;
pub const AV_CODEC_ID_ADPCM_AICA: AVCodecID = 69670;
pub const AV_CODEC_ID_ADPCM_PSX: AVCodecID = 69669;
pub const AV_CODEC_ID_ADPCM_THP_LE: AVCodecID = 69668;
pub const AV_CODEC_ID_ADPCM_G726LE: AVCodecID = 69667;
pub const AV_CODEC_ID_ADPCM_IMA_RAD: AVCodecID = 69666;
pub const AV_CODEC_ID_ADPCM_DTK: AVCodecID = 69665;
pub const AV_CODEC_ID_ADPCM_IMA_OKI: AVCodecID = 69664;
pub const AV_CODEC_ID_ADPCM_AFC: AVCodecID = 69663;
pub const AV_CODEC_ID_ADPCM_VIMA: AVCodecID = 69662;
pub const AV_CODEC_ID_ADPCM_IMA_APC: AVCodecID = 69661;
pub const AV_CODEC_ID_ADPCM_G722: AVCodecID = 69660;
pub const AV_CODEC_ID_ADPCM_IMA_ISS: AVCodecID = 69659;
pub const AV_CODEC_ID_ADPCM_EA_MAXIS_XA: AVCodecID = 69658;
pub const AV_CODEC_ID_ADPCM_EA_XAS: AVCodecID = 69657;
pub const AV_CODEC_ID_ADPCM_IMA_EA_EACS: AVCodecID = 69656;
pub const AV_CODEC_ID_ADPCM_IMA_EA_SEAD: AVCodecID = 69655;
pub const AV_CODEC_ID_ADPCM_EA_R2: AVCodecID = 69654;
pub const AV_CODEC_ID_ADPCM_EA_R3: AVCodecID = 69653;
pub const AV_CODEC_ID_ADPCM_EA_R1: AVCodecID = 69652;
pub const AV_CODEC_ID_ADPCM_IMA_AMV: AVCodecID = 69651;
pub const AV_CODEC_ID_ADPCM_THP: AVCodecID = 69650;
pub const AV_CODEC_ID_ADPCM_SBPRO_2: AVCodecID = 69649;
pub const AV_CODEC_ID_ADPCM_SBPRO_3: AVCodecID = 69648;
pub const AV_CODEC_ID_ADPCM_SBPRO_4: AVCodecID = 69647;
pub const AV_CODEC_ID_ADPCM_YAMAHA: AVCodecID = 69646;
pub const AV_CODEC_ID_ADPCM_SWF: AVCodecID = 69645;
pub const AV_CODEC_ID_ADPCM_CT: AVCodecID = 69644;
pub const AV_CODEC_ID_ADPCM_G726: AVCodecID = 69643;
pub const AV_CODEC_ID_ADPCM_EA: AVCodecID = 69642;
pub const AV_CODEC_ID_ADPCM_ADX: AVCodecID = 69641;
pub const AV_CODEC_ID_ADPCM_XA: AVCodecID = 69640;
pub const AV_CODEC_ID_ADPCM_4XM: AVCodecID = 69639;
pub const AV_CODEC_ID_ADPCM_MS: AVCodecID = 69638;
pub const AV_CODEC_ID_ADPCM_IMA_SMJPEG: AVCodecID = 69637;
pub const AV_CODEC_ID_ADPCM_IMA_WS: AVCodecID = 69636;
pub const AV_CODEC_ID_ADPCM_IMA_DK4: AVCodecID = 69635;
pub const AV_CODEC_ID_ADPCM_IMA_DK3: AVCodecID = 69634;
pub const AV_CODEC_ID_ADPCM_IMA_WAV: AVCodecID = 69633;
pub const AV_CODEC_ID_ADPCM_IMA_QT: AVCodecID = 69632;
pub const AV_CODEC_ID_PCM_SGA: AVCodecID = 65572;
pub const AV_CODEC_ID_PCM_VIDC: AVCodecID = 65571;
pub const AV_CODEC_ID_PCM_F24LE: AVCodecID = 65570;
pub const AV_CODEC_ID_PCM_F16LE: AVCodecID = 65569;
pub const AV_CODEC_ID_PCM_S64BE: AVCodecID = 65568;
pub const AV_CODEC_ID_PCM_S64LE: AVCodecID = 65567;
pub const AV_CODEC_ID_PCM_S16BE_PLANAR: AVCodecID = 65566;
pub const AV_CODEC_ID_PCM_S32LE_PLANAR: AVCodecID = 65565;
pub const AV_CODEC_ID_PCM_S24LE_PLANAR: AVCodecID = 65564;
pub const AV_CODEC_ID_PCM_S8_PLANAR: AVCodecID = 65563;
pub const AV_CODEC_ID_S302M: AVCodecID = 65562;
pub const AV_CODEC_ID_PCM_LXF: AVCodecID = 65561;
pub const AV_CODEC_ID_PCM_BLURAY: AVCodecID = 65560;
pub const AV_CODEC_ID_PCM_F64LE: AVCodecID = 65559;
pub const AV_CODEC_ID_PCM_F64BE: AVCodecID = 65558;
pub const AV_CODEC_ID_PCM_F32LE: AVCodecID = 65557;
pub const AV_CODEC_ID_PCM_F32BE: AVCodecID = 65556;
pub const AV_CODEC_ID_PCM_DVD: AVCodecID = 65555;
pub const AV_CODEC_ID_PCM_S16LE_PLANAR: AVCodecID = 65554;
pub const AV_CODEC_ID_PCM_ZORK: AVCodecID = 65553;
pub const AV_CODEC_ID_PCM_S24DAUD: AVCodecID = 65552;
pub const AV_CODEC_ID_PCM_U24BE: AVCodecID = 65551;
pub const AV_CODEC_ID_PCM_U24LE: AVCodecID = 65550;
pub const AV_CODEC_ID_PCM_S24BE: AVCodecID = 65549;
pub const AV_CODEC_ID_PCM_S24LE: AVCodecID = 65548;
pub const AV_CODEC_ID_PCM_U32BE: AVCodecID = 65547;
pub const AV_CODEC_ID_PCM_U32LE: AVCodecID = 65546;
pub const AV_CODEC_ID_PCM_S32BE: AVCodecID = 65545;
pub const AV_CODEC_ID_PCM_S32LE: AVCodecID = 65544;
pub const AV_CODEC_ID_PCM_ALAW: AVCodecID = 65543;
pub const AV_CODEC_ID_PCM_MULAW: AVCodecID = 65542;
pub const AV_CODEC_ID_PCM_U8: AVCodecID = 65541;
pub const AV_CODEC_ID_PCM_S8: AVCodecID = 65540;
pub const AV_CODEC_ID_PCM_U16BE: AVCodecID = 65539;
pub const AV_CODEC_ID_PCM_U16LE: AVCodecID = 65538;
pub const AV_CODEC_ID_PCM_S16BE: AVCodecID = 65537;
pub const AV_CODEC_ID_PCM_S16LE: AVCodecID = 65536;
pub const AV_CODEC_ID_FIRST_AUDIO: AVCodecID = 65536;
pub const AV_CODEC_ID_WEBP_ANIM: AVCodecID = 273;
pub const AV_CODEC_ID_JPEGXS: AVCodecID = 272;
pub const AV_CODEC_ID_PRORES_RAW: AVCodecID = 271;
pub const AV_CODEC_ID_APV: AVCodecID = 270;
pub const AV_CODEC_ID_JPEGXL_ANIM: AVCodecID = 269;
pub const AV_CODEC_ID_RV60: AVCodecID = 268;
pub const AV_CODEC_ID_DNXUC: AVCodecID = 267;
pub const AV_CODEC_ID_LEAD: AVCodecID = 266;
pub const AV_CODEC_ID_VMIX: AVCodecID = 265;
pub const AV_CODEC_ID_RTV1: AVCodecID = 264;
pub const AV_CODEC_ID_EVC: AVCodecID = 263;
pub const AV_CODEC_ID_PDV: AVCodecID = 262;
pub const AV_CODEC_ID_VQC: AVCodecID = 261;
pub const AV_CODEC_ID_MEDIA100: AVCodecID = 260;
pub const AV_CODEC_ID_WBMP: AVCodecID = 259;
pub const AV_CODEC_ID_RADIANCE_HDR: AVCodecID = 258;
pub const AV_CODEC_ID_PHM: AVCodecID = 257;
pub const AV_CODEC_ID_QOI: AVCodecID = 256;
pub const AV_CODEC_ID_JPEGXL: AVCodecID = 255;
pub const AV_CODEC_ID_VBN: AVCodecID = 254;
pub const AV_CODEC_ID_GEM: AVCodecID = 253;
pub const AV_CODEC_ID_SGA_VIDEO: AVCodecID = 252;
pub const AV_CODEC_ID_SIMBIOSIS_IMX: AVCodecID = 251;
pub const AV_CODEC_ID_CRI: AVCodecID = 250;
pub const AV_CODEC_ID_ARGO: AVCodecID = 249;
pub const AV_CODEC_ID_IPU: AVCodecID = 248;
pub const AV_CODEC_ID_PHOTOCD: AVCodecID = 247;
pub const AV_CODEC_ID_MOBICLIP: AVCodecID = 246;
pub const AV_CODEC_ID_PFM: AVCodecID = 245;
pub const AV_CODEC_ID_NOTCHLC: AVCodecID = 244;
pub const AV_CODEC_ID_MV30: AVCodecID = 243;
pub const AV_CODEC_ID_CDTOONS: AVCodecID = 242;
pub const AV_CODEC_ID_MVHA: AVCodecID = 241;
pub const AV_CODEC_ID_MVDV: AVCodecID = 240;
pub const AV_CODEC_ID_IMM5: AVCodecID = 239;
pub const AV_CODEC_ID_VP4: AVCodecID = 238;
pub const AV_CODEC_ID_LSCR: AVCodecID = 237;
pub const AV_CODEC_ID_AGM: AVCodecID = 236;
pub const AV_CODEC_ID_ARBC: AVCodecID = 235;
pub const AV_CODEC_ID_HYMT: AVCodecID = 234;
pub const AV_CODEC_ID_RASC: AVCodecID = 233;
pub const AV_CODEC_ID_WCMV: AVCodecID = 232;
pub const AV_CODEC_ID_MWSC: AVCodecID = 231;
pub const AV_CODEC_ID_PROSUMER: AVCodecID = 230;
pub const AV_CODEC_ID_IMM4: AVCodecID = 229;
pub const AV_CODEC_ID_FITS: AVCodecID = 228;
pub const AV_CODEC_ID_GDV: AVCodecID = 227;
pub const AV_CODEC_ID_SVG: AVCodecID = 226;
pub const AV_CODEC_ID_SRGC: AVCodecID = 225;
pub const AV_CODEC_ID_MSCC: AVCodecID = 224;
pub const AV_CODEC_ID_BITPACKED: AVCodecID = 223;
pub const AV_CODEC_ID_AV1: AVCodecID = 222;
pub const AV_CODEC_ID_XPM: AVCodecID = 221;
pub const AV_CODEC_ID_CLEARVIDEO: AVCodecID = 220;
pub const AV_CODEC_ID_SCPR: AVCodecID = 219;
pub const AV_CODEC_ID_FMVC: AVCodecID = 218;
pub const AV_CODEC_ID_SPEEDHQ: AVCodecID = 217;
pub const AV_CODEC_ID_PIXLET: AVCodecID = 216;
pub const AV_CODEC_ID_PSD: AVCodecID = 215;
pub const AV_CODEC_ID_YLC: AVCodecID = 214;
pub const AV_CODEC_ID_SHEERVIDEO: AVCodecID = 213;
pub const AV_CODEC_ID_MAGICYUV: AVCodecID = 212;
pub const AV_CODEC_ID_M101: AVCodecID = 211;
pub const AV_CODEC_ID_TRUEMOTION2RT: AVCodecID = 210;
pub const AV_CODEC_ID_CFHD: AVCodecID = 209;
pub const AV_CODEC_ID_DAALA: AVCodecID = 208;
pub const AV_CODEC_ID_APNG: AVCodecID = 207;
pub const AV_CODEC_ID_SMVJPEG: AVCodecID = 206;
pub const AV_CODEC_ID_SNOW: AVCodecID = 205;
pub const AV_CODEC_ID_XFACE: AVCodecID = 204;
pub const AV_CODEC_ID_CPIA: AVCodecID = 203;
pub const AV_CODEC_ID_AVRN: AVCodecID = 202;
pub const AV_CODEC_ID_YUV4: AVCodecID = 201;
pub const AV_CODEC_ID_TARGA_Y216: AVCodecID = 200;
pub const AV_CODEC_ID_AVUI: AVCodecID = 199;
pub const AV_CODEC_ID_012V: AVCodecID = 198;
pub const AV_CODEC_ID_AVRP: AVCodecID = 197;
pub const AV_CODEC_ID_Y41P: AVCodecID = 196;
pub const AV_CODEC_ID_VVC: AVCodecID = 195;
pub const AV_CODEC_ID_MSP2: AVCodecID = 194;
pub const AV_CODEC_ID_AVS3: AVCodecID = 193;
pub const AV_CODEC_ID_PGX: AVCodecID = 192;
pub const AV_CODEC_ID_AVS2: AVCodecID = 191;
pub const AV_CODEC_ID_RSCC: AVCodecID = 190;
pub const AV_CODEC_ID_SCREENPRESSO: AVCodecID = 189;
pub const AV_CODEC_ID_DXV: AVCodecID = 188;
pub const AV_CODEC_ID_DDS: AVCodecID = 187;
pub const AV_CODEC_ID_HAP: AVCodecID = 186;
pub const AV_CODEC_ID_HQ_HQA: AVCodecID = 185;
pub const AV_CODEC_ID_TDSC: AVCodecID = 184;
pub const AV_CODEC_ID_HQX: AVCodecID = 183;
pub const AV_CODEC_ID_MVC2: AVCodecID = 182;
pub const AV_CODEC_ID_MVC1: AVCodecID = 181;
pub const AV_CODEC_ID_SGIRLE: AVCodecID = 180;
pub const AV_CODEC_ID_SANM: AVCodecID = 179;
pub const AV_CODEC_ID_VP7: AVCodecID = 178;
pub const AV_CODEC_ID_EXR: AVCodecID = 177;
pub const AV_CODEC_ID_PAF_VIDEO: AVCodecID = 176;
pub const AV_CODEC_ID_BRENDER_PIX: AVCodecID = 175;
pub const AV_CODEC_ID_ALIAS_PIX: AVCodecID = 174;
pub const AV_CODEC_ID_FIC: AVCodecID = 173;
pub const AV_CODEC_ID_HEVC: AVCodecID = 172;
pub const AV_CODEC_ID_HNM4_VIDEO: AVCodecID = 171;
pub const AV_CODEC_ID_WEBP: AVCodecID = 170;
pub const AV_CODEC_ID_G2M: AVCodecID = 169;
pub const AV_CODEC_ID_ESCAPE130: AVCodecID = 168;
pub const AV_CODEC_ID_AIC: AVCodecID = 167;
pub const AV_CODEC_ID_VP9: AVCodecID = 166;
pub const AV_CODEC_ID_MSS2: AVCodecID = 165;
pub const AV_CODEC_ID_CLLC: AVCodecID = 164;
pub const AV_CODEC_ID_MTS2: AVCodecID = 163;
pub const AV_CODEC_ID_TSCC2: AVCodecID = 162;
pub const AV_CODEC_ID_MSA1: AVCodecID = 161;
pub const AV_CODEC_ID_MSS1: AVCodecID = 160;
pub const AV_CODEC_ID_ZEROCODEC: AVCodecID = 159;
pub const AV_CODEC_ID_XBM: AVCodecID = 158;
pub const AV_CODEC_ID_CDXL: AVCodecID = 157;
pub const AV_CODEC_ID_XWD: AVCodecID = 156;
pub const AV_CODEC_ID_DXTORY: AVCodecID = 155;
pub const AV_CODEC_ID_VBLE: AVCodecID = 154;
pub const AV_CODEC_ID_BMV_VIDEO: AVCodecID = 153;
pub const AV_CODEC_ID_UTVIDEO: AVCodecID = 152;
pub const AV_CODEC_ID_VC1IMAGE: AVCodecID = 151;
pub const AV_CODEC_ID_WMV3IMAGE: AVCodecID = 150;
pub const AV_CODEC_ID_DFA: AVCodecID = 149;
pub const AV_CODEC_ID_JV: AVCodecID = 148;
pub const AV_CODEC_ID_PRORES: AVCodecID = 147;
pub const AV_CODEC_ID_LAGARITH: AVCodecID = 146;
pub const AV_CODEC_ID_MXPEG: AVCodecID = 145;
pub const AV_CODEC_ID_R10K: AVCodecID = 144;
pub const AV_CODEC_ID_A64_MULTI5: AVCodecID = 143;
pub const AV_CODEC_ID_A64_MULTI: AVCodecID = 142;
pub const AV_CODEC_ID_ANSI: AVCodecID = 141;
pub const AV_CODEC_ID_PICTOR: AVCodecID = 140;
pub const AV_CODEC_ID_VP8: AVCodecID = 139;
pub const AV_CODEC_ID_YOP: AVCodecID = 138;
pub const AV_CODEC_ID_KGV1: AVCodecID = 137;
pub const AV_CODEC_ID_IFF_ILBM: AVCodecID = 136;
pub const AV_CODEC_ID_BINKVIDEO: AVCodecID = 135;
pub const AV_CODEC_ID_ANM: AVCodecID = 134;
pub const AV_CODEC_ID_R210: AVCodecID = 133;
pub const AV_CODEC_ID_CDGRAPHICS: AVCodecID = 132;
pub const AV_CODEC_ID_FLASHSV2: AVCodecID = 131;
pub const AV_CODEC_ID_FRWU: AVCodecID = 130;
pub const AV_CODEC_ID_MAD: AVCodecID = 129;
pub const AV_CODEC_ID_DPX: AVCodecID = 128;
pub const AV_CODEC_ID_V210: AVCodecID = 127;
pub const AV_CODEC_ID_TMV: AVCodecID = 126;
pub const AV_CODEC_ID_V210X: AVCodecID = 125;
pub const AV_CODEC_ID_AURA2: AVCodecID = 124;
pub const AV_CODEC_ID_AURA: AVCodecID = 123;
pub const AV_CODEC_ID_TQI: AVCodecID = 122;
pub const AV_CODEC_ID_TGQ: AVCodecID = 121;
pub const AV_CODEC_ID_TGV: AVCodecID = 120;
pub const AV_CODEC_ID_MOTIONPIXELS: AVCodecID = 119;
pub const AV_CODEC_ID_CMV: AVCodecID = 118;
pub const AV_CODEC_ID_BFI: AVCodecID = 117;
pub const AV_CODEC_ID_DIRAC: AVCodecID = 116;
pub const AV_CODEC_ID_ESCAPE124: AVCodecID = 115;
pub const AV_CODEC_ID_RL2: AVCodecID = 114;
pub const AV_CODEC_ID_MIMIC: AVCodecID = 113;
pub const AV_CODEC_ID_INDEO5: AVCodecID = 112;
pub const AV_CODEC_ID_INDEO4: AVCodecID = 111;
pub const AV_CODEC_ID_SUNRAST: AVCodecID = 110;
pub const AV_CODEC_ID_PCX: AVCodecID = 109;
pub const AV_CODEC_ID_VB: AVCodecID = 108;
pub const AV_CODEC_ID_AMV: AVCodecID = 107;
pub const AV_CODEC_ID_VP6A: AVCodecID = 106;
pub const AV_CODEC_ID_TXD: AVCodecID = 105;
pub const AV_CODEC_ID_PTX: AVCodecID = 104;
pub const AV_CODEC_ID_BETHSOFTVID: AVCodecID = 103;
pub const AV_CODEC_ID_C93: AVCodecID = 102;
pub const AV_CODEC_ID_SGI: AVCodecID = 101;
pub const AV_CODEC_ID_THP: AVCodecID = 100;
pub const AV_CODEC_ID_DNXHD: AVCodecID = 99;
pub const AV_CODEC_ID_DXA: AVCodecID = 98;
pub const AV_CODEC_ID_GIF: AVCodecID = 97;
pub const AV_CODEC_ID_TIFF: AVCodecID = 96;
pub const AV_CODEC_ID_TIERTEXSEQVIDEO: AVCodecID = 95;
pub const AV_CODEC_ID_DSICINVIDEO: AVCodecID = 94;
pub const AV_CODEC_ID_TARGA: AVCodecID = 93;
pub const AV_CODEC_ID_VP6F: AVCodecID = 92;
pub const AV_CODEC_ID_VP6: AVCodecID = 91;
pub const AV_CODEC_ID_VP5: AVCodecID = 90;
pub const AV_CODEC_ID_VMNC: AVCodecID = 89;
pub const AV_CODEC_ID_JPEG2000: AVCodecID = 88;
pub const AV_CODEC_ID_CAVS: AVCodecID = 87;
pub const AV_CODEC_ID_FLASHSV: AVCodecID = 86;
pub const AV_CODEC_ID_KMVC: AVCodecID = 85;
pub const AV_CODEC_ID_NUV: AVCodecID = 84;
pub const AV_CODEC_ID_SMACKVIDEO: AVCodecID = 83;
pub const AV_CODEC_ID_AVS: AVCodecID = 82;
pub const AV_CODEC_ID_ZMBV: AVCodecID = 81;
pub const AV_CODEC_ID_MMVIDEO: AVCodecID = 80;
pub const AV_CODEC_ID_CSCD: AVCodecID = 79;
pub const AV_CODEC_ID_BMP: AVCodecID = 78;
pub const AV_CODEC_ID_TRUEMOTION2: AVCodecID = 77;
pub const AV_CODEC_ID_FRAPS: AVCodecID = 76;
pub const AV_CODEC_ID_INDEO2: AVCodecID = 75;
pub const AV_CODEC_ID_AASC: AVCodecID = 74;
pub const AV_CODEC_ID_WNV1: AVCodecID = 73;
pub const AV_CODEC_ID_LOCO: AVCodecID = 72;
pub const AV_CODEC_ID_WMV3: AVCodecID = 71;
pub const AV_CODEC_ID_VC1: AVCodecID = 70;
pub const AV_CODEC_ID_RV40: AVCodecID = 69;
pub const AV_CODEC_ID_RV30: AVCodecID = 68;
pub const AV_CODEC_ID_FFVHUFF: AVCodecID = 67;
pub const AV_CODEC_ID_PAM: AVCodecID = 66;
pub const AV_CODEC_ID_PGMYUV: AVCodecID = 65;
pub const AV_CODEC_ID_PGM: AVCodecID = 64;
pub const AV_CODEC_ID_PBM: AVCodecID = 63;
pub const AV_CODEC_ID_PPM: AVCodecID = 62;
pub const AV_CODEC_ID_PNG: AVCodecID = 61;
pub const AV_CODEC_ID_QPEG: AVCodecID = 60;
pub const AV_CODEC_ID_VIXL: AVCodecID = 59;
pub const AV_CODEC_ID_QDRAW: AVCodecID = 58;
pub const AV_CODEC_ID_ULTI: AVCodecID = 57;
pub const AV_CODEC_ID_TSCC: AVCodecID = 56;
pub const AV_CODEC_ID_QTRLE: AVCodecID = 55;
pub const AV_CODEC_ID_ZLIB: AVCodecID = 54;
pub const AV_CODEC_ID_MSZH: AVCodecID = 53;
pub const AV_CODEC_ID_VMDVIDEO: AVCodecID = 52;
pub const AV_CODEC_ID_TRUEMOTION1: AVCodecID = 51;
pub const AV_CODEC_ID_FLIC: AVCodecID = 50;
pub const AV_CODEC_ID_SMC: AVCodecID = 49;
pub const AV_CODEC_ID_8BPS: AVCodecID = 48;
pub const AV_CODEC_ID_IDCIN: AVCodecID = 47;
pub const AV_CODEC_ID_MSVIDEO1: AVCodecID = 46;
pub const AV_CODEC_ID_MSRLE: AVCodecID = 45;
pub const AV_CODEC_ID_WS_VQA: AVCodecID = 44;
pub const AV_CODEC_ID_CINEPAK: AVCodecID = 43;
pub const AV_CODEC_ID_RPZA: AVCodecID = 42;
pub const AV_CODEC_ID_XAN_WC4: AVCodecID = 41;
pub const AV_CODEC_ID_XAN_WC3: AVCodecID = 40;
pub const AV_CODEC_ID_INTERPLAY_VIDEO: AVCodecID = 39;
pub const AV_CODEC_ID_ROQ: AVCodecID = 38;
pub const AV_CODEC_ID_MDEC: AVCodecID = 37;
pub const AV_CODEC_ID_CLJR: AVCodecID = 36;
pub const AV_CODEC_ID_VCR1: AVCodecID = 35;
pub const AV_CODEC_ID_4XM: AVCodecID = 34;
pub const AV_CODEC_ID_FFV1: AVCodecID = 33;
pub const AV_CODEC_ID_ASV2: AVCodecID = 32;
pub const AV_CODEC_ID_ASV1: AVCodecID = 31;
pub const AV_CODEC_ID_THEORA: AVCodecID = 30;
pub const AV_CODEC_ID_VP3: AVCodecID = 29;
pub const AV_CODEC_ID_INDEO3: AVCodecID = 28;
pub const AV_CODEC_ID_H264: AVCodecID = 27;
pub const AV_CODEC_ID_CYUV: AVCodecID = 26;
pub const AV_CODEC_ID_HUFFYUV: AVCodecID = 25;
pub const AV_CODEC_ID_DVVIDEO: AVCodecID = 24;
pub const AV_CODEC_ID_SVQ3: AVCodecID = 23;
pub const AV_CODEC_ID_SVQ1: AVCodecID = 22;
pub const AV_CODEC_ID_FLV1: AVCodecID = 21;
pub const AV_CODEC_ID_H263I: AVCodecID = 20;
pub const AV_CODEC_ID_H263P: AVCodecID = 19;
pub const AV_CODEC_ID_WMV2: AVCodecID = 18;
pub const AV_CODEC_ID_WMV1: AVCodecID = 17;
pub const AV_CODEC_ID_MSMPEG4V3: AVCodecID = 16;
pub const AV_CODEC_ID_MSMPEG4V2: AVCodecID = 15;
pub const AV_CODEC_ID_MSMPEG4V1: AVCodecID = 14;
pub const AV_CODEC_ID_RAWVIDEO: AVCodecID = 13;
pub const AV_CODEC_ID_MPEG4: AVCodecID = 12;
pub const AV_CODEC_ID_JPEGLS: AVCodecID = 11;
pub const AV_CODEC_ID_SP5X: AVCodecID = 10;
pub const AV_CODEC_ID_LJPEG: AVCodecID = 9;
pub const AV_CODEC_ID_MJPEGB: AVCodecID = 8;
pub const AV_CODEC_ID_MJPEG: AVCodecID = 7;
pub const AV_CODEC_ID_RV20: AVCodecID = 6;
pub const AV_CODEC_ID_RV10: AVCodecID = 5;
pub const AV_CODEC_ID_H263: AVCodecID = 4;
pub const AV_CODEC_ID_H261: AVCodecID = 3;
pub const AV_CODEC_ID_MPEG2VIDEO: AVCodecID = 2;
pub const AV_CODEC_ID_MPEG1VIDEO: AVCodecID = 1;
pub const AV_CODEC_ID_NONE: AVCodecID = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVProfile {
    pub profile: ::core::ffi::c_int,
    pub name: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodec {
    pub name: *const ::core::ffi::c_char,
    pub long_name: *const ::core::ffi::c_char,
    pub type_0: AVMediaType,
    pub id: AVCodecID,
    pub capabilities: ::core::ffi::c_int,
    pub max_lowres: uint8_t,
    pub priv_class: *const AVClass,
    pub profiles: *const AVProfile,
    pub wrapper_name: *const ::core::ffi::c_char,
}
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const AV_CODEC_HW_CONFIG_METHOD_AD_HOC: C2RustUnnamed_1 = 8;
pub const AV_CODEC_HW_CONFIG_METHOD_INTERNAL: C2RustUnnamed_1 = 4;
pub const AV_CODEC_HW_CONFIG_METHOD_HW_FRAMES_CTX: C2RustUnnamed_1 = 2;
pub const AV_CODEC_HW_CONFIG_METHOD_HW_DEVICE_CTX: C2RustUnnamed_1 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecHWConfig {
    pub pix_fmt: AVPixelFormat,
    pub methods: ::core::ffi::c_int,
    pub device_type: AVHWDeviceType,
}
pub type AVFieldOrder = ::core::ffi::c_uint;
pub const AV_FIELD_BT: AVFieldOrder = 5;
pub const AV_FIELD_TB: AVFieldOrder = 4;
pub const AV_FIELD_BB: AVFieldOrder = 3;
pub const AV_FIELD_TT: AVFieldOrder = 2;
pub const AV_FIELD_PROGRESSIVE: AVFieldOrder = 1;
pub const AV_FIELD_UNKNOWN: AVFieldOrder = 0;
pub type AVDiscard = ::core::ffi::c_int;
pub const AVDISCARD_ALL: AVDiscard = 48;
pub const AVDISCARD_NONKEY: AVDiscard = 32;
pub const AVDISCARD_NONINTRA: AVDiscard = 24;
pub const AVDISCARD_BIDIR: AVDiscard = 16;
pub const AVDISCARD_NONREF: AVDiscard = 8;
pub const AVDISCARD_DEFAULT: AVDiscard = 0;
pub const AVDISCARD_NONE: AVDiscard = -16;
pub type AVAudioServiceType = ::core::ffi::c_uint;
pub const AV_AUDIO_SERVICE_TYPE_NB: AVAudioServiceType = 9;
pub const AV_AUDIO_SERVICE_TYPE_KARAOKE: AVAudioServiceType = 8;
pub const AV_AUDIO_SERVICE_TYPE_VOICE_OVER: AVAudioServiceType = 7;
pub const AV_AUDIO_SERVICE_TYPE_EMERGENCY: AVAudioServiceType = 6;
pub const AV_AUDIO_SERVICE_TYPE_COMMENTARY: AVAudioServiceType = 5;
pub const AV_AUDIO_SERVICE_TYPE_DIALOGUE: AVAudioServiceType = 4;
pub const AV_AUDIO_SERVICE_TYPE_HEARING_IMPAIRED: AVAudioServiceType = 3;
pub const AV_AUDIO_SERVICE_TYPE_VISUALLY_IMPAIRED: AVAudioServiceType = 2;
pub const AV_AUDIO_SERVICE_TYPE_EFFECTS: AVAudioServiceType = 1;
pub const AV_AUDIO_SERVICE_TYPE_MAIN: AVAudioServiceType = 0;
pub type AVPacketSideDataType = ::core::ffi::c_uint;
pub const AV_PKT_DATA_NB: AVPacketSideDataType = 43;
pub const AV_PKT_DATA_HEVC_CONF: AVPacketSideDataType = 42;
pub const AV_PKT_DATA_DYNAMIC_HDR_SMPTE_2094_APP5: AVPacketSideDataType = 41;
pub const AV_PKT_DATA_EXIF: AVPacketSideDataType = 40;
pub const AV_PKT_DATA_RTCP_SR: AVPacketSideDataType = 39;
pub const AV_PKT_DATA_3D_REFERENCE_DISPLAYS: AVPacketSideDataType = 38;
pub const AV_PKT_DATA_LCEVC: AVPacketSideDataType = 37;
pub const AV_PKT_DATA_FRAME_CROPPING: AVPacketSideDataType = 36;
pub const AV_PKT_DATA_AMBIENT_VIEWING_ENVIRONMENT: AVPacketSideDataType = 35;
pub const AV_PKT_DATA_IAMF_RECON_GAIN_INFO_PARAM: AVPacketSideDataType = 34;
pub const AV_PKT_DATA_IAMF_DEMIXING_INFO_PARAM: AVPacketSideDataType = 33;
pub const AV_PKT_DATA_IAMF_MIX_GAIN_PARAM: AVPacketSideDataType = 32;
pub const AV_PKT_DATA_DYNAMIC_HDR10_PLUS: AVPacketSideDataType = 31;
pub const AV_PKT_DATA_S12M_TIMECODE: AVPacketSideDataType = 30;
pub const AV_PKT_DATA_DOVI_CONF: AVPacketSideDataType = 29;
pub const AV_PKT_DATA_ICC_PROFILE: AVPacketSideDataType = 28;
pub const AV_PKT_DATA_PRFT: AVPacketSideDataType = 27;
pub const AV_PKT_DATA_AFD: AVPacketSideDataType = 26;
pub const AV_PKT_DATA_ENCRYPTION_INFO: AVPacketSideDataType = 25;
pub const AV_PKT_DATA_ENCRYPTION_INIT_INFO: AVPacketSideDataType = 24;
pub const AV_PKT_DATA_A53_CC: AVPacketSideDataType = 23;
pub const AV_PKT_DATA_CONTENT_LIGHT_LEVEL: AVPacketSideDataType = 22;
pub const AV_PKT_DATA_SPHERICAL: AVPacketSideDataType = 21;
pub const AV_PKT_DATA_MASTERING_DISPLAY_METADATA: AVPacketSideDataType = 20;
pub const AV_PKT_DATA_MPEGTS_STREAM_ID: AVPacketSideDataType = 19;
pub const AV_PKT_DATA_METADATA_UPDATE: AVPacketSideDataType = 18;
pub const AV_PKT_DATA_WEBVTT_SETTINGS: AVPacketSideDataType = 17;
pub const AV_PKT_DATA_WEBVTT_IDENTIFIER: AVPacketSideDataType = 16;
pub const AV_PKT_DATA_MATROSKA_BLOCKADDITIONAL: AVPacketSideDataType = 15;
pub const AV_PKT_DATA_SUBTITLE_POSITION: AVPacketSideDataType = 14;
pub const AV_PKT_DATA_STRINGS_METADATA: AVPacketSideDataType = 13;
pub const AV_PKT_DATA_JP_DUALMONO: AVPacketSideDataType = 12;
pub const AV_PKT_DATA_SKIP_SAMPLES: AVPacketSideDataType = 11;
pub const AV_PKT_DATA_CPB_PROPERTIES: AVPacketSideDataType = 10;
pub const AV_PKT_DATA_FALLBACK_TRACK: AVPacketSideDataType = 9;
pub const AV_PKT_DATA_QUALITY_STATS: AVPacketSideDataType = 8;
pub const AV_PKT_DATA_AUDIO_SERVICE_TYPE: AVPacketSideDataType = 7;
pub const AV_PKT_DATA_STEREO3D: AVPacketSideDataType = 6;
pub const AV_PKT_DATA_DISPLAYMATRIX: AVPacketSideDataType = 5;
pub const AV_PKT_DATA_REPLAYGAIN: AVPacketSideDataType = 4;
pub const AV_PKT_DATA_H263_MB_INFO: AVPacketSideDataType = 3;
pub const AV_PKT_DATA_PARAM_CHANGE: AVPacketSideDataType = 2;
pub const AV_PKT_DATA_NEW_EXTRADATA: AVPacketSideDataType = 1;
pub const AV_PKT_DATA_PALETTE: AVPacketSideDataType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPacketSideData {
    pub data: *mut uint8_t,
    pub size: size_t,
    pub type_0: AVPacketSideDataType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPacket {
    pub buf: *mut AVBufferRef,
    pub pts: int64_t,
    pub dts: int64_t,
    pub data: *mut uint8_t,
    pub size: ::core::ffi::c_int,
    pub stream_index: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub side_data: *mut AVPacketSideData,
    pub side_data_elems: ::core::ffi::c_int,
    pub duration: int64_t,
    pub pos: int64_t,
    pub opaque: *mut ::core::ffi::c_void,
    pub opaque_ref: *mut AVBufferRef,
    pub time_base: AVRational,
}
pub type AVSideDataParamChangeFlags = ::core::ffi::c_uint;
pub const AV_SIDE_DATA_PARAM_CHANGE_DIMENSIONS: AVSideDataParamChangeFlags = 8;
pub const AV_SIDE_DATA_PARAM_CHANGE_SAMPLE_RATE: AVSideDataParamChangeFlags = 4;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecParameters {
    pub codec_type: AVMediaType,
    pub codec_id: AVCodecID,
    pub codec_tag: uint32_t,
    pub extradata: *mut uint8_t,
    pub extradata_size: ::core::ffi::c_int,
    pub coded_side_data: *mut AVPacketSideData,
    pub nb_coded_side_data: ::core::ffi::c_int,
    pub format: ::core::ffi::c_int,
    pub bit_rate: int64_t,
    pub bits_per_coded_sample: ::core::ffi::c_int,
    pub bits_per_raw_sample: ::core::ffi::c_int,
    pub profile: ::core::ffi::c_int,
    pub level: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub sample_aspect_ratio: AVRational,
    pub framerate: AVRational,
    pub field_order: AVFieldOrder,
    pub color_range: AVColorRange,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub color_space: AVColorSpace,
    pub chroma_location: AVChromaLocation,
    pub video_delay: ::core::ffi::c_int,
    pub ch_layout: AVChannelLayout,
    pub sample_rate: ::core::ffi::c_int,
    pub block_align: ::core::ffi::c_int,
    pub frame_size: ::core::ffi::c_int,
    pub initial_padding: ::core::ffi::c_int,
    pub trailing_padding: ::core::ffi::c_int,
    pub seek_preroll: ::core::ffi::c_int,
    pub alpha_mode: AVAlphaMode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RcOverride {
    pub start_frame: ::core::ffi::c_int,
    pub end_frame: ::core::ffi::c_int,
    pub qscale: ::core::ffi::c_int,
    pub quality_factor: ::core::ffi::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecContext {
    pub av_class: *const AVClass,
    pub log_level_offset: ::core::ffi::c_int,
    pub codec_type: AVMediaType,
    pub codec: *const AVCodec,
    pub codec_id: AVCodecID,
    pub codec_tag: ::core::ffi::c_uint,
    pub priv_data: *mut ::core::ffi::c_void,
    pub internal: *mut AVCodecInternal,
    pub opaque: *mut ::core::ffi::c_void,
    pub bit_rate: int64_t,
    pub flags: ::core::ffi::c_int,
    pub flags2: ::core::ffi::c_int,
    pub extradata: *mut uint8_t,
    pub extradata_size: ::core::ffi::c_int,
    pub time_base: AVRational,
    pub pkt_timebase: AVRational,
    pub framerate: AVRational,
    pub delay: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub coded_width: ::core::ffi::c_int,
    pub coded_height: ::core::ffi::c_int,
    pub sample_aspect_ratio: AVRational,
    pub pix_fmt: AVPixelFormat,
    pub sw_pix_fmt: AVPixelFormat,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub colorspace: AVColorSpace,
    pub color_range: AVColorRange,
    pub chroma_sample_location: AVChromaLocation,
    pub field_order: AVFieldOrder,
    pub refs: ::core::ffi::c_int,
    pub has_b_frames: ::core::ffi::c_int,
    pub slice_flags: ::core::ffi::c_int,
    pub draw_horiz_band: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *const AVFrame,
            *mut ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub get_format: Option<
        unsafe extern "C" fn(*mut AVCodecContext, *const AVPixelFormat) -> AVPixelFormat,
    >,
    pub max_b_frames: ::core::ffi::c_int,
    pub b_quant_factor: ::core::ffi::c_float,
    pub b_quant_offset: ::core::ffi::c_float,
    pub i_quant_factor: ::core::ffi::c_float,
    pub i_quant_offset: ::core::ffi::c_float,
    pub lumi_masking: ::core::ffi::c_float,
    pub temporal_cplx_masking: ::core::ffi::c_float,
    pub spatial_cplx_masking: ::core::ffi::c_float,
    pub p_masking: ::core::ffi::c_float,
    pub dark_masking: ::core::ffi::c_float,
    pub nsse_weight: ::core::ffi::c_int,
    pub me_cmp: ::core::ffi::c_int,
    pub me_sub_cmp: ::core::ffi::c_int,
    pub mb_cmp: ::core::ffi::c_int,
    pub ildct_cmp: ::core::ffi::c_int,
    pub dia_size: ::core::ffi::c_int,
    pub last_predictor_count: ::core::ffi::c_int,
    pub me_pre_cmp: ::core::ffi::c_int,
    pub pre_dia_size: ::core::ffi::c_int,
    pub me_subpel_quality: ::core::ffi::c_int,
    pub me_range: ::core::ffi::c_int,
    pub mb_decision: ::core::ffi::c_int,
    pub intra_matrix: *mut uint16_t,
    pub inter_matrix: *mut uint16_t,
    pub chroma_intra_matrix: *mut uint16_t,
    pub intra_dc_precision: ::core::ffi::c_int,
    pub mb_lmin: ::core::ffi::c_int,
    pub mb_lmax: ::core::ffi::c_int,
    pub bidir_refine: ::core::ffi::c_int,
    pub keyint_min: ::core::ffi::c_int,
    pub gop_size: ::core::ffi::c_int,
    pub mv0_threshold: ::core::ffi::c_int,
    pub slices: ::core::ffi::c_int,
    pub sample_rate: ::core::ffi::c_int,
    pub sample_fmt: AVSampleFormat,
    pub ch_layout: AVChannelLayout,
    pub frame_size: ::core::ffi::c_int,
    pub block_align: ::core::ffi::c_int,
    pub cutoff: ::core::ffi::c_int,
    pub audio_service_type: AVAudioServiceType,
    pub request_sample_fmt: AVSampleFormat,
    pub initial_padding: ::core::ffi::c_int,
    pub trailing_padding: ::core::ffi::c_int,
    pub seek_preroll: ::core::ffi::c_int,
    pub get_buffer2: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut AVFrame,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub bit_rate_tolerance: ::core::ffi::c_int,
    pub global_quality: ::core::ffi::c_int,
    pub compression_level: ::core::ffi::c_int,
    pub qcompress: ::core::ffi::c_float,
    pub qblur: ::core::ffi::c_float,
    pub qmin: ::core::ffi::c_int,
    pub qmax: ::core::ffi::c_int,
    pub max_qdiff: ::core::ffi::c_int,
    pub rc_buffer_size: ::core::ffi::c_int,
    pub rc_override_count: ::core::ffi::c_int,
    pub rc_override: *mut RcOverride,
    pub rc_max_rate: int64_t,
    pub rc_min_rate: int64_t,
    pub rc_max_available_vbv_use: ::core::ffi::c_float,
    pub rc_min_vbv_overflow_use: ::core::ffi::c_float,
    pub rc_initial_buffer_occupancy: ::core::ffi::c_int,
    pub trellis: ::core::ffi::c_int,
    pub stats_out: *mut ::core::ffi::c_char,
    pub stats_in: *mut ::core::ffi::c_char,
    pub workaround_bugs: ::core::ffi::c_int,
    pub strict_std_compliance: ::core::ffi::c_int,
    pub error_concealment: ::core::ffi::c_int,
    pub debug: ::core::ffi::c_int,
    pub err_recognition: ::core::ffi::c_int,
    pub hwaccel: *const AVHWAccel,
    pub hwaccel_context: *mut ::core::ffi::c_void,
    pub hw_frames_ctx: *mut AVBufferRef,
    pub hw_device_ctx: *mut AVBufferRef,
    pub hwaccel_flags: ::core::ffi::c_int,
    pub extra_hw_frames: ::core::ffi::c_int,
    pub error: [uint64_t; 8],
    pub dct_algo: ::core::ffi::c_int,
    pub idct_algo: ::core::ffi::c_int,
    pub bits_per_coded_sample: ::core::ffi::c_int,
    pub bits_per_raw_sample: ::core::ffi::c_int,
    pub thread_count: ::core::ffi::c_int,
    pub thread_type: ::core::ffi::c_int,
    pub active_thread_type: ::core::ffi::c_int,
    pub execute: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            Option<
                unsafe extern "C" fn(
                    *mut AVCodecContext,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
            >,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub execute2: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            Option<
                unsafe extern "C" fn(
                    *mut AVCodecContext,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub profile: ::core::ffi::c_int,
    pub level: ::core::ffi::c_int,
    pub skip_loop_filter: AVDiscard,
    pub skip_idct: AVDiscard,
    pub skip_frame: AVDiscard,
    pub skip_alpha: ::core::ffi::c_int,
    pub skip_top: ::core::ffi::c_int,
    pub skip_bottom: ::core::ffi::c_int,
    pub lowres: ::core::ffi::c_int,
    pub codec_descriptor: *const AVCodecDescriptor,
    pub sub_charenc: *mut ::core::ffi::c_char,
    pub sub_charenc_mode: ::core::ffi::c_int,
    pub subtitle_header_size: ::core::ffi::c_int,
    pub subtitle_header: *mut uint8_t,
    pub dump_separator: *mut uint8_t,
    pub codec_whitelist: *mut ::core::ffi::c_char,
    pub coded_side_data: *mut AVPacketSideData,
    pub nb_coded_side_data: ::core::ffi::c_int,
    pub export_side_data: ::core::ffi::c_int,
    pub max_pixels: int64_t,
    pub apply_cropping: ::core::ffi::c_int,
    pub discard_damaged_percentage: ::core::ffi::c_int,
    pub max_samples: int64_t,
    pub get_encode_buffer: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut AVPacket,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub frame_num: int64_t,
    pub side_data_prefer_packet: *mut ::core::ffi::c_int,
    pub nb_side_data_prefer_packet: ::core::ffi::c_uint,
    pub decoded_side_data: *mut *mut AVFrameSideData,
    pub nb_decoded_side_data: ::core::ffi::c_int,
    pub alpha_mode: AVAlphaMode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecDescriptor {
    pub id: AVCodecID,
    pub type_0: AVMediaType,
    pub name: *const ::core::ffi::c_char,
    pub long_name: *const ::core::ffi::c_char,
    pub props: ::core::ffi::c_int,
    pub mime_types: *const *const ::core::ffi::c_char,
    pub profiles: *const AVProfile,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVHWAccel {
    pub name: *const ::core::ffi::c_char,
    pub type_0: AVMediaType,
    pub id: AVCodecID,
    pub pix_fmt: AVPixelFormat,
    pub capabilities: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecInternal {
    pub is_copy: ::core::ffi::c_int,
    pub is_frame_mt: ::core::ffi::c_int,
    pub pad_samples: ::core::ffi::c_int,
    pub pool: *mut FramePool,
    pub progress_frame_pool: *mut AVRefStructPool,
    pub thread_ctx: *mut ::core::ffi::c_void,
    pub in_pkt: *mut AVPacket,
    pub bsf: *mut AVBSFContext,
    pub last_pkt_props: *mut AVPacket,
    pub byte_buffer: *mut uint8_t,
    pub byte_buffer_size: ::core::ffi::c_uint,
    pub frame_thread_encoder: *mut ::core::ffi::c_void,
    pub in_frame: *mut AVFrame,
    pub recon_frame: *mut AVFrame,
    pub needs_close: ::core::ffi::c_int,
    pub skip_samples: ::core::ffi::c_int,
    pub hwaccel_priv_data: *mut ::core::ffi::c_void,
    pub draining: ::core::ffi::c_int,
    pub buffer_pkt: *mut AVPacket,
    pub buffer_frame: *mut AVFrame,
    pub draining_done: ::core::ffi::c_int,
    pub warned_on_failed_allocation_from_fixed_pool: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBSFContext {
    pub av_class: *const AVClass,
    pub filter: *const AVBitStreamFilter,
    pub priv_data: *mut ::core::ffi::c_void,
    pub par_in: *mut AVCodecParameters,
    pub par_out: *mut AVCodecParameters,
    pub time_base_in: AVRational,
    pub time_base_out: AVRational,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBitStreamFilter {
    pub name: *const ::core::ffi::c_char,
    pub codec_ids: *const AVCodecID,
    pub priv_class: *const AVClass,
}
pub type AVSubtitleType = ::core::ffi::c_uint;
pub const SUBTITLE_ASS: AVSubtitleType = 3;
pub const SUBTITLE_TEXT: AVSubtitleType = 2;
pub const SUBTITLE_BITMAP: AVSubtitleType = 1;
pub const SUBTITLE_NONE: AVSubtitleType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVSubtitleRect {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub w: ::core::ffi::c_int,
    pub h: ::core::ffi::c_int,
    pub nb_colors: ::core::ffi::c_int,
    pub data: [*mut uint8_t; 4],
    pub linesize: [::core::ffi::c_int; 4],
    pub flags: ::core::ffi::c_int,
    pub type_0: AVSubtitleType,
    pub text: *mut ::core::ffi::c_char,
    pub ass: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVSubtitle {
    pub format: uint16_t,
    pub start_display_time: uint32_t,
    pub end_display_time: uint32_t,
    pub num_rects: ::core::ffi::c_uint,
    pub rects: *mut *mut AVSubtitleRect,
    pub pts: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub decode: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut AVFrame,
            *mut ::core::ffi::c_int,
            *mut AVPacket,
        ) -> ::core::ffi::c_int,
    >,
    pub decode_sub: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut AVSubtitle,
            *mut ::core::ffi::c_int,
            *const AVPacket,
        ) -> ::core::ffi::c_int,
    >,
    pub receive_frame: Option<
        unsafe extern "C" fn(*mut AVCodecContext, *mut AVFrame) -> ::core::ffi::c_int,
    >,
    pub encode: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut AVPacket,
            *const AVFrame,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub encode_sub: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut uint8_t,
            ::core::ffi::c_int,
            *const AVSubtitle,
        ) -> ::core::ffi::c_int,
    >,
    pub receive_packet: Option<
        unsafe extern "C" fn(*mut AVCodecContext, *mut AVPacket) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFCodec {
    pub p: AVCodec,
    pub caps_internal_is_decoder_color_ranges_alpha_modes_cb_type: [u8; 4],
    pub priv_data_size: ::core::ffi::c_int,
    pub update_thread_context: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *const AVCodecContext,
        ) -> ::core::ffi::c_int,
    >,
    pub update_thread_context_for_user: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *const AVCodecContext,
        ) -> ::core::ffi::c_int,
    >,
    pub defaults: *const FFCodecDefault,
    pub init: Option<unsafe extern "C" fn(*mut AVCodecContext) -> ::core::ffi::c_int>,
    pub cb: C2RustUnnamed_2,
    pub close: Option<unsafe extern "C" fn(*mut AVCodecContext) -> ::core::ffi::c_int>,
    pub flush: Option<unsafe extern "C" fn(*mut AVCodecContext) -> ()>,
    pub bsfs: *const ::core::ffi::c_char,
    pub hw_configs: *const *const AVCodecHWConfigInternal,
    pub codec_tags: *const uint32_t,
    pub get_supported_config: Option<
        unsafe extern "C" fn(
            *const AVCodecContext,
            *const AVCodec,
            AVCodecConfig,
            ::core::ffi::c_uint,
            *mut *const ::core::ffi::c_void,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub c2rust_unnamed: C2RustUnnamed_3,
}
impl FFCodec {
    pub fn caps_internal(&self) -> ::core::ffi::c_uint {
        crate::rt::bf_get(&self.caps_internal_is_decoder_color_ranges_alpha_modes_cb_type, 0, 23) as ::core::ffi::c_uint
    }
    pub fn set_caps_internal(&mut self, v: ::core::ffi::c_uint) {
        crate::rt::bf_set(&mut self.caps_internal_is_decoder_color_ranges_alpha_modes_cb_type, 0, 23, v as u64)
    }
    pub fn is_decoder(&self) -> ::core::ffi::c_uint {
        crate::rt::bf_get(&self.caps_internal_is_decoder_color_ranges_alpha_modes_cb_type, 24, 24) as ::core::ffi::c_uint
    }
    pub fn set_is_decoder(&mut self, v: ::core::ffi::c_uint) {
        crate::rt::bf_set(&mut self.caps_internal_is_decoder_color_ranges_alpha_modes_cb_type, 24, 24, v as u64)
    }
    pub fn color_ranges(&self) -> ::core::ffi::c_uint {
        crate::rt::bf_get(&self.caps_internal_is_decoder_color_ranges_alpha_modes_cb_type, 25, 26) as ::core::ffi::c_uint
    }
    pub fn set_color_ranges(&mut self, v: ::core::ffi::c_uint) {
        crate::rt::bf_set(&mut self.caps_internal_is_decoder_color_ranges_alpha_modes_cb_type, 25, 26, v as u64)
    }
    pub fn alpha_modes(&self) -> ::core::ffi::c_uint {
        crate::rt::bf_get(&self.caps_internal_is_decoder_color_ranges_alpha_modes_cb_type, 27, 28) as ::core::ffi::c_uint
    }
    pub fn set_alpha_modes(&mut self, v: ::core::ffi::c_uint) {
        crate::rt::bf_set(&mut self.caps_internal_is_decoder_color_ranges_alpha_modes_cb_type, 27, 28, v as u64)
    }
    pub fn cb_type(&self) -> ::core::ffi::c_uint {
        crate::rt::bf_get(&self.caps_internal_is_decoder_color_ranges_alpha_modes_cb_type, 29, 31) as ::core::ffi::c_uint
    }
    pub fn set_cb_type(&mut self, v: ::core::ffi::c_uint) {
        crate::rt::bf_set(&mut self.caps_internal_is_decoder_color_ranges_alpha_modes_cb_type, 29, 31, v as u64)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub c2rust_unnamed: C2RustUnnamed_5,
    pub c2rust_unnamed_0: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub ch_layouts: *const AVChannelLayout,
    pub supported_samplerates: *const ::core::ffi::c_int,
    pub sample_fmts: *const AVSampleFormat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub supported_framerates: *const AVRational,
    pub pix_fmts: *const AVPixelFormat,
}
pub type AVCodecConfig = ::core::ffi::c_uint;
pub const AV_CODEC_CONFIG_ALPHA_MODE: AVCodecConfig = 7;
pub const AV_CODEC_CONFIG_COLOR_SPACE: AVCodecConfig = 6;
pub const AV_CODEC_CONFIG_COLOR_RANGE: AVCodecConfig = 5;
pub const AV_CODEC_CONFIG_CHANNEL_LAYOUT: AVCodecConfig = 4;
pub const AV_CODEC_CONFIG_SAMPLE_FORMAT: AVCodecConfig = 3;
pub const AV_CODEC_CONFIG_SAMPLE_RATE: AVCodecConfig = 2;
pub const AV_CODEC_CONFIG_FRAME_RATE: AVCodecConfig = 1;
pub const AV_CODEC_CONFIG_PIX_FORMAT: AVCodecConfig = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecHWConfigInternal {
    pub public: AVCodecHWConfig,
    pub hwaccel: *const FFHWAccel,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFHWAccel {
    pub p: AVHWAccel,
    pub alloc_frame: Option<
        unsafe extern "C" fn(*mut AVCodecContext, *mut AVFrame) -> ::core::ffi::c_int,
    >,
    pub start_frame: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *const AVBufferRef,
            *const uint8_t,
            uint32_t,
        ) -> ::core::ffi::c_int,
    >,
    pub decode_params: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            ::core::ffi::c_int,
            *const uint8_t,
            uint32_t,
        ) -> ::core::ffi::c_int,
    >,
    pub decode_slice: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *const uint8_t,
            uint32_t,
        ) -> ::core::ffi::c_int,
    >,
    pub end_frame: Option<
        unsafe extern "C" fn(*mut AVCodecContext) -> ::core::ffi::c_int,
    >,
    pub frame_priv_data_size: ::core::ffi::c_int,
    pub priv_data_size: ::core::ffi::c_int,
    pub caps_internal: ::core::ffi::c_int,
    pub init: Option<unsafe extern "C" fn(*mut AVCodecContext) -> ::core::ffi::c_int>,
    pub uninit: Option<unsafe extern "C" fn(*mut AVCodecContext) -> ::core::ffi::c_int>,
    pub frame_params: Option<
        unsafe extern "C" fn(*mut AVCodecContext, *mut AVBufferRef) -> ::core::ffi::c_int,
    >,
    pub update_thread_context: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *const AVCodecContext,
        ) -> ::core::ffi::c_int,
    >,
    pub free_frame_priv: Option<
        unsafe extern "C" fn(AVRefStructOpaque, *mut ::core::ffi::c_void) -> (),
    >,
    pub flush: Option<unsafe extern "C" fn(*mut AVCodecContext) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union AVRefStructOpaque {
    pub nc: *mut ::core::ffi::c_void,
    pub c: *const ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFCodecDefault {
    pub key: *const ::core::ffi::c_char,
    pub value: *const ::core::ffi::c_char,
}
pub const FF_CODEC_CB_TYPE_DECODE_SUB: FFCodecType = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FrameDecodeData {
    pub post_process: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut AVFrame,
        ) -> ::core::ffi::c_int,
    >,
    pub post_process_opaque: *mut ::core::ffi::c_void,
    pub hwaccel_priv_post_process: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut AVFrame,
        ) -> ::core::ffi::c_int,
    >,
    pub hwaccel_priv: *mut ::core::ffi::c_void,
    pub hwaccel_priv_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DecodeContext {
    pub avci: AVCodecInternal,
    pub intra_only_flag: ::core::ffi::c_int,
    pub initial_pict_type: AVPictureType,
    pub nb_draining_errors: ::core::ffi::c_int,
    pub draining_started: ::core::ffi::c_int,
    pub pts_correction_num_faulty_pts: int64_t,
    pub pts_correction_num_faulty_dts: int64_t,
    pub pts_correction_last_pts: int64_t,
    pub pts_correction_last_dts: int64_t,
    pub side_data_pref_mask: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union unaligned_32 {
    pub l: uint32_t,
}
pub const FF_CODEC_CB_TYPE_RECEIVE_FRAME: FFCodecType = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SideDataMap {
    pub packet: AVPacketSideDataType,
    pub frame: AVFrameSideDataType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ProgressInternal {
    pub progress: ThreadProgress,
    pub f: *mut AVFrame,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ThreadProgress {
    pub progress: atomic_int,
    pub init: ::core::ffi::c_uint,
    pub progress_mutex: pthread_mutex_t,
    pub progress_cond: pthread_cond_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pthread_cond_t {
    pub __u: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub __i: [::core::ffi::c_int; 12],
    pub __vi: [::core::ffi::c_int; 12],
    pub __p: [*mut ::core::ffi::c_void; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pthread_mutex_t {
    pub __u: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub __i: [::core::ffi::c_int; 6],
    pub __vi: [::core::ffi::c_int; 6],
    pub __p: [*mut ::core::ffi::c_void; 6],
}
pub type atomic_int = ::core::ffi::c_int;
pub type FFCodecType = ::core::ffi::c_uint;
pub const FF_CODEC_CB_TYPE_RECEIVE_PACKET: FFCodecType = 5;
pub const FF_CODEC_CB_TYPE_ENCODE_SUB: FFCodecType = 4;
pub const FF_CODEC_CB_TYPE_ENCODE: FFCodecType = 3;
pub const FF_CODEC_CB_TYPE_DECODE: FFCodecType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVExifMetadata {
    pub entries: *mut AVExifEntry,
    pub count: ::core::ffi::c_uint,
    pub size: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVExifEntry {
    pub id: uint16_t,
    pub type_0: AVTiffDataType,
    pub count: uint32_t,
    pub ifd_offset: uint32_t,
    pub ifd_lead: *mut uint8_t,
    pub value: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub ptr: *mut ::core::ffi::c_void,
    pub sint: *mut int64_t,
    pub uint: *mut uint64_t,
    pub dbl: *mut ::core::ffi::c_double,
    pub str_0: *mut ::core::ffi::c_char,
    pub ubytes: *mut uint8_t,
    pub sbytes: *mut int8_t,
    pub rat: *mut AVRational,
    pub ifd: AVExifMetadata,
}
pub type AVTiffDataType = ::core::ffi::c_uint;
pub const AV_TIFF_IFD: AVTiffDataType = 13;
pub const AV_TIFF_DOUBLE: AVTiffDataType = 12;
pub const AV_TIFF_FLOAT: AVTiffDataType = 11;
pub const AV_TIFF_SRATIONAL: AVTiffDataType = 10;
pub const AV_TIFF_SLONG: AVTiffDataType = 9;
pub const AV_TIFF_SSHORT: AVTiffDataType = 8;
pub const AV_TIFF_UNDEFINED: AVTiffDataType = 7;
pub const AV_TIFF_SBYTE: AVTiffDataType = 6;
pub const AV_TIFF_RATIONAL: AVTiffDataType = 5;
pub const AV_TIFF_LONG: AVTiffDataType = 4;
pub const AV_TIFF_SHORT: AVTiffDataType = 3;
pub const AV_TIFF_STRING: AVTiffDataType = 2;
pub const AV_TIFF_BYTE: AVTiffDataType = 1;
pub type AVExifHeaderMode = ::core::ffi::c_uint;
pub const AV_EXIF_EXIF00: AVExifHeaderMode = 4;
pub const AV_EXIF_T_OFF: AVExifHeaderMode = 3;
pub const AV_EXIF_ASSUME_BE: AVExifHeaderMode = 2;
pub const AV_EXIF_ASSUME_LE: AVExifHeaderMode = 1;
pub const AV_EXIF_TIFF_HEADER: AVExifHeaderMode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ProgressFrame {
    pub f: *mut AVFrame,
    pub progress: *mut ProgressInternal,
}
#[inline]
unsafe extern "C" fn ff_thread_progress_reset(mut pro: *mut ThreadProgress) {
    *&raw mut (*pro).progress = if (*pro).init != 0 {
        -(1 as ::core::ffi::c_int)
    } else {
        2147483647 as ::core::ffi::c_int
    };
}
#[inline(always)]
unsafe extern "C" fn bytestream_get_le32(
    mut b: *mut *const uint8_t,
) -> ::core::ffi::c_uint {
    *b = (*b).offset(4 as ::core::ffi::c_int as isize);
    return (*((*b).offset(-(4 as ::core::ffi::c_int as isize)) as *const unaligned_32)).l
        as ::core::ffi::c_uint;
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
unsafe extern "C" fn decode_ctx(mut avci: *mut AVCodecInternal) -> *mut DecodeContext {
    return avci as *mut DecodeContext;
}
#[inline]
unsafe extern "C" fn av_make_error_string(
    mut errbuf: *mut ::core::ffi::c_char,
    mut errbuf_size: size_t,
    mut errnum: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    av_strerror(errnum, errbuf, errbuf_size);
    return errbuf;
}
unsafe extern "C" fn apply_param_change(
    mut avctx: *mut AVCodecContext,
    mut avpkt: *const AVPacket,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut ret: ::core::ffi::c_int = 0;
    let mut size: size_t = 0;
    let mut data: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut flags: uint32_t = 0;
    let mut val: int64_t = 0;
    data = av_packet_get_side_data(avpkt, AV_PKT_DATA_PARAM_CHANGE, &raw mut size);
    if data.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if (*(*avctx).codec).capabilities
        & (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int == 0
    {
        av_log(
            avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"This decoder does not support parameter changes, but PARAM_CHANGE side data was sent to it.\n\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        ret = -(22 as ::core::ffi::c_int);
    } else {
        if size < 4 as size_t {
            current_block = 12687097206862744754;
        } else {
            flags = bytestream_get_le32(&raw mut data) as uint32_t;
            size = size.wrapping_sub(4 as size_t);
            if flags
                & AV_SIDE_DATA_PARAM_CHANGE_SAMPLE_RATE as ::core::ffi::c_int as uint32_t
                != 0
            {
                if size < 4 as size_t {
                    current_block = 12687097206862744754;
                } else {
                    val = bytestream_get_le32(&raw mut data) as int64_t;
                    if val <= 0 as int64_t || val > 2147483647 as int64_t {
                        av_log(
                            avctx as *mut ::core::ffi::c_void,
                            16 as ::core::ffi::c_int,
                            b"Invalid sample rate\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        ret = -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                            | ('D' as i32) << 16 as ::core::ffi::c_int)
                            as ::core::ffi::c_uint
                            | ('A' as i32 as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                        current_block = 13525959574202245586;
                    } else {
                        (*avctx).sample_rate = val as ::core::ffi::c_int;
                        size = size.wrapping_sub(4 as size_t);
                        current_block = 9606288038608642794;
                    }
                }
            } else {
                current_block = 9606288038608642794;
            }
            match current_block {
                12687097206862744754 => {}
                13525959574202245586 => {}
                _ => {
                    if flags
                        & AV_SIDE_DATA_PARAM_CHANGE_DIMENSIONS as ::core::ffi::c_int
                            as uint32_t != 0
                    {
                        if size < 8 as size_t {
                            current_block = 12687097206862744754;
                        } else {
                            (*avctx).width = bytestream_get_le32(&raw mut data)
                                as ::core::ffi::c_int;
                            (*avctx).height = bytestream_get_le32(&raw mut data)
                                as ::core::ffi::c_int;
                            size = size.wrapping_sub(8 as size_t);
                            ret = ff_set_dimensions(
                                avctx,
                                (*avctx).width,
                                (*avctx).height,
                            );
                            if ret < 0 as ::core::ffi::c_int {
                                current_block = 13525959574202245586;
                            } else {
                                current_block = 5601891728916014340;
                            }
                        }
                    } else {
                        current_block = 5601891728916014340;
                    }
                    match current_block {
                        13525959574202245586 => {}
                        12687097206862744754 => {}
                        _ => return 0 as ::core::ffi::c_int,
                    }
                }
            }
        }
        match current_block {
            13525959574202245586 => {}
            _ => {
                av_log(
                    avctx as *mut ::core::ffi::c_void,
                    16 as ::core::ffi::c_int,
                    b"PARAM_CHANGE side data too small.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                ret = -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                    | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
            }
        }
    }
    if ret < 0 as ::core::ffi::c_int {
        av_log(
            avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"Error applying parameter changes.\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        if (*avctx).err_recognition
            & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
        {
            return ret;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn ffhwaccel(mut codec: *const AVHWAccel) -> *const FFHWAccel {
    return codec as *const FFHWAccel;
}
unsafe extern "C" fn extract_packet_props(
    mut avci: *mut AVCodecInternal,
    mut pkt: *const AVPacket,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    av_packet_unref((*avci).last_pkt_props);
    if !pkt.is_null() {
        ret = av_packet_copy_props((*avci).last_pkt_props, pkt);
    }
    return ret;
}
#[inline]
unsafe extern "C" fn av_image_copy2(
    mut dst_data: *const *mut uint8_t,
    mut dst_linesizes: *const ::core::ffi::c_int,
    mut src_data: *const *mut uint8_t,
    mut src_linesizes: *const ::core::ffi::c_int,
    mut pix_fmt: AVPixelFormat,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) {
    av_image_copy(
        dst_data,
        dst_linesizes,
        src_data as *const *const uint8_t,
        src_linesizes,
        pix_fmt,
        width,
        height,
    );
}
unsafe extern "C" fn decode_bsfs_init(
    mut avctx: *mut AVCodecContext,
) -> ::core::ffi::c_int {
    let mut avci: *mut AVCodecInternal = (*avctx).internal as *mut AVCodecInternal;
    let codec: *const FFCodec = ffcodec((*avctx).codec as *const AVCodec)
        as *const FFCodec;
    let mut ret: ::core::ffi::c_int = 0;
    if !(*avci).bsf.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    ret = av_bsf_list_parse_str((*codec).bsfs, &raw mut (*avci).bsf);
    if ret < 0 as ::core::ffi::c_int {
        let mut fresh8: [::core::ffi::c_char; 64] = [
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
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
        ];
        av_log(
            avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"Error parsing decoder bitstream filters '%s': %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            (*codec).bsfs,
            av_make_error_string(
                &raw mut fresh8 as *mut ::core::ffi::c_char,
                64 as size_t,
                ret,
            ),
        );
        if ret != -(12 as ::core::ffi::c_int) {
            ret = -((('B' as i32 | ('U' as i32) << 8 as ::core::ffi::c_int
                | ('G' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('!' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
    } else {
        (*(*avci).bsf).time_base_in = AVRational {
            num: 1 as ::core::ffi::c_int,
            den: 90000 as ::core::ffi::c_int,
        };
        ret = avcodec_parameters_from_context(
            (*(*avci).bsf).par_in as *mut AVCodecParameters,
            avctx,
        );
        if !(ret < 0 as ::core::ffi::c_int) {
            ret = av_bsf_init((*avci).bsf as *mut AVBSFContext);
            if !(ret < 0 as ::core::ffi::c_int) {
                return 0 as ::core::ffi::c_int;
            }
        }
    }
    av_bsf_free(&raw mut (*avci).bsf);
    return ret;
}
unsafe extern "C" fn decode_get_packet(
    mut avctx: *mut AVCodecContext,
    mut pkt: *mut AVPacket,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut avci: *mut AVCodecInternal = (*avctx).internal as *mut AVCodecInternal;
    let mut ret: ::core::ffi::c_int = 0;
    ret = av_bsf_receive_packet((*avci).bsf as *mut AVBSFContext, pkt);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    if (*ffcodec((*avctx).codec as *const AVCodec)).caps_internal() as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int == 0
    {
        ret = extract_packet_props((*avctx).internal as *mut AVCodecInternal, pkt);
        if ret < 0 as ::core::ffi::c_int {
            current_block = 16644483548944680971;
        } else {
            current_block = 15427931788582360902;
        }
    } else {
        current_block = 15427931788582360902;
    }
    match current_block {
        15427931788582360902 => {
            ret = apply_param_change(avctx, pkt);
            if !(ret < 0 as ::core::ffi::c_int) {
                return 0 as ::core::ffi::c_int;
            }
        }
        _ => {}
    }
    av_packet_unref(pkt);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ff_decode_get_packet(
    mut avctx: *mut AVCodecContext,
    mut pkt: *mut AVPacket,
) -> ::core::ffi::c_int {
    let mut avci: *mut AVCodecInternal = (*avctx).internal as *mut AVCodecInternal;
    let mut dc: *mut DecodeContext = decode_ctx(avci);
    if (*avci).draining != 0 {
        return -((('E' as i32 | ('O' as i32) << 8 as ::core::ffi::c_int
            | ('F' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | (' ' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if (*(*avctx).internal).is_frame_mt != 0 {
        return ff_thread_get_packet(avctx as *mut AVCodecContext, pkt);
    }
    loop {
        let mut ret: ::core::ffi::c_int = decode_get_packet(avctx, pkt);
        if ret == -(11 as ::core::ffi::c_int)
            && (!((*(*avci).buffer_pkt).data.is_null()
                && (*(*avci).buffer_pkt).side_data_elems == 0)
                || (*dc).draining_started != 0)
        {
            ret = av_bsf_send_packet(
                (*avci).bsf as *mut AVBSFContext,
                (*avci).buffer_pkt,
            );
            if ret >= 0 as ::core::ffi::c_int {
                continue;
            }
            av_packet_unref((*avci).buffer_pkt);
        }
        if ret
            == -((('E' as i32 | ('O' as i32) << 8 as ::core::ffi::c_int
                | ('F' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | (' ' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int)
        {
            (*avci).draining = 1 as ::core::ffi::c_int;
        }
        return ret;
    };
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
unsafe extern "C" fn guess_correct_pts(
    mut dc: *mut DecodeContext,
    mut reordered_pts: int64_t,
    mut dts: int64_t,
) -> int64_t {
    let mut pts: int64_t = 0x8000000000000000 as ::core::ffi::c_ulonglong as int64_t;
    if dts != 0x8000000000000000 as ::core::ffi::c_ulonglong as int64_t {
        (*dc).pts_correction_num_faulty_dts
            += (dts <= (*dc).pts_correction_last_dts) as ::core::ffi::c_int as int64_t;
        (*dc).pts_correction_last_dts = dts;
    } else if reordered_pts != 0x8000000000000000 as ::core::ffi::c_ulonglong as int64_t
    {
        (*dc).pts_correction_last_dts = reordered_pts;
    }
    if reordered_pts != 0x8000000000000000 as ::core::ffi::c_ulonglong as int64_t {
        (*dc).pts_correction_num_faulty_pts
            += (reordered_pts <= (*dc).pts_correction_last_pts) as ::core::ffi::c_int
                as int64_t;
        (*dc).pts_correction_last_pts = reordered_pts;
    } else if dts != 0x8000000000000000 as ::core::ffi::c_ulonglong as int64_t {
        (*dc).pts_correction_last_pts = dts;
    }
    if ((*dc).pts_correction_num_faulty_pts <= (*dc).pts_correction_num_faulty_dts
        || dts == 0x8000000000000000 as ::core::ffi::c_ulonglong as int64_t)
        && reordered_pts != 0x8000000000000000 as ::core::ffi::c_ulonglong as int64_t
    {
        pts = reordered_pts;
    } else {
        pts = dts;
    }
    return pts;
}
#[inline(always)]
unsafe extern "C" fn ffcodec(mut codec: *const AVCodec) -> *const FFCodec {
    return codec as *const FFCodec;
}
unsafe extern "C" fn discard_samples(
    mut avctx: *mut AVCodecContext,
    mut frame: *mut AVFrame,
    mut discarded_samples: *mut int64_t,
) -> ::core::ffi::c_int {
    let mut avci: *mut AVCodecInternal = (*avctx).internal as *mut AVCodecInternal;
    let mut side: *mut AVFrameSideData = ::core::ptr::null_mut::<AVFrameSideData>();
    let mut discard_padding: uint32_t = 0 as uint32_t;
    let mut skip_reason: uint8_t = 0 as uint8_t;
    let mut discard_reason: uint8_t = 0 as uint8_t;
    side = av_frame_get_side_data(frame, AV_FRAME_DATA_SKIP_SAMPLES);
    if !side.is_null() && (*side).size >= 10 as size_t {
        let mut skip_samples: ::core::ffi::c_int = (*((*side).data
            as *const unaligned_32))
            .l as ::core::ffi::c_int;
        if skip_samples != 0 {
            (*avci).skip_samples = skip_samples;
        }
        (*avci).skip_samples = if 0 as ::core::ffi::c_int > (*avci).skip_samples {
            0 as ::core::ffi::c_int
        } else {
            (*avci).skip_samples
        };
        discard_padding = (*((*side).data.offset(4 as ::core::ffi::c_int as isize)
            as *const unaligned_32))
            .l;
        av_log(
            avctx as *mut ::core::ffi::c_void,
            48 as ::core::ffi::c_int,
            b"skip %d / discard %d samples due to side data\n\0" as *const u8
                as *const ::core::ffi::c_char,
            (*avci).skip_samples,
            discard_padding as ::core::ffi::c_int,
        );
        skip_reason = *((*side).data.offset(8 as ::core::ffi::c_int as isize)
            as *const uint8_t)
            .offset(0 as ::core::ffi::c_int as isize);
        discard_reason = *((*side).data.offset(9 as ::core::ffi::c_int as isize)
            as *const uint8_t)
            .offset(0 as ::core::ffi::c_int as isize);
    }
    if (*avctx).flags2 & (1 as ::core::ffi::c_int) << 29 as ::core::ffi::c_int != 0 {
        if side.is_null() && ((*avci).skip_samples != 0 || discard_padding != 0) {
            side = av_frame_new_side_data(
                frame,
                AV_FRAME_DATA_SKIP_SAMPLES,
                10 as size_t,
            );
        }
        if !side.is_null() && ((*avci).skip_samples != 0 || discard_padding != 0) {
            (*((*side).data as *mut unaligned_32)).l = (*avci).skip_samples as uint32_t;
            (*((*side).data.offset(4 as ::core::ffi::c_int as isize)
                as *mut unaligned_32))
                .l = discard_padding;
            *(*side)
                .data
                .offset(8 as ::core::ffi::c_int as isize)
                .offset(0 as ::core::ffi::c_int as isize) = skip_reason;
            *(*side)
                .data
                .offset(9 as ::core::ffi::c_int as isize)
                .offset(0 as ::core::ffi::c_int as isize) = discard_reason;
            (*avci).skip_samples = 0 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
    av_frame_remove_side_data(frame, AV_FRAME_DATA_SKIP_SAMPLES);
    if (*frame).flags & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0 {
        (*avci).skip_samples = if 0 as ::core::ffi::c_int
            > (*avci).skip_samples - (*frame).nb_samples
        {
            0 as ::core::ffi::c_int
        } else {
            (*avci).skip_samples - (*frame).nb_samples
        };
        *discarded_samples += (*frame).nb_samples as int64_t;
        return -(11 as ::core::ffi::c_int);
    }
    if (*avci).skip_samples > 0 as ::core::ffi::c_int {
        if (*frame).nb_samples <= (*avci).skip_samples {
            *discarded_samples += (*frame).nb_samples as int64_t;
            (*avci).skip_samples -= (*frame).nb_samples;
            av_log(
                avctx as *mut ::core::ffi::c_void,
                48 as ::core::ffi::c_int,
                b"skip whole frame, skip left: %d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*avci).skip_samples,
            );
            return -(11 as ::core::ffi::c_int);
        } else {
            av_samples_copy(
                (*frame).extended_data,
                (*frame).extended_data,
                0 as ::core::ffi::c_int,
                (*avci).skip_samples,
                (*frame).nb_samples - (*avci).skip_samples,
                (*avctx).ch_layout.nb_channels,
                (*frame).format as AVSampleFormat,
            );
            if (*avctx).pkt_timebase.num != 0 && (*avctx).sample_rate != 0 {
                let mut diff_ts: int64_t = av_rescale_q(
                    (*avci).skip_samples as int64_t,
                    AVRational {
                        num: 1 as ::core::ffi::c_int,
                        den: (*avctx).sample_rate,
                    },
                    (*avctx).pkt_timebase,
                );
                if diff_ts != 0x8000000000000000 as ::core::ffi::c_ulonglong as int64_t {
                    if (*frame).pts
                        != 0x8000000000000000 as ::core::ffi::c_ulonglong as int64_t
                    {
                        (*frame).pts = av_sat_add64_c((*frame).pts, diff_ts);
                    }
                    if (*frame).pkt_dts
                        != 0x8000000000000000 as ::core::ffi::c_ulonglong as int64_t
                    {
                        (*frame).pkt_dts = av_sat_add64_c((*frame).pkt_dts, diff_ts);
                    }
                    if (*frame).duration >= diff_ts {
                        (*frame).duration = av_sat_sub64_c((*frame).duration, diff_ts);
                    }
                } else {
                    (*frame).pts = 0x8000000000000000 as ::core::ffi::c_ulonglong
                        as int64_t;
                    (*frame).pkt_dts = 0x8000000000000000 as ::core::ffi::c_ulonglong
                        as int64_t;
                    (*frame).duration = 0 as int64_t;
                }
            } else {
                av_log(
                    avctx as *mut ::core::ffi::c_void,
                    24 as ::core::ffi::c_int,
                    b"Could not update timestamps for skipped samples.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            av_log(
                avctx as *mut ::core::ffi::c_void,
                48 as ::core::ffi::c_int,
                b"skip %d/%d samples\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*avci).skip_samples,
                (*frame).nb_samples,
            );
            *discarded_samples += (*avci).skip_samples as int64_t;
            (*frame).nb_samples -= (*avci).skip_samples;
            (*avci).skip_samples = 0 as ::core::ffi::c_int;
        }
    }
    if discard_padding > 0 as uint32_t
        && discard_padding <= (*frame).nb_samples as uint32_t
    {
        if discard_padding == (*frame).nb_samples as uint32_t {
            *discarded_samples += (*frame).nb_samples as int64_t;
            return -(11 as ::core::ffi::c_int);
        } else {
            if (*avctx).pkt_timebase.num != 0 && (*avctx).sample_rate != 0 {
                let mut diff_ts_0: int64_t = av_rescale_q(
                    ((*frame).nb_samples as uint32_t).wrapping_sub(discard_padding)
                        as int64_t,
                    AVRational {
                        num: 1 as ::core::ffi::c_int,
                        den: (*avctx).sample_rate,
                    },
                    (*avctx).pkt_timebase,
                );
                (*frame).duration = if diff_ts_0
                    == 0x8000000000000000 as ::core::ffi::c_ulonglong as int64_t
                {
                    0 as int64_t
                } else {
                    diff_ts_0
                };
            } else {
                av_log(
                    avctx as *mut ::core::ffi::c_void,
                    24 as ::core::ffi::c_int,
                    b"Could not update timestamps for discarded samples.\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            av_log(
                avctx as *mut ::core::ffi::c_void,
                48 as ::core::ffi::c_int,
                b"discard %d/%d samples\n\0" as *const u8 as *const ::core::ffi::c_char,
                discard_padding as ::core::ffi::c_int,
                (*frame).nb_samples,
            );
            (*frame).nb_samples = ((*frame).nb_samples as uint32_t)
                .wrapping_sub(discard_padding) as ::core::ffi::c_int
                as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn ff_codec_is_decoder(
    mut avcodec: *const AVCodec,
) -> ::core::ffi::c_int {
    let codec: *const FFCodec = ffcodec(avcodec) as *const FFCodec;
    return (*codec).is_decoder() as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn av_sat_add64_c(mut a: int64_t, mut b: int64_t) -> int64_t {
    let mut tmp: int64_t = 0;
    let (fresh0, fresh1) = a.overflowing_add(b);
    *&raw mut tmp = fresh0;
    return if !fresh1 {
        tmp
    } else if tmp < 0 as int64_t {
        0x7fffffffffffffff as int64_t
    } else {
        -(1 as ::core::ffi::c_int) as int64_t - 0x7fffffffffffffff as int64_t
    };
}
#[inline(always)]
unsafe extern "C" fn av_sat_sub64_c(mut a: int64_t, mut b: int64_t) -> int64_t {
    let mut tmp: int64_t = 0;
    let (fresh2, fresh3) = a.overflowing_sub(b);
    *&raw mut tmp = fresh2;
    return if !fresh3 {
        tmp
    } else if tmp < 0 as int64_t {
        0x7fffffffffffffff as int64_t
    } else {
        -(1 as ::core::ffi::c_int) as int64_t - 0x7fffffffffffffff as int64_t
    };
}
#[inline]
unsafe extern "C" fn decode_simple_internal(
    mut avctx: *mut AVCodecContext,
    mut frame: *mut AVFrame,
    mut discarded_samples: *mut int64_t,
) -> ::core::ffi::c_int {
    let mut avci: *mut AVCodecInternal = (*avctx).internal as *mut AVCodecInternal;
    let mut dc: *mut DecodeContext = decode_ctx(avci);
    let pkt: *mut AVPacket = (*avci).in_pkt;
    let codec: *const FFCodec = ffcodec((*avctx).codec as *const AVCodec)
        as *const FFCodec;
    let mut got_frame: ::core::ffi::c_int = 0;
    let mut consumed: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    if (*pkt).data.is_null() && (*avci).draining == 0 {
        av_packet_unref(pkt);
        ret = ff_decode_get_packet(avctx, pkt);
        if ret < 0 as ::core::ffi::c_int
            && ret
                != -((('E' as i32 | ('O' as i32) << 8 as ::core::ffi::c_int
                    | ('F' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | (' ' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int)
        {
            return ret;
        }
    }
    if (*avci).draining_done != 0 {
        return -((('E' as i32 | ('O' as i32) << 8 as ::core::ffi::c_int
            | ('F' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | (' ' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if (*pkt).data.is_null()
        && (*(*avctx).codec).capabilities
            & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int == 0
    {
        return -((('E' as i32 | ('O' as i32) << 8 as ::core::ffi::c_int
            | ('F' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | (' ' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    got_frame = 0 as ::core::ffi::c_int;
    (*frame).pict_type = (*dc).initial_pict_type;
    (*frame).flags |= (*dc).intra_only_flag;
    consumed = (*codec)
        .cb
        .decode
        .expect(
            "non-null function pointer",
        )(
        avctx as *mut AVCodecContext,
        frame as *mut AVFrame,
        &raw mut got_frame,
        pkt as *mut AVPacket,
    );
    if (*codec).caps_internal() as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
    {
        (*frame).pkt_dts = (*pkt).dts;
    }
    if (*(*avctx).codec).type_0 as ::core::ffi::c_int
        == AVMEDIA_TYPE_VIDEO as ::core::ffi::c_int
    {
        ret = if got_frame == 0
            || (*frame).flags & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
        {
            -(11 as ::core::ffi::c_int)
        } else {
            0 as ::core::ffi::c_int
        };
    } else if (*(*avctx).codec).type_0 as ::core::ffi::c_int
        == AVMEDIA_TYPE_AUDIO as ::core::ffi::c_int
    {
        ret = if got_frame == 0 {
            -(11 as ::core::ffi::c_int)
        } else {
            discard_samples(avctx, frame, discarded_samples)
        };
    } else if 0 as ::core::ffi::c_int == 0 {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"0\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavcodec/decode.c\0" as *const u8 as *const ::core::ffi::c_char,
            471 as ::core::ffi::c_int,
        );
        abort();
    }
    if ret == -(11 as ::core::ffi::c_int) {
        av_frame_unref(frame);
    }
    if !(consumed != -(11 as ::core::ffi::c_int)) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"consumed != (-(11))\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavcodec/decode.c\0" as *const u8 as *const ::core::ffi::c_char,
            478 as ::core::ffi::c_int,
        );
        abort();
    }
    if consumed < 0 as ::core::ffi::c_int {
        ret = consumed;
    }
    if consumed >= 0 as ::core::ffi::c_int
        && (*(*avctx).codec).type_0 as ::core::ffi::c_int
            == AVMEDIA_TYPE_VIDEO as ::core::ffi::c_int
    {
        consumed = (*pkt).size;
    }
    if ret == 0 {
        if (*frame).buf[0 as ::core::ffi::c_int as usize].is_null() {
            av_log(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                0 as ::core::ffi::c_int,
                b"Assertion %s failed at %s:%d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"frame->buf[0]\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/libavcodec/decode.c\0" as *const u8 as *const ::core::ffi::c_char,
                485 as ::core::ffi::c_int,
            );
            abort();
        }
    }
    if ret == -(11 as ::core::ffi::c_int) {
        ret = 0 as ::core::ffi::c_int;
    }
    if (*avci).draining != 0 && got_frame == 0 {
        if ret < 0 as ::core::ffi::c_int {
            let mut nb_errors_max: ::core::ffi::c_int = 20 as ::core::ffi::c_int
                + (if 1 as ::core::ffi::c_int != 0
                    && (*avctx).active_thread_type & 1 as ::core::ffi::c_int != 0
                {
                    (*avctx).thread_count
                } else {
                    1 as ::core::ffi::c_int
                });
            let ref mut fresh6 = (*decode_ctx(avci)).nb_draining_errors;
            let fresh7 = *fresh6;
            *fresh6 = *fresh6 + 1;
            if fresh7 >= nb_errors_max {
                av_log(
                    avctx as *mut ::core::ffi::c_void,
                    16 as ::core::ffi::c_int,
                    b"Too many errors when draining, this is a bug. Stop draining and force EOF.\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
                (*avci).draining_done = 1 as ::core::ffi::c_int;
                ret = -((('B' as i32 | ('U' as i32) << 8 as ::core::ffi::c_int
                    | ('G' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('!' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
            }
        } else {
            (*avci).draining_done = 1 as ::core::ffi::c_int;
        }
    }
    if consumed >= (*pkt).size || ret < 0 as ::core::ffi::c_int {
        av_packet_unref(pkt);
    } else {
        (*pkt).data = (*pkt).data.offset(consumed as isize);
        (*pkt).size -= consumed;
        (*pkt).pts = 0x8000000000000000 as ::core::ffi::c_ulonglong as int64_t;
        (*pkt).dts = 0x8000000000000000 as ::core::ffi::c_ulonglong as int64_t;
        if (*codec).caps_internal() as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int == 0
        {
            (*(*avci).last_pkt_props).pts = 0x8000000000000000
                as ::core::ffi::c_ulonglong as int64_t;
            (*(*avci).last_pkt_props).dts = 0x8000000000000000
                as ::core::ffi::c_ulonglong as int64_t;
        }
    }
    return ret;
}
unsafe extern "C" fn detect_colorspace(
    mut c: *mut AVCodecContext,
    mut f: *mut AVFrame,
) -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn fill_frame_props(
    mut avctx: *const AVCodecContext,
    mut frame: *mut AVFrame,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    if (*frame).color_primaries as ::core::ffi::c_uint
        == AVCOL_PRI_UNSPECIFIED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*frame).color_primaries = (*avctx).color_primaries;
    }
    if (*frame).color_trc as ::core::ffi::c_uint
        == AVCOL_TRC_UNSPECIFIED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*frame).color_trc = (*avctx).color_trc;
    }
    if (*frame).colorspace as ::core::ffi::c_uint
        == AVCOL_SPC_UNSPECIFIED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*frame).colorspace = (*avctx).colorspace;
    }
    if (*frame).color_range as ::core::ffi::c_uint
        == AVCOL_RANGE_UNSPECIFIED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*frame).color_range = (*avctx).color_range;
    }
    if (*frame).chroma_location as ::core::ffi::c_uint
        == AVCHROMA_LOC_UNSPECIFIED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*frame).chroma_location = (*avctx).chroma_sample_location;
    }
    if (*frame).alpha_mode as ::core::ffi::c_uint
        == AVALPHA_MODE_UNSPECIFIED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*frame).alpha_mode = (*avctx).alpha_mode;
    }
    if (*avctx).codec_type as ::core::ffi::c_int
        == AVMEDIA_TYPE_VIDEO as ::core::ffi::c_int
    {
        if (*frame).sample_aspect_ratio.num == 0 {
            (*frame).sample_aspect_ratio = (*avctx).sample_aspect_ratio;
        }
        if (*frame).format == AV_PIX_FMT_NONE as ::core::ffi::c_int {
            (*frame).format = (*avctx).pix_fmt as ::core::ffi::c_int;
        }
    } else if (*(*avctx).codec).type_0 as ::core::ffi::c_int
        == AVMEDIA_TYPE_AUDIO as ::core::ffi::c_int
    {
        if (*frame).format == AV_SAMPLE_FMT_NONE as ::core::ffi::c_int {
            (*frame).format = (*avctx).sample_fmt as ::core::ffi::c_int;
        }
        if (*frame).ch_layout.nb_channels == 0 {
            ret = av_channel_layout_copy(
                &raw mut (*frame).ch_layout,
                &raw const (*avctx).ch_layout,
            );
            if ret < 0 as ::core::ffi::c_int {
                return ret;
            }
        }
        if (*frame).sample_rate == 0 {
            (*frame).sample_rate = (*avctx).sample_rate;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn decode_simple_receive_frame(
    mut avctx: *mut AVCodecContext,
    mut frame: *mut AVFrame,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut discarded_samples: int64_t = 0 as int64_t;
    while (*frame).buf[0 as ::core::ffi::c_int as usize].is_null() {
        if discarded_samples > (*avctx).max_samples {
            return -(11 as ::core::ffi::c_int);
        }
        ret = decode_simple_internal(avctx, frame, &raw mut discarded_samples);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_decode_receive_frame_internal(
    mut avctx: *mut AVCodecContext,
    mut frame: *mut AVFrame,
) -> ::core::ffi::c_int {
    let mut avci: *mut AVCodecInternal = (*avctx).internal as *mut AVCodecInternal;
    let mut dc: *mut DecodeContext = decode_ctx(avci);
    let codec: *const FFCodec = ffcodec((*avctx).codec as *const AVCodec)
        as *const FFCodec;
    let mut ret: ::core::ffi::c_int = 0;
    if !(*frame).buf[0 as ::core::ffi::c_int as usize].is_null() {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"!frame->buf[0]\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavcodec/decode.c\0" as *const u8 as *const ::core::ffi::c_char,
            632 as ::core::ffi::c_int,
        );
        abort();
    }
    if (*codec).cb_type() as ::core::ffi::c_int
        == FF_CODEC_CB_TYPE_RECEIVE_FRAME as ::core::ffi::c_int
    {
        loop {
            (*frame).pict_type = (*dc).initial_pict_type;
            (*frame).flags |= (*dc).intra_only_flag;
            ret = (*codec)
                .cb
                .receive_frame
                .expect(
                    "non-null function pointer",
                )(avctx as *mut AVCodecContext, frame as *mut AVFrame);
            if !(ret == 0) {
                break;
            }
            if (*(*avctx).codec).type_0 as ::core::ffi::c_int
                == AVMEDIA_TYPE_AUDIO as ::core::ffi::c_int
            {
                let mut discarded_samples: int64_t = 0 as int64_t;
                ret = discard_samples(avctx, frame, &raw mut discarded_samples);
            }
            if !(ret == -(11 as ::core::ffi::c_int)
                || (*frame).flags & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                    != 0)
            {
                break;
            }
            av_frame_unref(frame);
        }
    } else {
        ret = decode_simple_receive_frame(avctx, frame);
    }
    if ret
        == -((('E' as i32 | ('O' as i32) << 8 as ::core::ffi::c_int
            | ('F' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | (' ' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int)
    {
        (*avci).draining_done = 1 as ::core::ffi::c_int;
    }
    return ret;
}
unsafe extern "C" fn decode_receive_frame_internal(
    mut avctx: *mut AVCodecContext,
    mut frame: *mut AVFrame,
    mut flags: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut avci: *mut AVCodecInternal = (*avctx).internal as *mut AVCodecInternal;
    let mut dc: *mut DecodeContext = decode_ctx(avci);
    let mut ret: ::core::ffi::c_int = 0;
    let mut ok: ::core::ffi::c_int = 0;
    if (*avctx).active_thread_type & 1 as ::core::ffi::c_int != 0 {
        ret = ff_thread_receive_frame(avctx as *mut AVCodecContext, frame, flags);
    } else {
        ret = ff_decode_receive_frame_internal(avctx, frame);
    }
    ok = detect_colorspace(avctx, frame);
    if ok < 0 as ::core::ffi::c_int {
        av_frame_unref(frame);
        return ok;
    }
    if ret == 0 {
        if (*avctx).codec_type as ::core::ffi::c_int
            == AVMEDIA_TYPE_VIDEO as ::core::ffi::c_int
        {
            if (*frame).width == 0 {
                (*frame).width = (*avctx).width;
            }
            if (*frame).height == 0 {
                (*frame).height = (*avctx).height;
            }
        }
        ret = fill_frame_props(avctx, frame);
        if ret < 0 as ::core::ffi::c_int {
            av_frame_unref(frame);
            return ret;
        }
        (*frame).best_effort_timestamp = guess_correct_pts(
            dc,
            (*frame).pts,
            (*frame).pkt_dts,
        );
        if !(!(*frame).private_ref.is_null()
            || (*(*avctx).codec).capabilities
                & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int == 0)
        {
            av_log(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                0 as ::core::ffi::c_int,
                b"Assertion %s failed at %s:%d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"frame->private_ref || !(avctx->codec->capabilities & (1 << 1))\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"src/libavcodec/decode.c\0" as *const u8 as *const ::core::ffi::c_char,
                701 as ::core::ffi::c_int,
            );
            abort();
        }
        if !(*frame).private_ref.is_null() {
            let mut fdd: *mut FrameDecodeData = (*frame).private_ref
                as *mut FrameDecodeData;
            if (*fdd).hwaccel_priv_post_process.is_some() {
                ret = (*fdd)
                    .hwaccel_priv_post_process
                    .expect(
                        "non-null function pointer",
                    )(avctx as *mut ::core::ffi::c_void, frame);
                if ret < 0 as ::core::ffi::c_int {
                    av_frame_unref(frame);
                    return ret;
                }
            }
            if (*fdd).post_process.is_some() {
                ret = (*fdd)
                    .post_process
                    .expect(
                        "non-null function pointer",
                    )(avctx as *mut ::core::ffi::c_void, frame);
                if ret < 0 as ::core::ffi::c_int {
                    av_frame_unref(frame);
                    return ret;
                }
            }
        }
    }
    av_refstruct_unref(&raw mut (*frame).private_ref as *mut ::core::ffi::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn avcodec_send_packet(
    mut avctx: *mut AVCodecContext,
    mut avpkt: *const AVPacket,
) -> ::core::ffi::c_int {
    let mut avci: *mut AVCodecInternal = (*avctx).internal as *mut AVCodecInternal;
    let mut dc: *mut DecodeContext = decode_ctx(avci);
    let mut ret: ::core::ffi::c_int = 0;
    if avcodec_is_open(avctx) == 0
        || av_codec_is_decoder((*avctx).codec as *const AVCodec) == 0
    {
        return -(22 as ::core::ffi::c_int);
    }
    if (*dc).draining_started != 0 {
        return -((('E' as i32 | ('O' as i32) << 8 as ::core::ffi::c_int
            | ('F' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | (' ' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if !avpkt.is_null() && (*avpkt).size == 0 && !(*avpkt).data.is_null() {
        return -(22 as ::core::ffi::c_int);
    }
    if !avpkt.is_null() && (!(*avpkt).data.is_null() || (*avpkt).side_data_elems != 0) {
        if !((*(*avci).buffer_pkt).data.is_null()
            && (*(*avci).buffer_pkt).side_data_elems == 0)
        {
            return -(11 as ::core::ffi::c_int);
        }
        ret = av_packet_ref((*avci).buffer_pkt, avpkt);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
    } else {
        (*dc).draining_started = 1 as ::core::ffi::c_int;
    }
    if (*(*avci).buffer_frame).buf[0 as ::core::ffi::c_int as usize].is_null()
        && (*dc).draining_started == 0
    {
        ret = decode_receive_frame_internal(
            avctx,
            (*avci).buffer_frame,
            0 as ::core::ffi::c_uint,
        );
        if ret < 0 as ::core::ffi::c_int && ret != -(11 as ::core::ffi::c_int)
            && ret
                != -((('E' as i32 | ('O' as i32) << 8 as ::core::ffi::c_int
                    | ('F' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | (' ' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int)
        {
            return ret;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn apply_cropping(
    mut avctx: *mut AVCodecContext,
    mut frame: *mut AVFrame,
) -> ::core::ffi::c_int {
    if (*frame).crop_left
        >= (2147483647 as ::core::ffi::c_int as size_t).wrapping_sub((*frame).crop_right)
        || (*frame).crop_top
            >= (2147483647 as ::core::ffi::c_int as size_t)
                .wrapping_sub((*frame).crop_bottom)
        || (*frame).crop_left.wrapping_add((*frame).crop_right)
            >= (*frame).width as size_t
        || (*frame).crop_top.wrapping_add((*frame).crop_bottom)
            >= (*frame).height as size_t
    {
        av_log(
            avctx as *mut ::core::ffi::c_void,
            24 as ::core::ffi::c_int,
            b"Invalid cropping information set by a decoder: %zu/%zu/%zu/%zu (frame size %dx%d). This is a bug, please report it\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            (*frame).crop_left,
            (*frame).crop_right,
            (*frame).crop_top,
            (*frame).crop_bottom,
            (*frame).width,
            (*frame).height,
        );
        (*frame).crop_left = 0 as size_t;
        (*frame).crop_right = 0 as size_t;
        (*frame).crop_top = 0 as size_t;
        (*frame).crop_bottom = 0 as size_t;
        return 0 as ::core::ffi::c_int;
    }
    if (*avctx).apply_cropping == 0 {
        return 0 as ::core::ffi::c_int;
    }
    return av_frame_apply_cropping(
        frame,
        if (*avctx).flags & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
            AV_FRAME_CROP_UNALIGNED as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
}
unsafe extern "C" fn frame_validate(
    mut avctx: *mut AVCodecContext,
    mut frame: *mut AVFrame,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    if !((*frame).buf[0 as ::core::ffi::c_int as usize].is_null()
        || (*frame).format < 0 as ::core::ffi::c_int)
    {
        match (*avctx).codec_type as ::core::ffi::c_int {
            0 => {
                if (*frame).width <= 0 as ::core::ffi::c_int
                    || (*frame).height <= 0 as ::core::ffi::c_int
                {
                    current_block = 8647610728685033056;
                } else {
                    current_block = 11650488183268122163;
                }
            }
            1 => {
                if av_channel_layout_check(&raw mut (*frame).ch_layout) == 0
                    || (*frame).sample_rate <= 0 as ::core::ffi::c_int
                {
                    current_block = 8647610728685033056;
                } else {
                    current_block = 11650488183268122163;
                }
            }
            _ => {
                if 0 as ::core::ffi::c_int == 0 {
                    av_log(
                        ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        0 as ::core::ffi::c_int,
                        b"Assertion %s failed at %s:%d\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"0\0" as *const u8 as *const ::core::ffi::c_char,
                        b"src/libavcodec/decode.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        807 as ::core::ffi::c_int,
                    );
                    abort();
                }
                current_block = 11650488183268122163;
            }
        }
        match current_block {
            8647610728685033056 => {}
            _ => return 0 as ::core::ffi::c_int,
        }
    }
    av_log(
        avctx as *mut ::core::ffi::c_void,
        16 as ::core::ffi::c_int,
        b"An invalid frame was output by a decoder. This is a bug, please report it.\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    return -((('B' as i32 | ('U' as i32) << 8 as ::core::ffi::c_int
        | ('G' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
        | ('!' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
        as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ff_decode_receive_frame(
    mut avctx: *mut AVCodecContext,
    mut frame: *mut AVFrame,
    mut flags: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut avci: *mut AVCodecInternal = (*avctx).internal as *mut AVCodecInternal;
    let mut ret: ::core::ffi::c_int = 0;
    if !(*(*avci).buffer_frame).buf[0 as ::core::ffi::c_int as usize].is_null() {
        av_frame_move_ref(frame, (*avci).buffer_frame);
    } else {
        ret = decode_receive_frame_internal(avctx, frame, flags);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
    }
    ret = frame_validate(avctx, frame);
    if !(ret < 0 as ::core::ffi::c_int) {
        if (*avctx).codec_type as ::core::ffi::c_int
            == AVMEDIA_TYPE_VIDEO as ::core::ffi::c_int
        {
            ret = apply_cropping(avctx, frame);
            if ret < 0 as ::core::ffi::c_int {
                current_block = 5113661372009179135;
            } else {
                current_block = 2868539653012386629;
            }
        } else {
            current_block = 2868539653012386629;
        }
        match current_block {
            5113661372009179135 => {}
            _ => {
                (*avctx).frame_num += 1;
                return 0 as ::core::ffi::c_int;
            }
        }
    }
    av_frame_unref(frame);
    return ret;
}
unsafe extern "C" fn get_subtitle_defaults(mut sub: *mut AVSubtitle) {
    memset(
        sub as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<AVSubtitle>() as size_t,
    );
    (*sub).pts = 0x8000000000000000 as ::core::ffi::c_ulonglong as int64_t;
}
unsafe extern "C" fn recode_subtitle(
    mut avctx: *mut AVCodecContext,
    mut outpkt: *mut *const AVPacket,
    mut inpkt: *const AVPacket,
    mut buf_pkt: *mut AVPacket,
) -> ::core::ffi::c_int {
    let mut cd: iconv_t = -(1 as ::core::ffi::c_int) as iconv_t;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut inb: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut outb: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut inl: size_t = 0;
    let mut outl: size_t = 0;
    if (*avctx).sub_charenc_mode != 1 as ::core::ffi::c_int
        || (*inpkt).size == 0 as ::core::ffi::c_int
    {
        *outpkt = inpkt;
        return 0 as ::core::ffi::c_int;
    }
    inb = (*inpkt).data as *mut ::core::ffi::c_char;
    inl = (*inpkt).size as size_t;
    if inl
        >= (2147483647 as ::core::ffi::c_int / 4 as ::core::ffi::c_int
            - 64 as ::core::ffi::c_int) as size_t
    {
        av_log(
            avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"Subtitles packet is too big for recoding\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(34 as ::core::ffi::c_int);
    }
    cd = iconv_open(
        b"UTF-8\0" as *const u8 as *const ::core::ffi::c_char,
        (*avctx).sub_charenc,
    );
    if !(cd != -(1 as ::core::ffi::c_int) as iconv_t) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"cd != (iconv_t)-1\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavcodec/decode.c\0" as *const u8 as *const ::core::ffi::c_char,
            880 as ::core::ffi::c_int,
        );
        abort();
    }
    ret = av_new_packet(buf_pkt, inl.wrapping_mul(4 as size_t) as ::core::ffi::c_int);
    if !(ret < 0 as ::core::ffi::c_int) {
        ret = av_packet_copy_props(buf_pkt, inpkt);
        if !(ret < 0 as ::core::ffi::c_int) {
            outb = (*buf_pkt).data as *mut ::core::ffi::c_char;
            outl = (*buf_pkt).size as size_t;
            if iconv(cd, &raw mut inb, &raw mut inl, &raw mut outb, &raw mut outl)
                == -(1 as ::core::ffi::c_int) as size_t
                || iconv(
                    cd,
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                    ::core::ptr::null_mut::<size_t>(),
                    &raw mut outb,
                    &raw mut outl,
                ) == -(1 as ::core::ffi::c_int) as size_t
                || outl >= (*buf_pkt).size as size_t || inl != 0 as size_t
            {
                ret = if -*__errno_location() > -(1 as ::core::ffi::c_int) {
                    -(1 as ::core::ffi::c_int)
                } else {
                    -*__errno_location()
                };
                av_log(
                    avctx as *mut ::core::ffi::c_void,
                    16 as ::core::ffi::c_int,
                    b"Unable to recode subtitle event \"%s\" from %s to UTF-8\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    (*inpkt).data,
                    (*avctx).sub_charenc,
                );
            } else {
                (*buf_pkt).size = ((*buf_pkt).size as size_t).wrapping_sub(outl)
                    as ::core::ffi::c_int as ::core::ffi::c_int;
                memset(
                    (*buf_pkt).data.offset((*buf_pkt).size as isize)
                        as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    outl as size_t,
                );
                *outpkt = buf_pkt;
                ret = 0 as ::core::ffi::c_int;
            }
        }
    }
    if ret < 0 as ::core::ffi::c_int {
        av_packet_unref(buf_pkt);
    }
    if cd != -(1 as ::core::ffi::c_int) as iconv_t {
        iconv_close(cd);
    }
    return ret;
}
unsafe extern "C" fn utf8_check(mut str: *const uint8_t) -> ::core::ffi::c_int {
    let mut byte: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut codepoint: uint32_t = 0;
    let mut min: uint32_t = 0;
    while *str != 0 {
        byte = str;
        let fresh4 = byte;
        byte = byte.offset(1);
        codepoint = *fresh4 as uint32_t;
        let mut top: uint32_t = (codepoint & 128 as uint32_t) >> 1 as ::core::ffi::c_int;
        if codepoint & 0xc0 as uint32_t == 0x80 as uint32_t
            || codepoint >= 0xfe as uint32_t
        {
            return 0 as ::core::ffi::c_int;
        }
        while codepoint & top != 0 {
            let fresh5 = byte;
            byte = byte.offset(1);
            let mut tmp: ::core::ffi::c_uint = (*fresh5 as ::core::ffi::c_int
                - 128 as ::core::ffi::c_int) as ::core::ffi::c_uint;
            if tmp >> 6 as ::core::ffi::c_int != 0 {
                return 0 as ::core::ffi::c_int;
            }
            codepoint = ((codepoint as ::core::ffi::c_uint) << 6 as ::core::ffi::c_int)
                .wrapping_add(tmp) as uint32_t;
            top <<= 5 as ::core::ffi::c_int;
        }
        codepoint &= (top << 1 as ::core::ffi::c_int).wrapping_sub(1 as uint32_t);
        min = (if byte.offset_from(str) as ::core::ffi::c_int == 1 as ::core::ffi::c_int
        {
            0 as ::core::ffi::c_int
        } else if byte.offset_from(str) as ::core::ffi::c_int == 2 as ::core::ffi::c_int
        {
            0x80 as ::core::ffi::c_int
        } else {
            (1 as ::core::ffi::c_int)
                << 5 as ::core::ffi::c_int * byte.offset_from(str) as ::core::ffi::c_int
                    - 4 as ::core::ffi::c_int
        }) as uint32_t;
        if codepoint < min || codepoint >= 0x110000 as ::core::ffi::c_int as uint32_t
            || codepoint == 0xfffe as uint32_t
            || codepoint >= 0xd800 as uint32_t && codepoint <= 0xdfff as uint32_t
        {
            return 0 as ::core::ffi::c_int;
        }
        str = byte;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn avcodec_decode_subtitle2(
    mut avctx: *mut AVCodecContext,
    mut sub: *mut AVSubtitle,
    mut got_sub_ptr: *mut ::core::ffi::c_int,
    mut avpkt: *const AVPacket,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*avpkt).data.is_null() && (*avpkt).size != 0 {
        av_log(
            avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"invalid packet: NULL data, size != 0\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(22 as ::core::ffi::c_int);
    }
    if (*avctx).codec.is_null() {
        return -(22 as ::core::ffi::c_int);
    }
    if (*ffcodec((*avctx).codec as *const AVCodec)).cb_type() as ::core::ffi::c_int
        != FF_CODEC_CB_TYPE_DECODE_SUB as ::core::ffi::c_int
    {
        av_log(
            avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"Codec not subtitle decoder\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(22 as ::core::ffi::c_int);
    }
    *got_sub_ptr = 0 as ::core::ffi::c_int;
    get_subtitle_defaults(sub);
    if (*(*avctx).codec).capabilities
        & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0 || (*avpkt).size != 0
    {
        let mut avci: *mut AVCodecInternal = (*avctx).internal as *mut AVCodecInternal;
        let mut pkt: *const AVPacket = ::core::ptr::null::<AVPacket>();
        ret = recode_subtitle(avctx, &raw mut pkt, avpkt, (*avci).buffer_pkt);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        if (*avctx).pkt_timebase.num != 0
            && (*avpkt).pts != 0x8000000000000000 as ::core::ffi::c_ulonglong as int64_t
        {
            (*sub).pts = av_rescale_q(
                (*avpkt).pts,
                (*avctx).pkt_timebase,
                AVRational {
                    num: 1 as ::core::ffi::c_int,
                    den: 1000000 as ::core::ffi::c_int,
                },
            );
        }
        ret = (*ffcodec((*avctx).codec as *const AVCodec))
            .cb
            .decode_sub
            .expect(
                "non-null function pointer",
            )(
            avctx as *mut AVCodecContext,
            sub as *mut AVSubtitle,
            got_sub_ptr,
            pkt as *const AVPacket,
        );
        if pkt == (*avci).buffer_pkt as *const AVPacket {
            av_packet_unref((*avci).buffer_pkt);
        }
        if ret < 0 as ::core::ffi::c_int {
            *got_sub_ptr = 0 as ::core::ffi::c_int;
            avsubtitle_free(sub);
            return ret;
        }
        if (*sub).num_rects != 0 && (*sub).end_display_time == 0
            && (*avpkt).duration != 0 && (*avctx).pkt_timebase.num != 0
        {
            let mut ms: AVRational = AVRational {
                num: 1 as ::core::ffi::c_int,
                den: 1000 as ::core::ffi::c_int,
            };
            (*sub).end_display_time = av_rescale_q(
                (*avpkt).duration,
                (*avctx).pkt_timebase,
                ms,
            ) as uint32_t;
        }
        if (*(*avctx).codec_descriptor).props
            & (1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int != 0
        {
            (*sub).format = 0 as uint16_t;
        } else if (*(*avctx).codec_descriptor).props
            & (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int != 0
        {
            (*sub).format = 1 as uint16_t;
        }
        let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while i < (*sub).num_rects {
            if (*avctx).sub_charenc_mode != 2 as ::core::ffi::c_int
                && !(**(*sub).rects.offset(i as isize)).ass.is_null()
                && utf8_check((**(*sub).rects.offset(i as isize)).ass as *const uint8_t)
                    == 0
            {
                av_log(
                    avctx as *mut ::core::ffi::c_void,
                    16 as ::core::ffi::c_int,
                    b"Invalid UTF-8 in decoded subtitles text; maybe missing -sub_charenc option\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
                avsubtitle_free(sub);
                *got_sub_ptr = 0 as ::core::ffi::c_int;
                return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                    | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
            }
            i = i.wrapping_add(1);
        }
        if *got_sub_ptr != 0 {
            (*avctx).frame_num += 1;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn avcodec_default_get_format(
    mut avctx: *mut AVCodecContext,
    mut fmt: *const AVPixelFormat,
) -> AVPixelFormat {
    let mut config: *const AVCodecHWConfig = ::core::ptr::null::<AVCodecHWConfig>();
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    if !(*avctx).hw_device_ctx.is_null()
        && !(*ffcodec((*avctx).codec as *const AVCodec)).hw_configs.is_null()
    {
        let mut device_ctx: *mut AVHWDeviceContext = (*(*avctx).hw_device_ctx).data
            as *mut AVHWDeviceContext;
        i = 0 as ::core::ffi::c_int;
        loop {
            config = &raw const (**(*(ffcodec
                as unsafe extern "C" fn(
                    *const AVCodec,
                ) -> *const FFCodec)((*avctx).codec as *const AVCodec))
                .hw_configs
                .offset(i as isize))
                .public;
            if config.is_null() {
                break;
            }
            if !((*config).methods
                & AV_CODEC_HW_CONFIG_METHOD_HW_DEVICE_CTX as ::core::ffi::c_int == 0)
            {
                if !((*device_ctx).type_0 as ::core::ffi::c_uint
                    != (*config).device_type as ::core::ffi::c_uint)
                {
                    n = 0 as ::core::ffi::c_int;
                    while *fmt.offset(n as isize) as ::core::ffi::c_int
                        != AV_PIX_FMT_NONE as ::core::ffi::c_int
                    {
                        if (*config).pix_fmt as ::core::ffi::c_int
                            == *fmt.offset(n as isize) as ::core::ffi::c_int
                        {
                            return *fmt.offset(n as isize);
                        }
                        n += 1;
                    }
                }
            }
            i += 1;
        }
    }
    n = 0 as ::core::ffi::c_int;
    while *fmt.offset(n as isize) as ::core::ffi::c_int
        != AV_PIX_FMT_NONE as ::core::ffi::c_int
    {
        let mut desc: *const AVPixFmtDescriptor = av_pix_fmt_desc_get(
            *fmt.offset(n as isize),
        );
        if (*desc).flags
            & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint64_t == 0
        {
            return *fmt.offset(n as isize);
        }
        n += 1;
    }
    n = 0 as ::core::ffi::c_int;
    while *fmt.offset(n as isize) as ::core::ffi::c_int
        != AV_PIX_FMT_NONE as ::core::ffi::c_int
    {
        i = 0 as ::core::ffi::c_int;
        loop {
            config = avcodec_get_hw_config((*avctx).codec as *const AVCodec, i);
            if config.is_null() {
                break;
            }
            if (*config).pix_fmt as ::core::ffi::c_int
                == *fmt.offset(n as isize) as ::core::ffi::c_int
            {
                break;
            }
            i += 1;
        }
        if config.is_null() {
            return *fmt.offset(n as isize);
        }
        if (*config).methods & AV_CODEC_HW_CONFIG_METHOD_INTERNAL as ::core::ffi::c_int
            != 0
        {
            return *fmt.offset(n as isize);
        }
        n += 1;
    }
    return AV_PIX_FMT_NONE;
}
#[no_mangle]
pub unsafe extern "C" fn ff_decode_get_hw_frames_ctx(
    mut avctx: *mut AVCodecContext,
    mut dev_type: AVHWDeviceType,
) -> ::core::ffi::c_int {
    let mut device_ctx: *mut AVHWDeviceContext = ::core::ptr::null_mut::<
        AVHWDeviceContext,
    >();
    let mut frames_ctx: *mut AVHWFramesContext = ::core::ptr::null_mut::<
        AVHWFramesContext,
    >();
    let mut ret: ::core::ffi::c_int = 0;
    if (*avctx).hwaccel.is_null() {
        return -(38 as ::core::ffi::c_int);
    }
    if !(*avctx).hw_frames_ctx.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if (*avctx).hw_device_ctx.is_null() {
        av_log(
            avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"A hardware frames or device context is required for hardware accelerated decoding.\n\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return -(22 as ::core::ffi::c_int);
    }
    device_ctx = (*(*avctx).hw_device_ctx).data as *mut AVHWDeviceContext;
    if (*device_ctx).type_0 as ::core::ffi::c_uint != dev_type as ::core::ffi::c_uint {
        av_log(
            avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"Device type %s expected for hardware decoding, but got %s.\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            av_hwdevice_get_type_name(dev_type),
            av_hwdevice_get_type_name((*device_ctx).type_0),
        );
        return -(22 as ::core::ffi::c_int);
    }
    ret = avcodec_get_hw_frames_parameters(
        avctx,
        (*avctx).hw_device_ctx,
        (*(*avctx).hwaccel).pix_fmt,
        &raw mut (*avctx).hw_frames_ctx,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    frames_ctx = (*(*avctx).hw_frames_ctx).data as *mut AVHWFramesContext;
    if (*frames_ctx).initial_pool_size != 0 {
        (*frames_ctx).initial_pool_size += 3 as ::core::ffi::c_int;
    }
    ret = av_hwframe_ctx_init((*avctx).hw_frames_ctx);
    if ret < 0 as ::core::ffi::c_int {
        av_buffer_unref(&raw mut (*avctx).hw_frames_ctx);
        return ret;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn avcodec_get_hw_frames_parameters(
    mut avctx: *mut AVCodecContext,
    mut device_ref: *mut AVBufferRef,
    mut hw_pix_fmt: AVPixelFormat,
    mut out_frames_ref: *mut *mut AVBufferRef,
) -> ::core::ffi::c_int {
    let mut frames_ref: *mut AVBufferRef = ::core::ptr::null_mut::<AVBufferRef>();
    let mut hw_config: *const AVCodecHWConfigInternal = ::core::ptr::null::<
        AVCodecHWConfigInternal,
    >();
    let mut hwa: *const FFHWAccel = ::core::ptr::null::<FFHWAccel>();
    let mut i: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut clean_priv_data: bool = 0 as ::core::ffi::c_int != 0;
    i = 0 as ::core::ffi::c_int;
    loop {
        hw_config = *(*ffcodec((*avctx).codec as *const AVCodec))
            .hw_configs
            .offset(i as isize) as *const AVCodecHWConfigInternal;
        if hw_config.is_null() {
            return -(2 as ::core::ffi::c_int);
        }
        if (*hw_config).public.pix_fmt as ::core::ffi::c_int
            == hw_pix_fmt as ::core::ffi::c_int
        {
            break;
        }
        i += 1;
    }
    hwa = (*hw_config).hwaccel as *const FFHWAccel;
    if hwa.is_null() || (*hwa).frame_params.is_none() {
        return -(2 as ::core::ffi::c_int);
    }
    frames_ref = av_hwframe_ctx_alloc(device_ref);
    if frames_ref.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    if (*(*avctx).internal).hwaccel_priv_data.is_null() {
        (*(*avctx).internal).hwaccel_priv_data = av_mallocz(
            (*hwa).priv_data_size as size_t,
        );
        if (*(*avctx).internal).hwaccel_priv_data.is_null() {
            av_buffer_unref(&raw mut frames_ref);
            return -(12 as ::core::ffi::c_int);
        }
        clean_priv_data = 1 as ::core::ffi::c_int != 0;
    }
    ret = (*hwa).frame_params.expect("non-null function pointer")(avctx, frames_ref);
    if ret >= 0 as ::core::ffi::c_int {
        let mut frames_ctx: *mut AVHWFramesContext = (*frames_ref).data
            as *mut AVHWFramesContext;
        if (*frames_ctx).initial_pool_size != 0 {
            if (*avctx).extra_hw_frames > 0 as ::core::ffi::c_int {
                (*frames_ctx).initial_pool_size += (*avctx).extra_hw_frames;
            }
            if (*avctx).active_thread_type & 1 as ::core::ffi::c_int != 0 {
                (*frames_ctx).initial_pool_size += (*avctx).thread_count;
            }
        }
        *out_frames_ref = frames_ref;
    } else {
        if clean_priv_data {
            av_freep(
                &raw mut (*(*avctx).internal).hwaccel_priv_data
                    as *mut ::core::ffi::c_void,
            );
        }
        av_buffer_unref(&raw mut frames_ref);
    }
    return ret;
}
unsafe extern "C" fn hwaccel_init(
    mut avctx: *mut AVCodecContext,
    mut hwaccel: *const FFHWAccel,
) -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int = 0;
    if (*hwaccel).p.capabilities & 0x200 as ::core::ffi::c_int != 0
        && (*avctx).strict_std_compliance > -(2 as ::core::ffi::c_int)
    {
        av_log(
            avctx as *mut ::core::ffi::c_void,
            24 as ::core::ffi::c_int,
            b"Ignoring experimental hwaccel: %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            (*hwaccel).p.name,
        );
        return -((('P' as i32 | ('A' as i32) << 8 as ::core::ffi::c_int
            | ('W' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('E' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if (*(*avctx).internal).hwaccel_priv_data.is_null() && (*hwaccel).priv_data_size != 0
    {
        (*(*avctx).internal).hwaccel_priv_data = av_mallocz(
            (*hwaccel).priv_data_size as size_t,
        );
        if (*(*avctx).internal).hwaccel_priv_data.is_null() {
            return -(12 as ::core::ffi::c_int);
        }
    }
    (*avctx).hwaccel = &raw const (*hwaccel).p as *const AVHWAccel;
    if (*hwaccel).init.is_some() {
        err = (*hwaccel).init.expect("non-null function pointer")(avctx);
        if err < 0 as ::core::ffi::c_int {
            av_log(
                avctx as *mut ::core::ffi::c_void,
                16 as ::core::ffi::c_int,
                b"Failed setup for format %s: hwaccel initialisation returned error.\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                av_get_pix_fmt_name((*hwaccel).p.pix_fmt),
            );
            av_freep(
                &raw mut (*(*avctx).internal).hwaccel_priv_data
                    as *mut ::core::ffi::c_void,
            );
            (*avctx).hwaccel = ::core::ptr::null::<AVHWAccel>();
            return err;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn av_frame_side_data_get(
    mut sd: *const *mut AVFrameSideData,
    nb_sd: ::core::ffi::c_int,
    mut type_0: AVFrameSideDataType,
) -> *const AVFrameSideData {
    return av_frame_side_data_get_c(sd as *const *const AVFrameSideData, nb_sd, type_0);
}
#[no_mangle]
pub unsafe extern "C" fn ff_hwaccel_uninit(mut avctx: *mut AVCodecContext) {
    if !(*avctx).hwaccel.is_null()
        && (*ffhwaccel((*avctx).hwaccel as *const AVHWAccel)).uninit.is_some()
    {
        (*ffhwaccel((*avctx).hwaccel as *const AVHWAccel))
            .uninit
            .expect("non-null function pointer")(avctx);
    }
    av_freep(
        &raw mut (*(*avctx).internal).hwaccel_priv_data as *mut ::core::ffi::c_void,
    );
    (*avctx).hwaccel = ::core::ptr::null::<AVHWAccel>();
    av_buffer_unref(&raw mut (*avctx).hw_frames_ctx);
}
#[no_mangle]
pub unsafe extern "C" fn ff_get_format(
    mut avctx: *mut AVCodecContext,
    mut fmt: *const AVPixelFormat,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut desc: *const AVPixFmtDescriptor = ::core::ptr::null::<AVPixFmtDescriptor>();
    let mut choices: *mut AVPixelFormat = ::core::ptr::null_mut::<AVPixelFormat>();
    let mut ret: AVPixelFormat = AV_PIX_FMT_YUV420P;
    let mut user_choice: AVPixelFormat = AV_PIX_FMT_YUV420P;
    let mut hw_config: *const AVCodecHWConfigInternal = ::core::ptr::null::<
        AVCodecHWConfigInternal,
    >();
    let mut config: *const AVCodecHWConfig = ::core::ptr::null::<AVCodecHWConfig>();
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    n = 0 as ::core::ffi::c_int;
    while *fmt.offset(n as isize) as ::core::ffi::c_int
        != AV_PIX_FMT_NONE as ::core::ffi::c_int
    {
        n += 1;
    }
    if !(n >= 1 as ::core::ffi::c_int) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"n >= 1\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavcodec/decode.c\0" as *const u8 as *const ::core::ffi::c_char,
            1241 as ::core::ffi::c_int,
        );
        abort();
    }
    desc = av_pix_fmt_desc_get(*fmt.offset((n - 1 as ::core::ffi::c_int) as isize));
    if !((*desc).flags
        & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint64_t != 0)
    {
        (*avctx).sw_pix_fmt = *fmt.offset((n - 1 as ::core::ffi::c_int) as isize);
    }
    choices = av_memdup(
        fmt as *const ::core::ffi::c_void,
        ((n + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<AVPixelFormat>() as size_t),
    ) as *mut AVPixelFormat;
    if choices.is_null() {
        return AV_PIX_FMT_NONE as ::core::ffi::c_int;
    }
    loop {
        ff_hwaccel_uninit(avctx);
        user_choice = (*avctx)
            .get_format
            .expect("non-null function pointer")(avctx as *mut AVCodecContext, choices);
        if user_choice as ::core::ffi::c_int == AV_PIX_FMT_NONE as ::core::ffi::c_int {
            ret = AV_PIX_FMT_NONE;
            break;
        } else {
            desc = av_pix_fmt_desc_get(user_choice);
            if desc.is_null() {
                av_log(
                    avctx as *mut ::core::ffi::c_void,
                    16 as ::core::ffi::c_int,
                    b"Invalid format returned by get_format() callback.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                ret = AV_PIX_FMT_NONE;
                break;
            } else {
                av_log(
                    avctx as *mut ::core::ffi::c_void,
                    48 as ::core::ffi::c_int,
                    b"Format %s chosen by get_format().\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*desc).name,
                );
                i = 0 as ::core::ffi::c_int;
                while i < n {
                    if *choices.offset(i as isize) as ::core::ffi::c_int
                        == user_choice as ::core::ffi::c_int
                    {
                        break;
                    }
                    i += 1;
                }
                if i == n {
                    av_log(
                        avctx as *mut ::core::ffi::c_void,
                        16 as ::core::ffi::c_int,
                        b"Invalid return from get_format(): %s not in possible list.\n\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        (*desc).name,
                    );
                    ret = AV_PIX_FMT_NONE;
                    break;
                } else {
                    if !(*ffcodec((*avctx).codec as *const AVCodec)).hw_configs.is_null()
                    {
                        i = 0 as ::core::ffi::c_int;
                        loop {
                            hw_config = *(*ffcodec((*avctx).codec as *const AVCodec))
                                .hw_configs
                                .offset(i as isize) as *const AVCodecHWConfigInternal;
                            if hw_config.is_null() {
                                break;
                            }
                            if (*hw_config).public.pix_fmt as ::core::ffi::c_int
                                == user_choice as ::core::ffi::c_int
                            {
                                break;
                            }
                            i += 1;
                        }
                    } else {
                        hw_config = ::core::ptr::null::<AVCodecHWConfigInternal>();
                    }
                    if hw_config.is_null() {
                        ret = user_choice;
                        break;
                    } else {
                        config = &raw const (*hw_config).public;
                        if (*config).methods
                            & AV_CODEC_HW_CONFIG_METHOD_HW_FRAMES_CTX
                                as ::core::ffi::c_int != 0
                            && !(*avctx).hw_frames_ctx.is_null()
                        {
                            let mut frames_ctx: *const AVHWFramesContext = (*(*avctx)
                                .hw_frames_ctx)
                                .data as *mut AVHWFramesContext;
                            if (*frames_ctx).format as ::core::ffi::c_int
                                != user_choice as ::core::ffi::c_int
                            {
                                av_log(
                                    avctx as *mut ::core::ffi::c_void,
                                    16 as ::core::ffi::c_int,
                                    b"Invalid setup for format %s: does not match the format of the provided frames context.\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    (*desc).name,
                                );
                                current_block = 10204190573580270709;
                            } else {
                                current_block = 13763002826403452995;
                            }
                        } else if (*config).methods
                            & AV_CODEC_HW_CONFIG_METHOD_HW_DEVICE_CTX
                                as ::core::ffi::c_int != 0
                            && !(*avctx).hw_device_ctx.is_null()
                        {
                            let mut device_ctx: *const AVHWDeviceContext = (*(*avctx)
                                .hw_device_ctx)
                                .data as *mut AVHWDeviceContext;
                            if (*device_ctx).type_0 as ::core::ffi::c_uint
                                != (*config).device_type as ::core::ffi::c_uint
                            {
                                av_log(
                                    avctx as *mut ::core::ffi::c_void,
                                    16 as ::core::ffi::c_int,
                                    b"Invalid setup for format %s: does not match the type of the provided device context.\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    (*desc).name,
                                );
                                current_block = 10204190573580270709;
                            } else {
                                current_block = 13763002826403452995;
                            }
                        } else if (*config).methods
                            & AV_CODEC_HW_CONFIG_METHOD_INTERNAL as ::core::ffi::c_int
                            != 0
                        {
                            current_block = 13763002826403452995;
                        } else if (*config).methods
                            & AV_CODEC_HW_CONFIG_METHOD_AD_HOC as ::core::ffi::c_int != 0
                        {
                            current_block = 13763002826403452995;
                        } else {
                            av_log(
                                avctx as *mut ::core::ffi::c_void,
                                16 as ::core::ffi::c_int,
                                b"Invalid setup for format %s: missing configuration.\n\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                                (*desc).name,
                            );
                            current_block = 10204190573580270709;
                        }
                        match current_block {
                            13763002826403452995 => {
                                if !(*hw_config).hwaccel.is_null() {
                                    av_log(
                                        avctx as *mut ::core::ffi::c_void,
                                        48 as ::core::ffi::c_int,
                                        b"Format %s requires hwaccel %s initialisation.\n\0"
                                            as *const u8 as *const ::core::ffi::c_char,
                                        (*desc).name,
                                        (*(*hw_config).hwaccel).p.name,
                                    );
                                    err = hwaccel_init(
                                        avctx,
                                        (*hw_config).hwaccel as *const FFHWAccel,
                                    );
                                    if err < 0 as ::core::ffi::c_int {
                                        current_block = 10204190573580270709;
                                    } else {
                                        current_block = 5159818223158340697;
                                    }
                                } else {
                                    current_block = 5159818223158340697;
                                }
                                match current_block {
                                    10204190573580270709 => {}
                                    _ => {
                                        ret = user_choice;
                                        break;
                                    }
                                }
                            }
                            _ => {}
                        }
                        av_log(
                            avctx as *mut ::core::ffi::c_void,
                            48 as ::core::ffi::c_int,
                            b"Format %s not usable, retrying get_format() without it.\n\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            (*desc).name,
                        );
                        i = 0 as ::core::ffi::c_int;
                        while i < n {
                            if *choices.offset(i as isize) as ::core::ffi::c_int
                                == user_choice as ::core::ffi::c_int
                            {
                                break;
                            }
                            i += 1;
                        }
                        while (i + 1 as ::core::ffi::c_int) < n {
                            *choices.offset(i as isize) = *choices
                                .offset((i + 1 as ::core::ffi::c_int) as isize);
                            i += 1;
                        }
                        n -= 1;
                    }
                }
            }
        }
    }
    if (ret as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        ff_hwaccel_uninit(avctx);
    }
    av_freep(&raw mut choices as *mut ::core::ffi::c_void);
    return ret as ::core::ffi::c_int;
}
unsafe extern "C" fn packet_side_data_get(
    mut sd: *const AVPacketSideData,
    mut nb_sd: ::core::ffi::c_int,
    mut type_0: AVPacketSideDataType,
) -> *const AVPacketSideData {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < nb_sd {
        if (*sd.offset(i as isize)).type_0 as ::core::ffi::c_uint
            == type_0 as ::core::ffi::c_uint
        {
            return sd.offset(i as isize) as *const AVPacketSideData;
        }
        i += 1;
    }
    return ::core::ptr::null::<AVPacketSideData>();
}
#[no_mangle]
pub unsafe extern "C" fn ff_get_coded_side_data(
    mut avctx: *const AVCodecContext,
    mut type_0: AVPacketSideDataType,
) -> *const AVPacketSideData {
    return packet_side_data_get(
        (*avctx).coded_side_data,
        (*avctx).nb_coded_side_data,
        type_0,
    );
}
unsafe extern "C" fn side_data_stereo3d_merge(
    mut sd_frame: *mut AVFrameSideData,
    mut sd_pkt: *const AVPacketSideData,
) -> ::core::ffi::c_int {
    let mut src: *const AVStereo3D = ::core::ptr::null::<AVStereo3D>();
    let mut dst: *mut AVStereo3D = ::core::ptr::null_mut::<AVStereo3D>();
    let mut ret: ::core::ffi::c_int = 0;
    ret = av_buffer_make_writable(&raw mut (*sd_frame).buf);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    (*sd_frame).data = (*(*sd_frame).buf).data;
    dst = (*sd_frame).data as *mut AVStereo3D;
    src = (*sd_pkt).data as *const AVStereo3D;
    if (*dst).type_0 as ::core::ffi::c_uint
        == AV_STEREO3D_UNSPEC as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*dst).type_0 = (*src).type_0;
    }
    if (*dst).view as ::core::ffi::c_uint
        == AV_STEREO3D_VIEW_UNSPEC as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*dst).view = (*src).view;
    }
    if (*dst).primary_eye as ::core::ffi::c_uint
        == AV_PRIMARY_EYE_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*dst).primary_eye = (*src).primary_eye;
    }
    if (*dst).baseline == 0 {
        (*dst).baseline = (*src).baseline;
    }
    if (*dst).horizontal_disparity_adjustment.num == 0 {
        (*dst).horizontal_disparity_adjustment = (*src).horizontal_disparity_adjustment;
    }
    if (*dst).horizontal_field_of_view.num == 0 {
        (*dst).horizontal_field_of_view = (*src).horizontal_field_of_view;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn side_data_exif_parse(
    mut dst: *mut AVFrame,
    mut sd_pkt: *const AVPacketSideData,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut ifd: AVExifMetadata = AVExifMetadata {
        entries: ::core::ptr::null_mut::<AVExifEntry>(),
        count: 0,
        size: 0,
    };
    let mut entry: *mut AVExifEntry = ::core::ptr::null_mut::<AVExifEntry>();
    let mut buf: *mut AVBufferRef = ::core::ptr::null_mut::<AVBufferRef>();
    let mut sd_frame: *mut AVFrameSideData = ::core::ptr::null_mut::<AVFrameSideData>();
    let mut ret: ::core::ffi::c_int = 0;
    ret = av_exif_parse_buffer(
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        (*sd_pkt).data,
        (*sd_pkt).size,
        &raw mut ifd,
        AV_EXIF_TIFF_HEADER,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    ret = av_exif_get_entry(
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        &raw mut ifd,
        av_exif_get_tag_id(b"Orientation\0" as *const u8 as *const ::core::ffi::c_char)
            as uint16_t,
        0 as ::core::ffi::c_int,
        &raw mut entry,
    );
    if !(ret < 0 as ::core::ffi::c_int) {
        if entry.is_null() {
            ret = av_exif_ifd_to_dict(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                &raw mut ifd,
                &raw mut (*dst).metadata,
            );
            if !(ret < 0 as ::core::ffi::c_int) {
                sd_frame = av_frame_side_data_new(
                    &raw mut (*dst).side_data,
                    &raw mut (*dst).nb_side_data,
                    AV_FRAME_DATA_EXIF,
                    (*sd_pkt).size,
                    0 as ::core::ffi::c_uint,
                );
                if !sd_frame.is_null() {
                    memcpy(
                        (*sd_frame).data as *mut ::core::ffi::c_void,
                        (*sd_pkt).data as *const ::core::ffi::c_void,
                        (*sd_pkt).size as size_t,
                    );
                }
                ret = if !sd_frame.is_null() {
                    0 as ::core::ffi::c_int
                } else {
                    -(12 as ::core::ffi::c_int)
                };
            }
        } else if (*entry).count <= 0 as uint32_t
            || (*entry).type_0 as ::core::ffi::c_uint
                != AV_TIFF_SHORT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            ret = -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        } else {
            if !av_frame_side_data_get(
                    (*dst).side_data,
                    (*dst).nb_side_data,
                    AV_FRAME_DATA_DISPLAYMATRIX,
                )
                .is_null()
            {
                current_block = 14686729676684133893;
            } else {
                sd_frame = av_frame_side_data_new(
                    &raw mut (*dst).side_data,
                    &raw mut (*dst).nb_side_data,
                    AV_FRAME_DATA_DISPLAYMATRIX,
                    (::core::mem::size_of::<int32_t>() as size_t)
                        .wrapping_mul(9 as size_t),
                    0 as ::core::ffi::c_uint,
                );
                if sd_frame.is_null() {
                    ret = -(12 as ::core::ffi::c_int);
                    current_block = 2530802615946759448;
                } else {
                    ret = av_exif_orientation_to_matrix(
                        (*sd_frame).data as *mut int32_t,
                        *(*entry).value.uint.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int,
                    );
                    if ret < 0 as ::core::ffi::c_int {
                        current_block = 2530802615946759448;
                    } else {
                        current_block = 14686729676684133893;
                    }
                }
            }
            match current_block {
                2530802615946759448 => {}
                _ => {
                    av_exif_remove_entry(
                        ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        &raw mut ifd,
                        (*entry).id,
                        0 as ::core::ffi::c_int,
                    );
                    ret = av_exif_ifd_to_dict(
                        ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        &raw mut ifd,
                        &raw mut (*dst).metadata,
                    );
                    if !(ret < 0 as ::core::ffi::c_int) {
                        ret = av_exif_write(
                            ::core::ptr::null_mut::<::core::ffi::c_void>(),
                            &raw mut ifd,
                            &raw mut buf,
                            AV_EXIF_TIFF_HEADER,
                        );
                        if !(ret < 0 as ::core::ffi::c_int) {
                            if av_frame_side_data_add(
                                    &raw mut (*dst).side_data,
                                    &raw mut (*dst).nb_side_data,
                                    AV_FRAME_DATA_EXIF,
                                    &raw mut buf,
                                    0 as ::core::ffi::c_uint,
                                )
                                .is_null()
                            {
                                ret = -(12 as ::core::ffi::c_int);
                            } else {
                                ret = 0 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
        }
    }
    av_buffer_unref(&raw mut buf);
    av_exif_free(&raw mut ifd);
    return ret;
}
unsafe extern "C" fn side_data_map(
    mut dst: *mut AVFrame,
    mut sd_src: *const AVPacketSideData,
    mut nb_sd_src: ::core::ffi::c_int,
    mut map: *const SideDataMap,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while ((*map.offset(i as isize)).packet as ::core::ffi::c_uint)
        < AV_PKT_DATA_NB as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let type_pkt: AVPacketSideDataType = (*map.offset(i as isize)).packet;
        let type_frame: AVFrameSideDataType = (*map.offset(i as isize)).frame;
        let mut sd_pkt: *const AVPacketSideData = ::core::ptr::null::<
            AVPacketSideData,
        >();
        let mut sd_frame: *mut AVFrameSideData = ::core::ptr::null_mut::<
            AVFrameSideData,
        >();
        sd_pkt = packet_side_data_get(sd_src, nb_sd_src, type_pkt);
        if !sd_pkt.is_null() {
            sd_frame = av_frame_get_side_data(dst, type_frame);
            if !sd_frame.is_null() {
                if type_frame as ::core::ffi::c_uint
                    == AV_FRAME_DATA_STEREO3D as ::core::ffi::c_int
                        as ::core::ffi::c_uint
                {
                    let mut ret: ::core::ffi::c_int = side_data_stereo3d_merge(
                        sd_frame,
                        sd_pkt,
                    );
                    if ret < 0 as ::core::ffi::c_int {
                        return ret;
                    }
                }
            } else {
                match type_pkt as ::core::ffi::c_uint {
                    40 => {
                        let mut ret_0: ::core::ffi::c_int = side_data_exif_parse(
                            dst,
                            sd_pkt,
                        );
                        if ret_0 < 0 as ::core::ffi::c_int {
                            return ret_0;
                        }
                    }
                    _ => {
                        sd_frame = av_frame_new_side_data(
                            dst,
                            type_frame,
                            (*sd_pkt).size,
                        );
                        if sd_frame.is_null() {
                            return -(12 as ::core::ffi::c_int);
                        }
                        memcpy(
                            (*sd_frame).data as *mut ::core::ffi::c_void,
                            (*sd_pkt).data as *const ::core::ffi::c_void,
                            (*sd_pkt).size as size_t,
                        );
                    }
                }
            }
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn add_metadata_from_side_data(
    mut avpkt: *const AVPacket,
    mut frame: *mut AVFrame,
) -> ::core::ffi::c_int {
    let mut size: size_t = 0;
    let mut side_metadata: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut frame_md: *mut *mut AVDictionary = &raw mut (*frame).metadata;
    side_metadata = av_packet_get_side_data(
        avpkt,
        AV_PKT_DATA_STRINGS_METADATA,
        &raw mut size,
    );
    return av_packet_unpack_dictionary(side_metadata, size, frame_md);
}
#[no_mangle]
pub unsafe extern "C" fn ff_decode_frame_props_from_pkt(
    mut avctx: *const AVCodecContext,
    mut frame: *mut AVFrame,
    mut pkt: *const AVPacket,
) -> ::core::ffi::c_int {
    static mut sd: [SideDataMap; 11] = [
        SideDataMap {
            packet: AV_PKT_DATA_A53_CC,
            frame: AV_FRAME_DATA_A53_CC,
        },
        SideDataMap {
            packet: AV_PKT_DATA_AFD,
            frame: AV_FRAME_DATA_AFD,
        },
        SideDataMap {
            packet: AV_PKT_DATA_DYNAMIC_HDR10_PLUS,
            frame: AV_FRAME_DATA_DYNAMIC_HDR_PLUS,
        },
        SideDataMap {
            packet: AV_PKT_DATA_DYNAMIC_HDR_SMPTE_2094_APP5,
            frame: AV_FRAME_DATA_DYNAMIC_HDR_SMPTE_2094_APP5,
        },
        SideDataMap {
            packet: AV_PKT_DATA_S12M_TIMECODE,
            frame: AV_FRAME_DATA_S12M_TIMECODE,
        },
        SideDataMap {
            packet: AV_PKT_DATA_SKIP_SAMPLES,
            frame: AV_FRAME_DATA_SKIP_SAMPLES,
        },
        SideDataMap {
            packet: AV_PKT_DATA_LCEVC,
            frame: AV_FRAME_DATA_LCEVC,
        },
        SideDataMap {
            packet: AV_PKT_DATA_IAMF_MIX_GAIN_PARAM,
            frame: AV_FRAME_DATA_IAMF_MIX_GAIN_PARAM,
        },
        SideDataMap {
            packet: AV_PKT_DATA_IAMF_DEMIXING_INFO_PARAM,
            frame: AV_FRAME_DATA_IAMF_DEMIXING_INFO_PARAM,
        },
        SideDataMap {
            packet: AV_PKT_DATA_IAMF_RECON_GAIN_INFO_PARAM,
            frame: AV_FRAME_DATA_IAMF_RECON_GAIN_INFO_PARAM,
        },
        SideDataMap {
            packet: AV_PKT_DATA_NB,
            frame: AV_FRAME_DATA_PANSCAN,
        },
    ];
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    (*frame).pts = (*pkt).pts;
    (*frame).duration = (*pkt).duration;
    if (*pkt).side_data_elems != 0 {
        ret = side_data_map(
            frame,
            (*pkt).side_data,
            (*pkt).side_data_elems,
            &raw const ff_sd_global_map as *const SideDataMap,
        );
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        ret = side_data_map(
            frame,
            (*pkt).side_data,
            (*pkt).side_data_elems,
            &raw const sd as *const SideDataMap,
        );
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        add_metadata_from_side_data(pkt, frame);
    }
    if (*pkt).flags & 0x4 as ::core::ffi::c_int != 0 {
        (*frame).flags |= (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
    }
    if (*avctx).flags & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0 {
        ret = av_buffer_replace(&raw mut (*frame).opaque_ref, (*pkt).opaque_ref);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        (*frame).opaque = (*pkt).opaque;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_decode_frame_props(
    mut avctx: *mut AVCodecContext,
    mut frame: *mut AVFrame,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    ret = side_data_map(
        frame,
        (*avctx).coded_side_data,
        (*avctx).nb_coded_side_data,
        &raw const ff_sd_global_map as *const SideDataMap,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*avctx).nb_decoded_side_data {
        let mut src: *const AVFrameSideData = *(*avctx)
            .decoded_side_data
            .offset(i as isize);
        if av_frame_get_side_data(frame, (*src).type_0).is_null() {
            ret = av_frame_side_data_clone(
                &raw mut (*frame).side_data,
                &raw mut (*frame).nb_side_data,
                src,
                0 as ::core::ffi::c_uint,
            );
            if ret < 0 as ::core::ffi::c_int {
                return ret;
            }
        }
        i += 1;
    }
    if (*ffcodec((*avctx).codec as *const AVCodec)).caps_internal() as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int == 0
    {
        let mut pkt: *const AVPacket = (*(*avctx).internal).last_pkt_props;
        ret = ff_decode_frame_props_from_pkt(avctx, frame, pkt);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
    }
    ret = fill_frame_props(avctx, frame);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    match (*(*avctx).codec).type_0 as ::core::ffi::c_int {
        0 => {
            if (*frame).width != 0 && (*frame).height != 0
                && av_image_check_sar(
                    (*frame).width as ::core::ffi::c_uint,
                    (*frame).height as ::core::ffi::c_uint,
                    (*frame).sample_aspect_ratio,
                ) < 0 as ::core::ffi::c_int
            {
                av_log(
                    avctx as *mut ::core::ffi::c_void,
                    24 as ::core::ffi::c_int,
                    b"ignoring invalid SAR: %u/%u\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*frame).sample_aspect_ratio.num,
                    (*frame).sample_aspect_ratio.den,
                );
                (*frame).sample_aspect_ratio = AVRational {
                    num: 0 as ::core::ffi::c_int,
                    den: 1 as ::core::ffi::c_int,
                };
            }
        }
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn validate_avframe_allocation(
    mut avctx: *mut AVCodecContext,
    mut frame: *mut AVFrame,
) {
    if (*avctx).codec_type as ::core::ffi::c_int
        == AVMEDIA_TYPE_VIDEO as ::core::ffi::c_int
    {
        let mut i: ::core::ffi::c_int = 0;
        let mut num_planes: ::core::ffi::c_int = av_pix_fmt_count_planes(
            (*frame).format as AVPixelFormat,
        );
        let mut desc: *const AVPixFmtDescriptor = av_pix_fmt_desc_get(
            (*frame).format as AVPixelFormat,
        );
        let mut flags: ::core::ffi::c_int = (if !desc.is_null() {
            (*desc).flags
        } else {
            0 as uint64_t
        }) as ::core::ffi::c_int;
        if num_planes == 1 as ::core::ffi::c_int
            && flags & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0
        {
            num_planes = 2 as ::core::ffi::c_int;
        }
        i = 0 as ::core::ffi::c_int;
        while i < num_planes {
            if (*frame).data[i as usize].is_null() {
                av_log(
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    0 as ::core::ffi::c_int,
                    b"Assertion %s failed at %s:%d\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"frame->data[i]\0" as *const u8 as *const ::core::ffi::c_char,
                    b"src/libavcodec/decode.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1680 as ::core::ffi::c_int,
                );
                abort();
            }
            i += 1;
        }
        i = num_planes;
        while num_planes > 0 as ::core::ffi::c_int
            && (i as usize)
                < (::core::mem::size_of::<[*mut uint8_t; 8]>() as usize)
                    .wrapping_div(::core::mem::size_of::<*mut uint8_t>() as usize)
        {
            if !(*frame).data[i as usize].is_null() {
                av_log(
                    avctx as *mut ::core::ffi::c_void,
                    16 as ::core::ffi::c_int,
                    b"Buffer returned by get_buffer2() did not zero unused plane pointers\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            (*frame).data[i as usize] = ::core::ptr::null_mut::<uint8_t>();
            i += 1;
        }
    }
}
unsafe extern "C" fn decode_data_free(
    mut unused: AVRefStructOpaque,
    mut obj: *mut ::core::ffi::c_void,
) {
    let mut fdd: *mut FrameDecodeData = obj as *mut FrameDecodeData;
    if (*fdd).hwaccel_priv_free.is_some() {
        (*fdd)
            .hwaccel_priv_free
            .expect("non-null function pointer")((*fdd).hwaccel_priv);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ff_attach_decode_data(
    mut avctx: *mut AVCodecContext,
    mut frame: *mut AVFrame,
) -> ::core::ffi::c_int {
    let mut fdd: *mut FrameDecodeData = ::core::ptr::null_mut::<FrameDecodeData>();
    av_refstruct_unref(&raw mut (*frame).private_ref as *mut ::core::ffi::c_void);
    fdd = av_refstruct_alloc_ext(
        ::core::mem::size_of::<FrameDecodeData>() as size_t,
        0 as ::core::ffi::c_uint,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        Some(
            decode_data_free
                as unsafe extern "C" fn(
                    AVRefStructOpaque,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
    ) as *mut FrameDecodeData;
    if fdd.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    (*frame).private_ref = fdd as *mut ::core::ffi::c_void;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_get_buffer(
    mut avctx: *mut AVCodecContext,
    mut frame: *mut AVFrame,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut hwaccel: *const FFHWAccel = ffhwaccel((*avctx).hwaccel as *const AVHWAccel);
    let mut override_dimensions: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut ret: ::core::ffi::c_int = 0;
    if ff_codec_is_decoder((*avctx).codec as *const AVCodec) == 0 {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"ff_codec_is_decoder(avctx->codec)\0" as *const u8
                as *const ::core::ffi::c_char,
            b"src/libavcodec/decode.c\0" as *const u8 as *const ::core::ffi::c_char,
            1783 as ::core::ffi::c_int,
        );
        abort();
    }
    if (*avctx).codec_type as ::core::ffi::c_int
        == AVMEDIA_TYPE_VIDEO as ::core::ffi::c_int
    {
        if (*avctx).width as ::core::ffi::c_uint
            > (2147483647 as ::core::ffi::c_int - 16 as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            || {
                ret = av_image_check_size2(
                    ((*avctx).width + 16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                        & !(16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                        as ::core::ffi::c_uint,
                    (*avctx).height as ::core::ffi::c_uint,
                    (*avctx).max_pixels,
                    AV_PIX_FMT_NONE,
                    0 as ::core::ffi::c_int,
                    avctx as *mut ::core::ffi::c_void,
                );
                ret < 0 as ::core::ffi::c_int
            } || ((*avctx).pix_fmt as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        {
            av_log(
                avctx as *mut ::core::ffi::c_void,
                16 as ::core::ffi::c_int,
                b"video_get_buffer: image parameters invalid\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            ret = -(22 as ::core::ffi::c_int);
            current_block = 2256972617474165558;
        } else {
            if (*frame).width <= 0 as ::core::ffi::c_int
                || (*frame).height <= 0 as ::core::ffi::c_int
            {
                (*frame).width = if (*avctx).width
                    > (if 0 == 0 {
                        -(-(*avctx).coded_width >> (*avctx).lowres)
                    } else {
                        (*avctx).coded_width
                            + ((1 as ::core::ffi::c_int) << (*avctx).lowres)
                            - 1 as ::core::ffi::c_int >> (*avctx).lowres
                    })
                {
                    (*avctx).width
                } else if 0 == 0 {
                    -(-(*avctx).coded_width >> (*avctx).lowres)
                } else {
                    (*avctx).coded_width + ((1 as ::core::ffi::c_int) << (*avctx).lowres)
                        - 1 as ::core::ffi::c_int >> (*avctx).lowres
                };
                (*frame).height = if (*avctx).height
                    > (if 0 == 0 {
                        -(-(*avctx).coded_height >> (*avctx).lowres)
                    } else {
                        (*avctx).coded_height
                            + ((1 as ::core::ffi::c_int) << (*avctx).lowres)
                            - 1 as ::core::ffi::c_int >> (*avctx).lowres
                    })
                {
                    (*avctx).height
                } else if 0 == 0 {
                    -(-(*avctx).coded_height >> (*avctx).lowres)
                } else {
                    (*avctx).coded_height
                        + ((1 as ::core::ffi::c_int) << (*avctx).lowres)
                        - 1 as ::core::ffi::c_int >> (*avctx).lowres
                };
                override_dimensions = 0 as ::core::ffi::c_int;
            }
            if !(*frame).data[0 as ::core::ffi::c_int as usize].is_null()
                || !(*frame).data[1 as ::core::ffi::c_int as usize].is_null()
                || !(*frame).data[2 as ::core::ffi::c_int as usize].is_null()
                || !(*frame).data[3 as ::core::ffi::c_int as usize].is_null()
            {
                av_log(
                    avctx as *mut ::core::ffi::c_void,
                    16 as ::core::ffi::c_int,
                    b"pic->data[*]!=NULL in get_buffer_internal\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                ret = -(22 as ::core::ffi::c_int);
                current_block = 2256972617474165558;
            } else {
                current_block = 13797916685926291137;
            }
        }
    } else if (*avctx).codec_type as ::core::ffi::c_int
        == AVMEDIA_TYPE_AUDIO as ::core::ffi::c_int
    {
        if (*frame).nb_samples as int64_t * (*avctx).ch_layout.nb_channels as int64_t
            > (*avctx).max_samples
        {
            av_log(
                avctx as *mut ::core::ffi::c_void,
                16 as ::core::ffi::c_int,
                b"samples per frame %d, exceeds max_samples %lld\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*frame).nb_samples,
                (*avctx).max_samples,
            );
            ret = -(22 as ::core::ffi::c_int);
            current_block = 2256972617474165558;
        } else {
            current_block = 13797916685926291137;
        }
    } else {
        current_block = 13797916685926291137;
    }
    match current_block {
        13797916685926291137 => {
            ret = ff_decode_frame_props(avctx, frame);
            if !(ret < 0 as ::core::ffi::c_int) {
                if !hwaccel.is_null() {
                    if (*hwaccel).alloc_frame.is_some() {
                        ret = (*hwaccel)
                            .alloc_frame
                            .expect("non-null function pointer")(avctx, frame);
                        current_block = 7202070187249336029;
                    } else {
                        current_block = 7056779235015430508;
                    }
                } else {
                    (*avctx).sw_pix_fmt = (*avctx).pix_fmt;
                    current_block = 7056779235015430508;
                }
                match current_block {
                    7056779235015430508 => {
                        ret = (*avctx)
                            .get_buffer2
                            .expect(
                                "non-null function pointer",
                            )(avctx as *mut AVCodecContext, frame, flags);
                        if ret < 0 as ::core::ffi::c_int {
                            current_block = 2256972617474165558;
                        } else {
                            validate_avframe_allocation(avctx, frame);
                            ret = ff_attach_decode_data(avctx, frame);
                            if ret < 0 as ::core::ffi::c_int {
                                current_block = 2256972617474165558;
                            } else {
                                current_block = 7202070187249336029;
                            }
                        }
                    }
                    _ => {}
                }
                match current_block {
                    2256972617474165558 => {}
                    _ => {
                        if (*avctx).codec_type as ::core::ffi::c_int
                            == AVMEDIA_TYPE_VIDEO as ::core::ffi::c_int
                            && override_dimensions == 0
                            && (*ffcodec((*avctx).codec as *const AVCodec))
                                .caps_internal() as ::core::ffi::c_int
                                & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int == 0
                        {
                            (*frame).width = (*avctx).width;
                            (*frame).height = (*avctx).height;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if ret < 0 as ::core::ffi::c_int {
        av_log(
            avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"get_buffer() failed\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        av_frame_unref(frame);
    }
    return ret;
}
unsafe extern "C" fn reget_buffer_internal(
    mut avctx: *mut AVCodecContext,
    mut frame: *mut AVFrame,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    if !((*avctx).codec_type as ::core::ffi::c_int
        == AVMEDIA_TYPE_VIDEO as ::core::ffi::c_int)
    {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"avctx->codec_type == AVMEDIA_TYPE_VIDEO\0" as *const u8
                as *const ::core::ffi::c_char,
            b"src/libavcodec/decode.c\0" as *const u8 as *const ::core::ffi::c_char,
            1854 as ::core::ffi::c_int,
        );
        abort();
    }
    (*frame).flags &= !((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int);
    if !(*frame).data[0 as ::core::ffi::c_int as usize].is_null()
        && ((*frame).width != (*avctx).width || (*frame).height != (*avctx).height
            || (*frame).format != (*avctx).pix_fmt as ::core::ffi::c_int)
    {
        av_log(
            avctx as *mut ::core::ffi::c_void,
            24 as ::core::ffi::c_int,
            b"Picture changed from size:%dx%d fmt:%s to size:%dx%d fmt:%s in reget buffer()\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            (*frame).width,
            (*frame).height,
            av_get_pix_fmt_name((*frame).format as AVPixelFormat),
            (*avctx).width,
            (*avctx).height,
            av_get_pix_fmt_name((*avctx).pix_fmt),
        );
        av_frame_unref(frame);
    }
    if (*frame).data[0 as ::core::ffi::c_int as usize].is_null() {
        return ff_get_buffer(
            avctx,
            frame,
            (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
        );
    }
    av_frame_side_data_free(&raw mut (*frame).side_data, &raw mut (*frame).nb_side_data);
    if flags & 1 as ::core::ffi::c_int != 0 || av_frame_is_writable(frame) != 0 {
        return ff_decode_frame_props(avctx, frame);
    }
    let mut data: [*mut uint8_t; 4] = [::core::ptr::null_mut::<uint8_t>(); 4];
    let mut buf: [*mut AVBufferRef; 4] = [::core::ptr::null_mut::<AVBufferRef>(); 4];
    let mut linesize: [::core::ffi::c_int; 4] = [0; 4];
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        data[i as usize] = (*frame).data[i as usize];
        linesize[i as usize] = (*frame).linesize[i as usize];
        buf[i as usize] = (*frame).buf[i as usize];
        (*frame).buf[i as usize] = ::core::ptr::null_mut::<AVBufferRef>();
        i += 1;
    }
    av_frame_unref(frame);
    ret = ff_get_buffer(
        avctx,
        frame,
        (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
    );
    if ret >= 0 as ::core::ffi::c_int {
        av_image_copy2(
            &raw mut (*frame).data as *mut *mut uint8_t as *const *mut uint8_t,
            &raw mut (*frame).linesize as *mut ::core::ffi::c_int
                as *const ::core::ffi::c_int,
            &raw mut data as *mut *mut uint8_t as *const *mut uint8_t,
            &raw mut linesize as *mut ::core::ffi::c_int as *const ::core::ffi::c_int,
            (*frame).format as AVPixelFormat,
            (*frame).width,
            (*frame).height,
        );
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < 4 as ::core::ffi::c_int {
        av_buffer_unref(
            (&raw mut buf as *mut *mut AVBufferRef).offset(i_0 as isize)
                as *mut *mut AVBufferRef,
        );
        i_0 += 1;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ff_reget_buffer(
    mut avctx: *mut AVCodecContext,
    mut frame: *mut AVFrame,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = reget_buffer_internal(avctx, frame, flags);
    if ret < 0 as ::core::ffi::c_int {
        av_log(
            avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"reget_buffer() failed\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return ret;
}
unsafe extern "C" fn check_progress_consistency(mut f: *const ProgressFrame) {}
#[no_mangle]
pub unsafe extern "C" fn ff_progress_frame_alloc(
    mut avctx: *mut AVCodecContext,
    mut f: *mut ProgressFrame,
) -> ::core::ffi::c_int {
    let mut pool: *mut AVRefStructPool = (*(*avctx).internal).progress_frame_pool
        as *mut AVRefStructPool;
    (*f).progress = av_refstruct_pool_get(pool) as *mut ProgressInternal;
    if (*f).progress.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    (*f).f = (*(*f).progress).f;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_progress_frame_get_buffer(
    mut avctx: *mut AVCodecContext,
    mut f: *mut ProgressFrame,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = ff_progress_frame_alloc(avctx, f);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    ret = ff_thread_get_buffer(avctx, (*(*f).progress).f as *mut AVFrame, flags);
    if ret < 0 as ::core::ffi::c_int {
        (*f).f = ::core::ptr::null_mut::<AVFrame>();
        av_refstruct_unref(&raw mut (*f).progress as *mut ::core::ffi::c_void);
        return ret;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_progress_frame_ref(
    mut dst: *mut ProgressFrame,
    mut src: *const ProgressFrame,
) {
    (*dst).f = (*src).f;
    (*dst).progress = av_refstruct_ref((*src).progress as *mut ::core::ffi::c_void)
        as *mut ProgressInternal;
}
#[no_mangle]
pub unsafe extern "C" fn ff_progress_frame_unref(mut f: *mut ProgressFrame) {
    check_progress_consistency(f);
    (*f).f = ::core::ptr::null_mut::<AVFrame>();
    av_refstruct_unref(&raw mut (*f).progress as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ff_progress_frame_replace(
    mut dst: *mut ProgressFrame,
    mut src: *const ProgressFrame,
) {
    if dst == src as *mut ProgressFrame {
        return;
    }
    ff_progress_frame_unref(dst);
    check_progress_consistency(src);
    if !(*src).f.is_null() {
        ff_progress_frame_ref(dst, src);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ff_progress_frame_report(
    mut f: *mut ProgressFrame,
    mut n: ::core::ffi::c_int,
) {
    ff_thread_progress_report(&raw mut (*(*f).progress).progress, n);
}
#[no_mangle]
pub unsafe extern "C" fn ff_progress_frame_await(
    mut f: *const ProgressFrame,
    mut n: ::core::ffi::c_int,
) {
    ff_thread_progress_await(&raw mut (*(*f).progress).progress, n);
}
#[cold]
unsafe extern "C" fn progress_frame_pool_init_cb(
    mut opaque: AVRefStructOpaque,
    mut obj: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut avctx: *const AVCodecContext = opaque.nc as *const AVCodecContext;
    let mut progress: *mut ProgressInternal = obj as *mut ProgressInternal;
    let mut ret: ::core::ffi::c_int = 0;
    ret = ff_thread_progress_init(
        &raw mut (*progress).progress,
        (*avctx).active_thread_type & 1 as ::core::ffi::c_int,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    (*progress).f = av_frame_alloc() as *mut AVFrame;
    if (*progress).f.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn progress_frame_pool_reset_cb(
    mut unused: AVRefStructOpaque,
    mut obj: *mut ::core::ffi::c_void,
) {
    let mut progress: *mut ProgressInternal = obj as *mut ProgressInternal;
    ff_thread_progress_reset(&raw mut (*progress).progress);
    av_frame_unref((*progress).f as *mut AVFrame);
}
#[cold]
unsafe extern "C" fn progress_frame_pool_free_entry_cb(
    mut opaque: AVRefStructOpaque,
    mut obj: *mut ::core::ffi::c_void,
) {
    let mut progress: *mut ProgressInternal = obj as *mut ProgressInternal;
    ff_thread_progress_destroy(&raw mut (*progress).progress);
    av_frame_free(&raw mut (*progress).f);
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ff_decode_preinit(
    mut avctx: *mut AVCodecContext,
) -> ::core::ffi::c_int {
    let mut avci: *mut AVCodecInternal = (*avctx).internal as *mut AVCodecInternal;
    let mut dc: *mut DecodeContext = decode_ctx(avci);
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    (*dc).initial_pict_type = AV_PICTURE_TYPE_NONE;
    if (*(*avctx).codec_descriptor).props
        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0
    {
        (*dc).intra_only_flag = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
        if (*avctx).codec_type as ::core::ffi::c_int
            == AVMEDIA_TYPE_VIDEO as ::core::ffi::c_int
        {
            (*dc).initial_pict_type = AV_PICTURE_TYPE_I;
        }
    }
    av_freep(&raw mut (*avctx).subtitle_header as *mut ::core::ffi::c_void);
    if ((*(*avctx).codec).max_lowres as ::core::ffi::c_int) < (*avctx).lowres
        || (*avctx).lowres < 0 as ::core::ffi::c_int
    {
        av_log(
            avctx as *mut ::core::ffi::c_void,
            24 as ::core::ffi::c_int,
            b"The maximum value for lowres supported by the decoder is %d\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            (*(*avctx).codec).max_lowres as ::core::ffi::c_int,
        );
        (*avctx).lowres = (*(*avctx).codec).max_lowres as ::core::ffi::c_int;
    }
    if !(*avctx).sub_charenc.is_null() {
        if (*avctx).codec_type as ::core::ffi::c_int
            != AVMEDIA_TYPE_SUBTITLE as ::core::ffi::c_int
        {
            av_log(
                avctx as *mut ::core::ffi::c_void,
                16 as ::core::ffi::c_int,
                b"Character encoding is only supported with subtitles codecs\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            return -(22 as ::core::ffi::c_int);
        } else if (*(*avctx).codec_descriptor).props
            & (1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int != 0
        {
            av_log(
                avctx as *mut ::core::ffi::c_void,
                24 as ::core::ffi::c_int,
                b"Codec '%s' is bitmap-based, subtitles character encoding will be ignored\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                (*(*avctx).codec_descriptor).name,
            );
            (*avctx).sub_charenc_mode = -(1 as ::core::ffi::c_int);
        } else {
            if (*avctx).sub_charenc_mode == 0 as ::core::ffi::c_int {
                (*avctx).sub_charenc_mode = 1 as ::core::ffi::c_int;
            }
            if (*avctx).sub_charenc_mode == 1 as ::core::ffi::c_int {
                let mut cd: iconv_t = iconv_open(
                    b"UTF-8\0" as *const u8 as *const ::core::ffi::c_char,
                    (*avctx).sub_charenc,
                );
                if cd == -(1 as ::core::ffi::c_int) as iconv_t {
                    ret = -*__errno_location();
                    av_log(
                        avctx as *mut ::core::ffi::c_void,
                        16 as ::core::ffi::c_int,
                        b"Unable to open iconv context with input character encoding \"%s\"\n\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        (*avctx).sub_charenc,
                    );
                    return ret;
                }
                iconv_close(cd);
            }
        }
    }
    (*dc).pts_correction_num_faulty_dts = 0 as int64_t;
    (*dc).pts_correction_num_faulty_pts = (*dc).pts_correction_num_faulty_dts;
    (*dc).pts_correction_last_dts = (-(1 as ::core::ffi::c_int)
        as ::core::ffi::c_longlong - 0x7fffffffffffffff as ::core::ffi::c_longlong)
        as int64_t;
    (*dc).pts_correction_last_pts = (*dc).pts_correction_last_dts;
    if 0 as ::core::ffi::c_int == 0
        && (*avctx).flags & (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int != 0
        && (*(*avctx).codec_descriptor).type_0 as ::core::ffi::c_int
            == AVMEDIA_TYPE_VIDEO as ::core::ffi::c_int
    {
        av_log(
            avctx as *mut ::core::ffi::c_void,
            24 as ::core::ffi::c_int,
            b"gray decoding requested but not enabled at configuration time\n\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*avctx).flags2 & (1 as ::core::ffi::c_int) << 28 as ::core::ffi::c_int != 0 {
        (*avctx).export_side_data
            |= (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
    }
    if (*avctx).nb_side_data_prefer_packet == 1 as ::core::ffi::c_uint
        && *(*avctx).side_data_prefer_packet.offset(0 as ::core::ffi::c_int as isize)
            == -(1 as ::core::ffi::c_int)
    {
        (*dc).side_data_pref_mask = !(0 as ::core::ffi::c_ulonglong) as uint64_t;
    } else {
        let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while i < (*avctx).nb_side_data_prefer_packet {
            let mut val: ::core::ffi::c_int = *(*avctx)
                .side_data_prefer_packet
                .offset(i as isize);
            if val < 0 as ::core::ffi::c_int
                || val >= AV_PKT_DATA_NB as ::core::ffi::c_int
            {
                av_log(
                    avctx as *mut ::core::ffi::c_void,
                    16 as ::core::ffi::c_int,
                    b"Invalid side data type: %d\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    val,
                );
                return -(22 as ::core::ffi::c_int);
            }
            let mut j: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
            while ((*(&raw const ff_sd_global_map as *const SideDataMap)
                .offset(j as isize))
                .packet as ::core::ffi::c_uint)
                < AV_PKT_DATA_NB as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if (*(&raw const ff_sd_global_map as *const SideDataMap)
                    .offset(j as isize))
                    .packet as ::core::ffi::c_uint == val as ::core::ffi::c_uint
                {
                    val = (*(&raw const ff_sd_global_map as *const SideDataMap)
                        .offset(j as isize))
                        .frame as ::core::ffi::c_int;
                    if val >= 64 as ::core::ffi::c_int {
                        av_log(
                            avctx as *mut ::core::ffi::c_void,
                            16 as ::core::ffi::c_int,
                            b"Side data type too big\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        return -((('B' as i32 | ('U' as i32) << 8 as ::core::ffi::c_int
                            | ('G' as i32) << 16 as ::core::ffi::c_int)
                            as ::core::ffi::c_uint
                            | ('!' as i32 as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                    }
                    (*dc).side_data_pref_mask |= (1 as ::core::ffi::c_ulonglong) << val;
                }
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
    }
    (*avci).in_pkt = av_packet_alloc();
    (*avci).last_pkt_props = av_packet_alloc();
    if (*avci).in_pkt.is_null() || (*avci).last_pkt_props.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    if (*ffcodec((*avctx).codec as *const AVCodec)).caps_internal() as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int != 0
    {
        (*avci).progress_frame_pool = av_refstruct_pool_alloc_ext(
            ::core::mem::size_of::<ProgressInternal>() as size_t,
            ((1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int)
                as ::core::ffi::c_uint,
            avctx as *mut ::core::ffi::c_void,
            Some(
                progress_frame_pool_init_cb
                    as unsafe extern "C" fn(
                        AVRefStructOpaque,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                progress_frame_pool_reset_cb
                    as unsafe extern "C" fn(
                        AVRefStructOpaque,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            ),
            Some(
                progress_frame_pool_free_entry_cb
                    as unsafe extern "C" fn(
                        AVRefStructOpaque,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            ),
            None,
        ) as *mut AVRefStructPool;
        if (*avci).progress_frame_pool.is_null() {
            return -(12 as ::core::ffi::c_int);
        }
    }
    ret = decode_bsfs_init(avctx);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    if (*avctx).export_side_data & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
        == 0
    {
        (*avctx).codec_type as ::core::ffi::c_int
            == AVMEDIA_TYPE_VIDEO as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn side_data_pref(
    mut avctx: *const AVCodecContext,
    mut sd: *mut *mut *mut AVFrameSideData,
    mut nb_sd: *mut ::core::ffi::c_int,
    mut type_0: AVFrameSideDataType,
) -> ::core::ffi::c_int {
    let mut dc: *mut DecodeContext = decode_ctx(
        (*avctx).internal as *mut AVCodecInternal,
    );
    if !av_frame_side_data_get(*sd, *nb_sd, type_0).is_null() {
        if (*dc).side_data_pref_mask as ::core::ffi::c_ulonglong
            & (1 as ::core::ffi::c_ulonglong) << type_0 as ::core::ffi::c_uint != 0
        {
            return 1 as ::core::ffi::c_int;
        }
        av_frame_side_data_remove(sd, nb_sd, type_0);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_frame_new_side_data(
    mut avctx: *const AVCodecContext,
    mut frame: *mut AVFrame,
    mut type_0: AVFrameSideDataType,
    mut size: size_t,
    mut psd: *mut *mut AVFrameSideData,
) -> ::core::ffi::c_int {
    let mut sd: *mut AVFrameSideData = ::core::ptr::null_mut::<AVFrameSideData>();
    if side_data_pref(
        avctx,
        &raw mut (*frame).side_data,
        &raw mut (*frame).nb_side_data,
        type_0,
    ) != 0
    {
        if !psd.is_null() {
            *psd = ::core::ptr::null_mut::<AVFrameSideData>();
        }
        return 0 as ::core::ffi::c_int;
    }
    sd = av_frame_new_side_data(frame, type_0, size);
    if !psd.is_null() {
        *psd = sd;
    }
    return if !sd.is_null() {
        0 as ::core::ffi::c_int
    } else {
        -(12 as ::core::ffi::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ff_frame_new_side_data_from_buf_ext(
    mut avctx: *const AVCodecContext,
    mut sd: *mut *mut *mut AVFrameSideData,
    mut nb_sd: *mut ::core::ffi::c_int,
    mut type_0: AVFrameSideDataType,
    mut buf: *mut *mut AVBufferRef,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !(side_data_pref(avctx, sd, nb_sd, type_0) != 0) {
        if av_frame_side_data_add(sd, nb_sd, type_0, buf, 0 as ::core::ffi::c_uint)
            .is_null()
        {
            ret = -(12 as ::core::ffi::c_int);
        }
    }
    av_buffer_unref(buf);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ff_frame_new_side_data_from_buf(
    mut avctx: *const AVCodecContext,
    mut frame: *mut AVFrame,
    mut type_0: AVFrameSideDataType,
    mut buf: *mut *mut AVBufferRef,
) -> ::core::ffi::c_int {
    return ff_frame_new_side_data_from_buf_ext(
        avctx,
        &raw mut (*frame).side_data,
        &raw mut (*frame).nb_side_data,
        type_0,
        buf,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ff_decode_mastering_display_new_ext(
    mut avctx: *const AVCodecContext,
    mut sd: *mut *mut *mut AVFrameSideData,
    mut nb_sd: *mut ::core::ffi::c_int,
    mut mdm: *mut *mut AVMasteringDisplayMetadata,
) -> ::core::ffi::c_int {
    let mut buf: *mut AVBufferRef = ::core::ptr::null_mut::<AVBufferRef>();
    let mut size: size_t = 0;
    if side_data_pref(avctx, sd, nb_sd, AV_FRAME_DATA_MASTERING_DISPLAY_METADATA) != 0 {
        *mdm = ::core::ptr::null_mut::<AVMasteringDisplayMetadata>();
        return 0 as ::core::ffi::c_int;
    }
    *mdm = av_mastering_display_metadata_alloc_size(&raw mut size)
        as *mut AVMasteringDisplayMetadata;
    if (*mdm).is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    buf = av_buffer_create(
        *mdm as *mut uint8_t,
        size,
        None,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        0 as ::core::ffi::c_int,
    );
    if buf.is_null() {
        av_freep(mdm as *mut ::core::ffi::c_void);
        return -(12 as ::core::ffi::c_int);
    }
    if av_frame_side_data_add(
            sd,
            nb_sd,
            AV_FRAME_DATA_MASTERING_DISPLAY_METADATA,
            &raw mut buf,
            0 as ::core::ffi::c_uint,
        )
        .is_null()
    {
        *mdm = ::core::ptr::null_mut::<AVMasteringDisplayMetadata>();
        av_buffer_unref(&raw mut buf);
        return -(12 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_decode_mastering_display_new(
    mut avctx: *const AVCodecContext,
    mut frame: *mut AVFrame,
    mut mdm: *mut *mut AVMasteringDisplayMetadata,
) -> ::core::ffi::c_int {
    if side_data_pref(
        avctx,
        &raw mut (*frame).side_data,
        &raw mut (*frame).nb_side_data,
        AV_FRAME_DATA_MASTERING_DISPLAY_METADATA,
    ) != 0
    {
        *mdm = ::core::ptr::null_mut::<AVMasteringDisplayMetadata>();
        return 0 as ::core::ffi::c_int;
    }
    *mdm = av_mastering_display_metadata_create_side_data(frame);
    return if !(*mdm).is_null() {
        0 as ::core::ffi::c_int
    } else {
        -(12 as ::core::ffi::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ff_decode_content_light_new_ext(
    mut avctx: *const AVCodecContext,
    mut sd: *mut *mut *mut AVFrameSideData,
    mut nb_sd: *mut ::core::ffi::c_int,
    mut clm: *mut *mut AVContentLightMetadata,
) -> ::core::ffi::c_int {
    let mut buf: *mut AVBufferRef = ::core::ptr::null_mut::<AVBufferRef>();
    let mut size: size_t = 0;
    if side_data_pref(avctx, sd, nb_sd, AV_FRAME_DATA_CONTENT_LIGHT_LEVEL) != 0 {
        *clm = ::core::ptr::null_mut::<AVContentLightMetadata>();
        return 0 as ::core::ffi::c_int;
    }
    *clm = av_content_light_metadata_alloc(&raw mut size);
    if (*clm).is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    buf = av_buffer_create(
        *clm as *mut uint8_t,
        size,
        None,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        0 as ::core::ffi::c_int,
    );
    if buf.is_null() {
        av_freep(clm as *mut ::core::ffi::c_void);
        return -(12 as ::core::ffi::c_int);
    }
    if av_frame_side_data_add(
            sd,
            nb_sd,
            AV_FRAME_DATA_CONTENT_LIGHT_LEVEL,
            &raw mut buf,
            0 as ::core::ffi::c_uint,
        )
        .is_null()
    {
        *clm = ::core::ptr::null_mut::<AVContentLightMetadata>();
        av_buffer_unref(&raw mut buf);
        return -(12 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_decode_content_light_new(
    mut avctx: *const AVCodecContext,
    mut frame: *mut AVFrame,
    mut clm: *mut *mut AVContentLightMetadata,
) -> ::core::ffi::c_int {
    if side_data_pref(
        avctx,
        &raw mut (*frame).side_data,
        &raw mut (*frame).nb_side_data,
        AV_FRAME_DATA_CONTENT_LIGHT_LEVEL,
    ) != 0
    {
        *clm = ::core::ptr::null_mut::<AVContentLightMetadata>();
        return 0 as ::core::ffi::c_int;
    }
    *clm = av_content_light_metadata_create_side_data(frame);
    return if !(*clm).is_null() {
        0 as ::core::ffi::c_int
    } else {
        -(12 as ::core::ffi::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ff_copy_palette(
    mut dst: *mut ::core::ffi::c_void,
    mut src: *const AVPacket,
    mut logctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut size: size_t = 0;
    let mut pal: *const ::core::ffi::c_void = av_packet_get_side_data(
        src,
        AV_PKT_DATA_PALETTE,
        &raw mut size,
    ) as *const ::core::ffi::c_void;
    if !pal.is_null() && size == 1024 as size_t {
        memcpy(dst, pal, 1024 as size_t);
        return 1 as ::core::ffi::c_int;
    } else if !pal.is_null() {
        av_log(
            logctx,
            16 as ::core::ffi::c_int,
            b"Palette size %zu is wrong\n\0" as *const u8 as *const ::core::ffi::c_char,
            size,
        );
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_hwaccel_frame_priv_alloc(
    mut avctx: *mut AVCodecContext,
    mut hwaccel_picture_private: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut hwaccel: *const FFHWAccel = ffhwaccel((*avctx).hwaccel as *const AVHWAccel);
    if hwaccel.is_null() || (*hwaccel).frame_priv_data_size == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if !(*hwaccel_picture_private).is_null() {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"!*hwaccel_picture_private\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavcodec/decode.c\0" as *const u8 as *const ::core::ffi::c_char,
            2343 as ::core::ffi::c_int,
        );
        abort();
    }
    if (*hwaccel).free_frame_priv.is_some() {
        let mut frames_ctx: *mut AVHWFramesContext = ::core::ptr::null_mut::<
            AVHWFramesContext,
        >();
        if (*avctx).hw_frames_ctx.is_null() {
            return -(22 as ::core::ffi::c_int);
        }
        frames_ctx = (*(*avctx).hw_frames_ctx).data as *mut AVHWFramesContext;
        *hwaccel_picture_private = av_refstruct_alloc_ext(
            (*hwaccel).frame_priv_data_size as size_t,
            0 as ::core::ffi::c_uint,
            (*frames_ctx).device_ctx as *mut ::core::ffi::c_void,
            (*hwaccel).free_frame_priv,
        );
    } else {
        *hwaccel_picture_private = av_refstruct_allocz(
            (*hwaccel).frame_priv_data_size as size_t,
        );
    }
    if (*hwaccel_picture_private).is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ff_decode_flush_buffers(mut avctx: *mut AVCodecContext) {
    let mut avci: *mut AVCodecInternal = (*avctx).internal as *mut AVCodecInternal;
    let mut dc: *mut DecodeContext = decode_ctx(avci);
    av_packet_unref((*avci).last_pkt_props);
    av_packet_unref((*avci).in_pkt);
    (*dc).pts_correction_num_faulty_dts = 0 as int64_t;
    (*dc).pts_correction_num_faulty_pts = (*dc).pts_correction_num_faulty_dts;
    (*dc).pts_correction_last_dts = (-(1 as ::core::ffi::c_int)
        as ::core::ffi::c_longlong - 0x7fffffffffffffff as ::core::ffi::c_longlong)
        as int64_t;
    (*dc).pts_correction_last_pts = (*dc).pts_correction_last_dts;
    if !(*avci).bsf.is_null() {
        av_bsf_flush((*avci).bsf as *mut AVBSFContext);
    }
    (*dc).nb_draining_errors = 0 as ::core::ffi::c_int;
    (*dc).draining_started = 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ff_decode_internal_alloc() -> *mut AVCodecInternal {
    return av_mallocz(::core::mem::size_of::<DecodeContext>() as size_t)
        as *mut AVCodecInternal;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ff_decode_internal_sync(
    mut dst: *mut AVCodecContext,
    mut src: *const AVCodecContext,
) {
    let mut src_dc: *const DecodeContext = decode_ctx(
        (*src).internal as *mut AVCodecInternal,
    );
    let mut dst_dc: *mut DecodeContext = decode_ctx(
        (*dst).internal as *mut AVCodecInternal,
    );
    (*dst_dc).initial_pict_type = (*src_dc).initial_pict_type;
    (*dst_dc).intra_only_flag = (*src_dc).intra_only_flag;
    (*dst_dc).side_data_pref_mask = (*src_dc).side_data_pref_mask;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ff_decode_internal_uninit(mut avctx: *mut AVCodecContext) {}
unsafe extern "C" fn attach_displaymatrix(
    mut avctx: *mut AVCodecContext,
    mut frame: *mut AVFrame,
    mut orientation: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut sd: *mut AVFrameSideData = ::core::ptr::null_mut::<AVFrameSideData>();
    let mut matrix: *mut int32_t = ::core::ptr::null_mut::<int32_t>();
    let mut ret: ::core::ffi::c_int = 0;
    if orientation < 1 as ::core::ffi::c_int || orientation > 8 as ::core::ffi::c_int {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    ret = ff_frame_new_side_data(
        avctx,
        frame,
        AV_FRAME_DATA_DISPLAYMATRIX,
        (::core::mem::size_of::<int32_t>() as size_t).wrapping_mul(9 as size_t),
        &raw mut sd,
    );
    if ret < 0 as ::core::ffi::c_int {
        let mut fresh9: [::core::ffi::c_char; 64] = [
            0 as ::core::ffi::c_int as ::core::ffi::c_char,
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
        ];
        av_log(
            avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"Could not allocate frame side data: %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            av_make_error_string(
                &raw mut fresh9 as *mut ::core::ffi::c_char,
                64 as size_t,
                ret,
            ),
        );
        return ret;
    }
    if !sd.is_null() {
        matrix = (*sd).data as *mut int32_t;
        ret = av_exif_orientation_to_matrix(matrix, orientation);
    }
    return ret;
}
unsafe extern "C" fn exif_attach_ifd(
    mut avctx: *mut AVCodecContext,
    mut frame: *mut AVFrame,
    mut ifd: *const AVExifMetadata,
    mut pbuf: *mut *mut AVBufferRef,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut orient: *const AVExifEntry = ::core::ptr::null::<AVExifEntry>();
    let mut cloned: *mut AVExifMetadata = ::core::ptr::null_mut::<AVExifMetadata>();
    let mut ret: ::core::ffi::c_int = 0;
    let mut i: size_t = 0 as size_t;
    while i < (*ifd).count {
        let mut entry: *const AVExifEntry = (*ifd).entries.offset(i as isize)
            as *mut AVExifEntry;
        if (*entry).id as ::core::ffi::c_int
            == av_exif_get_tag_id(
                b"Orientation\0" as *const u8 as *const ::core::ffi::c_char,
            ) && (*entry).count > 0 as uint32_t
            && (*entry).type_0 as ::core::ffi::c_uint
                == AV_TIFF_SHORT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            orient = entry;
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    if !orient.is_null() {
        av_log(
            avctx as *mut ::core::ffi::c_void,
            48 as ::core::ffi::c_int,
            b"found EXIF orientation: %llu\n\0" as *const u8
                as *const ::core::ffi::c_char,
            *(*orient).value.uint.offset(0 as ::core::ffi::c_int as isize),
        );
        ret = attach_displaymatrix(
            avctx,
            frame,
            *(*orient).value.uint.offset(0 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int,
        );
        if ret < 0 as ::core::ffi::c_int {
            av_log(
                avctx as *mut ::core::ffi::c_void,
                24 as ::core::ffi::c_int,
                b"unable to attach displaymatrix from EXIF\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            current_block = 9606288038608642794;
        } else {
            cloned = av_exif_clone_ifd(ifd);
            if cloned.is_null() {
                ret = -(12 as ::core::ffi::c_int);
                current_block = 5354974598731121138;
            } else {
                av_exif_remove_entry(
                    avctx as *mut ::core::ffi::c_void,
                    cloned,
                    (*orient).id,
                    0 as ::core::ffi::c_int,
                );
                ifd = cloned;
                current_block = 9606288038608642794;
            }
        }
    } else {
        current_block = 9606288038608642794;
    }
    match current_block {
        9606288038608642794 => {
            ret = av_exif_ifd_to_dict(
                avctx as *mut ::core::ffi::c_void,
                ifd,
                &raw mut (*frame).metadata,
            );
            if !(ret < 0 as ::core::ffi::c_int) {
                if !cloned.is_null() || (*pbuf).is_null() {
                    av_buffer_unref(pbuf);
                    ret = av_exif_write(
                        avctx as *mut ::core::ffi::c_void,
                        ifd,
                        pbuf,
                        AV_EXIF_TIFF_HEADER,
                    );
                    if ret < 0 as ::core::ffi::c_int {
                        current_block = 5354974598731121138;
                    } else {
                        current_block = 11584701595673473500;
                    }
                } else {
                    current_block = 11584701595673473500;
                }
                match current_block {
                    5354974598731121138 => {}
                    _ => {
                        ret = ff_frame_new_side_data_from_buf(
                            avctx,
                            frame,
                            AV_FRAME_DATA_EXIF,
                            pbuf,
                        );
                        if !(ret < 0 as ::core::ffi::c_int) {
                            ret = 0 as ::core::ffi::c_int;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    av_buffer_unref(pbuf);
    av_exif_free(cloned);
    av_free(cloned as *mut ::core::ffi::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ff_decode_exif_attach_ifd(
    mut avctx: *mut AVCodecContext,
    mut frame: *mut AVFrame,
    mut ifd: *const AVExifMetadata,
) -> ::core::ffi::c_int {
    let mut dummy: *mut AVBufferRef = ::core::ptr::null_mut::<AVBufferRef>();
    return exif_attach_ifd(avctx, frame, ifd, &raw mut dummy);
}
#[no_mangle]
pub unsafe extern "C" fn ff_decode_exif_attach_buffer(
    mut avctx: *mut AVCodecContext,
    mut frame: *mut AVFrame,
    mut pbuf: *mut *mut AVBufferRef,
    mut header_mode: AVExifHeaderMode,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut data: *mut AVBufferRef = *pbuf;
    let mut ifd: AVExifMetadata = AVExifMetadata {
        entries: ::core::ptr::null_mut::<AVExifEntry>(),
        count: 0,
        size: 0,
    };
    ret = av_exif_parse_buffer(
        avctx as *mut ::core::ffi::c_void,
        (*data).data,
        (*data).size,
        &raw mut ifd,
        header_mode,
    );
    if !(ret < 0 as ::core::ffi::c_int) {
        ret = exif_attach_ifd(avctx, frame, &raw mut ifd, pbuf);
    }
    av_buffer_unref(pbuf);
    av_exif_free(&raw mut ifd);
    return ret;
}
#[repr(C)]
pub struct AVOptionRanges {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVOption {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVBuffer {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVBufferPool {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVDictionary {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVRefStructPool {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct FramePool {
    _opaque: [u8; 0],
}
