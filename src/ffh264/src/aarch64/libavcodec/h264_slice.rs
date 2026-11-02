// Translated from FFmpeg 9.0.1 libavcodec/h264_slice.c
// (preprocessed for aarch64-unknown-linux-musl) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// H.26L/H.264/AVC/JVT/14496-10/... decoder
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

extern "C" {
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    static ff_h264_golomb_to_pict_type: [uint8_t; 5];
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn ff_h264qpel_init(c: *mut H264QpelContext, bit_depth: ::core::ffi::c_int);
    fn ff_thread_can_start_frame(avctx: *mut AVCodecContext) -> ::core::ffi::c_int;
    fn ff_h264chroma_init(c: *mut H264ChromaContext, bit_depth: ::core::ffi::c_int);
    static ff_zigzag_direct: [uint8_t; 64];
    static ff_zigzag_scan: [uint8_t; 17];
    static ff_golomb_vlc_len: [uint8_t; 512];
    static ff_ue_golomb_vlc_code: [uint8_t; 512];
    static ff_se_golomb_vlc_code: [int8_t; 512];
    fn ff_thread_report_progress(
        f: *mut ThreadFrame,
        progress: ::core::ffi::c_int,
        field: ::core::ffi::c_int,
    );
    fn abort() -> !;
    fn ff_init_cabac_decoder(
        c: *mut CABACContext,
        buf: *const uint8_t,
        buf_size: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ff_thread_get_buffer(
        avctx: *mut AVCodecContext,
        f: *mut AVFrame,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ff_thread_await_progress(
        f: *const ThreadFrame,
        progress: ::core::ffi::c_int,
        field: ::core::ffi::c_int,
    );
    fn ff_thread_get_ext_buffer(
        avctx: *mut AVCodecContext,
        f: *mut ThreadFrame,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ff_videodsp_init(ctx: *mut VideoDSPContext, bpc: ::core::ffi::c_int);
    fn ff_thread_release_ext_buffer(f: *mut ThreadFrame);
    fn ff_thread_ref_frame(
        dst: *mut ThreadFrame,
        src: *const ThreadFrame,
    ) -> ::core::ffi::c_int;
    fn ff_h264_pred_weight_table(
        gb: *mut GetBitContext,
        sps: *const SPS,
        ref_count: *const ::core::ffi::c_int,
        slice_type_nos: ::core::ffi::c_int,
        pwt: *mut H264PredWeightTable,
        picture_structure: ::core::ffi::c_int,
        logctx: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn ff_er_frame_start(s: *mut ERContext);
    fn av_timecode_get_smpte(
        rate: AVRational,
        drop_0: ::core::ffi::c_int,
        hh: ::core::ffi::c_int,
        mm: ::core::ffi::c_int,
        ss: ::core::ffi::c_int,
        ff: ::core::ffi::c_int,
    ) -> uint32_t;
    fn ff_er_add_slice(
        s: *mut ERContext,
        startx: ::core::ffi::c_int,
        starty: ::core::ffi::c_int,
        endx: ::core::ffi::c_int,
        endy: ::core::ffi::c_int,
        status: ::core::ffi::c_int,
    );
    fn avpriv_request_sample(
        avc: *mut ::core::ffi::c_void,
        msg: *const ::core::ffi::c_char,
        ...
    );
    fn ff_set_sar(avctx: *mut AVCodecContext, sar: AVRational) -> ::core::ffi::c_int;
    fn ff_h264_parse_ref_count(
        plist_count: *mut ::core::ffi::c_int,
        ref_count: *mut ::core::ffi::c_int,
        gb: *mut GetBitContext,
        pps: *const PPS,
        slice_type_nos: ::core::ffi::c_int,
        picture_structure: ::core::ffi::c_int,
        logctx: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn ff_h264_pred_init(
        h: *mut H264PredContext,
        codec_id: ::core::ffi::c_int,
        bit_depth: ::core::ffi::c_int,
        chroma_format_idc: ::core::ffi::c_int,
    );
    fn ff_h264_init_poc(
        pic_field_poc: *mut ::core::ffi::c_int,
        pic_poc: *mut ::core::ffi::c_int,
        sps: *const SPS,
        poc: *mut H264POCContext,
        picture_structure: ::core::ffi::c_int,
        nal_ref_idc: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn av_refstruct_unref(objp: *mut ::core::ffi::c_void);
    fn av_reduce(
        dst_num: *mut ::core::ffi::c_int,
        dst_den: *mut ::core::ffi::c_int,
        num: int64_t,
        den: int64_t,
        max: int64_t,
    ) -> ::core::ffi::c_int;
    fn ff_h264dsp_init(
        c: *mut H264DSPContext,
        bit_depth: ::core::ffi::c_int,
        chroma_format_idc: ::core::ffi::c_int,
    );
    fn av_timecode_make_smpte_tc_string2(
        buf: *mut ::core::ffi::c_char,
        rate: AVRational,
        tcsmpte: uint32_t,
        prevent_df: ::core::ffi::c_int,
        skip_field: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn ff_get_format(
        avctx: *mut AVCodecContext,
        fmt: *const AVPixelFormat,
    ) -> ::core::ffi::c_int;
    fn av_refstruct_ref_c(obj: *const ::core::ffi::c_void) -> *const ::core::ffi::c_void;
    fn ff_h264_sei_process_picture_timing(
        h: *mut H264SEIPictureTiming,
        sps: *const SPS,
        logctx: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn av_refstruct_replace(
        dstp: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
    );
    fn ff_h264_get_profile(sps: *const SPS) -> ::core::ffi::c_int;
    fn ff_hwaccel_frame_priv_alloc(
        avctx: *mut AVCodecContext,
        hwaccel_picture_private: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn ff_h2645_sei_ctx_replace(
        dst: *mut H2645SEI,
        src: *const H2645SEI,
    ) -> ::core::ffi::c_int;
    fn av_dict_set(
        pm: *mut *mut AVDictionary,
        key: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ff_h2645_sei_to_frame(
        frame: *mut AVFrame,
        sei: *mut H2645SEI,
        codec_id: AVCodecID,
        avctx: *mut AVCodecContext,
        vui: *const H2645VUI,
        bit_depth_luma: ::core::ffi::c_uint,
        bit_depth_chroma: ::core::ffi::c_uint,
        seed: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ff_frame_new_side_data(
        avctx: *const AVCodecContext,
        frame: *mut AVFrame,
        type_0: AVFrameSideDataType,
        size: size_t,
        sd: *mut *mut AVFrameSideData,
    ) -> ::core::ffi::c_int;
    fn ff_frame_new_side_data_from_buf(
        avctx: *const AVCodecContext,
        frame: *mut AVFrame,
        type_0: AVFrameSideDataType,
        buf: *mut *mut AVBufferRef,
    ) -> ::core::ffi::c_int;
    fn av_pix_fmt_desc_get(pix_fmt: AVPixelFormat) -> *const AVPixFmtDescriptor;
    fn av_pix_fmt_get_chroma_sub_sample(
        pix_fmt: AVPixelFormat,
        h_shift: *mut ::core::ffi::c_int,
        v_shift: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn av_refstruct_pool_alloc(
        size: size_t,
        flags: ::core::ffi::c_uint,
    ) -> *mut AVRefStructPool;
    fn av_color_transfer_name(
        transfer: AVColorTransferCharacteristic,
    ) -> *const ::core::ffi::c_char;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn av_refstruct_pool_get(pool: *mut AVRefStructPool) -> *mut ::core::ffi::c_void;
    fn av_get_picture_type_char(pict_type: AVPictureType) -> ::core::ffi::c_char;
    fn av_fast_malloc(
        ptr: *mut ::core::ffi::c_void,
        size: *mut ::core::ffi::c_uint,
        min_size: size_t,
    );
    fn av_get_pix_fmt_name(pix_fmt: AVPixelFormat) -> *const ::core::ffi::c_char;
    fn av_fast_mallocz(
        ptr: *mut ::core::ffi::c_void,
        size: *mut ::core::ffi::c_uint,
        min_size: size_t,
    );
    fn av_freep(ptr: *mut ::core::ffi::c_void);
    fn av_memcpy_backptr(
        dst: *mut uint8_t,
        back: ::core::ffi::c_int,
        cnt: ::core::ffi::c_int,
    );
    fn ff_h264_alloc_tables(h: *mut H264Context) -> ::core::ffi::c_int;
    fn ff_h264_decode_ref_pic_list_reordering(
        sl: *mut H264SliceContext,
        logctx: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn ff_h264_build_ref_list(
        h: *mut H264Context,
        sl: *mut H264SliceContext,
    ) -> ::core::ffi::c_int;
    fn ff_h264_execute_ref_pic_marking(h: *mut H264Context) -> ::core::ffi::c_int;
    fn ff_h264_decode_ref_pic_marking(
        sl: *mut H264SliceContext,
        gb: *mut GetBitContext,
        nal: *const H2645NAL,
        logctx: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn ff_h264_hl_decode_mb(h: *const H264Context, sl: *mut H264SliceContext);
    fn ff_h264_decode_mb_cavlc(
        h: *const H264Context,
        sl: *mut H264SliceContext,
    ) -> ::core::ffi::c_int;
    fn ff_h264_decode_mb_cabac(
        h: *const H264Context,
        sl: *mut H264SliceContext,
    ) -> ::core::ffi::c_int;
    fn ff_h264_init_cabac_states(h: *const H264Context, sl: *mut H264SliceContext);
    fn ff_h264_direct_dist_scale_factor(
        h: *const H264Context,
        sl: *mut H264SliceContext,
    );
    fn ff_h264_direct_ref_list_init(h: *const H264Context, sl: *mut H264SliceContext);
    fn ff_h264_filter_mb_fast(
        h: *const H264Context,
        sl: *mut H264SliceContext,
        mb_x: ::core::ffi::c_int,
        mb_y: ::core::ffi::c_int,
        img_y: *mut uint8_t,
        img_cb: *mut uint8_t,
        img_cr: *mut uint8_t,
        linesize: ::core::ffi::c_uint,
        uvlinesize: ::core::ffi::c_uint,
    );
    fn ff_h264_filter_mb(
        h: *const H264Context,
        sl: *mut H264SliceContext,
        mb_x: ::core::ffi::c_int,
        mb_y: ::core::ffi::c_int,
        img_y: *mut uint8_t,
        img_cb: *mut uint8_t,
        img_cr: *mut uint8_t,
        linesize: ::core::ffi::c_uint,
        uvlinesize: ::core::ffi::c_uint,
    );
    fn ff_h264_field_end(
        h: *mut H264Context,
        sl: *mut H264SliceContext,
        in_setup: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ff_h264_ref_picture(
        dst: *mut H264Picture,
        src: *const H264Picture,
    ) -> ::core::ffi::c_int;
    fn ff_h264_replace_picture(
        dst: *mut H264Picture,
        src: *const H264Picture,
    ) -> ::core::ffi::c_int;
    fn ff_h264_unref_picture(pic: *mut H264Picture);
    fn ff_h264_slice_context_init(h: *mut H264Context, sl: *mut H264SliceContext);
    fn ff_h264_draw_horiz_band(
        h: *const H264Context,
        sl: *mut H264SliceContext,
        y: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    );
    fn ff_h264_flush_change(h: *mut H264Context);
    fn ff_h264_free_tables(h: *mut H264Context);
    fn ff_h264_set_erpic(dst: *mut ERPicture, src: *const H264Picture);
}
pub type size_t = ::core::ffi::c_ulong;
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
pub type ptrdiff_t = ::core::ffi::c_long;
pub type int8_t = ::core::ffi::c_schar;
pub type int16_t = ::core::ffi::c_short;
pub type int32_t = ::core::ffi::c_int;
pub type int64_t = ::core::ffi::c_long;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint16_t = ::core::ffi::c_ushort;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint64_t = ::core::ffi::c_ulong;
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
pub struct AVHWAccel {
    pub name: *const ::core::ffi::c_char,
    pub type_0: AVMediaType,
    pub id: AVCodecID,
    pub pix_fmt: AVPixelFormat,
    pub capabilities: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CABACContext {
    pub low: ::core::ffi::c_int,
    pub range: ::core::ffi::c_int,
    pub bytestream_start: *const uint8_t,
    pub bytestream: *const uint8_t,
    pub bytestream_end: *const uint8_t,
}
pub type atomic_int = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ERPicture {
    pub f: *mut AVFrame,
    pub tf: *const ThreadFrame,
    pub progress: *const ThreadProgress,
    pub motion_val: [*mut [int16_t; 2]; 2],
    pub ref_index: [*mut int8_t; 2],
    pub mb_type: *mut uint32_t,
    pub field_picture: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ThreadFrame {
    pub f: *mut AVFrame,
    pub owner: [*mut AVCodecContext; 2],
    pub progress: *mut ThreadFrameProgress,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ERContext {
    pub avctx: *mut AVCodecContext,
    pub sad: Option<
        unsafe extern "C" fn(
            *mut MPVEncContext,
            *const uint8_t,
            *const uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub mb_index2xy: *mut ::core::ffi::c_int,
    pub mb_num: ::core::ffi::c_int,
    pub mb_width: ::core::ffi::c_int,
    pub mb_height: ::core::ffi::c_int,
    pub mb_stride: ptrdiff_t,
    pub b8_stride: ptrdiff_t,
    pub error_count: atomic_int,
    pub error_occurred: ::core::ffi::c_int,
    pub error_status_table: *mut uint8_t,
    pub er_temp_buffer: *mut uint8_t,
    pub dc_val: [*mut int16_t; 3],
    pub mbskip_table: *mut uint8_t,
    pub mbintra_table: *mut uint8_t,
    pub mv: [[[::core::ffi::c_int; 2]; 4]; 2],
    pub cur_pic: ERPicture,
    pub last_pic: ERPicture,
    pub next_pic: ERPicture,
    pub ref_index: [*mut int8_t; 2],
    pub motion_val_base: [*mut [int16_t; 2]; 2],
    pub pp_time: uint16_t,
    pub pb_time: uint16_t,
    pub quarter_sample: ::core::ffi::c_int,
    pub partitioned_frame: ::core::ffi::c_int,
    pub decode_mb: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut [[[::core::ffi::c_int; 2]; 4]; 2],
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub opaque: *mut ::core::ffi::c_void,
}
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const H264_NAL_UNSPECIFIED31: C2RustUnnamed_0 = 31;
pub const H264_NAL_UNSPECIFIED30: C2RustUnnamed_0 = 30;
pub const H264_NAL_UNSPECIFIED29: C2RustUnnamed_0 = 29;
pub const H264_NAL_UNSPECIFIED28: C2RustUnnamed_0 = 28;
pub const H264_NAL_UNSPECIFIED27: C2RustUnnamed_0 = 27;
pub const H264_NAL_UNSPECIFIED26: C2RustUnnamed_0 = 26;
pub const H264_NAL_UNSPECIFIED25: C2RustUnnamed_0 = 25;
pub const H264_NAL_UNSPECIFIED24: C2RustUnnamed_0 = 24;
pub const H264_NAL_RESERVED23: C2RustUnnamed_0 = 23;
pub const H264_NAL_RESERVED22: C2RustUnnamed_0 = 22;
pub const H264_NAL_DEPTH_EXTEN_SLICE: C2RustUnnamed_0 = 21;
pub const H264_NAL_EXTEN_SLICE: C2RustUnnamed_0 = 20;
pub const H264_NAL_AUXILIARY_SLICE: C2RustUnnamed_0 = 19;
pub const H264_NAL_RESERVED18: C2RustUnnamed_0 = 18;
pub const H264_NAL_RESERVED17: C2RustUnnamed_0 = 17;
pub const H264_NAL_DPS: C2RustUnnamed_0 = 16;
pub const H264_NAL_SUB_SPS: C2RustUnnamed_0 = 15;
pub const H264_NAL_PREFIX: C2RustUnnamed_0 = 14;
pub const H264_NAL_SPS_EXT: C2RustUnnamed_0 = 13;
pub const H264_NAL_FILLER_DATA: C2RustUnnamed_0 = 12;
pub const H264_NAL_END_STREAM: C2RustUnnamed_0 = 11;
pub const H264_NAL_END_SEQUENCE: C2RustUnnamed_0 = 10;
pub const H264_NAL_AUD: C2RustUnnamed_0 = 9;
pub const H264_NAL_PPS: C2RustUnnamed_0 = 8;
pub const H264_NAL_SPS: C2RustUnnamed_0 = 7;
pub const H264_NAL_SEI: C2RustUnnamed_0 = 6;
pub const H264_NAL_IDR_SLICE: C2RustUnnamed_0 = 5;
pub const H264_NAL_DPC: C2RustUnnamed_0 = 4;
pub const H264_NAL_DPB: C2RustUnnamed_0 = 3;
pub const H264_NAL_DPA: C2RustUnnamed_0 = 2;
pub const H264_NAL_SLICE: C2RustUnnamed_0 = 1;
pub const H264_NAL_UNSPECIFIED: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const H264_MAX_HEIGHT: C2RustUnnamed_1 = 16880;
pub const H264_MAX_WIDTH: C2RustUnnamed_1 = 16880;
pub const H264_MAX_MB_HEIGHT: C2RustUnnamed_1 = 1055;
pub const H264_MAX_MB_WIDTH: C2RustUnnamed_1 = 1055;
pub const H264_MAX_MB_PIC_SIZE: C2RustUnnamed_1 = 139264;
pub const H264_MAX_CPB_CNT: C2RustUnnamed_1 = 32;
pub const H264_MAX_SLICE_GROUPS: C2RustUnnamed_1 = 8;
pub const H264_MAX_MMCO_COUNT: C2RustUnnamed_1 = 67;
pub const H264_MAX_RPLM_COUNT: C2RustUnnamed_1 = 33;
pub const H264_MAX_REFS: C2RustUnnamed_1 = 32;
pub const H264_MAX_DPB_FRAMES: C2RustUnnamed_1 = 16;
pub const H264_MAX_PPS_COUNT: C2RustUnnamed_1 = 256;
pub const H264_MAX_SPS_COUNT: C2RustUnnamed_1 = 32;
#[derive(Copy, Clone)]
#[repr(C)]
pub union av_alias64 {
    pub u64_0: uint64_t,
    pub u32_0: [uint32_t; 2],
    pub u16_0: [uint16_t; 4],
    pub u8_0: [uint8_t; 8],
    pub f64_0: ::core::ffi::c_double,
    pub f32_0: [::core::ffi::c_float; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union av_alias32 {
    pub u32_0: uint32_t,
    pub u16_0: [uint16_t; 2],
    pub u8_0: [uint8_t; 4],
    pub f32_0: ::core::ffi::c_float,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union unaligned_64 {
    pub l: uint64_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SPS {
    pub sps_id: ::core::ffi::c_uint,
    pub profile_idc: ::core::ffi::c_int,
    pub level_idc: ::core::ffi::c_int,
    pub chroma_format_idc: ::core::ffi::c_int,
    pub transform_bypass: ::core::ffi::c_int,
    pub log2_max_frame_num: ::core::ffi::c_int,
    pub poc_type: ::core::ffi::c_int,
    pub log2_max_poc_lsb: ::core::ffi::c_int,
    pub delta_pic_order_always_zero_flag: ::core::ffi::c_int,
    pub offset_for_non_ref_pic: ::core::ffi::c_int,
    pub offset_for_top_to_bottom_field: ::core::ffi::c_int,
    pub poc_cycle_length: ::core::ffi::c_int,
    pub ref_frame_count: ::core::ffi::c_int,
    pub gaps_in_frame_num_allowed_flag: ::core::ffi::c_int,
    pub mb_width: ::core::ffi::c_int,
    pub mb_height: ::core::ffi::c_int,
    pub frame_mbs_only_flag: ::core::ffi::c_int,
    pub mb_aff: ::core::ffi::c_int,
    pub direct_8x8_inference_flag: ::core::ffi::c_int,
    pub crop: ::core::ffi::c_int,
    pub crop_left: ::core::ffi::c_uint,
    pub crop_right: ::core::ffi::c_uint,
    pub crop_top: ::core::ffi::c_uint,
    pub crop_bottom: ::core::ffi::c_uint,
    pub vui_parameters_present_flag: ::core::ffi::c_int,
    pub vui: H2645VUI,
    pub timing_info_present_flag: ::core::ffi::c_int,
    pub num_units_in_tick: uint32_t,
    pub time_scale: uint32_t,
    pub fixed_frame_rate_flag: ::core::ffi::c_int,
    pub offset_for_ref_frame: [int32_t; 256],
    pub bitstream_restriction_flag: ::core::ffi::c_int,
    pub num_reorder_frames: ::core::ffi::c_int,
    pub max_dec_frame_buffering: ::core::ffi::c_int,
    pub scaling_matrix_present: ::core::ffi::c_int,
    pub scaling_matrix_present_mask: uint16_t,
    pub scaling_matrix4: [[uint8_t; 16]; 6],
    pub scaling_matrix8: [[uint8_t; 64]; 6],
    pub nal_hrd_parameters_present_flag: ::core::ffi::c_int,
    pub vcl_hrd_parameters_present_flag: ::core::ffi::c_int,
    pub pic_struct_present_flag: ::core::ffi::c_int,
    pub time_offset_length: ::core::ffi::c_int,
    pub cpb_cnt: ::core::ffi::c_int,
    pub bit_rate_scale: ::core::ffi::c_int,
    pub bit_rate_value: [uint32_t; 32],
    pub cpb_size_value: [uint32_t; 32],
    pub cpr_flag: uint32_t,
    pub initial_cpb_removal_delay_length: ::core::ffi::c_int,
    pub cpb_removal_delay_length: ::core::ffi::c_int,
    pub dpb_output_delay_length: ::core::ffi::c_int,
    pub bit_depth_luma: ::core::ffi::c_int,
    pub bit_depth_chroma: ::core::ffi::c_int,
    pub residual_color_transform_flag: ::core::ffi::c_int,
    pub constraint_set_flags: ::core::ffi::c_int,
    pub data: [uint8_t; 4096],
    pub data_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PPS {
    pub pps_id: ::core::ffi::c_uint,
    pub sps_id: ::core::ffi::c_uint,
    pub cabac: ::core::ffi::c_int,
    pub pic_order_present: ::core::ffi::c_int,
    pub slice_group_count: ::core::ffi::c_int,
    pub mb_slice_group_map_type: ::core::ffi::c_int,
    pub ref_count: [::core::ffi::c_uint; 2],
    pub weighted_pred: ::core::ffi::c_int,
    pub weighted_bipred_idc: ::core::ffi::c_int,
    pub init_qp: ::core::ffi::c_int,
    pub init_qs: ::core::ffi::c_int,
    pub chroma_qp_index_offset: [::core::ffi::c_int; 2],
    pub deblocking_filter_parameters_present: ::core::ffi::c_int,
    pub constrained_intra_pred: ::core::ffi::c_int,
    pub redundant_pic_cnt_present: ::core::ffi::c_int,
    pub transform_8x8_mode: ::core::ffi::c_int,
    pub pic_scaling_matrix_present_flag: ::core::ffi::c_int,
    pub pic_scaling_matrix_present_mask: uint16_t,
    pub scaling_matrix4: [[uint8_t; 16]; 6],
    pub scaling_matrix8: [[uint8_t; 64]; 6],
    pub chroma_qp_table: [[uint8_t; 88]; 2],
    pub chroma_qp_diff: ::core::ffi::c_int,
    pub data: [uint8_t; 4096],
    pub data_size: size_t,
    pub dequant4_buffer: [[[uint32_t; 16]; 88]; 6],
    pub dequant8_buffer: [[[uint32_t; 64]; 88]; 6],
    pub dequant4_coeff: [*mut [uint32_t; 16]; 6],
    pub dequant8_coeff: [*mut [uint32_t; 64]; 6],
    pub sps: *const SPS,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264ParamSets {
    pub sps_list: [*const SPS; 32],
    pub pps_list: [*const PPS; 256],
    pub pps: *const PPS,
    pub sps: *const SPS,
    pub overread_warning_printed: [::core::ffi::c_int; 2],
}
pub type MMCOOpcode = ::core::ffi::c_uint;
pub const MMCO_LONG: MMCOOpcode = 6;
pub const MMCO_RESET: MMCOOpcode = 5;
pub const MMCO_SET_MAX_LONG: MMCOOpcode = 4;
pub const MMCO_SHORT2LONG: MMCOOpcode = 3;
pub const MMCO_LONG2UNUSED: MMCOOpcode = 2;
pub const MMCO_SHORT2UNUSED: MMCOOpcode = 1;
pub const MMCO_END: MMCOOpcode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264PredWeightTable {
    pub use_weight: ::core::ffi::c_int,
    pub use_weight_chroma: ::core::ffi::c_int,
    pub luma_log2_weight_denom: ::core::ffi::c_int,
    pub chroma_log2_weight_denom: ::core::ffi::c_int,
    pub luma_weight_flag: [::core::ffi::c_int; 2],
    pub chroma_weight_flag: [::core::ffi::c_int; 2],
    pub luma_weight: [[[::core::ffi::c_int; 2]; 2]; 48],
    pub chroma_weight: [[[[::core::ffi::c_int; 2]; 2]; 2]; 48],
    pub implicit_weight: [[[::core::ffi::c_int; 2]; 48]; 48],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264POCContext {
    pub poc_lsb: ::core::ffi::c_int,
    pub poc_msb: ::core::ffi::c_int,
    pub delta_poc_bottom: ::core::ffi::c_int,
    pub delta_poc: [::core::ffi::c_int; 2],
    pub frame_num: ::core::ffi::c_int,
    pub prev_poc_msb: ::core::ffi::c_int,
    pub prev_poc_lsb: ::core::ffi::c_int,
    pub frame_num_offset: ::core::ffi::c_int,
    pub prev_frame_num_offset: ::core::ffi::c_int,
    pub prev_frame_num: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFilmGrainAFGS1Params {
    pub enable: ::core::ffi::c_int,
    pub sets: [*mut AVBufferRef; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFITUTT35Meta {
    pub afd: *mut AVBufferRef,
    pub a53_cc: *mut AVBufferRef,
    pub lcevc: *mut AVBufferRef,
    pub hdr_plus: *mut AVBufferRef,
    pub hdr_smpte2094_app5: *mut AVBufferRef,
    pub hdr_vivid: *mut AVBufferRef,
    pub dovi: *mut AVBufferRef,
    pub aom_film_grain: AVFilmGrainAFGS1Params,
}
pub type SEIFpaType = ::core::ffi::c_uint;
pub const SEI_FPA_H264_TYPE_2D: SEIFpaType = 6;
pub const SEI_FPA_TYPE_INTERLEAVE_TEMPORAL: SEIFpaType = 5;
pub const SEI_FPA_TYPE_TOP_BOTTOM: SEIFpaType = 4;
pub const SEI_FPA_TYPE_SIDE_BY_SIDE: SEIFpaType = 3;
pub const SEI_FPA_H264_TYPE_INTERLEAVE_ROW: SEIFpaType = 2;
pub const SEI_FPA_H264_TYPE_INTERLEAVE_COLUMN: SEIFpaType = 1;
pub const SEI_FPA_H264_TYPE_CHECKERBOARD: SEIFpaType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2645SEIUnregistered {
    pub buf_ref: *mut *mut AVBufferRef,
    pub nb_buf_ref: ::core::ffi::c_uint,
    pub x264_build: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2645SEIFramePacking {
    pub present: ::core::ffi::c_int,
    pub arrangement_id: ::core::ffi::c_int,
    pub arrangement_cancel_flag: ::core::ffi::c_int,
    pub arrangement_type: SEIFpaType,
    pub arrangement_repetition_period: ::core::ffi::c_int,
    pub content_interpretation_type: ::core::ffi::c_int,
    pub quincunx_sampling_flag: ::core::ffi::c_int,
    pub current_frame_is_frame0_flag: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2645SEIDisplayOrientation {
    pub present: ::core::ffi::c_int,
    pub anticlockwise_rotation: ::core::ffi::c_int,
    pub hflip: ::core::ffi::c_int,
    pub vflip: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2645SEIAlternativeTransfer {
    pub present: ::core::ffi::c_int,
    pub preferred_transfer_characteristics: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2645SEIAmbientViewingEnvironment {
    pub present: ::core::ffi::c_int,
    pub ambient_illuminance: uint32_t,
    pub ambient_light_x: uint16_t,
    pub ambient_light_y: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2645SEIFilmGrainCharacteristics {
    pub present: ::core::ffi::c_int,
    pub model_id: ::core::ffi::c_int,
    pub separate_colour_description_present_flag: ::core::ffi::c_int,
    pub bit_depth_luma: ::core::ffi::c_int,
    pub bit_depth_chroma: ::core::ffi::c_int,
    pub full_range: ::core::ffi::c_int,
    pub color_primaries: ::core::ffi::c_int,
    pub transfer_characteristics: ::core::ffi::c_int,
    pub matrix_coeffs: ::core::ffi::c_int,
    pub blending_mode_id: ::core::ffi::c_int,
    pub log2_scale_factor: ::core::ffi::c_int,
    pub comp_model_present_flag: [::core::ffi::c_int; 3],
    pub num_intensity_intervals: [uint16_t; 3],
    pub num_model_values: [uint8_t; 3],
    pub intensity_interval_lower_bound: [[uint8_t; 256]; 3],
    pub intensity_interval_upper_bound: [[uint8_t; 256]; 3],
    pub comp_model_value: [[[int16_t; 6]; 256]; 3],
    pub repetition_period: ::core::ffi::c_int,
    pub persistence_flag: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2645SEIMasteringDisplay {
    pub present: ::core::ffi::c_int,
    pub display_primaries: [[uint16_t; 2]; 3],
    pub white_point: [uint16_t; 2],
    pub max_luminance: uint32_t,
    pub min_luminance: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2645SEIContentLight {
    pub present: ::core::ffi::c_int,
    pub max_content_light_level: uint16_t,
    pub max_pic_average_light_level: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2645SEI {
    pub itut_t35: FFITUTT35Meta,
    pub unregistered: H2645SEIUnregistered,
    pub frame_packing: H2645SEIFramePacking,
    pub display_orientation: H2645SEIDisplayOrientation,
    pub alternative_transfer: H2645SEIAlternativeTransfer,
    pub ambient_viewing_environment: H2645SEIAmbientViewingEnvironment,
    pub mastering_display: H2645SEIMasteringDisplay,
    pub content_light: H2645SEIContentLight,
    pub film_grain_characteristics: *mut H2645SEIFilmGrainCharacteristics,
}
pub type H264_SEI_PicStructType = ::core::ffi::c_uint;
pub const H264_SEI_PIC_STRUCT_FRAME_TRIPLING: H264_SEI_PicStructType = 8;
pub const H264_SEI_PIC_STRUCT_FRAME_DOUBLING: H264_SEI_PicStructType = 7;
pub const H264_SEI_PIC_STRUCT_BOTTOM_TOP_BOTTOM: H264_SEI_PicStructType = 6;
pub const H264_SEI_PIC_STRUCT_TOP_BOTTOM_TOP: H264_SEI_PicStructType = 5;
pub const H264_SEI_PIC_STRUCT_BOTTOM_TOP: H264_SEI_PicStructType = 4;
pub const H264_SEI_PIC_STRUCT_TOP_BOTTOM: H264_SEI_PicStructType = 3;
pub const H264_SEI_PIC_STRUCT_BOTTOM_FIELD: H264_SEI_PicStructType = 2;
pub const H264_SEI_PIC_STRUCT_TOP_FIELD: H264_SEI_PicStructType = 1;
pub const H264_SEI_PIC_STRUCT_FRAME: H264_SEI_PicStructType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264SEITimeCode {
    pub full: ::core::ffi::c_int,
    pub frame: ::core::ffi::c_int,
    pub seconds: ::core::ffi::c_int,
    pub minutes: ::core::ffi::c_int,
    pub hours: ::core::ffi::c_int,
    pub dropframe: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264SEIPictureTiming {
    pub payload: [uint8_t; 40],
    pub payload_size_bytes: ::core::ffi::c_int,
    pub present: ::core::ffi::c_int,
    pub pic_struct: H264_SEI_PicStructType,
    pub ct_type: ::core::ffi::c_int,
    pub dpb_output_delay: ::core::ffi::c_int,
    pub cpb_removal_delay: ::core::ffi::c_int,
    pub timecode: [H264SEITimeCode; 3],
    pub timecode_cnt: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264SEIRecoveryPoint {
    pub recovery_frame_cnt: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264SEIBufferingPeriod {
    pub present: ::core::ffi::c_int,
    pub initial_cpb_removal_delay: [::core::ffi::c_int; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264SEIGreenMetaData {
    pub green_metadata_type: uint8_t,
    pub period_type: uint8_t,
    pub num_seconds: uint16_t,
    pub num_pictures: uint16_t,
    pub percent_non_zero_macroblocks: uint8_t,
    pub percent_intra_coded_macroblocks: uint8_t,
    pub percent_six_tap_filtering: uint8_t,
    pub percent_alpha_point_deblocking_instance: uint8_t,
    pub xsd_metric_type: uint8_t,
    pub xsd_metric_value: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264SEIContext {
    pub common: H2645SEI,
    pub picture_timing: H264SEIPictureTiming,
    pub recovery_point: H264SEIRecoveryPoint,
    pub buffering_period: H264SEIBufferingPeriod,
    pub green_metadata: H264SEIGreenMetaData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2645NAL {
    pub data: *const uint8_t,
    pub size: ::core::ffi::c_int,
    pub size_bits: ::core::ffi::c_int,
    pub raw_size: ::core::ffi::c_int,
    pub raw_data: *const uint8_t,
    pub gb: GetBitContext,
    pub type_0: ::core::ffi::c_int,
    pub ref_idc: ::core::ffi::c_int,
    pub temporal_id: ::core::ffi::c_int,
    pub nuh_layer_id: ::core::ffi::c_int,
    pub skipped_bytes: ::core::ffi::c_int,
    pub skipped_bytes_pos_size: ::core::ffi::c_int,
    pub skipped_bytes_pos: *mut ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2645RBSP {
    pub rbsp_buffer: *mut uint8_t,
    pub rbsp_buffer_ref: *mut AVBufferRef,
    pub rbsp_buffer_alloc_size: ::core::ffi::c_int,
    pub rbsp_buffer_size: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2645Packet {
    pub nals: *mut H2645NAL,
    pub rbsp: H2645RBSP,
    pub nb_nals: ::core::ffi::c_int,
    pub nals_allocated: ::core::ffi::c_int,
    pub nal_buffer_size: ::core::ffi::c_uint,
}
pub type h264_chroma_mc_func = Option<
    unsafe extern "C" fn(
        *mut uint8_t,
        *const uint8_t,
        ptrdiff_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264ChromaContext {
    pub put_h264_chroma_pixels_tab: [h264_chroma_mc_func; 4],
    pub avg_h264_chroma_pixels_tab: [h264_chroma_mc_func; 4],
}
pub type h264_weight_func = Option<
    unsafe extern "C" fn(
        *mut uint8_t,
        ptrdiff_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> (),
>;
pub type h264_biweight_func = Option<
    unsafe extern "C" fn(
        *mut uint8_t,
        *mut uint8_t,
        ptrdiff_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264DSPContext {
    pub weight_pixels_tab: [h264_weight_func; 4],
    pub biweight_pixels_tab: [h264_biweight_func; 4],
    pub v_loop_filter_luma: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut int8_t,
        ) -> (),
    >,
    pub h_loop_filter_luma: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut int8_t,
        ) -> (),
    >,
    pub h_loop_filter_luma_mbaff: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut int8_t,
        ) -> (),
    >,
    pub v_loop_filter_luma_intra: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub h_loop_filter_luma_intra: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub h_loop_filter_luma_mbaff_intra: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub v_loop_filter_chroma: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut int8_t,
        ) -> (),
    >,
    pub h_loop_filter_chroma: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut int8_t,
        ) -> (),
    >,
    pub h_loop_filter_chroma_mbaff: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut int8_t,
        ) -> (),
    >,
    pub v_loop_filter_chroma_intra: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub h_loop_filter_chroma_intra: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub h_loop_filter_chroma_mbaff_intra: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub loop_filter_strength: Option<
        unsafe extern "C" fn(
            *mut [[int16_t; 4]; 4],
            *mut uint8_t,
            *mut [int8_t; 40],
            *mut [[int16_t; 2]; 40],
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub idct_add: Option<
        unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ::core::ffi::c_int) -> (),
    >,
    pub idct8_add: Option<
        unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ::core::ffi::c_int) -> (),
    >,
    pub idct_dc_add: Option<
        unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ::core::ffi::c_int) -> (),
    >,
    pub idct8_dc_add: Option<
        unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ::core::ffi::c_int) -> (),
    >,
    pub idct_add16: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            *const ::core::ffi::c_int,
            *mut int16_t,
            ::core::ffi::c_int,
            *const uint8_t,
        ) -> (),
    >,
    pub idct8_add4: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            *const ::core::ffi::c_int,
            *mut int16_t,
            ::core::ffi::c_int,
            *const uint8_t,
        ) -> (),
    >,
    pub idct_add8: Option<
        unsafe extern "C" fn(
            *mut *mut uint8_t,
            *const ::core::ffi::c_int,
            *mut int16_t,
            ::core::ffi::c_int,
            *const uint8_t,
        ) -> (),
    >,
    pub idct_add16intra: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            *const ::core::ffi::c_int,
            *mut int16_t,
            ::core::ffi::c_int,
            *const uint8_t,
        ) -> (),
    >,
    pub luma_dc_dequant_idct: Option<
        unsafe extern "C" fn(*mut int16_t, *mut int16_t, ::core::ffi::c_int) -> (),
    >,
    pub chroma_dc_dequant_idct: Option<
        unsafe extern "C" fn(*mut int16_t, ::core::ffi::c_int) -> (),
    >,
    pub add_pixels8_clear: Option<
        unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ::core::ffi::c_int) -> (),
    >,
    pub add_pixels4_clear: Option<
        unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ::core::ffi::c_int) -> (),
    >,
    pub startcode_find_candidate: Option<
        unsafe extern "C" fn(*const uint8_t, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264PredContext {
    pub pred4x4: [Option<
        unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
    >; 15],
    pub pred8x8l: [Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ptrdiff_t,
        ) -> (),
    >; 12],
    pub pred8x8: [Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>; 11],
    pub pred16x16: [Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>; 9],
    pub pred4x4_add: [Option<
        unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ptrdiff_t) -> (),
    >; 2],
    pub pred8x8l_add: [Option<
        unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ptrdiff_t) -> (),
    >; 2],
    pub pred8x8l_filter_add: [Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            *mut int16_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ptrdiff_t,
        ) -> (),
    >; 2],
    pub pred8x8_add: [Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            *const ::core::ffi::c_int,
            *mut int16_t,
            ptrdiff_t,
        ) -> (),
    >; 3],
    pub pred16x16_add: [Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            *const ::core::ffi::c_int,
            *mut int16_t,
            ptrdiff_t,
        ) -> (),
    >; 3],
}
pub type qpel_mc_func = Option<
    unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264QpelContext {
    pub put_h264_qpel_pixels_tab: [[qpel_mc_func; 16]; 3],
    pub avg_h264_qpel_pixels_tab: [[qpel_mc_func; 16]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoDSPContext {
    pub emulated_edge_mc: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            *const uint8_t,
            ptrdiff_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch: Option<
        unsafe extern "C" fn(*const uint8_t, ptrdiff_t, ::core::ffi::c_int) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MMCO {
    pub opcode: MMCOOpcode,
    pub short_pic_num: ::core::ffi::c_int,
    pub long_arg: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264Picture {
    pub f: *mut AVFrame,
    pub tf: ThreadFrame,
    pub f_grain: *mut AVFrame,
    pub qscale_table_base: *mut int8_t,
    pub qscale_table: *mut int8_t,
    pub motion_val_base: [*mut [int16_t; 2]; 2],
    pub motion_val: [*mut [int16_t; 2]; 2],
    pub mb_type_base: *mut uint32_t,
    pub mb_type: *mut uint32_t,
    pub hwaccel_picture_private: *mut ::core::ffi::c_void,
    pub ref_index: [*mut int8_t; 2],
    pub field_poc: [::core::ffi::c_int; 2],
    pub poc: ::core::ffi::c_int,
    pub frame_num: ::core::ffi::c_int,
    pub mmco_reset: ::core::ffi::c_int,
    pub pic_id: ::core::ffi::c_int,
    pub long_ref: ::core::ffi::c_int,
    pub ref_poc: [[[::core::ffi::c_int; 32]; 2]; 2],
    pub ref_count: [[::core::ffi::c_int; 2]; 2],
    pub mbaff: ::core::ffi::c_int,
    pub field_picture: ::core::ffi::c_int,
    pub reference: ::core::ffi::c_int,
    pub recovered: ::core::ffi::c_int,
    pub invalid_gap: ::core::ffi::c_int,
    pub sei_recovery_frame_cnt: ::core::ffi::c_int,
    pub needs_fg: ::core::ffi::c_int,
    pub pps: *const PPS,
    pub mb_width: ::core::ffi::c_int,
    pub mb_height: ::core::ffi::c_int,
    pub mb_stride: ::core::ffi::c_int,
    pub decode_error_flags: *mut atomic_int,
    pub gray: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264Ref {
    pub data: [*mut uint8_t; 3],
    pub linesize: [::core::ffi::c_int; 3],
    pub reference: ::core::ffi::c_int,
    pub poc: ::core::ffi::c_int,
    pub pic_id: ::core::ffi::c_int,
    pub parent: *const H264Picture,
}
#[derive(Copy, Clone)]
#[repr(C, align(16))]
pub struct H264SliceContext {
    pub h264: *const H264Context,
    pub gb: GetBitContext,
    pub er: *mut ERContext,
    pub slice_num: ::core::ffi::c_int,
    pub slice_type: ::core::ffi::c_int,
    pub slice_type_nos: ::core::ffi::c_int,
    pub slice_type_fixed: ::core::ffi::c_int,
    pub qscale: ::core::ffi::c_int,
    pub chroma_qp: [::core::ffi::c_int; 2],
    pub qp_thresh: ::core::ffi::c_int,
    pub last_qscale_diff: ::core::ffi::c_int,
    pub deblocking_filter: ::core::ffi::c_int,
    pub slice_alpha_c0_offset: ::core::ffi::c_int,
    pub slice_beta_offset: ::core::ffi::c_int,
    pub pwt: H264PredWeightTable,
    pub prev_mb_skipped: ::core::ffi::c_int,
    pub next_mb_skipped: ::core::ffi::c_int,
    pub chroma_pred_mode: ::core::ffi::c_int,
    pub intra16x16_pred_mode: ::core::ffi::c_int,
    pub intra4x4_pred_mode_cache: [int8_t; 40],
    pub intra4x4_pred_mode: *mut int8_t,
    pub topleft_mb_xy: ::core::ffi::c_int,
    pub top_mb_xy: ::core::ffi::c_int,
    pub topright_mb_xy: ::core::ffi::c_int,
    pub left_mb_xy: [::core::ffi::c_int; 2],
    pub topleft_type: ::core::ffi::c_int,
    pub top_type: ::core::ffi::c_int,
    pub topright_type: ::core::ffi::c_int,
    pub left_type: [::core::ffi::c_int; 2],
    pub left_block: *const uint8_t,
    pub topleft_partition: ::core::ffi::c_int,
    pub topleft_samples_available: ::core::ffi::c_uint,
    pub top_samples_available: ::core::ffi::c_uint,
    pub topright_samples_available: ::core::ffi::c_uint,
    pub left_samples_available: ::core::ffi::c_uint,
    pub linesize: ptrdiff_t,
    pub uvlinesize: ptrdiff_t,
    pub mb_linesize: ptrdiff_t,
    pub mb_uvlinesize: ptrdiff_t,
    pub mb_x: ::core::ffi::c_int,
    pub mb_y: ::core::ffi::c_int,
    pub mb_xy: ::core::ffi::c_int,
    pub resync_mb_x: ::core::ffi::c_int,
    pub resync_mb_y: ::core::ffi::c_int,
    pub first_mb_addr: ::core::ffi::c_uint,
    pub next_slice_idx: ::core::ffi::c_int,
    pub mb_skip_run: ::core::ffi::c_int,
    pub is_complex: ::core::ffi::c_int,
    pub picture_structure: ::core::ffi::c_int,
    pub mb_field_decoding_flag: ::core::ffi::c_int,
    pub mb_mbaff: ::core::ffi::c_int,
    pub redundant_pic_count: ::core::ffi::c_int,
    pub neighbor_transform_size: ::core::ffi::c_int,
    pub direct_spatial_mv_pred: ::core::ffi::c_int,
    pub col_parity: ::core::ffi::c_int,
    pub col_fieldoff: ::core::ffi::c_int,
    pub cbp: ::core::ffi::c_int,
    pub top_cbp: ::core::ffi::c_int,
    pub left_cbp: ::core::ffi::c_int,
    pub dist_scale_factor: [::core::ffi::c_int; 32],
    pub dist_scale_factor_field: [[::core::ffi::c_int; 32]; 2],
    pub map_col_to_list0: [[::core::ffi::c_int; 48]; 2],
    pub map_col_to_list0_field: [[[::core::ffi::c_int; 48]; 2]; 2],
    pub ref_count: [::core::ffi::c_uint; 2],
    pub list_count: ::core::ffi::c_uint,
    pub ref_list: [[H264Ref; 48]; 2],
    pub ref_modifications: [[C2RustUnnamed_2; 32]; 2],
    pub nb_ref_modifications: [::core::ffi::c_int; 2],
    pub pps_id: ::core::ffi::c_uint,
    pub intra_pcm_ptr: *const uint8_t,
    pub bipred_scratchpad: *mut uint8_t,
    pub edge_emu_buffer: *mut uint8_t,
    pub top_borders: [*mut [uint8_t; 96]; 2],
    pub bipred_scratchpad_allocated: ::core::ffi::c_int,
    pub edge_emu_buffer_allocated: ::core::ffi::c_int,
    pub top_borders_allocated: [::core::ffi::c_int; 2],
    pub non_zero_count_cache: [uint8_t; 120],
    pub mv_cache: [[[int16_t; 2]; 40]; 2],
    pub ref_cache: [[int8_t; 40]; 2],
    pub mvd_cache: [[[uint8_t; 2]; 40]; 2],
    pub direct_cache: [uint8_t; 40],
    pub sub_mb_type: [uint16_t; 4],
    pub mb: [int16_t; 1536],
    pub mb_luma_dc: [[int16_t; 32]; 3],
    pub mb_padding: [int16_t; 512],
    pub mvd_table: [*mut [uint8_t; 2]; 2],
    pub cabac: CABACContext,
    pub cabac_state: [uint8_t; 1024],
    pub cabac_init_idc: ::core::ffi::c_int,
    pub mmco: [MMCO; 67],
    pub nb_mmco: ::core::ffi::c_int,
    pub explicit_ref_marking: ::core::ffi::c_int,
    pub frame_num: ::core::ffi::c_int,
    pub idr_pic_id: ::core::ffi::c_int,
    pub poc_lsb: ::core::ffi::c_int,
    pub delta_poc_bottom: ::core::ffi::c_int,
    pub delta_poc: [::core::ffi::c_int; 2],
    pub curr_pic_num: ::core::ffi::c_int,
    pub max_pic_num: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub op: uint8_t,
    pub val: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264Context {
    pub class: *const AVClass,
    pub avctx: *mut AVCodecContext,
    pub vdsp: VideoDSPContext,
    pub h264dsp: H264DSPContext,
    pub h264chroma: H264ChromaContext,
    pub h264qpel: H264QpelContext,
    pub DPB: [H264Picture; 36],
    pub cur_pic_ptr: *mut H264Picture,
    pub cur_pic: H264Picture,
    pub last_pic_for_ec: H264Picture,
    pub slice_ctx: *mut H264SliceContext,
    pub nb_slice_ctx: ::core::ffi::c_int,
    pub nb_slice_ctx_queued: ::core::ffi::c_int,
    pub pkt: H2645Packet,
    pub pixel_shift: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub chroma_x_shift: ::core::ffi::c_int,
    pub chroma_y_shift: ::core::ffi::c_int,
    pub droppable: ::core::ffi::c_int,
    pub context_initialized: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub workaround_bugs: ::core::ffi::c_int,
    pub x264_build: ::core::ffi::c_int,
    pub postpone_filter: ::core::ffi::c_int,
    pub picture_idr: ::core::ffi::c_int,
    pub picture_intra_only: ::core::ffi::c_int,
    pub crop_left: ::core::ffi::c_int,
    pub crop_right: ::core::ffi::c_int,
    pub crop_top: ::core::ffi::c_int,
    pub crop_bottom: ::core::ffi::c_int,
    pub intra4x4_pred_mode: *mut int8_t,
    pub hpc: H264PredContext,
    pub non_zero_count: *mut [uint8_t; 48],
    pub block_offset: [::core::ffi::c_int; 96],
    pub mb2b_xy: *mut uint32_t,
    pub mb2br_xy: *mut uint32_t,
    pub b_stride: ::core::ffi::c_int,
    pub slice_table: *mut uint16_t,
    pub mb_aff_frame: ::core::ffi::c_int,
    pub picture_structure: ::core::ffi::c_int,
    pub first_field: ::core::ffi::c_int,
    pub list_counts: *mut uint8_t,
    pub cbp_table: *mut uint16_t,
    pub chroma_pred_mode_table: *mut uint8_t,
    pub mvd_table: [*mut [uint8_t; 2]; 2],
    pub direct_table: *mut uint8_t,
    pub scan_padding: [uint8_t; 16],
    pub zigzag_scan: [uint8_t; 16],
    pub zigzag_scan8x8: [uint8_t; 64],
    pub zigzag_scan8x8_cavlc: [uint8_t; 64],
    pub field_scan: [uint8_t; 16],
    pub field_scan8x8: [uint8_t; 64],
    pub field_scan8x8_cavlc: [uint8_t; 64],
    pub zigzag_scan_q0: [uint8_t; 16],
    pub zigzag_scan8x8_q0: [uint8_t; 64],
    pub zigzag_scan8x8_cavlc_q0: [uint8_t; 64],
    pub field_scan_q0: [uint8_t; 16],
    pub field_scan8x8_q0: [uint8_t; 64],
    pub field_scan8x8_cavlc_q0: [uint8_t; 64],
    pub mb_y: ::core::ffi::c_int,
    pub mb_height: ::core::ffi::c_int,
    pub mb_width: ::core::ffi::c_int,
    pub mb_stride: ::core::ffi::c_int,
    pub mb_num: ::core::ffi::c_int,
    pub nal_ref_idc: ::core::ffi::c_int,
    pub nal_unit_type: ::core::ffi::c_int,
    pub has_slice: ::core::ffi::c_int,
    pub is_avc: ::core::ffi::c_int,
    pub nal_length_size: ::core::ffi::c_int,
    pub bit_depth_luma: ::core::ffi::c_int,
    pub chroma_format_idc: ::core::ffi::c_int,
    pub ps: H264ParamSets,
    pub slice_table_base: *mut uint16_t,
    pub poc: H264POCContext,
    pub default_ref: [H264Ref; 2],
    pub short_ref: [*mut H264Picture; 32],
    pub long_ref: [*mut H264Picture; 32],
    pub delayed_pic: [*mut H264Picture; 18],
    pub last_pocs: [::core::ffi::c_int; 16],
    pub next_output_pic: *mut H264Picture,
    pub next_outputed_poc: ::core::ffi::c_int,
    pub poc_offset: ::core::ffi::c_int,
    pub mmco: [MMCO; 67],
    pub nb_mmco: ::core::ffi::c_int,
    pub mmco_reset: ::core::ffi::c_int,
    pub explicit_ref_marking: ::core::ffi::c_int,
    pub long_ref_count: ::core::ffi::c_int,
    pub short_ref_count: ::core::ffi::c_int,
    pub current_slice: ::core::ffi::c_int,
    pub prev_interlaced_frame: ::core::ffi::c_int,
    pub valid_recovery_point: ::core::ffi::c_int,
    pub recovery_frame: ::core::ffi::c_int,
    pub frame_recovered: ::core::ffi::c_int,
    pub has_recovery_point: ::core::ffi::c_int,
    pub missing_fields: ::core::ffi::c_int,
    pub setup_finished: ::core::ffi::c_int,
    pub cur_chroma_format_idc: ::core::ffi::c_int,
    pub cur_bit_depth_luma: ::core::ffi::c_int,
    pub slice_row: [int16_t; 32],
    pub width_from_caller: ::core::ffi::c_int,
    pub height_from_caller: ::core::ffi::c_int,
    pub enable_er: ::core::ffi::c_int,
    pub er: ERContext,
    pub dc_val_base: *mut int16_t,
    pub sei: H264SEIContext,
    pub qscale_table_pool: *mut AVRefStructPool,
    pub mb_type_pool: *mut AVRefStructPool,
    pub motion_val_pool: *mut AVRefStructPool,
    pub ref_index_pool: *mut AVRefStructPool,
    pub decode_error_flags_pool: *mut AVRefStructPool,
    pub ref2frm: [[[::core::ffi::c_int; 64]; 2]; 32],
    pub non_gray: ::core::ffi::c_int,
    pub noref_gray: ::core::ffi::c_int,
    pub skip_gray: ::core::ffi::c_int,
}
static mut scan8: [uint8_t; 51] = [
    (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 10 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
];
#[inline(always)]
unsafe extern "C" fn fill_rectangle(
    mut vp: *mut ::core::ffi::c_void,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
    mut stride: ::core::ffi::c_int,
    mut val: uint32_t,
    mut size: ::core::ffi::c_int,
) {
    let mut p: *mut uint8_t = vp as *mut uint8_t;
    w *= size;
    stride *= size;
    if w == 2 as ::core::ffi::c_int {
        let v: uint16_t = (if size == 4 as ::core::ffi::c_int {
            val
        } else {
            val.wrapping_mul(0x101 as uint32_t)
        }) as uint16_t;
        *(p.offset((0 as ::core::ffi::c_int * stride) as isize) as *mut uint16_t) = v;
        if h == 1 as ::core::ffi::c_int {
            return;
        }
        *(p.offset((1 as ::core::ffi::c_int * stride) as isize) as *mut uint16_t) = v;
        if h == 2 as ::core::ffi::c_int {
            return;
        }
        *(p.offset((2 as ::core::ffi::c_int * stride) as isize) as *mut uint16_t) = v;
        *(p.offset((3 as ::core::ffi::c_int * stride) as isize) as *mut uint16_t) = v;
    } else if w == 4 as ::core::ffi::c_int {
        let v_0: uint32_t = if size == 4 as ::core::ffi::c_int {
            val
        } else if size == 2 as ::core::ffi::c_int {
            val.wrapping_mul(0x10001 as ::core::ffi::c_int as uint32_t)
        } else {
            val.wrapping_mul(0x1010101 as ::core::ffi::c_int as uint32_t)
        };
        *(p.offset((0 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = v_0;
        if h == 1 as ::core::ffi::c_int {
            return;
        }
        *(p.offset((1 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = v_0;
        if h == 2 as ::core::ffi::c_int {
            return;
        }
        *(p.offset((2 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = v_0;
        *(p.offset((3 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = v_0;
    } else if w == 8 as ::core::ffi::c_int {
        let v_1: uint64_t = (if size == 2 as ::core::ffi::c_int {
            (val as ::core::ffi::c_ulonglong)
                .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong)
        } else {
            (val as ::core::ffi::c_ulonglong)
                .wrapping_mul(0x100000001 as ::core::ffi::c_ulonglong)
        }) as uint64_t;
        *(p.offset((0 as ::core::ffi::c_int * stride) as isize) as *mut uint64_t) = v_1;
        if h == 1 as ::core::ffi::c_int {
            return;
        }
        *(p.offset((1 as ::core::ffi::c_int * stride) as isize) as *mut uint64_t) = v_1;
        if h == 2 as ::core::ffi::c_int {
            return;
        }
        *(p.offset((2 as ::core::ffi::c_int * stride) as isize) as *mut uint64_t) = v_1;
        *(p.offset((3 as ::core::ffi::c_int * stride) as isize) as *mut uint64_t) = v_1;
    } else if w == 16 as ::core::ffi::c_int {
        let v_2: uint64_t = (val as ::core::ffi::c_ulonglong)
            .wrapping_mul(0x100000001 as ::core::ffi::c_ulonglong) as uint64_t;
        *(p
            .offset(0 as ::core::ffi::c_int as isize)
            .offset((0 as ::core::ffi::c_int * stride) as isize) as *mut uint64_t) = v_2;
        *(p
            .offset(8 as ::core::ffi::c_int as isize)
            .offset((0 as ::core::ffi::c_int * stride) as isize) as *mut uint64_t) = v_2;
        *(p
            .offset(0 as ::core::ffi::c_int as isize)
            .offset((1 as ::core::ffi::c_int * stride) as isize) as *mut uint64_t) = v_2;
        *(p
            .offset(8 as ::core::ffi::c_int as isize)
            .offset((1 as ::core::ffi::c_int * stride) as isize) as *mut uint64_t) = v_2;
        if h == 2 as ::core::ffi::c_int {
            return;
        }
        *(p
            .offset(0 as ::core::ffi::c_int as isize)
            .offset((2 as ::core::ffi::c_int * stride) as isize) as *mut uint64_t) = v_2;
        *(p
            .offset(8 as ::core::ffi::c_int as isize)
            .offset((2 as ::core::ffi::c_int * stride) as isize) as *mut uint64_t) = v_2;
        *(p
            .offset(0 as ::core::ffi::c_int as isize)
            .offset((3 as ::core::ffi::c_int * stride) as isize) as *mut uint64_t) = v_2;
        *(p
            .offset(8 as ::core::ffi::c_int as isize)
            .offset((3 as ::core::ffi::c_int * stride) as isize) as *mut uint64_t) = v_2;
    }
}
static mut field_scan: [uint8_t; 17] = [
    (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
        as uint8_t,
    (1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
        as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
        as uint8_t,
    (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
        as uint8_t,
    (1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
        as uint8_t,
    (2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
        as uint8_t,
    (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
        as uint8_t,
    (2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
        as uint8_t,
    (2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
        as uint8_t,
    (3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
        as uint8_t,
    (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
        as uint8_t,
    (3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
        as uint8_t,
    (3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
        as uint8_t,
    0,
];
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
        let mut log: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            * (31 as ::core::ffi::c_int
                - (buf | 1 as ::core::ffi::c_uint).leading_zeros() as i32)
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
static mut field_scan8x8: [uint8_t; 65] = [
    (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (1 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (1 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (1 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (2 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (2 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (2 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (2 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (3 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (3 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (3 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (3 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    0,
];
#[inline(always)]
unsafe extern "C" fn av_bswap32(mut x: uint32_t) -> uint32_t {
    return (x << 8 as ::core::ffi::c_int & 0xff00 as uint32_t
        | x >> 8 as ::core::ffi::c_int & 0xff as uint32_t) << 16 as ::core::ffi::c_int
        | ((x >> 16 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            & 0xff00 as uint32_t
            | x >> 16 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
                & 0xff as uint32_t);
}
unsafe extern "C" fn refill(mut c: *mut CABACContext) {
    (*c).low
        += ((*(*c).bytestream.offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int) << 9 as ::core::ffi::c_int)
            + ((*(*c).bytestream.offset(1 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int) << 1 as ::core::ffi::c_int);
    (*c).low
        -= ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int;
    if (*c).bytestream < (*c).bytestream_end {
        (*c).bytestream = (*c)
            .bytestream
            .offset((16 as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as isize);
    }
}
#[inline]
unsafe extern "C" fn av_bswap64(mut x: uint64_t) -> uint64_t {
    return (av_bswap32(x as uint32_t) as uint64_t) << 32 as ::core::ffi::c_int
        | av_bswap32((x >> 32 as ::core::ffi::c_int) as uint32_t) as uint64_t;
}
static mut field_scan8x8_cavlc: [uint8_t; 65] = [
    (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (2 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (3 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (1 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (2 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (3 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (1 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (2 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (3 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (1 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (2 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (3 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    0,
];
#[inline]
unsafe extern "C" fn renorm_cabac_decoder_once(mut c: *mut CABACContext) {
    let mut shift: ::core::ffi::c_int = (((*c).range - 0x100 as ::core::ffi::c_int)
        as uint32_t >> 31 as ::core::ffi::c_int) as ::core::ffi::c_int;
    (*c).range <<= shift;
    (*c).low <<= shift;
    if (*c).low
        & ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int == 0
    {
        refill(c);
    }
}
#[inline]
unsafe extern "C" fn av_cmp_q(
    mut a: AVRational,
    mut b: AVRational,
) -> ::core::ffi::c_int {
    let tmp: int64_t = a.num as int64_t * b.den as int64_t
        - b.num as int64_t * a.den as int64_t;
    if tmp != 0 {
        return ((tmp ^ a.den as int64_t ^ b.den as int64_t) >> 63 as ::core::ffi::c_int)
            as ::core::ffi::c_int | 1 as ::core::ffi::c_int
    } else if b.den != 0 && a.den != 0 {
        return 0 as ::core::ffi::c_int
    } else if a.num != 0 && b.num != 0 {
        return (a.num >> 31 as ::core::ffi::c_int) - (b.num >> 31 as ::core::ffi::c_int)
    } else {
        return -(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int
    };
}
static mut zigzag_scan8x8_cavlc: [uint8_t; 65] = [
    (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (3 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (1 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (2 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (2 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (2 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (3 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (1 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (1 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (3 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (2 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (3 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    0,
];
#[inline]
unsafe extern "C" fn get_ue_golomb_long(
    mut gb: *mut GetBitContext,
) -> ::core::ffi::c_uint {
    let mut buf: ::core::ffi::c_uint = 0;
    let mut log: ::core::ffi::c_uint = 0;
    buf = show_bits_long(gb, 32 as ::core::ffi::c_int);
    log = (31 as ::core::ffi::c_int
        - (31 as ::core::ffi::c_int
            - (buf | 1 as ::core::ffi::c_uint).leading_zeros() as i32))
        as ::core::ffi::c_uint;
    skip_bits_long(gb, log as ::core::ffi::c_int);
    return get_bits_long(
            gb,
            log.wrapping_add(1 as ::core::ffi::c_uint) as ::core::ffi::c_int,
        )
        .wrapping_sub(1 as ::core::ffi::c_uint);
}
unsafe extern "C" fn release_unused_pictures(
    mut h: *mut H264Context,
    mut remove_current: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 36 as ::core::ffi::c_int {
        if !(*(*h).DPB[i as usize].f).buf[0 as ::core::ffi::c_int as usize].is_null()
            && (*h).DPB[i as usize].reference == 0
            && (remove_current != 0
                || (&raw mut (*h).DPB as *mut H264Picture).offset(i as isize)
                    as *mut H264Picture != (*h).cur_pic_ptr)
        {
            ff_h264_unref_picture(
                (&raw mut (*h).DPB as *mut H264Picture).offset(i as isize)
                    as *mut H264Picture,
            );
        }
        i += 1;
    }
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
#[inline(always)]
unsafe extern "C" fn pack16to32(
    mut a: ::core::ffi::c_uint,
    mut b: ::core::ffi::c_uint,
) -> uint32_t {
    return (a as uint32_t & 0xffff as uint32_t)
        .wrapping_add((b as uint32_t) << 16 as ::core::ffi::c_int);
}
unsafe extern "C" fn alloc_scratch_buffers(
    mut sl: *mut H264SliceContext,
    mut linesize: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut h: *const H264Context = (*sl).h264 as *const H264Context;
    let mut alloc_size: ::core::ffi::c_int = (if linesize >= 0 as ::core::ffi::c_int {
        linesize
    } else {
        -linesize
    }) + 32 as ::core::ffi::c_int + 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
        & !(32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int);
    av_fast_malloc(
        &raw mut (*sl).bipred_scratchpad as *mut ::core::ffi::c_void,
        &raw mut (*sl).bipred_scratchpad_allocated as *mut ::core::ffi::c_uint,
        (16 as ::core::ffi::c_int * 6 as ::core::ffi::c_int * alloc_size) as size_t,
    );
    av_fast_malloc(
        &raw mut (*sl).edge_emu_buffer as *mut ::core::ffi::c_void,
        &raw mut (*sl).edge_emu_buffer_allocated as *mut ::core::ffi::c_uint,
        (alloc_size * 2 as ::core::ffi::c_int * 21 as ::core::ffi::c_int) as size_t,
    );
    av_fast_mallocz(
        (&raw mut (*sl).top_borders as *mut *mut [uint8_t; 96])
            .offset(0 as ::core::ffi::c_int as isize) as *mut *mut [uint8_t; 96]
            as *mut ::core::ffi::c_void,
        (&raw mut (*sl).top_borders_allocated as *mut ::core::ffi::c_int)
            .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int
            as *mut ::core::ffi::c_uint,
        (((*h).mb_width * 16 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t)
            .wrapping_mul(2 as size_t),
    );
    av_fast_mallocz(
        (&raw mut (*sl).top_borders as *mut *mut [uint8_t; 96])
            .offset(1 as ::core::ffi::c_int as isize) as *mut *mut [uint8_t; 96]
            as *mut ::core::ffi::c_void,
        (&raw mut (*sl).top_borders_allocated as *mut ::core::ffi::c_int)
            .offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int
            as *mut ::core::ffi::c_uint,
        (((*h).mb_width * 16 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t)
            .wrapping_mul(2 as size_t),
    );
    if (*sl).bipred_scratchpad.is_null() || (*sl).edge_emu_buffer.is_null()
        || (*sl).top_borders[0 as ::core::ffi::c_int as usize].is_null()
        || (*sl).top_borders[1 as ::core::ffi::c_int as usize].is_null()
    {
        av_freep(&raw mut (*sl).bipred_scratchpad as *mut ::core::ffi::c_void);
        av_freep(&raw mut (*sl).edge_emu_buffer as *mut ::core::ffi::c_void);
        av_freep(
            (&raw mut (*sl).top_borders as *mut *mut [uint8_t; 96])
                .offset(0 as ::core::ffi::c_int as isize) as *mut *mut [uint8_t; 96]
                as *mut ::core::ffi::c_void,
        );
        av_freep(
            (&raw mut (*sl).top_borders as *mut *mut [uint8_t; 96])
                .offset(1 as ::core::ffi::c_int as isize) as *mut *mut [uint8_t; 96]
                as *mut ::core::ffi::c_void,
        );
        (*sl).bipred_scratchpad_allocated = 0 as ::core::ffi::c_int;
        (*sl).edge_emu_buffer_allocated = 0 as ::core::ffi::c_int;
        (*sl).top_borders_allocated[0 as ::core::ffi::c_int as usize] = 0
            as ::core::ffi::c_int;
        (*sl).top_borders_allocated[1 as ::core::ffi::c_int as usize] = 0
            as ::core::ffi::c_int;
        return -(12 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn init_table_pools(mut h: *mut H264Context) -> ::core::ffi::c_int {
    let big_mb_num: ::core::ffi::c_int = (*h).mb_stride
        * ((*h).mb_height + 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int;
    let mb_array_size: ::core::ffi::c_int = (*h).mb_stride * (*h).mb_height;
    let b4_stride: ::core::ffi::c_int = (*h).mb_width * 4 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int;
    let b4_array_size: ::core::ffi::c_int = b4_stride * (*h).mb_height
        * 4 as ::core::ffi::c_int;
    (*h).qscale_table_pool = av_refstruct_pool_alloc(
        (big_mb_num + (*h).mb_stride) as size_t,
        0 as ::core::ffi::c_uint,
    ) as *mut AVRefStructPool;
    (*h).mb_type_pool = av_refstruct_pool_alloc(
        ((big_mb_num + (*h).mb_stride) as size_t)
            .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        0 as ::core::ffi::c_uint,
    ) as *mut AVRefStructPool;
    (*h).motion_val_pool = av_refstruct_pool_alloc(
        ((2 as ::core::ffi::c_int * (b4_array_size + 4 as ::core::ffi::c_int)) as size_t)
            .wrapping_mul(::core::mem::size_of::<int16_t>() as size_t),
        0 as ::core::ffi::c_uint,
    ) as *mut AVRefStructPool;
    (*h).ref_index_pool = av_refstruct_pool_alloc(
        (4 as ::core::ffi::c_int * mb_array_size) as size_t,
        0 as ::core::ffi::c_uint,
    ) as *mut AVRefStructPool;
    if (*h).qscale_table_pool.is_null() || (*h).mb_type_pool.is_null()
        || (*h).motion_val_pool.is_null() || (*h).ref_index_pool.is_null()
    {
        av_refstruct_pool_uninit(&raw mut (*h).qscale_table_pool);
        av_refstruct_pool_uninit(&raw mut (*h).mb_type_pool);
        av_refstruct_pool_uninit(&raw mut (*h).motion_val_pool);
        av_refstruct_pool_uninit(&raw mut (*h).ref_index_pool);
        return -(12 as ::core::ffi::c_int);
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
unsafe extern "C" fn get_cabac_terminate(
    mut c: *mut CABACContext,
) -> ::core::ffi::c_int {
    (*c).range -= 2 as ::core::ffi::c_int;
    if (*c).low < (*c).range << 16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int {
        renorm_cabac_decoder_once(c);
        return 0 as ::core::ffi::c_int;
    } else {
        return (*c).bytestream.offset_from((*c).bytestream_start) as ::core::ffi::c_long
            as ::core::ffi::c_int
    };
}
unsafe extern "C" fn alloc_picture(
    mut h: *mut H264Context,
    mut pic: *mut H264Picture,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut i: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !(*(*pic).f).data[0 as ::core::ffi::c_int as usize].is_null() {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"!pic->f->data[0]\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavcodec/h264_slice.c\0" as *const u8 as *const ::core::ffi::c_char,
            192 as ::core::ffi::c_int,
        );
        abort();
    }
    if !(*h).sei.common.itut_t35.lcevc.is_null() {
        ret = ff_frame_new_side_data_from_buf(
            (*h).avctx,
            (*pic).f,
            AV_FRAME_DATA_LCEVC,
            &raw mut (*h).sei.common.itut_t35.lcevc,
        );
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
    }
    (*pic).tf.f = (*pic).f;
    ret = ff_thread_get_ext_buffer(
        (*h).avctx,
        &raw mut (*pic).tf,
        if (*pic).reference != 0 {
            (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    if !(ret < 0 as ::core::ffi::c_int) {
        if (*pic).needs_fg != 0 {
            (*(*pic).f_grain).format = (*(*pic).f).format;
            (*(*pic).f_grain).width = (*(*pic).f).width;
            (*(*pic).f_grain).height = (*(*pic).f).height;
            ret = ff_thread_get_buffer(
                (*h).avctx,
                (*pic).f_grain,
                0 as ::core::ffi::c_int,
            );
            if ret < 0 as ::core::ffi::c_int {
                current_block = 3808817155228481924;
            } else {
                current_block = 17407779659766490442;
            }
        } else {
            current_block = 17407779659766490442;
        }
        match current_block {
            3808817155228481924 => {}
            _ => {
                ret = ff_hwaccel_frame_priv_alloc(
                    (*h).avctx,
                    &raw mut (*pic).hwaccel_picture_private,
                );
                if !(ret < 0 as ::core::ffi::c_int) {
                    if !(*h).decode_error_flags_pool.is_null() {
                        (*pic).decode_error_flags = av_refstruct_pool_get(
                            (*h).decode_error_flags_pool as *mut AVRefStructPool,
                        ) as *mut atomic_int;
                        if (*pic).decode_error_flags.is_null() {
                            current_block = 3808817155228481924;
                        } else {
                            *(*pic).decode_error_flags = 0 as ::core::ffi::c_int;
                            current_block = 12147880666119273379;
                        }
                    } else {
                        current_block = 12147880666119273379;
                    }
                    match current_block {
                        3808817155228481924 => {}
                        _ => {
                            if 0 as ::core::ffi::c_int != 0
                                && (*(*h).avctx).hwaccel.is_null()
                                && (*h).flags
                                    & (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int != 0
                                && !(*(*pic).f)
                                    .data[2 as ::core::ffi::c_int as usize]
                                    .is_null()
                            {
                                let mut h_chroma_shift: ::core::ffi::c_int = 0;
                                let mut v_chroma_shift: ::core::ffi::c_int = 0;
                                av_pix_fmt_get_chroma_sub_sample(
                                    (*(*pic).f).format as AVPixelFormat,
                                    &raw mut h_chroma_shift,
                                    &raw mut v_chroma_shift,
                                );
                                i = 0 as ::core::ffi::c_int;
                                while i
                                    < (if 0 == 0 {
                                        -(-(*(*pic).f).height >> v_chroma_shift)
                                    } else {
                                        (*(*pic).f).height
                                            + ((1 as ::core::ffi::c_int) << v_chroma_shift)
                                            - 1 as ::core::ffi::c_int >> v_chroma_shift
                                    })
                                {
                                    memset(
                                        (*(*pic).f)
                                            .data[1 as ::core::ffi::c_int as usize]
                                            .offset(
                                                ((*(*pic).f).linesize[1 as ::core::ffi::c_int as usize] * i)
                                                    as isize,
                                            ) as *mut ::core::ffi::c_void,
                                        0x80 as ::core::ffi::c_int,
                                        (if 0 == 0 {
                                            -(-(*(*pic).f).width >> h_chroma_shift)
                                        } else {
                                            (*(*pic).f).width
                                                + ((1 as ::core::ffi::c_int) << h_chroma_shift)
                                                - 1 as ::core::ffi::c_int >> h_chroma_shift
                                        }) as size_t,
                                    );
                                    memset(
                                        (*(*pic).f)
                                            .data[2 as ::core::ffi::c_int as usize]
                                            .offset(
                                                ((*(*pic).f).linesize[2 as ::core::ffi::c_int as usize] * i)
                                                    as isize,
                                            ) as *mut ::core::ffi::c_void,
                                        0x80 as ::core::ffi::c_int,
                                        (if 0 == 0 {
                                            -(-(*(*pic).f).width >> h_chroma_shift)
                                        } else {
                                            (*(*pic).f).width
                                                + ((1 as ::core::ffi::c_int) << h_chroma_shift)
                                                - 1 as ::core::ffi::c_int >> h_chroma_shift
                                        }) as size_t,
                                    );
                                    i += 1;
                                }
                            }
                            if (*h).qscale_table_pool.is_null() {
                                ret = init_table_pools(h);
                                if ret < 0 as ::core::ffi::c_int {
                                    current_block = 3808817155228481924;
                                } else {
                                    current_block = 11459959175219260272;
                                }
                            } else {
                                current_block = 11459959175219260272;
                            }
                            match current_block {
                                3808817155228481924 => {}
                                _ => {
                                    (*pic).qscale_table_base = av_refstruct_pool_get(
                                        (*h).qscale_table_pool as *mut AVRefStructPool,
                                    ) as *mut int8_t;
                                    (*pic).mb_type_base = av_refstruct_pool_get(
                                        (*h).mb_type_pool as *mut AVRefStructPool,
                                    ) as *mut uint32_t;
                                    if !((*pic).qscale_table_base.is_null()
                                        || (*pic).mb_type_base.is_null())
                                    {
                                        (*pic).mb_type = (*pic)
                                            .mb_type_base
                                            .offset((2 as ::core::ffi::c_int * (*h).mb_stride) as isize)
                                            .offset(1 as ::core::ffi::c_int as isize);
                                        (*pic).qscale_table = (*pic)
                                            .qscale_table_base
                                            .offset((2 as ::core::ffi::c_int * (*h).mb_stride) as isize)
                                            .offset(1 as ::core::ffi::c_int as isize);
                                        i = 0 as ::core::ffi::c_int;
                                        loop {
                                            if !(i < 2 as ::core::ffi::c_int) {
                                                current_block = 9007357115414505193;
                                                break;
                                            }
                                            (*pic).motion_val_base[i as usize] = av_refstruct_pool_get(
                                                (*h).motion_val_pool as *mut AVRefStructPool,
                                            ) as *mut [int16_t; 2];
                                            (*pic).ref_index[i as usize] = av_refstruct_pool_get(
                                                (*h).ref_index_pool as *mut AVRefStructPool,
                                            ) as *mut int8_t;
                                            if (*pic).motion_val_base[i as usize].is_null()
                                                || (*pic).ref_index[i as usize].is_null()
                                            {
                                                current_block = 3808817155228481924;
                                                break;
                                            }
                                            (*pic).motion_val[i as usize] = (*pic)
                                                .motion_val_base[i as usize]
                                                .offset(4 as ::core::ffi::c_int as isize);
                                            i += 1;
                                        }
                                        match current_block {
                                            3808817155228481924 => {}
                                            _ => {
                                                (*pic).pps = av_refstruct_ref_c(
                                                    (*h).ps.pps as *const ::core::ffi::c_void,
                                                ) as *const PPS;
                                                (*pic).mb_width = (*h).mb_width;
                                                (*pic).mb_height = (*h).mb_height;
                                                (*pic).mb_stride = (*h).mb_stride;
                                                return 0 as ::core::ffi::c_int;
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
    ff_h264_unref_picture(pic);
    return if ret < 0 as ::core::ffi::c_int { ret } else { -(12 as ::core::ffi::c_int) };
}
#[inline(always)]
unsafe extern "C" fn av_clip_int8_c(mut a: ::core::ffi::c_int) -> int8_t {
    if (a as ::core::ffi::c_uint).wrapping_add(0x80 as ::core::ffi::c_uint)
        & !(0xff as ::core::ffi::c_int) as ::core::ffi::c_uint != 0
    {
        return (a >> 31 as ::core::ffi::c_int ^ 0x7f as ::core::ffi::c_int) as int8_t
    } else {
        return a as int8_t
    };
}
#[inline]
unsafe extern "C" fn get_se_golomb(mut gb: *mut GetBitContext) -> ::core::ffi::c_int {
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
        return ff_se_golomb_vlc_code[buf as usize] as ::core::ffi::c_int;
    } else {
        let mut log: ::core::ffi::c_int = 31 as ::core::ffi::c_int
            - (buf | 1 as ::core::ffi::c_uint).leading_zeros() as i32;
        let mut sign: ::core::ffi::c_int = 0;
        re_index = if re_size_plus8
            > re_index
                .wrapping_add((31 as ::core::ffi::c_int - log) as ::core::ffi::c_uint)
        {
            re_index
                .wrapping_add((31 as ::core::ffi::c_int - log) as ::core::ffi::c_uint)
        } else {
            re_size_plus8
        };
        re_cache = (av_bswap32(
            (*((*gb).buffer.offset((re_index >> 3 as ::core::ffi::c_int) as isize)
                as *const unaligned_32))
                .l,
        ) << (re_index & 7 as ::core::ffi::c_uint)
            >> 32 as ::core::ffi::c_int - 32 as ::core::ffi::c_int)
            as ::core::ffi::c_uint;
        buf = re_cache as ::core::ffi::c_uint;
        buf >>= log;
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
        sign = (buf & 1 as ::core::ffi::c_uint).wrapping_neg() as ::core::ffi::c_int;
        buf = (buf >> 1 as ::core::ffi::c_int ^ sign as ::core::ffi::c_uint)
            .wrapping_sub(sign as ::core::ffi::c_uint);
        return buf as ::core::ffi::c_int;
    };
}
#[inline]
unsafe extern "C" fn get_bits_count(mut s: *const GetBitContext) -> ::core::ffi::c_int {
    return (*s).index;
}
unsafe extern "C" fn find_unused_picture(
    mut h: *const H264Context,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 36 as ::core::ffi::c_int {
        if (*(*h).DPB[i as usize].f).buf[0 as ::core::ffi::c_int as usize].is_null() {
            return i;
        }
        i += 1;
    }
    return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
        | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
        | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
        as ::core::ffi::c_int);
}
#[inline]
unsafe extern "C" fn skip_bits_long(
    mut s: *mut GetBitContext,
    mut n: ::core::ffi::c_int,
) {
    (*s).index += av_clip_c(n, -(*s).index, (*s).size_in_bits_plus8 - (*s).index);
}
#[inline]
unsafe extern "C" fn av_refstruct_pool_uninit(mut poolp: *mut *mut AVRefStructPool) {
    av_refstruct_unref(poolp as *mut ::core::ffi::c_void);
}
#[inline(always)]
unsafe extern "C" fn av_zero_extend_c(
    mut a: ::core::ffi::c_uint,
    mut p: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    return a & ((1 as ::core::ffi::c_uint) << p).wrapping_sub(1 as ::core::ffi::c_uint);
}
unsafe extern "C" fn copy_picture_range(
    mut to: *mut *mut H264Picture,
    mut from: *const *mut H264Picture,
    mut count: ::core::ffi::c_int,
    mut new_base: *mut H264Context,
    mut old_base: *const H264Context,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < count {
        let ref mut fresh49 = *to.offset(i as isize);
        *fresh49 = if !(*from.offset(i as isize)).is_null()
            && *from.offset(i as isize)
                >= &raw const (*old_base).DPB as *const H264Picture as *mut H264Picture
            && *from.offset(i as isize)
                < (&raw const (*old_base).DPB as *const H264Picture)
                    .offset(36 as ::core::ffi::c_int as isize) as *mut H264Picture
        {
            (&raw mut (*new_base).DPB as *mut H264Picture)
                .offset(
                    (*from.offset(i as isize))
                        .offset_from(&raw const (*old_base).DPB as *const H264Picture)
                        as ::core::ffi::c_long as isize,
                ) as *mut H264Picture
        } else {
            ::core::ptr::null_mut::<H264Picture>()
        };
        i += 1;
    }
}
unsafe extern "C" fn color_frame(
    mut frame: *mut AVFrame,
    mut c: *const ::core::ffi::c_int,
) {
    let mut desc: *const AVPixFmtDescriptor = av_pix_fmt_desc_get(
        (*frame).format as AVPixelFormat,
    );
    if (*desc).flags & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t
        == 0
    {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"desc->flags & (1 << 4)\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavcodec/h264_slice.c\0" as *const u8 as *const ::core::ffi::c_char,
            310 as ::core::ffi::c_int,
        );
        abort();
    }
    let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while p < (*desc).nb_components as ::core::ffi::c_int {
        let mut dst: *mut uint8_t = (*frame).data[p as usize];
        let mut is_chroma: ::core::ffi::c_int = (p == 1 as ::core::ffi::c_int
            || p == 2 as ::core::ffi::c_int) as ::core::ffi::c_int;
        let mut bytes: ::core::ffi::c_int = if is_chroma != 0 {
            if 0 == 0 {
                -(-(*frame).width >> (*desc).log2_chroma_w as ::core::ffi::c_int)
            } else {
                (*frame).width
                    + ((1 as ::core::ffi::c_int)
                        << (*desc).log2_chroma_w as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int
                    >> (*desc).log2_chroma_w as ::core::ffi::c_int
            }
        } else {
            (*frame).width
        };
        let mut height: ::core::ffi::c_int = if is_chroma != 0 {
            if 0 == 0 {
                -(-(*frame).height >> (*desc).log2_chroma_h as ::core::ffi::c_int)
            } else {
                (*frame).height
                    + ((1 as ::core::ffi::c_int)
                        << (*desc).log2_chroma_h as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int
                    >> (*desc).log2_chroma_h as ::core::ffi::c_int
            }
        } else {
            (*frame).height
        };
        if (*desc).comp[0 as ::core::ffi::c_int as usize].depth
            >= 9 as ::core::ffi::c_int
        {
            if bytes >= 1 as ::core::ffi::c_int {
                *(dst as *mut uint16_t).offset(0 as ::core::ffi::c_int as isize) = *c
                    .offset(p as isize) as uint16_t;
            }
            if bytes >= 2 as ::core::ffi::c_int {
                av_memcpy_backptr(
                    dst.offset(2 as ::core::ffi::c_int as isize),
                    2 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int * (bytes - 1 as ::core::ffi::c_int),
                );
            }
            dst = dst.offset((*frame).linesize[p as usize] as isize);
            let mut y: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while y < height {
                memcpy(
                    dst as *mut ::core::ffi::c_void,
                    (*frame).data[p as usize] as *const ::core::ffi::c_void,
                    (2 as ::core::ffi::c_int * bytes) as size_t,
                );
                dst = dst.offset((*frame).linesize[p as usize] as isize);
                y += 1;
            }
        } else {
            let mut y_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while y_0 < height {
                memset(
                    dst as *mut ::core::ffi::c_void,
                    *c.offset(p as isize),
                    bytes as size_t,
                );
                dst = dst.offset((*frame).linesize[p as usize] as isize);
                y_0 += 1;
            }
        }
        p += 1;
    }
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
pub unsafe extern "C" fn ff_h264_update_thread_context(
    mut dst: *mut AVCodecContext,
    mut src: *const AVCodecContext,
) -> ::core::ffi::c_int {
    let mut h: *mut H264Context = (*dst).priv_data as *mut H264Context;
    let mut h1: *mut H264Context = (*src).priv_data as *mut H264Context;
    let mut inited: ::core::ffi::c_int = (*h).context_initialized;
    let mut err: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut need_reinit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    if dst == src as *mut AVCodecContext {
        return 0 as ::core::ffi::c_int;
    }
    if inited != 0 && (*h1).ps.sps.is_null() {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if inited != 0
        && ((*h).width != (*h1).width || (*h).height != (*h1).height
            || (*h).mb_width != (*h1).mb_width || (*h).mb_height != (*h1).mb_height
            || (*h).ps.sps.is_null()
            || (*(*h).ps.sps).bit_depth_luma != (*(*h1).ps.sps).bit_depth_luma
            || (*(*h).ps.sps).chroma_format_idc != (*(*h1).ps.sps).chroma_format_idc
            || (*(*h).ps.sps).vui.matrix_coeffs as ::core::ffi::c_uint
                != (*(*h1).ps.sps).vui.matrix_coeffs as ::core::ffi::c_uint)
    {
        need_reinit = 1 as ::core::ffi::c_int;
    }
    memcpy(
        &raw mut (*h).block_offset as *mut ::core::ffi::c_int
            as *mut ::core::ffi::c_void,
        &raw mut (*h1).block_offset as *mut ::core::ffi::c_int
            as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_int; 96]>() as size_t,
    );
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (i_0 as usize)
        < (::core::mem::size_of::<[*const SPS; 32]>() as usize)
            .wrapping_div(::core::mem::size_of::<*const SPS>() as usize)
    {
        av_refstruct_replace(
            (&raw mut (*h).ps.sps_list as *mut *const SPS).offset(i_0 as isize)
                as *mut *const SPS as *mut ::core::ffi::c_void,
            (*h1).ps.sps_list[i_0 as usize] as *const ::core::ffi::c_void,
        );
        i_0 += 1;
    }
    let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (i_1 as usize)
        < (::core::mem::size_of::<[*const PPS; 256]>() as usize)
            .wrapping_div(::core::mem::size_of::<*const PPS>() as usize)
    {
        av_refstruct_replace(
            (&raw mut (*h).ps.pps_list as *mut *const PPS).offset(i_1 as isize)
                as *mut *const PPS as *mut ::core::ffi::c_void,
            (*h1).ps.pps_list[i_1 as usize] as *const ::core::ffi::c_void,
        );
        i_1 += 1;
    }
    av_refstruct_replace(
        &raw mut (*h).ps.pps as *mut ::core::ffi::c_void,
        (*h1).ps.pps as *const ::core::ffi::c_void,
    );
    (*h).ps.sps = (*h1).ps.sps;
    if need_reinit != 0 || inited == 0 {
        (*h).width = (*h1).width;
        (*h).height = (*h1).height;
        (*h).mb_height = (*h1).mb_height;
        (*h).mb_width = (*h1).mb_width;
        (*h).mb_num = (*h1).mb_num;
        (*h).mb_stride = (*h1).mb_stride;
        (*h).b_stride = (*h1).b_stride;
        (*h).x264_build = (*h1).x264_build;
        if (*h).context_initialized != 0 || (*h1).context_initialized != 0 {
            err = h264_slice_header_init(h);
            if err < 0 as ::core::ffi::c_int {
                av_log(
                    (*h).avctx as *mut ::core::ffi::c_void,
                    16 as ::core::ffi::c_int,
                    b"h264_slice_header_init() failed\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return err;
            }
        }
        memcpy(
            &raw mut (*h).block_offset as *mut ::core::ffi::c_int
                as *mut ::core::ffi::c_void,
            &raw mut (*h1).block_offset as *mut ::core::ffi::c_int
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_int; 96]>() as size_t,
        );
    }
    (*h).width_from_caller = (*h1).width_from_caller;
    (*h).height_from_caller = (*h1).height_from_caller;
    (*h).first_field = (*h1).first_field;
    (*h).picture_structure = (*h1).picture_structure;
    (*h).mb_aff_frame = (*h1).mb_aff_frame;
    (*h).droppable = (*h1).droppable;
    i = 0 as ::core::ffi::c_int;
    while i < 36 as ::core::ffi::c_int {
        ret = ff_h264_replace_picture(
            (&raw mut (*h).DPB as *mut H264Picture).offset(i as isize)
                as *mut H264Picture,
            (&raw mut (*h1).DPB as *mut H264Picture).offset(i as isize)
                as *mut H264Picture,
        );
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        i += 1;
    }
    (*h).cur_pic_ptr = if !(*h1).cur_pic_ptr.is_null()
        && (*h1).cur_pic_ptr >= &raw mut (*h1).DPB as *mut H264Picture
        && (*h1).cur_pic_ptr
            < (&raw mut (*h1).DPB as *mut H264Picture)
                .offset(36 as ::core::ffi::c_int as isize)
    {
        (&raw mut (*h).DPB as *mut H264Picture)
            .offset(
                (*h1).cur_pic_ptr.offset_from(&raw mut (*h1).DPB as *mut H264Picture)
                    as ::core::ffi::c_long as isize,
            ) as *mut H264Picture
    } else {
        ::core::ptr::null_mut::<H264Picture>()
    };
    ret = ff_h264_replace_picture(&raw mut (*h).cur_pic, &raw mut (*h1).cur_pic);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    (*h).enable_er = (*h1).enable_er;
    (*h).workaround_bugs = (*h1).workaround_bugs;
    (*h).droppable = (*h1).droppable;
    (*h).is_avc = (*h1).is_avc;
    (*h).nal_length_size = (*h1).nal_length_size;
    memcpy(
        &raw mut (*h).poc as *mut ::core::ffi::c_void,
        &raw mut (*h1).poc as *const ::core::ffi::c_void,
        ::core::mem::size_of::<H264POCContext>() as size_t,
    );
    memcpy(
        &raw mut (*h).short_ref as *mut *mut H264Picture as *mut ::core::ffi::c_void,
        &raw mut (*h1).short_ref as *mut *mut H264Picture as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[*mut H264Picture; 32]>() as size_t,
    );
    memcpy(
        &raw mut (*h).long_ref as *mut *mut H264Picture as *mut ::core::ffi::c_void,
        &raw mut (*h1).long_ref as *mut *mut H264Picture as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[*mut H264Picture; 32]>() as size_t,
    );
    memcpy(
        &raw mut (*h).delayed_pic as *mut *mut H264Picture as *mut ::core::ffi::c_void,
        &raw mut (*h1).delayed_pic as *mut *mut H264Picture
            as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[*mut H264Picture; 18]>() as size_t,
    );
    memcpy(
        &raw mut (*h).last_pocs as *mut ::core::ffi::c_int as *mut ::core::ffi::c_void,
        &raw mut (*h1).last_pocs as *mut ::core::ffi::c_int
            as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_int; 16]>() as size_t,
    );
    (*h).next_output_pic = (*h1).next_output_pic;
    (*h).next_outputed_poc = (*h1).next_outputed_poc;
    (*h).poc_offset = (*h1).poc_offset;
    memcpy(
        &raw mut (*h).mmco as *mut MMCO as *mut ::core::ffi::c_void,
        &raw mut (*h1).mmco as *mut MMCO as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[MMCO; 67]>() as size_t,
    );
    (*h).nb_mmco = (*h1).nb_mmco;
    (*h).mmco_reset = (*h1).mmco_reset;
    (*h).explicit_ref_marking = (*h1).explicit_ref_marking;
    (*h).long_ref_count = (*h1).long_ref_count;
    (*h).short_ref_count = (*h1).short_ref_count;
    copy_picture_range(
        &raw mut (*h).short_ref as *mut *mut H264Picture,
        &raw mut (*h1).short_ref as *mut *mut H264Picture,
        32 as ::core::ffi::c_int,
        h,
        h1,
    );
    copy_picture_range(
        &raw mut (*h).long_ref as *mut *mut H264Picture,
        &raw mut (*h1).long_ref as *mut *mut H264Picture,
        32 as ::core::ffi::c_int,
        h,
        h1,
    );
    copy_picture_range(
        &raw mut (*h).delayed_pic as *mut *mut H264Picture,
        &raw mut (*h1).delayed_pic as *mut *mut H264Picture,
        (::core::mem::size_of::<[*mut H264Picture; 18]>() as usize)
            .wrapping_div(::core::mem::size_of::<*mut H264Picture>() as usize)
            as ::core::ffi::c_int,
        h,
        h1,
    );
    (*h).frame_recovered = (*h1).frame_recovered;
    ret = ff_h2645_sei_ctx_replace(&raw mut (*h).sei.common, &raw mut (*h1).sei.common);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    (*h).sei.common.unregistered.x264_build = (*h1).sei.common.unregistered.x264_build;
    if (*h).cur_pic_ptr.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if (*h).droppable == 0 {
        err = ff_h264_execute_ref_pic_marking(h);
        (*h).poc.prev_poc_msb = (*h).poc.poc_msb;
        (*h).poc.prev_poc_lsb = (*h).poc.poc_lsb;
    }
    (*h).poc.prev_frame_num_offset = (*h).poc.frame_num_offset;
    (*h).poc.prev_frame_num = (*h).poc.frame_num;
    (*h).recovery_frame = (*h1).recovery_frame;
    (*h).non_gray = (*h1).non_gray;
    return err;
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
pub unsafe extern "C" fn ff_h264_update_thread_context_for_user(
    mut dst: *mut AVCodecContext,
    mut src: *const AVCodecContext,
) -> ::core::ffi::c_int {
    let mut h: *mut H264Context = (*dst).priv_data as *mut H264Context;
    let mut h1: *const H264Context = (*src).priv_data as *const H264Context;
    (*h).is_avc = (*h1).is_avc;
    (*h).nal_length_size = (*h1).nal_length_size;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn h264_frame_start(mut h: *mut H264Context) -> ::core::ffi::c_int {
    let mut pic: *mut H264Picture = ::core::ptr::null_mut::<H264Picture>();
    let mut i: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let pixel_shift: ::core::ffi::c_int = (*h).pixel_shift;
    if ff_thread_can_start_frame((*h).avctx) == 0 {
        av_log(
            (*h).avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"Attempt to start a frame outside SETUP state\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -((('B' as i32 | ('U' as i32) << 8 as ::core::ffi::c_int
            | ('G' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('!' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    release_unused_pictures(h, 1 as ::core::ffi::c_int);
    (*h).cur_pic_ptr = ::core::ptr::null_mut::<H264Picture>();
    i = find_unused_picture(h);
    if i < 0 as ::core::ffi::c_int {
        av_log(
            (*h).avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"no frame buffer available\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return i;
    }
    pic = (&raw mut (*h).DPB as *mut H264Picture).offset(i as isize) as *mut H264Picture;
    (*pic).reference = if (*h).droppable != 0 {
        0 as ::core::ffi::c_int
    } else {
        (*h).picture_structure
    };
    (*pic).field_picture = ((*h).picture_structure != 3 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    (*pic).frame_num = (*h).poc.frame_num;
    (*(*pic).f).flags &= !((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int);
    (*pic).mmco_reset = 0 as ::core::ffi::c_int;
    (*pic).recovered = 0 as ::core::ffi::c_int;
    (*pic).invalid_gap = 0 as ::core::ffi::c_int;
    (*pic).sei_recovery_frame_cnt = (*h).sei.recovery_point.recovery_frame_cnt;
    (*(*pic).f).pict_type = (*(*h).slice_ctx.offset(0 as ::core::ffi::c_int as isize))
        .slice_type as AVPictureType;
    (*(*pic).f).crop_left = (*h).crop_left as size_t;
    (*(*pic).f).crop_right = (*h).crop_right as size_t;
    (*(*pic).f).crop_top = (*h).crop_top as size_t;
    (*(*pic).f).crop_bottom = (*h).crop_bottom as size_t;
    (*pic).needs_fg = (!(*h).sei.common.film_grain_characteristics.is_null()
        && (*(*h).sei.common.film_grain_characteristics).present != 0
        && (*(*h).avctx).hwaccel.is_null()
        && (*(*h).avctx).export_side_data
            & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int == 0)
        as ::core::ffi::c_int;
    ret = alloc_picture(h, pic);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    (*h).cur_pic_ptr = pic;
    ff_h264_unref_picture(&raw mut (*h).cur_pic);
    ff_h264_set_erpic(&raw mut (*h).er.cur_pic, ::core::ptr::null::<H264Picture>());
    ret = ff_h264_ref_picture(&raw mut (*h).cur_pic, (*h).cur_pic_ptr);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*h).nb_slice_ctx {
        (*(*h).slice_ctx.offset(i as isize)).linesize = (*(*(*h).cur_pic_ptr).f)
            .linesize[0 as ::core::ffi::c_int as usize] as ptrdiff_t;
        (*(*h).slice_ctx.offset(i as isize)).uvlinesize = (*(*(*h).cur_pic_ptr).f)
            .linesize[1 as ::core::ffi::c_int as usize] as ptrdiff_t;
        i += 1;
    }
    if 1 as ::core::ffi::c_int != 0 && (*h).enable_er != 0 {
        ff_er_frame_start(&raw mut (*h).er);
        ff_h264_set_erpic(&raw mut (*h).er.last_pic, ::core::ptr::null::<H264Picture>());
        ff_h264_set_erpic(&raw mut (*h).er.next_pic, ::core::ptr::null::<H264Picture>());
    }
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        (*h).block_offset[i as usize] = (4 as ::core::ffi::c_int
            * (scan8[i as usize] as ::core::ffi::c_int
                - scan8[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                & 7 as ::core::ffi::c_int) << pixel_shift)
            + 4 as ::core::ffi::c_int
                * (*(*pic).f).linesize[0 as ::core::ffi::c_int as usize]
                * (scan8[i as usize] as ::core::ffi::c_int
                    - scan8[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    >> 3 as ::core::ffi::c_int);
        (*h).block_offset[(48 as ::core::ffi::c_int + i) as usize] = (4
            as ::core::ffi::c_int
            * (scan8[i as usize] as ::core::ffi::c_int
                - scan8[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                & 7 as ::core::ffi::c_int) << pixel_shift)
            + 8 as ::core::ffi::c_int
                * (*(*pic).f).linesize[0 as ::core::ffi::c_int as usize]
                * (scan8[i as usize] as ::core::ffi::c_int
                    - scan8[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    >> 3 as ::core::ffi::c_int);
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        (*h).block_offset[(32 as ::core::ffi::c_int + i) as usize] = (4
            as ::core::ffi::c_int
            * (scan8[i as usize] as ::core::ffi::c_int
                - scan8[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                & 7 as ::core::ffi::c_int) << pixel_shift)
            + 4 as ::core::ffi::c_int
                * (*(*pic).f).linesize[1 as ::core::ffi::c_int as usize]
                * (scan8[i as usize] as ::core::ffi::c_int
                    - scan8[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    >> 3 as ::core::ffi::c_int);
        (*h).block_offset[(16 as ::core::ffi::c_int + i) as usize] = (*h)
            .block_offset[(32 as ::core::ffi::c_int + i) as usize];
        (*h)
            .block_offset[(48 as ::core::ffi::c_int + 32 as ::core::ffi::c_int + i)
            as usize] = (4 as ::core::ffi::c_int
            * (scan8[i as usize] as ::core::ffi::c_int
                - scan8[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                & 7 as ::core::ffi::c_int) << pixel_shift)
            + 8 as ::core::ffi::c_int
                * (*(*pic).f).linesize[1 as ::core::ffi::c_int as usize]
                * (scan8[i as usize] as ::core::ffi::c_int
                    - scan8[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    >> 3 as ::core::ffi::c_int);
        (*h)
            .block_offset[(48 as ::core::ffi::c_int + 16 as ::core::ffi::c_int + i)
            as usize] = (*h)
            .block_offset[(48 as ::core::ffi::c_int + 32 as ::core::ffi::c_int + i)
            as usize];
        i += 1;
    }
    (*(*h).cur_pic_ptr).reference = 0 as ::core::ffi::c_int;
    (*(*h).cur_pic_ptr).field_poc[1 as ::core::ffi::c_int as usize] = 2147483647
        as ::core::ffi::c_int;
    (*(*h).cur_pic_ptr).field_poc[0 as ::core::ffi::c_int as usize] = (*(*h).cur_pic_ptr)
        .field_poc[1 as ::core::ffi::c_int as usize];
    (*h).next_output_pic = ::core::ptr::null_mut::<H264Picture>();
    (*h).postpone_filter = 0 as ::core::ffi::c_int;
    (*h).mb_aff_frame = ((*(*h).ps.sps).mb_aff != 0
        && (*h).picture_structure == 3 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if (*h).sei.common.unregistered.x264_build >= 0 as ::core::ffi::c_int {
        (*h).x264_build = (*h).sei.common.unregistered.x264_build;
    }
    return 0 as ::core::ffi::c_int;
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
unsafe extern "C" fn align_get_bits(mut s: *mut GetBitContext) -> *const uint8_t {
    let mut n: ::core::ffi::c_int = -get_bits_count(s) & 7 as ::core::ffi::c_int;
    if n != 0 {
        skip_bits(s, n);
    }
    return (*s).buffer.offset(((*s).index >> 3 as ::core::ffi::c_int) as isize);
}
#[inline(always)]
unsafe extern "C" fn backup_mb_border(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut src_y: *const uint8_t,
    mut src_cb: *const uint8_t,
    mut src_cr: *const uint8_t,
    mut linesize: ::core::ffi::c_int,
    mut uvlinesize: ::core::ffi::c_int,
    mut simple: ::core::ffi::c_int,
) {
    let mut top_border: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut top_idx: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let pixel_shift: ::core::ffi::c_int = (*h).pixel_shift;
    let mut chroma444: ::core::ffi::c_int = ((*(*h).ps.sps).chroma_format_idc
        == 3 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut chroma422: ::core::ffi::c_int = ((*(*h).ps.sps).chroma_format_idc
        == 2 as ::core::ffi::c_int) as ::core::ffi::c_int;
    src_y = src_y.offset(-(linesize as isize));
    src_cb = src_cb.offset(-(uvlinesize as isize));
    src_cr = src_cr.offset(-(uvlinesize as isize));
    if simple == 0 && (*h).mb_aff_frame != 0 {
        if (*sl).mb_y & 1 as ::core::ffi::c_int != 0 {
            if (*sl).mb_mbaff == 0 {
                top_border = &raw mut *(*(&raw mut (*sl).top_borders
                    as *mut *mut [uint8_t; 96])
                    .offset(0 as ::core::ffi::c_int as isize))
                    .offset((*sl).mb_x as isize) as *mut uint8_t;
                (*(top_border as *mut av_alias64)).u64_0 = (*(src_y
                    .offset((15 as ::core::ffi::c_int * linesize) as isize)
                    as *const av_alias64))
                    .u64_0;
                (*((top_border as *mut ::core::ffi::c_char)
                    .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64))
                    .u64_0 = (*((src_y
                    .offset((15 as ::core::ffi::c_int * linesize) as isize)
                    as *mut ::core::ffi::c_char)
                    .offset(8 as ::core::ffi::c_int as isize) as *const av_alias64))
                    .u64_0;
                if pixel_shift != 0 {
                    (*(top_border.offset(16 as ::core::ffi::c_int as isize)
                        as *mut av_alias64))
                        .u64_0 = (*(src_y
                        .offset((15 as ::core::ffi::c_int * linesize) as isize)
                        .offset(16 as ::core::ffi::c_int as isize) as *const av_alias64))
                        .u64_0;
                    (*((top_border.offset(16 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char)
                        .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64))
                        .u64_0 = (*((src_y
                        .offset((15 as ::core::ffi::c_int * linesize) as isize)
                        .offset(16 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char)
                        .offset(8 as ::core::ffi::c_int as isize) as *const av_alias64))
                        .u64_0;
                }
                if simple != 0 || 0 as ::core::ffi::c_int == 0
                    || (*h).flags & (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int
                        == 0
                {
                    if chroma444 != 0 {
                        if pixel_shift != 0 {
                            (*(top_border.offset(32 as ::core::ffi::c_int as isize)
                                as *mut av_alias64))
                                .u64_0 = (*(src_cb
                                .offset((15 as ::core::ffi::c_int * uvlinesize) as isize)
                                as *const av_alias64))
                                .u64_0;
                            (*((top_border.offset(32 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_char)
                                .offset(8 as ::core::ffi::c_int as isize)
                                as *mut av_alias64))
                                .u64_0 = (*((src_cb
                                .offset((15 as ::core::ffi::c_int * uvlinesize) as isize)
                                as *mut ::core::ffi::c_char)
                                .offset(8 as ::core::ffi::c_int as isize)
                                as *const av_alias64))
                                .u64_0;
                            (*(top_border.offset(48 as ::core::ffi::c_int as isize)
                                as *mut av_alias64))
                                .u64_0 = (*(src_cb
                                .offset((15 as ::core::ffi::c_int * uvlinesize) as isize)
                                .offset(16 as ::core::ffi::c_int as isize)
                                as *const av_alias64))
                                .u64_0;
                            (*((top_border.offset(48 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_char)
                                .offset(8 as ::core::ffi::c_int as isize)
                                as *mut av_alias64))
                                .u64_0 = (*((src_cb
                                .offset((15 as ::core::ffi::c_int * uvlinesize) as isize)
                                .offset(16 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_char)
                                .offset(8 as ::core::ffi::c_int as isize)
                                as *const av_alias64))
                                .u64_0;
                            (*(top_border.offset(64 as ::core::ffi::c_int as isize)
                                as *mut av_alias64))
                                .u64_0 = (*(src_cr
                                .offset((15 as ::core::ffi::c_int * uvlinesize) as isize)
                                as *const av_alias64))
                                .u64_0;
                            (*((top_border.offset(64 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_char)
                                .offset(8 as ::core::ffi::c_int as isize)
                                as *mut av_alias64))
                                .u64_0 = (*((src_cr
                                .offset((15 as ::core::ffi::c_int * uvlinesize) as isize)
                                as *mut ::core::ffi::c_char)
                                .offset(8 as ::core::ffi::c_int as isize)
                                as *const av_alias64))
                                .u64_0;
                            (*(top_border.offset(80 as ::core::ffi::c_int as isize)
                                as *mut av_alias64))
                                .u64_0 = (*(src_cr
                                .offset((15 as ::core::ffi::c_int * uvlinesize) as isize)
                                .offset(16 as ::core::ffi::c_int as isize)
                                as *const av_alias64))
                                .u64_0;
                            (*((top_border.offset(80 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_char)
                                .offset(8 as ::core::ffi::c_int as isize)
                                as *mut av_alias64))
                                .u64_0 = (*((src_cr
                                .offset((15 as ::core::ffi::c_int * uvlinesize) as isize)
                                .offset(16 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_char)
                                .offset(8 as ::core::ffi::c_int as isize)
                                as *const av_alias64))
                                .u64_0;
                        } else {
                            (*(top_border.offset(16 as ::core::ffi::c_int as isize)
                                as *mut av_alias64))
                                .u64_0 = (*(src_cb
                                .offset((15 as ::core::ffi::c_int * uvlinesize) as isize)
                                as *const av_alias64))
                                .u64_0;
                            (*((top_border.offset(16 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_char)
                                .offset(8 as ::core::ffi::c_int as isize)
                                as *mut av_alias64))
                                .u64_0 = (*((src_cb
                                .offset((15 as ::core::ffi::c_int * uvlinesize) as isize)
                                as *mut ::core::ffi::c_char)
                                .offset(8 as ::core::ffi::c_int as isize)
                                as *const av_alias64))
                                .u64_0;
                            (*(top_border.offset(32 as ::core::ffi::c_int as isize)
                                as *mut av_alias64))
                                .u64_0 = (*(src_cr
                                .offset((15 as ::core::ffi::c_int * uvlinesize) as isize)
                                as *const av_alias64))
                                .u64_0;
                            (*((top_border.offset(32 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_char)
                                .offset(8 as ::core::ffi::c_int as isize)
                                as *mut av_alias64))
                                .u64_0 = (*((src_cr
                                .offset((15 as ::core::ffi::c_int * uvlinesize) as isize)
                                as *mut ::core::ffi::c_char)
                                .offset(8 as ::core::ffi::c_int as isize)
                                as *const av_alias64))
                                .u64_0;
                        }
                    } else if chroma422 != 0 {
                        if pixel_shift != 0 {
                            (*(top_border.offset(32 as ::core::ffi::c_int as isize)
                                as *mut av_alias64))
                                .u64_0 = (*(src_cb
                                .offset((15 as ::core::ffi::c_int * uvlinesize) as isize)
                                as *const av_alias64))
                                .u64_0;
                            (*((top_border.offset(32 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_char)
                                .offset(8 as ::core::ffi::c_int as isize)
                                as *mut av_alias64))
                                .u64_0 = (*((src_cb
                                .offset((15 as ::core::ffi::c_int * uvlinesize) as isize)
                                as *mut ::core::ffi::c_char)
                                .offset(8 as ::core::ffi::c_int as isize)
                                as *const av_alias64))
                                .u64_0;
                            (*(top_border.offset(48 as ::core::ffi::c_int as isize)
                                as *mut av_alias64))
                                .u64_0 = (*(src_cr
                                .offset((15 as ::core::ffi::c_int * uvlinesize) as isize)
                                as *const av_alias64))
                                .u64_0;
                            (*((top_border.offset(48 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_char)
                                .offset(8 as ::core::ffi::c_int as isize)
                                as *mut av_alias64))
                                .u64_0 = (*((src_cr
                                .offset((15 as ::core::ffi::c_int * uvlinesize) as isize)
                                as *mut ::core::ffi::c_char)
                                .offset(8 as ::core::ffi::c_int as isize)
                                as *const av_alias64))
                                .u64_0;
                        } else {
                            (*(top_border.offset(16 as ::core::ffi::c_int as isize)
                                as *mut av_alias64))
                                .u64_0 = (*(src_cb
                                .offset((15 as ::core::ffi::c_int * uvlinesize) as isize)
                                as *const av_alias64))
                                .u64_0;
                            (*(top_border.offset(24 as ::core::ffi::c_int as isize)
                                as *mut av_alias64))
                                .u64_0 = (*(src_cr
                                .offset((15 as ::core::ffi::c_int * uvlinesize) as isize)
                                as *const av_alias64))
                                .u64_0;
                        }
                    } else if pixel_shift != 0 {
                        (*(top_border.offset(32 as ::core::ffi::c_int as isize)
                            as *mut av_alias64))
                            .u64_0 = (*(src_cb
                            .offset((7 as ::core::ffi::c_int * uvlinesize) as isize)
                            as *const av_alias64))
                            .u64_0;
                        (*((top_border.offset(32 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_char)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64))
                            .u64_0 = (*((src_cb
                            .offset((7 as ::core::ffi::c_int * uvlinesize) as isize)
                            as *mut ::core::ffi::c_char)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *const av_alias64))
                            .u64_0;
                        (*(top_border.offset(48 as ::core::ffi::c_int as isize)
                            as *mut av_alias64))
                            .u64_0 = (*(src_cr
                            .offset((7 as ::core::ffi::c_int * uvlinesize) as isize)
                            as *const av_alias64))
                            .u64_0;
                        (*((top_border.offset(48 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_char)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64))
                            .u64_0 = (*((src_cr
                            .offset((7 as ::core::ffi::c_int * uvlinesize) as isize)
                            as *mut ::core::ffi::c_char)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *const av_alias64))
                            .u64_0;
                    } else {
                        (*(top_border.offset(16 as ::core::ffi::c_int as isize)
                            as *mut av_alias64))
                            .u64_0 = (*(src_cb
                            .offset((7 as ::core::ffi::c_int * uvlinesize) as isize)
                            as *const av_alias64))
                            .u64_0;
                        (*(top_border.offset(24 as ::core::ffi::c_int as isize)
                            as *mut av_alias64))
                            .u64_0 = (*(src_cr
                            .offset((7 as ::core::ffi::c_int * uvlinesize) as isize)
                            as *const av_alias64))
                            .u64_0;
                    }
                }
            }
        } else if (*sl).mb_mbaff != 0 {
            top_idx = 0 as ::core::ffi::c_int;
        } else {
            return
        }
    }
    top_border = &raw mut *(*(&raw mut (*sl).top_borders as *mut *mut [uint8_t; 96])
        .offset(top_idx as isize))
        .offset((*sl).mb_x as isize) as *mut uint8_t;
    (*(top_border as *mut av_alias64)).u64_0 = (*(src_y
        .offset((16 as ::core::ffi::c_int * linesize) as isize) as *const av_alias64))
        .u64_0;
    (*((top_border as *mut ::core::ffi::c_char).offset(8 as ::core::ffi::c_int as isize)
        as *mut av_alias64))
        .u64_0 = (*((src_y.offset((16 as ::core::ffi::c_int * linesize) as isize)
        as *mut ::core::ffi::c_char)
        .offset(8 as ::core::ffi::c_int as isize) as *const av_alias64))
        .u64_0;
    if pixel_shift != 0 {
        (*(top_border.offset(16 as ::core::ffi::c_int as isize) as *mut av_alias64))
            .u64_0 = (*(src_y
            .offset((16 as ::core::ffi::c_int * linesize) as isize)
            .offset(16 as ::core::ffi::c_int as isize) as *const av_alias64))
            .u64_0;
        (*((top_border.offset(16 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char)
            .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64))
            .u64_0 = (*((src_y
            .offset((16 as ::core::ffi::c_int * linesize) as isize)
            .offset(16 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char)
            .offset(8 as ::core::ffi::c_int as isize) as *const av_alias64))
            .u64_0;
    }
    if simple != 0 || 0 as ::core::ffi::c_int == 0
        || (*h).flags & (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int == 0
    {
        if chroma444 != 0 {
            if pixel_shift != 0 {
                (*(top_border.offset(32 as ::core::ffi::c_int as isize)
                    as *mut av_alias64))
                    .u64_0 = (*(src_cb
                    .offset((16 as ::core::ffi::c_int * linesize) as isize)
                    as *const av_alias64))
                    .u64_0;
                (*((top_border.offset(32 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char)
                    .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64))
                    .u64_0 = (*((src_cb
                    .offset((16 as ::core::ffi::c_int * linesize) as isize)
                    as *mut ::core::ffi::c_char)
                    .offset(8 as ::core::ffi::c_int as isize) as *const av_alias64))
                    .u64_0;
                (*(top_border.offset(48 as ::core::ffi::c_int as isize)
                    as *mut av_alias64))
                    .u64_0 = (*(src_cb
                    .offset((16 as ::core::ffi::c_int * linesize) as isize)
                    .offset(16 as ::core::ffi::c_int as isize) as *const av_alias64))
                    .u64_0;
                (*((top_border.offset(48 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char)
                    .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64))
                    .u64_0 = (*((src_cb
                    .offset((16 as ::core::ffi::c_int * linesize) as isize)
                    .offset(16 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char)
                    .offset(8 as ::core::ffi::c_int as isize) as *const av_alias64))
                    .u64_0;
                (*(top_border.offset(64 as ::core::ffi::c_int as isize)
                    as *mut av_alias64))
                    .u64_0 = (*(src_cr
                    .offset((16 as ::core::ffi::c_int * linesize) as isize)
                    as *const av_alias64))
                    .u64_0;
                (*((top_border.offset(64 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char)
                    .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64))
                    .u64_0 = (*((src_cr
                    .offset((16 as ::core::ffi::c_int * linesize) as isize)
                    as *mut ::core::ffi::c_char)
                    .offset(8 as ::core::ffi::c_int as isize) as *const av_alias64))
                    .u64_0;
                (*(top_border.offset(80 as ::core::ffi::c_int as isize)
                    as *mut av_alias64))
                    .u64_0 = (*(src_cr
                    .offset((16 as ::core::ffi::c_int * linesize) as isize)
                    .offset(16 as ::core::ffi::c_int as isize) as *const av_alias64))
                    .u64_0;
                (*((top_border.offset(80 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char)
                    .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64))
                    .u64_0 = (*((src_cr
                    .offset((16 as ::core::ffi::c_int * linesize) as isize)
                    .offset(16 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char)
                    .offset(8 as ::core::ffi::c_int as isize) as *const av_alias64))
                    .u64_0;
            } else {
                (*(top_border.offset(16 as ::core::ffi::c_int as isize)
                    as *mut av_alias64))
                    .u64_0 = (*(src_cb
                    .offset((16 as ::core::ffi::c_int * linesize) as isize)
                    as *const av_alias64))
                    .u64_0;
                (*((top_border.offset(16 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char)
                    .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64))
                    .u64_0 = (*((src_cb
                    .offset((16 as ::core::ffi::c_int * linesize) as isize)
                    as *mut ::core::ffi::c_char)
                    .offset(8 as ::core::ffi::c_int as isize) as *const av_alias64))
                    .u64_0;
                (*(top_border.offset(32 as ::core::ffi::c_int as isize)
                    as *mut av_alias64))
                    .u64_0 = (*(src_cr
                    .offset((16 as ::core::ffi::c_int * linesize) as isize)
                    as *const av_alias64))
                    .u64_0;
                (*((top_border.offset(32 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char)
                    .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64))
                    .u64_0 = (*((src_cr
                    .offset((16 as ::core::ffi::c_int * linesize) as isize)
                    as *mut ::core::ffi::c_char)
                    .offset(8 as ::core::ffi::c_int as isize) as *const av_alias64))
                    .u64_0;
            }
        } else if chroma422 != 0 {
            if pixel_shift != 0 {
                (*(top_border.offset(32 as ::core::ffi::c_int as isize)
                    as *mut av_alias64))
                    .u64_0 = (*(src_cb
                    .offset((16 as ::core::ffi::c_int * uvlinesize) as isize)
                    as *const av_alias64))
                    .u64_0;
                (*((top_border.offset(32 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char)
                    .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64))
                    .u64_0 = (*((src_cb
                    .offset((16 as ::core::ffi::c_int * uvlinesize) as isize)
                    as *mut ::core::ffi::c_char)
                    .offset(8 as ::core::ffi::c_int as isize) as *const av_alias64))
                    .u64_0;
                (*(top_border.offset(48 as ::core::ffi::c_int as isize)
                    as *mut av_alias64))
                    .u64_0 = (*(src_cr
                    .offset((16 as ::core::ffi::c_int * uvlinesize) as isize)
                    as *const av_alias64))
                    .u64_0;
                (*((top_border.offset(48 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char)
                    .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64))
                    .u64_0 = (*((src_cr
                    .offset((16 as ::core::ffi::c_int * uvlinesize) as isize)
                    as *mut ::core::ffi::c_char)
                    .offset(8 as ::core::ffi::c_int as isize) as *const av_alias64))
                    .u64_0;
            } else {
                (*(top_border.offset(16 as ::core::ffi::c_int as isize)
                    as *mut av_alias64))
                    .u64_0 = (*(src_cb
                    .offset((16 as ::core::ffi::c_int * uvlinesize) as isize)
                    as *const av_alias64))
                    .u64_0;
                (*(top_border.offset(24 as ::core::ffi::c_int as isize)
                    as *mut av_alias64))
                    .u64_0 = (*(src_cr
                    .offset((16 as ::core::ffi::c_int * uvlinesize) as isize)
                    as *const av_alias64))
                    .u64_0;
            }
        } else if pixel_shift != 0 {
            (*(top_border.offset(32 as ::core::ffi::c_int as isize) as *mut av_alias64))
                .u64_0 = (*(src_cb
                .offset((8 as ::core::ffi::c_int * uvlinesize) as isize)
                as *const av_alias64))
                .u64_0;
            (*((top_border.offset(32 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char)
                .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64))
                .u64_0 = (*((src_cb
                .offset((8 as ::core::ffi::c_int * uvlinesize) as isize)
                as *mut ::core::ffi::c_char)
                .offset(8 as ::core::ffi::c_int as isize) as *const av_alias64))
                .u64_0;
            (*(top_border.offset(48 as ::core::ffi::c_int as isize) as *mut av_alias64))
                .u64_0 = (*(src_cr
                .offset((8 as ::core::ffi::c_int * uvlinesize) as isize)
                as *const av_alias64))
                .u64_0;
            (*((top_border.offset(48 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char)
                .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64))
                .u64_0 = (*((src_cr
                .offset((8 as ::core::ffi::c_int * uvlinesize) as isize)
                as *mut ::core::ffi::c_char)
                .offset(8 as ::core::ffi::c_int as isize) as *const av_alias64))
                .u64_0;
        } else {
            (*(top_border.offset(16 as ::core::ffi::c_int as isize) as *mut av_alias64))
                .u64_0 = (*(src_cb
                .offset((8 as ::core::ffi::c_int * uvlinesize) as isize)
                as *const av_alias64))
                .u64_0;
            (*(top_border.offset(24 as ::core::ffi::c_int as isize) as *mut av_alias64))
                .u64_0 = (*(src_cr
                .offset((8 as ::core::ffi::c_int * uvlinesize) as isize)
                as *const av_alias64))
                .u64_0;
        }
    }
}
#[inline(always)]
unsafe extern "C" fn get_chroma_qp(
    mut pps: *const PPS,
    mut t: ::core::ffi::c_int,
    mut qscale: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (*pps).chroma_qp_table[t as usize][qscale as usize] as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn get_bits_left(mut gb: *mut GetBitContext) -> ::core::ffi::c_int {
    return (*gb).size_in_bits - get_bits_count(gb);
}
unsafe extern "C" fn implicit_weight_table(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut field: ::core::ffi::c_int,
) {
    let mut ref0: ::core::ffi::c_int = 0;
    let mut ref1: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut cur_poc: ::core::ffi::c_int = 0;
    let mut ref_start: ::core::ffi::c_int = 0;
    let mut ref_count0: ::core::ffi::c_int = 0;
    let mut ref_count1: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 2 as ::core::ffi::c_int {
        (*sl).pwt.luma_weight_flag[i as usize] = 0 as ::core::ffi::c_int;
        (*sl).pwt.chroma_weight_flag[i as usize] = 0 as ::core::ffi::c_int;
        i += 1;
    }
    if field < 0 as ::core::ffi::c_int {
        if (*h).picture_structure == 3 as ::core::ffi::c_int {
            cur_poc = (*(*h).cur_pic_ptr).poc;
        } else {
            cur_poc = (*(*h).cur_pic_ptr)
                .field_poc[((*h).picture_structure - 1 as ::core::ffi::c_int) as usize];
        }
        if (*sl).ref_count[0 as ::core::ffi::c_int as usize] == 1 as ::core::ffi::c_uint
            && (*sl).ref_count[1 as ::core::ffi::c_int as usize]
                == 1 as ::core::ffi::c_uint && (*h).mb_aff_frame == 0
            && ((*sl)
                .ref_list[0 as ::core::ffi::c_int
                    as usize][0 as ::core::ffi::c_int as usize]
                .poc as int64_t
                + (*sl)
                    .ref_list[1 as ::core::ffi::c_int
                        as usize][0 as ::core::ffi::c_int as usize]
                    .poc as int64_t) as ::core::ffi::c_longlong
                == 2 as ::core::ffi::c_longlong * cur_poc as ::core::ffi::c_longlong
        {
            (*sl).pwt.use_weight = 0 as ::core::ffi::c_int;
            (*sl).pwt.use_weight_chroma = 0 as ::core::ffi::c_int;
            return;
        }
        ref_start = 0 as ::core::ffi::c_int;
        ref_count0 = (*sl).ref_count[0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int;
        ref_count1 = (*sl).ref_count[1 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int;
    } else {
        cur_poc = (*(*h).cur_pic_ptr).field_poc[field as usize];
        ref_start = 16 as ::core::ffi::c_int;
        ref_count0 = (16 as ::core::ffi::c_uint)
            .wrapping_add(
                (2 as ::core::ffi::c_uint)
                    .wrapping_mul((*sl).ref_count[0 as ::core::ffi::c_int as usize]),
            ) as ::core::ffi::c_int;
        ref_count1 = (16 as ::core::ffi::c_uint)
            .wrapping_add(
                (2 as ::core::ffi::c_uint)
                    .wrapping_mul((*sl).ref_count[1 as ::core::ffi::c_int as usize]),
            ) as ::core::ffi::c_int;
    }
    (*sl).pwt.use_weight = 2 as ::core::ffi::c_int;
    (*sl).pwt.use_weight_chroma = 2 as ::core::ffi::c_int;
    (*sl).pwt.luma_log2_weight_denom = 5 as ::core::ffi::c_int;
    (*sl).pwt.chroma_log2_weight_denom = 5 as ::core::ffi::c_int;
    ref0 = ref_start;
    while ref0 < ref_count0 {
        let mut poc0: int64_t = (*sl)
            .ref_list[0 as ::core::ffi::c_int as usize][ref0 as usize]
            .poc as int64_t;
        ref1 = ref_start;
        while ref1 < ref_count1 {
            let mut w: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
            if (*(*sl).ref_list[0 as ::core::ffi::c_int as usize][ref0 as usize].parent)
                .long_ref == 0
                && (*(*sl)
                    .ref_list[1 as ::core::ffi::c_int as usize][ref1 as usize]
                    .parent)
                    .long_ref == 0
            {
                let mut poc1: ::core::ffi::c_int = (*sl)
                    .ref_list[1 as ::core::ffi::c_int as usize][ref1 as usize]
                    .poc;
                let mut td: ::core::ffi::c_int = av_clip_int8_c(
                    (poc1 as int64_t - poc0) as ::core::ffi::c_int,
                ) as ::core::ffi::c_int;
                if td != 0 {
                    let mut tb: ::core::ffi::c_int = av_clip_int8_c(
                        (cur_poc as int64_t - poc0) as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int;
                    let mut tx: ::core::ffi::c_int = (16384 as ::core::ffi::c_int
                        + ((if td >= 0 as ::core::ffi::c_int { td } else { -td })
                            >> 1 as ::core::ffi::c_int)) / td;
                    let mut dist_scale_factor: ::core::ffi::c_int = tb * tx
                        + 32 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int;
                    if dist_scale_factor >= -(64 as ::core::ffi::c_int)
                        && dist_scale_factor <= 128 as ::core::ffi::c_int
                    {
                        w = 64 as ::core::ffi::c_int - dist_scale_factor;
                    }
                }
            }
            if field < 0 as ::core::ffi::c_int {
                (*sl)
                    .pwt
                    .implicit_weight[ref0
                    as usize][ref1 as usize][1 as ::core::ffi::c_int as usize] = w;
                (*sl)
                    .pwt
                    .implicit_weight[ref0
                    as usize][ref1 as usize][0 as ::core::ffi::c_int as usize] = (*sl)
                    .pwt
                    .implicit_weight[ref0
                    as usize][ref1 as usize][1 as ::core::ffi::c_int as usize];
            } else {
                (*sl)
                    .pwt
                    .implicit_weight[ref0 as usize][ref1 as usize][field as usize] = w;
            }
            ref1 += 1;
        }
        ref0 += 1;
    }
}
unsafe extern "C" fn init_scan_tables(mut h: *mut H264Context) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        (*h).zigzag_scan[i as usize] = (ff_zigzag_scan[i as usize] as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int
            | (ff_zigzag_scan[i as usize] as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as uint8_t;
        (*h).field_scan[i as usize] = (field_scan[i as usize] as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int
            | (field_scan[i as usize] as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int) as uint8_t;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 64 as ::core::ffi::c_int {
        (*h).zigzag_scan8x8[i as usize] = (ff_zigzag_direct[i as usize]
            as ::core::ffi::c_int >> 3 as ::core::ffi::c_int
            | (ff_zigzag_direct[i as usize] as ::core::ffi::c_int
                & 7 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint8_t;
        (*h).zigzag_scan8x8_cavlc[i as usize] = (zigzag_scan8x8_cavlc[i as usize]
            as ::core::ffi::c_int >> 3 as ::core::ffi::c_int
            | (zigzag_scan8x8_cavlc[i as usize] as ::core::ffi::c_int
                & 7 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint8_t;
        (*h).field_scan8x8[i as usize] = (field_scan8x8[i as usize] as ::core::ffi::c_int
            >> 3 as ::core::ffi::c_int
            | (field_scan8x8[i as usize] as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                << 3 as ::core::ffi::c_int) as uint8_t;
        (*h).field_scan8x8_cavlc[i as usize] = (field_scan8x8_cavlc[i as usize]
            as ::core::ffi::c_int >> 3 as ::core::ffi::c_int
            | (field_scan8x8_cavlc[i as usize] as ::core::ffi::c_int
                & 7 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint8_t;
        i += 1;
    }
    if (*(*h).ps.sps).transform_bypass != 0 {
        memcpy(
            &raw mut (*h).zigzag_scan_q0 as *mut uint8_t as *mut ::core::ffi::c_void,
            &raw const ff_zigzag_scan as *const uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
        );
        memcpy(
            &raw mut (*h).zigzag_scan8x8_q0 as *mut uint8_t as *mut ::core::ffi::c_void,
            &raw const ff_zigzag_direct as *const uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
        );
        memcpy(
            &raw mut (*h).zigzag_scan8x8_cavlc_q0 as *mut uint8_t
                as *mut ::core::ffi::c_void,
            &raw const zigzag_scan8x8_cavlc as *const uint8_t
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
        );
        memcpy(
            &raw mut (*h).field_scan_q0 as *mut uint8_t as *mut ::core::ffi::c_void,
            &raw const field_scan as *const uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
        );
        memcpy(
            &raw mut (*h).field_scan8x8_q0 as *mut uint8_t as *mut ::core::ffi::c_void,
            &raw const field_scan8x8 as *const uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
        );
        memcpy(
            &raw mut (*h).field_scan8x8_cavlc_q0 as *mut uint8_t
                as *mut ::core::ffi::c_void,
            &raw const field_scan8x8_cavlc as *const uint8_t
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
        );
    } else {
        memcpy(
            &raw mut (*h).zigzag_scan_q0 as *mut uint8_t as *mut ::core::ffi::c_void,
            &raw mut (*h).zigzag_scan as *mut uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
        );
        memcpy(
            &raw mut (*h).zigzag_scan8x8_q0 as *mut uint8_t as *mut ::core::ffi::c_void,
            &raw mut (*h).zigzag_scan8x8 as *mut uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
        );
        memcpy(
            &raw mut (*h).zigzag_scan8x8_cavlc_q0 as *mut uint8_t
                as *mut ::core::ffi::c_void,
            &raw mut (*h).zigzag_scan8x8_cavlc as *mut uint8_t
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
        );
        memcpy(
            &raw mut (*h).field_scan_q0 as *mut uint8_t as *mut ::core::ffi::c_void,
            &raw mut (*h).field_scan as *mut uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
        );
        memcpy(
            &raw mut (*h).field_scan8x8_q0 as *mut uint8_t as *mut ::core::ffi::c_void,
            &raw mut (*h).field_scan8x8 as *mut uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
        );
        memcpy(
            &raw mut (*h).field_scan8x8_cavlc_q0 as *mut uint8_t
                as *mut ::core::ffi::c_void,
            &raw mut (*h).field_scan8x8_cavlc as *mut uint8_t
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
        );
    };
}
unsafe extern "C" fn get_pixel_format(
    mut h: *mut H264Context,
    mut force_callback: ::core::ffi::c_int,
) -> AVPixelFormat {
    let mut pix_fmts: [AVPixelFormat; 2] = [AV_PIX_FMT_YUV420P; 2];
    let mut fmt: *mut AVPixelFormat = &raw mut pix_fmts as *mut AVPixelFormat;
    match (*(*h).ps.sps).bit_depth_luma {
        9 => {
            if (*(*h).ps.sps).chroma_format_idc == 3 as ::core::ffi::c_int {
                if (*(*h).avctx).colorspace as ::core::ffi::c_uint
                    == AVCOL_SPC_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    let fresh3 = fmt;
                    fmt = fmt.offset(1);
                    *fresh3 = AV_PIX_FMT_GBRP9LE;
                } else {
                    let fresh4 = fmt;
                    fmt = fmt.offset(1);
                    *fresh4 = AV_PIX_FMT_YUV444P9LE;
                }
            } else if (*(*h).ps.sps).chroma_format_idc == 2 as ::core::ffi::c_int {
                let fresh5 = fmt;
                fmt = fmt.offset(1);
                *fresh5 = AV_PIX_FMT_YUV422P9LE;
            } else {
                let fresh6 = fmt;
                fmt = fmt.offset(1);
                *fresh6 = AV_PIX_FMT_YUV420P9LE;
            }
        }
        10 => {
            if (*(*h).ps.sps).chroma_format_idc == 3 as ::core::ffi::c_int {
                if (*(*h).avctx).colorspace as ::core::ffi::c_uint
                    == AVCOL_SPC_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    let fresh7 = fmt;
                    fmt = fmt.offset(1);
                    *fresh7 = AV_PIX_FMT_GBRP10LE;
                } else {
                    let fresh8 = fmt;
                    fmt = fmt.offset(1);
                    *fresh8 = AV_PIX_FMT_YUV444P10LE;
                }
            } else if (*(*h).ps.sps).chroma_format_idc == 2 as ::core::ffi::c_int {
                let fresh9 = fmt;
                fmt = fmt.offset(1);
                *fresh9 = AV_PIX_FMT_YUV422P10LE;
            } else {
                let fresh10 = fmt;
                fmt = fmt.offset(1);
                *fresh10 = AV_PIX_FMT_YUV420P10LE;
            }
        }
        12 => {
            if (*(*h).ps.sps).chroma_format_idc == 3 as ::core::ffi::c_int {
                if (*(*h).avctx).colorspace as ::core::ffi::c_uint
                    == AVCOL_SPC_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    let fresh11 = fmt;
                    fmt = fmt.offset(1);
                    *fresh11 = AV_PIX_FMT_GBRP12LE;
                } else {
                    let fresh12 = fmt;
                    fmt = fmt.offset(1);
                    *fresh12 = AV_PIX_FMT_YUV444P12LE;
                }
            } else if (*(*h).ps.sps).chroma_format_idc == 2 as ::core::ffi::c_int {
                let fresh13 = fmt;
                fmt = fmt.offset(1);
                *fresh13 = AV_PIX_FMT_YUV422P12LE;
            } else {
                let fresh14 = fmt;
                fmt = fmt.offset(1);
                *fresh14 = AV_PIX_FMT_YUV420P12LE;
            }
        }
        14 => {
            if (*(*h).ps.sps).chroma_format_idc == 3 as ::core::ffi::c_int {
                if (*(*h).avctx).colorspace as ::core::ffi::c_uint
                    == AVCOL_SPC_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    let fresh15 = fmt;
                    fmt = fmt.offset(1);
                    *fresh15 = AV_PIX_FMT_GBRP14LE;
                } else {
                    let fresh16 = fmt;
                    fmt = fmt.offset(1);
                    *fresh16 = AV_PIX_FMT_YUV444P14LE;
                }
            } else if (*(*h).ps.sps).chroma_format_idc == 2 as ::core::ffi::c_int {
                let fresh17 = fmt;
                fmt = fmt.offset(1);
                *fresh17 = AV_PIX_FMT_YUV422P14LE;
            } else {
                let fresh18 = fmt;
                fmt = fmt.offset(1);
                *fresh18 = AV_PIX_FMT_YUV420P14LE;
            }
        }
        8 => {
            if (*(*h).ps.sps).chroma_format_idc == 3 as ::core::ffi::c_int {
                if (*(*h).avctx).colorspace as ::core::ffi::c_uint
                    == AVCOL_SPC_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    let fresh19 = fmt;
                    fmt = fmt.offset(1);
                    *fresh19 = AV_PIX_FMT_GBRP;
                } else if (*(*h).avctx).color_range as ::core::ffi::c_uint
                    == AVCOL_RANGE_JPEG as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    let fresh20 = fmt;
                    fmt = fmt.offset(1);
                    *fresh20 = AV_PIX_FMT_YUVJ444P;
                } else {
                    let fresh21 = fmt;
                    fmt = fmt.offset(1);
                    *fresh21 = AV_PIX_FMT_YUV444P;
                }
            } else if (*(*h).ps.sps).chroma_format_idc == 2 as ::core::ffi::c_int {
                if (*(*h).avctx).color_range as ::core::ffi::c_uint
                    == AVCOL_RANGE_JPEG as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    let fresh22 = fmt;
                    fmt = fmt.offset(1);
                    *fresh22 = AV_PIX_FMT_YUVJ422P;
                } else {
                    let fresh23 = fmt;
                    fmt = fmt.offset(1);
                    *fresh23 = AV_PIX_FMT_YUV422P;
                }
            } else if (*(*h).avctx).color_range as ::core::ffi::c_uint
                == AVCOL_RANGE_JPEG as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                let fresh24 = fmt;
                fmt = fmt.offset(1);
                *fresh24 = AV_PIX_FMT_YUVJ420P;
            } else {
                let fresh25 = fmt;
                fmt = fmt.offset(1);
                *fresh25 = AV_PIX_FMT_YUV420P;
            }
        }
        _ => {
            av_log(
                (*h).avctx as *mut ::core::ffi::c_void,
                16 as ::core::ffi::c_int,
                b"Unsupported bit depth %d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*(*h).ps.sps).bit_depth_luma,
            );
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int) as AVPixelFormat;
        }
    }
    *fmt = AV_PIX_FMT_NONE;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while pix_fmts[i as usize] as ::core::ffi::c_int
        != AV_PIX_FMT_NONE as ::core::ffi::c_int
    {
        if pix_fmts[i as usize] as ::core::ffi::c_int
            == (*(*h).avctx).pix_fmt as ::core::ffi::c_int && force_callback == 0
        {
            return pix_fmts[i as usize];
        }
        i += 1;
    }
    return ff_get_format((*h).avctx, &raw mut pix_fmts as *mut AVPixelFormat)
        as AVPixelFormat;
}
unsafe extern "C" fn init_dimensions(mut h: *mut H264Context) {
    let mut sps: *const SPS = (*h).ps.sps;
    let mut cr: ::core::ffi::c_int = (*sps).crop_right as ::core::ffi::c_int;
    let mut cl: ::core::ffi::c_int = (*sps).crop_left as ::core::ffi::c_int;
    let mut ct: ::core::ffi::c_int = (*sps).crop_top as ::core::ffi::c_int;
    let mut cb: ::core::ffi::c_int = (*sps).crop_bottom as ::core::ffi::c_int;
    let mut width: ::core::ffi::c_int = (*h).width - (cr + cl);
    let mut height: ::core::ffi::c_int = (*h).height - (ct + cb);
    if !((*sps).crop_right.wrapping_add((*sps).crop_left)
        < (*h).width as ::core::ffi::c_uint)
    {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"sps->crop_right + sps->crop_left < (unsigned)h->width\0" as *const u8
                as *const ::core::ffi::c_char,
            b"src/libavcodec/h264_slice.c\0" as *const u8 as *const ::core::ffi::c_char,
            934 as ::core::ffi::c_int,
        );
        abort();
    }
    if !((*sps).crop_top.wrapping_add((*sps).crop_bottom)
        < (*h).height as ::core::ffi::c_uint)
    {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"sps->crop_top + sps->crop_bottom < (unsigned)h->height\0" as *const u8
                as *const ::core::ffi::c_char,
            b"src/libavcodec/h264_slice.c\0" as *const u8 as *const ::core::ffi::c_char,
            935 as ::core::ffi::c_int,
        );
        abort();
    }
    if (*h).width_from_caller > 0 as ::core::ffi::c_int
        && (*h).height_from_caller > 0 as ::core::ffi::c_int && (*sps).crop_top == 0
        && (*sps).crop_left == 0
        && (*h).width_from_caller + 16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
            & !(16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            == width + 16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                & !(16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        && (*h).height_from_caller + 16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
            & !(16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            == height + 16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                & !(16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        && (*h).width_from_caller <= width && (*h).height_from_caller <= height
    {
        width = (*h).width_from_caller;
        height = (*h).height_from_caller;
        cl = 0 as ::core::ffi::c_int;
        ct = 0 as ::core::ffi::c_int;
        cr = (*h).width - width;
        cb = (*h).height - height;
    } else {
        (*h).width_from_caller = 0 as ::core::ffi::c_int;
        (*h).height_from_caller = 0 as ::core::ffi::c_int;
    }
    (*(*h).avctx).coded_width = (*h).width;
    (*(*h).avctx).coded_height = (*h).height;
    (*(*h).avctx).width = width;
    (*(*h).avctx).height = height;
    (*h).crop_right = cr;
    (*h).crop_left = cl;
    (*h).crop_top = ct;
    (*h).crop_bottom = cb;
}
unsafe extern "C" fn h264_slice_header_init(
    mut h: *mut H264Context,
) -> ::core::ffi::c_int {
    let mut sps: *const SPS = (*h).ps.sps;
    let mut i: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    if sps.is_null() {
        ret = -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    } else {
        ff_set_sar((*h).avctx, (*sps).vui.sar);
        av_pix_fmt_get_chroma_sub_sample(
            (*(*h).avctx).pix_fmt,
            &raw mut (*h).chroma_x_shift,
            &raw mut (*h).chroma_y_shift,
        );
        if (*sps).timing_info_present_flag != 0 {
            let mut den: int64_t = (*sps).time_scale as int64_t;
            if ((*h).x264_build as ::core::ffi::c_uint) < 44 as ::core::ffi::c_uint {
                den *= 2 as int64_t;
            }
            av_reduce(
                &raw mut (*(*h).avctx).framerate.den,
                &raw mut (*(*h).avctx).framerate.num,
                (*sps).num_units_in_tick.wrapping_mul(2 as uint32_t) as int64_t,
                den,
                ((1 as ::core::ffi::c_int) << 30 as ::core::ffi::c_int) as int64_t,
            );
        }
        ff_h264_free_tables(h);
        (*h).first_field = 0 as ::core::ffi::c_int;
        (*h).prev_interlaced_frame = 1 as ::core::ffi::c_int;
        init_scan_tables(h);
        ret = ff_h264_alloc_tables(h);
        if ret < 0 as ::core::ffi::c_int {
            av_log(
                (*h).avctx as *mut ::core::ffi::c_void,
                16 as ::core::ffi::c_int,
                b"Could not allocate memory\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        } else if (*sps).bit_depth_luma < 8 as ::core::ffi::c_int
            || (*sps).bit_depth_luma > 14 as ::core::ffi::c_int
            || (*sps).bit_depth_luma == 11 as ::core::ffi::c_int
            || (*sps).bit_depth_luma == 13 as ::core::ffi::c_int
        {
            av_log(
                (*h).avctx as *mut ::core::ffi::c_void,
                16 as ::core::ffi::c_int,
                b"Unsupported bit depth %d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*sps).bit_depth_luma,
            );
            ret = -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        } else {
            (*(*h).avctx).bits_per_raw_sample = (*sps).bit_depth_luma;
            (*h).cur_bit_depth_luma = (*(*h).avctx).bits_per_raw_sample;
            (*h).cur_chroma_format_idc = (*sps).chroma_format_idc;
            (*h).pixel_shift = ((*sps).bit_depth_luma > 8 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            (*h).chroma_format_idc = (*sps).chroma_format_idc;
            (*h).bit_depth_luma = (*sps).bit_depth_luma;
            ff_h264dsp_init(
                &raw mut (*h).h264dsp,
                (*sps).bit_depth_luma,
                (*sps).chroma_format_idc,
            );
            ff_h264chroma_init(&raw mut (*h).h264chroma, (*sps).bit_depth_chroma);
            ff_h264qpel_init(&raw mut (*h).h264qpel, (*sps).bit_depth_luma);
            ff_h264_pred_init(
                &raw mut (*h).hpc,
                AV_CODEC_ID_H264 as ::core::ffi::c_int,
                (*sps).bit_depth_luma,
                (*sps).chroma_format_idc,
            );
            ff_videodsp_init(&raw mut (*h).vdsp, (*sps).bit_depth_luma);
            if 1 as ::core::ffi::c_int == 0
                || (*(*h).avctx).active_thread_type & 2 as ::core::ffi::c_int == 0
            {
                ff_h264_slice_context_init(
                    h,
                    (*h).slice_ctx.offset(0 as ::core::ffi::c_int as isize)
                        as *mut H264SliceContext,
                );
            } else {
                i = 0 as ::core::ffi::c_int;
                while i < (*h).nb_slice_ctx {
                    let mut sl: *mut H264SliceContext = (*h).slice_ctx.offset(i as isize)
                        as *mut H264SliceContext;
                    (*sl).h264 = h;
                    (*sl).intra4x4_pred_mode = (*h)
                        .intra4x4_pred_mode
                        .offset(
                            (i * 8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                * (*h).mb_stride) as isize,
                        );
                    (*sl).mvd_table[0 as ::core::ffi::c_int as usize] = (*h)
                        .mvd_table[0 as ::core::ffi::c_int as usize]
                        .offset(
                            (i * 8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                * (*h).mb_stride) as isize,
                        );
                    (*sl).mvd_table[1 as ::core::ffi::c_int as usize] = (*h)
                        .mvd_table[1 as ::core::ffi::c_int as usize]
                        .offset(
                            (i * 8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                * (*h).mb_stride) as isize,
                        );
                    ff_h264_slice_context_init(h, sl);
                    i += 1;
                }
            }
            (*h).context_initialized = 1 as ::core::ffi::c_int;
            return 0 as ::core::ffi::c_int;
        }
    }
    ff_h264_free_tables(h);
    (*h).context_initialized = 0 as ::core::ffi::c_int;
    return ret;
}
unsafe extern "C" fn non_j_pixfmt(mut a: AVPixelFormat) -> AVPixelFormat {
    match a as ::core::ffi::c_int {
        12 => return AV_PIX_FMT_YUV420P,
        13 => return AV_PIX_FMT_YUV422P,
        14 => return AV_PIX_FMT_YUV444P,
        _ => return a,
    };
}
unsafe extern "C" fn h264_init_ps(
    mut h: *mut H264Context,
    mut sl: *const H264SliceContext,
    mut first_slice: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut sps: *const SPS = ::core::ptr::null::<SPS>();
    let mut needs_reinit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut must_reinit: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    if first_slice != 0 {
        av_refstruct_replace(
            &raw mut (*h).ps.pps as *mut ::core::ffi::c_void,
            (*h).ps.pps_list[(*sl).pps_id as usize] as *const ::core::ffi::c_void,
        );
    }
    if (*h).ps.sps != (*(*h).ps.pps).sps {
        (*h).ps.sps = (*(*h).ps.pps).sps;
        if (*h).mb_width != (*(*h).ps.sps).mb_width
            || (*h).mb_height != (*(*h).ps.sps).mb_height
            || (*h).cur_bit_depth_luma != (*(*h).ps.sps).bit_depth_luma
            || (*h).cur_chroma_format_idc != (*(*h).ps.sps).chroma_format_idc
        {
            needs_reinit = 1 as ::core::ffi::c_int;
        }
        if (*h).bit_depth_luma != (*(*h).ps.sps).bit_depth_luma
            || (*h).chroma_format_idc != (*(*h).ps.sps).chroma_format_idc
        {
            needs_reinit = 1 as ::core::ffi::c_int;
        }
    }
    sps = (*h).ps.sps;
    must_reinit = ((*h).context_initialized != 0
        && (16 as ::core::ffi::c_int * (*sps).mb_width != (*(*h).avctx).coded_width
            || 16 as ::core::ffi::c_int * (*sps).mb_height != (*(*h).avctx).coded_height
            || (*h).cur_bit_depth_luma != (*sps).bit_depth_luma
            || (*h).cur_chroma_format_idc != (*sps).chroma_format_idc
            || (*h).mb_width != (*sps).mb_width || (*h).mb_height != (*sps).mb_height))
        as ::core::ffi::c_int;
    if (*(*h).avctx).pix_fmt as ::core::ffi::c_int
        == AV_PIX_FMT_NONE as ::core::ffi::c_int
        || non_j_pixfmt((*(*h).avctx).pix_fmt) as ::core::ffi::c_int
            != non_j_pixfmt(get_pixel_format(h, 0 as ::core::ffi::c_int))
                as ::core::ffi::c_int
    {
        must_reinit = 1 as ::core::ffi::c_int;
    }
    if first_slice != 0
        && av_cmp_q((*sps).vui.sar, (*(*h).avctx).sample_aspect_ratio) != 0
    {
        must_reinit = 1 as ::core::ffi::c_int;
    }
    if (*h).setup_finished == 0 {
        (*(*h).avctx).profile = ff_h264_get_profile(sps);
        (*(*h).avctx).level = (*sps).level_idc;
        (*(*h).avctx).refs = (*sps).ref_frame_count;
        (*h).mb_width = (*sps).mb_width;
        (*h).mb_height = (*sps).mb_height;
        (*h).mb_num = (*h).mb_width * (*h).mb_height;
        (*h).mb_stride = (*h).mb_width + 1 as ::core::ffi::c_int;
        (*h).b_stride = (*h).mb_width * 4 as ::core::ffi::c_int;
        (*h).chroma_y_shift = ((*sps).chroma_format_idc <= 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        (*h).width = 16 as ::core::ffi::c_int * (*h).mb_width;
        (*h).height = 16 as ::core::ffi::c_int * (*h).mb_height;
        init_dimensions(h);
        if (*sps).vui.video_signal_type_present_flag != 0 {
            (*(*h).avctx).color_range = (if (*sps).vui.video_full_range_flag
                > 0 as ::core::ffi::c_int
            {
                AVCOL_RANGE_JPEG as ::core::ffi::c_int
            } else {
                AVCOL_RANGE_MPEG as ::core::ffi::c_int
            }) as AVColorRange;
            if (*sps).vui.colour_description_present_flag != 0 {
                if (*(*h).avctx).colorspace as ::core::ffi::c_uint
                    != (*sps).vui.matrix_coeffs as ::core::ffi::c_uint
                {
                    needs_reinit = 1 as ::core::ffi::c_int;
                }
                (*(*h).avctx).color_primaries = (*sps).vui.colour_primaries;
                (*(*h).avctx).color_trc = (*sps).vui.transfer_characteristics;
                (*(*h).avctx).colorspace = (*sps).vui.matrix_coeffs;
            }
        }
        if (*h).sei.common.alternative_transfer.present != 0
            && !av_color_transfer_name(
                    (*h)
                        .sei
                        .common
                        .alternative_transfer
                        .preferred_transfer_characteristics
                        as AVColorTransferCharacteristic,
                )
                .is_null()
            && (*h).sei.common.alternative_transfer.preferred_transfer_characteristics
                != AVCOL_TRC_UNSPECIFIED as ::core::ffi::c_int
        {
            (*(*h).avctx).color_trc = (*h)
                .sei
                .common
                .alternative_transfer
                .preferred_transfer_characteristics as AVColorTransferCharacteristic;
        }
    }
    (*(*h).avctx).chroma_sample_location = (*sps).vui.chroma_location;
    if (*h).context_initialized == 0 || must_reinit != 0 || needs_reinit != 0 {
        let mut flush_changes: ::core::ffi::c_int = (*h).context_initialized;
        (*h).context_initialized = 0 as ::core::ffi::c_int;
        if sl != (*h).slice_ctx as *const H264SliceContext {
            av_log(
                (*h).avctx as *mut ::core::ffi::c_void,
                16 as ::core::ffi::c_int,
                b"changing width %d -> %d / height %d -> %d on slice %d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*h).width,
                (*(*h).avctx).coded_width,
                (*h).height,
                (*(*h).avctx).coded_height,
                (*h).current_slice + 1 as ::core::ffi::c_int,
            );
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        if flush_changes != 0 {
            ff_h264_flush_change(h);
        }
        ret = get_pixel_format(
            h,
            (must_reinit != 0 || needs_reinit != 0) as ::core::ffi::c_int,
        ) as ::core::ffi::c_int;
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        (*(*h).avctx).pix_fmt = ret as AVPixelFormat;
        av_log(
            (*h).avctx as *mut ::core::ffi::c_void,
            40 as ::core::ffi::c_int,
            b"Reinit context to %dx%d, pix_fmt: %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            (*h).width,
            (*h).height,
            av_get_pix_fmt_name((*(*h).avctx).pix_fmt),
        );
        ret = h264_slice_header_init(h);
        if ret < 0 as ::core::ffi::c_int {
            av_log(
                (*h).avctx as *mut ::core::ffi::c_void,
                16 as ::core::ffi::c_int,
                b"h264_slice_header_init() failed\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return ret;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn h264_export_frame_props(
    mut h: *mut H264Context,
) -> ::core::ffi::c_int {
    let mut sps: *const SPS = (*h).ps.sps;
    let mut cur: *mut H264Picture = (*h).cur_pic_ptr;
    let mut out: *mut AVFrame = (*cur).f;
    let mut interlaced_frame: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut top_field_first: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ret: ::core::ffi::c_int = 0;
    (*out).flags &= !((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int);
    (*out).repeat_pict = 0 as ::core::ffi::c_int;
    if (*h).sei.picture_timing.present != 0 {
        let mut ret_0: ::core::ffi::c_int = ff_h264_sei_process_picture_timing(
            &raw mut (*h).sei.picture_timing,
            sps,
            (*h).avctx as *mut ::core::ffi::c_void,
        );
        if ret_0 < 0 as ::core::ffi::c_int {
            av_log(
                (*h).avctx as *mut ::core::ffi::c_void,
                16 as ::core::ffi::c_int,
                b"Error processing a picture timing SEI\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            if (*(*h).avctx).err_recognition
                & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
            {
                return ret_0;
            }
            (*h).sei.picture_timing.present = 0 as ::core::ffi::c_int;
        }
    }
    if (*sps).pic_struct_present_flag != 0 && (*h).sei.picture_timing.present != 0 {
        let mut pt: *const H264SEIPictureTiming = &raw mut (*h).sei.picture_timing;
        match (*pt).pic_struct as ::core::ffi::c_uint {
            1 | 2 => {
                interlaced_frame = 1 as ::core::ffi::c_int;
            }
            3 | 4 => {
                if (*h).mb_aff_frame != 0
                    || (*h).picture_structure != 3 as ::core::ffi::c_int
                {
                    interlaced_frame = 1 as ::core::ffi::c_int;
                } else {
                    interlaced_frame = ((*h).prev_interlaced_frame != 0)
                        as ::core::ffi::c_int;
                }
            }
            5 | 6 => {
                (*out).repeat_pict = 1 as ::core::ffi::c_int;
            }
            7 => {
                (*out).repeat_pict = 2 as ::core::ffi::c_int;
            }
            8 => {
                (*out).repeat_pict = 4 as ::core::ffi::c_int;
            }
            0 | _ => {}
        }
        if (*pt).ct_type & 3 as ::core::ffi::c_int != 0
            && (*pt).pic_struct as ::core::ffi::c_uint
                <= H264_SEI_PIC_STRUCT_BOTTOM_TOP as ::core::ffi::c_int
                    as ::core::ffi::c_uint
        {
            interlaced_frame = ((*pt).ct_type
                & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        }
    } else {
        interlaced_frame = ((*h).mb_aff_frame != 0
            || (*h).picture_structure != 3 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
    (*h).prev_interlaced_frame = interlaced_frame;
    if (*cur).field_poc[0 as ::core::ffi::c_int as usize]
        != (*cur).field_poc[1 as ::core::ffi::c_int as usize]
    {
        top_field_first = ((*cur).field_poc[0 as ::core::ffi::c_int as usize]
            < (*cur).field_poc[1 as ::core::ffi::c_int as usize]) as ::core::ffi::c_int;
    } else if (*sps).pic_struct_present_flag != 0 && (*h).sei.picture_timing.present != 0
    {
        if (*h).sei.picture_timing.pic_struct as ::core::ffi::c_uint
            == H264_SEI_PIC_STRUCT_TOP_BOTTOM as ::core::ffi::c_int
                as ::core::ffi::c_uint
            || (*h).sei.picture_timing.pic_struct as ::core::ffi::c_uint
                == H264_SEI_PIC_STRUCT_TOP_BOTTOM_TOP as ::core::ffi::c_int
                    as ::core::ffi::c_uint
        {
            top_field_first = 1 as ::core::ffi::c_int;
        }
    } else if interlaced_frame != 0 {
        top_field_first = 1 as ::core::ffi::c_int;
    }
    (*out).flags
        |= ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) * interlaced_frame
            | ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) * top_field_first;
    ret = ff_h2645_sei_to_frame(
        out,
        &raw mut (*h).sei.common,
        AV_CODEC_ID_H264,
        (*h).avctx,
        &raw const (*sps).vui,
        (*sps).bit_depth_luma as ::core::ffi::c_uint,
        (*sps).bit_depth_chroma as ::core::ffi::c_uint,
        ((*cur).poc as ::core::ffi::c_uint)
            .wrapping_add(
                ((*h).poc_offset << 5 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            ) as ::core::ffi::c_int,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    if (*h).sei.picture_timing.timecode_cnt > 0 as ::core::ffi::c_int {
        let mut tc_sd: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut tcbuf: [::core::ffi::c_char; 23] = [0; 23];
        let mut tcside: *mut AVFrameSideData = ::core::ptr::null_mut::<
            AVFrameSideData,
        >();
        ret = ff_frame_new_side_data(
            (*h).avctx,
            out,
            AV_FRAME_DATA_S12M_TIMECODE,
            (::core::mem::size_of::<uint32_t>() as size_t).wrapping_mul(4 as size_t),
            &raw mut tcside,
        );
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        if !tcside.is_null() {
            tc_sd = (*tcside).data as *mut uint32_t;
            *tc_sd.offset(0 as ::core::ffi::c_int as isize) = (*h)
                .sei
                .picture_timing
                .timecode_cnt as uint32_t;
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while (i as uint32_t) < *tc_sd.offset(0 as ::core::ffi::c_int as isize) {
                let mut drop_0: ::core::ffi::c_int = (*h)
                    .sei
                    .picture_timing
                    .timecode[i as usize]
                    .dropframe;
                let mut hh: ::core::ffi::c_int = (*h)
                    .sei
                    .picture_timing
                    .timecode[i as usize]
                    .hours;
                let mut mm: ::core::ffi::c_int = (*h)
                    .sei
                    .picture_timing
                    .timecode[i as usize]
                    .minutes;
                let mut ss: ::core::ffi::c_int = (*h)
                    .sei
                    .picture_timing
                    .timecode[i as usize]
                    .seconds;
                let mut ff: ::core::ffi::c_int = (*h)
                    .sei
                    .picture_timing
                    .timecode[i as usize]
                    .frame;
                *tc_sd.offset((i + 1 as ::core::ffi::c_int) as isize) = av_timecode_get_smpte(
                    (*(*h).avctx).framerate,
                    drop_0,
                    hh,
                    mm,
                    ss,
                    ff,
                );
                av_timecode_make_smpte_tc_string2(
                    &raw mut tcbuf as *mut ::core::ffi::c_char,
                    (*(*h).avctx).framerate,
                    *tc_sd.offset((i + 1 as ::core::ffi::c_int) as isize),
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                );
                av_dict_set(
                    &raw mut (*out).metadata,
                    b"timecode\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut tcbuf as *mut ::core::ffi::c_char,
                    0 as ::core::ffi::c_int,
                );
                i += 1;
            }
        }
        (*h).sei.picture_timing.timecode_cnt = 0 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn h264_select_output_frame(
    mut h: *mut H264Context,
) -> ::core::ffi::c_int {
    let mut sps: *const SPS = (*h).ps.sps;
    let mut out: *mut H264Picture = (*h).cur_pic_ptr;
    let mut cur: *mut H264Picture = (*h).cur_pic_ptr;
    let mut i: ::core::ffi::c_int = 0;
    let mut pics: ::core::ffi::c_int = 0;
    let mut out_of_order: ::core::ffi::c_int = 0;
    let mut out_idx: ::core::ffi::c_int = 0;
    (*cur).mmco_reset = (*h).mmco_reset;
    (*h).mmco_reset = 0 as ::core::ffi::c_int;
    if (*sps).bitstream_restriction_flag != 0
        || (*(*h).avctx).strict_std_compliance >= 1 as ::core::ffi::c_int
    {
        (*(*h).avctx).has_b_frames = if (*(*h).avctx).has_b_frames
            > (*sps).num_reorder_frames
        {
            (*(*h).avctx).has_b_frames
        } else {
            (*sps).num_reorder_frames
        };
    }
    i = 0 as ::core::ffi::c_int;
    loop {
        if i == H264_MAX_DPB_FRAMES as ::core::ffi::c_int
            || (*cur).poc < (*h).last_pocs[i as usize]
        {
            if i != 0 {
                (*h).last_pocs[(i - 1 as ::core::ffi::c_int) as usize] = (*cur).poc;
            }
            break;
        } else {
            if i != 0 {
                (*h).last_pocs[(i - 1 as ::core::ffi::c_int) as usize] = (*h)
                    .last_pocs[i as usize];
            }
            i += 1;
        }
    }
    out_of_order = H264_MAX_DPB_FRAMES as ::core::ffi::c_int - i;
    if (*(*cur).f).pict_type as ::core::ffi::c_uint
        == AV_PICTURE_TYPE_B as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*h)
            .last_pocs[(H264_MAX_DPB_FRAMES as ::core::ffi::c_int
            - 2 as ::core::ffi::c_int) as usize]
            > -(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int
            && (*h)
                .last_pocs[(H264_MAX_DPB_FRAMES as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] as int64_t
                - (*h)
                    .last_pocs[(H264_MAX_DPB_FRAMES as ::core::ffi::c_int
                    - 2 as ::core::ffi::c_int) as usize] as int64_t > 2 as int64_t
    {
        out_of_order = if out_of_order > 1 as ::core::ffi::c_int {
            out_of_order
        } else {
            1 as ::core::ffi::c_int
        };
    }
    if out_of_order == H264_MAX_DPB_FRAMES as ::core::ffi::c_int {
        av_log(
            (*h).avctx as *mut ::core::ffi::c_void,
            40 as ::core::ffi::c_int,
            b"Invalid POC %d<%d\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*cur).poc,
            (*h).last_pocs[0 as ::core::ffi::c_int as usize],
        );
        i = 1 as ::core::ffi::c_int;
        while i < H264_MAX_DPB_FRAMES as ::core::ffi::c_int {
            (*h).last_pocs[i as usize] = -(2147483647 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int;
            i += 1;
        }
        (*h).last_pocs[0 as ::core::ffi::c_int as usize] = (*cur).poc;
        (*cur).mmco_reset = 1 as ::core::ffi::c_int;
    } else if (*(*h).avctx).has_b_frames < out_of_order
        && (*sps).bitstream_restriction_flag == 0
    {
        let mut loglevel: ::core::ffi::c_int = if (*(*h).avctx).frame_num > 1 as int64_t
        {
            24 as ::core::ffi::c_int
        } else {
            40 as ::core::ffi::c_int
        };
        av_log(
            (*h).avctx as *mut ::core::ffi::c_void,
            loglevel,
            b"Increasing reorder buffer to %d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            out_of_order,
        );
        (*(*h).avctx).has_b_frames = out_of_order;
    }
    pics = 0 as ::core::ffi::c_int;
    while !(*h).delayed_pic[pics as usize].is_null() {
        pics += 1;
    }
    if !(pics <= H264_MAX_DPB_FRAMES as ::core::ffi::c_int) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"pics <= H264_MAX_DPB_FRAMES\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavcodec/h264_slice.c\0" as *const u8 as *const ::core::ffi::c_char,
            1339 as ::core::ffi::c_int,
        );
        abort();
    }
    let fresh2 = pics;
    pics = pics + 1;
    (*h).delayed_pic[fresh2 as usize] = cur;
    if (*cur).reference == 0 as ::core::ffi::c_int {
        (*cur).reference = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
    }
    out = (*h).delayed_pic[0 as ::core::ffi::c_int as usize];
    out_idx = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while !(*h).delayed_pic[i as usize].is_null()
        && (*(*(*h).delayed_pic[i as usize]).f).flags
            & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int == 0
        && (*(*h).delayed_pic[i as usize]).mmco_reset == 0
    {
        if (*(*h).delayed_pic[i as usize]).poc < (*out).poc {
            out = (*h).delayed_pic[i as usize];
            out_idx = i;
        }
        i += 1;
    }
    if (*(*h).avctx).has_b_frames == 0 as ::core::ffi::c_int
        && ((*(*(*h).delayed_pic[0 as ::core::ffi::c_int as usize]).f).flags
            & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0
            || (*(*h).delayed_pic[0 as ::core::ffi::c_int as usize]).mmco_reset != 0)
    {
        (*h).next_outputed_poc = -(2147483647 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int;
    }
    out_of_order = ((*out).poc < (*h).next_outputed_poc) as ::core::ffi::c_int;
    if out_of_order != 0 || pics > (*(*h).avctx).has_b_frames {
        (*out).reference &= !((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int);
        i = out_idx;
        while !(*h).delayed_pic[i as usize].is_null() {
            (*h).delayed_pic[i as usize] = (*h)
                .delayed_pic[(i + 1 as ::core::ffi::c_int) as usize];
            i += 1;
        }
    }
    if out_of_order == 0 && pics > (*(*h).avctx).has_b_frames {
        (*h).next_output_pic = out;
        if out_idx == 0 as ::core::ffi::c_int
            && !(*h).delayed_pic[0 as ::core::ffi::c_int as usize].is_null()
            && ((*(*(*h).delayed_pic[0 as ::core::ffi::c_int as usize]).f).flags
                & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0
                || (*(*h).delayed_pic[0 as ::core::ffi::c_int as usize]).mmco_reset != 0)
        {
            (*h).next_outputed_poc = -(2147483647 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int;
        } else {
            (*h).next_outputed_poc = (*out).poc;
        }
        (*h).frame_recovered |= (*out).recovered;
        (*out).recovered
            |= (*h).frame_recovered
                & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
        if (*out).recovered == 0 {
            if (*(*h).avctx).flags & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                == 0
                && (*(*h).avctx).flags2
                    & (1 as ::core::ffi::c_int) << 22 as ::core::ffi::c_int == 0
            {
                (*h).next_output_pic = ::core::ptr::null_mut::<H264Picture>();
            } else {
                (*(*out).f).flags
                    |= (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
            }
        }
    } else {
        av_log(
            (*h).avctx as *mut ::core::ffi::c_void,
            48 as ::core::ffi::c_int,
            b"no picture %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            if out_of_order != 0 {
                b"ooo\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn h264_field_start(
    mut h: *mut H264Context,
    mut sl: *const H264SliceContext,
    mut nal: *const H2645NAL,
    mut first_slice: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut sps: *const SPS = ::core::ptr::null::<SPS>();
    let mut last_pic_structure: ::core::ffi::c_int = 0;
    let mut last_pic_droppable: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    ret = h264_init_ps(h, sl, first_slice);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    sps = (*h).ps.sps;
    if (*sps).bitstream_restriction_flag != 0
        && (*(*h).avctx).has_b_frames < (*sps).num_reorder_frames
    {
        (*(*h).avctx).has_b_frames = (*sps).num_reorder_frames;
    }
    last_pic_droppable = (*h).droppable;
    last_pic_structure = (*h).picture_structure;
    (*h).droppable = ((*nal).ref_idc == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    (*h).picture_structure = (*sl).picture_structure;
    (*h).poc.frame_num = (*sl).frame_num;
    (*h).poc.poc_lsb = (*sl).poc_lsb;
    (*h).poc.delta_poc_bottom = (*sl).delta_poc_bottom;
    (*h).poc.delta_poc[0 as ::core::ffi::c_int as usize] = (*sl)
        .delta_poc[0 as ::core::ffi::c_int as usize];
    (*h).poc.delta_poc[1 as ::core::ffi::c_int as usize] = (*sl)
        .delta_poc[1 as ::core::ffi::c_int as usize];
    if (*nal).type_0 == H264_NAL_IDR_SLICE as ::core::ffi::c_int {
        (*h).poc_offset = (*sl).idr_pic_id;
    } else if (*h).picture_intra_only != 0 {
        (*h).poc_offset = 0 as ::core::ffi::c_int;
    }
    if (*h).poc.frame_num != (*h).poc.prev_frame_num {
        let mut unwrap_prev_frame_num: ::core::ffi::c_int = (*h).poc.prev_frame_num;
        let mut max_frame_num: ::core::ffi::c_int = (1 as ::core::ffi::c_int)
            << (*sps).log2_max_frame_num;
        if unwrap_prev_frame_num > (*h).poc.frame_num {
            unwrap_prev_frame_num -= max_frame_num;
        }
        if (*h).poc.frame_num - unwrap_prev_frame_num > (*sps).ref_frame_count {
            unwrap_prev_frame_num = (*h).poc.frame_num - (*sps).ref_frame_count
                - 1 as ::core::ffi::c_int;
            if unwrap_prev_frame_num < 0 as ::core::ffi::c_int {
                unwrap_prev_frame_num += max_frame_num;
            }
            (*h).poc.prev_frame_num = unwrap_prev_frame_num;
        }
    }
    if (*h).first_field != 0 {
        let mut last_field: ::core::ffi::c_int = (last_pic_structure
            == 2 as ::core::ffi::c_int) as ::core::ffi::c_int;
        if (*h).cur_pic_ptr.is_null() {
            av_log(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                0 as ::core::ffi::c_int,
                b"Assertion %s failed at %s:%d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"h->cur_pic_ptr\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/libavcodec/h264_slice.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                1457 as ::core::ffi::c_int,
            );
            abort();
        }
        if (*(*(*h).cur_pic_ptr).f).buf[0 as ::core::ffi::c_int as usize].is_null() {
            av_log(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                0 as ::core::ffi::c_int,
                b"Assertion %s failed at %s:%d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"h->cur_pic_ptr->f->buf[0]\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"src/libavcodec/h264_slice.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                1458 as ::core::ffi::c_int,
            );
            abort();
        }
        if (*(*h).cur_pic_ptr).tf.owner[last_field as usize] == (*h).avctx {
            ff_thread_report_progress(
                &raw mut (*(*h).cur_pic_ptr).tf,
                2147483647 as ::core::ffi::c_int,
                last_field,
            );
        }
        if !((*h).picture_structure != 3 as ::core::ffi::c_int)
            || (*h).picture_structure == last_pic_structure
        {
            if last_pic_structure != 3 as ::core::ffi::c_int {
                ff_thread_report_progress(
                    &raw mut (*(*h).cur_pic_ptr).tf,
                    2147483647 as ::core::ffi::c_int,
                    (last_pic_structure == 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                );
            }
        } else if (*(*h).cur_pic_ptr).frame_num != (*h).poc.frame_num {
            if last_pic_structure != 3 as ::core::ffi::c_int {
                ff_thread_report_progress(
                    &raw mut (*(*h).cur_pic_ptr).tf,
                    2147483647 as ::core::ffi::c_int,
                    (last_pic_structure == 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                );
            }
        } else if !(last_pic_structure == 1 as ::core::ffi::c_int
            && (*h).picture_structure == 2 as ::core::ffi::c_int
            || last_pic_structure == 2 as ::core::ffi::c_int
                && (*h).picture_structure == 1 as ::core::ffi::c_int)
        {
            av_log(
                (*h).avctx as *mut ::core::ffi::c_void,
                16 as ::core::ffi::c_int,
                b"Invalid field mode combination %d/%d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                last_pic_structure,
                (*h).picture_structure,
            );
            (*h).picture_structure = last_pic_structure;
            (*h).droppable = last_pic_droppable;
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        } else if last_pic_droppable != (*h).droppable {
            avpriv_request_sample(
                (*h).avctx as *mut ::core::ffi::c_void,
                b"Found reference and non-reference fields in the same frame, which\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            (*h).picture_structure = last_pic_structure;
            (*h).droppable = last_pic_droppable;
            return -((('P' as i32 | ('A' as i32) << 8 as ::core::ffi::c_int
                | ('W' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('E' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
    }
    while (*h).poc.frame_num != (*h).poc.prev_frame_num && (*h).first_field == 0
        && (*h).poc.frame_num
            != ((*h).poc.prev_frame_num + 1 as ::core::ffi::c_int)
                % ((1 as ::core::ffi::c_int) << (*sps).log2_max_frame_num)
    {
        let mut prev: *const H264Picture = if (*h).short_ref_count != 0 {
            (*h).short_ref[0 as ::core::ffi::c_int as usize]
        } else {
            ::core::ptr::null_mut::<H264Picture>()
        };
        av_log(
            (*h).avctx as *mut ::core::ffi::c_void,
            48 as ::core::ffi::c_int,
            b"Frame num gap %d %d\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*h).poc.frame_num,
            (*h).poc.prev_frame_num,
        );
        if (*sps).gaps_in_frame_num_allowed_flag == 0 {
            i = 0 as ::core::ffi::c_int;
            while (i as usize)
                < (::core::mem::size_of::<[::core::ffi::c_int; 16]>() as usize)
                    .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>() as usize)
            {
                (*h).last_pocs[i as usize] = -(2147483647 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int;
                i += 1;
            }
        }
        ret = h264_frame_start(h);
        if ret < 0 as ::core::ffi::c_int {
            (*h).first_field = 0 as ::core::ffi::c_int;
            return ret;
        }
        (*h).poc.prev_frame_num += 1;
        (*h).poc.prev_frame_num
            %= (1 as ::core::ffi::c_int) << (*sps).log2_max_frame_num;
        (*(*h).cur_pic_ptr).frame_num = (*h).poc.prev_frame_num;
        (*(*h).cur_pic_ptr).invalid_gap = ((*sps).gaps_in_frame_num_allowed_flag == 0)
            as ::core::ffi::c_int;
        ff_thread_report_progress(
            &raw mut (*(*h).cur_pic_ptr).tf,
            2147483647 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        ff_thread_report_progress(
            &raw mut (*(*h).cur_pic_ptr).tf,
            2147483647 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        (*h).explicit_ref_marking = 0 as ::core::ffi::c_int;
        ret = ff_h264_execute_ref_pic_marking(h);
        if ret < 0 as ::core::ffi::c_int
            && (*(*h).avctx).err_recognition
                & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
        {
            return ret;
        }
        if (*h).short_ref_count != 0 {
            let mut c: [::core::ffi::c_int; 4] = [
                (1 as ::core::ffi::c_int)
                    << (*(*h).ps.sps).bit_depth_luma - 1 as ::core::ffi::c_int,
                (1 as ::core::ffi::c_int)
                    << (*(*h).ps.sps).bit_depth_chroma - 1 as ::core::ffi::c_int,
                (1 as ::core::ffi::c_int)
                    << (*(*h).ps.sps).bit_depth_chroma - 1 as ::core::ffi::c_int,
                -(1 as ::core::ffi::c_int),
            ];
            if !prev.is_null()
                && (*(*(*h).short_ref[0 as ::core::ffi::c_int as usize]).f).width
                    == (*(*prev).f).width
                && (*(*(*h).short_ref[0 as ::core::ffi::c_int as usize]).f).height
                    == (*(*prev).f).height
                && (*(*(*h).short_ref[0 as ::core::ffi::c_int as usize]).f).format
                    == (*(*prev).f).format
            {
                ff_thread_await_progress(
                    &raw const (*prev).tf,
                    2147483647 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                );
                if (*prev).field_picture != 0 {
                    ff_thread_await_progress(
                        &raw const (*prev).tf,
                        2147483647 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                    );
                }
                ff_thread_release_ext_buffer(
                    &raw mut (**(&raw mut (*h).short_ref as *mut *mut H264Picture)
                        .offset(0 as ::core::ffi::c_int as isize))
                        .tf,
                );
                (*(*h).short_ref[0 as ::core::ffi::c_int as usize]).tf.f = (*(*h)
                    .short_ref[0 as ::core::ffi::c_int as usize])
                    .f;
                ret = ff_thread_ref_frame(
                    &raw mut (**(&raw mut (*h).short_ref as *mut *mut H264Picture)
                        .offset(0 as ::core::ffi::c_int as isize))
                        .tf,
                    &raw const (*prev).tf,
                );
                if ret < 0 as ::core::ffi::c_int {
                    return ret;
                }
                (*(*h).short_ref[0 as ::core::ffi::c_int as usize]).poc = ((*prev).poc
                    as ::core::ffi::c_uint)
                    .wrapping_add(2 as ::core::ffi::c_uint) as ::core::ffi::c_int;
                (*(*h).short_ref[0 as ::core::ffi::c_int as usize]).gray = (*prev).gray;
                ff_thread_report_progress(
                    &raw mut (**(&raw mut (*h).short_ref as *mut *mut H264Picture)
                        .offset(0 as ::core::ffi::c_int as isize))
                        .tf,
                    2147483647 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                );
                if (*(*h).short_ref[0 as ::core::ffi::c_int as usize]).field_picture != 0
                {
                    ff_thread_report_progress(
                        &raw mut (**(&raw mut (*h).short_ref as *mut *mut H264Picture)
                            .offset(0 as ::core::ffi::c_int as isize))
                            .tf,
                        2147483647 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                    );
                }
            } else if (*h).frame_recovered == 0 {
                if (*(*h).avctx).hwaccel.is_null() {
                    color_frame(
                        (*(*h).short_ref[0 as ::core::ffi::c_int as usize]).f,
                        &raw mut c as *mut ::core::ffi::c_int
                            as *const ::core::ffi::c_int,
                    );
                }
                (*(*h).short_ref[0 as ::core::ffi::c_int as usize]).gray = 1
                    as ::core::ffi::c_int;
            }
            (*(*h).short_ref[0 as ::core::ffi::c_int as usize]).frame_num = (*h)
                .poc
                .prev_frame_num;
        }
    }
    if (*h).first_field != 0 {
        if (*h).cur_pic_ptr.is_null() {
            av_log(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                0 as ::core::ffi::c_int,
                b"Assertion %s failed at %s:%d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"h->cur_pic_ptr\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/libavcodec/h264_slice.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                1577 as ::core::ffi::c_int,
            );
            abort();
        }
        if (*(*(*h).cur_pic_ptr).f).buf[0 as ::core::ffi::c_int as usize].is_null() {
            av_log(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                0 as ::core::ffi::c_int,
                b"Assertion %s failed at %s:%d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"h->cur_pic_ptr->f->buf[0]\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"src/libavcodec/h264_slice.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                1578 as ::core::ffi::c_int,
            );
            abort();
        }
        if !((*h).picture_structure != 3 as ::core::ffi::c_int)
            || (*h).picture_structure == last_pic_structure
        {
            (*h).missing_fields += 1;
            (*h).cur_pic_ptr = ::core::ptr::null_mut::<H264Picture>();
            (*h).first_field = ((*h).picture_structure != 3 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
        } else {
            (*h).missing_fields = 0 as ::core::ffi::c_int;
            if (*(*h).cur_pic_ptr).frame_num != (*h).poc.frame_num {
                ff_thread_report_progress(
                    &raw mut (*(*h).cur_pic_ptr).tf,
                    2147483647 as ::core::ffi::c_int,
                    ((*h).picture_structure == 2 as ::core::ffi::c_int)
                        as ::core::ffi::c_int,
                );
                (*h).first_field = 1 as ::core::ffi::c_int;
                (*h).cur_pic_ptr = ::core::ptr::null_mut::<H264Picture>();
            } else if (*(*h).cur_pic_ptr).reference
                & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
            {
                (*h).first_field = 1 as ::core::ffi::c_int;
                (*h).cur_pic_ptr = ::core::ptr::null_mut::<H264Picture>();
            } else {
                (*h).first_field = 0 as ::core::ffi::c_int;
            }
        }
    } else {
        (*h).first_field = ((*h).picture_structure != 3 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
    }
    if !((*h).picture_structure != 3 as ::core::ffi::c_int) || (*h).first_field != 0 {
        if h264_frame_start(h) < 0 as ::core::ffi::c_int {
            (*h).first_field = 0 as ::core::ffi::c_int;
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
    } else {
        let mut field: ::core::ffi::c_int = ((*h).picture_structure
            == 2 as ::core::ffi::c_int) as ::core::ffi::c_int;
        release_unused_pictures(h, 0 as ::core::ffi::c_int);
        (*(*h).cur_pic_ptr).tf.owner[field as usize] = (*h).avctx;
    }
    if (*h).picture_structure != 3 as ::core::ffi::c_int {
        i = ((*h).picture_structure == 2 as ::core::ffi::c_int) as ::core::ffi::c_int;
        while i < (*h).mb_height {
            memset(
                (*h).slice_table.offset((i * (*h).mb_stride) as isize)
                    as *mut ::core::ffi::c_void,
                -(1 as ::core::ffi::c_int),
                (((*h).mb_stride
                    - (i + 1 as ::core::ffi::c_int == (*h).mb_height)
                        as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
            );
            i += 1;
        }
    } else {
        memset(
            (*h).slice_table as *mut ::core::ffi::c_void,
            -(1 as ::core::ffi::c_int),
            (((*h).mb_height * (*h).mb_stride - 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        );
    }
    ret = ff_h264_init_poc(
        &raw mut (*(*h).cur_pic_ptr).field_poc as *mut ::core::ffi::c_int,
        &raw mut (*(*h).cur_pic_ptr).poc,
        (*h).ps.sps,
        &raw mut (*h).poc,
        (*h).picture_structure,
        (*nal).ref_idc,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    memcpy(
        &raw mut (*h).mmco as *mut MMCO as *mut ::core::ffi::c_void,
        &raw const (*sl).mmco as *const MMCO as *const ::core::ffi::c_void,
        ((*sl).nb_mmco as size_t).wrapping_mul(::core::mem::size_of::<MMCO>() as size_t),
    );
    (*h).nb_mmco = (*sl).nb_mmco;
    (*h).explicit_ref_marking = (*sl).explicit_ref_marking;
    (*h).picture_idr = ((*nal).type_0 == H264_NAL_IDR_SLICE as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    if (*h).sei.recovery_point.recovery_frame_cnt >= 0 as ::core::ffi::c_int {
        let sei_recovery_frame_cnt: ::core::ffi::c_int = (*h)
            .sei
            .recovery_point
            .recovery_frame_cnt;
        if (*h).poc.frame_num != sei_recovery_frame_cnt
            || (*sl).slice_type_nos != AV_PICTURE_TYPE_I as ::core::ffi::c_int
        {
            (*h).valid_recovery_point = 1 as ::core::ffi::c_int;
        }
        if (*h).recovery_frame < 0 as ::core::ffi::c_int
            || av_zero_extend_c(
                ((*h).recovery_frame - (*h).poc.frame_num) as ::core::ffi::c_uint,
                (*(*h).ps.sps).log2_max_frame_num as ::core::ffi::c_uint,
            ) > sei_recovery_frame_cnt as ::core::ffi::c_uint
        {
            (*h).recovery_frame = av_zero_extend_c(
                ((*h).poc.frame_num + sei_recovery_frame_cnt) as ::core::ffi::c_uint,
                (*(*h).ps.sps).log2_max_frame_num as ::core::ffi::c_uint,
            ) as ::core::ffi::c_int;
            if (*h).valid_recovery_point == 0 {
                (*h).recovery_frame = (*h).poc.frame_num;
            }
        }
    }
    (*(*(*h).cur_pic_ptr).f).flags
        |= ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
            * ((*nal).type_0 == H264_NAL_IDR_SLICE as ::core::ffi::c_int)
                as ::core::ffi::c_int;
    if (*nal).type_0 == H264_NAL_IDR_SLICE as ::core::ffi::c_int {
        (*(*h).cur_pic_ptr).recovered
            |= (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
        (*h).frame_recovered |= (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
    }
    if (*h).recovery_frame == (*h).poc.frame_num && (*nal).ref_idc != 0 {
        (*h).recovery_frame = -(1 as ::core::ffi::c_int);
        (*(*h).cur_pic_ptr).recovered
            |= (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
    }
    (*(*h).cur_pic_ptr).recovered |= (*h).frame_recovered;
    if !((*h).picture_structure != 3 as ::core::ffi::c_int) || (*h).first_field == 0
        || (*h).missing_fields > 1 as ::core::ffi::c_int
    {
        ret = h264_export_frame_props(h);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        ret = h264_select_output_frame(h);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn h264_slice_header_parse(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut nal: *const H2645NAL,
) -> ::core::ffi::c_int {
    let mut sps: *const SPS = ::core::ptr::null::<SPS>();
    let mut pps: *const PPS = ::core::ptr::null::<PPS>();
    let mut ret: ::core::ffi::c_int = 0;
    let mut slice_type: ::core::ffi::c_uint = 0;
    let mut tmp: ::core::ffi::c_uint = 0;
    let mut i: ::core::ffi::c_uint = 0;
    let mut field_pic_flag: ::core::ffi::c_int = 0;
    let mut bottom_field_flag: ::core::ffi::c_int = 0;
    let mut first_slice: ::core::ffi::c_int = (sl == (*h).slice_ctx
        && (*h).current_slice == 0) as ::core::ffi::c_int;
    let mut picture_structure: ::core::ffi::c_int = 0;
    if first_slice != 0 {
        if (*h).setup_finished != 0 {
            av_log(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                0 as ::core::ffi::c_int,
                b"Assertion %s failed at %s:%d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"!h->setup_finished\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/libavcodec/h264_slice.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                1708 as ::core::ffi::c_int,
            );
            abort();
        }
    }
    (*sl).first_mb_addr = get_ue_golomb_long(&raw mut (*sl).gb);
    slice_type = get_ue_golomb_31(&raw mut (*sl).gb) as ::core::ffi::c_uint;
    if slice_type > 9 as ::core::ffi::c_uint {
        av_log(
            (*h).avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"slice type %d too large at %d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            slice_type,
            (*sl).first_mb_addr,
        );
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if slice_type > 4 as ::core::ffi::c_uint {
        slice_type = slice_type.wrapping_sub(5 as ::core::ffi::c_uint);
        (*sl).slice_type_fixed = 1 as ::core::ffi::c_int;
    } else {
        (*sl).slice_type_fixed = 0 as ::core::ffi::c_int;
    }
    slice_type = ff_h264_golomb_to_pict_type[slice_type as usize] as ::core::ffi::c_uint;
    (*sl).slice_type = slice_type as ::core::ffi::c_int;
    (*sl).slice_type_nos = (slice_type & 3 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    if (*nal).type_0 == H264_NAL_IDR_SLICE as ::core::ffi::c_int
        && (*sl).slice_type_nos != AV_PICTURE_TYPE_I as ::core::ffi::c_int
    {
        av_log(
            (*h).avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"A non-intra slice in an IDR NAL unit.\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    (*sl).pps_id = get_ue_golomb(&raw mut (*sl).gb) as ::core::ffi::c_uint;
    if (*sl).pps_id >= 256 as ::core::ffi::c_uint {
        av_log(
            (*h).avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"pps_id %u out of range\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*sl).pps_id,
        );
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if (*h).ps.pps_list[(*sl).pps_id as usize].is_null() {
        av_log(
            (*h).avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"non-existing PPS %u referenced\n\0" as *const u8
                as *const ::core::ffi::c_char,
            (*sl).pps_id,
        );
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    pps = (*h).ps.pps_list[(*sl).pps_id as usize];
    sps = (*pps).sps;
    (*sl).frame_num = get_bits(&raw mut (*sl).gb, (*sps).log2_max_frame_num)
        as ::core::ffi::c_int;
    if first_slice == 0 {
        if (*h).poc.frame_num != (*sl).frame_num {
            av_log(
                (*h).avctx as *mut ::core::ffi::c_void,
                16 as ::core::ffi::c_int,
                b"Frame num change from %d to %d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*h).poc.frame_num,
                (*sl).frame_num,
            );
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
    }
    (*sl).mb_mbaff = 0 as ::core::ffi::c_int;
    if (*sps).frame_mbs_only_flag != 0 {
        picture_structure = 3 as ::core::ffi::c_int;
    } else {
        if (*sps).direct_8x8_inference_flag == 0
            && slice_type
                == AV_PICTURE_TYPE_B as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            av_log(
                (*h).avctx as *mut ::core::ffi::c_void,
                16 as ::core::ffi::c_int,
                b"This stream was generated by a broken encoder, invalid 8x8 inference\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        field_pic_flag = get_bits1(&raw mut (*sl).gb) as ::core::ffi::c_int;
        if field_pic_flag != 0 {
            bottom_field_flag = get_bits1(&raw mut (*sl).gb) as ::core::ffi::c_int;
            picture_structure = 1 as ::core::ffi::c_int + bottom_field_flag;
        } else {
            picture_structure = 3 as ::core::ffi::c_int;
        }
    }
    (*sl).picture_structure = picture_structure;
    (*sl).mb_field_decoding_flag = (picture_structure != 3 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    if picture_structure == 3 as ::core::ffi::c_int {
        (*sl).curr_pic_num = (*sl).frame_num;
        (*sl).max_pic_num = (1 as ::core::ffi::c_int) << (*sps).log2_max_frame_num;
    } else {
        (*sl).curr_pic_num = 2 as ::core::ffi::c_int * (*sl).frame_num
            + 1 as ::core::ffi::c_int;
        (*sl).max_pic_num = (1 as ::core::ffi::c_int)
            << (*sps).log2_max_frame_num + 1 as ::core::ffi::c_int;
    }
    if (*nal).type_0 == H264_NAL_IDR_SLICE as ::core::ffi::c_int {
        let mut idr_pic_id: ::core::ffi::c_uint = get_ue_golomb_long(&raw mut (*sl).gb);
        if idr_pic_id < 65536 as ::core::ffi::c_int as ::core::ffi::c_uint {
            (*sl).idr_pic_id = idr_pic_id as ::core::ffi::c_int;
        } else {
            av_log(
                (*h).avctx as *mut ::core::ffi::c_void,
                24 as ::core::ffi::c_int,
                b"idr_pic_id is invalid\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    (*sl).poc_lsb = 0 as ::core::ffi::c_int;
    (*sl).delta_poc_bottom = 0 as ::core::ffi::c_int;
    if (*sps).poc_type == 0 as ::core::ffi::c_int {
        (*sl).poc_lsb = get_bits(&raw mut (*sl).gb, (*sps).log2_max_poc_lsb)
            as ::core::ffi::c_int;
        if (*pps).pic_order_present == 1 as ::core::ffi::c_int
            && picture_structure == 3 as ::core::ffi::c_int
        {
            (*sl).delta_poc_bottom = get_se_golomb(&raw mut (*sl).gb);
        }
    }
    (*sl).delta_poc[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
    (*sl).delta_poc[0 as ::core::ffi::c_int as usize] = (*sl)
        .delta_poc[1 as ::core::ffi::c_int as usize];
    if (*sps).poc_type == 1 as ::core::ffi::c_int
        && (*sps).delta_pic_order_always_zero_flag == 0
    {
        (*sl).delta_poc[0 as ::core::ffi::c_int as usize] = get_se_golomb(
            &raw mut (*sl).gb,
        );
        if (*pps).pic_order_present == 1 as ::core::ffi::c_int
            && picture_structure == 3 as ::core::ffi::c_int
        {
            (*sl).delta_poc[1 as ::core::ffi::c_int as usize] = get_se_golomb(
                &raw mut (*sl).gb,
            );
        }
    }
    (*sl).redundant_pic_count = 0 as ::core::ffi::c_int;
    if (*pps).redundant_pic_cnt_present != 0 {
        (*sl).redundant_pic_count = get_ue_golomb(&raw mut (*sl).gb);
    }
    if (*sl).slice_type_nos == AV_PICTURE_TYPE_B as ::core::ffi::c_int {
        (*sl).direct_spatial_mv_pred = get_bits1(&raw mut (*sl).gb)
            as ::core::ffi::c_int;
    }
    ret = ff_h264_parse_ref_count(
        &raw mut (*sl).list_count as *mut ::core::ffi::c_int,
        &raw mut (*sl).ref_count as *mut ::core::ffi::c_uint as *mut ::core::ffi::c_int,
        &raw mut (*sl).gb,
        pps,
        (*sl).slice_type_nos,
        picture_structure,
        (*h).avctx as *mut ::core::ffi::c_void,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    if (*sl).slice_type_nos != AV_PICTURE_TYPE_I as ::core::ffi::c_int {
        ret = ff_h264_decode_ref_pic_list_reordering(
            sl,
            (*h).avctx as *mut ::core::ffi::c_void,
        );
        if ret < 0 as ::core::ffi::c_int {
            (*sl).ref_count[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_uint;
            (*sl).ref_count[1 as ::core::ffi::c_int as usize] = (*sl)
                .ref_count[0 as ::core::ffi::c_int as usize];
            return ret;
        }
    }
    (*sl).pwt.use_weight = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_uint;
    while i < 2 as ::core::ffi::c_uint {
        (*sl).pwt.luma_weight_flag[i as usize] = 0 as ::core::ffi::c_int;
        (*sl).pwt.chroma_weight_flag[i as usize] = 0 as ::core::ffi::c_int;
        i = i.wrapping_add(1);
    }
    if (*pps).weighted_pred != 0
        && (*sl).slice_type_nos == AV_PICTURE_TYPE_P as ::core::ffi::c_int
        || (*pps).weighted_bipred_idc == 1 as ::core::ffi::c_int
            && (*sl).slice_type_nos == AV_PICTURE_TYPE_B as ::core::ffi::c_int
    {
        ret = ff_h264_pred_weight_table(
            &raw mut (*sl).gb,
            sps,
            &raw mut (*sl).ref_count as *mut ::core::ffi::c_uint
                as *const ::core::ffi::c_int,
            (*sl).slice_type_nos,
            &raw mut (*sl).pwt,
            picture_structure,
            (*h).avctx as *mut ::core::ffi::c_void,
        );
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
    }
    (*sl).explicit_ref_marking = 0 as ::core::ffi::c_int;
    if (*nal).ref_idc != 0 {
        ret = ff_h264_decode_ref_pic_marking(
            sl,
            &raw mut (*sl).gb,
            nal,
            (*h).avctx as *mut ::core::ffi::c_void,
        );
        if ret < 0 as ::core::ffi::c_int
            && (*(*h).avctx).err_recognition
                & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
        {
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
    }
    if (*sl).slice_type_nos != AV_PICTURE_TYPE_I as ::core::ffi::c_int
        && (*pps).cabac != 0
    {
        tmp = get_ue_golomb_31(&raw mut (*sl).gb) as ::core::ffi::c_uint;
        if tmp > 2 as ::core::ffi::c_uint {
            av_log(
                (*h).avctx as *mut ::core::ffi::c_void,
                16 as ::core::ffi::c_int,
                b"cabac_init_idc %u overflow\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                tmp,
            );
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        (*sl).cabac_init_idc = tmp as ::core::ffi::c_int;
    }
    (*sl).last_qscale_diff = 0 as ::core::ffi::c_int;
    tmp = ((*pps).init_qp as ::core::ffi::c_uint)
        .wrapping_add(get_se_golomb(&raw mut (*sl).gb) as ::core::ffi::c_uint);
    if tmp
        > (51 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * ((*sps).bit_depth_luma - 8 as ::core::ffi::c_int))
            as ::core::ffi::c_uint
    {
        av_log(
            (*h).avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"QP %u out of range\n\0" as *const u8 as *const ::core::ffi::c_char,
            tmp,
        );
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    (*sl).qscale = tmp as ::core::ffi::c_int;
    (*sl).chroma_qp[0 as ::core::ffi::c_int as usize] = get_chroma_qp(
        pps,
        0 as ::core::ffi::c_int,
        (*sl).qscale,
    );
    (*sl).chroma_qp[1 as ::core::ffi::c_int as usize] = get_chroma_qp(
        pps,
        1 as ::core::ffi::c_int,
        (*sl).qscale,
    );
    if (*sl).slice_type == AV_PICTURE_TYPE_SP as ::core::ffi::c_int {
        get_bits1(&raw mut (*sl).gb);
    }
    if (*sl).slice_type == AV_PICTURE_TYPE_SP as ::core::ffi::c_int
        || (*sl).slice_type == AV_PICTURE_TYPE_SI as ::core::ffi::c_int
    {
        get_se_golomb(&raw mut (*sl).gb);
    }
    (*sl).deblocking_filter = 1 as ::core::ffi::c_int;
    (*sl).slice_alpha_c0_offset = 0 as ::core::ffi::c_int;
    (*sl).slice_beta_offset = 0 as ::core::ffi::c_int;
    if (*pps).deblocking_filter_parameters_present != 0 {
        tmp = get_ue_golomb_31(&raw mut (*sl).gb) as ::core::ffi::c_uint;
        if tmp > 2 as ::core::ffi::c_uint {
            av_log(
                (*h).avctx as *mut ::core::ffi::c_void,
                16 as ::core::ffi::c_int,
                b"deblocking_filter_idc %u out of range\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                tmp,
            );
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        (*sl).deblocking_filter = tmp as ::core::ffi::c_int;
        if (*sl).deblocking_filter < 2 as ::core::ffi::c_int {
            (*sl).deblocking_filter ^= 1 as ::core::ffi::c_int;
        }
        if (*sl).deblocking_filter != 0 {
            let mut slice_alpha_c0_offset_div2: ::core::ffi::c_int = get_se_golomb(
                &raw mut (*sl).gb,
            );
            let mut slice_beta_offset_div2: ::core::ffi::c_int = get_se_golomb(
                &raw mut (*sl).gb,
            );
            if slice_alpha_c0_offset_div2 > 6 as ::core::ffi::c_int
                || slice_alpha_c0_offset_div2 < -(6 as ::core::ffi::c_int)
                || slice_beta_offset_div2 > 6 as ::core::ffi::c_int
                || slice_beta_offset_div2 < -(6 as ::core::ffi::c_int)
            {
                av_log(
                    (*h).avctx as *mut ::core::ffi::c_void,
                    16 as ::core::ffi::c_int,
                    b"deblocking filter parameters %d %d out of range\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    slice_alpha_c0_offset_div2,
                    slice_beta_offset_div2,
                );
                return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                    | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
            }
            (*sl).slice_alpha_c0_offset = slice_alpha_c0_offset_div2
                * 2 as ::core::ffi::c_int;
            (*sl).slice_beta_offset = slice_beta_offset_div2 * 2 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn h264_slice_init(
    mut h: *mut H264Context,
    mut sl: *mut H264SliceContext,
    mut nal: *const H2645NAL,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*h).picture_idr != 0 && (*nal).type_0 != H264_NAL_IDR_SLICE as ::core::ffi::c_int
    {
        av_log(
            (*h).avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"Invalid mix of IDR and non-IDR slices\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if (*sl).first_mb_addr
        << ((*h).mb_aff_frame != 0 || (*h).picture_structure != 3 as ::core::ffi::c_int)
            as ::core::ffi::c_int >= (*h).mb_num as ::core::ffi::c_uint
        || (*sl).first_mb_addr >= (*h).mb_num as ::core::ffi::c_uint
    {
        av_log(
            (*h).avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"first_mb_in_slice overflow\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    (*sl).mb_x = (*sl).first_mb_addr.wrapping_rem((*h).mb_width as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
    (*sl).resync_mb_x = (*sl).mb_x;
    (*sl).mb_y = ((*sl).first_mb_addr.wrapping_div((*h).mb_width as ::core::ffi::c_uint)
        << ((*h).mb_aff_frame != 0 || (*h).picture_structure != 3 as ::core::ffi::c_int)
            as ::core::ffi::c_int) as ::core::ffi::c_int;
    (*sl).resync_mb_y = (*sl).mb_y;
    if (*h).picture_structure == 2 as ::core::ffi::c_int {
        (*sl).mb_y = (*sl).mb_y + 1 as ::core::ffi::c_int;
        (*sl).resync_mb_y = (*sl).mb_y;
    }
    ret = ff_h264_build_ref_list(h, sl);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    if (*(*h).ps.pps).weighted_bipred_idc == 2 as ::core::ffi::c_int
        && (*sl).slice_type_nos == AV_PICTURE_TYPE_B as ::core::ffi::c_int
    {
        implicit_weight_table(h, sl, -(1 as ::core::ffi::c_int));
        if (*h).mb_aff_frame != 0 {
            implicit_weight_table(h, sl, 0 as ::core::ffi::c_int);
            implicit_weight_table(h, sl, 1 as ::core::ffi::c_int);
        }
    }
    if (*sl).slice_type_nos == AV_PICTURE_TYPE_B as ::core::ffi::c_int
        && (*sl).direct_spatial_mv_pred == 0
    {
        ff_h264_direct_dist_scale_factor(h, sl);
    }
    ff_h264_direct_ref_list_init(h, sl);
    if (*(*h).avctx).skip_loop_filter as ::core::ffi::c_int
        >= AVDISCARD_ALL as ::core::ffi::c_int
        || (*(*h).avctx).skip_loop_filter as ::core::ffi::c_int
            >= AVDISCARD_NONKEY as ::core::ffi::c_int
            && (*h).nal_unit_type != H264_NAL_IDR_SLICE as ::core::ffi::c_int
        || (*(*h).avctx).skip_loop_filter as ::core::ffi::c_int
            >= AVDISCARD_NONINTRA as ::core::ffi::c_int
            && (*sl).slice_type_nos != AV_PICTURE_TYPE_I as ::core::ffi::c_int
        || (*(*h).avctx).skip_loop_filter as ::core::ffi::c_int
            >= AVDISCARD_BIDIR as ::core::ffi::c_int
            && (*sl).slice_type_nos == AV_PICTURE_TYPE_B as ::core::ffi::c_int
        || (*(*h).avctx).skip_loop_filter as ::core::ffi::c_int
            >= AVDISCARD_NONREF as ::core::ffi::c_int
            && (*nal).ref_idc == 0 as ::core::ffi::c_int
    {
        (*sl).deblocking_filter = 0 as ::core::ffi::c_int;
    }
    if (*sl).deblocking_filter == 1 as ::core::ffi::c_int
        && (*h).nb_slice_ctx > 1 as ::core::ffi::c_int
    {
        if (*(*h).avctx).flags2 & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            != 0
        {
            (*sl).deblocking_filter = 2 as ::core::ffi::c_int;
        } else {
            (*h).postpone_filter = 1 as ::core::ffi::c_int;
        }
    }
    (*sl).qp_thresh = 15 as ::core::ffi::c_int
        - (if (*sl).slice_alpha_c0_offset > (*sl).slice_beta_offset {
            (*sl).slice_beta_offset
        } else {
            (*sl).slice_alpha_c0_offset
        })
        - (if (if 0 as ::core::ffi::c_int
            > (*(*h).ps.pps).chroma_qp_index_offset[0 as ::core::ffi::c_int as usize]
        {
            0 as ::core::ffi::c_int
        } else {
            (*(*h).ps.pps).chroma_qp_index_offset[0 as ::core::ffi::c_int as usize]
        }) > (*(*h).ps.pps).chroma_qp_index_offset[1 as ::core::ffi::c_int as usize]
        {
            (if 0 as ::core::ffi::c_int
                > (*(*h).ps.pps).chroma_qp_index_offset[0 as ::core::ffi::c_int as usize]
            {
                0 as ::core::ffi::c_int
            } else {
                (*(*h).ps.pps).chroma_qp_index_offset[0 as ::core::ffi::c_int as usize]
            })
        } else {
            (*(*h).ps.pps).chroma_qp_index_offset[1 as ::core::ffi::c_int as usize]
        })
        + 6 as ::core::ffi::c_int
            * ((*(*h).ps.sps).bit_depth_luma - 8 as ::core::ffi::c_int);
    if (*h).current_slice >= 0xfffe as ::core::ffi::c_int {
        av_log(
            (*h).avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"Too many slices (%d)\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*h).current_slice + 1 as ::core::ffi::c_int,
        );
        return -((('P' as i32 | ('A' as i32) << 8 as ::core::ffi::c_int
            | ('W' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('E' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    (*h).current_slice += 1;
    (*sl).slice_num = (*h).current_slice;
    if (*sl).slice_num != 0 {
        (*h)
            .slice_row[((*sl).slice_num - 1 as ::core::ffi::c_int
            & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize] = (*sl)
            .resync_mb_y as int16_t;
    }
    if (*h)
        .slice_row[((*sl).slice_num & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        as usize] as ::core::ffi::c_int + 3 as ::core::ffi::c_int >= (*sl).resync_mb_y
        && (*h)
            .slice_row[((*sl).slice_num
            & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int <= (*sl).resync_mb_y
        && (*sl).slice_num >= 32 as ::core::ffi::c_int
    {
        av_log(
            (*h).avctx as *mut ::core::ffi::c_void,
            24 as ::core::ffi::c_int,
            b"Possibly too many slices (%d >= %d), increase MAX_SLICES and recompile if there are artifacts\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            (*sl).slice_num,
            32 as ::core::ffi::c_int,
        );
    }
    j = 0 as ::core::ffi::c_int;
    while j < 2 as ::core::ffi::c_int {
        let mut id_list: [::core::ffi::c_int; 16] = [0; 16];
        let mut ref2frm: *mut ::core::ffi::c_int = &raw mut *(&raw mut *(&raw mut (*h)
            .ref2frm as *mut [[::core::ffi::c_int; 64]; 2])
            .offset(
                ((*sl).slice_num & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    as isize,
            ) as *mut [::core::ffi::c_int; 64])
            .offset(j as isize) as *mut ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < 16 as ::core::ffi::c_int {
            id_list[i as usize] = 60 as ::core::ffi::c_int;
            if (j as ::core::ffi::c_uint) < (*sl).list_count
                && (i as ::core::ffi::c_uint) < (*sl).ref_count[j as usize]
                && !(*(*(*sl).ref_list[j as usize][i as usize].parent).f)
                    .buf[0 as ::core::ffi::c_int as usize]
                    .is_null()
            {
                let mut k: ::core::ffi::c_int = 0;
                let mut buf: *const AVBuffer = (*(*(*(*sl)
                    .ref_list[j as usize][i as usize]
                    .parent)
                    .f)
                    .buf[0 as ::core::ffi::c_int as usize])
                    .buffer;
                k = 0 as ::core::ffi::c_int;
                while k < (*h).short_ref_count {
                    if (*(*(*(*h).short_ref[k as usize]).f)
                        .buf[0 as ::core::ffi::c_int as usize])
                        .buffer == buf as *mut AVBuffer
                    {
                        id_list[i as usize] = k;
                        break;
                    } else {
                        k += 1;
                    }
                }
                k = 0 as ::core::ffi::c_int;
                while k < (*h).long_ref_count {
                    if !(*h).long_ref[k as usize].is_null()
                        && (*(*(*(*h).long_ref[k as usize]).f)
                            .buf[0 as ::core::ffi::c_int as usize])
                            .buffer == buf as *mut AVBuffer
                    {
                        id_list[i as usize] = (*h).short_ref_count + k;
                        break;
                    } else {
                        k += 1;
                    }
                }
            }
            i += 1;
        }
        let ref mut fresh0 = *ref2frm.offset(1 as ::core::ffi::c_int as isize);
        *fresh0 = -(1 as ::core::ffi::c_int);
        *ref2frm.offset(0 as ::core::ffi::c_int as isize) = *fresh0;
        i = 0 as ::core::ffi::c_int;
        while i < 16 as ::core::ffi::c_int {
            *ref2frm.offset((i + 2 as ::core::ffi::c_int) as isize) = 4
                as ::core::ffi::c_int * id_list[i as usize]
                + ((*sl).ref_list[j as usize][i as usize].reference
                    & 3 as ::core::ffi::c_int);
            i += 1;
        }
        let ref mut fresh1 = *ref2frm
            .offset((18 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize);
        *fresh1 = -(1 as ::core::ffi::c_int);
        *ref2frm.offset((18 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize) = *fresh1;
        i = 16 as ::core::ffi::c_int;
        while i < 48 as ::core::ffi::c_int {
            *ref2frm.offset((i + 4 as ::core::ffi::c_int) as isize) = 4
                as ::core::ffi::c_int
                * id_list[(i - 16 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    as usize]
                + ((*sl).ref_list[j as usize][i as usize].reference
                    & 3 as ::core::ffi::c_int);
            i += 1;
        }
        j += 1;
    }
    if (*sl).slice_type_nos == AV_PICTURE_TYPE_I as ::core::ffi::c_int {
        (*(*h).cur_pic_ptr).gray = 0 as ::core::ffi::c_int;
        (*h).non_gray = 1 as ::core::ffi::c_int;
    } else {
        let mut gray: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        j = 0 as ::core::ffi::c_int;
        while (j as ::core::ffi::c_uint) < (*sl).list_count {
            i = 0 as ::core::ffi::c_int;
            while (i as ::core::ffi::c_uint) < (*sl).ref_count[j as usize] {
                gray |= (*(*sl).ref_list[j as usize][i as usize].parent).gray;
                i += 1;
            }
            j += 1;
        }
        (*(*h).cur_pic_ptr).gray = gray;
    }
    if (*(*h).avctx).debug & 1 as ::core::ffi::c_int != 0 {
        av_log(
            (*h).avctx as *mut ::core::ffi::c_void,
            48 as ::core::ffi::c_int,
            b"slice:%d %c mb:%d %c%s%s frame:%d poc:%d/%d ref:%d/%d qp:%d loop:%d:%d:%d weight:%d%s %s\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            (*sl).slice_num,
            if (*h).picture_structure == 3 as ::core::ffi::c_int {
                'F' as i32
            } else if (*h).picture_structure == 1 as ::core::ffi::c_int {
                'T' as i32
            } else {
                'B' as i32
            },
            (*sl).mb_y * (*h).mb_width + (*sl).mb_x,
            av_get_picture_type_char((*sl).slice_type as AVPictureType)
                as ::core::ffi::c_int,
            if (*sl).slice_type_fixed != 0 {
                b" fix\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
            if (*nal).type_0 == H264_NAL_IDR_SLICE as ::core::ffi::c_int {
                b" IDR\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
            (*h).poc.frame_num,
            (*(*h).cur_pic_ptr).field_poc[0 as ::core::ffi::c_int as usize],
            (*(*h).cur_pic_ptr).field_poc[1 as ::core::ffi::c_int as usize],
            (*sl).ref_count[0 as ::core::ffi::c_int as usize],
            (*sl).ref_count[1 as ::core::ffi::c_int as usize],
            (*sl).qscale,
            (*sl).deblocking_filter,
            (*sl).slice_alpha_c0_offset,
            (*sl).slice_beta_offset,
            (*sl).pwt.use_weight,
            if (*sl).pwt.use_weight == 1 as ::core::ffi::c_int
                && (*sl).pwt.use_weight_chroma != 0
            {
                b"c\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
            if (*sl).slice_type == AV_PICTURE_TYPE_B as ::core::ffi::c_int {
                if (*sl).direct_spatial_mv_pred != 0 {
                    b"SPAT\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"TEMP\0" as *const u8 as *const ::core::ffi::c_char
                }
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_h264_queue_decode_slice(
    mut h: *mut H264Context,
    mut nal: *const H2645NAL,
) -> ::core::ffi::c_int {
    let mut sl: *mut H264SliceContext = (*h)
        .slice_ctx
        .offset((*h).nb_slice_ctx_queued as isize);
    let mut first_slice: ::core::ffi::c_int = (sl == (*h).slice_ctx
        && (*h).current_slice == 0) as ::core::ffi::c_int;
    let mut ret: ::core::ffi::c_int = 0;
    (*sl).gb = (*nal).gb;
    ret = h264_slice_header_parse(h, sl, nal);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    if (*sl).redundant_pic_count > 0 as ::core::ffi::c_int {
        (*sl).ref_count[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_uint;
        (*sl).ref_count[0 as ::core::ffi::c_int as usize] = (*sl)
            .ref_count[1 as ::core::ffi::c_int as usize];
        return 0 as ::core::ffi::c_int;
    }
    if (*sl).first_mb_addr == 0 as ::core::ffi::c_uint || (*h).current_slice == 0 {
        if (*h).setup_finished != 0 {
            av_log(
                (*h).avctx as *mut ::core::ffi::c_void,
                16 as ::core::ffi::c_int,
                b"Too many fields\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
    }
    if (*sl).first_mb_addr == 0 as ::core::ffi::c_uint {
        if (*h).current_slice != 0 {
            if (*h).nb_slice_ctx_queued != 0 {
                let mut tmp_ctx: H264SliceContext = H264SliceContext {
                    h264: ::core::ptr::null::<H264Context>(),
                    gb: GetBitContext {
                        buffer: ::core::ptr::null::<uint8_t>(),
                        index: 0,
                        size_in_bits: 0,
                        size_in_bits_plus8: 0,
                    },
                    er: ::core::ptr::null_mut::<ERContext>(),
                    slice_num: 0,
                    slice_type: 0,
                    slice_type_nos: 0,
                    slice_type_fixed: 0,
                    qscale: 0,
                    chroma_qp: [0; 2],
                    qp_thresh: 0,
                    last_qscale_diff: 0,
                    deblocking_filter: 0,
                    slice_alpha_c0_offset: 0,
                    slice_beta_offset: 0,
                    pwt: H264PredWeightTable {
                        use_weight: 0,
                        use_weight_chroma: 0,
                        luma_log2_weight_denom: 0,
                        chroma_log2_weight_denom: 0,
                        luma_weight_flag: [0; 2],
                        chroma_weight_flag: [0; 2],
                        luma_weight: [[[0; 2]; 2]; 48],
                        chroma_weight: [[[[0; 2]; 2]; 2]; 48],
                        implicit_weight: [[[0; 2]; 48]; 48],
                    },
                    prev_mb_skipped: 0,
                    next_mb_skipped: 0,
                    chroma_pred_mode: 0,
                    intra16x16_pred_mode: 0,
                    intra4x4_pred_mode_cache: [0; 40],
                    intra4x4_pred_mode: ::core::ptr::null_mut::<int8_t>(),
                    topleft_mb_xy: 0,
                    top_mb_xy: 0,
                    topright_mb_xy: 0,
                    left_mb_xy: [0; 2],
                    topleft_type: 0,
                    top_type: 0,
                    topright_type: 0,
                    left_type: [0; 2],
                    left_block: ::core::ptr::null::<uint8_t>(),
                    topleft_partition: 0,
                    topleft_samples_available: 0,
                    top_samples_available: 0,
                    topright_samples_available: 0,
                    left_samples_available: 0,
                    linesize: 0,
                    uvlinesize: 0,
                    mb_linesize: 0,
                    mb_uvlinesize: 0,
                    mb_x: 0,
                    mb_y: 0,
                    mb_xy: 0,
                    resync_mb_x: 0,
                    resync_mb_y: 0,
                    first_mb_addr: 0,
                    next_slice_idx: 0,
                    mb_skip_run: 0,
                    is_complex: 0,
                    picture_structure: 0,
                    mb_field_decoding_flag: 0,
                    mb_mbaff: 0,
                    redundant_pic_count: 0,
                    neighbor_transform_size: 0,
                    direct_spatial_mv_pred: 0,
                    col_parity: 0,
                    col_fieldoff: 0,
                    cbp: 0,
                    top_cbp: 0,
                    left_cbp: 0,
                    dist_scale_factor: [0; 32],
                    dist_scale_factor_field: [[0; 32]; 2],
                    map_col_to_list0: [[0; 48]; 2],
                    map_col_to_list0_field: [[[0; 48]; 2]; 2],
                    ref_count: [0; 2],
                    list_count: 0,
                    ref_list: [[H264Ref {
                        data: [::core::ptr::null_mut::<uint8_t>(); 3],
                        linesize: [0; 3],
                        reference: 0,
                        poc: 0,
                        pic_id: 0,
                        parent: ::core::ptr::null::<H264Picture>(),
                    }; 48]; 2],
                    ref_modifications: [[C2RustUnnamed_2 { op: 0, val: 0 }; 32]; 2],
                    nb_ref_modifications: [0; 2],
                    pps_id: 0,
                    intra_pcm_ptr: ::core::ptr::null::<uint8_t>(),
                    bipred_scratchpad: ::core::ptr::null_mut::<uint8_t>(),
                    edge_emu_buffer: ::core::ptr::null_mut::<uint8_t>(),
                    top_borders: [::core::ptr::null_mut::<[uint8_t; 96]>(); 2],
                    bipred_scratchpad_allocated: 0,
                    edge_emu_buffer_allocated: 0,
                    top_borders_allocated: [0; 2],
                    non_zero_count_cache: [0; 120],
                    mv_cache: [[[0; 2]; 40]; 2],
                    ref_cache: [[0; 40]; 2],
                    mvd_cache: [[[0; 2]; 40]; 2],
                    direct_cache: [0; 40],
                    sub_mb_type: [0; 4],
                    mb: [0; 1536],
                    mb_luma_dc: [[0; 32]; 3],
                    mb_padding: [0; 512],
                    mvd_table: [::core::ptr::null_mut::<[uint8_t; 2]>(); 2],
                    cabac: CABACContext {
                        low: 0,
                        range: 0,
                        bytestream_start: ::core::ptr::null::<uint8_t>(),
                        bytestream: ::core::ptr::null::<uint8_t>(),
                        bytestream_end: ::core::ptr::null::<uint8_t>(),
                    },
                    cabac_state: [0; 1024],
                    cabac_init_idc: 0,
                    mmco: [MMCO {
                        opcode: MMCO_END,
                        short_pic_num: 0,
                        long_arg: 0,
                    }; 67],
                    nb_mmco: 0,
                    explicit_ref_marking: 0,
                    frame_num: 0,
                    idr_pic_id: 0,
                    poc_lsb: 0,
                    delta_poc_bottom: 0,
                    delta_poc: [0; 2],
                    curr_pic_num: 0,
                    max_pic_num: 0,
                };
                ret = ff_h264_execute_decode_slices(h);
                if ret < 0 as ::core::ffi::c_int
                    && (*(*h).avctx).err_recognition
                        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
                {
                    return ret;
                }
                memcpy(
                    &raw mut tmp_ctx as *mut ::core::ffi::c_void,
                    (*h).slice_ctx as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<H264SliceContext>() as size_t,
                );
                memcpy(
                    (*h).slice_ctx as *mut ::core::ffi::c_void,
                    sl as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<H264SliceContext>() as size_t,
                );
                memcpy(
                    sl as *mut ::core::ffi::c_void,
                    &raw mut tmp_ctx as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<H264SliceContext>() as size_t,
                );
                sl = (*h).slice_ctx;
            }
            if !(*h).cur_pic_ptr.is_null()
                && (*h).picture_structure != 3 as ::core::ffi::c_int
                && (*h).first_field != 0
            {
                ret = ff_h264_field_end(h, (*h).slice_ctx, 1 as ::core::ffi::c_int);
                if ret < 0 as ::core::ffi::c_int {
                    return ret;
                }
            } else if !(*h).cur_pic_ptr.is_null()
                && !((*h).picture_structure != 3 as ::core::ffi::c_int)
                && (*h).first_field == 0
                && (*h).nal_unit_type == H264_NAL_IDR_SLICE as ::core::ffi::c_int
            {
                av_log(
                    (*h).avctx as *mut ::core::ffi::c_void,
                    24 as ::core::ffi::c_int,
                    b"Broken frame packetizing\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                ret = ff_h264_field_end(h, (*h).slice_ctx, 1 as ::core::ffi::c_int);
                ff_thread_report_progress(
                    &raw mut (*(*h).cur_pic_ptr).tf,
                    2147483647 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                );
                ff_thread_report_progress(
                    &raw mut (*(*h).cur_pic_ptr).tf,
                    2147483647 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
                (*h).cur_pic_ptr = ::core::ptr::null_mut::<H264Picture>();
                if ret < 0 as ::core::ffi::c_int {
                    return ret;
                }
            } else {
                return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                    | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int)
            }
        }
        if (*h).first_field == 0 {
            if !(*h).cur_pic_ptr.is_null() && (*h).droppable == 0 {
                ff_thread_report_progress(
                    &raw mut (*(*h).cur_pic_ptr).tf,
                    2147483647 as ::core::ffi::c_int,
                    ((*h).picture_structure == 2 as ::core::ffi::c_int)
                        as ::core::ffi::c_int,
                );
            }
            (*h).cur_pic_ptr = ::core::ptr::null_mut::<H264Picture>();
        }
    }
    if (*h).current_slice == 0 {
        if !(sl == (*h).slice_ctx) {
            av_log(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                0 as ::core::ffi::c_int,
                b"Assertion %s failed at %s:%d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"sl == h->slice_ctx\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/libavcodec/h264_slice.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                2137 as ::core::ffi::c_int,
            );
            abort();
        }
    }
    if (*h).current_slice == 0 as ::core::ffi::c_int && (*h).first_field == 0 {
        if (*(*h).avctx).skip_frame as ::core::ffi::c_int
            >= AVDISCARD_NONREF as ::core::ffi::c_int && (*h).nal_ref_idc == 0
            || (*(*h).avctx).skip_frame as ::core::ffi::c_int
                >= AVDISCARD_BIDIR as ::core::ffi::c_int
                && (*sl).slice_type_nos == AV_PICTURE_TYPE_B as ::core::ffi::c_int
            || (*(*h).avctx).skip_frame as ::core::ffi::c_int
                >= AVDISCARD_NONINTRA as ::core::ffi::c_int
                && (*sl).slice_type_nos != AV_PICTURE_TYPE_I as ::core::ffi::c_int
            || (*(*h).avctx).skip_frame as ::core::ffi::c_int
                >= AVDISCARD_NONKEY as ::core::ffi::c_int
                && (*h).nal_unit_type != H264_NAL_IDR_SLICE as ::core::ffi::c_int
                && (*h).sei.recovery_point.recovery_frame_cnt < 0 as ::core::ffi::c_int
            || (*(*h).avctx).skip_frame as ::core::ffi::c_int
                >= AVDISCARD_ALL as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    if first_slice == 0 {
        let mut pps: *const PPS = (*h).ps.pps_list[(*sl).pps_id as usize];
        if (*(*h).ps.pps).sps_id != (*pps).sps_id
            || (*(*h).ps.pps).transform_8x8_mode != (*pps).transform_8x8_mode
        {
            av_log(
                (*h).avctx as *mut ::core::ffi::c_void,
                16 as ::core::ffi::c_int,
                b"PPS changed between slices\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        if (*h).ps.sps != (*pps).sps {
            av_log(
                (*h).avctx as *mut ::core::ffi::c_void,
                16 as ::core::ffi::c_int,
                b"SPS changed in the middle of the frame\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
    }
    if (*h).current_slice == 0 as ::core::ffi::c_int {
        ret = h264_field_start(h, sl, nal, first_slice);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
    } else if (*h).picture_structure != (*sl).picture_structure
        || (*h).droppable
            != ((*nal).ref_idc == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
    {
        av_log(
            (*h).avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"Changing field mode (%d -> %d) between slices is not allowed\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            (*h).picture_structure,
            (*sl).picture_structure,
        );
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    } else if (*h).cur_pic_ptr.is_null() {
        av_log(
            (*h).avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"unset cur_pic_ptr on slice %d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            (*h).current_slice + 1 as ::core::ffi::c_int,
        );
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    ret = h264_slice_init(h, sl, nal);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    (*h).nb_slice_ctx_queued += 1;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_h264_get_slice_type(
    mut sl: *const H264SliceContext,
) -> ::core::ffi::c_int {
    match (*sl).slice_type {
        2 => return 0 as ::core::ffi::c_int,
        3 => return 1 as ::core::ffi::c_int,
        1 => return 2 as ::core::ffi::c_int,
        6 => return 3 as ::core::ffi::c_int,
        5 => return 4 as ::core::ffi::c_int,
        _ => {
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
    };
}
#[inline(always)]
unsafe extern "C" fn fill_filter_caches_inter(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut mb_type: ::core::ffi::c_int,
    mut top_xy: ::core::ffi::c_int,
    mut left_xy: *const ::core::ffi::c_int,
    mut top_type: ::core::ffi::c_int,
    mut left_type: *const ::core::ffi::c_int,
    mut mb_xy: ::core::ffi::c_int,
    mut list: ::core::ffi::c_int,
) {
    let mut b_stride: ::core::ffi::c_int = (*h).b_stride;
    let mut mv_dst: *mut [int16_t; 2] = (&raw mut *(&raw mut (*sl).mv_cache
        as *mut [[int16_t; 2]; 40])
        .offset(list as isize) as *mut [int16_t; 2])
        .offset(
            *(&raw const scan8 as *const uint8_t)
                .offset(0 as ::core::ffi::c_int as isize) as isize,
        ) as *mut [int16_t; 2];
    let mut ref_cache: *mut int8_t = (&raw mut *(&raw mut (*sl).ref_cache
        as *mut [int8_t; 40])
        .offset(list as isize) as *mut int8_t)
        .offset(
            *(&raw const scan8 as *const uint8_t)
                .offset(0 as ::core::ffi::c_int as isize) as isize,
        ) as *mut int8_t;
    if mb_type
        & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) != 0
        || mb_type & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int != 0
    {
        if top_type
            & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int * list != 0
        {
            let b_xy: ::core::ffi::c_int = (*(*h).mb2b_xy.offset(top_xy as isize))
                .wrapping_add((3 as ::core::ffi::c_int * b_stride) as uint32_t)
                as ::core::ffi::c_int;
            let b8_xy: ::core::ffi::c_int = 4 as ::core::ffi::c_int * top_xy
                + 2 as ::core::ffi::c_int;
            let mut ref2frm: *const ::core::ffi::c_int = (&raw const *(&raw const *(&raw const (*h)
                .ref2frm as *const [[::core::ffi::c_int; 64]; 2])
                .offset(
                    (*(*h).slice_table.offset(top_xy as isize) as ::core::ffi::c_int
                        & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
                ) as *const [::core::ffi::c_int; 64])
                .offset(list as isize) as *const ::core::ffi::c_int)
                .offset(
                    (if (*sl).mb_mbaff != 0 {
                        20 as ::core::ffi::c_int
                    } else {
                        2 as ::core::ffi::c_int
                    }) as isize,
                ) as *const ::core::ffi::c_int;
            (*(mv_dst
                .offset(-((1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize))
                as *mut av_alias64))
                .u64_0 = (*(&raw mut *(*(&raw const (*h).cur_pic.motion_val
                as *const *mut [int16_t; 2])
                .offset(list as isize))
                .offset((b_xy + 0 as ::core::ffi::c_int) as isize) as *mut int16_t
                as *const av_alias64))
                .u64_0;
            (*((mv_dst
                .offset(-((1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize))
                as *mut ::core::ffi::c_char)
                .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64))
                .u64_0 = (*((&raw mut *(*(&raw const (*h).cur_pic.motion_val
                as *const *mut [int16_t; 2])
                .offset(list as isize))
                .offset((b_xy + 0 as ::core::ffi::c_int) as isize) as *mut int16_t
                as *mut ::core::ffi::c_char)
                .offset(8 as ::core::ffi::c_int as isize) as *const av_alias64))
                .u64_0;
            let ref mut fresh42 = *ref_cache
                .offset(
                    (1 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
                );
            *fresh42 = *ref2frm
                .offset(
                    *(*h)
                        .cur_pic
                        .ref_index[list as usize]
                        .offset((b8_xy + 0 as ::core::ffi::c_int) as isize) as isize,
                ) as int8_t;
            *ref_cache
                .offset(
                    (0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
                ) = *fresh42;
            let ref mut fresh43 = *ref_cache
                .offset(
                    (3 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
                );
            *fresh43 = *ref2frm
                .offset(
                    *(*h)
                        .cur_pic
                        .ref_index[list as usize]
                        .offset((b8_xy + 1 as ::core::ffi::c_int) as isize) as isize,
                ) as int8_t;
            *ref_cache
                .offset(
                    (2 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
                ) = *fresh43;
        } else {
            (*(mv_dst
                .offset(-((1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize))
                as *mut av_alias64))
                .u64_0 = 0 as uint64_t;
            (*((mv_dst
                .offset(-((1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize))
                as *mut ::core::ffi::c_char)
                .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64))
                .u64_0 = 0 as uint64_t;
            (*(ref_cache
                .offset(
                    (0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
                ) as *mut int8_t as *mut av_alias32))
                .u32_0 = ((-(1 as ::core::ffi::c_int) & 0xff as ::core::ffi::c_int)
                as ::core::ffi::c_uint)
                .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
        }
        if (mb_type ^ *left_type.offset(0 as ::core::ffi::c_int as isize))
            & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int == 0
        {
            if *left_type.offset(0 as ::core::ffi::c_int as isize)
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int * list != 0
            {
                let b_xy_0: ::core::ffi::c_int = (*(*h)
                    .mb2b_xy
                    .offset(*left_xy.offset(0 as ::core::ffi::c_int as isize) as isize))
                    .wrapping_add(3 as uint32_t) as ::core::ffi::c_int;
                let b8_xy_0: ::core::ffi::c_int = 4 as ::core::ffi::c_int
                    * *left_xy.offset(0 as ::core::ffi::c_int as isize)
                    + 1 as ::core::ffi::c_int;
                let mut ref2frm_0: *const ::core::ffi::c_int = (&raw const *(&raw const *(&raw const (*h)
                    .ref2frm as *const [[::core::ffi::c_int; 64]; 2])
                    .offset(
                        (*(*h)
                            .slice_table
                            .offset(
                                *left_xy.offset(0 as ::core::ffi::c_int as isize) as isize,
                            ) as ::core::ffi::c_int
                            & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                            as isize,
                    ) as *const [::core::ffi::c_int; 64])
                    .offset(list as isize) as *const ::core::ffi::c_int)
                    .offset(
                        (if (*sl).mb_mbaff != 0 {
                            20 as ::core::ffi::c_int
                        } else {
                            2 as ::core::ffi::c_int
                        }) as isize,
                    ) as *const ::core::ffi::c_int;
                (*(mv_dst
                    .offset(-(1 as ::core::ffi::c_int as isize))
                    .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias32))
                    .u32_0 = (*(&raw mut *(*(&raw const (*h).cur_pic.motion_val
                    as *const *mut [int16_t; 2])
                    .offset(list as isize))
                    .offset((b_xy_0 + b_stride * 0 as ::core::ffi::c_int) as isize)
                    as *mut int16_t as *const av_alias32))
                    .u32_0;
                (*(mv_dst
                    .offset(-(1 as ::core::ffi::c_int as isize))
                    .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias32))
                    .u32_0 = (*(&raw mut *(*(&raw const (*h).cur_pic.motion_val
                    as *const *mut [int16_t; 2])
                    .offset(list as isize))
                    .offset((b_xy_0 + b_stride * 1 as ::core::ffi::c_int) as isize)
                    as *mut int16_t as *const av_alias32))
                    .u32_0;
                (*(mv_dst
                    .offset(-(1 as ::core::ffi::c_int as isize))
                    .offset(16 as ::core::ffi::c_int as isize) as *mut av_alias32))
                    .u32_0 = (*(&raw mut *(*(&raw const (*h).cur_pic.motion_val
                    as *const *mut [int16_t; 2])
                    .offset(list as isize))
                    .offset((b_xy_0 + b_stride * 2 as ::core::ffi::c_int) as isize)
                    as *mut int16_t as *const av_alias32))
                    .u32_0;
                (*(mv_dst
                    .offset(-(1 as ::core::ffi::c_int as isize))
                    .offset(24 as ::core::ffi::c_int as isize) as *mut av_alias32))
                    .u32_0 = (*(&raw mut *(*(&raw const (*h).cur_pic.motion_val
                    as *const *mut [int16_t; 2])
                    .offset(list as isize))
                    .offset((b_xy_0 + b_stride * 3 as ::core::ffi::c_int) as isize)
                    as *mut int16_t as *const av_alias32))
                    .u32_0;
                let ref mut fresh44 = *ref_cache
                    .offset(
                        (-(1 as ::core::ffi::c_int) + 8 as ::core::ffi::c_int) as isize,
                    );
                *fresh44 = *ref2frm_0
                    .offset(
                        *(*h)
                            .cur_pic
                            .ref_index[list as usize]
                            .offset(
                                (b8_xy_0
                                    + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int)
                                    as isize,
                            ) as isize,
                    ) as int8_t;
                *ref_cache
                    .offset(
                        (-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int) as isize,
                    ) = *fresh44;
                let ref mut fresh45 = *ref_cache
                    .offset(
                        (-(1 as ::core::ffi::c_int) + 24 as ::core::ffi::c_int) as isize,
                    );
                *fresh45 = *ref2frm_0
                    .offset(
                        *(*h)
                            .cur_pic
                            .ref_index[list as usize]
                            .offset(
                                (b8_xy_0
                                    + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                                    as isize,
                            ) as isize,
                    ) as int8_t;
                *ref_cache
                    .offset(
                        (-(1 as ::core::ffi::c_int) + 16 as ::core::ffi::c_int) as isize,
                    ) = *fresh45;
            } else {
                (*(mv_dst
                    .offset(-(1 as ::core::ffi::c_int as isize))
                    .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias32))
                    .u32_0 = 0 as uint32_t;
                (*(mv_dst
                    .offset(-(1 as ::core::ffi::c_int as isize))
                    .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias32))
                    .u32_0 = 0 as uint32_t;
                (*(mv_dst
                    .offset(-(1 as ::core::ffi::c_int as isize))
                    .offset(16 as ::core::ffi::c_int as isize) as *mut av_alias32))
                    .u32_0 = 0 as uint32_t;
                (*(mv_dst
                    .offset(-(1 as ::core::ffi::c_int as isize))
                    .offset(24 as ::core::ffi::c_int as isize) as *mut av_alias32))
                    .u32_0 = 0 as uint32_t;
                let ref mut fresh46 = *ref_cache
                    .offset(
                        (-(1 as ::core::ffi::c_int) + 24 as ::core::ffi::c_int) as isize,
                    );
                *fresh46 = -(1 as ::core::ffi::c_int) as int8_t;
                let ref mut fresh47 = *ref_cache
                    .offset(
                        (-(1 as ::core::ffi::c_int) + 16 as ::core::ffi::c_int) as isize,
                    );
                *fresh47 = *fresh46;
                let ref mut fresh48 = *ref_cache
                    .offset(
                        (-(1 as ::core::ffi::c_int) + 8 as ::core::ffi::c_int) as isize,
                    );
                *fresh48 = *fresh47;
                *ref_cache
                    .offset(
                        (-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int) as isize,
                    ) = *fresh48;
            }
        }
    }
    if mb_type
        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int * list == 0
    {
        fill_rectangle(
            mv_dst as *mut ::core::ffi::c_void,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            pack16to32(0 as ::core::ffi::c_uint, 0 as ::core::ffi::c_uint),
            4 as ::core::ffi::c_int,
        );
        (*(ref_cache.offset((0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
            as *mut int8_t as *mut av_alias32))
            .u32_0 = ((-(1 as ::core::ffi::c_int) & 0xff as ::core::ffi::c_int)
            as ::core::ffi::c_uint)
            .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
        (*(ref_cache.offset((1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
            as *mut int8_t as *mut av_alias32))
            .u32_0 = ((-(1 as ::core::ffi::c_int) & 0xff as ::core::ffi::c_int)
            as ::core::ffi::c_uint)
            .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
        (*(ref_cache.offset((2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
            as *mut int8_t as *mut av_alias32))
            .u32_0 = ((-(1 as ::core::ffi::c_int) & 0xff as ::core::ffi::c_int)
            as ::core::ffi::c_uint)
            .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
        (*(ref_cache.offset((3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
            as *mut int8_t as *mut av_alias32))
            .u32_0 = ((-(1 as ::core::ffi::c_int) & 0xff as ::core::ffi::c_int)
            as ::core::ffi::c_uint)
            .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
        return;
    }
    let mut ref_0: *const int8_t = (*(&raw const (*h).cur_pic.ref_index
        as *const *mut int8_t)
        .offset(list as isize))
        .offset((4 as ::core::ffi::c_int * mb_xy) as isize) as *mut int8_t;
    let mut ref2frm_1: *const ::core::ffi::c_int = (&raw const *(&raw const *(&raw const (*h)
        .ref2frm as *const [[::core::ffi::c_int; 64]; 2])
        .offset(
            ((*sl).slice_num & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as isize,
        ) as *const [::core::ffi::c_int; 64])
        .offset(list as isize) as *const ::core::ffi::c_int)
        .offset(
            (if (*sl).mb_mbaff != 0 {
                20 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int
            }) as isize,
        ) as *const ::core::ffi::c_int;
    let mut ref01: uint32_t = (pack16to32(
        *ref2frm_1.offset(*ref_0.offset(0 as ::core::ffi::c_int as isize) as isize)
            as ::core::ffi::c_uint,
        *ref2frm_1.offset(*ref_0.offset(1 as ::core::ffi::c_int as isize) as isize)
            as ::core::ffi::c_uint,
    ) & 0xff00ff as ::core::ffi::c_int as uint32_t)
        .wrapping_mul(0x101 as uint32_t);
    let mut ref23: uint32_t = (pack16to32(
        *ref2frm_1.offset(*ref_0.offset(2 as ::core::ffi::c_int as isize) as isize)
            as ::core::ffi::c_uint,
        *ref2frm_1.offset(*ref_0.offset(3 as ::core::ffi::c_int as isize) as isize)
            as ::core::ffi::c_uint,
    ) & 0xff00ff as ::core::ffi::c_int as uint32_t)
        .wrapping_mul(0x101 as uint32_t);
    (*(ref_cache.offset((0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
        as *mut int8_t as *mut av_alias32))
        .u32_0 = ref01;
    (*(ref_cache.offset((1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
        as *mut int8_t as *mut av_alias32))
        .u32_0 = ref01;
    (*(ref_cache.offset((2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
        as *mut int8_t as *mut av_alias32))
        .u32_0 = ref23;
    (*(ref_cache.offset((3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
        as *mut int8_t as *mut av_alias32))
        .u32_0 = ref23;
    let mut mv_src: *mut [int16_t; 2] = (*(&raw const (*h).cur_pic.motion_val
        as *const *mut [int16_t; 2])
        .offset(list as isize))
        .offset(
            (4 as ::core::ffi::c_int * (*sl).mb_x
                + 4 as ::core::ffi::c_int * (*sl).mb_y * b_stride) as isize,
        ) as *mut [int16_t; 2];
    (*(mv_dst.offset((8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize)
        as *mut av_alias64))
        .u64_0 = (*(mv_src.offset((0 as ::core::ffi::c_int * b_stride) as isize)
        as *const av_alias64))
        .u64_0;
    (*((mv_dst.offset((8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize)
        as *mut ::core::ffi::c_char)
        .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64))
        .u64_0 = (*((mv_src.offset((0 as ::core::ffi::c_int * b_stride) as isize)
        as *mut ::core::ffi::c_char)
        .offset(8 as ::core::ffi::c_int as isize) as *const av_alias64))
        .u64_0;
    (*(mv_dst.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
        as *mut av_alias64))
        .u64_0 = (*(mv_src.offset((1 as ::core::ffi::c_int * b_stride) as isize)
        as *const av_alias64))
        .u64_0;
    (*((mv_dst.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
        as *mut ::core::ffi::c_char)
        .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64))
        .u64_0 = (*((mv_src.offset((1 as ::core::ffi::c_int * b_stride) as isize)
        as *mut ::core::ffi::c_char)
        .offset(8 as ::core::ffi::c_int as isize) as *const av_alias64))
        .u64_0;
    (*(mv_dst.offset((8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
        as *mut av_alias64))
        .u64_0 = (*(mv_src.offset((2 as ::core::ffi::c_int * b_stride) as isize)
        as *const av_alias64))
        .u64_0;
    (*((mv_dst.offset((8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
        as *mut ::core::ffi::c_char)
        .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64))
        .u64_0 = (*((mv_src.offset((2 as ::core::ffi::c_int * b_stride) as isize)
        as *mut ::core::ffi::c_char)
        .offset(8 as ::core::ffi::c_int as isize) as *const av_alias64))
        .u64_0;
    (*(mv_dst.offset((8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize)
        as *mut av_alias64))
        .u64_0 = (*(mv_src.offset((3 as ::core::ffi::c_int * b_stride) as isize)
        as *const av_alias64))
        .u64_0;
    (*((mv_dst.offset((8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize)
        as *mut ::core::ffi::c_char)
        .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64))
        .u64_0 = (*((mv_src.offset((3 as ::core::ffi::c_int * b_stride) as isize)
        as *mut ::core::ffi::c_char)
        .offset(8 as ::core::ffi::c_int as isize) as *const av_alias64))
        .u64_0;
}
unsafe extern "C" fn fill_filter_caches(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut mb_type: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mb_xy: ::core::ffi::c_int = (*sl).mb_xy;
    let mut top_xy: ::core::ffi::c_int = 0;
    let mut left_xy: [::core::ffi::c_int; 2] = [0; 2];
    let mut top_type: ::core::ffi::c_int = 0;
    let mut left_type: [::core::ffi::c_int; 2] = [0; 2];
    let mut nnz: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut nnz_cache: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    top_xy = mb_xy - ((*h).mb_stride << (*sl).mb_field_decoding_flag);
    left_xy[0 as ::core::ffi::c_int as usize] = mb_xy - 1 as ::core::ffi::c_int;
    left_xy[1 as ::core::ffi::c_int as usize] = left_xy[0 as ::core::ffi::c_int
        as usize];
    if (*h).mb_aff_frame != 0 {
        let left_mb_field_flag: ::core::ffi::c_int = (*(*h)
            .cur_pic
            .mb_type
            .offset((mb_xy - 1 as ::core::ffi::c_int) as isize)
            & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint32_t)
            as ::core::ffi::c_int;
        let curr_mb_field_flag: ::core::ffi::c_int = mb_type
            & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int;
        if (*sl).mb_y & 1 as ::core::ffi::c_int != 0 {
            if left_mb_field_flag != curr_mb_field_flag {
                left_xy[0 as ::core::ffi::c_int as usize] -= (*h).mb_stride;
            }
        } else {
            if curr_mb_field_flag != 0 {
                top_xy = (top_xy as uint32_t)
                    .wrapping_add(
                        (*h).mb_stride as uint32_t
                            & (*(*h).cur_pic.mb_type.offset(top_xy as isize)
                                >> 7 as ::core::ffi::c_int & 1 as uint32_t)
                                .wrapping_sub(1 as uint32_t),
                    ) as ::core::ffi::c_int as ::core::ffi::c_int;
            }
            if left_mb_field_flag != curr_mb_field_flag {
                left_xy[1 as ::core::ffi::c_int as usize] += (*h).mb_stride;
            }
        }
    }
    (*sl).top_mb_xy = top_xy;
    (*sl).left_mb_xy[0 as ::core::ffi::c_int as usize] = left_xy[0 as ::core::ffi::c_int
        as usize];
    (*sl).left_mb_xy[1 as ::core::ffi::c_int as usize] = left_xy[1 as ::core::ffi::c_int
        as usize];
    let mut qp_thresh: ::core::ffi::c_int = (*sl).qp_thresh;
    let mut qp: ::core::ffi::c_int = *(*h).cur_pic.qscale_table.offset(mb_xy as isize)
        as ::core::ffi::c_int;
    if qp <= qp_thresh
        && (left_xy[0 as ::core::ffi::c_int as usize] < 0 as ::core::ffi::c_int
            || qp
                + *(*h)
                    .cur_pic
                    .qscale_table
                    .offset(left_xy[0 as ::core::ffi::c_int as usize] as isize)
                    as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int <= qp_thresh)
        && (top_xy < 0 as ::core::ffi::c_int
            || qp
                + *(*h).cur_pic.qscale_table.offset(top_xy as isize)
                    as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int <= qp_thresh)
    {
        if (*h).mb_aff_frame == 0 {
            return 1 as ::core::ffi::c_int;
        }
        if (left_xy[0 as ::core::ffi::c_int as usize] < 0 as ::core::ffi::c_int
            || qp
                + *(*h)
                    .cur_pic
                    .qscale_table
                    .offset(left_xy[1 as ::core::ffi::c_int as usize] as isize)
                    as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int <= qp_thresh)
            && (top_xy < (*h).mb_stride
                || qp
                    + *(*h)
                        .cur_pic
                        .qscale_table
                        .offset((top_xy - (*h).mb_stride) as isize) as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int <= qp_thresh)
        {
            return 1 as ::core::ffi::c_int;
        }
    }
    top_type = *(*h).cur_pic.mb_type.offset(top_xy as isize) as ::core::ffi::c_int;
    left_type[0 as ::core::ffi::c_int as usize] = *(*h)
        .cur_pic
        .mb_type
        .offset(left_xy[0 as ::core::ffi::c_int as usize] as isize)
        as ::core::ffi::c_int;
    left_type[1 as ::core::ffi::c_int as usize] = *(*h)
        .cur_pic
        .mb_type
        .offset(left_xy[1 as ::core::ffi::c_int as usize] as isize)
        as ::core::ffi::c_int;
    if (*sl).deblocking_filter == 2 as ::core::ffi::c_int {
        if *(*h).slice_table.offset(top_xy as isize) as ::core::ffi::c_int
            != (*sl).slice_num
        {
            top_type = 0 as ::core::ffi::c_int;
        }
        if *(*h).slice_table.offset(left_xy[1 as ::core::ffi::c_int as usize] as isize)
            as ::core::ffi::c_int != (*sl).slice_num
        {
            left_type[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
            left_type[0 as ::core::ffi::c_int as usize] = left_type[1
                as ::core::ffi::c_int as usize];
        }
    } else {
        if *(*h).slice_table.offset(top_xy as isize) as ::core::ffi::c_int
            == 0xffff as ::core::ffi::c_int
        {
            top_type = 0 as ::core::ffi::c_int;
        }
        if *(*h).slice_table.offset(left_xy[1 as ::core::ffi::c_int as usize] as isize)
            as ::core::ffi::c_int == 0xffff as ::core::ffi::c_int
        {
            left_type[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
            left_type[0 as ::core::ffi::c_int as usize] = left_type[1
                as ::core::ffi::c_int as usize];
        }
    }
    (*sl).top_type = top_type;
    (*sl).left_type[0 as ::core::ffi::c_int as usize] = left_type[0 as ::core::ffi::c_int
        as usize];
    (*sl).left_type[1 as ::core::ffi::c_int as usize] = left_type[1 as ::core::ffi::c_int
        as usize];
    if mb_type & 7 as ::core::ffi::c_int != 0 {
        return 0 as ::core::ffi::c_int;
    }
    fill_filter_caches_inter(
        h,
        sl,
        mb_type,
        top_xy,
        &raw mut left_xy as *mut ::core::ffi::c_int as *const ::core::ffi::c_int,
        top_type,
        &raw mut left_type as *mut ::core::ffi::c_int as *const ::core::ffi::c_int,
        mb_xy,
        0 as ::core::ffi::c_int,
    );
    if (*sl).list_count == 2 as ::core::ffi::c_uint {
        fill_filter_caches_inter(
            h,
            sl,
            mb_type,
            top_xy,
            &raw mut left_xy as *mut ::core::ffi::c_int as *const ::core::ffi::c_int,
            top_type,
            &raw mut left_type as *mut ::core::ffi::c_int as *const ::core::ffi::c_int,
            mb_xy,
            1 as ::core::ffi::c_int,
        );
    }
    nnz = &raw mut *(*h).non_zero_count.offset(mb_xy as isize) as *mut uint8_t;
    nnz_cache = &raw mut (*sl).non_zero_count_cache as *mut uint8_t;
    (*(nnz_cache
        .offset(
            (4 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                as isize,
        ) as *mut uint8_t as *mut av_alias32))
        .u32_0 = (*(nnz.offset(0 as ::core::ffi::c_int as isize) as *const uint8_t
        as *const av_alias32))
        .u32_0;
    (*(nnz_cache
        .offset(
            (4 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                as isize,
        ) as *mut uint8_t as *mut av_alias32))
        .u32_0 = (*(nnz.offset(4 as ::core::ffi::c_int as isize) as *const uint8_t
        as *const av_alias32))
        .u32_0;
    (*(nnz_cache
        .offset(
            (4 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as isize,
        ) as *mut uint8_t as *mut av_alias32))
        .u32_0 = (*(nnz.offset(8 as ::core::ffi::c_int as isize) as *const uint8_t
        as *const av_alias32))
        .u32_0;
    (*(nnz_cache
        .offset(
            (4 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as isize,
        ) as *mut uint8_t as *mut av_alias32))
        .u32_0 = (*(nnz.offset(12 as ::core::ffi::c_int as isize) as *const uint8_t
        as *const av_alias32))
        .u32_0;
    (*sl).cbp = *(*h).cbp_table.offset(mb_xy as isize) as ::core::ffi::c_int;
    if top_type != 0 {
        nnz = &raw mut *(*h).non_zero_count.offset(top_xy as isize) as *mut uint8_t;
        (*(nnz_cache
            .offset(
                (4 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize,
            ) as *mut uint8_t as *mut av_alias32))
            .u32_0 = (*(nnz
            .offset((3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
            as *const uint8_t as *const av_alias32))
            .u32_0;
    }
    if left_type[0 as ::core::ffi::c_int as usize] != 0 {
        nnz = &raw mut *(*h)
            .non_zero_count
            .offset(
                *(&raw mut left_xy as *mut ::core::ffi::c_int)
                    .offset(0 as ::core::ffi::c_int as isize) as isize,
            ) as *mut uint8_t;
        *nnz_cache
            .offset(
                (3 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize,
            ) = *nnz
            .offset(
                (3 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
            );
        *nnz_cache
            .offset(
                (3 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
            ) = *nnz
            .offset(
                (3 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
            );
        *nnz_cache
            .offset(
                (3 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
            ) = *nnz
            .offset(
                (3 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
            );
        *nnz_cache
            .offset(
                (3 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
            ) = *nnz
            .offset(
                (3 as ::core::ffi::c_int
                    + 3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
            );
    }
    if (*(*h).ps.pps).cabac == 0 && (*(*h).ps.pps).transform_8x8_mode != 0 {
        if top_type & 0x1000000 as ::core::ffi::c_int != 0 {
            let ref mut fresh26 = *nnz_cache
                .offset(
                    (5 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize,
                );
            *fresh26 = ((*(*h).cbp_table.offset(top_xy as isize) as ::core::ffi::c_int
                & 0x4000 as ::core::ffi::c_int) >> 12 as ::core::ffi::c_int) as uint8_t;
            *nnz_cache
                .offset(
                    (4 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize,
                ) = *fresh26;
            let ref mut fresh27 = *nnz_cache
                .offset(
                    (7 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize,
                );
            *fresh27 = ((*(*h).cbp_table.offset(top_xy as isize) as ::core::ffi::c_int
                & 0x8000 as ::core::ffi::c_int) >> 12 as ::core::ffi::c_int) as uint8_t;
            *nnz_cache
                .offset(
                    (6 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize,
                ) = *fresh27;
        }
        if left_type[0 as ::core::ffi::c_int as usize] & 0x1000000 as ::core::ffi::c_int
            != 0
        {
            let ref mut fresh28 = *nnz_cache
                .offset(
                    (3 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                );
            *fresh28 = ((*(*h)
                .cbp_table
                .offset(left_xy[0 as ::core::ffi::c_int as usize] as isize)
                as ::core::ffi::c_int & 0x2000 as ::core::ffi::c_int)
                >> 12 as ::core::ffi::c_int) as uint8_t;
            *nnz_cache
                .offset(
                    (3 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize,
                ) = *fresh28;
        }
        if left_type[1 as ::core::ffi::c_int as usize] & 0x1000000 as ::core::ffi::c_int
            != 0
        {
            let ref mut fresh29 = *nnz_cache
                .offset(
                    (3 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                );
            *fresh29 = ((*(*h)
                .cbp_table
                .offset(left_xy[1 as ::core::ffi::c_int as usize] as isize)
                as ::core::ffi::c_int & 0x8000 as ::core::ffi::c_int)
                >> 12 as ::core::ffi::c_int) as uint8_t;
            *nnz_cache
                .offset(
                    (3 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                ) = *fresh29;
        }
        if mb_type & 0x1000000 as ::core::ffi::c_int != 0 {
            let ref mut fresh30 = *nnz_cache
                .offset(scan8[3 as ::core::ffi::c_int as usize] as isize);
            *fresh30 = (((*sl).cbp & 0x1000 as ::core::ffi::c_int)
                >> 12 as ::core::ffi::c_int) as uint8_t;
            let ref mut fresh31 = *nnz_cache
                .offset(scan8[2 as ::core::ffi::c_int as usize] as isize);
            *fresh31 = *fresh30;
            let ref mut fresh32 = *nnz_cache
                .offset(scan8[1 as ::core::ffi::c_int as usize] as isize);
            *fresh32 = *fresh31;
            *nnz_cache.offset(scan8[0 as ::core::ffi::c_int as usize] as isize) = *fresh32;
            let ref mut fresh33 = *nnz_cache
                .offset(
                    scan8[(3 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as usize]
                        as isize,
                );
            *fresh33 = (((*sl).cbp & 0x2000 as ::core::ffi::c_int)
                >> 12 as ::core::ffi::c_int) as uint8_t;
            let ref mut fresh34 = *nnz_cache
                .offset(
                    scan8[(2 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as usize]
                        as isize,
                );
            *fresh34 = *fresh33;
            let ref mut fresh35 = *nnz_cache
                .offset(
                    scan8[(1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as usize]
                        as isize,
                );
            *fresh35 = *fresh34;
            *nnz_cache
                .offset(
                    scan8[(0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as usize]
                        as isize,
                ) = *fresh35;
            let ref mut fresh36 = *nnz_cache
                .offset(
                    scan8[(3 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as usize]
                        as isize,
                );
            *fresh36 = (((*sl).cbp & 0x4000 as ::core::ffi::c_int)
                >> 12 as ::core::ffi::c_int) as uint8_t;
            let ref mut fresh37 = *nnz_cache
                .offset(
                    scan8[(2 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as usize]
                        as isize,
                );
            *fresh37 = *fresh36;
            let ref mut fresh38 = *nnz_cache
                .offset(
                    scan8[(1 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as usize]
                        as isize,
                );
            *fresh38 = *fresh37;
            *nnz_cache
                .offset(
                    scan8[(0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as usize]
                        as isize,
                ) = *fresh38;
            let ref mut fresh39 = *nnz_cache
                .offset(
                    scan8[(3 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as usize]
                        as isize,
                );
            *fresh39 = (((*sl).cbp & 0x8000 as ::core::ffi::c_int)
                >> 12 as ::core::ffi::c_int) as uint8_t;
            let ref mut fresh40 = *nnz_cache
                .offset(
                    scan8[(2 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as usize]
                        as isize,
                );
            *fresh40 = *fresh39;
            let ref mut fresh41 = *nnz_cache
                .offset(
                    scan8[(1 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as usize]
                        as isize,
                );
            *fresh41 = *fresh40;
            *nnz_cache
                .offset(
                    scan8[(0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as usize]
                        as isize,
                ) = *fresh41;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn loop_filter(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut start_x: ::core::ffi::c_int,
    mut end_x: ::core::ffi::c_int,
) {
    let mut dest_y: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut dest_cb: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut dest_cr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut linesize: ::core::ffi::c_int = 0;
    let mut uvlinesize: ::core::ffi::c_int = 0;
    let mut mb_x: ::core::ffi::c_int = 0;
    let mut mb_y: ::core::ffi::c_int = 0;
    let end_mb_y: ::core::ffi::c_int = (*sl).mb_y + (*h).mb_aff_frame;
    let old_slice_type: ::core::ffi::c_int = (*sl).slice_type;
    let pixel_shift: ::core::ffi::c_int = (*h).pixel_shift;
    let block_h: ::core::ffi::c_int = 16 as ::core::ffi::c_int >> (*h).chroma_y_shift;
    if (*h).postpone_filter != 0 {
        return;
    }
    if (*sl).deblocking_filter != 0 {
        mb_x = start_x;
        while mb_x < end_x {
            mb_y = end_mb_y - (*h).mb_aff_frame;
            while mb_y <= end_mb_y {
                let mut mb_xy: ::core::ffi::c_int = 0;
                let mut mb_type: ::core::ffi::c_int = 0;
                (*sl).mb_xy = mb_x + mb_y * (*h).mb_stride;
                mb_xy = (*sl).mb_xy;
                mb_type = *(*h).cur_pic.mb_type.offset(mb_xy as isize)
                    as ::core::ffi::c_int;
                if (*h).mb_aff_frame != 0 {
                    (*sl).mb_field_decoding_flag = (mb_type
                        & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0)
                        as ::core::ffi::c_int;
                    (*sl).mb_mbaff = (*sl).mb_field_decoding_flag;
                }
                (*sl).mb_x = mb_x;
                (*sl).mb_y = mb_y;
                dest_y = (*(*h).cur_pic.f)
                    .data[0 as ::core::ffi::c_int as usize]
                    .offset(
                        (((mb_x << pixel_shift) as ptrdiff_t
                            + mb_y as ptrdiff_t * (*sl).linesize) * 16 as ptrdiff_t)
                            as isize,
                    );
                dest_cb = (*(*h).cur_pic.f)
                    .data[1 as ::core::ffi::c_int as usize]
                    .offset(
                        ((mb_x << pixel_shift)
                            * ((8 as ::core::ffi::c_int)
                                << ((*(*h).ps.sps).chroma_format_idc
                                    == 3 as ::core::ffi::c_int) as ::core::ffi::c_int)) as isize,
                    )
                    .offset(
                        (mb_y as ptrdiff_t * (*sl).uvlinesize * block_h as ptrdiff_t)
                            as isize,
                    );
                dest_cr = (*(*h).cur_pic.f)
                    .data[2 as ::core::ffi::c_int as usize]
                    .offset(
                        ((mb_x << pixel_shift)
                            * ((8 as ::core::ffi::c_int)
                                << ((*(*h).ps.sps).chroma_format_idc
                                    == 3 as ::core::ffi::c_int) as ::core::ffi::c_int)) as isize,
                    )
                    .offset(
                        (mb_y as ptrdiff_t * (*sl).uvlinesize * block_h as ptrdiff_t)
                            as isize,
                    );
                if (*sl).mb_field_decoding_flag != 0 {
                    (*sl).mb_linesize = (*sl).linesize * 2 as ptrdiff_t;
                    linesize = (*sl).mb_linesize as ::core::ffi::c_int;
                    (*sl).mb_uvlinesize = (*sl).uvlinesize * 2 as ptrdiff_t;
                    uvlinesize = (*sl).mb_uvlinesize as ::core::ffi::c_int;
                    if mb_y & 1 as ::core::ffi::c_int != 0 {
                        dest_y = dest_y
                            .offset(-(((*sl).linesize * 15 as ptrdiff_t) as isize));
                        dest_cb = dest_cb
                            .offset(
                                -(((*sl).uvlinesize
                                    * (block_h - 1 as ::core::ffi::c_int) as ptrdiff_t)
                                    as isize),
                            );
                        dest_cr = dest_cr
                            .offset(
                                -(((*sl).uvlinesize
                                    * (block_h - 1 as ::core::ffi::c_int) as ptrdiff_t)
                                    as isize),
                            );
                    }
                } else {
                    (*sl).mb_linesize = (*sl).linesize;
                    linesize = (*sl).mb_linesize as ::core::ffi::c_int;
                    (*sl).mb_uvlinesize = (*sl).uvlinesize;
                    uvlinesize = (*sl).mb_uvlinesize as ::core::ffi::c_int;
                }
                backup_mb_border(
                    h,
                    sl,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    linesize,
                    uvlinesize,
                    0 as ::core::ffi::c_int,
                );
                if !(fill_filter_caches(h, sl, mb_type) != 0) {
                    (*sl).chroma_qp[0 as ::core::ffi::c_int as usize] = get_chroma_qp(
                        (*h).ps.pps,
                        0 as ::core::ffi::c_int,
                        *(*h).cur_pic.qscale_table.offset(mb_xy as isize)
                            as ::core::ffi::c_int,
                    );
                    (*sl).chroma_qp[1 as ::core::ffi::c_int as usize] = get_chroma_qp(
                        (*h).ps.pps,
                        1 as ::core::ffi::c_int,
                        *(*h).cur_pic.qscale_table.offset(mb_xy as isize)
                            as ::core::ffi::c_int,
                    );
                    if (*h).mb_aff_frame != 0 {
                        ff_h264_filter_mb(
                            h,
                            sl,
                            mb_x,
                            mb_y,
                            dest_y,
                            dest_cb,
                            dest_cr,
                            linesize as ::core::ffi::c_uint,
                            uvlinesize as ::core::ffi::c_uint,
                        );
                    } else {
                        ff_h264_filter_mb_fast(
                            h,
                            sl,
                            mb_x,
                            mb_y,
                            dest_y,
                            dest_cb,
                            dest_cr,
                            linesize as ::core::ffi::c_uint,
                            uvlinesize as ::core::ffi::c_uint,
                        );
                    }
                }
                mb_y += 1;
            }
            mb_x += 1;
        }
    }
    (*sl).slice_type = old_slice_type;
    (*sl).mb_x = end_x;
    (*sl).mb_y = end_mb_y - (*h).mb_aff_frame;
    (*sl).chroma_qp[0 as ::core::ffi::c_int as usize] = get_chroma_qp(
        (*h).ps.pps,
        0 as ::core::ffi::c_int,
        (*sl).qscale,
    );
    (*sl).chroma_qp[1 as ::core::ffi::c_int as usize] = get_chroma_qp(
        (*h).ps.pps,
        1 as ::core::ffi::c_int,
        (*sl).qscale,
    );
}
unsafe extern "C" fn predict_field_decoding_flag(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
) {
    let mb_xy: ::core::ffi::c_int = (*sl).mb_x + (*sl).mb_y * (*h).mb_stride;
    let mut mb_type: ::core::ffi::c_int = (if *(*h)
        .slice_table
        .offset((mb_xy - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
        == (*sl).slice_num
    {
        *(*h).cur_pic.mb_type.offset((mb_xy - 1 as ::core::ffi::c_int) as isize)
    } else if *(*h).slice_table.offset((mb_xy - (*h).mb_stride) as isize)
        as ::core::ffi::c_int == (*sl).slice_num
    {
        *(*h).cur_pic.mb_type.offset((mb_xy - (*h).mb_stride) as isize)
    } else {
        0 as uint32_t
    }) as ::core::ffi::c_int;
    (*sl).mb_field_decoding_flag = if mb_type
        & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    (*sl).mb_mbaff = (*sl).mb_field_decoding_flag;
}
unsafe extern "C" fn decode_finish_row(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
) {
    let mut top: ::core::ffi::c_int = 16 as ::core::ffi::c_int
        * ((*sl).mb_y
            >> ((*h).picture_structure != 3 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
    let mut pic_height: ::core::ffi::c_int = 16 as ::core::ffi::c_int * (*h).mb_height
        >> ((*h).picture_structure != 3 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut height: ::core::ffi::c_int = (16 as ::core::ffi::c_int) << (*h).mb_aff_frame;
    let mut deblock_border: ::core::ffi::c_int = (16 as ::core::ffi::c_int
        + 4 as ::core::ffi::c_int) << (*h).mb_aff_frame;
    if (*sl).deblocking_filter != 0 {
        if top + height >= pic_height {
            height += deblock_border;
        }
        top -= deblock_border;
    }
    if top >= pic_height || top + height < 0 as ::core::ffi::c_int {
        return;
    }
    height = if height > pic_height - top { pic_height - top } else { height };
    if top < 0 as ::core::ffi::c_int {
        height = top + height;
        top = 0 as ::core::ffi::c_int;
    }
    ff_h264_draw_horiz_band(h, sl, top, height);
    if (*h).droppable != 0 || (*h).er.error_occurred != 0 {
        return;
    }
    ff_thread_report_progress(
        &raw mut (*(*h).cur_pic_ptr).tf,
        top + height - 1 as ::core::ffi::c_int,
        ((*h).picture_structure == 2 as ::core::ffi::c_int) as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn er_add_slice(
    mut sl: *mut H264SliceContext,
    mut startx: ::core::ffi::c_int,
    mut starty: ::core::ffi::c_int,
    mut endx: ::core::ffi::c_int,
    mut endy: ::core::ffi::c_int,
    mut status: ::core::ffi::c_int,
) {
    if (*(*sl).h264).enable_er == 0 {
        return;
    }
    ff_er_add_slice((*sl).er, startx, starty, endx, endy, status);
}
unsafe extern "C" fn decode_slice(
    mut avctx: *mut AVCodecContext,
    mut arg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut sl: *mut H264SliceContext = arg as *mut H264SliceContext;
    let mut h: *const H264Context = (*sl).h264 as *const H264Context;
    let mut lf_x_start: ::core::ffi::c_int = (*sl).mb_x;
    let mut orig_deblock: ::core::ffi::c_int = (*sl).deblocking_filter;
    let mut ret: ::core::ffi::c_int = 0;
    (*sl).linesize = (*(*(*h).cur_pic_ptr).f).linesize[0 as ::core::ffi::c_int as usize]
        as ptrdiff_t;
    (*sl).uvlinesize = (*(*(*h).cur_pic_ptr).f)
        .linesize[1 as ::core::ffi::c_int as usize] as ptrdiff_t;
    ret = alloc_scratch_buffers(sl, (*sl).linesize as ::core::ffi::c_int);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    (*sl).mb_skip_run = -(1 as ::core::ffi::c_int);
    if !((*h).block_offset[15 as ::core::ffi::c_int as usize] as ptrdiff_t
        == (4 as ::core::ffi::c_int
            * (scan8[15 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                - scan8[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                & 7 as ::core::ffi::c_int) << (*h).pixel_shift) as ptrdiff_t
            + 4 as ptrdiff_t * (*sl).linesize
                * (scan8[15 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    - scan8[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    >> 3 as ::core::ffi::c_int) as ptrdiff_t)
    {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"h->block_offset[15] == (4 * ((scan8[15] - scan8[0]) & 7) << h->pixel_shift) + 4 * sl->linesize * ((scan8[15] - scan8[0]) >> 3)\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"src/libavcodec/h264_slice.c\0" as *const u8 as *const ::core::ffi::c_char,
            2581 as ::core::ffi::c_int,
        );
        abort();
    }
    if (*h).postpone_filter != 0 {
        (*sl).deblocking_filter = 0 as ::core::ffi::c_int;
    }
    (*sl).is_complex = ((*h).mb_aff_frame != 0
        || (*h).picture_structure != 3 as ::core::ffi::c_int
        || 0 as ::core::ffi::c_int != 0
            && (*h).flags & (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int != 0)
        as ::core::ffi::c_int;
    if (*(*h).avctx).active_thread_type & 2 as ::core::ffi::c_int == 0
        && (*h).picture_structure == 3 as ::core::ffi::c_int
        && !(*(*sl).er).error_status_table.is_null()
    {
        let start_i: ::core::ffi::c_int = av_clip_c(
            (*sl).resync_mb_x + (*sl).resync_mb_y * (*h).mb_width,
            0 as ::core::ffi::c_int,
            (*h).mb_num - 1 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int;
        if start_i != 0 {
            let mut prev_status: ::core::ffi::c_int = *(*(*sl).er)
                .error_status_table
                .offset(
                    *(*(*sl).er)
                        .mb_index2xy
                        .offset((start_i - 1 as ::core::ffi::c_int) as isize) as isize,
                ) as ::core::ffi::c_int;
            prev_status &= !(1 as ::core::ffi::c_int);
            if prev_status
                != 64 as ::core::ffi::c_int | 32 as ::core::ffi::c_int
                    | 16 as ::core::ffi::c_int
            {
                (*(*sl).er).error_occurred = 1 as ::core::ffi::c_int;
            }
        }
    }
    if (*(*h).ps.pps).cabac != 0 {
        align_get_bits(&raw mut (*sl).gb);
        ret = ff_init_cabac_decoder(
            &raw mut (*sl).cabac,
            (*sl)
                .gb
                .buffer
                .offset(
                    (get_bits_count(&raw mut (*sl).gb) / 8 as ::core::ffi::c_int)
                        as isize,
                ),
            (get_bits_left(&raw mut (*sl).gb) + 7 as ::core::ffi::c_int)
                / 8 as ::core::ffi::c_int,
        );
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        ff_h264_init_cabac_states(h, sl);
        loop {
            let mut ret_0: ::core::ffi::c_int = 0;
            let mut eos: ::core::ffi::c_int = 0;
            if (*sl).mb_x + (*sl).mb_y * (*h).mb_width >= (*sl).next_slice_idx {
                av_log(
                    (*h).avctx as *mut ::core::ffi::c_void,
                    16 as ::core::ffi::c_int,
                    b"Slice overlaps with next at %d\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*sl).next_slice_idx,
                );
                er_add_slice(
                    sl,
                    (*sl).resync_mb_x,
                    (*sl).resync_mb_y,
                    (*sl).mb_x,
                    (*sl).mb_y,
                    2 as ::core::ffi::c_int | 4 as ::core::ffi::c_int
                        | 8 as ::core::ffi::c_int,
                );
                return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                    | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
            }
            ret_0 = ff_h264_decode_mb_cabac(h, sl);
            if ret_0 >= 0 as ::core::ffi::c_int {
                ff_h264_hl_decode_mb(h, sl);
            }
            if ret_0 >= 0 as ::core::ffi::c_int && (*h).mb_aff_frame != 0 {
                (*sl).mb_y += 1;
                ret_0 = ff_h264_decode_mb_cabac(h, sl);
                if ret_0 >= 0 as ::core::ffi::c_int {
                    ff_h264_hl_decode_mb(h, sl);
                }
                (*sl).mb_y -= 1;
            }
            eos = get_cabac_terminate(&raw mut (*sl).cabac);
            if (*h).workaround_bugs & 16384 as ::core::ffi::c_int != 0
                && (*sl).cabac.bytestream
                    > (*sl).cabac.bytestream_end.offset(2 as ::core::ffi::c_int as isize)
            {
                er_add_slice(
                    sl,
                    (*sl).resync_mb_x,
                    (*sl).resync_mb_y,
                    (*sl).mb_x - 1 as ::core::ffi::c_int,
                    (*sl).mb_y,
                    16 as ::core::ffi::c_int | 32 as ::core::ffi::c_int
                        | 64 as ::core::ffi::c_int,
                );
                if (*sl).mb_x >= lf_x_start {
                    loop_filter(h, sl, lf_x_start, (*sl).mb_x + 1 as ::core::ffi::c_int);
                }
                break;
            } else {
                if (*sl).cabac.bytestream
                    > (*sl).cabac.bytestream_end.offset(2 as ::core::ffi::c_int as isize)
                {
                    av_log(
                        (*h).avctx as *mut ::core::ffi::c_void,
                        48 as ::core::ffi::c_int,
                        b"bytestream overread %td\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        (*sl).cabac.bytestream_end.offset_from((*sl).cabac.bytestream)
                            as ::core::ffi::c_long,
                    );
                }
                if ret_0 < 0 as ::core::ffi::c_int
                    || (*sl).cabac.bytestream
                        > (*sl)
                            .cabac
                            .bytestream_end
                            .offset(4 as ::core::ffi::c_int as isize)
                {
                    av_log(
                        (*h).avctx as *mut ::core::ffi::c_void,
                        16 as ::core::ffi::c_int,
                        b"error while decoding MB %d %d, bytestream %td\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        (*sl).mb_x,
                        (*sl).mb_y,
                        (*sl).cabac.bytestream_end.offset_from((*sl).cabac.bytestream)
                            as ::core::ffi::c_long,
                    );
                    er_add_slice(
                        sl,
                        (*sl).resync_mb_x,
                        (*sl).resync_mb_y,
                        (*sl).mb_x,
                        (*sl).mb_y,
                        2 as ::core::ffi::c_int | 4 as ::core::ffi::c_int
                            | 8 as ::core::ffi::c_int,
                    );
                    return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                        | ('D' as i32) << 16 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint
                        | ('A' as i32 as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                }
                (*sl).mb_x += 1;
                if (*sl).mb_x >= (*h).mb_width {
                    loop_filter(h, sl, lf_x_start, (*sl).mb_x);
                    lf_x_start = 0 as ::core::ffi::c_int;
                    (*sl).mb_x = lf_x_start;
                    decode_finish_row(h, sl);
                    (*sl).mb_y += 1;
                    if (*h).mb_aff_frame != 0
                        || (*h).picture_structure != 3 as ::core::ffi::c_int
                    {
                        (*sl).mb_y += 1;
                        if (*h).mb_aff_frame != 0 && (*sl).mb_y < (*h).mb_height {
                            predict_field_decoding_flag(h, sl);
                        }
                    }
                }
                if !(eos != 0 || (*sl).mb_y >= (*h).mb_height) {
                    continue;
                }
                er_add_slice(
                    sl,
                    (*sl).resync_mb_x,
                    (*sl).resync_mb_y,
                    (*sl).mb_x - 1 as ::core::ffi::c_int,
                    (*sl).mb_y,
                    16 as ::core::ffi::c_int | 32 as ::core::ffi::c_int
                        | 64 as ::core::ffi::c_int,
                );
                if (*sl).mb_x > lf_x_start {
                    loop_filter(h, sl, lf_x_start, (*sl).mb_x);
                }
                break;
            }
        }
    } else {
        loop {
            let mut ret_1: ::core::ffi::c_int = 0;
            if (*sl).mb_x + (*sl).mb_y * (*h).mb_width >= (*sl).next_slice_idx {
                av_log(
                    (*h).avctx as *mut ::core::ffi::c_void,
                    16 as ::core::ffi::c_int,
                    b"Slice overlaps with next at %d\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*sl).next_slice_idx,
                );
                er_add_slice(
                    sl,
                    (*sl).resync_mb_x,
                    (*sl).resync_mb_y,
                    (*sl).mb_x,
                    (*sl).mb_y,
                    2 as ::core::ffi::c_int | 4 as ::core::ffi::c_int
                        | 8 as ::core::ffi::c_int,
                );
                return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                    | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
            }
            ret_1 = ff_h264_decode_mb_cavlc(h, sl);
            if ret_1 >= 0 as ::core::ffi::c_int {
                ff_h264_hl_decode_mb(h, sl);
            }
            if ret_1 >= 0 as ::core::ffi::c_int && (*h).mb_aff_frame != 0 {
                (*sl).mb_y += 1;
                ret_1 = ff_h264_decode_mb_cavlc(h, sl);
                if ret_1 >= 0 as ::core::ffi::c_int {
                    ff_h264_hl_decode_mb(h, sl);
                }
                (*sl).mb_y -= 1;
            }
            if ret_1 < 0 as ::core::ffi::c_int {
                av_log(
                    (*h).avctx as *mut ::core::ffi::c_void,
                    16 as ::core::ffi::c_int,
                    b"error while decoding MB %d %d\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*sl).mb_x,
                    (*sl).mb_y,
                );
                er_add_slice(
                    sl,
                    (*sl).resync_mb_x,
                    (*sl).resync_mb_y,
                    (*sl).mb_x,
                    (*sl).mb_y,
                    2 as ::core::ffi::c_int | 4 as ::core::ffi::c_int
                        | 8 as ::core::ffi::c_int,
                );
                return ret_1;
            }
            (*sl).mb_x += 1;
            if (*sl).mb_x >= (*h).mb_width {
                loop_filter(h, sl, lf_x_start, (*sl).mb_x);
                lf_x_start = 0 as ::core::ffi::c_int;
                (*sl).mb_x = lf_x_start;
                decode_finish_row(h, sl);
                (*sl).mb_y += 1;
                if (*h).mb_aff_frame != 0
                    || (*h).picture_structure != 3 as ::core::ffi::c_int
                {
                    (*sl).mb_y += 1;
                    if (*h).mb_aff_frame != 0 && (*sl).mb_y < (*h).mb_height {
                        predict_field_decoding_flag(h, sl);
                    }
                }
                if (*sl).mb_y >= (*h).mb_height {
                    if get_bits_left(&raw mut (*sl).gb) == 0 as ::core::ffi::c_int
                        || get_bits_left(&raw mut (*sl).gb) > 0 as ::core::ffi::c_int
                            && (*(*h).avctx).err_recognition
                                & (1 as ::core::ffi::c_int) << 18 as ::core::ffi::c_int == 0
                    {
                        er_add_slice(
                            sl,
                            (*sl).resync_mb_x,
                            (*sl).resync_mb_y,
                            (*sl).mb_x - 1 as ::core::ffi::c_int,
                            (*sl).mb_y,
                            16 as ::core::ffi::c_int | 32 as ::core::ffi::c_int
                                | 64 as ::core::ffi::c_int,
                        );
                        break;
                    } else {
                        er_add_slice(
                            sl,
                            (*sl).resync_mb_x,
                            (*sl).resync_mb_y,
                            (*sl).mb_x,
                            (*sl).mb_y,
                            16 as ::core::ffi::c_int | 32 as ::core::ffi::c_int
                                | 64 as ::core::ffi::c_int,
                        );
                        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                            | ('D' as i32) << 16 as ::core::ffi::c_int)
                            as ::core::ffi::c_uint
                            | ('A' as i32 as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                    }
                }
            }
            if !(get_bits_left(&raw mut (*sl).gb) <= 0 as ::core::ffi::c_int
                && (*sl).mb_skip_run <= 0 as ::core::ffi::c_int)
            {
                continue;
            }
            if get_bits_left(&raw mut (*sl).gb) == 0 as ::core::ffi::c_int {
                er_add_slice(
                    sl,
                    (*sl).resync_mb_x,
                    (*sl).resync_mb_y,
                    (*sl).mb_x - 1 as ::core::ffi::c_int,
                    (*sl).mb_y,
                    16 as ::core::ffi::c_int | 32 as ::core::ffi::c_int
                        | 64 as ::core::ffi::c_int,
                );
                if (*sl).mb_x > lf_x_start {
                    loop_filter(h, sl, lf_x_start, (*sl).mb_x);
                }
                break;
            } else {
                er_add_slice(
                    sl,
                    (*sl).resync_mb_x,
                    (*sl).resync_mb_y,
                    (*sl).mb_x,
                    (*sl).mb_y,
                    2 as ::core::ffi::c_int | 4 as ::core::ffi::c_int
                        | 8 as ::core::ffi::c_int,
                );
                return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                    | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
            }
        }
    }
    (*sl).deblocking_filter = orig_deblock;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_h264_execute_decode_slices(
    mut h: *mut H264Context,
) -> ::core::ffi::c_int {
    let avctx: *mut AVCodecContext = (*h).avctx;
    let mut sl: *mut H264SliceContext = ::core::ptr::null_mut::<H264SliceContext>();
    let mut context_count: ::core::ffi::c_int = (*h).nb_slice_ctx_queued;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    (*(*h).slice_ctx.offset(0 as ::core::ffi::c_int as isize)).next_slice_idx = 2147483647
        as ::core::ffi::c_int;
    if !(*(*h).avctx).hwaccel.is_null() || context_count < 1 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if !(context_count != 0
        && (*(*h).slice_ctx.offset((context_count - 1 as ::core::ffi::c_int) as isize))
            .mb_y < (*h).mb_height)
    {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"context_count && h->slice_ctx[context_count - 1].mb_y < h->mb_height\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"src/libavcodec/h264_slice.c\0" as *const u8 as *const ::core::ffi::c_char,
            2789 as ::core::ffi::c_int,
        );
        abort();
    }
    if context_count == 1 as ::core::ffi::c_int {
        (*(*h).slice_ctx.offset(0 as ::core::ffi::c_int as isize)).next_slice_idx = (*h)
            .mb_width * (*h).mb_height;
        (*h).postpone_filter = 0 as ::core::ffi::c_int;
        ret = decode_slice(
            avctx as *mut AVCodecContext,
            (*h).slice_ctx.offset(0 as ::core::ffi::c_int as isize)
                as *mut H264SliceContext as *mut ::core::ffi::c_void,
        );
        (*h).mb_y = (*(*h).slice_ctx.offset(0 as ::core::ffi::c_int as isize)).mb_y;
        ret < 0 as ::core::ffi::c_int;
    } else {
        if !(context_count > 0 as ::core::ffi::c_int) {
            av_log(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                0 as ::core::ffi::c_int,
                b"Assertion %s failed at %s:%d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"context_count > 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/libavcodec/h264_slice.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                2801 as ::core::ffi::c_int,
            );
            abort();
        }
        i = 0 as ::core::ffi::c_int;
        while i < context_count {
            let mut next_slice_idx: ::core::ffi::c_int = (*h).mb_width * (*h).mb_height;
            let mut slice_idx: ::core::ffi::c_int = 0;
            sl = (*h).slice_ctx.offset(i as isize) as *mut H264SliceContext;
            slice_idx = (*sl).mb_y * (*h).mb_width + (*sl).mb_x;
            j = 0 as ::core::ffi::c_int;
            while j < context_count {
                let mut sl2: *mut H264SliceContext = (*h).slice_ctx.offset(j as isize)
                    as *mut H264SliceContext;
                let mut slice_idx2: ::core::ffi::c_int = (*sl2).mb_y * (*h).mb_width
                    + (*sl2).mb_x;
                if !(i == j || slice_idx2 < slice_idx) {
                    next_slice_idx = if next_slice_idx > slice_idx2 {
                        slice_idx2
                    } else {
                        next_slice_idx
                    };
                }
                j += 1;
            }
            (*sl).next_slice_idx = next_slice_idx;
            i += 1;
        }
        (*avctx)
            .execute
            .expect(
                "non-null function pointer",
            )(
            avctx as *mut AVCodecContext,
            Some(
                decode_slice
                    as unsafe extern "C" fn(
                        *mut AVCodecContext,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            (*h).slice_ctx as *mut ::core::ffi::c_void,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            context_count,
            ::core::mem::size_of::<H264SliceContext>() as ::core::ffi::c_int,
        );
        sl = (*h).slice_ctx.offset((context_count - 1 as ::core::ffi::c_int) as isize)
            as *mut H264SliceContext;
        (*h).mb_y = (*sl).mb_y;
        if (*h).postpone_filter != 0 {
            (*h).postpone_filter = 0 as ::core::ffi::c_int;
            i = 0 as ::core::ffi::c_int;
            while i < context_count {
                let mut y_end: ::core::ffi::c_int = 0;
                let mut x_end: ::core::ffi::c_int = 0;
                sl = (*h).slice_ctx.offset(i as isize) as *mut H264SliceContext;
                y_end = if (*sl).mb_y + 1 as ::core::ffi::c_int > (*h).mb_height {
                    (*h).mb_height
                } else {
                    (*sl).mb_y + 1 as ::core::ffi::c_int
                };
                x_end = if (*sl).mb_y >= (*h).mb_height {
                    (*h).mb_width
                } else {
                    (*sl).mb_x
                };
                j = (*sl).resync_mb_y;
                while j < y_end {
                    (*sl).mb_y = j;
                    loop_filter(
                        h,
                        sl,
                        if j > (*sl).resync_mb_y {
                            0 as ::core::ffi::c_int
                        } else {
                            (*sl).resync_mb_x
                        },
                        if j == y_end - 1 as ::core::ffi::c_int {
                            x_end
                        } else {
                            (*h).mb_width
                        },
                    );
                    j
                        += 1 as ::core::ffi::c_int
                            + ((*h).mb_aff_frame != 0
                                || (*h).picture_structure != 3 as ::core::ffi::c_int)
                                as ::core::ffi::c_int;
                }
                i += 1;
            }
        }
    }
    (*h).nb_slice_ctx_queued = 0 as ::core::ffi::c_int;
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
pub struct AVDictionary {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVCodecDescriptor {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVCodecInternal {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct MPVEncContext {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct ThreadProgress {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct ThreadFrameProgress {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVRefStructPool {
    _opaque: [u8; 0],
}
