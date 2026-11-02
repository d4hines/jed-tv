// Translated from FFmpeg 9.0.1 libavutil/frame.c
// (preprocessed for armv7-unknown-linux-musleabihf) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
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
    fn ff_frame_side_data_add_from_buf(
        sd: *mut *mut *mut AVFrameSideData,
        nb_sd: *mut ::core::ffi::c_int,
        type_0: AVFrameSideDataType,
        buf: *mut AVBufferRef,
    ) -> *mut AVFrameSideData;
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
    fn av_image_fill_linesizes(
        linesizes: *mut ::core::ffi::c_int,
        pix_fmt: AVPixelFormat,
        width: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn av_image_fill_plane_sizes(
        size: *mut size_t,
        pix_fmt: AVPixelFormat,
        height: ::core::ffi::c_int,
        linesizes: *const ptrdiff_t,
    ) -> ::core::ffi::c_int;
    fn av_buffer_alloc(size: size_t) -> *mut AVBufferRef;
    fn av_image_fill_pointers(
        data: *mut *mut uint8_t,
        pix_fmt: AVPixelFormat,
        height: ::core::ffi::c_int,
        ptr: *mut uint8_t,
        linesizes: *const ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn av_malloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn av_refstruct_unref(objp: *mut ::core::ffi::c_void);
    fn av_sample_fmt_is_planar(sample_fmt: AVSampleFormat) -> ::core::ffi::c_int;
    fn av_refstruct_replace(
        dstp: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
    );
    fn av_buffer_ref(buf: *const AVBufferRef) -> *mut AVBufferRef;
    fn av_samples_get_buffer_size(
        linesize: *mut ::core::ffi::c_int,
        nb_channels: ::core::ffi::c_int,
        nb_samples: ::core::ffi::c_int,
        sample_fmt: AVSampleFormat,
        align: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn av_buffer_unref(buf: *mut *mut AVBufferRef);
    fn av_calloc(nmemb: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn av_buffer_is_writable(buf: *const AVBufferRef) -> ::core::ffi::c_int;
    fn av_image_copy(
        dst_data: *const *mut uint8_t,
        dst_linesizes: *const ::core::ffi::c_int,
        src_data: *const *const uint8_t,
        src_linesizes: *const ::core::ffi::c_int,
        pix_fmt: AVPixelFormat,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    );
    fn av_pix_fmt_desc_get(pix_fmt: AVPixelFormat) -> *const AVPixFmtDescriptor;
    fn av_dict_copy(
        dst: *mut *mut AVDictionary,
        src: *const AVDictionary,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn av_buffer_replace(
        dst: *mut *mut AVBufferRef,
        src: *const AVBufferRef,
    ) -> ::core::ffi::c_int;
    fn av_dict_free(m: *mut *mut AVDictionary);
    fn av_pix_fmt_count_planes(pix_fmt: AVPixelFormat) -> ::core::ffi::c_int;
    fn av_realloc_array(
        ptr: *mut ::core::ffi::c_void,
        nmemb: size_t,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn av_samples_copy(
        dst: *const *mut uint8_t,
        src: *const *mut uint8_t,
        dst_offset: ::core::ffi::c_int,
        src_offset: ::core::ffi::c_int,
        nb_samples: ::core::ffi::c_int,
        nb_channels: ::core::ffi::c_int,
        sample_fmt: AVSampleFormat,
    ) -> ::core::ffi::c_int;
    fn av_image_check_size(
        w: ::core::ffi::c_uint,
        h: ::core::ffi::c_uint,
        log_offset: ::core::ffi::c_int,
        log_ctx: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn av_hwframe_get_buffer(
        hwframe_ctx: *mut AVBufferRef,
        frame: *mut AVFrame,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn av_freep(ptr: *mut ::core::ffi::c_void);
    fn av_hwframe_transfer_data(
        dst: *mut AVFrame,
        src: *const AVFrame,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn av_memdup(
        p: *const ::core::ffi::c_void,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn av_channel_layout_uninit(channel_layout: *mut AVChannelLayout);
    fn av_channel_layout_copy(
        dst: *mut AVChannelLayout,
        src: *const AVChannelLayout,
    ) -> ::core::ffi::c_int;
    fn av_channel_layout_check(
        channel_layout: *const AVChannelLayout,
    ) -> ::core::ffi::c_int;
    fn av_channel_layout_compare(
        chl: *const AVChannelLayout,
        chl1: *const AVChannelLayout,
    ) -> ::core::ffi::c_int;
    fn av_frame_side_data_free(
        sd: *mut *mut *mut AVFrameSideData,
        nb_sd: *mut ::core::ffi::c_int,
    );
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
pub type ptrdiff_t = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBufferRef {
    pub buffer: *mut AVBuffer,
    pub data: *mut uint8_t,
    pub size: size_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: ::core::ffi::c_int,
    pub den: ::core::ffi::c_int,
}
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
pub struct AVComponentDescriptor {
    pub plane: ::core::ffi::c_int,
    pub step: ::core::ffi::c_int,
    pub offset: ::core::ffi::c_int,
    pub shift: ::core::ffi::c_int,
    pub depth: ::core::ffi::c_int,
}
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const AV_FRAME_CROP_UNALIGNED: C2RustUnnamed_0 = 1;
unsafe extern "C" fn get_frame_defaults(mut frame: *mut AVFrame) {
    memset(
        frame as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<AVFrame>() as size_t,
    );
    (*frame).pkt_dts = 0x8000000000000000 as ::core::ffi::c_ulonglong as int64_t;
    (*frame).pts = (*frame).pkt_dts;
    (*frame).best_effort_timestamp = 0x8000000000000000 as ::core::ffi::c_ulonglong
        as int64_t;
    (*frame).duration = 0 as int64_t;
    (*frame).time_base = AVRational {
        num: 0 as ::core::ffi::c_int,
        den: 1 as ::core::ffi::c_int,
    };
    (*frame).sample_aspect_ratio = AVRational {
        num: 0 as ::core::ffi::c_int,
        den: 1 as ::core::ffi::c_int,
    };
    (*frame).format = -(1 as ::core::ffi::c_int);
    (*frame).extended_data = &raw mut (*frame).data as *mut *mut uint8_t;
    (*frame).color_primaries = AVCOL_PRI_UNSPECIFIED;
    (*frame).color_trc = AVCOL_TRC_UNSPECIFIED;
    (*frame).colorspace = AVCOL_SPC_UNSPECIFIED;
    (*frame).color_range = AVCOL_RANGE_UNSPECIFIED;
    (*frame).chroma_location = AVCHROMA_LOC_UNSPECIFIED;
    (*frame).alpha_mode = AVALPHA_MODE_UNSPECIFIED;
    (*frame).flags = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_frame_alloc() -> *mut AVFrame {
    let mut frame: *mut AVFrame = av_malloc(::core::mem::size_of::<AVFrame>() as size_t)
        as *mut AVFrame;
    if frame.is_null() {
        return ::core::ptr::null_mut::<AVFrame>();
    }
    get_frame_defaults(frame);
    return frame;
}
#[no_mangle]
pub unsafe extern "C" fn av_frame_free(mut frame: *mut *mut AVFrame) {
    if frame.is_null() || (*frame).is_null() {
        return;
    }
    av_frame_unref(*frame);
    av_freep(frame as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn get_video_buffer(
    mut frame: *mut AVFrame,
    mut align: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut desc: *const AVPixFmtDescriptor = av_pix_fmt_desc_get(
        (*frame).format as AVPixelFormat,
    );
    let mut ret: ::core::ffi::c_int = 0;
    let mut padded_height: ::core::ffi::c_int = 0;
    let mut plane_padding: ::core::ffi::c_int = 0;
    let mut linesizes: [ptrdiff_t; 4] = [0; 4];
    let mut total_size: size_t = 0;
    let mut sizes: [size_t; 4] = [0; 4];
    if desc.is_null() {
        return -(22 as ::core::ffi::c_int);
    }
    ret = av_image_check_size(
        (*frame).width as ::core::ffi::c_uint,
        (*frame).height as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    if align <= 0 as ::core::ffi::c_int {
        align = if 0 as ::core::ffi::c_int != 0 {
            64 as ::core::ffi::c_int
        } else {
            32 as ::core::ffi::c_int
        };
    }
    plane_padding = if (if 0 as ::core::ffi::c_int != 0 {
        64 as ::core::ffi::c_int
    } else {
        32 as ::core::ffi::c_int
    }) > align
    {
        if 0 as ::core::ffi::c_int != 0 {
            64 as ::core::ffi::c_int
        } else {
            32 as ::core::ffi::c_int
        }
    } else {
        align
    };
    if (*frame).linesize[0 as ::core::ffi::c_int as usize] == 0 {
        let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        while i <= align {
            ret = av_image_fill_linesizes(
                &raw mut (*frame).linesize as *mut ::core::ffi::c_int,
                (*frame).format as AVPixelFormat,
                (*frame).width + i - 1 as ::core::ffi::c_int
                    & !(i - 1 as ::core::ffi::c_int),
            );
            if ret < 0 as ::core::ffi::c_int {
                return ret;
            }
            if (*frame).linesize[0 as ::core::ffi::c_int as usize]
                & align - 1 as ::core::ffi::c_int == 0
            {
                break;
            }
            i += i;
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < 4 as ::core::ffi::c_int && (*frame).linesize[i_0 as usize] != 0 {
            (*frame).linesize[i_0 as usize] = (*frame).linesize[i_0 as usize] + align
                - 1 as ::core::ffi::c_int & !(align - 1 as ::core::ffi::c_int);
            i_0 += 1;
        }
    }
    let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_1 < 4 as ::core::ffi::c_int {
        linesizes[i_1 as usize] = (*frame).linesize[i_1 as usize] as ptrdiff_t;
        i_1 += 1;
    }
    padded_height = (*frame).height + 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
        & !(32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int);
    ret = av_image_fill_plane_sizes(
        &raw mut sizes as *mut size_t,
        (*frame).format as AVPixelFormat,
        padded_height,
        &raw mut linesizes as *mut ptrdiff_t as *const ptrdiff_t,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    total_size = (4 as ::core::ffi::c_int * plane_padding
        + 4 as ::core::ffi::c_int * align) as size_t;
    let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_2 < 4 as ::core::ffi::c_int {
        if sizes[i_2 as usize] > (0xffffffff as size_t).wrapping_sub(total_size) {
            return -(22 as ::core::ffi::c_int);
        }
        total_size = total_size.wrapping_add(sizes[i_2 as usize]);
        i_2 += 1;
    }
    (*frame).buf[0 as ::core::ffi::c_int as usize] = av_buffer_alloc(total_size);
    if (*frame).buf[0 as ::core::ffi::c_int as usize].is_null() {
        ret = -(12 as ::core::ffi::c_int);
    } else {
        ret = av_image_fill_pointers(
            &raw mut (*frame).data as *mut *mut uint8_t,
            (*frame).format as AVPixelFormat,
            padded_height,
            (*(*frame).buf[0 as ::core::ffi::c_int as usize]).data,
            &raw mut (*frame).linesize as *mut ::core::ffi::c_int
                as *const ::core::ffi::c_int,
        );
        if !(ret < 0 as ::core::ffi::c_int) {
            let mut i_3: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while i_3 < 4 as ::core::ffi::c_int {
                if !(*frame).data[i_3 as usize].is_null() {
                    (*frame).data[i_3 as usize] = (*frame)
                        .data[i_3 as usize]
                        .offset((i_3 * plane_padding) as isize);
                }
                (*frame).data[i_3 as usize] = (((*frame).data[i_3 as usize] as uintptr_t)
                    .wrapping_add(align as uintptr_t)
                    .wrapping_sub(1 as uintptr_t)
                    & !(align - 1 as ::core::ffi::c_int) as uintptr_t) as *mut uint8_t;
                i_3 += 1;
            }
            (*frame).extended_data = &raw mut (*frame).data as *mut *mut uint8_t;
            return 0 as ::core::ffi::c_int;
        }
    }
    av_frame_unref(frame);
    return ret;
}
#[inline(always)]
unsafe extern "C" fn ff_ctz_c(mut v: ::core::ffi::c_int) -> ::core::ffi::c_int {
    static mut debruijn_ctz32: [uint8_t; 32] = [
        0 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        28 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        29 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
        24 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        30 as ::core::ffi::c_int as uint8_t,
        22 as ::core::ffi::c_int as uint8_t,
        20 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        25 as ::core::ffi::c_int as uint8_t,
        17 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        31 as ::core::ffi::c_int as uint8_t,
        27 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        23 as ::core::ffi::c_int as uint8_t,
        21 as ::core::ffi::c_int as uint8_t,
        19 as ::core::ffi::c_int as uint8_t,
        16 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        26 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
        18 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
    ];
    return debruijn_ctz32[((v as ::core::ffi::c_uint
        & (v as ::core::ffi::c_uint).wrapping_neg())
        .wrapping_mul(0x77cb531 as ::core::ffi::c_uint) >> 27 as ::core::ffi::c_int)
        as usize] as ::core::ffi::c_int;
}
unsafe extern "C" fn get_audio_buffer(
    mut frame: *mut AVFrame,
    mut align: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut planar: ::core::ffi::c_int = av_sample_fmt_is_planar(
        (*frame).format as AVSampleFormat,
    );
    let mut channels: ::core::ffi::c_int = 0;
    let mut planes: ::core::ffi::c_int = 0;
    let mut size: size_t = 0;
    let mut ret: ::core::ffi::c_int = 0;
    channels = (*frame).ch_layout.nb_channels;
    planes = if planar != 0 { channels } else { 1 as ::core::ffi::c_int };
    if (*frame).linesize[0 as ::core::ffi::c_int as usize] == 0 {
        ret = av_samples_get_buffer_size(
            (&raw mut (*frame).linesize as *mut ::core::ffi::c_int)
                .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int,
            channels,
            (*frame).nb_samples,
            (*frame).format as AVSampleFormat,
            align,
        );
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
    }
    if align <= 0 as ::core::ffi::c_int {
        align = if 0 as ::core::ffi::c_int != 0 {
            64 as ::core::ffi::c_int
        } else {
            32 as ::core::ffi::c_int
        };
    }
    if planes > 8 as ::core::ffi::c_int {
        (*frame).extended_data = av_calloc(
            planes as size_t,
            ::core::mem::size_of::<*mut uint8_t>() as size_t,
        ) as *mut *mut uint8_t;
        (*frame).extended_buf = av_calloc(
            (planes - 8 as ::core::ffi::c_int) as size_t,
            ::core::mem::size_of::<*mut AVBufferRef>() as size_t,
        ) as *mut *mut AVBufferRef;
        if (*frame).extended_data.is_null() || (*frame).extended_buf.is_null() {
            av_freep(&raw mut (*frame).extended_data as *mut ::core::ffi::c_void);
            av_freep(&raw mut (*frame).extended_buf as *mut ::core::ffi::c_void);
            return -(12 as ::core::ffi::c_int);
        }
        (*frame).nb_extended_buf = planes - 8 as ::core::ffi::c_int;
    } else {
        (*frame).extended_data = &raw mut (*frame).data as *mut *mut uint8_t;
    }
    if (*frame).linesize[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint
        > (0xffffffff as ::core::ffi::c_uint).wrapping_sub(align as ::core::ffi::c_uint)
    {
        return -(22 as ::core::ffi::c_int);
    }
    size = ((*frame).linesize[0 as ::core::ffi::c_int as usize] as size_t)
        .wrapping_add(align as size_t);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i
        < (if planes > 8 as ::core::ffi::c_int {
            8 as ::core::ffi::c_int
        } else {
            planes
        })
    {
        (*frame).buf[i as usize] = av_buffer_alloc(size);
        if (*frame).buf[i as usize].is_null() {
            av_frame_unref(frame);
            return -(12 as ::core::ffi::c_int);
        }
        (*frame).data[i as usize] = (((*(*frame).buf[i as usize]).data as uintptr_t)
            .wrapping_add(align as uintptr_t)
            .wrapping_sub(1 as uintptr_t)
            & !(align - 1 as ::core::ffi::c_int) as uintptr_t) as *mut uint8_t;
        let ref mut fresh1 = *(*frame).extended_data.offset(i as isize);
        *fresh1 = (*frame).data[i as usize];
        i += 1;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < planes - 8 as ::core::ffi::c_int {
        let ref mut fresh2 = *(*frame).extended_buf.offset(i_0 as isize);
        *fresh2 = av_buffer_alloc(size);
        if (*(*frame).extended_buf.offset(i_0 as isize)).is_null() {
            av_frame_unref(frame);
            return -(12 as ::core::ffi::c_int);
        }
        let ref mut fresh3 = *(*frame)
            .extended_data
            .offset((i_0 + 8 as ::core::ffi::c_int) as isize);
        *fresh3 = (((**(*frame).extended_buf.offset(i_0 as isize)).data as uintptr_t)
            .wrapping_add(align as uintptr_t)
            .wrapping_sub(1 as uintptr_t)
            & !(align - 1 as ::core::ffi::c_int) as uintptr_t) as *mut uint8_t;
        i_0 += 1;
    }
    return 0 as ::core::ffi::c_int;
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
#[no_mangle]
pub unsafe extern "C" fn av_frame_get_buffer(
    mut frame: *mut AVFrame,
    mut align: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*frame).format < 0 as ::core::ffi::c_int {
        return -(22 as ::core::ffi::c_int);
    }
    if (*frame).width > 0 as ::core::ffi::c_int
        && (*frame).height > 0 as ::core::ffi::c_int
    {
        return get_video_buffer(frame, align)
    } else if (*frame).nb_samples > 0 as ::core::ffi::c_int
        && av_channel_layout_check(&raw mut (*frame).ch_layout) != 0
    {
        return get_audio_buffer(frame, align)
    }
    return -(22 as ::core::ffi::c_int);
}
unsafe extern "C" fn frame_copy_props(
    mut dst: *mut AVFrame,
    mut src: *const AVFrame,
    mut force_copy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    (*dst).pict_type = (*src).pict_type;
    (*dst).sample_aspect_ratio = (*src).sample_aspect_ratio;
    (*dst).crop_top = (*src).crop_top;
    (*dst).crop_bottom = (*src).crop_bottom;
    (*dst).crop_left = (*src).crop_left;
    (*dst).crop_right = (*src).crop_right;
    (*dst).pts = (*src).pts;
    (*dst).duration = (*src).duration;
    (*dst).repeat_pict = (*src).repeat_pict;
    (*dst).sample_rate = (*src).sample_rate;
    (*dst).opaque = (*src).opaque;
    (*dst).pkt_dts = (*src).pkt_dts;
    (*dst).time_base = (*src).time_base;
    (*dst).quality = (*src).quality;
    (*dst).best_effort_timestamp = (*src).best_effort_timestamp;
    (*dst).flags = (*src).flags;
    (*dst).decode_error_flags = (*src).decode_error_flags;
    (*dst).color_primaries = (*src).color_primaries;
    (*dst).color_trc = (*src).color_trc;
    (*dst).colorspace = (*src).colorspace;
    (*dst).color_range = (*src).color_range;
    (*dst).chroma_location = (*src).chroma_location;
    (*dst).alpha_mode = (*src).alpha_mode;
    av_dict_copy(&raw mut (*dst).metadata, (*src).metadata, 0 as ::core::ffi::c_int);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*src).nb_side_data {
        let mut sd_src: *const AVFrameSideData = *(*src).side_data.offset(i as isize);
        let mut sd_dst: *mut AVFrameSideData = ::core::ptr::null_mut::<
            AVFrameSideData,
        >();
        if !((*sd_src).type_0 as ::core::ffi::c_uint
            == AV_FRAME_DATA_PANSCAN as ::core::ffi::c_int as ::core::ffi::c_uint
            && ((*src).width != (*dst).width || (*src).height != (*dst).height))
        {
            if force_copy != 0 {
                sd_dst = av_frame_new_side_data(dst, (*sd_src).type_0, (*sd_src).size);
                if sd_dst.is_null() {
                    av_frame_side_data_free(
                        &raw mut (*dst).side_data,
                        &raw mut (*dst).nb_side_data,
                    );
                    return -(12 as ::core::ffi::c_int);
                }
                memcpy(
                    (*sd_dst).data as *mut ::core::ffi::c_void,
                    (*sd_src).data as *const ::core::ffi::c_void,
                    (*sd_src).size,
                );
            } else {
                let mut ref_0: *mut AVBufferRef = av_buffer_ref((*sd_src).buf);
                sd_dst = av_frame_new_side_data_from_buf(dst, (*sd_src).type_0, ref_0);
                if sd_dst.is_null() {
                    av_buffer_unref(&raw mut ref_0);
                    av_frame_side_data_free(
                        &raw mut (*dst).side_data,
                        &raw mut (*dst).nb_side_data,
                    );
                    return -(12 as ::core::ffi::c_int);
                }
            }
            av_dict_copy(
                &raw mut (*sd_dst).metadata,
                (*sd_src).metadata,
                0 as ::core::ffi::c_int,
            );
        }
        i += 1;
    }
    av_refstruct_replace(
        &raw mut (*dst).private_ref as *mut ::core::ffi::c_void,
        (*src).private_ref,
    );
    return av_buffer_replace(&raw mut (*dst).opaque_ref, (*src).opaque_ref);
}
#[no_mangle]
pub unsafe extern "C" fn av_frame_ref(
    mut dst: *mut AVFrame,
    mut src: *const AVFrame,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    (*dst).format = (*src).format;
    (*dst).width = (*src).width;
    (*dst).height = (*src).height;
    (*dst).nb_samples = (*src).nb_samples;
    ret = frame_copy_props(dst, src, 0 as ::core::ffi::c_int);
    if !(ret < 0 as ::core::ffi::c_int) {
        ret = av_channel_layout_copy(
            &raw mut (*dst).ch_layout,
            &raw const (*src).ch_layout,
        );
        if !(ret < 0 as ::core::ffi::c_int) {
            if (*src).buf[0 as ::core::ffi::c_int as usize].is_null() {
                ret = av_frame_get_buffer(dst, 0 as ::core::ffi::c_int);
                if !(ret < 0 as ::core::ffi::c_int) {
                    ret = av_frame_copy(dst, src);
                    if !(ret < 0 as ::core::ffi::c_int) {
                        return 0 as ::core::ffi::c_int;
                    }
                }
            } else {
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                loop {
                    if !((i as usize)
                        < (::core::mem::size_of::<[*mut AVBufferRef; 8]>() as usize)
                            .wrapping_div(
                                ::core::mem::size_of::<*mut AVBufferRef>() as usize,
                            ))
                    {
                        current_block = 8457315219000651999;
                        break;
                    }
                    if !(*src).buf[i as usize].is_null() {
                        (*dst).buf[i as usize] = av_buffer_ref((*src).buf[i as usize]);
                        if (*dst).buf[i as usize].is_null() {
                            ret = -(12 as ::core::ffi::c_int);
                            current_block = 17127117832774799448;
                            break;
                        }
                    }
                    i += 1;
                }
                match current_block {
                    17127117832774799448 => {}
                    _ => {
                        if !(*src).extended_buf.is_null() {
                            (*dst).extended_buf = av_calloc(
                                (*src).nb_extended_buf as size_t,
                                ::core::mem::size_of::<*mut AVBufferRef>() as size_t,
                            ) as *mut *mut AVBufferRef;
                            if (*dst).extended_buf.is_null() {
                                ret = -(12 as ::core::ffi::c_int);
                                current_block = 17127117832774799448;
                            } else {
                                (*dst).nb_extended_buf = (*src).nb_extended_buf;
                                let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                loop {
                                    if !(i_0 < (*src).nb_extended_buf) {
                                        current_block = 14763689060501151050;
                                        break;
                                    }
                                    let ref mut fresh0 = *(*dst)
                                        .extended_buf
                                        .offset(i_0 as isize);
                                    *fresh0 = av_buffer_ref(
                                        *(*src).extended_buf.offset(i_0 as isize),
                                    );
                                    if (*(*dst).extended_buf.offset(i_0 as isize)).is_null() {
                                        ret = -(12 as ::core::ffi::c_int);
                                        current_block = 17127117832774799448;
                                        break;
                                    } else {
                                        i_0 += 1;
                                    }
                                }
                            }
                        } else {
                            current_block = 14763689060501151050;
                        }
                        match current_block {
                            17127117832774799448 => {}
                            _ => {
                                if !(*src).hw_frames_ctx.is_null() {
                                    (*dst).hw_frames_ctx = av_buffer_ref((*src).hw_frames_ctx);
                                    if (*dst).hw_frames_ctx.is_null() {
                                        ret = -(12 as ::core::ffi::c_int);
                                        current_block = 17127117832774799448;
                                    } else {
                                        current_block = 17281240262373992796;
                                    }
                                } else {
                                    current_block = 17281240262373992796;
                                }
                                match current_block {
                                    17127117832774799448 => {}
                                    _ => {
                                        if (*src).extended_data
                                            != &raw const (*src).data as *const *mut uint8_t
                                                as *mut *mut uint8_t
                                        {
                                            let mut ch: ::core::ffi::c_int = (*dst)
                                                .ch_layout
                                                .nb_channels;
                                            if ch <= 0 as ::core::ffi::c_int
                                                || ch as usize
                                                    > (0xffffffff as usize)
                                                        .wrapping_div(
                                                            ::core::mem::size_of::<*mut uint8_t>() as usize,
                                                        )
                                            {
                                                ret = -(22 as ::core::ffi::c_int);
                                                current_block = 17127117832774799448;
                                            } else {
                                                (*dst).extended_data = av_memdup(
                                                    (*src).extended_data as *const ::core::ffi::c_void,
                                                    (::core::mem::size_of::<*mut uint8_t>() as size_t)
                                                        .wrapping_mul(ch as size_t),
                                                ) as *mut *mut uint8_t;
                                                if (*dst).extended_data.is_null() {
                                                    ret = -(12 as ::core::ffi::c_int);
                                                    current_block = 17127117832774799448;
                                                } else {
                                                    current_block = 3123434771885419771;
                                                }
                                            }
                                        } else {
                                            (*dst).extended_data = &raw mut (*dst).data
                                                as *mut *mut uint8_t;
                                            current_block = 3123434771885419771;
                                        }
                                        match current_block {
                                            17127117832774799448 => {}
                                            _ => {
                                                memcpy(
                                                    &raw mut (*dst).data as *mut *mut uint8_t
                                                        as *mut ::core::ffi::c_void,
                                                    &raw const (*src).data as *const *mut uint8_t
                                                        as *const ::core::ffi::c_void,
                                                    ::core::mem::size_of::<[*mut uint8_t; 8]>() as size_t,
                                                );
                                                memcpy(
                                                    &raw mut (*dst).linesize as *mut ::core::ffi::c_int
                                                        as *mut ::core::ffi::c_void,
                                                    &raw const (*src).linesize as *const ::core::ffi::c_int
                                                        as *const ::core::ffi::c_void,
                                                    ::core::mem::size_of::<[::core::ffi::c_int; 8]>() as size_t,
                                                );
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
    av_frame_unref(dst);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_frame_replace(
    mut dst: *mut AVFrame,
    mut src: *const AVFrame,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if dst == src as *mut AVFrame {
        return -(22 as ::core::ffi::c_int);
    }
    if (*src).buf[0 as ::core::ffi::c_int as usize].is_null() {
        av_frame_unref(dst);
        if !(*src).data[0 as ::core::ffi::c_int as usize].is_null()
            || !(*src).data[1 as ::core::ffi::c_int as usize].is_null()
            || !(*src).data[2 as ::core::ffi::c_int as usize].is_null()
            || !(*src).data[3 as ::core::ffi::c_int as usize].is_null()
        {
            return av_frame_ref(dst, src);
        }
        ret = frame_copy_props(dst, src, 0 as ::core::ffi::c_int);
        if ret < 0 as ::core::ffi::c_int {
            current_block = 1214107480904122818;
        } else {
            current_block = 2473556513754201174;
        }
    } else {
        current_block = 2473556513754201174;
    }
    match current_block {
        2473556513754201174 => {
            (*dst).format = (*src).format;
            (*dst).width = (*src).width;
            (*dst).height = (*src).height;
            (*dst).nb_samples = (*src).nb_samples;
            ret = av_channel_layout_copy(
                &raw mut (*dst).ch_layout,
                &raw const (*src).ch_layout,
            );
            if !(ret < 0 as ::core::ffi::c_int) {
                av_frame_side_data_free(
                    &raw mut (*dst).side_data,
                    &raw mut (*dst).nb_side_data,
                );
                av_dict_free(&raw mut (*dst).metadata);
                ret = frame_copy_props(dst, src, 0 as ::core::ffi::c_int);
                if !(ret < 0 as ::core::ffi::c_int) {
                    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    loop {
                        if !((i as usize)
                            < (::core::mem::size_of::<[*mut AVBufferRef; 8]>() as usize)
                                .wrapping_div(
                                    ::core::mem::size_of::<*mut AVBufferRef>() as usize,
                                ))
                        {
                            current_block = 15652330335145281839;
                            break;
                        }
                        ret = av_buffer_replace(
                            (&raw mut (*dst).buf as *mut *mut AVBufferRef)
                                .offset(i as isize) as *mut *mut AVBufferRef,
                            (*src).buf[i as usize],
                        );
                        if ret < 0 as ::core::ffi::c_int {
                            current_block = 1214107480904122818;
                            break;
                        }
                        i += 1;
                    }
                    match current_block {
                        1214107480904122818 => {}
                        _ => {
                            if !(*src).extended_buf.is_null() {
                                if (*dst).nb_extended_buf != (*src).nb_extended_buf {
                                    let mut nb_extended_buf: ::core::ffi::c_int = if (*dst)
                                        .nb_extended_buf > (*src).nb_extended_buf
                                    {
                                        (*src).nb_extended_buf
                                    } else {
                                        (*dst).nb_extended_buf
                                    };
                                    let mut tmp: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
                                        ::core::ffi::c_void,
                                    >();
                                    let mut i_0: ::core::ffi::c_int = nb_extended_buf;
                                    while i_0 < (*dst).nb_extended_buf {
                                        av_buffer_unref(
                                            (*dst).extended_buf.offset(i_0 as isize)
                                                as *mut *mut AVBufferRef,
                                        );
                                        i_0 += 1;
                                    }
                                    tmp = av_realloc_array(
                                        (*dst).extended_buf as *mut ::core::ffi::c_void,
                                        (*src).nb_extended_buf as size_t,
                                        ::core::mem::size_of::<*mut AVBufferRef>() as size_t,
                                    );
                                    if tmp.is_null() {
                                        ret = -(12 as ::core::ffi::c_int);
                                        current_block = 1214107480904122818;
                                    } else {
                                        (*dst).extended_buf = tmp as *mut *mut AVBufferRef;
                                        (*dst).nb_extended_buf = (*src).nb_extended_buf;
                                        memset(
                                            (*dst).extended_buf.offset(nb_extended_buf as isize)
                                                as *mut *mut AVBufferRef as *mut ::core::ffi::c_void,
                                            0 as ::core::ffi::c_int,
                                            (((*src).nb_extended_buf - nb_extended_buf) as size_t)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<*mut AVBufferRef>() as size_t,
                                                ),
                                        );
                                        current_block = 15925075030174552612;
                                    }
                                } else {
                                    current_block = 15925075030174552612;
                                }
                                match current_block {
                                    1214107480904122818 => {}
                                    _ => {
                                        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                        loop {
                                            if !(i_1 < (*src).nb_extended_buf) {
                                                current_block = 6417057564578538666;
                                                break;
                                            }
                                            ret = av_buffer_replace(
                                                (*dst).extended_buf.offset(i_1 as isize)
                                                    as *mut *mut AVBufferRef,
                                                *(*src).extended_buf.offset(i_1 as isize),
                                            );
                                            if ret < 0 as ::core::ffi::c_int {
                                                current_block = 1214107480904122818;
                                                break;
                                            }
                                            i_1 += 1;
                                        }
                                    }
                                }
                            } else {
                                if !(*dst).extended_buf.is_null() {
                                    let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                    while i_2 < (*dst).nb_extended_buf {
                                        av_buffer_unref(
                                            (*dst).extended_buf.offset(i_2 as isize)
                                                as *mut *mut AVBufferRef,
                                        );
                                        i_2 += 1;
                                    }
                                    av_freep(
                                        &raw mut (*dst).extended_buf as *mut ::core::ffi::c_void,
                                    );
                                }
                                current_block = 6417057564578538666;
                            }
                            match current_block {
                                1214107480904122818 => {}
                                _ => {
                                    ret = av_buffer_replace(
                                        &raw mut (*dst).hw_frames_ctx,
                                        (*src).hw_frames_ctx,
                                    );
                                    if !(ret < 0 as ::core::ffi::c_int) {
                                        if (*dst).extended_data
                                            != &raw mut (*dst).data as *mut *mut uint8_t
                                        {
                                            av_freep(
                                                &raw mut (*dst).extended_data as *mut ::core::ffi::c_void,
                                            );
                                        }
                                        if (*src).extended_data
                                            != &raw const (*src).data as *const *mut uint8_t
                                                as *mut *mut uint8_t
                                        {
                                            let mut ch: ::core::ffi::c_int = (*dst)
                                                .ch_layout
                                                .nb_channels;
                                            if ch <= 0 as ::core::ffi::c_int
                                                || ch as usize
                                                    > (0xffffffff as usize)
                                                        .wrapping_div(
                                                            ::core::mem::size_of::<*mut uint8_t>() as usize,
                                                        )
                                            {
                                                ret = -(22 as ::core::ffi::c_int);
                                                current_block = 1214107480904122818;
                                            } else {
                                                (*dst).extended_data = av_memdup(
                                                    (*src).extended_data as *const ::core::ffi::c_void,
                                                    (::core::mem::size_of::<*mut uint8_t>() as size_t)
                                                        .wrapping_mul(ch as size_t),
                                                ) as *mut *mut uint8_t;
                                                if (*dst).extended_data.is_null() {
                                                    ret = -(12 as ::core::ffi::c_int);
                                                    current_block = 1214107480904122818;
                                                } else {
                                                    current_block = 10150597327160359210;
                                                }
                                            }
                                        } else {
                                            (*dst).extended_data = &raw mut (*dst).data
                                                as *mut *mut uint8_t;
                                            current_block = 10150597327160359210;
                                        }
                                        match current_block {
                                            1214107480904122818 => {}
                                            _ => {
                                                memcpy(
                                                    &raw mut (*dst).data as *mut *mut uint8_t
                                                        as *mut ::core::ffi::c_void,
                                                    &raw const (*src).data as *const *mut uint8_t
                                                        as *const ::core::ffi::c_void,
                                                    ::core::mem::size_of::<[*mut uint8_t; 8]>() as size_t,
                                                );
                                                memcpy(
                                                    &raw mut (*dst).linesize as *mut ::core::ffi::c_int
                                                        as *mut ::core::ffi::c_void,
                                                    &raw const (*src).linesize as *const ::core::ffi::c_int
                                                        as *const ::core::ffi::c_void,
                                                    ::core::mem::size_of::<[::core::ffi::c_int; 8]>() as size_t,
                                                );
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
        _ => {}
    }
    av_frame_unref(dst);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_frame_clone(mut src: *const AVFrame) -> *mut AVFrame {
    let mut ret: *mut AVFrame = av_frame_alloc();
    if ret.is_null() {
        return ::core::ptr::null_mut::<AVFrame>();
    }
    if av_frame_ref(ret, src) < 0 as ::core::ffi::c_int {
        av_frame_free(&raw mut ret);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_frame_unref(mut frame: *mut AVFrame) {
    if frame.is_null() {
        return;
    }
    av_frame_side_data_free(&raw mut (*frame).side_data, &raw mut (*frame).nb_side_data);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (i as usize)
        < (::core::mem::size_of::<[*mut AVBufferRef; 8]>() as usize)
            .wrapping_div(::core::mem::size_of::<*mut AVBufferRef>() as usize)
    {
        av_buffer_unref(
            (&raw mut (*frame).buf as *mut *mut AVBufferRef).offset(i as isize)
                as *mut *mut AVBufferRef,
        );
        i += 1;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < (*frame).nb_extended_buf {
        av_buffer_unref(
            (*frame).extended_buf.offset(i_0 as isize) as *mut *mut AVBufferRef,
        );
        i_0 += 1;
    }
    av_freep(&raw mut (*frame).extended_buf as *mut ::core::ffi::c_void);
    av_dict_free(&raw mut (*frame).metadata);
    av_buffer_unref(&raw mut (*frame).hw_frames_ctx);
    av_buffer_unref(&raw mut (*frame).opaque_ref);
    av_refstruct_unref(&raw mut (*frame).private_ref as *mut ::core::ffi::c_void);
    if (*frame).extended_data != &raw mut (*frame).data as *mut *mut uint8_t {
        av_freep(&raw mut (*frame).extended_data as *mut ::core::ffi::c_void);
    }
    av_channel_layout_uninit(&raw mut (*frame).ch_layout);
    get_frame_defaults(frame);
}
#[no_mangle]
pub unsafe extern "C" fn av_frame_move_ref(
    mut dst: *mut AVFrame,
    mut src: *mut AVFrame,
) {
    *dst = *src;
    if (*src).extended_data == &raw mut (*src).data as *mut *mut uint8_t {
        (*dst).extended_data = &raw mut (*dst).data as *mut *mut uint8_t;
    }
    get_frame_defaults(src);
}
#[no_mangle]
pub unsafe extern "C" fn av_frame_is_writable(
    mut frame: *mut AVFrame,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if (*frame).buf[0 as ::core::ffi::c_int as usize].is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (i as usize)
        < (::core::mem::size_of::<[*mut AVBufferRef; 8]>() as usize)
            .wrapping_div(::core::mem::size_of::<*mut AVBufferRef>() as usize)
    {
        if !(*frame).buf[i as usize].is_null() {
            ret
                &= (av_buffer_is_writable((*frame).buf[i as usize]) != 0)
                    as ::core::ffi::c_int;
        }
        i += 1;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < (*frame).nb_extended_buf {
        ret
            &= (av_buffer_is_writable(*(*frame).extended_buf.offset(i_0 as isize)) != 0)
                as ::core::ffi::c_int;
        i_0 += 1;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_frame_make_writable(
    mut frame: *mut AVFrame,
) -> ::core::ffi::c_int {
    let mut tmp: AVFrame = AVFrame {
        data: [::core::ptr::null_mut::<uint8_t>(); 8],
        linesize: [0; 8],
        extended_data: ::core::ptr::null_mut::<*mut uint8_t>(),
        width: 0,
        height: 0,
        nb_samples: 0,
        format: 0,
        pict_type: AV_PICTURE_TYPE_NONE,
        sample_aspect_ratio: AVRational { num: 0, den: 0 },
        pts: 0,
        pkt_dts: 0,
        time_base: AVRational { num: 0, den: 0 },
        quality: 0,
        opaque: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        repeat_pict: 0,
        sample_rate: 0,
        buf: [::core::ptr::null_mut::<AVBufferRef>(); 8],
        extended_buf: ::core::ptr::null_mut::<*mut AVBufferRef>(),
        nb_extended_buf: 0,
        side_data: ::core::ptr::null_mut::<*mut AVFrameSideData>(),
        nb_side_data: 0,
        flags: 0,
        color_range: AVCOL_RANGE_UNSPECIFIED,
        color_primaries: AVCOL_PRI_RESERVED0,
        color_trc: AVCOL_TRC_RESERVED0,
        colorspace: AVCOL_SPC_RGB,
        chroma_location: AVCHROMA_LOC_UNSPECIFIED,
        best_effort_timestamp: 0,
        metadata: ::core::ptr::null_mut::<AVDictionary>(),
        decode_error_flags: 0,
        hw_frames_ctx: ::core::ptr::null_mut::<AVBufferRef>(),
        opaque_ref: ::core::ptr::null_mut::<AVBufferRef>(),
        crop_top: 0,
        crop_bottom: 0,
        crop_left: 0,
        crop_right: 0,
        private_ref: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ch_layout: AVChannelLayout {
            order: AV_CHANNEL_ORDER_UNSPEC,
            nb_channels: 0,
            u: C2RustUnnamed { mask: 0 },
            opaque: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
        duration: 0,
        alpha_mode: AVALPHA_MODE_UNSPECIFIED,
    };
    let mut ret: ::core::ffi::c_int = 0;
    if av_frame_is_writable(frame) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    memset(
        &raw mut tmp as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<AVFrame>() as size_t,
    );
    tmp.format = (*frame).format;
    tmp.width = (*frame).width;
    tmp.height = (*frame).height;
    tmp.nb_samples = (*frame).nb_samples;
    ret = av_channel_layout_copy(&raw mut tmp.ch_layout, &raw mut (*frame).ch_layout);
    if ret < 0 as ::core::ffi::c_int {
        av_frame_unref(&raw mut tmp);
        return ret;
    }
    if !(*frame).hw_frames_ctx.is_null() {
        ret = av_hwframe_get_buffer(
            (*frame).hw_frames_ctx,
            &raw mut tmp,
            0 as ::core::ffi::c_int,
        );
    } else {
        ret = av_frame_get_buffer(&raw mut tmp, 0 as ::core::ffi::c_int);
    }
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    ret = av_frame_copy(&raw mut tmp, frame);
    if ret < 0 as ::core::ffi::c_int {
        av_frame_unref(&raw mut tmp);
        return ret;
    }
    ret = av_frame_copy_props(&raw mut tmp, frame);
    if ret < 0 as ::core::ffi::c_int {
        av_frame_unref(&raw mut tmp);
        return ret;
    }
    av_frame_unref(frame);
    *frame = tmp;
    if &raw mut tmp.data as *mut *mut uint8_t == tmp.extended_data {
        (*frame).extended_data = &raw mut (*frame).data as *mut *mut uint8_t;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_frame_copy_props(
    mut dst: *mut AVFrame,
    mut src: *const AVFrame,
) -> ::core::ffi::c_int {
    return frame_copy_props(dst, src, 1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn av_frame_get_plane_buffer(
    mut frame: *const AVFrame,
    mut plane: ::core::ffi::c_int,
) -> *mut AVBufferRef {
    let mut data: uintptr_t = 0;
    let mut planes: ::core::ffi::c_int = 0;
    if (*frame).nb_samples != 0 {
        let mut channels: ::core::ffi::c_int = (*frame).ch_layout.nb_channels;
        if channels == 0 {
            return ::core::ptr::null_mut::<AVBufferRef>();
        }
        planes = if av_sample_fmt_is_planar((*frame).format as AVSampleFormat) != 0 {
            channels
        } else {
            1 as ::core::ffi::c_int
        };
    } else {
        planes = 4 as ::core::ffi::c_int;
    }
    if plane < 0 as ::core::ffi::c_int || plane >= planes
        || (*(*frame).extended_data.offset(plane as isize)).is_null()
    {
        return ::core::ptr::null_mut::<AVBufferRef>();
    }
    data = *(*frame).extended_data.offset(plane as isize) as uintptr_t;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (i as usize)
        < (::core::mem::size_of::<[*mut AVBufferRef; 8]>() as usize)
            .wrapping_div(::core::mem::size_of::<*mut AVBufferRef>() as usize)
        && !(*frame).buf[i as usize].is_null()
    {
        let mut buf: *mut AVBufferRef = (*frame).buf[i as usize];
        let mut buf_begin: uintptr_t = (*buf).data as uintptr_t;
        if data >= buf_begin && data < (buf_begin as size_t).wrapping_add((*buf).size) {
            return buf;
        }
        i += 1;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < (*frame).nb_extended_buf {
        let mut buf_0: *mut AVBufferRef = *(*frame).extended_buf.offset(i_0 as isize);
        let mut buf_begin_0: uintptr_t = (*buf_0).data as uintptr_t;
        if data >= buf_begin_0
            && data < (buf_begin_0 as size_t).wrapping_add((*buf_0).size)
        {
            return buf_0;
        }
        i_0 += 1;
    }
    return ::core::ptr::null_mut::<AVBufferRef>();
}
#[no_mangle]
pub unsafe extern "C" fn av_frame_new_side_data_from_buf(
    mut frame: *mut AVFrame,
    mut type_0: AVFrameSideDataType,
    mut buf: *mut AVBufferRef,
) -> *mut AVFrameSideData {
    return ff_frame_side_data_add_from_buf(
        &raw mut (*frame).side_data,
        &raw mut (*frame).nb_side_data,
        type_0,
        buf,
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_frame_new_side_data(
    mut frame: *mut AVFrame,
    mut type_0: AVFrameSideDataType,
    mut size: size_t,
) -> *mut AVFrameSideData {
    let mut ret: *mut AVFrameSideData = ::core::ptr::null_mut::<AVFrameSideData>();
    let mut buf: *mut AVBufferRef = av_buffer_alloc(size);
    ret = av_frame_new_side_data_from_buf(frame, type_0, buf);
    if ret.is_null() {
        av_buffer_unref(&raw mut buf);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_frame_get_side_data(
    mut frame: *const AVFrame,
    mut type_0: AVFrameSideDataType,
) -> *mut AVFrameSideData {
    return av_frame_side_data_get((*frame).side_data, (*frame).nb_side_data, type_0)
        as *mut AVFrameSideData;
}
unsafe extern "C" fn frame_copy_video(
    mut dst: *mut AVFrame,
    mut src: *const AVFrame,
) -> ::core::ffi::c_int {
    let mut planes: ::core::ffi::c_int = 0;
    if (*dst).width < (*src).width || (*dst).height < (*src).height {
        return -(22 as ::core::ffi::c_int);
    }
    if !(*src).hw_frames_ctx.is_null() || !(*dst).hw_frames_ctx.is_null() {
        return av_hwframe_transfer_data(dst, src, 0 as ::core::ffi::c_int);
    }
    planes = av_pix_fmt_count_planes((*dst).format as AVPixelFormat);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < planes {
        if (*dst).data[i as usize].is_null() || (*src).data[i as usize].is_null() {
            return -(22 as ::core::ffi::c_int);
        }
        i += 1;
    }
    av_image_copy2(
        &raw mut (*dst).data as *mut *mut uint8_t as *const *mut uint8_t,
        &raw mut (*dst).linesize as *mut ::core::ffi::c_int as *const ::core::ffi::c_int,
        &raw const (*src).data as *const *mut uint8_t,
        &raw const (*src).linesize as *const ::core::ffi::c_int,
        (*dst).format as AVPixelFormat,
        (*src).width,
        (*src).height,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn frame_copy_audio(
    mut dst: *mut AVFrame,
    mut src: *const AVFrame,
) -> ::core::ffi::c_int {
    let mut planar: ::core::ffi::c_int = av_sample_fmt_is_planar(
        (*dst).format as AVSampleFormat,
    );
    let mut channels: ::core::ffi::c_int = (*dst).ch_layout.nb_channels;
    let mut planes: ::core::ffi::c_int = if planar != 0 {
        channels
    } else {
        1 as ::core::ffi::c_int
    };
    if (*dst).nb_samples != (*src).nb_samples
        || av_channel_layout_compare(
            &raw mut (*dst).ch_layout,
            &raw const (*src).ch_layout,
        ) != 0
    {
        return -(22 as ::core::ffi::c_int);
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < planes {
        if (*(*dst).extended_data.offset(i as isize)).is_null()
            || (*(*src).extended_data.offset(i as isize)).is_null()
        {
            return -(22 as ::core::ffi::c_int);
        }
        i += 1;
    }
    av_samples_copy(
        (*dst).extended_data,
        (*src).extended_data,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        (*dst).nb_samples,
        channels,
        (*dst).format as AVSampleFormat,
    );
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_frame_copy(
    mut dst: *mut AVFrame,
    mut src: *const AVFrame,
) -> ::core::ffi::c_int {
    if (*dst).format != (*src).format || (*dst).format < 0 as ::core::ffi::c_int {
        return -(22 as ::core::ffi::c_int);
    }
    if (*dst).width > 0 as ::core::ffi::c_int && (*dst).height > 0 as ::core::ffi::c_int
    {
        return frame_copy_video(dst, src)
    } else if (*dst).nb_samples > 0 as ::core::ffi::c_int
        && av_channel_layout_check(&raw mut (*dst).ch_layout) != 0
    {
        return frame_copy_audio(dst, src)
    }
    return -(22 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn av_frame_remove_side_data(
    mut frame: *mut AVFrame,
    mut type_0: AVFrameSideDataType,
) {
    av_frame_side_data_remove(
        &raw mut (*frame).side_data,
        &raw mut (*frame).nb_side_data,
        type_0,
    );
}
unsafe extern "C" fn calc_cropping_offsets(
    mut offsets: *mut size_t,
    mut frame: *const AVFrame,
    mut desc: *const AVPixFmtDescriptor,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !(*frame).data[i as usize].is_null() {
        let mut comp: *const AVComponentDescriptor = ::core::ptr::null::<
            AVComponentDescriptor,
        >();
        let mut shift_x: ::core::ffi::c_int = if i == 1 as ::core::ffi::c_int
            || i == 2 as ::core::ffi::c_int
        {
            (*desc).log2_chroma_w as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        let mut shift_y: ::core::ffi::c_int = if i == 1 as ::core::ffi::c_int
            || i == 2 as ::core::ffi::c_int
        {
            (*desc).log2_chroma_h as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        if (*desc).flags
            & ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint64_t != 0
            && i == 1 as ::core::ffi::c_int
        {
            *offsets.offset(i as isize) = 0 as size_t;
            break;
        } else {
            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j < (*desc).nb_components as ::core::ffi::c_int {
                if (*desc).comp[j as usize].plane == i {
                    comp = (&raw const (*desc).comp as *const AVComponentDescriptor)
                        .offset(j as isize) as *const AVComponentDescriptor;
                    break;
                } else {
                    j += 1;
                }
            }
            if comp.is_null() {
                return -((('B' as i32 | ('U' as i32) << 8 as ::core::ffi::c_int
                    | ('G' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('!' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
            }
            *offsets.offset(i as isize) = ((*frame).crop_top >> shift_y)
                .wrapping_mul((*frame).linesize[i as usize] as size_t)
                .wrapping_add(
                    ((*frame).crop_left >> shift_x).wrapping_mul((*comp).step as size_t),
                );
            i += 1;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_frame_apply_cropping(
    mut frame: *mut AVFrame,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut desc: *const AVPixFmtDescriptor = ::core::ptr::null::<AVPixFmtDescriptor>();
    let mut offsets: [size_t; 4] = [0; 4];
    let mut ret: ::core::ffi::c_int = 0;
    if !((*frame).width > 0 as ::core::ffi::c_int
        && (*frame).height > 0 as ::core::ffi::c_int)
    {
        return -(22 as ::core::ffi::c_int);
    }
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
        return -(34 as ::core::ffi::c_int);
    }
    desc = av_pix_fmt_desc_get((*frame).format as AVPixelFormat);
    if desc.is_null() {
        return -((('B' as i32 | ('U' as i32) << 8 as ::core::ffi::c_int
            | ('G' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('!' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if (*desc).flags
        & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint64_t != 0
    {
        (*frame).width = ((*frame).width as size_t).wrapping_sub((*frame).crop_right)
            as ::core::ffi::c_int as ::core::ffi::c_int;
        (*frame).height = ((*frame).height as size_t).wrapping_sub((*frame).crop_bottom)
            as ::core::ffi::c_int as ::core::ffi::c_int;
        (*frame).crop_right = 0 as size_t;
        (*frame).crop_bottom = 0 as size_t;
        return 0 as ::core::ffi::c_int;
    }
    ret = calc_cropping_offsets(&raw mut offsets as *mut size_t, frame, desc);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    if flags & AV_FRAME_CROP_UNALIGNED as ::core::ffi::c_int == 0 {
        let mut log2_crop_align: ::core::ffi::c_int = if (*frame).crop_left != 0 {
            ff_ctz_c((*frame).crop_left as ::core::ffi::c_int)
        } else {
            2147483647 as ::core::ffi::c_int
        };
        let mut min_log2_align: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while !(*frame).data[i as usize].is_null() {
            let mut log2_align: ::core::ffi::c_int = if offsets[i as usize] != 0 {
                ff_ctz_c(offsets[i as usize] as ::core::ffi::c_int)
            } else {
                2147483647 as ::core::ffi::c_int
            };
            min_log2_align = if log2_align > min_log2_align {
                min_log2_align
            } else {
                log2_align
            };
            i += 1;
        }
        if log2_crop_align < min_log2_align {
            return -((('B' as i32 | ('U' as i32) << 8 as ::core::ffi::c_int
                | ('G' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('!' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        if min_log2_align < 5 as ::core::ffi::c_int
            && log2_crop_align != 2147483647 as ::core::ffi::c_int
        {
            (*frame).crop_left
                &= !(((1 as ::core::ffi::c_int)
                    << 5 as ::core::ffi::c_int + log2_crop_align - min_log2_align)
                    - 1 as ::core::ffi::c_int) as size_t;
            ret = calc_cropping_offsets(&raw mut offsets as *mut size_t, frame, desc);
            if ret < 0 as ::core::ffi::c_int {
                return ret;
            }
        }
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !(*frame).data[i_0 as usize].is_null() {
        (*frame).data[i_0 as usize] = (*frame)
            .data[i_0 as usize]
            .offset(offsets[i_0 as usize] as isize);
        i_0 += 1;
    }
    (*frame).width = ((*frame).width as size_t)
        .wrapping_sub((*frame).crop_left.wrapping_add((*frame).crop_right))
        as ::core::ffi::c_int as ::core::ffi::c_int;
    (*frame).height = ((*frame).height as size_t)
        .wrapping_sub((*frame).crop_top.wrapping_add((*frame).crop_bottom))
        as ::core::ffi::c_int as ::core::ffi::c_int;
    (*frame).crop_left = 0 as size_t;
    (*frame).crop_right = 0 as size_t;
    (*frame).crop_top = 0 as size_t;
    (*frame).crop_bottom = 0 as size_t;
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
#[repr(C)]
pub struct AVBuffer {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVDictionary {
    _opaque: [u8; 0],
}
