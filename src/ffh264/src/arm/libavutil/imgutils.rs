// Translated from FFmpeg 9.0.1 libavutil/imgutils.c
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
    fn av_pix_fmt_desc_get(pix_fmt: AVPixelFormat) -> *const AVPixFmtDescriptor;
    fn av_rescale_rnd(a: int64_t, b: int64_t, c: int64_t, rnd: AVRounding) -> int64_t;
    fn av_pix_fmt_count_planes(pix_fmt: AVPixelFormat) -> ::core::ffi::c_int;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn av_free(ptr: *mut ::core::ffi::c_void);
    fn av_default_item_name(ctx: *mut ::core::ffi::c_void) -> *const ::core::ffi::c_char;
    fn av_write_image_line2(
        src: *const ::core::ffi::c_void,
        data: *mut *mut uint8_t,
        linesize: *const ::core::ffi::c_int,
        desc: *const AVPixFmtDescriptor,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        c: ::core::ffi::c_int,
        w: ::core::ffi::c_int,
        src_element_size: ::core::ffi::c_int,
    );
    fn av_memcpy_backptr(
        dst: *mut uint8_t,
        back: ::core::ffi::c_int,
        cnt: ::core::ffi::c_int,
    );
}
pub type size_t = ::core::ffi::c_uint;
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
pub type int64_t = ::core::ffi::c_longlong;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint64_t = ::core::ffi::c_ulonglong;
pub type ptrdiff_t = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: ::core::ffi::c_int,
    pub den: ::core::ffi::c_int,
}
pub type AVRounding = ::core::ffi::c_uint;
pub const AV_ROUND_PASS_MINMAX: AVRounding = 8192;
pub const AV_ROUND_NEAR_INF: AVRounding = 5;
pub const AV_ROUND_UP: AVRounding = 3;
pub const AV_ROUND_DOWN: AVRounding = 2;
pub const AV_ROUND_INF: AVRounding = 1;
pub const AV_ROUND_ZERO: AVRounding = 0;
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
pub type AVColorRange = ::core::ffi::c_uint;
pub const AVCOL_RANGE_NB: AVColorRange = 3;
pub const AVCOL_RANGE_JPEG: AVColorRange = 2;
pub const AVCOL_RANGE_MPEG: AVColorRange = 1;
pub const AVCOL_RANGE_UNSPECIFIED: AVColorRange = 0;
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
pub struct ImgUtils {
    pub class: *const AVClass,
    pub log_offset: ::core::ffi::c_int,
    pub log_ctx: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union unaligned_32 {
    pub l: uint32_t,
}
#[no_mangle]
pub unsafe extern "C" fn av_image_fill_max_pixsteps(
    mut max_pixsteps: *mut ::core::ffi::c_int,
    mut max_pixstep_comps: *mut ::core::ffi::c_int,
    mut pixdesc: *const AVPixFmtDescriptor,
) {
    let mut i: ::core::ffi::c_int = 0;
    memset(
        max_pixsteps as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (4 as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    );
    if !max_pixstep_comps.is_null() {
        memset(
            max_pixstep_comps as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (4 as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        );
    }
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        let mut comp: *const AVComponentDescriptor = (&raw const (*pixdesc).comp
            as *const AVComponentDescriptor)
            .offset(i as isize) as *const AVComponentDescriptor;
        if (*comp).step > *max_pixsteps.offset((*comp).plane as isize) {
            *max_pixsteps.offset((*comp).plane as isize) = (*comp).step;
            if !max_pixstep_comps.is_null() {
                *max_pixstep_comps.offset((*comp).plane as isize) = i;
            }
        }
        i += 1;
    }
}
#[inline]
unsafe extern "C" fn image_get_linesize(
    mut width: ::core::ffi::c_int,
    mut plane: ::core::ffi::c_int,
    mut max_step: ::core::ffi::c_int,
    mut max_step_comp: ::core::ffi::c_int,
    mut desc: *const AVPixFmtDescriptor,
) -> ::core::ffi::c_int {
    let mut s: ::core::ffi::c_int = 0;
    let mut shifted_w: ::core::ffi::c_int = 0;
    let mut linesize: ::core::ffi::c_int = 0;
    if desc.is_null() {
        return -(22 as ::core::ffi::c_int);
    }
    if width < 0 as ::core::ffi::c_int {
        return -(22 as ::core::ffi::c_int);
    }
    s = if max_step_comp == 1 as ::core::ffi::c_int
        || max_step_comp == 2 as ::core::ffi::c_int
    {
        (*desc).log2_chroma_w as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    shifted_w = if 0 == 0 {
        -(-width >> s)
    } else {
        width + ((1 as ::core::ffi::c_int) << s) - 1 as ::core::ffi::c_int >> s
    };
    if shifted_w != 0 && max_step > 2147483647 as ::core::ffi::c_int / shifted_w {
        return -(22 as ::core::ffi::c_int);
    }
    linesize = max_step * shifted_w;
    if (*desc).flags & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as uint64_t
        != 0
    {
        linesize = linesize + 7 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int;
    }
    return linesize;
}
#[no_mangle]
pub unsafe extern "C" fn av_image_get_linesize(
    mut pix_fmt: AVPixelFormat,
    mut width: ::core::ffi::c_int,
    mut plane: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut desc: *const AVPixFmtDescriptor = av_pix_fmt_desc_get(pix_fmt);
    let mut max_step: [::core::ffi::c_int; 4] = [0; 4];
    let mut max_step_comp: [::core::ffi::c_int; 4] = [0; 4];
    if desc.is_null()
        || (*desc).flags
            & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint64_t != 0
    {
        return -(22 as ::core::ffi::c_int);
    }
    av_image_fill_max_pixsteps(
        &raw mut max_step as *mut ::core::ffi::c_int,
        &raw mut max_step_comp as *mut ::core::ffi::c_int,
        desc,
    );
    return image_get_linesize(
        width,
        plane,
        max_step[plane as usize],
        max_step_comp[plane as usize],
        desc,
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_image_fill_linesizes(
    mut linesizes: *mut ::core::ffi::c_int,
    mut pix_fmt: AVPixelFormat,
    mut width: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut desc: *const AVPixFmtDescriptor = av_pix_fmt_desc_get(pix_fmt);
    let mut max_step: [::core::ffi::c_int; 4] = [0; 4];
    let mut max_step_comp: [::core::ffi::c_int; 4] = [0; 4];
    memset(
        linesizes as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (4 as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    );
    if desc.is_null()
        || (*desc).flags
            & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint64_t != 0
    {
        return -(22 as ::core::ffi::c_int);
    }
    av_image_fill_max_pixsteps(
        &raw mut max_step as *mut ::core::ffi::c_int,
        &raw mut max_step_comp as *mut ::core::ffi::c_int,
        desc,
    );
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        ret = image_get_linesize(
            width,
            i,
            max_step[i as usize],
            max_step_comp[i as usize],
            desc,
        );
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        *linesizes.offset(i as isize) = ret;
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_image_fill_plane_sizes(
    mut sizes: *mut size_t,
    mut pix_fmt: AVPixelFormat,
    mut height: ::core::ffi::c_int,
    mut linesizes: *const ptrdiff_t,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut has_plane: [::core::ffi::c_int; 4] = [0 as ::core::ffi::c_int; 4];
    let mut desc: *const AVPixFmtDescriptor = av_pix_fmt_desc_get(pix_fmt);
    memset(
        sizes as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<size_t>() as size_t).wrapping_mul(4 as size_t),
    );
    if desc.is_null()
        || (*desc).flags
            & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint64_t != 0
    {
        return -(22 as ::core::ffi::c_int);
    }
    if *linesizes.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
        > (0xffffffff as ::core::ffi::c_uint).wrapping_div(height as ::core::ffi::c_uint)
    {
        return -(22 as ::core::ffi::c_int);
    }
    *sizes.offset(0 as ::core::ffi::c_int as isize) = (*linesizes
        .offset(0 as ::core::ffi::c_int as isize) as size_t)
        .wrapping_mul(height as size_t);
    if (*desc).flags & ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint64_t
        != 0
    {
        *sizes.offset(1 as ::core::ffi::c_int as isize) = (256 as ::core::ffi::c_int
            * 4 as ::core::ffi::c_int) as size_t;
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        has_plane[(*desc).comp[i as usize].plane as usize] = 1 as ::core::ffi::c_int;
        i += 1;
    }
    i = 1 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int && has_plane[i as usize] != 0 {
        let mut h: ::core::ffi::c_int = 0;
        let mut s: ::core::ffi::c_int = if i == 1 as ::core::ffi::c_int
            || i == 2 as ::core::ffi::c_int
        {
            (*desc).log2_chroma_h as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        h = if 0 == 0 {
            -(-height >> s)
        } else {
            height + ((1 as ::core::ffi::c_int) << s) - 1 as ::core::ffi::c_int >> s
        };
        if *linesizes.offset(i as isize) as ::core::ffi::c_uint
            > (0xffffffff as ::core::ffi::c_uint).wrapping_div(h as ::core::ffi::c_uint)
        {
            return -(22 as ::core::ffi::c_int);
        }
        *sizes.offset(i as isize) = (h as size_t)
            .wrapping_mul(*linesizes.offset(i as isize) as size_t);
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_image_fill_pointers(
    mut data: *mut *mut uint8_t,
    mut pix_fmt: AVPixelFormat,
    mut height: ::core::ffi::c_int,
    mut ptr: *mut uint8_t,
    mut linesizes: *const ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut linesizes1: [ptrdiff_t; 4] = [0; 4];
    let mut sizes: [size_t; 4] = [0; 4];
    memset(
        data as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<*mut uint8_t>() as size_t).wrapping_mul(4 as size_t),
    );
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        linesizes1[i as usize] = *linesizes.offset(i as isize) as ptrdiff_t;
        i += 1;
    }
    ret = av_image_fill_plane_sizes(
        &raw mut sizes as *mut size_t,
        pix_fmt,
        height,
        &raw mut linesizes1 as *mut ptrdiff_t as *const ptrdiff_t,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    ret = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        if sizes[i as usize] > (2147483647 as ::core::ffi::c_int - ret) as size_t {
            return -(22 as ::core::ffi::c_int);
        }
        ret = (ret as size_t).wrapping_add(sizes[i as usize]) as ::core::ffi::c_int
            as ::core::ffi::c_int;
        i += 1;
    }
    if ptr.is_null() {
        return ret;
    }
    let ref mut fresh0 = *data.offset(0 as ::core::ffi::c_int as isize);
    *fresh0 = ptr;
    i = 1 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int && sizes[i as usize] != 0 {
        let ref mut fresh1 = *data.offset(i as isize);
        *fresh1 = (*data.offset((i - 1 as ::core::ffi::c_int) as isize))
            .offset(sizes[(i - 1 as ::core::ffi::c_int) as usize] as isize);
        i += 1;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn avpriv_set_systematic_pal2(
    mut pal: *mut uint32_t,
    mut pix_fmt: AVPixelFormat,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 256 as ::core::ffi::c_int {
        let mut r: ::core::ffi::c_int = 0;
        let mut g: ::core::ffi::c_int = 0;
        let mut b: ::core::ffi::c_int = 0;
        match pix_fmt as ::core::ffi::c_int {
            20 => {
                r = (i >> 5 as ::core::ffi::c_int) * 36 as ::core::ffi::c_int;
                g = (i >> 2 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    * 36 as ::core::ffi::c_int;
                b = (i & 3 as ::core::ffi::c_int) * 85 as ::core::ffi::c_int;
            }
            17 => {
                b = (i >> 6 as ::core::ffi::c_int) * 85 as ::core::ffi::c_int;
                g = (i >> 3 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    * 36 as ::core::ffi::c_int;
                r = (i & 7 as ::core::ffi::c_int) * 36 as ::core::ffi::c_int;
            }
            22 => {
                r = (i >> 3 as ::core::ffi::c_int) * 255 as ::core::ffi::c_int;
                g = (i >> 1 as ::core::ffi::c_int & 3 as ::core::ffi::c_int)
                    * 85 as ::core::ffi::c_int;
                b = (i & 1 as ::core::ffi::c_int) * 255 as ::core::ffi::c_int;
            }
            19 => {
                b = (i >> 3 as ::core::ffi::c_int) * 255 as ::core::ffi::c_int;
                g = (i >> 1 as ::core::ffi::c_int & 3 as ::core::ffi::c_int)
                    * 85 as ::core::ffi::c_int;
                r = (i & 1 as ::core::ffi::c_int) * 255 as ::core::ffi::c_int;
            }
            8 => {
                g = i;
                b = g;
                r = b;
            }
            _ => return -(22 as ::core::ffi::c_int),
        }
        *pal.offset(i as isize) = ((b + (g << 8 as ::core::ffi::c_int)
            + (r << 16 as ::core::ffi::c_int)) as ::core::ffi::c_uint)
            .wrapping_add((0xff as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as uint32_t;
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_image_alloc(
    mut pointers: *mut *mut uint8_t,
    mut linesizes: *mut ::core::ffi::c_int,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
    mut pix_fmt: AVPixelFormat,
    mut align: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut desc: *const AVPixFmtDescriptor = av_pix_fmt_desc_get(pix_fmt);
    let mut i: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut linesizes1: [ptrdiff_t; 4] = [0; 4];
    let mut total_size: size_t = 0;
    let mut sizes: [size_t; 4] = [0; 4];
    let mut buf: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    if desc.is_null() {
        return -(22 as ::core::ffi::c_int);
    }
    ret = av_image_check_size(
        w as ::core::ffi::c_uint,
        h as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    ret = av_image_fill_linesizes(
        linesizes,
        pix_fmt,
        (if align > 7 as ::core::ffi::c_int {
            w + 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                & !(8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        } else {
            w
        }),
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        *linesizes.offset(i as isize) = *linesizes.offset(i as isize) + align
            - 1 as ::core::ffi::c_int & !(align - 1 as ::core::ffi::c_int);
        linesizes1[i as usize] = *linesizes.offset(i as isize) as ptrdiff_t;
        i += 1;
    }
    ret = av_image_fill_plane_sizes(
        &raw mut sizes as *mut size_t,
        pix_fmt,
        h,
        &raw mut linesizes1 as *mut ptrdiff_t as *const ptrdiff_t,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    total_size = align as size_t;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        if total_size > (0xffffffff as size_t).wrapping_sub(sizes[i as usize]) {
            return -(22 as ::core::ffi::c_int);
        }
        total_size = total_size.wrapping_add(sizes[i as usize]);
        i += 1;
    }
    buf = av_malloc(total_size) as *mut uint8_t;
    if buf.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    ret = av_image_fill_pointers(
        pointers,
        pix_fmt,
        h,
        buf,
        linesizes as *const ::core::ffi::c_int,
    );
    if ret < 0 as ::core::ffi::c_int {
        av_free(buf as *mut ::core::ffi::c_void);
        return ret;
    }
    if (*desc).flags & ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint64_t
        != 0
    {
        avpriv_set_systematic_pal2(
            *pointers.offset(1 as ::core::ffi::c_int as isize) as *mut uint32_t,
            pix_fmt,
        );
        if align < 4 as ::core::ffi::c_int {
            av_log(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                16 as ::core::ffi::c_int,
                b"Formats with a palette require a minimum alignment of 4\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            av_free(buf as *mut ::core::ffi::c_void);
            return -(22 as ::core::ffi::c_int);
        }
    }
    if (*desc).flags & ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint64_t
        != 0 && !(*pointers.offset(1 as ::core::ffi::c_int as isize)).is_null()
        && (*pointers.offset(1 as ::core::ffi::c_int as isize))
            .offset_from(*pointers.offset(0 as ::core::ffi::c_int as isize))
            as ::core::ffi::c_int
            > *linesizes.offset(0 as ::core::ffi::c_int as isize) * h
    {
        memset(
            (*pointers.offset(0 as ::core::ffi::c_int as isize))
                .offset(
                    (*linesizes.offset(0 as ::core::ffi::c_int as isize) * h) as isize,
                ) as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((*pointers.offset(1 as ::core::ffi::c_int as isize))
                .offset_from(*pointers.offset(0 as ::core::ffi::c_int as isize))
                as ::core::ffi::c_int
                - *linesizes.offset(0 as ::core::ffi::c_int as isize) * h) as size_t,
        );
    }
    return ret;
}
static mut imgutils_class: AVClass = unsafe {
    AVClass {
        class_name: b"IMGUTILS\0" as *const u8 as *const ::core::ffi::c_char,
        item_name: Some(
            av_default_item_name
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                ) -> *const ::core::ffi::c_char,
        ),
        option: ::core::ptr::null::<AVOption>(),
        version: (61 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            | 101 as ::core::ffi::c_int,
        log_level_offset_offset: 4 as ::core::ffi::c_uint as ::core::ffi::c_int,
        parent_log_context_offset: 8 as ::core::ffi::c_uint as ::core::ffi::c_int,
        category: AV_CLASS_CATEGORY_NA,
        get_category: None,
        query_ranges: None,
        child_next: None,
        child_class_iterate: None,
        state_flags_offset: 0,
    }
};
#[no_mangle]
pub unsafe extern "C" fn av_image_check_size2(
    mut w: ::core::ffi::c_uint,
    mut h: ::core::ffi::c_uint,
    mut max_pixels: int64_t,
    mut pix_fmt: AVPixelFormat,
    mut log_offset: ::core::ffi::c_int,
    mut log_ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut imgutils: ImgUtils = ImgUtils {
        class: &raw const imgutils_class,
        log_offset: log_offset,
        log_ctx: log_ctx,
    };
    let mut stride: int64_t = av_image_get_linesize(
        pix_fmt,
        w as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ) as int64_t;
    if stride <= 0 as int64_t {
        stride = (8 as ::core::ffi::c_longlong * w as ::core::ffi::c_longlong)
            as int64_t;
    }
    stride += (128 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as int64_t;
    if w == 0 as ::core::ffi::c_uint || h == 0 as ::core::ffi::c_uint
        || w > 0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint
        || h > 0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint
        || stride >= 2147483647 as int64_t
        || (stride as ::core::ffi::c_ulonglong)
            .wrapping_mul(
                (h as ::core::ffi::c_ulonglong)
                    .wrapping_add(128 as ::core::ffi::c_ulonglong),
            ) >= 2147483647 as ::core::ffi::c_ulonglong
    {
        av_log(
            &raw mut imgutils as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"Picture size %ux%u is invalid\n\0" as *const u8
                as *const ::core::ffi::c_char,
            w,
            h,
        );
        return -(22 as ::core::ffi::c_int);
    }
    if max_pixels < 0x7fffffffffffffff as ::core::ffi::c_longlong {
        if w as int64_t * h as int64_t > max_pixels {
            av_log(
                &raw mut imgutils as *mut ::core::ffi::c_void,
                16 as ::core::ffi::c_int,
                b"Picture size %ux%u exceeds specified max pixel count %lld, see the documentation if you wish to increase it\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                w,
                h,
                max_pixels,
            );
            return -(22 as ::core::ffi::c_int);
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_image_check_size(
    mut w: ::core::ffi::c_uint,
    mut h: ::core::ffi::c_uint,
    mut log_offset: ::core::ffi::c_int,
    mut log_ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return av_image_check_size2(
        w,
        h,
        0x7fffffffffffffff as int64_t,
        AV_PIX_FMT_NONE,
        log_offset,
        log_ctx,
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_image_check_sar(
    mut w: ::core::ffi::c_uint,
    mut h: ::core::ffi::c_uint,
    mut sar: AVRational,
) -> ::core::ffi::c_int {
    let mut scaled_dim: int64_t = 0;
    if sar.den <= 0 as ::core::ffi::c_int || sar.num < 0 as ::core::ffi::c_int {
        return -(22 as ::core::ffi::c_int);
    }
    if sar.num == 0 || sar.num == sar.den {
        return 0 as ::core::ffi::c_int;
    }
    if sar.num < sar.den {
        scaled_dim = av_rescale_rnd(
            w as int64_t,
            sar.num as int64_t,
            sar.den as int64_t,
            AV_ROUND_ZERO,
        );
    } else {
        scaled_dim = av_rescale_rnd(
            h as int64_t,
            sar.den as int64_t,
            sar.num as int64_t,
            AV_ROUND_ZERO,
        );
    }
    if scaled_dim > 0 as int64_t {
        return 0 as ::core::ffi::c_int;
    }
    return -(22 as ::core::ffi::c_int);
}
unsafe extern "C" fn image_copy_plane(
    mut dst: *mut uint8_t,
    mut dst_linesize: ptrdiff_t,
    mut src: *const uint8_t,
    mut src_linesize: ptrdiff_t,
    mut bytewidth: ptrdiff_t,
    mut height: ::core::ffi::c_int,
) {
    if dst.is_null() || src.is_null() {
        return;
    }
    if !((if src_linesize >= 0 as ::core::ffi::c_int {
        src_linesize
    } else {
        -src_linesize
    }) >= bytewidth)
    {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"((src_linesize) >= 0 ? (src_linesize) : (-(src_linesize))) >= bytewidth\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"src/libavutil/imgutils.c\0" as *const u8 as *const ::core::ffi::c_char,
            350 as ::core::ffi::c_int,
        );
        abort();
    }
    if !((if dst_linesize >= 0 as ::core::ffi::c_int {
        dst_linesize
    } else {
        -dst_linesize
    }) >= bytewidth)
    {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"((dst_linesize) >= 0 ? (dst_linesize) : (-(dst_linesize))) >= bytewidth\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"src/libavutil/imgutils.c\0" as *const u8 as *const ::core::ffi::c_char,
            351 as ::core::ffi::c_int,
        );
        abort();
    }
    while height > 0 as ::core::ffi::c_int {
        memcpy(
            dst as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            bytewidth as size_t,
        );
        dst = dst.offset(dst_linesize as isize);
        src = src.offset(src_linesize as isize);
        height -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn av_image_copy_plane_uc_from(
    mut dst: *mut uint8_t,
    mut dst_linesize: ptrdiff_t,
    mut src: *const uint8_t,
    mut src_linesize: ptrdiff_t,
    mut bytewidth: ptrdiff_t,
    mut height: ::core::ffi::c_int,
) {
    let mut ret: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if ret < 0 as ::core::ffi::c_int {
        image_copy_plane(dst, dst_linesize, src, src_linesize, bytewidth, height);
    }
}
#[no_mangle]
pub unsafe extern "C" fn av_image_copy_plane(
    mut dst: *mut uint8_t,
    mut dst_linesize: ::core::ffi::c_int,
    mut src: *const uint8_t,
    mut src_linesize: ::core::ffi::c_int,
    mut bytewidth: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) {
    image_copy_plane(
        dst,
        dst_linesize as ptrdiff_t,
        src,
        src_linesize as ptrdiff_t,
        bytewidth as ptrdiff_t,
        height,
    );
}
unsafe extern "C" fn image_copy(
    mut dst_data: *const *mut uint8_t,
    mut dst_linesizes: *const ptrdiff_t,
    mut src_data: *const *const uint8_t,
    mut src_linesizes: *const ptrdiff_t,
    mut pix_fmt: AVPixelFormat,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut copy_plane: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            *const uint8_t,
            ptrdiff_t,
            ptrdiff_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
) {
    let mut desc: *const AVPixFmtDescriptor = av_pix_fmt_desc_get(pix_fmt);
    if desc.is_null()
        || (*desc).flags
            & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint64_t != 0
    {
        return;
    }
    if (*desc).flags & ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint64_t
        != 0
    {
        copy_plane
            .expect(
                "non-null function pointer",
            )(
            *dst_data.offset(0 as ::core::ffi::c_int as isize),
            *dst_linesizes.offset(0 as ::core::ffi::c_int as isize),
            *src_data.offset(0 as ::core::ffi::c_int as isize),
            *src_linesizes.offset(0 as ::core::ffi::c_int as isize),
            width as ptrdiff_t,
            height,
        );
        if (*desc).flags
            & ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint64_t != 0
            || !(*dst_data.offset(1 as ::core::ffi::c_int as isize)).is_null()
                && !(*src_data.offset(1 as ::core::ffi::c_int as isize)).is_null()
        {
            memcpy(
                *dst_data.offset(1 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_void,
                *src_data.offset(1 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_void,
                (4 as ::core::ffi::c_int * 256 as ::core::ffi::c_int) as size_t,
            );
        }
    } else {
        let mut i: ::core::ffi::c_int = 0;
        let mut planes_nb: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < (*desc).nb_components as ::core::ffi::c_int {
            planes_nb = if planes_nb
                > (*desc).comp[i as usize].plane + 1 as ::core::ffi::c_int
            {
                planes_nb
            } else {
                (*desc).comp[i as usize].plane + 1 as ::core::ffi::c_int
            };
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < planes_nb {
            let mut h: ::core::ffi::c_int = height;
            let mut bwidth: ptrdiff_t = av_image_get_linesize(pix_fmt, width, i)
                as ptrdiff_t;
            if bwidth < 0 as ::core::ffi::c_int {
                av_log(
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    16 as ::core::ffi::c_int,
                    b"av_image_get_linesize failed\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return;
            }
            if i == 1 as ::core::ffi::c_int || i == 2 as ::core::ffi::c_int {
                h = if 0 == 0 {
                    -(-height >> (*desc).log2_chroma_h as ::core::ffi::c_int)
                } else {
                    height
                        + ((1 as ::core::ffi::c_int)
                            << (*desc).log2_chroma_h as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int
                        >> (*desc).log2_chroma_h as ::core::ffi::c_int
                };
            }
            copy_plane
                .expect(
                    "non-null function pointer",
                )(
                *dst_data.offset(i as isize),
                *dst_linesizes.offset(i as isize),
                *src_data.offset(i as isize),
                *src_linesizes.offset(i as isize),
                bwidth,
                h,
            );
            i += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_image_copy(
    mut dst_data: *const *mut uint8_t,
    mut dst_linesizes: *const ::core::ffi::c_int,
    mut src_data: *const *const uint8_t,
    mut src_linesizes: *const ::core::ffi::c_int,
    mut pix_fmt: AVPixelFormat,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) {
    let mut dst_linesizes1: [ptrdiff_t; 4] = [0; 4];
    let mut src_linesizes1: [ptrdiff_t; 4] = [0; 4];
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        dst_linesizes1[i as usize] = *dst_linesizes.offset(i as isize) as ptrdiff_t;
        src_linesizes1[i as usize] = *src_linesizes.offset(i as isize) as ptrdiff_t;
        i += 1;
    }
    image_copy(
        dst_data,
        &raw mut dst_linesizes1 as *mut ptrdiff_t as *const ptrdiff_t,
        src_data,
        &raw mut src_linesizes1 as *mut ptrdiff_t as *const ptrdiff_t,
        pix_fmt,
        width,
        height,
        Some(
            image_copy_plane
                as unsafe extern "C" fn(
                    *mut uint8_t,
                    ptrdiff_t,
                    *const uint8_t,
                    ptrdiff_t,
                    ptrdiff_t,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_image_copy_uc_from(
    mut dst_data: *const *mut uint8_t,
    mut dst_linesizes: *const ptrdiff_t,
    mut src_data: *const *const uint8_t,
    mut src_linesizes: *const ptrdiff_t,
    mut pix_fmt: AVPixelFormat,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) {
    image_copy(
        dst_data,
        dst_linesizes,
        src_data,
        src_linesizes,
        pix_fmt,
        width,
        height,
        Some(
            av_image_copy_plane_uc_from
                as unsafe extern "C" fn(
                    *mut uint8_t,
                    ptrdiff_t,
                    *const uint8_t,
                    ptrdiff_t,
                    ptrdiff_t,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_image_fill_arrays(
    mut dst_data: *mut *mut uint8_t,
    mut dst_linesize: *mut ::core::ffi::c_int,
    mut src: *const uint8_t,
    mut pix_fmt: AVPixelFormat,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut align: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    ret = av_image_check_size(
        width as ::core::ffi::c_uint,
        height as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    ret = av_image_fill_linesizes(dst_linesize, pix_fmt, width);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        *dst_linesize.offset(i as isize) = *dst_linesize.offset(i as isize) + align
            - 1 as ::core::ffi::c_int & !(align - 1 as ::core::ffi::c_int);
        i += 1;
    }
    return av_image_fill_pointers(
        dst_data,
        pix_fmt,
        height,
        src as *mut uint8_t,
        dst_linesize as *const ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_image_get_buffer_size(
    mut pix_fmt: AVPixelFormat,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut align: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut linesize: [::core::ffi::c_int; 4] = [0; 4];
    let mut aligned_linesize: [ptrdiff_t; 4] = [0; 4];
    let mut sizes: [size_t; 4] = [0; 4];
    let mut desc: *const AVPixFmtDescriptor = av_pix_fmt_desc_get(pix_fmt);
    if desc.is_null() {
        return -(22 as ::core::ffi::c_int);
    }
    ret = av_image_check_size(
        width as ::core::ffi::c_uint,
        height as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    ret = av_image_fill_linesizes(
        &raw mut linesize as *mut ::core::ffi::c_int,
        pix_fmt,
        width,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        aligned_linesize[i as usize] = (linesize[i as usize] + align
            - 1 as ::core::ffi::c_int & !(align - 1 as ::core::ffi::c_int)) as ptrdiff_t;
        i += 1;
    }
    ret = av_image_fill_plane_sizes(
        &raw mut sizes as *mut size_t,
        pix_fmt,
        height,
        &raw mut aligned_linesize as *mut ptrdiff_t as *const ptrdiff_t,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    ret = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        if sizes[i as usize] > (2147483647 as ::core::ffi::c_int - ret) as size_t {
            return -(22 as ::core::ffi::c_int);
        }
        ret = (ret as size_t).wrapping_add(sizes[i as usize]) as ::core::ffi::c_int
            as ::core::ffi::c_int;
        i += 1;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_image_copy_to_buffer(
    mut dst: *mut uint8_t,
    mut dst_size: ::core::ffi::c_int,
    mut src_data: *const *const uint8_t,
    mut src_linesize: *const ::core::ffi::c_int,
    mut pix_fmt: AVPixelFormat,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut align: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut nb_planes: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut linesize: [::core::ffi::c_int; 4] = [0; 4];
    let mut size: ::core::ffi::c_int = av_image_get_buffer_size(
        pix_fmt,
        width,
        height,
        align,
    );
    let mut desc: *const AVPixFmtDescriptor = av_pix_fmt_desc_get(pix_fmt);
    let mut ret: ::core::ffi::c_int = 0;
    if size > dst_size || size < 0 as ::core::ffi::c_int || desc.is_null() {
        return -(22 as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*desc).nb_components as ::core::ffi::c_int {
        nb_planes = if (*desc).comp[i as usize].plane > nb_planes {
            (*desc).comp[i as usize].plane
        } else {
            nb_planes
        };
        i += 1;
    }
    nb_planes += 1;
    ret = av_image_fill_linesizes(
        &raw mut linesize as *mut ::core::ffi::c_int,
        pix_fmt,
        width,
    );
    if !(ret >= 0 as ::core::ffi::c_int) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"ret >= 0\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavutil/imgutils.c\0" as *const u8 as *const ::core::ffi::c_char,
            521 as ::core::ffi::c_int,
        );
        abort();
    }
    i = 0 as ::core::ffi::c_int;
    while i < nb_planes {
        let mut h: ::core::ffi::c_int = 0;
        let mut shift: ::core::ffi::c_int = if i == 1 as ::core::ffi::c_int
            || i == 2 as ::core::ffi::c_int
        {
            (*desc).log2_chroma_h as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        let mut src: *const uint8_t = *src_data.offset(i as isize);
        h = if 0 == 0 {
            -(-height >> shift)
        } else {
            height + ((1 as ::core::ffi::c_int) << shift) - 1 as ::core::ffi::c_int
                >> shift
        };
        j = 0 as ::core::ffi::c_int;
        while j < h {
            memcpy(
                dst as *mut ::core::ffi::c_void,
                src as *const ::core::ffi::c_void,
                linesize[i as usize] as size_t,
            );
            dst = dst
                .offset(
                    (linesize[i as usize] + align - 1 as ::core::ffi::c_int
                        & !(align - 1 as ::core::ffi::c_int)) as isize,
                );
            src = src.offset(*src_linesize.offset(i as isize) as isize);
            j += 1;
        }
        i += 1;
    }
    if (*desc).flags & ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint64_t
        != 0
    {
        let mut d32: *mut uint32_t = dst as *mut uint32_t;
        i = 0 as ::core::ffi::c_int;
        while i < 256 as ::core::ffi::c_int {
            (*(d32.offset(i as isize) as *mut unaligned_32)).l = (*((*src_data
                .offset(1 as ::core::ffi::c_int as isize))
                .offset((4 as ::core::ffi::c_int * i) as isize) as *const unaligned_32))
                .l;
            i += 1;
        }
    }
    return size;
}
unsafe extern "C" fn memset_bytes(
    mut dst: *mut uint8_t,
    mut dst_size: size_t,
    mut clear: *mut uint8_t,
    mut clear_size: size_t,
) {
    let mut same: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    if clear_size == 0 {
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while (i as size_t) < clear_size {
        if *clear.offset(i as isize) as ::core::ffi::c_int
            != *clear.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        {
            same = 0 as ::core::ffi::c_int;
            break;
        } else {
            i += 1;
        }
    }
    if same != 0 {
        clear_size = 1 as size_t;
    }
    if clear_size == 1 as size_t {
        memset(
            dst as *mut ::core::ffi::c_void,
            *clear.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            dst_size as size_t,
        );
    } else {
        if clear_size > dst_size {
            clear_size = dst_size;
        }
        memcpy(
            dst as *mut ::core::ffi::c_void,
            clear as *const ::core::ffi::c_void,
            clear_size as size_t,
        );
        av_memcpy_backptr(
            dst.offset(clear_size as isize),
            clear_size as ::core::ffi::c_int,
            dst_size.wrapping_sub(clear_size) as ::core::ffi::c_int,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_image_fill_color(
    mut dst_data: *const *mut uint8_t,
    mut dst_linesize: *const ptrdiff_t,
    mut pix_fmt: AVPixelFormat,
    mut color: *const uint32_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut desc: *const AVPixFmtDescriptor = av_pix_fmt_desc_get(pix_fmt);
    let mut nb_planes: ::core::ffi::c_int = av_pix_fmt_count_planes(pix_fmt);
    let mut clear_block: [[uint8_t; 32]; 4] = [
        [
            0 as ::core::ffi::c_int as uint8_t,
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
        [0; 32],
        [0; 32],
        [0; 32],
    ];
    let mut clear_block_size: [::core::ffi::c_int; 4] = [0 as ::core::ffi::c_int; 4];
    let mut plane_line_bytes: [ptrdiff_t; 4] = [0 as ::core::ffi::c_int; 4];
    let mut bitstream: ::core::ffi::c_int = 0;
    let mut plane: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    if desc.is_null() || nb_planes < 1 as ::core::ffi::c_int
        || nb_planes > 4 as ::core::ffi::c_int
        || (*desc).flags
            & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint64_t != 0
    {
        return -(22 as ::core::ffi::c_int);
    }
    bitstream = ((*desc).flags
        & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as uint64_t != 0)
        as ::core::ffi::c_int;
    c = 0 as ::core::ffi::c_int;
    while c < (*desc).nb_components as ::core::ffi::c_int {
        let comp: AVComponentDescriptor = (*desc).comp[c as usize];
        clear_block_size[comp.plane as usize] = if clear_block_size[comp.plane as usize]
            > comp.step
        {
            clear_block_size[comp.plane as usize]
        } else {
            comp.step
        };
        if clear_block_size[comp.plane as usize] > 32 as ::core::ffi::c_int {
            return -(22 as ::core::ffi::c_int);
        }
        c += 1;
    }
    c = 0 as ::core::ffi::c_int;
    while c < (*desc).nb_components as ::core::ffi::c_int {
        let comp_0: AVComponentDescriptor = (*desc).comp[c as usize];
        let mut w: ::core::ffi::c_int = (if bitstream != 0 {
            8 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) * clear_block_size[comp_0.plane as usize] / comp_0.step;
        let mut c_data: [*mut uint8_t; 4] = [::core::ptr::null_mut::<uint8_t>(); 4];
        let c_linesize: [::core::ffi::c_int; 4] = [0 as ::core::ffi::c_int; 4];
        let mut src_array: [uint32_t; 32] = [0; 32];
        let mut x: ::core::ffi::c_int = 0;
        if comp_0.depth > 32 as ::core::ffi::c_int {
            return -(22 as ::core::ffi::c_int);
        }
        if w < 1 as ::core::ffi::c_int {
            return -(22 as ::core::ffi::c_int);
        }
        x = 0 as ::core::ffi::c_int;
        while x < w {
            src_array[x as usize] = *color.offset(c as isize);
            x += 1;
        }
        x = 0 as ::core::ffi::c_int;
        while x < 4 as ::core::ffi::c_int {
            c_data[x as usize] = (&raw mut *(&raw mut clear_block as *mut [uint8_t; 32])
                .offset(x as isize) as *mut uint8_t)
                .offset(0 as ::core::ffi::c_int as isize) as *mut uint8_t;
            x += 1;
        }
        av_write_image_line2(
            &raw mut src_array as *mut uint32_t as *const ::core::ffi::c_void,
            &raw mut c_data as *mut *mut uint8_t,
            &raw const c_linesize as *const ::core::ffi::c_int,
            desc,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            c,
            w,
            4 as ::core::ffi::c_int,
        );
        c += 1;
    }
    plane = 0 as ::core::ffi::c_int;
    while plane < nb_planes {
        plane_line_bytes[plane as usize] = av_image_get_linesize(pix_fmt, width, plane)
            as ptrdiff_t;
        if plane_line_bytes[plane as usize] < 0 as ::core::ffi::c_int {
            return -(22 as ::core::ffi::c_int);
        }
        plane += 1;
    }
    if dst_data.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    plane = 0 as ::core::ffi::c_int;
    while plane < nb_planes {
        let mut bytewidth: size_t = plane_line_bytes[plane as usize] as size_t;
        let mut data: *mut uint8_t = *dst_data.offset(plane as isize);
        let mut chroma_div: ::core::ffi::c_int = if plane == 1 as ::core::ffi::c_int
            || plane == 2 as ::core::ffi::c_int
        {
            (*desc).log2_chroma_h as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        let mut plane_h: ::core::ffi::c_int = if 0 == 0 {
            -(-height >> chroma_div)
        } else {
            height + ((1 as ::core::ffi::c_int) << chroma_div) - 1 as ::core::ffi::c_int
                >> chroma_div
        };
        while plane_h > 0 as ::core::ffi::c_int {
            memset_bytes(
                data,
                bytewidth,
                (&raw mut *(&raw mut clear_block as *mut [uint8_t; 32])
                    .offset(plane as isize) as *mut uint8_t)
                    .offset(0 as ::core::ffi::c_int as isize) as *mut uint8_t,
                clear_block_size[plane as usize] as size_t,
            );
            data = data.offset(*dst_linesize.offset(plane as isize) as isize);
            plane_h -= 1;
        }
        plane += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_image_fill_black(
    mut dst_data: *const *mut uint8_t,
    mut dst_linesize: *const ptrdiff_t,
    mut pix_fmt: AVPixelFormat,
    mut range: AVColorRange,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut desc: *const AVPixFmtDescriptor = av_pix_fmt_desc_get(pix_fmt);
    let mut nb_planes: ::core::ffi::c_int = av_pix_fmt_count_planes(pix_fmt);
    let mut rgb: ::core::ffi::c_int = 0;
    let mut xyz: ::core::ffi::c_int = 0;
    let mut pal: ::core::ffi::c_int = 0;
    let mut limited: ::core::ffi::c_int = 0;
    let mut alpha: ::core::ffi::c_int = 0;
    let mut fltp: ::core::ffi::c_int = 0;
    let mut colors: [uint32_t; 4] = [0 as ::core::ffi::c_int as uint32_t, 0, 0, 0];
    if desc.is_null() || nb_planes < 1 as ::core::ffi::c_int
        || nb_planes > 4 as ::core::ffi::c_int
        || (*desc).flags
            & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint64_t != 0
    {
        return -(22 as ::core::ffi::c_int);
    }
    rgb = ((*desc).flags
        & ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t != 0)
        as ::core::ffi::c_int;
    xyz = ((*desc).flags
        & ((1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int) as uint64_t != 0)
        as ::core::ffi::c_int;
    pal = ((*desc).flags
        & ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint64_t != 0)
        as ::core::ffi::c_int;
    limited = (rgb == 0 && xyz == 0 && pal == 0
        && range as ::core::ffi::c_uint
            != AVCOL_RANGE_JPEG as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
    alpha = (pal == 0
        && (*desc).flags
            & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t != 0)
        as ::core::ffi::c_int;
    fltp = ((*desc).flags
        & ((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as uint64_t != 0)
        as ::core::ffi::c_int;
    let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while c < (*desc).nb_components as ::core::ffi::c_int {
        let comp: AVComponentDescriptor = (*desc).comp[c as usize];
        let mut color: uint32_t = 0 as uint32_t;
        if comp.depth > 32 as ::core::ffi::c_int {
            return -(22 as ::core::ffi::c_int);
        }
        if pix_fmt as ::core::ffi::c_int == AV_PIX_FMT_MONOWHITE as ::core::ffi::c_int {
            color = 1 as uint32_t;
        } else if c + 1 as ::core::ffi::c_int
            == (*desc).nb_components as ::core::ffi::c_int && alpha != 0
        {
            if fltp != 0 {
                if comp.depth != 16 as ::core::ffi::c_int
                    && comp.depth != 32 as ::core::ffi::c_int
                {
                    return -(22 as ::core::ffi::c_int);
                }
                color = (if comp.depth == 16 as ::core::ffi::c_int {
                    0x3c00 as ::core::ffi::c_int
                } else {
                    0x3f800000 as ::core::ffi::c_int
                }) as uint32_t;
            } else {
                color = ((if comp.depth == 32 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int
                } else {
                    (1 as ::core::ffi::c_int) << comp.depth
                }) - 1 as ::core::ffi::c_int) as uint32_t;
            }
        } else if c == 0 as ::core::ffi::c_int && limited != 0
            && comp.depth > 1 as ::core::ffi::c_int
        {
            if comp.depth < 8 as ::core::ffi::c_int
                || fltp != 0 && comp.depth != 16 as ::core::ffi::c_int
                    && comp.depth != 32 as ::core::ffi::c_int
            {
                return -(22 as ::core::ffi::c_int);
            }
            if fltp != 0 {
                color = (if comp.depth == 16 as ::core::ffi::c_int {
                    0x2c00 as ::core::ffi::c_int
                } else {
                    0x3d800000 as ::core::ffi::c_int
                }) as uint32_t;
            } else {
                color = ((16 as ::core::ffi::c_int)
                    << comp.depth - 8 as ::core::ffi::c_int) as uint32_t;
            }
        } else if (c == 1 as ::core::ffi::c_int || c == 2 as ::core::ffi::c_int)
            && rgb == 0 && xyz == 0
        {
            if comp.depth < 8 as ::core::ffi::c_int
                || fltp != 0 && comp.depth != 16 as ::core::ffi::c_int
                    && comp.depth != 32 as ::core::ffi::c_int
            {
                return -(22 as ::core::ffi::c_int);
            }
            if fltp != 0 {
                color = (if comp.depth == 16 as ::core::ffi::c_int {
                    0x3800 as ::core::ffi::c_int
                } else {
                    0x3f000000 as ::core::ffi::c_int
                }) as uint32_t;
            } else {
                color = ((128 as ::core::ffi::c_int)
                    << comp.depth - 8 as ::core::ffi::c_int) as uint32_t;
            }
        }
        colors[c as usize] = color;
        c += 1;
    }
    return av_image_fill_color(
        dst_data,
        dst_linesize,
        pix_fmt,
        &raw mut colors as *mut uint32_t as *const uint32_t,
        width,
        height,
        0 as ::core::ffi::c_int,
    );
}
#[repr(C)]
pub struct AVOptionRanges {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVOption {
    _opaque: [u8; 0],
}
