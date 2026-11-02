// Translated from FFmpeg 9.0.1 libavcodec/aom_film_grain.c
// (preprocessed for aarch64-unknown-linux-musl) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// AOM film grain synthesis
// Copyright (c) 2023 Niklas Haas <ffmpeg@haasn.xyz>
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
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn abort() -> !;
    fn av_buffer_create(
        data: *mut uint8_t,
        size: size_t,
        free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint8_t) -> ()>,
        opaque: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_int,
    ) -> *mut AVBufferRef;
    fn av_image_copy_plane(
        dst: *mut uint8_t,
        dst_linesize: ::core::ffi::c_int,
        src: *const uint8_t,
        src_linesize: ::core::ffi::c_int,
        bytewidth: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    );
    fn av_buffer_unref(buf: *mut *mut AVBufferRef);
    fn av_pix_fmt_desc_get(pix_fmt: AVPixelFormat) -> *const AVPixFmtDescriptor;
    fn av_film_grain_params_alloc(size: *mut size_t) -> *mut AVFilmGrainParams;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn av_free(ptr: *mut ::core::ffi::c_void);
    fn av_freep(ptr: *mut ::core::ffi::c_void);
    fn av_frame_side_data_add(
        sd: *mut *mut *mut AVFrameSideData,
        nb_sd: *mut ::core::ffi::c_int,
        type_0: AVFrameSideDataType,
        buf: *mut *mut AVBufferRef,
        flags: ::core::ffi::c_uint,
    ) -> *mut AVFrameSideData;
}
pub type size_t = ::core::ffi::c_ulong;
pub type ptrdiff_t = ::core::ffi::c_long;
pub type intptr_t = ::core::ffi::c_long;
pub type int8_t = ::core::ffi::c_schar;
pub type int16_t = ::core::ffi::c_short;
pub type int64_t = ::core::ffi::c_long;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint16_t = ::core::ffi::c_ushort;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint64_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBufferRef {
    pub buffer: *mut AVBuffer,
    pub data: *mut uint8_t,
    pub size: size_t,
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
pub type AVPictureType = ::core::ffi::c_uint;
pub const AV_PICTURE_TYPE_BI: AVPictureType = 7;
pub const AV_PICTURE_TYPE_SP: AVPictureType = 6;
pub const AV_PICTURE_TYPE_SI: AVPictureType = 5;
pub const AV_PICTURE_TYPE_S: AVPictureType = 4;
pub const AV_PICTURE_TYPE_B: AVPictureType = 3;
pub const AV_PICTURE_TYPE_P: AVPictureType = 2;
pub const AV_PICTURE_TYPE_I: AVPictureType = 1;
pub const AV_PICTURE_TYPE_NONE: AVPictureType = 0;
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
pub type AVFilmGrainParamsType = ::core::ffi::c_uint;
pub const AV_FILM_GRAIN_PARAMS_H274: AVFilmGrainParamsType = 2;
pub const AV_FILM_GRAIN_PARAMS_AV1: AVFilmGrainParamsType = 1;
pub const AV_FILM_GRAIN_PARAMS_NONE: AVFilmGrainParamsType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFilmGrainAOMParams {
    pub num_y_points: ::core::ffi::c_int,
    pub y_points: [[uint8_t; 2]; 14],
    pub chroma_scaling_from_luma: ::core::ffi::c_int,
    pub num_uv_points: [::core::ffi::c_int; 2],
    pub uv_points: [[[uint8_t; 2]; 10]; 2],
    pub scaling_shift: ::core::ffi::c_int,
    pub ar_coeff_lag: ::core::ffi::c_int,
    pub ar_coeffs_y: [int8_t; 24],
    pub ar_coeffs_uv: [[int8_t; 25]; 2],
    pub ar_coeff_shift: ::core::ffi::c_int,
    pub grain_scale_shift: ::core::ffi::c_int,
    pub uv_mult: [::core::ffi::c_int; 2],
    pub uv_mult_luma: [::core::ffi::c_int; 2],
    pub uv_offset: [::core::ffi::c_int; 2],
    pub overlap_flag: ::core::ffi::c_int,
    pub limit_output_range: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFilmGrainH274Params {
    pub model_id: ::core::ffi::c_int,
    pub blending_mode_id: ::core::ffi::c_int,
    pub log2_scale_factor: ::core::ffi::c_int,
    pub component_model_present: [::core::ffi::c_int; 3],
    pub num_intensity_intervals: [uint16_t; 3],
    pub num_model_values: [uint8_t; 3],
    pub intensity_interval_lower_bound: [[uint8_t; 256]; 3],
    pub intensity_interval_upper_bound: [[uint8_t; 256]; 3],
    pub comp_model_value: [[[int16_t; 6]; 256]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFilmGrainParams {
    pub type_0: AVFilmGrainParamsType,
    pub seed: uint64_t,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub subsampling_x: ::core::ffi::c_int,
    pub subsampling_y: ::core::ffi::c_int,
    pub color_range: AVColorRange,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub color_space: AVColorSpace,
    pub bit_depth_luma: ::core::ffi::c_int,
    pub bit_depth_chroma: ::core::ffi::c_int,
    pub codec: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub aom: AVFilmGrainAOMParams,
    pub h274: AVFilmGrainH274Params,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFilmGrainAFGS1Params {
    pub enable: ::core::ffi::c_int,
    pub sets: [*mut AVBufferRef; 8],
}
pub const FG_BLOCK_SIZE: C2RustUnnamed_1 = 32;
pub const GRAIN_WIDTH: C2RustUnnamed_1 = 82;
pub const SUB_GRAIN_WIDTH: C2RustUnnamed_1 = 44;
pub const GRAIN_HEIGHT: C2RustUnnamed_1 = 73;
pub const SUB_GRAIN_HEIGHT: C2RustUnnamed_1 = 38;
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
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn get_random_number(
    bits: ::core::ffi::c_int,
    state: *mut ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let r: ::core::ffi::c_int = *state as ::core::ffi::c_int;
    let mut bit: ::core::ffi::c_uint = ((r >> 0 as ::core::ffi::c_int
        ^ r >> 1 as ::core::ffi::c_int ^ r >> 3 as ::core::ffi::c_int
        ^ r >> 12 as ::core::ffi::c_int) & 1 as ::core::ffi::c_int)
        as ::core::ffi::c_uint;
    *state = (r >> 1 as ::core::ffi::c_int) as ::core::ffi::c_uint
        | bit << 15 as ::core::ffi::c_int;
    return av_zero_extend_c(
        *state >> 16 as ::core::ffi::c_int - bits,
        bits as ::core::ffi::c_uint,
    ) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn round2(
    x: ::core::ffi::c_int,
    shift: uint64_t,
) -> ::core::ffi::c_int {
    return x + ((1 as ::core::ffi::c_int) << shift >> 1 as ::core::ffi::c_int) >> shift;
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
pub unsafe extern "C" fn ff_aom_apply_film_grain(
    mut out: *mut AVFrame,
    mut in_0: *const AVFrame,
    mut params: *const AVFilmGrainParams,
) -> ::core::ffi::c_int {
    let data: *const AVFilmGrainAOMParams = &raw const (*params).codec.aom;
    let mut desc: *const AVPixFmtDescriptor = av_pix_fmt_desc_get(
        (*out).format as AVPixelFormat,
    );
    let subx: ::core::ffi::c_int = (*desc).log2_chroma_w as ::core::ffi::c_int;
    let suby: ::core::ffi::c_int = (*desc).log2_chroma_h as ::core::ffi::c_int;
    let pxstep: ::core::ffi::c_int = (*desc).comp[0 as ::core::ffi::c_int as usize].step;
    if !((*out).format == (*in_0).format) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"out->format == in->format\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavcodec/aom_film_grain.c\0" as *const u8
                as *const ::core::ffi::c_char,
            76 as ::core::ffi::c_int,
        );
        abort();
    }
    if !((*params).type_0 as ::core::ffi::c_uint
        == AV_FILM_GRAIN_PARAMS_AV1 as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"params->type == AV_FILM_GRAIN_PARAMS_AV1\0" as *const u8
                as *const ::core::ffi::c_char,
            b"src/libavcodec/aom_film_grain.c\0" as *const u8
                as *const ::core::ffi::c_char,
            77 as ::core::ffi::c_int,
        );
        abort();
    }
    if (*params).codec.aom.num_y_points == 0 {
        av_image_copy_plane(
            (*out).data[0 as ::core::ffi::c_int as usize],
            (*out).linesize[0 as ::core::ffi::c_int as usize],
            (*in_0).data[0 as ::core::ffi::c_int as usize],
            (*in_0).linesize[0 as ::core::ffi::c_int as usize],
            (*out).width * pxstep,
            (*out).height,
        );
    }
    let mut uv: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while uv < 2 as ::core::ffi::c_int {
        if (*data).num_uv_points[uv as usize] == 0 {
            av_image_copy_plane(
                (*out).data[(1 as ::core::ffi::c_int + uv) as usize],
                (*out).linesize[(1 as ::core::ffi::c_int + uv) as usize],
                (*in_0).data[(1 as ::core::ffi::c_int + uv) as usize],
                (*in_0).linesize[(1 as ::core::ffi::c_int + uv) as usize],
                (if 0 == 0 {
                    -(-(*out).width >> subx)
                } else {
                    (*out).width + ((1 as ::core::ffi::c_int) << subx)
                        - 1 as ::core::ffi::c_int >> subx
                }) * pxstep,
                if 0 == 0 {
                    -(-(*out).height >> suby)
                } else {
                    (*out).height + ((1 as ::core::ffi::c_int) << suby)
                        - 1 as ::core::ffi::c_int >> suby
                },
            );
        }
        uv += 1;
    }
    match (*in_0).format {
        8 | 0 | 4 | 5 | 12 | 13 | 14 => return apply_film_grain_8(out, in_0, params),
        173 | 60 | 70 | 66 => {
            return apply_film_grain_16(out, in_0, params, 9 as ::core::ffi::c_int);
        }
        168 | 62 | 64 | 68 => {
            return apply_film_grain_16(out, in_0, params, 10 as ::core::ffi::c_int);
        }
        166 | 123 | 127 | 131 => {
            return apply_film_grain_16(out, in_0, params, 12 as ::core::ffi::c_int);
        }
        _ => {}
    }
    return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
        | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
        | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
        as ::core::ffi::c_int);
}
unsafe extern "C" fn generate_grain_y_c_16(
    mut buf: *mut [int16_t; 82],
    params: *const AVFilmGrainParams,
    bitdepth: ::core::ffi::c_int,
) {
    let data: *const AVFilmGrainAOMParams = &raw const (*params).codec.aom;
    let bitdepth_min_8: ::core::ffi::c_int = bitdepth - 8 as ::core::ffi::c_int;
    let mut seed: ::core::ffi::c_uint = (*params).seed as ::core::ffi::c_uint;
    let shift: ::core::ffi::c_int = 4 as ::core::ffi::c_int - bitdepth_min_8
        + (*data).grain_scale_shift;
    let grain_ctr: ::core::ffi::c_int = (128 as ::core::ffi::c_int) << bitdepth_min_8;
    let grain_min: ::core::ffi::c_int = -grain_ctr;
    let grain_max: ::core::ffi::c_int = grain_ctr - 1 as ::core::ffi::c_int;
    let ar_pad: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    let ar_lag: ::core::ffi::c_int = (*data).ar_coeff_lag;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < GRAIN_HEIGHT as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < GRAIN_WIDTH as ::core::ffi::c_int {
            let value: ::core::ffi::c_int = get_random_number(
                11 as ::core::ffi::c_int,
                &raw mut seed,
            ) as ::core::ffi::c_int;
            (*buf.offset(y as isize))[x as usize] = round2(
                gaussian_sequence[value as usize] as ::core::ffi::c_int,
                shift as uint64_t,
            ) as int16_t;
            x += 1;
        }
        y += 1;
    }
    let mut y_0: ::core::ffi::c_int = ar_pad;
    while y_0 < GRAIN_HEIGHT as ::core::ffi::c_int {
        let mut x_0: ::core::ffi::c_int = ar_pad;
        while x_0 < GRAIN_WIDTH as ::core::ffi::c_int - ar_pad {
            let mut coeff: *const int8_t = &raw const (*data).ar_coeffs_y
                as *const int8_t;
            let mut sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut grain: ::core::ffi::c_int = 0;
            let mut dy: ::core::ffi::c_int = -ar_lag;
            while dy <= 0 as ::core::ffi::c_int {
                let mut dx: ::core::ffi::c_int = -ar_lag;
                while dx <= ar_lag {
                    if dx == 0 && dy == 0 {
                        break;
                    }
                    let fresh1 = coeff;
                    coeff = coeff.offset(1);
                    sum
                        += *fresh1 as ::core::ffi::c_int
                            * (*buf.offset((y_0 + dy) as isize))[(x_0 + dx) as usize]
                                as ::core::ffi::c_int;
                    dx += 1;
                }
                dy += 1;
            }
            grain = (*buf.offset(y_0 as isize))[x_0 as usize] as ::core::ffi::c_int
                + round2(sum, (*data).ar_coeff_shift as uint64_t);
            (*buf.offset(y_0 as isize))[x_0 as usize] = av_clip_c(
                grain,
                grain_min,
                grain_max,
            ) as int16_t;
            x_0 += 1;
        }
        y_0 += 1;
    }
}
unsafe extern "C" fn generate_grain_y_c_8(
    mut buf: *mut [int8_t; 82],
    params: *const AVFilmGrainParams,
) {
    let data: *const AVFilmGrainAOMParams = &raw const (*params).codec.aom;
    let bitdepth_min_8: ::core::ffi::c_int = 8 as ::core::ffi::c_int
        - 8 as ::core::ffi::c_int;
    let mut seed: ::core::ffi::c_uint = (*params).seed as ::core::ffi::c_uint;
    let shift: ::core::ffi::c_int = 4 as ::core::ffi::c_int - bitdepth_min_8
        + (*data).grain_scale_shift;
    let grain_ctr: ::core::ffi::c_int = (128 as ::core::ffi::c_int) << bitdepth_min_8;
    let grain_min: ::core::ffi::c_int = -grain_ctr;
    let grain_max: ::core::ffi::c_int = grain_ctr - 1 as ::core::ffi::c_int;
    let ar_pad: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    let ar_lag: ::core::ffi::c_int = (*data).ar_coeff_lag;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < GRAIN_HEIGHT as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < GRAIN_WIDTH as ::core::ffi::c_int {
            let value: ::core::ffi::c_int = get_random_number(
                11 as ::core::ffi::c_int,
                &raw mut seed,
            ) as ::core::ffi::c_int;
            (*buf.offset(y as isize))[x as usize] = round2(
                gaussian_sequence[value as usize] as ::core::ffi::c_int,
                shift as uint64_t,
            ) as int8_t;
            x += 1;
        }
        y += 1;
    }
    let mut y_0: ::core::ffi::c_int = ar_pad;
    while y_0 < GRAIN_HEIGHT as ::core::ffi::c_int {
        let mut x_0: ::core::ffi::c_int = ar_pad;
        while x_0 < GRAIN_WIDTH as ::core::ffi::c_int - ar_pad {
            let mut coeff: *const int8_t = &raw const (*data).ar_coeffs_y
                as *const int8_t;
            let mut sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut grain: ::core::ffi::c_int = 0;
            let mut dy: ::core::ffi::c_int = -ar_lag;
            while dy <= 0 as ::core::ffi::c_int {
                let mut dx: ::core::ffi::c_int = -ar_lag;
                while dx <= ar_lag {
                    if dx == 0 && dy == 0 {
                        break;
                    }
                    let fresh3 = coeff;
                    coeff = coeff.offset(1);
                    sum
                        += *fresh3 as ::core::ffi::c_int
                            * (*buf.offset((y_0 + dy) as isize))[(x_0 + dx) as usize]
                                as ::core::ffi::c_int;
                    dx += 1;
                }
                dy += 1;
            }
            grain = (*buf.offset(y_0 as isize))[x_0 as usize] as ::core::ffi::c_int
                + round2(sum, (*data).ar_coeff_shift as uint64_t);
            (*buf.offset(y_0 as isize))[x_0 as usize] = av_clip_c(
                grain,
                grain_min,
                grain_max,
            ) as int8_t;
            x_0 += 1;
        }
        y_0 += 1;
    }
}
unsafe extern "C" fn generate_grain_uv_c_16(
    mut buf: *mut [int16_t; 82],
    mut buf_y: *const [int16_t; 82],
    params: *const AVFilmGrainParams,
    uv: intptr_t,
    subx: ::core::ffi::c_int,
    suby: ::core::ffi::c_int,
    bitdepth: ::core::ffi::c_int,
) {
    let data: *const AVFilmGrainAOMParams = &raw const (*params).codec.aom;
    let bitdepth_min_8: ::core::ffi::c_int = bitdepth - 8 as ::core::ffi::c_int;
    let mut seed: ::core::ffi::c_uint = ((*params).seed
        ^ (if uv != 0 {
            0x49d8 as ::core::ffi::c_int
        } else {
            0xb524 as ::core::ffi::c_int
        }) as uint64_t) as ::core::ffi::c_uint;
    let shift: ::core::ffi::c_int = 4 as ::core::ffi::c_int - bitdepth_min_8
        + (*data).grain_scale_shift;
    let grain_ctr: ::core::ffi::c_int = (128 as ::core::ffi::c_int) << bitdepth_min_8;
    let grain_min: ::core::ffi::c_int = -grain_ctr;
    let grain_max: ::core::ffi::c_int = grain_ctr - 1 as ::core::ffi::c_int;
    let chromaW: ::core::ffi::c_int = if subx != 0 {
        SUB_GRAIN_WIDTH as ::core::ffi::c_int as ::core::ffi::c_int
    } else {
        GRAIN_WIDTH as ::core::ffi::c_int as ::core::ffi::c_int
    };
    let chromaH: ::core::ffi::c_int = if suby != 0 {
        SUB_GRAIN_HEIGHT as ::core::ffi::c_int as ::core::ffi::c_int
    } else {
        GRAIN_HEIGHT as ::core::ffi::c_int as ::core::ffi::c_int
    };
    let ar_pad: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    let ar_lag: ::core::ffi::c_int = (*data).ar_coeff_lag;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < chromaH {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < chromaW {
            let value: ::core::ffi::c_int = get_random_number(
                11 as ::core::ffi::c_int,
                &raw mut seed,
            ) as ::core::ffi::c_int;
            (*buf.offset(y as isize))[x as usize] = round2(
                gaussian_sequence[value as usize] as ::core::ffi::c_int,
                shift as uint64_t,
            ) as int16_t;
            x += 1;
        }
        y += 1;
    }
    let mut y_0: ::core::ffi::c_int = ar_pad;
    while y_0 < chromaH {
        let mut x_0: ::core::ffi::c_int = ar_pad;
        while x_0 < chromaW - ar_pad {
            let mut coeff: *const int8_t = &raw const *(&raw const (*data).ar_coeffs_uv
                as *const [int8_t; 25])
                .offset(uv as isize) as *const int8_t;
            let mut sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut grain: ::core::ffi::c_int = 0;
            let mut dy: ::core::ffi::c_int = -ar_lag;
            while dy <= 0 as ::core::ffi::c_int {
                let mut dx: ::core::ffi::c_int = -ar_lag;
                while dx <= ar_lag {
                    if dx == 0 && dy == 0 {
                        let lumaX: ::core::ffi::c_int = (x_0 - ar_pad << subx) + ar_pad;
                        let lumaY: ::core::ffi::c_int = (y_0 - ar_pad << suby) + ar_pad;
                        let mut luma: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        if (*data).num_y_points == 0 {
                            break;
                        }
                        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i <= suby {
                            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while j <= subx {
                                luma
                                    += (*buf_y
                                        .offset((lumaY + i) as isize))[(lumaX + j) as usize]
                                        as ::core::ffi::c_int;
                                j += 1;
                            }
                            i += 1;
                        }
                        luma = round2(luma, (subx + suby) as uint64_t);
                        sum += luma * *coeff as ::core::ffi::c_int;
                        break;
                    } else {
                        let fresh0 = coeff;
                        coeff = coeff.offset(1);
                        sum
                            += *fresh0 as ::core::ffi::c_int
                                * (*buf.offset((y_0 + dy) as isize))[(x_0 + dx) as usize]
                                    as ::core::ffi::c_int;
                        dx += 1;
                    }
                }
                dy += 1;
            }
            grain = (*buf.offset(y_0 as isize))[x_0 as usize] as ::core::ffi::c_int
                + round2(sum, (*data).ar_coeff_shift as uint64_t);
            (*buf.offset(y_0 as isize))[x_0 as usize] = av_clip_c(
                grain,
                grain_min,
                grain_max,
            ) as int16_t;
            x_0 += 1;
        }
        y_0 += 1;
    }
}
unsafe extern "C" fn generate_grain_uv_c_8(
    mut buf: *mut [int8_t; 82],
    mut buf_y: *const [int8_t; 82],
    params: *const AVFilmGrainParams,
    uv: intptr_t,
    subx: ::core::ffi::c_int,
    suby: ::core::ffi::c_int,
) {
    let data: *const AVFilmGrainAOMParams = &raw const (*params).codec.aom;
    let bitdepth_min_8: ::core::ffi::c_int = 8 as ::core::ffi::c_int
        - 8 as ::core::ffi::c_int;
    let mut seed: ::core::ffi::c_uint = ((*params).seed
        ^ (if uv != 0 {
            0x49d8 as ::core::ffi::c_int
        } else {
            0xb524 as ::core::ffi::c_int
        }) as uint64_t) as ::core::ffi::c_uint;
    let shift: ::core::ffi::c_int = 4 as ::core::ffi::c_int - bitdepth_min_8
        + (*data).grain_scale_shift;
    let grain_ctr: ::core::ffi::c_int = (128 as ::core::ffi::c_int) << bitdepth_min_8;
    let grain_min: ::core::ffi::c_int = -grain_ctr;
    let grain_max: ::core::ffi::c_int = grain_ctr - 1 as ::core::ffi::c_int;
    let chromaW: ::core::ffi::c_int = if subx != 0 {
        SUB_GRAIN_WIDTH as ::core::ffi::c_int as ::core::ffi::c_int
    } else {
        GRAIN_WIDTH as ::core::ffi::c_int as ::core::ffi::c_int
    };
    let chromaH: ::core::ffi::c_int = if suby != 0 {
        SUB_GRAIN_HEIGHT as ::core::ffi::c_int as ::core::ffi::c_int
    } else {
        GRAIN_HEIGHT as ::core::ffi::c_int as ::core::ffi::c_int
    };
    let ar_pad: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    let ar_lag: ::core::ffi::c_int = (*data).ar_coeff_lag;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < chromaH {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < chromaW {
            let value: ::core::ffi::c_int = get_random_number(
                11 as ::core::ffi::c_int,
                &raw mut seed,
            ) as ::core::ffi::c_int;
            (*buf.offset(y as isize))[x as usize] = round2(
                gaussian_sequence[value as usize] as ::core::ffi::c_int,
                shift as uint64_t,
            ) as int8_t;
            x += 1;
        }
        y += 1;
    }
    let mut y_0: ::core::ffi::c_int = ar_pad;
    while y_0 < chromaH {
        let mut x_0: ::core::ffi::c_int = ar_pad;
        while x_0 < chromaW - ar_pad {
            let mut coeff: *const int8_t = &raw const *(&raw const (*data).ar_coeffs_uv
                as *const [int8_t; 25])
                .offset(uv as isize) as *const int8_t;
            let mut sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut grain: ::core::ffi::c_int = 0;
            let mut dy: ::core::ffi::c_int = -ar_lag;
            while dy <= 0 as ::core::ffi::c_int {
                let mut dx: ::core::ffi::c_int = -ar_lag;
                while dx <= ar_lag {
                    if dx == 0 && dy == 0 {
                        let lumaX: ::core::ffi::c_int = (x_0 - ar_pad << subx) + ar_pad;
                        let lumaY: ::core::ffi::c_int = (y_0 - ar_pad << suby) + ar_pad;
                        let mut luma: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        if (*data).num_y_points == 0 {
                            break;
                        }
                        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i <= suby {
                            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while j <= subx {
                                luma
                                    += (*buf_y
                                        .offset((lumaY + i) as isize))[(lumaX + j) as usize]
                                        as ::core::ffi::c_int;
                                j += 1;
                            }
                            i += 1;
                        }
                        luma = round2(luma, (subx + suby) as uint64_t);
                        sum += luma * *coeff as ::core::ffi::c_int;
                        break;
                    } else {
                        let fresh2 = coeff;
                        coeff = coeff.offset(1);
                        sum
                            += *fresh2 as ::core::ffi::c_int
                                * (*buf.offset((y_0 + dy) as isize))[(x_0 + dx) as usize]
                                    as ::core::ffi::c_int;
                        dx += 1;
                    }
                }
                dy += 1;
            }
            grain = (*buf.offset(y_0 as isize))[x_0 as usize] as ::core::ffi::c_int
                + round2(sum, (*data).ar_coeff_shift as uint64_t);
            (*buf.offset(y_0 as isize))[x_0 as usize] = av_clip_c(
                grain,
                grain_min,
                grain_max,
            ) as int8_t;
            x_0 += 1;
        }
        y_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ff_aom_parse_film_grain_sets(
    mut s: *mut AVFilmGrainAFGS1Params,
    mut payload: *const uint8_t,
    mut payload_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut gbc: GetBitContext = GetBitContext {
        buffer: ::core::ptr::null::<uint8_t>(),
        index: 0,
        size_in_bits: 0,
        size_in_bits_plus8: 0,
    };
    let mut gb: *mut GetBitContext = &raw mut gbc;
    let mut aom: *mut AVFilmGrainAOMParams = ::core::ptr::null_mut::<
        AVFilmGrainAOMParams,
    >();
    let mut fgp: *mut AVFilmGrainParams = ::core::ptr::null_mut::<AVFilmGrainParams>();
    let mut ref_0: *mut AVFilmGrainParams = ::core::ptr::null_mut::<AVFilmGrainParams>();
    let mut ret: ::core::ffi::c_int = 0;
    let mut num_sets: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut uv: ::core::ffi::c_int = 0;
    let mut num_y_coeffs: ::core::ffi::c_int = 0;
    let mut update_grain: ::core::ffi::c_int = 0;
    let mut luma_only: ::core::ffi::c_int = 0;
    ret = init_get_bits8(gb, payload, payload_size);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    (*s).enable = get_bits1(gb) as ::core::ffi::c_int;
    if (*s).enable == 0 {
        return 0 as ::core::ffi::c_int;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (i_0 as usize)
        < (::core::mem::size_of::<[*mut AVBufferRef; 8]>() as usize)
            .wrapping_div(::core::mem::size_of::<*mut AVBufferRef>() as usize)
    {
        av_buffer_unref(
            (&raw mut (*s).sets as *mut *mut AVBufferRef).offset(i_0 as isize)
                as *mut *mut AVBufferRef,
        );
        i_0 += 1;
    }
    skip_bits(gb, 4 as ::core::ffi::c_int);
    num_sets = get_bits(gb, 3 as ::core::ffi::c_int)
        .wrapping_add(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    n = 0 as ::core::ffi::c_int;
    's_49: loop {
        if !(n < num_sets) {
            current_block = 7545150590528655645;
            break;
        }
        let mut payload_4byte: ::core::ffi::c_int = 0;
        let mut payload_size_0: ::core::ffi::c_int = 0;
        let mut set_idx: ::core::ffi::c_int = 0;
        let mut apply_units_log2: ::core::ffi::c_int = 0;
        let mut vsc_flag: ::core::ffi::c_int = 0;
        let mut predict_scaling: ::core::ffi::c_int = 0;
        let mut predict_y_scaling: ::core::ffi::c_int = 0;
        let mut predict_uv_scaling: [::core::ffi::c_int; 2] = [0; 2];
        let mut payload_bits: ::core::ffi::c_int = 0;
        let mut start_position: ::core::ffi::c_int = 0;
        let mut fgp_size: size_t = 0;
        start_position = get_bits_count(gb);
        payload_4byte = get_bits1(gb) as ::core::ffi::c_int;
        payload_size_0 = get_bits(
            gb,
            if payload_4byte != 0 {
                2 as ::core::ffi::c_int
            } else {
                8 as ::core::ffi::c_int
            },
        ) as ::core::ffi::c_int;
        set_idx = get_bits(gb, 3 as ::core::ffi::c_int) as ::core::ffi::c_int;
        fgp = av_film_grain_params_alloc(&raw mut fgp_size);
        if fgp.is_null() || !(*s).sets[set_idx as usize].is_null() {
            current_block = 18363848623709723027;
            break;
        }
        aom = &raw mut (*fgp).codec.aom;
        (*fgp).type_0 = (if get_bits1(gb) != 0 {
            AV_FILM_GRAIN_PARAMS_AV1 as ::core::ffi::c_int
        } else {
            AV_FILM_GRAIN_PARAMS_NONE as ::core::ffi::c_int
        }) as AVFilmGrainParamsType;
        if (*fgp).type_0 as u64 == 0 {
            av_freep(&raw mut fgp as *mut ::core::ffi::c_void);
        } else {
            (*fgp).seed = get_bits(gb, 16 as ::core::ffi::c_int) as uint64_t;
            update_grain = get_bits1(gb) as ::core::ffi::c_int;
            if update_grain == 0 {
                av_freep(&raw mut fgp as *mut ::core::ffi::c_void);
            } else {
                apply_units_log2 = get_bits(gb, 4 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
                (*fgp).width = (get_bits(gb, 12 as ::core::ffi::c_int)
                    << apply_units_log2) as ::core::ffi::c_int;
                (*fgp).height = (get_bits(gb, 12 as ::core::ffi::c_int)
                    << apply_units_log2) as ::core::ffi::c_int;
                luma_only = get_bits1(gb) as ::core::ffi::c_int;
                if luma_only != 0 {
                    (*fgp).subsampling_y = 0 as ::core::ffi::c_int;
                    (*fgp).subsampling_x = (*fgp).subsampling_y;
                } else {
                    (*fgp).subsampling_x = get_bits1(gb) as ::core::ffi::c_int;
                    (*fgp).subsampling_y = get_bits1(gb) as ::core::ffi::c_int;
                }
                (*fgp).bit_depth_chroma = 0 as ::core::ffi::c_int;
                (*fgp).bit_depth_luma = (*fgp).bit_depth_chroma;
                (*fgp).color_primaries = AVCOL_PRI_UNSPECIFIED;
                (*fgp).color_trc = AVCOL_TRC_UNSPECIFIED;
                (*fgp).color_space = AVCOL_SPC_UNSPECIFIED;
                (*fgp).color_range = AVCOL_RANGE_UNSPECIFIED;
                vsc_flag = get_bits1(gb) as ::core::ffi::c_int;
                if vsc_flag != 0 {
                    let mut cicp_flag: ::core::ffi::c_int = 0;
                    (*fgp).bit_depth_luma = get_bits(gb, 3 as ::core::ffi::c_int)
                        .wrapping_add(8 as ::core::ffi::c_uint) as ::core::ffi::c_int;
                    if luma_only == 0 {
                        (*fgp).bit_depth_chroma = (*fgp).bit_depth_luma;
                    }
                    cicp_flag = get_bits1(gb) as ::core::ffi::c_int;
                    if cicp_flag != 0 {
                        (*fgp).color_primaries = get_bits(gb, 8 as ::core::ffi::c_int)
                            as AVColorPrimaries;
                        (*fgp).color_trc = get_bits(gb, 8 as ::core::ffi::c_int)
                            as AVColorTransferCharacteristic;
                        (*fgp).color_space = get_bits(gb, 8 as ::core::ffi::c_int)
                            as AVColorSpace;
                        (*fgp).color_range = (if get_bits1(gb) != 0 {
                            AVCOL_RANGE_JPEG as ::core::ffi::c_int
                        } else {
                            AVCOL_RANGE_MPEG as ::core::ffi::c_int
                        }) as AVColorRange;
                        if (*fgp).color_primaries as ::core::ffi::c_uint
                            >= AVCOL_PRI_NB as ::core::ffi::c_int as ::core::ffi::c_uint
                            || (*fgp).color_primaries as ::core::ffi::c_uint
                                == AVCOL_PRI_RESERVED as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                            || (*fgp).color_primaries as ::core::ffi::c_uint
                                == AVCOL_PRI_RESERVED0 as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                            || (*fgp).color_trc as ::core::ffi::c_uint
                                >= AVCOL_TRC_NB as ::core::ffi::c_int as ::core::ffi::c_uint
                            || (*fgp).color_trc as ::core::ffi::c_uint
                                == AVCOL_TRC_RESERVED as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                            || (*fgp).color_trc as ::core::ffi::c_uint
                                == AVCOL_TRC_RESERVED0 as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                            || (*fgp).color_space as ::core::ffi::c_uint
                                >= AVCOL_SPC_NB as ::core::ffi::c_int as ::core::ffi::c_uint
                            || (*fgp).color_space as ::core::ffi::c_uint
                                == AVCOL_SPC_RESERVED as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                        {
                            current_block = 18363848623709723027;
                            break;
                        }
                    }
                }
                predict_scaling = get_bits1(gb) as ::core::ffi::c_int;
                if predict_scaling != 0 && ref_0.is_null() {
                    current_block = 18363848623709723027;
                    break;
                }
                predict_y_scaling = (if predict_scaling != 0 {
                    get_bits1(gb)
                } else {
                    0 as ::core::ffi::c_uint
                }) as ::core::ffi::c_int;
                if predict_y_scaling != 0 {
                    let mut y_scale: ::core::ffi::c_int = 0;
                    let mut y_offset: ::core::ffi::c_int = 0;
                    let mut bits_res: ::core::ffi::c_int = 0;
                    y_scale = get_bits(gb, 9 as ::core::ffi::c_int)
                        .wrapping_sub(256 as ::core::ffi::c_uint) as ::core::ffi::c_int;
                    y_offset = get_bits(gb, 9 as ::core::ffi::c_int)
                        .wrapping_sub(256 as ::core::ffi::c_uint) as ::core::ffi::c_int;
                    bits_res = get_bits(gb, 3 as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                    if bits_res != 0 {
                        let mut res: [::core::ffi::c_int; 14] = [0; 14];
                        let mut pred: ::core::ffi::c_int = 0;
                        let mut granularity: ::core::ffi::c_int = 0;
                        (*aom).num_y_points = (*ref_0).codec.aom.num_y_points;
                        i = 0 as ::core::ffi::c_int;
                        while i < (*aom).num_y_points {
                            res[i as usize] = get_bits(gb, bits_res)
                                as ::core::ffi::c_int;
                            i += 1;
                        }
                        granularity = get_bits(gb, 3 as ::core::ffi::c_int)
                            as ::core::ffi::c_int;
                        i = 0 as ::core::ffi::c_int;
                        while i < (*aom).num_y_points {
                            pred = (*ref_0)
                                .codec
                                .aom
                                .y_points[i as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int;
                            pred = (pred * y_scale + 8 as ::core::ffi::c_int
                                >> 4 as ::core::ffi::c_int) + y_offset;
                            pred
                                += (res[i as usize]
                                    - ((1 as ::core::ffi::c_int)
                                        << bits_res - 1 as ::core::ffi::c_int)) * granularity;
                            (*aom)
                                .y_points[i as usize][0 as ::core::ffi::c_int as usize] = (*ref_0)
                                .codec
                                .aom
                                .y_points[i as usize][0 as ::core::ffi::c_int as usize];
                            (*aom)
                                .y_points[i as usize][1 as ::core::ffi::c_int as usize] = av_clip_uint8_c(
                                pred,
                            );
                            i += 1;
                        }
                    }
                } else {
                    (*aom).num_y_points = get_bits(gb, 4 as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                    if (*aom).num_y_points > 14 as ::core::ffi::c_int {
                        current_block = 18363848623709723027;
                        break;
                    }
                    if (*aom).num_y_points != 0 {
                        let mut bits_inc: ::core::ffi::c_int = 0;
                        let mut bits_scaling: ::core::ffi::c_int = 0;
                        let mut y_value: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        bits_inc = get_bits(gb, 3 as ::core::ffi::c_int)
                            .wrapping_add(1 as ::core::ffi::c_uint)
                            as ::core::ffi::c_int;
                        bits_scaling = get_bits(gb, 2 as ::core::ffi::c_int)
                            .wrapping_add(5 as ::core::ffi::c_uint)
                            as ::core::ffi::c_int;
                        i = 0 as ::core::ffi::c_int;
                        while i < (*aom).num_y_points {
                            y_value = (y_value as ::core::ffi::c_uint)
                                .wrapping_add(get_bits(gb, bits_inc)) as ::core::ffi::c_int
                                as ::core::ffi::c_int;
                            if y_value > 0xff as ::core::ffi::c_int {
                                current_block = 18363848623709723027;
                                break 's_49;
                            }
                            (*aom)
                                .y_points[i as usize][0 as ::core::ffi::c_int as usize] = y_value
                                as uint8_t;
                            (*aom)
                                .y_points[i as usize][1 as ::core::ffi::c_int as usize] = get_bits(
                                gb,
                                bits_scaling,
                            ) as uint8_t;
                            i += 1;
                        }
                    }
                }
                if luma_only != 0 {
                    (*aom).chroma_scaling_from_luma = 0 as ::core::ffi::c_int;
                    (*aom).num_uv_points[1 as ::core::ffi::c_int as usize] = 0
                        as ::core::ffi::c_int;
                    (*aom).num_uv_points[0 as ::core::ffi::c_int as usize] = (*aom)
                        .num_uv_points[1 as ::core::ffi::c_int as usize];
                } else {
                    (*aom).chroma_scaling_from_luma = get_bits1(gb)
                        as ::core::ffi::c_int;
                    if (*aom).chroma_scaling_from_luma != 0 {
                        (*aom).num_uv_points[1 as ::core::ffi::c_int as usize] = 0
                            as ::core::ffi::c_int;
                        (*aom).num_uv_points[0 as ::core::ffi::c_int as usize] = (*aom)
                            .num_uv_points[1 as ::core::ffi::c_int as usize];
                    } else {
                        uv = 0 as ::core::ffi::c_int;
                        while uv < 2 as ::core::ffi::c_int {
                            predict_uv_scaling[uv as usize] = (if predict_scaling != 0 {
                                get_bits1(gb)
                            } else {
                                0 as ::core::ffi::c_uint
                            }) as ::core::ffi::c_int;
                            if predict_uv_scaling[uv as usize] != 0 {
                                let mut uv_scale: ::core::ffi::c_int = 0;
                                let mut uv_offset: ::core::ffi::c_int = 0;
                                let mut bits_res_0: ::core::ffi::c_int = 0;
                                uv_scale = get_bits(gb, 9 as ::core::ffi::c_int)
                                    .wrapping_sub(256 as ::core::ffi::c_uint)
                                    as ::core::ffi::c_int;
                                uv_offset = get_bits(gb, 9 as ::core::ffi::c_int)
                                    .wrapping_sub(256 as ::core::ffi::c_uint)
                                    as ::core::ffi::c_int;
                                bits_res_0 = get_bits(gb, 3 as ::core::ffi::c_int)
                                    as ::core::ffi::c_int;
                                (*aom).uv_mult[uv as usize] = (*ref_0)
                                    .codec
                                    .aom
                                    .uv_mult[uv as usize];
                                (*aom).uv_mult_luma[uv as usize] = (*ref_0)
                                    .codec
                                    .aom
                                    .uv_mult_luma[uv as usize];
                                (*aom).uv_offset[uv as usize] = (*ref_0)
                                    .codec
                                    .aom
                                    .uv_offset[uv as usize];
                                if bits_res_0 != 0 {
                                    let mut res_0: [::core::ffi::c_int; 10] = [0; 10];
                                    let mut pred_0: ::core::ffi::c_int = 0;
                                    let mut granularity_0: ::core::ffi::c_int = 0;
                                    (*aom).num_uv_points[uv as usize] = (*ref_0)
                                        .codec
                                        .aom
                                        .num_uv_points[uv as usize];
                                    i = 0 as ::core::ffi::c_int;
                                    while i < (*aom).num_uv_points[uv as usize] {
                                        res_0[i as usize] = get_bits(gb, bits_res_0)
                                            as ::core::ffi::c_int;
                                        i += 1;
                                    }
                                    granularity_0 = get_bits(gb, 3 as ::core::ffi::c_int)
                                        as ::core::ffi::c_int;
                                    i = 0 as ::core::ffi::c_int;
                                    while i < (*aom).num_uv_points[uv as usize] {
                                        pred_0 = (*ref_0)
                                            .codec
                                            .aom
                                            .uv_points[uv
                                            as usize][i as usize][1 as ::core::ffi::c_int as usize]
                                            as ::core::ffi::c_int;
                                        pred_0 = (pred_0 * uv_scale + 8 as ::core::ffi::c_int
                                            >> 4 as ::core::ffi::c_int) + uv_offset;
                                        pred_0
                                            += (res_0[i as usize]
                                                - ((1 as ::core::ffi::c_int)
                                                    << bits_res_0 - 1 as ::core::ffi::c_int)) * granularity_0;
                                        (*aom)
                                            .uv_points[uv
                                            as usize][i as usize][0 as ::core::ffi::c_int as usize] = (*ref_0)
                                            .codec
                                            .aom
                                            .uv_points[uv
                                            as usize][i as usize][0 as ::core::ffi::c_int as usize];
                                        (*aom)
                                            .uv_points[uv
                                            as usize][i as usize][1 as ::core::ffi::c_int as usize] = av_clip_uint8_c(
                                            pred_0,
                                        );
                                        i += 1;
                                    }
                                }
                            } else {
                                let mut bits_inc_0: ::core::ffi::c_int = 0;
                                let mut bits_scaling_0: ::core::ffi::c_int = 0;
                                let mut uv_offset_0: ::core::ffi::c_int = 0;
                                let mut uv_value: ::core::ffi::c_int = 0
                                    as ::core::ffi::c_int;
                                (*aom).num_uv_points[uv as usize] = get_bits(
                                    gb,
                                    4 as ::core::ffi::c_int,
                                ) as ::core::ffi::c_int;
                                if (*aom).num_uv_points[uv as usize]
                                    > 10 as ::core::ffi::c_int
                                {
                                    current_block = 18363848623709723027;
                                    break 's_49;
                                }
                                bits_inc_0 = get_bits(gb, 3 as ::core::ffi::c_int)
                                    .wrapping_add(1 as ::core::ffi::c_uint)
                                    as ::core::ffi::c_int;
                                bits_scaling_0 = get_bits(gb, 2 as ::core::ffi::c_int)
                                    .wrapping_add(5 as ::core::ffi::c_uint)
                                    as ::core::ffi::c_int;
                                uv_offset_0 = get_bits(gb, 8 as ::core::ffi::c_int)
                                    as ::core::ffi::c_int;
                                i = 0 as ::core::ffi::c_int;
                                while i < (*aom).num_uv_points[uv as usize] {
                                    uv_value = (uv_value as ::core::ffi::c_uint)
                                        .wrapping_add(get_bits(gb, bits_inc_0))
                                        as ::core::ffi::c_int as ::core::ffi::c_int;
                                    if uv_value > 0xff as ::core::ffi::c_int {
                                        current_block = 18363848623709723027;
                                        break 's_49;
                                    }
                                    (*aom)
                                        .uv_points[uv
                                        as usize][i as usize][0 as ::core::ffi::c_int as usize] = uv_value
                                        as uint8_t;
                                    (*aom)
                                        .uv_points[uv
                                        as usize][i as usize][1 as ::core::ffi::c_int as usize] = get_bits(
                                            gb,
                                            bits_scaling_0,
                                        )
                                        .wrapping_add(uv_offset_0 as ::core::ffi::c_uint)
                                        as uint8_t;
                                    i += 1;
                                }
                            }
                            uv += 1;
                        }
                    }
                }
                (*aom).scaling_shift = get_bits(gb, 2 as ::core::ffi::c_int)
                    .wrapping_add(8 as ::core::ffi::c_uint) as ::core::ffi::c_int;
                (*aom).ar_coeff_lag = get_bits(gb, 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
                num_y_coeffs = 2 as ::core::ffi::c_int * (*aom).ar_coeff_lag
                    * ((*aom).ar_coeff_lag + 1 as ::core::ffi::c_int);
                if (*aom).num_y_points != 0 {
                    let mut ar_bits: ::core::ffi::c_int = get_bits(
                            gb,
                            2 as ::core::ffi::c_int,
                        )
                        .wrapping_add(5 as ::core::ffi::c_uint) as ::core::ffi::c_int;
                    i = 0 as ::core::ffi::c_int;
                    while i < num_y_coeffs {
                        (*aom).ar_coeffs_y[i as usize] = get_bits(gb, ar_bits)
                            .wrapping_sub(
                                ((1 as ::core::ffi::c_int)
                                    << ar_bits - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint,
                            ) as int8_t;
                        i += 1;
                    }
                }
                uv = 0 as ::core::ffi::c_int;
                while uv < 2 as ::core::ffi::c_int {
                    if (*aom).chroma_scaling_from_luma != 0
                        || (*aom).num_uv_points[uv as usize] != 0
                    {
                        let mut ar_bits_0: ::core::ffi::c_int = get_bits(
                                gb,
                                2 as ::core::ffi::c_int,
                            )
                            .wrapping_add(5 as ::core::ffi::c_uint)
                            as ::core::ffi::c_int;
                        i = 0 as ::core::ffi::c_int;
                        while i
                            < num_y_coeffs
                                + ((*aom).num_y_points != 0) as ::core::ffi::c_int
                        {
                            (*aom).ar_coeffs_uv[uv as usize][i as usize] = get_bits(
                                    gb,
                                    ar_bits_0,
                                )
                                .wrapping_sub(
                                    ((1 as ::core::ffi::c_int)
                                        << ar_bits_0 - 1 as ::core::ffi::c_int)
                                        as ::core::ffi::c_uint,
                                ) as int8_t;
                            i += 1;
                        }
                    }
                    uv += 1;
                }
                (*aom).ar_coeff_shift = get_bits(gb, 2 as ::core::ffi::c_int)
                    .wrapping_add(6 as ::core::ffi::c_uint) as ::core::ffi::c_int;
                (*aom).grain_scale_shift = get_bits(gb, 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
                uv = 0 as ::core::ffi::c_int;
                while uv < 2 as ::core::ffi::c_int {
                    if (*aom).num_uv_points[uv as usize] != 0
                        && predict_uv_scaling[uv as usize] == 0
                    {
                        (*aom).uv_mult[uv as usize] = get_bits(
                                gb,
                                8 as ::core::ffi::c_int,
                            )
                            .wrapping_sub(128 as ::core::ffi::c_uint)
                            as ::core::ffi::c_int;
                        (*aom).uv_mult_luma[uv as usize] = get_bits(
                                gb,
                                8 as ::core::ffi::c_int,
                            )
                            .wrapping_sub(128 as ::core::ffi::c_uint)
                            as ::core::ffi::c_int;
                        (*aom).uv_offset[uv as usize] = get_bits(
                                gb,
                                9 as ::core::ffi::c_int,
                            )
                            .wrapping_sub(256 as ::core::ffi::c_uint)
                            as ::core::ffi::c_int;
                    }
                    uv += 1;
                }
                (*aom).overlap_flag = get_bits1(gb) as ::core::ffi::c_int;
                (*aom).limit_output_range = get_bits1(gb) as ::core::ffi::c_int;
                if n == 0 as ::core::ffi::c_int {
                    ref_0 = fgp;
                }
                payload_bits = get_bits_count(gb) - start_position;
                if payload_bits > payload_size_0 * 8 as ::core::ffi::c_int {
                    current_block = 18363848623709723027;
                    break;
                }
                skip_bits(gb, payload_size_0 * 8 as ::core::ffi::c_int - payload_bits);
                av_buffer_unref(
                    (&raw mut (*s).sets as *mut *mut AVBufferRef)
                        .offset(set_idx as isize) as *mut *mut AVBufferRef,
                );
                (*s).sets[set_idx as usize] = av_buffer_create(
                    fgp as *mut uint8_t,
                    fgp_size,
                    None,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    0 as ::core::ffi::c_int,
                );
                if (*s).sets[set_idx as usize].is_null() {
                    current_block = 18363848623709723027;
                    break;
                }
            }
        }
        n += 1;
    }
    match current_block {
        7545150590528655645 => return 0 as ::core::ffi::c_int,
        _ => {
            av_free(fgp as *mut ::core::ffi::c_void);
            ff_aom_uninit_film_grain_params(s);
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
    };
}
#[inline]
unsafe extern "C" fn sample_lut_16(
    mut grain_lut: *const [int16_t; 82],
    mut offsets: *const [::core::ffi::c_int; 2],
    subx: ::core::ffi::c_int,
    suby: ::core::ffi::c_int,
    bx: ::core::ffi::c_int,
    by: ::core::ffi::c_int,
    x: ::core::ffi::c_int,
    y: ::core::ffi::c_int,
) -> int16_t {
    let randval: ::core::ffi::c_int = (*offsets.offset(bx as isize))[by as usize];
    let offx: ::core::ffi::c_int = 3 as ::core::ffi::c_int
        + (2 as ::core::ffi::c_int >> subx)
            * (3 as ::core::ffi::c_int + (randval >> 4 as ::core::ffi::c_int));
    let offy: ::core::ffi::c_int = 3 as ::core::ffi::c_int
        + (2 as ::core::ffi::c_int >> suby)
            * (3 as ::core::ffi::c_int + (randval & 0xf as ::core::ffi::c_int));
    return (*grain_lut
        .offset(
            (offy + y + (FG_BLOCK_SIZE as ::core::ffi::c_int >> suby) * by) as isize,
        ))[(offx + x + (FG_BLOCK_SIZE as ::core::ffi::c_int >> subx) * bx) as usize];
}
#[inline]
unsafe extern "C" fn sample_lut_8(
    mut grain_lut: *const [int8_t; 82],
    mut offsets: *const [::core::ffi::c_int; 2],
    subx: ::core::ffi::c_int,
    suby: ::core::ffi::c_int,
    bx: ::core::ffi::c_int,
    by: ::core::ffi::c_int,
    x: ::core::ffi::c_int,
    y: ::core::ffi::c_int,
) -> int8_t {
    let randval: ::core::ffi::c_int = (*offsets.offset(bx as isize))[by as usize];
    let offx: ::core::ffi::c_int = 3 as ::core::ffi::c_int
        + (2 as ::core::ffi::c_int >> subx)
            * (3 as ::core::ffi::c_int + (randval >> 4 as ::core::ffi::c_int));
    let offy: ::core::ffi::c_int = 3 as ::core::ffi::c_int
        + (2 as ::core::ffi::c_int >> suby)
            * (3 as ::core::ffi::c_int + (randval & 0xf as ::core::ffi::c_int));
    return (*grain_lut
        .offset(
            (offy + y + (FG_BLOCK_SIZE as ::core::ffi::c_int >> suby) * by) as isize,
        ))[(offx + x + (FG_BLOCK_SIZE as ::core::ffi::c_int >> subx) * bx) as usize];
}
#[inline(always)]
unsafe extern "C" fn av_clip_c(
    mut a: ::core::ffi::c_int,
    mut amin: ::core::ffi::c_int,
    mut amax: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if a < amin { return amin } else if a > amax { return amax } else { return a };
}
unsafe extern "C" fn fgy_32x32xn_c_16(
    dst_row: *mut uint16_t,
    src_row: *const uint16_t,
    stride: ptrdiff_t,
    params: *const AVFilmGrainParams,
    pw: size_t,
    mut scaling: *const uint8_t,
    mut grain_lut: *const [int16_t; 82],
    bh: ::core::ffi::c_int,
    row_num: ::core::ffi::c_int,
    bitdepth: ::core::ffi::c_int,
) {
    let data: *const AVFilmGrainAOMParams = &raw const (*params).codec.aom;
    let rows: ::core::ffi::c_int = 1 as ::core::ffi::c_int
        + ((*data).overlap_flag != 0 && row_num > 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
    let bitdepth_min_8: ::core::ffi::c_int = bitdepth - 8 as ::core::ffi::c_int;
    let grain_ctr: ::core::ffi::c_int = (128 as ::core::ffi::c_int) << bitdepth_min_8;
    let grain_min: ::core::ffi::c_int = -grain_ctr;
    let grain_max: ::core::ffi::c_int = grain_ctr - 1 as ::core::ffi::c_int;
    let mut seed: [::core::ffi::c_uint; 2] = [0; 2];
    let mut offsets: [[::core::ffi::c_int; 2]; 2] = [[0; 2]; 2];
    let mut min_value: ::core::ffi::c_int = 0;
    let mut max_value: ::core::ffi::c_int = 0;
    if (*data).limit_output_range != 0 {
        min_value = (16 as ::core::ffi::c_int) << bitdepth_min_8;
        max_value = (235 as ::core::ffi::c_int) << bitdepth_min_8;
    } else {
        min_value = 0 as ::core::ffi::c_int;
        max_value = ((1 as ::core::ffi::c_int) << bitdepth) - 1 as ::core::ffi::c_int;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < rows {
        seed[i as usize] = (*params).seed as ::core::ffi::c_uint;
        seed[i as usize]
            ^= (((row_num - i) * 37 as ::core::ffi::c_int + 178 as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                as ::core::ffi::c_uint;
        seed[i as usize]
            ^= ((row_num - i) * 173 as ::core::ffi::c_int + 105 as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uint;
        i += 1;
    }
    let mut bx: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while (bx as size_t) < pw {
        let bw: ::core::ffi::c_int = (if FG_BLOCK_SIZE as ::core::ffi::c_int
            as ::core::ffi::c_uint
            > (pw as ::core::ffi::c_int as ::core::ffi::c_uint).wrapping_sub(bx)
        {
            (pw as ::core::ffi::c_int as ::core::ffi::c_uint).wrapping_sub(bx)
        } else {
            FG_BLOCK_SIZE as ::core::ffi::c_int as ::core::ffi::c_uint
        }) as ::core::ffi::c_int;
        let mut src: *const uint16_t = ::core::ptr::null::<uint16_t>();
        let mut dst: *mut uint16_t = ::core::ptr::null_mut::<uint16_t>();
        let mut noise: ::core::ffi::c_int = 0;
        let ystart: ::core::ffi::c_int = if (*data).overlap_flag != 0 && row_num != 0 {
            if 2 as ::core::ffi::c_int > bh { bh } else { 2 as ::core::ffi::c_int }
        } else {
            0 as ::core::ffi::c_int
        };
        let xstart: ::core::ffi::c_int = if (*data).overlap_flag != 0 && bx != 0 {
            if 2 as ::core::ffi::c_int > bw { bw } else { 2 as ::core::ffi::c_int }
        } else {
            0 as ::core::ffi::c_int
        };
        static mut w: [[::core::ffi::c_int; 2]; 2] = [
            [27 as ::core::ffi::c_int, 17 as ::core::ffi::c_int],
            [17 as ::core::ffi::c_int, 27 as ::core::ffi::c_int],
        ];
        if (*data).overlap_flag != 0 && bx != 0 {
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0 < rows {
                offsets[1 as ::core::ffi::c_int as usize][i_0 as usize] = offsets[0
                    as ::core::ffi::c_int as usize][i_0 as usize];
                i_0 += 1;
            }
        }
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1 < rows {
            offsets[0 as ::core::ffi::c_int as usize][i_1 as usize] = get_random_number(
                8 as ::core::ffi::c_int,
                (&raw mut seed as *mut ::core::ffi::c_uint).offset(i_1 as isize)
                    as *mut ::core::ffi::c_uint,
            );
            i_1 += 1;
        }
        let mut y: ::core::ffi::c_int = ystart;
        while y < bh {
            let mut x: ::core::ffi::c_int = xstart;
            while x < bw {
                let mut grain: ::core::ffi::c_int = sample_lut_16(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    x,
                    y,
                ) as ::core::ffi::c_int;
                src = ((src_row as *const ::core::ffi::c_char)
                    .offset((y as ptrdiff_t * stride) as isize) as *const uint16_t)
                    .offset(x as isize)
                    .offset(bx as isize);
                dst = ((dst_row as *mut ::core::ffi::c_char)
                    .offset((y as ptrdiff_t * stride) as isize) as *mut uint16_t)
                    .offset(x as isize)
                    .offset(bx as isize);
                noise = round2(
                    *scaling.offset(*src as isize) as ::core::ffi::c_int * grain,
                    (*data).scaling_shift as uint64_t,
                );
                *dst = av_clip_c(
                    *src as ::core::ffi::c_int + noise,
                    min_value,
                    max_value,
                ) as uint16_t;
                x += 1;
            }
            let mut x_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_0 < xstart {
                let mut grain_0: ::core::ffi::c_int = sample_lut_16(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    x_0,
                    y,
                ) as ::core::ffi::c_int;
                let mut old: ::core::ffi::c_int = sample_lut_16(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    x_0,
                    y,
                ) as ::core::ffi::c_int;
                grain_0 = round2(
                    old * w[x_0 as usize][0 as ::core::ffi::c_int as usize]
                        + grain_0 * w[x_0 as usize][1 as ::core::ffi::c_int as usize],
                    5 as uint64_t,
                );
                grain_0 = av_clip_c(grain_0, grain_min, grain_max);
                src = ((src_row as *const ::core::ffi::c_char)
                    .offset((y as ptrdiff_t * stride) as isize) as *const uint16_t)
                    .offset(x_0 as isize)
                    .offset(bx as isize);
                dst = ((dst_row as *mut ::core::ffi::c_char)
                    .offset((y as ptrdiff_t * stride) as isize) as *mut uint16_t)
                    .offset(x_0 as isize)
                    .offset(bx as isize);
                noise = round2(
                    *scaling.offset(*src as isize) as ::core::ffi::c_int * grain_0,
                    (*data).scaling_shift as uint64_t,
                );
                *dst = av_clip_c(
                    *src as ::core::ffi::c_int + noise,
                    min_value,
                    max_value,
                ) as uint16_t;
                x_0 += 1;
            }
            y += 1;
        }
        let mut y_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y_0 < ystart {
            let mut x_1: ::core::ffi::c_int = xstart;
            while x_1 < bw {
                let mut grain_1: ::core::ffi::c_int = sample_lut_16(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    x_1,
                    y_0,
                ) as ::core::ffi::c_int;
                let mut old_0: ::core::ffi::c_int = sample_lut_16(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    x_1,
                    y_0,
                ) as ::core::ffi::c_int;
                grain_1 = round2(
                    old_0 * w[y_0 as usize][0 as ::core::ffi::c_int as usize]
                        + grain_1 * w[y_0 as usize][1 as ::core::ffi::c_int as usize],
                    5 as uint64_t,
                );
                grain_1 = av_clip_c(grain_1, grain_min, grain_max);
                src = ((src_row as *const ::core::ffi::c_char)
                    .offset((y_0 as ptrdiff_t * stride) as isize) as *const uint16_t)
                    .offset(x_1 as isize)
                    .offset(bx as isize);
                dst = ((dst_row as *mut ::core::ffi::c_char)
                    .offset((y_0 as ptrdiff_t * stride) as isize) as *mut uint16_t)
                    .offset(x_1 as isize)
                    .offset(bx as isize);
                noise = round2(
                    *scaling.offset(*src as isize) as ::core::ffi::c_int * grain_1,
                    (*data).scaling_shift as uint64_t,
                );
                *dst = av_clip_c(
                    *src as ::core::ffi::c_int + noise,
                    min_value,
                    max_value,
                ) as uint16_t;
                x_1 += 1;
            }
            let mut x_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_2 < xstart {
                let mut grain_2: ::core::ffi::c_int = sample_lut_16(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    x_2,
                    y_0,
                ) as ::core::ffi::c_int;
                let mut top: ::core::ffi::c_int = sample_lut_16(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    x_2,
                    y_0,
                ) as ::core::ffi::c_int;
                let mut old_1: ::core::ffi::c_int = sample_lut_16(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    x_2,
                    y_0,
                ) as ::core::ffi::c_int;
                top = round2(
                    old_1 * w[x_2 as usize][0 as ::core::ffi::c_int as usize]
                        + top * w[x_2 as usize][1 as ::core::ffi::c_int as usize],
                    5 as uint64_t,
                );
                top = av_clip_c(top, grain_min, grain_max);
                old_1 = sample_lut_16(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    x_2,
                    y_0,
                ) as ::core::ffi::c_int;
                grain_2 = round2(
                    old_1 * w[x_2 as usize][0 as ::core::ffi::c_int as usize]
                        + grain_2 * w[x_2 as usize][1 as ::core::ffi::c_int as usize],
                    5 as uint64_t,
                );
                grain_2 = av_clip_c(grain_2, grain_min, grain_max);
                grain_2 = round2(
                    top * w[y_0 as usize][0 as ::core::ffi::c_int as usize]
                        + grain_2 * w[y_0 as usize][1 as ::core::ffi::c_int as usize],
                    5 as uint64_t,
                );
                grain_2 = av_clip_c(grain_2, grain_min, grain_max);
                src = ((src_row as *const ::core::ffi::c_char)
                    .offset((y_0 as ptrdiff_t * stride) as isize) as *const uint16_t)
                    .offset(x_2 as isize)
                    .offset(bx as isize);
                dst = ((dst_row as *mut ::core::ffi::c_char)
                    .offset((y_0 as ptrdiff_t * stride) as isize) as *mut uint16_t)
                    .offset(x_2 as isize)
                    .offset(bx as isize);
                noise = round2(
                    *scaling.offset(*src as isize) as ::core::ffi::c_int * grain_2,
                    (*data).scaling_shift as uint64_t,
                );
                *dst = av_clip_c(
                    *src as ::core::ffi::c_int + noise,
                    min_value,
                    max_value,
                ) as uint16_t;
                x_2 += 1;
            }
            y_0 += 1;
        }
        bx = bx.wrapping_add(FG_BLOCK_SIZE as ::core::ffi::c_int as ::core::ffi::c_uint);
    }
}
unsafe extern "C" fn fgy_32x32xn_c_8(
    dst_row: *mut uint8_t,
    src_row: *const uint8_t,
    stride: ptrdiff_t,
    params: *const AVFilmGrainParams,
    pw: size_t,
    mut scaling: *const uint8_t,
    mut grain_lut: *const [int8_t; 82],
    bh: ::core::ffi::c_int,
    row_num: ::core::ffi::c_int,
) {
    let data: *const AVFilmGrainAOMParams = &raw const (*params).codec.aom;
    let rows: ::core::ffi::c_int = 1 as ::core::ffi::c_int
        + ((*data).overlap_flag != 0 && row_num > 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
    let bitdepth_min_8: ::core::ffi::c_int = 8 as ::core::ffi::c_int
        - 8 as ::core::ffi::c_int;
    let grain_ctr: ::core::ffi::c_int = (128 as ::core::ffi::c_int) << bitdepth_min_8;
    let grain_min: ::core::ffi::c_int = -grain_ctr;
    let grain_max: ::core::ffi::c_int = grain_ctr - 1 as ::core::ffi::c_int;
    let mut seed: [::core::ffi::c_uint; 2] = [0; 2];
    let mut offsets: [[::core::ffi::c_int; 2]; 2] = [[0; 2]; 2];
    let mut min_value: ::core::ffi::c_int = 0;
    let mut max_value: ::core::ffi::c_int = 0;
    if (*data).limit_output_range != 0 {
        min_value = (16 as ::core::ffi::c_int) << bitdepth_min_8;
        max_value = (235 as ::core::ffi::c_int) << bitdepth_min_8;
    } else {
        min_value = 0 as ::core::ffi::c_int;
        max_value = 0xff as ::core::ffi::c_int;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < rows {
        seed[i as usize] = (*params).seed as ::core::ffi::c_uint;
        seed[i as usize]
            ^= (((row_num - i) * 37 as ::core::ffi::c_int + 178 as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                as ::core::ffi::c_uint;
        seed[i as usize]
            ^= ((row_num - i) * 173 as ::core::ffi::c_int + 105 as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uint;
        i += 1;
    }
    let mut bx: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while (bx as size_t) < pw {
        let bw: ::core::ffi::c_int = (if FG_BLOCK_SIZE as ::core::ffi::c_int
            as ::core::ffi::c_uint
            > (pw as ::core::ffi::c_int as ::core::ffi::c_uint).wrapping_sub(bx)
        {
            (pw as ::core::ffi::c_int as ::core::ffi::c_uint).wrapping_sub(bx)
        } else {
            FG_BLOCK_SIZE as ::core::ffi::c_int as ::core::ffi::c_uint
        }) as ::core::ffi::c_int;
        let mut src: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut dst: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut noise: ::core::ffi::c_int = 0;
        let ystart: ::core::ffi::c_int = if (*data).overlap_flag != 0 && row_num != 0 {
            if 2 as ::core::ffi::c_int > bh { bh } else { 2 as ::core::ffi::c_int }
        } else {
            0 as ::core::ffi::c_int
        };
        let xstart: ::core::ffi::c_int = if (*data).overlap_flag != 0 && bx != 0 {
            if 2 as ::core::ffi::c_int > bw { bw } else { 2 as ::core::ffi::c_int }
        } else {
            0 as ::core::ffi::c_int
        };
        static mut w: [[::core::ffi::c_int; 2]; 2] = [
            [27 as ::core::ffi::c_int, 17 as ::core::ffi::c_int],
            [17 as ::core::ffi::c_int, 27 as ::core::ffi::c_int],
        ];
        if (*data).overlap_flag != 0 && bx != 0 {
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0 < rows {
                offsets[1 as ::core::ffi::c_int as usize][i_0 as usize] = offsets[0
                    as ::core::ffi::c_int as usize][i_0 as usize];
                i_0 += 1;
            }
        }
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1 < rows {
            offsets[0 as ::core::ffi::c_int as usize][i_1 as usize] = get_random_number(
                8 as ::core::ffi::c_int,
                (&raw mut seed as *mut ::core::ffi::c_uint).offset(i_1 as isize)
                    as *mut ::core::ffi::c_uint,
            );
            i_1 += 1;
        }
        let mut y: ::core::ffi::c_int = ystart;
        while y < bh {
            let mut x: ::core::ffi::c_int = xstart;
            while x < bw {
                let mut grain: ::core::ffi::c_int = sample_lut_8(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    x,
                    y,
                ) as ::core::ffi::c_int;
                src = ((src_row as *const ::core::ffi::c_char)
                    .offset((y as ptrdiff_t * stride) as isize) as *const uint8_t)
                    .offset(x as isize)
                    .offset(bx as isize);
                dst = ((dst_row as *mut ::core::ffi::c_char)
                    .offset((y as ptrdiff_t * stride) as isize) as *mut uint8_t)
                    .offset(x as isize)
                    .offset(bx as isize);
                noise = round2(
                    *scaling.offset(*src as isize) as ::core::ffi::c_int * grain,
                    (*data).scaling_shift as uint64_t,
                );
                *dst = av_clip_c(
                    *src as ::core::ffi::c_int + noise,
                    min_value,
                    max_value,
                ) as uint8_t;
                x += 1;
            }
            let mut x_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_0 < xstart {
                let mut grain_0: ::core::ffi::c_int = sample_lut_8(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    x_0,
                    y,
                ) as ::core::ffi::c_int;
                let mut old: ::core::ffi::c_int = sample_lut_8(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    x_0,
                    y,
                ) as ::core::ffi::c_int;
                grain_0 = round2(
                    old * w[x_0 as usize][0 as ::core::ffi::c_int as usize]
                        + grain_0 * w[x_0 as usize][1 as ::core::ffi::c_int as usize],
                    5 as uint64_t,
                );
                grain_0 = av_clip_c(grain_0, grain_min, grain_max);
                src = ((src_row as *const ::core::ffi::c_char)
                    .offset((y as ptrdiff_t * stride) as isize) as *const uint8_t)
                    .offset(x_0 as isize)
                    .offset(bx as isize);
                dst = ((dst_row as *mut ::core::ffi::c_char)
                    .offset((y as ptrdiff_t * stride) as isize) as *mut uint8_t)
                    .offset(x_0 as isize)
                    .offset(bx as isize);
                noise = round2(
                    *scaling.offset(*src as isize) as ::core::ffi::c_int * grain_0,
                    (*data).scaling_shift as uint64_t,
                );
                *dst = av_clip_c(
                    *src as ::core::ffi::c_int + noise,
                    min_value,
                    max_value,
                ) as uint8_t;
                x_0 += 1;
            }
            y += 1;
        }
        let mut y_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y_0 < ystart {
            let mut x_1: ::core::ffi::c_int = xstart;
            while x_1 < bw {
                let mut grain_1: ::core::ffi::c_int = sample_lut_8(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    x_1,
                    y_0,
                ) as ::core::ffi::c_int;
                let mut old_0: ::core::ffi::c_int = sample_lut_8(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    x_1,
                    y_0,
                ) as ::core::ffi::c_int;
                grain_1 = round2(
                    old_0 * w[y_0 as usize][0 as ::core::ffi::c_int as usize]
                        + grain_1 * w[y_0 as usize][1 as ::core::ffi::c_int as usize],
                    5 as uint64_t,
                );
                grain_1 = av_clip_c(grain_1, grain_min, grain_max);
                src = ((src_row as *const ::core::ffi::c_char)
                    .offset((y_0 as ptrdiff_t * stride) as isize) as *const uint8_t)
                    .offset(x_1 as isize)
                    .offset(bx as isize);
                dst = ((dst_row as *mut ::core::ffi::c_char)
                    .offset((y_0 as ptrdiff_t * stride) as isize) as *mut uint8_t)
                    .offset(x_1 as isize)
                    .offset(bx as isize);
                noise = round2(
                    *scaling.offset(*src as isize) as ::core::ffi::c_int * grain_1,
                    (*data).scaling_shift as uint64_t,
                );
                *dst = av_clip_c(
                    *src as ::core::ffi::c_int + noise,
                    min_value,
                    max_value,
                ) as uint8_t;
                x_1 += 1;
            }
            let mut x_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_2 < xstart {
                let mut grain_2: ::core::ffi::c_int = sample_lut_8(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    x_2,
                    y_0,
                ) as ::core::ffi::c_int;
                let mut top: ::core::ffi::c_int = sample_lut_8(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    x_2,
                    y_0,
                ) as ::core::ffi::c_int;
                let mut old_1: ::core::ffi::c_int = sample_lut_8(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    x_2,
                    y_0,
                ) as ::core::ffi::c_int;
                top = round2(
                    old_1 * w[x_2 as usize][0 as ::core::ffi::c_int as usize]
                        + top * w[x_2 as usize][1 as ::core::ffi::c_int as usize],
                    5 as uint64_t,
                );
                top = av_clip_c(top, grain_min, grain_max);
                old_1 = sample_lut_8(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    x_2,
                    y_0,
                ) as ::core::ffi::c_int;
                grain_2 = round2(
                    old_1 * w[x_2 as usize][0 as ::core::ffi::c_int as usize]
                        + grain_2 * w[x_2 as usize][1 as ::core::ffi::c_int as usize],
                    5 as uint64_t,
                );
                grain_2 = av_clip_c(grain_2, grain_min, grain_max);
                grain_2 = round2(
                    top * w[y_0 as usize][0 as ::core::ffi::c_int as usize]
                        + grain_2 * w[y_0 as usize][1 as ::core::ffi::c_int as usize],
                    5 as uint64_t,
                );
                grain_2 = av_clip_c(grain_2, grain_min, grain_max);
                src = ((src_row as *const ::core::ffi::c_char)
                    .offset((y_0 as ptrdiff_t * stride) as isize) as *const uint8_t)
                    .offset(x_2 as isize)
                    .offset(bx as isize);
                dst = ((dst_row as *mut ::core::ffi::c_char)
                    .offset((y_0 as ptrdiff_t * stride) as isize) as *mut uint8_t)
                    .offset(x_2 as isize)
                    .offset(bx as isize);
                noise = round2(
                    *scaling.offset(*src as isize) as ::core::ffi::c_int * grain_2,
                    (*data).scaling_shift as uint64_t,
                );
                *dst = av_clip_c(
                    *src as ::core::ffi::c_int + noise,
                    min_value,
                    max_value,
                ) as uint8_t;
                x_2 += 1;
            }
            y_0 += 1;
        }
        bx = bx.wrapping_add(FG_BLOCK_SIZE as ::core::ffi::c_int as ::core::ffi::c_uint);
    }
}
#[inline(always)]
unsafe extern "C" fn av_clip_uint8_c(mut a: ::core::ffi::c_int) -> uint8_t {
    if a & !(0xff as ::core::ffi::c_int) != 0 {
        return (!a >> 31 as ::core::ffi::c_int) as uint8_t
    } else {
        return a as uint8_t
    };
}
#[inline]
unsafe extern "C" fn get_bits_count(mut s: *const GetBitContext) -> ::core::ffi::c_int {
    return (*s).index;
}
#[inline(always)]
unsafe extern "C" fn av_zero_extend_c(
    mut a: ::core::ffi::c_uint,
    mut p: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    return a & ((1 as ::core::ffi::c_uint) << p).wrapping_sub(1 as ::core::ffi::c_uint);
}
unsafe extern "C" fn fguv_32x32xn_c_8(
    dst_row: *mut uint8_t,
    src_row: *const uint8_t,
    stride: ptrdiff_t,
    params: *const AVFilmGrainParams,
    pw: size_t,
    mut scaling: *const uint8_t,
    mut grain_lut: *const [int8_t; 82],
    bh: ::core::ffi::c_int,
    row_num: ::core::ffi::c_int,
    luma_row: *const uint8_t,
    luma_stride: ptrdiff_t,
    uv: ::core::ffi::c_int,
    is_id: ::core::ffi::c_int,
    sx: ::core::ffi::c_int,
    sy: ::core::ffi::c_int,
) {
    let data: *const AVFilmGrainAOMParams = &raw const (*params).codec.aom;
    let rows: ::core::ffi::c_int = 1 as ::core::ffi::c_int
        + ((*data).overlap_flag != 0 && row_num > 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
    let bitdepth_min_8: ::core::ffi::c_int = 8 as ::core::ffi::c_int
        - 8 as ::core::ffi::c_int;
    let grain_ctr: ::core::ffi::c_int = (128 as ::core::ffi::c_int) << bitdepth_min_8;
    let grain_min: ::core::ffi::c_int = -grain_ctr;
    let grain_max: ::core::ffi::c_int = grain_ctr - 1 as ::core::ffi::c_int;
    let mut seed: [::core::ffi::c_uint; 2] = [0; 2];
    let mut offsets: [[::core::ffi::c_int; 2]; 2] = [[0; 2]; 2];
    let mut min_value: ::core::ffi::c_int = 0;
    let mut max_value: ::core::ffi::c_int = 0;
    if (*data).limit_output_range != 0 {
        min_value = (16 as ::core::ffi::c_int) << bitdepth_min_8;
        max_value = (if is_id != 0 {
            235 as ::core::ffi::c_int
        } else {
            240 as ::core::ffi::c_int
        }) << bitdepth_min_8;
    } else {
        min_value = 0 as ::core::ffi::c_int;
        max_value = 0xff as ::core::ffi::c_int;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < rows {
        seed[i as usize] = (*params).seed as ::core::ffi::c_uint;
        seed[i as usize]
            ^= (((row_num - i) * 37 as ::core::ffi::c_int + 178 as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                as ::core::ffi::c_uint;
        seed[i as usize]
            ^= ((row_num - i) * 173 as ::core::ffi::c_int + 105 as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uint;
        i += 1;
    }
    let mut bx: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while (bx as size_t) < pw {
        let bw: ::core::ffi::c_int = if FG_BLOCK_SIZE as ::core::ffi::c_int >> sx
            > pw.wrapping_sub(bx as size_t) as ::core::ffi::c_int
        {
            pw.wrapping_sub(bx as size_t) as ::core::ffi::c_int
        } else {
            FG_BLOCK_SIZE as ::core::ffi::c_int as ::core::ffi::c_int >> sx
        };
        let mut val: ::core::ffi::c_int = 0;
        let mut lx: ::core::ffi::c_int = 0;
        let mut ly: ::core::ffi::c_int = 0;
        let mut noise: ::core::ffi::c_int = 0;
        let mut src: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut luma: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut dst: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut avg: uint8_t = 0;
        let ystart: ::core::ffi::c_int = if (*data).overlap_flag != 0 && row_num != 0 {
            if 2 as ::core::ffi::c_int >> sy > bh {
                bh
            } else {
                2 as ::core::ffi::c_int >> sy
            }
        } else {
            0 as ::core::ffi::c_int
        };
        let xstart: ::core::ffi::c_int = if (*data).overlap_flag != 0 && bx != 0 {
            if 2 as ::core::ffi::c_int >> sx > bw {
                bw
            } else {
                2 as ::core::ffi::c_int >> sx
            }
        } else {
            0 as ::core::ffi::c_int
        };
        static mut w: [[[::core::ffi::c_int; 2]; 2]; 2] = [
            [
                [27 as ::core::ffi::c_int, 17 as ::core::ffi::c_int],
                [17 as ::core::ffi::c_int, 27 as ::core::ffi::c_int],
            ],
            [[23 as ::core::ffi::c_int, 22 as ::core::ffi::c_int], [0; 2]],
        ];
        if (*data).overlap_flag != 0 && bx != 0 {
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0 < rows {
                offsets[1 as ::core::ffi::c_int as usize][i_0 as usize] = offsets[0
                    as ::core::ffi::c_int as usize][i_0 as usize];
                i_0 += 1;
            }
        }
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1 < rows {
            offsets[0 as ::core::ffi::c_int as usize][i_1 as usize] = get_random_number(
                8 as ::core::ffi::c_int,
                (&raw mut seed as *mut ::core::ffi::c_uint).offset(i_1 as isize)
                    as *mut ::core::ffi::c_uint,
            );
            i_1 += 1;
        }
        let mut y: ::core::ffi::c_int = ystart;
        while y < bh {
            let mut x: ::core::ffi::c_int = xstart;
            while x < bw {
                let mut grain: ::core::ffi::c_int = sample_lut_8(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    sx,
                    sy,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    x,
                    y,
                ) as ::core::ffi::c_int;
                lx = (bx.wrapping_add(x as ::core::ffi::c_uint) << sx)
                    as ::core::ffi::c_int;
                ly = y << sy;
                luma = ((luma_row as *const ::core::ffi::c_char)
                    .offset((ly as ptrdiff_t * luma_stride) as isize) as *const uint8_t)
                    .offset(lx as isize);
                avg = *luma.offset(0 as ::core::ffi::c_int as isize);
                if sx != 0 {
                    avg = (avg as ::core::ffi::c_int
                        + *luma.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as uint8_t;
                }
                src = ((src_row as *const ::core::ffi::c_char)
                    .offset((y as ptrdiff_t * stride) as isize) as *const uint8_t)
                    .offset(bx as isize)
                    .offset(x as isize);
                dst = ((dst_row as *mut ::core::ffi::c_char)
                    .offset((y as ptrdiff_t * stride) as isize) as *mut uint8_t)
                    .offset(bx as isize)
                    .offset(x as isize);
                val = avg as ::core::ffi::c_int;
                if (*data).chroma_scaling_from_luma == 0 {
                    let combined: ::core::ffi::c_int = avg as ::core::ffi::c_int
                        * (*data).uv_mult_luma[uv as usize]
                        + *src as ::core::ffi::c_int * (*data).uv_mult[uv as usize];
                    val = av_clip_c(
                        (combined >> 6 as ::core::ffi::c_int)
                            + (*data).uv_offset[uv as usize]
                                * ((1 as ::core::ffi::c_int) << bitdepth_min_8),
                        0 as ::core::ffi::c_int,
                        0xff as ::core::ffi::c_int,
                    );
                }
                noise = round2(
                    *scaling.offset(val as isize) as ::core::ffi::c_int * grain,
                    (*data).scaling_shift as uint64_t,
                );
                *dst = av_clip_c(
                    *src as ::core::ffi::c_int + noise,
                    min_value,
                    max_value,
                ) as uint8_t;
                x += 1;
            }
            let mut x_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_0 < xstart {
                let mut grain_0: ::core::ffi::c_int = sample_lut_8(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    sx,
                    sy,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    x_0,
                    y,
                ) as ::core::ffi::c_int;
                let mut old: ::core::ffi::c_int = sample_lut_8(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    sx,
                    sy,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    x_0,
                    y,
                ) as ::core::ffi::c_int;
                grain_0 = round2(
                    old * w[sx as usize][x_0 as usize][0 as ::core::ffi::c_int as usize]
                        + grain_0
                            * w[sx
                                as usize][x_0 as usize][1 as ::core::ffi::c_int as usize],
                    5 as uint64_t,
                );
                grain_0 = av_clip_c(grain_0, grain_min, grain_max);
                lx = (bx.wrapping_add(x_0 as ::core::ffi::c_uint) << sx)
                    as ::core::ffi::c_int;
                ly = y << sy;
                luma = ((luma_row as *const ::core::ffi::c_char)
                    .offset((ly as ptrdiff_t * luma_stride) as isize) as *const uint8_t)
                    .offset(lx as isize);
                avg = *luma.offset(0 as ::core::ffi::c_int as isize);
                if sx != 0 {
                    avg = (avg as ::core::ffi::c_int
                        + *luma.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as uint8_t;
                }
                src = ((src_row as *const ::core::ffi::c_char)
                    .offset((y as ptrdiff_t * stride) as isize) as *const uint8_t)
                    .offset(bx as isize)
                    .offset(x_0 as isize);
                dst = ((dst_row as *mut ::core::ffi::c_char)
                    .offset((y as ptrdiff_t * stride) as isize) as *mut uint8_t)
                    .offset(bx as isize)
                    .offset(x_0 as isize);
                val = avg as ::core::ffi::c_int;
                if (*data).chroma_scaling_from_luma == 0 {
                    let combined_0: ::core::ffi::c_int = avg as ::core::ffi::c_int
                        * (*data).uv_mult_luma[uv as usize]
                        + *src as ::core::ffi::c_int * (*data).uv_mult[uv as usize];
                    val = av_clip_c(
                        (combined_0 >> 6 as ::core::ffi::c_int)
                            + (*data).uv_offset[uv as usize]
                                * ((1 as ::core::ffi::c_int) << bitdepth_min_8),
                        0 as ::core::ffi::c_int,
                        0xff as ::core::ffi::c_int,
                    );
                }
                noise = round2(
                    *scaling.offset(val as isize) as ::core::ffi::c_int * grain_0,
                    (*data).scaling_shift as uint64_t,
                );
                *dst = av_clip_c(
                    *src as ::core::ffi::c_int + noise,
                    min_value,
                    max_value,
                ) as uint8_t;
                x_0 += 1;
            }
            y += 1;
        }
        let mut y_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y_0 < ystart {
            let mut x_1: ::core::ffi::c_int = xstart;
            while x_1 < bw {
                let mut grain_1: ::core::ffi::c_int = sample_lut_8(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    sx,
                    sy,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    x_1,
                    y_0,
                ) as ::core::ffi::c_int;
                let mut old_0: ::core::ffi::c_int = sample_lut_8(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    sx,
                    sy,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    x_1,
                    y_0,
                ) as ::core::ffi::c_int;
                grain_1 = round2(
                    old_0
                        * w[sy as usize][y_0 as usize][0 as ::core::ffi::c_int as usize]
                        + grain_1
                            * w[sy
                                as usize][y_0 as usize][1 as ::core::ffi::c_int as usize],
                    5 as uint64_t,
                );
                grain_1 = av_clip_c(grain_1, grain_min, grain_max);
                lx = (bx.wrapping_add(x_1 as ::core::ffi::c_uint) << sx)
                    as ::core::ffi::c_int;
                ly = y_0 << sy;
                luma = ((luma_row as *const ::core::ffi::c_char)
                    .offset((ly as ptrdiff_t * luma_stride) as isize) as *const uint8_t)
                    .offset(lx as isize);
                avg = *luma.offset(0 as ::core::ffi::c_int as isize);
                if sx != 0 {
                    avg = (avg as ::core::ffi::c_int
                        + *luma.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as uint8_t;
                }
                src = ((src_row as *const ::core::ffi::c_char)
                    .offset((y_0 as ptrdiff_t * stride) as isize) as *const uint8_t)
                    .offset(bx as isize)
                    .offset(x_1 as isize);
                dst = ((dst_row as *mut ::core::ffi::c_char)
                    .offset((y_0 as ptrdiff_t * stride) as isize) as *mut uint8_t)
                    .offset(bx as isize)
                    .offset(x_1 as isize);
                val = avg as ::core::ffi::c_int;
                if (*data).chroma_scaling_from_luma == 0 {
                    let combined_1: ::core::ffi::c_int = avg as ::core::ffi::c_int
                        * (*data).uv_mult_luma[uv as usize]
                        + *src as ::core::ffi::c_int * (*data).uv_mult[uv as usize];
                    val = av_clip_c(
                        (combined_1 >> 6 as ::core::ffi::c_int)
                            + (*data).uv_offset[uv as usize]
                                * ((1 as ::core::ffi::c_int) << bitdepth_min_8),
                        0 as ::core::ffi::c_int,
                        0xff as ::core::ffi::c_int,
                    );
                }
                noise = round2(
                    *scaling.offset(val as isize) as ::core::ffi::c_int * grain_1,
                    (*data).scaling_shift as uint64_t,
                );
                *dst = av_clip_c(
                    *src as ::core::ffi::c_int + noise,
                    min_value,
                    max_value,
                ) as uint8_t;
                x_1 += 1;
            }
            let mut x_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_2 < xstart {
                let mut top: ::core::ffi::c_int = sample_lut_8(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    sx,
                    sy,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    x_2,
                    y_0,
                ) as ::core::ffi::c_int;
                let mut old_1: ::core::ffi::c_int = sample_lut_8(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    sx,
                    sy,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    x_2,
                    y_0,
                ) as ::core::ffi::c_int;
                let mut grain_2: ::core::ffi::c_int = sample_lut_8(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    sx,
                    sy,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    x_2,
                    y_0,
                ) as ::core::ffi::c_int;
                top = round2(
                    old_1
                        * w[sx as usize][x_2 as usize][0 as ::core::ffi::c_int as usize]
                        + top
                            * w[sx
                                as usize][x_2 as usize][1 as ::core::ffi::c_int as usize],
                    5 as uint64_t,
                );
                top = av_clip_c(top, grain_min, grain_max);
                old_1 = sample_lut_8(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    sx,
                    sy,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    x_2,
                    y_0,
                ) as ::core::ffi::c_int;
                grain_2 = round2(
                    old_1
                        * w[sx as usize][x_2 as usize][0 as ::core::ffi::c_int as usize]
                        + grain_2
                            * w[sx
                                as usize][x_2 as usize][1 as ::core::ffi::c_int as usize],
                    5 as uint64_t,
                );
                grain_2 = av_clip_c(grain_2, grain_min, grain_max);
                grain_2 = round2(
                    top * w[sy as usize][y_0 as usize][0 as ::core::ffi::c_int as usize]
                        + grain_2
                            * w[sy
                                as usize][y_0 as usize][1 as ::core::ffi::c_int as usize],
                    5 as uint64_t,
                );
                grain_2 = av_clip_c(grain_2, grain_min, grain_max);
                lx = (bx.wrapping_add(x_2 as ::core::ffi::c_uint) << sx)
                    as ::core::ffi::c_int;
                ly = y_0 << sy;
                luma = ((luma_row as *const ::core::ffi::c_char)
                    .offset((ly as ptrdiff_t * luma_stride) as isize) as *const uint8_t)
                    .offset(lx as isize);
                avg = *luma.offset(0 as ::core::ffi::c_int as isize);
                if sx != 0 {
                    avg = (avg as ::core::ffi::c_int
                        + *luma.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as uint8_t;
                }
                src = ((src_row as *const ::core::ffi::c_char)
                    .offset((y_0 as ptrdiff_t * stride) as isize) as *const uint8_t)
                    .offset(bx as isize)
                    .offset(x_2 as isize);
                dst = ((dst_row as *mut ::core::ffi::c_char)
                    .offset((y_0 as ptrdiff_t * stride) as isize) as *mut uint8_t)
                    .offset(bx as isize)
                    .offset(x_2 as isize);
                val = avg as ::core::ffi::c_int;
                if (*data).chroma_scaling_from_luma == 0 {
                    let combined_2: ::core::ffi::c_int = avg as ::core::ffi::c_int
                        * (*data).uv_mult_luma[uv as usize]
                        + *src as ::core::ffi::c_int * (*data).uv_mult[uv as usize];
                    val = av_clip_c(
                        (combined_2 >> 6 as ::core::ffi::c_int)
                            + (*data).uv_offset[uv as usize]
                                * ((1 as ::core::ffi::c_int) << bitdepth_min_8),
                        0 as ::core::ffi::c_int,
                        0xff as ::core::ffi::c_int,
                    );
                }
                noise = round2(
                    *scaling.offset(val as isize) as ::core::ffi::c_int * grain_2,
                    (*data).scaling_shift as uint64_t,
                );
                *dst = av_clip_c(
                    *src as ::core::ffi::c_int + noise,
                    min_value,
                    max_value,
                ) as uint8_t;
                x_2 += 1;
            }
            y_0 += 1;
        }
        bx = bx
            .wrapping_add(
                (FG_BLOCK_SIZE as ::core::ffi::c_int >> sx) as ::core::ffi::c_uint,
            );
    }
}
unsafe extern "C" fn fguv_32x32xn_c_16(
    dst_row: *mut uint16_t,
    src_row: *const uint16_t,
    stride: ptrdiff_t,
    params: *const AVFilmGrainParams,
    pw: size_t,
    mut scaling: *const uint8_t,
    mut grain_lut: *const [int16_t; 82],
    bh: ::core::ffi::c_int,
    row_num: ::core::ffi::c_int,
    luma_row: *const uint16_t,
    luma_stride: ptrdiff_t,
    uv: ::core::ffi::c_int,
    is_id: ::core::ffi::c_int,
    sx: ::core::ffi::c_int,
    sy: ::core::ffi::c_int,
    bitdepth: ::core::ffi::c_int,
) {
    let data: *const AVFilmGrainAOMParams = &raw const (*params).codec.aom;
    let rows: ::core::ffi::c_int = 1 as ::core::ffi::c_int
        + ((*data).overlap_flag != 0 && row_num > 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
    let bitdepth_min_8: ::core::ffi::c_int = bitdepth - 8 as ::core::ffi::c_int;
    let grain_ctr: ::core::ffi::c_int = (128 as ::core::ffi::c_int) << bitdepth_min_8;
    let grain_min: ::core::ffi::c_int = -grain_ctr;
    let grain_max: ::core::ffi::c_int = grain_ctr - 1 as ::core::ffi::c_int;
    let mut seed: [::core::ffi::c_uint; 2] = [0; 2];
    let mut offsets: [[::core::ffi::c_int; 2]; 2] = [[0; 2]; 2];
    let mut min_value: ::core::ffi::c_int = 0;
    let mut max_value: ::core::ffi::c_int = 0;
    if (*data).limit_output_range != 0 {
        min_value = (16 as ::core::ffi::c_int) << bitdepth_min_8;
        max_value = (if is_id != 0 {
            235 as ::core::ffi::c_int
        } else {
            240 as ::core::ffi::c_int
        }) << bitdepth_min_8;
    } else {
        min_value = 0 as ::core::ffi::c_int;
        max_value = ((1 as ::core::ffi::c_int) << bitdepth) - 1 as ::core::ffi::c_int;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < rows {
        seed[i as usize] = (*params).seed as ::core::ffi::c_uint;
        seed[i as usize]
            ^= (((row_num - i) * 37 as ::core::ffi::c_int + 178 as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                as ::core::ffi::c_uint;
        seed[i as usize]
            ^= ((row_num - i) * 173 as ::core::ffi::c_int + 105 as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uint;
        i += 1;
    }
    let mut bx: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while (bx as size_t) < pw {
        let bw: ::core::ffi::c_int = if FG_BLOCK_SIZE as ::core::ffi::c_int >> sx
            > pw.wrapping_sub(bx as size_t) as ::core::ffi::c_int
        {
            pw.wrapping_sub(bx as size_t) as ::core::ffi::c_int
        } else {
            FG_BLOCK_SIZE as ::core::ffi::c_int as ::core::ffi::c_int >> sx
        };
        let mut val: ::core::ffi::c_int = 0;
        let mut lx: ::core::ffi::c_int = 0;
        let mut ly: ::core::ffi::c_int = 0;
        let mut noise: ::core::ffi::c_int = 0;
        let mut src: *const uint16_t = ::core::ptr::null::<uint16_t>();
        let mut luma: *const uint16_t = ::core::ptr::null::<uint16_t>();
        let mut dst: *mut uint16_t = ::core::ptr::null_mut::<uint16_t>();
        let mut avg: uint16_t = 0;
        let ystart: ::core::ffi::c_int = if (*data).overlap_flag != 0 && row_num != 0 {
            if 2 as ::core::ffi::c_int >> sy > bh {
                bh
            } else {
                2 as ::core::ffi::c_int >> sy
            }
        } else {
            0 as ::core::ffi::c_int
        };
        let xstart: ::core::ffi::c_int = if (*data).overlap_flag != 0 && bx != 0 {
            if 2 as ::core::ffi::c_int >> sx > bw {
                bw
            } else {
                2 as ::core::ffi::c_int >> sx
            }
        } else {
            0 as ::core::ffi::c_int
        };
        static mut w: [[[::core::ffi::c_int; 2]; 2]; 2] = [
            [
                [27 as ::core::ffi::c_int, 17 as ::core::ffi::c_int],
                [17 as ::core::ffi::c_int, 27 as ::core::ffi::c_int],
            ],
            [[23 as ::core::ffi::c_int, 22 as ::core::ffi::c_int], [0; 2]],
        ];
        if (*data).overlap_flag != 0 && bx != 0 {
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0 < rows {
                offsets[1 as ::core::ffi::c_int as usize][i_0 as usize] = offsets[0
                    as ::core::ffi::c_int as usize][i_0 as usize];
                i_0 += 1;
            }
        }
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1 < rows {
            offsets[0 as ::core::ffi::c_int as usize][i_1 as usize] = get_random_number(
                8 as ::core::ffi::c_int,
                (&raw mut seed as *mut ::core::ffi::c_uint).offset(i_1 as isize)
                    as *mut ::core::ffi::c_uint,
            );
            i_1 += 1;
        }
        let mut y: ::core::ffi::c_int = ystart;
        while y < bh {
            let mut x: ::core::ffi::c_int = xstart;
            while x < bw {
                let mut grain: ::core::ffi::c_int = sample_lut_16(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    sx,
                    sy,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    x,
                    y,
                ) as ::core::ffi::c_int;
                lx = (bx.wrapping_add(x as ::core::ffi::c_uint) << sx)
                    as ::core::ffi::c_int;
                ly = y << sy;
                luma = ((luma_row as *const ::core::ffi::c_char)
                    .offset((ly as ptrdiff_t * luma_stride) as isize) as *const uint16_t)
                    .offset(lx as isize);
                avg = *luma.offset(0 as ::core::ffi::c_int as isize);
                if sx != 0 {
                    avg = (avg as ::core::ffi::c_int
                        + *luma.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as uint16_t;
                }
                src = ((src_row as *const ::core::ffi::c_char)
                    .offset((y as ptrdiff_t * stride) as isize) as *const uint16_t)
                    .offset(bx as isize)
                    .offset(x as isize);
                dst = ((dst_row as *mut ::core::ffi::c_char)
                    .offset((y as ptrdiff_t * stride) as isize) as *mut uint16_t)
                    .offset(bx as isize)
                    .offset(x as isize);
                val = avg as ::core::ffi::c_int;
                if (*data).chroma_scaling_from_luma == 0 {
                    let combined: ::core::ffi::c_int = avg as ::core::ffi::c_int
                        * (*data).uv_mult_luma[uv as usize]
                        + *src as ::core::ffi::c_int * (*data).uv_mult[uv as usize];
                    val = av_clip_c(
                        (combined >> 6 as ::core::ffi::c_int)
                            + (*data).uv_offset[uv as usize]
                                * ((1 as ::core::ffi::c_int) << bitdepth_min_8),
                        0 as ::core::ffi::c_int,
                        ((1 as ::core::ffi::c_int) << bitdepth) - 1 as ::core::ffi::c_int,
                    );
                }
                noise = round2(
                    *scaling.offset(val as isize) as ::core::ffi::c_int * grain,
                    (*data).scaling_shift as uint64_t,
                );
                *dst = av_clip_c(
                    *src as ::core::ffi::c_int + noise,
                    min_value,
                    max_value,
                ) as uint16_t;
                x += 1;
            }
            let mut x_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_0 < xstart {
                let mut grain_0: ::core::ffi::c_int = sample_lut_16(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    sx,
                    sy,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    x_0,
                    y,
                ) as ::core::ffi::c_int;
                let mut old: ::core::ffi::c_int = sample_lut_16(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    sx,
                    sy,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    x_0,
                    y,
                ) as ::core::ffi::c_int;
                grain_0 = round2(
                    old * w[sx as usize][x_0 as usize][0 as ::core::ffi::c_int as usize]
                        + grain_0
                            * w[sx
                                as usize][x_0 as usize][1 as ::core::ffi::c_int as usize],
                    5 as uint64_t,
                );
                grain_0 = av_clip_c(grain_0, grain_min, grain_max);
                lx = (bx.wrapping_add(x_0 as ::core::ffi::c_uint) << sx)
                    as ::core::ffi::c_int;
                ly = y << sy;
                luma = ((luma_row as *const ::core::ffi::c_char)
                    .offset((ly as ptrdiff_t * luma_stride) as isize) as *const uint16_t)
                    .offset(lx as isize);
                avg = *luma.offset(0 as ::core::ffi::c_int as isize);
                if sx != 0 {
                    avg = (avg as ::core::ffi::c_int
                        + *luma.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as uint16_t;
                }
                src = ((src_row as *const ::core::ffi::c_char)
                    .offset((y as ptrdiff_t * stride) as isize) as *const uint16_t)
                    .offset(bx as isize)
                    .offset(x_0 as isize);
                dst = ((dst_row as *mut ::core::ffi::c_char)
                    .offset((y as ptrdiff_t * stride) as isize) as *mut uint16_t)
                    .offset(bx as isize)
                    .offset(x_0 as isize);
                val = avg as ::core::ffi::c_int;
                if (*data).chroma_scaling_from_luma == 0 {
                    let combined_0: ::core::ffi::c_int = avg as ::core::ffi::c_int
                        * (*data).uv_mult_luma[uv as usize]
                        + *src as ::core::ffi::c_int * (*data).uv_mult[uv as usize];
                    val = av_clip_c(
                        (combined_0 >> 6 as ::core::ffi::c_int)
                            + (*data).uv_offset[uv as usize]
                                * ((1 as ::core::ffi::c_int) << bitdepth_min_8),
                        0 as ::core::ffi::c_int,
                        ((1 as ::core::ffi::c_int) << bitdepth) - 1 as ::core::ffi::c_int,
                    );
                }
                noise = round2(
                    *scaling.offset(val as isize) as ::core::ffi::c_int * grain_0,
                    (*data).scaling_shift as uint64_t,
                );
                *dst = av_clip_c(
                    *src as ::core::ffi::c_int + noise,
                    min_value,
                    max_value,
                ) as uint16_t;
                x_0 += 1;
            }
            y += 1;
        }
        let mut y_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y_0 < ystart {
            let mut x_1: ::core::ffi::c_int = xstart;
            while x_1 < bw {
                let mut grain_1: ::core::ffi::c_int = sample_lut_16(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    sx,
                    sy,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    x_1,
                    y_0,
                ) as ::core::ffi::c_int;
                let mut old_0: ::core::ffi::c_int = sample_lut_16(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    sx,
                    sy,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    x_1,
                    y_0,
                ) as ::core::ffi::c_int;
                grain_1 = round2(
                    old_0
                        * w[sy as usize][y_0 as usize][0 as ::core::ffi::c_int as usize]
                        + grain_1
                            * w[sy
                                as usize][y_0 as usize][1 as ::core::ffi::c_int as usize],
                    5 as uint64_t,
                );
                grain_1 = av_clip_c(grain_1, grain_min, grain_max);
                lx = (bx.wrapping_add(x_1 as ::core::ffi::c_uint) << sx)
                    as ::core::ffi::c_int;
                ly = y_0 << sy;
                luma = ((luma_row as *const ::core::ffi::c_char)
                    .offset((ly as ptrdiff_t * luma_stride) as isize) as *const uint16_t)
                    .offset(lx as isize);
                avg = *luma.offset(0 as ::core::ffi::c_int as isize);
                if sx != 0 {
                    avg = (avg as ::core::ffi::c_int
                        + *luma.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as uint16_t;
                }
                src = ((src_row as *const ::core::ffi::c_char)
                    .offset((y_0 as ptrdiff_t * stride) as isize) as *const uint16_t)
                    .offset(bx as isize)
                    .offset(x_1 as isize);
                dst = ((dst_row as *mut ::core::ffi::c_char)
                    .offset((y_0 as ptrdiff_t * stride) as isize) as *mut uint16_t)
                    .offset(bx as isize)
                    .offset(x_1 as isize);
                val = avg as ::core::ffi::c_int;
                if (*data).chroma_scaling_from_luma == 0 {
                    let combined_1: ::core::ffi::c_int = avg as ::core::ffi::c_int
                        * (*data).uv_mult_luma[uv as usize]
                        + *src as ::core::ffi::c_int * (*data).uv_mult[uv as usize];
                    val = av_clip_c(
                        (combined_1 >> 6 as ::core::ffi::c_int)
                            + (*data).uv_offset[uv as usize]
                                * ((1 as ::core::ffi::c_int) << bitdepth_min_8),
                        0 as ::core::ffi::c_int,
                        ((1 as ::core::ffi::c_int) << bitdepth) - 1 as ::core::ffi::c_int,
                    );
                }
                noise = round2(
                    *scaling.offset(val as isize) as ::core::ffi::c_int * grain_1,
                    (*data).scaling_shift as uint64_t,
                );
                *dst = av_clip_c(
                    *src as ::core::ffi::c_int + noise,
                    min_value,
                    max_value,
                ) as uint16_t;
                x_1 += 1;
            }
            let mut x_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_2 < xstart {
                let mut top: ::core::ffi::c_int = sample_lut_16(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    sx,
                    sy,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    x_2,
                    y_0,
                ) as ::core::ffi::c_int;
                let mut old_1: ::core::ffi::c_int = sample_lut_16(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    sx,
                    sy,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    x_2,
                    y_0,
                ) as ::core::ffi::c_int;
                let mut grain_2: ::core::ffi::c_int = sample_lut_16(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    sx,
                    sy,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    x_2,
                    y_0,
                ) as ::core::ffi::c_int;
                top = round2(
                    old_1
                        * w[sx as usize][x_2 as usize][0 as ::core::ffi::c_int as usize]
                        + top
                            * w[sx
                                as usize][x_2 as usize][1 as ::core::ffi::c_int as usize],
                    5 as uint64_t,
                );
                top = av_clip_c(top, grain_min, grain_max);
                old_1 = sample_lut_16(
                    grain_lut,
                    &raw mut offsets as *mut [::core::ffi::c_int; 2]
                        as *const [::core::ffi::c_int; 2],
                    sx,
                    sy,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    x_2,
                    y_0,
                ) as ::core::ffi::c_int;
                grain_2 = round2(
                    old_1
                        * w[sx as usize][x_2 as usize][0 as ::core::ffi::c_int as usize]
                        + grain_2
                            * w[sx
                                as usize][x_2 as usize][1 as ::core::ffi::c_int as usize],
                    5 as uint64_t,
                );
                grain_2 = av_clip_c(grain_2, grain_min, grain_max);
                grain_2 = round2(
                    top * w[sy as usize][y_0 as usize][0 as ::core::ffi::c_int as usize]
                        + grain_2
                            * w[sy
                                as usize][y_0 as usize][1 as ::core::ffi::c_int as usize],
                    5 as uint64_t,
                );
                grain_2 = av_clip_c(grain_2, grain_min, grain_max);
                lx = (bx.wrapping_add(x_2 as ::core::ffi::c_uint) << sx)
                    as ::core::ffi::c_int;
                ly = y_0 << sy;
                luma = ((luma_row as *const ::core::ffi::c_char)
                    .offset((ly as ptrdiff_t * luma_stride) as isize) as *const uint16_t)
                    .offset(lx as isize);
                avg = *luma.offset(0 as ::core::ffi::c_int as isize);
                if sx != 0 {
                    avg = (avg as ::core::ffi::c_int
                        + *luma.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as uint16_t;
                }
                src = ((src_row as *const ::core::ffi::c_char)
                    .offset((y_0 as ptrdiff_t * stride) as isize) as *const uint16_t)
                    .offset(bx as isize)
                    .offset(x_2 as isize);
                dst = ((dst_row as *mut ::core::ffi::c_char)
                    .offset((y_0 as ptrdiff_t * stride) as isize) as *mut uint16_t)
                    .offset(bx as isize)
                    .offset(x_2 as isize);
                val = avg as ::core::ffi::c_int;
                if (*data).chroma_scaling_from_luma == 0 {
                    let combined_2: ::core::ffi::c_int = avg as ::core::ffi::c_int
                        * (*data).uv_mult_luma[uv as usize]
                        + *src as ::core::ffi::c_int * (*data).uv_mult[uv as usize];
                    val = av_clip_c(
                        (combined_2 >> 6 as ::core::ffi::c_int)
                            + (*data).uv_offset[uv as usize]
                                * ((1 as ::core::ffi::c_int) << bitdepth_min_8),
                        0 as ::core::ffi::c_int,
                        ((1 as ::core::ffi::c_int) << bitdepth) - 1 as ::core::ffi::c_int,
                    );
                }
                noise = round2(
                    *scaling.offset(val as isize) as ::core::ffi::c_int * grain_2,
                    (*data).scaling_shift as uint64_t,
                );
                *dst = av_clip_c(
                    *src as ::core::ffi::c_int + noise,
                    min_value,
                    max_value,
                ) as uint16_t;
                x_2 += 1;
            }
            y_0 += 1;
        }
        bx = bx
            .wrapping_add(
                (FG_BLOCK_SIZE as ::core::ffi::c_int >> sx) as ::core::ffi::c_uint,
            );
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
pub unsafe extern "C" fn ff_aom_attach_film_grain_sets(
    mut s: *const AVFilmGrainAFGS1Params,
    mut frame: *mut AVFrame,
) -> ::core::ffi::c_int {
    if (*s).enable == 0 {
        return 0 as ::core::ffi::c_int;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (i as usize)
        < (::core::mem::size_of::<[*mut AVBufferRef; 8]>() as usize)
            .wrapping_div(::core::mem::size_of::<*mut AVBufferRef>() as usize)
    {
        if !(*s).sets[i as usize].is_null() {
            if av_frame_side_data_add(
                    &raw mut (*frame).side_data,
                    &raw mut (*frame).nb_side_data,
                    AV_FRAME_DATA_FILM_GRAIN_PARAMS,
                    (&raw const (*s).sets as *const *mut AVBufferRef).offset(i as isize)
                        as *const *mut AVBufferRef as *mut *mut AVBufferRef,
                    ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint,
                )
                .is_null()
            {
                return -(12 as ::core::ffi::c_int);
            }
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_aom_uninit_film_grain_params(
    mut s: *mut AVFilmGrainAFGS1Params,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (i as usize)
        < (::core::mem::size_of::<[*mut AVBufferRef; 8]>() as usize)
            .wrapping_div(::core::mem::size_of::<*mut AVBufferRef>() as usize)
    {
        av_buffer_unref(
            (&raw mut (*s).sets as *mut *mut AVBufferRef).offset(i as isize)
                as *mut *mut AVBufferRef,
        );
        i += 1;
    }
    (*s).enable = 0 as ::core::ffi::c_int;
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
static mut gaussian_sequence: [int16_t; 2048] = [
    56 as ::core::ffi::c_int as int16_t,
    568 as ::core::ffi::c_int as int16_t,
    -(180 as ::core::ffi::c_int) as int16_t,
    172 as ::core::ffi::c_int as int16_t,
    124 as ::core::ffi::c_int as int16_t,
    -(84 as ::core::ffi::c_int) as int16_t,
    172 as ::core::ffi::c_int as int16_t,
    -(64 as ::core::ffi::c_int) as int16_t,
    -(900 as ::core::ffi::c_int) as int16_t,
    24 as ::core::ffi::c_int as int16_t,
    820 as ::core::ffi::c_int as int16_t,
    224 as ::core::ffi::c_int as int16_t,
    1248 as ::core::ffi::c_int as int16_t,
    996 as ::core::ffi::c_int as int16_t,
    272 as ::core::ffi::c_int as int16_t,
    -(8 as ::core::ffi::c_int) as int16_t,
    -(916 as ::core::ffi::c_int) as int16_t,
    -(388 as ::core::ffi::c_int) as int16_t,
    -(732 as ::core::ffi::c_int) as int16_t,
    -(104 as ::core::ffi::c_int) as int16_t,
    -(188 as ::core::ffi::c_int) as int16_t,
    800 as ::core::ffi::c_int as int16_t,
    112 as ::core::ffi::c_int as int16_t,
    -(652 as ::core::ffi::c_int) as int16_t,
    -(320 as ::core::ffi::c_int) as int16_t,
    -(376 as ::core::ffi::c_int) as int16_t,
    140 as ::core::ffi::c_int as int16_t,
    -(252 as ::core::ffi::c_int) as int16_t,
    492 as ::core::ffi::c_int as int16_t,
    -(168 as ::core::ffi::c_int) as int16_t,
    44 as ::core::ffi::c_int as int16_t,
    -(788 as ::core::ffi::c_int) as int16_t,
    588 as ::core::ffi::c_int as int16_t,
    -(584 as ::core::ffi::c_int) as int16_t,
    500 as ::core::ffi::c_int as int16_t,
    -(228 as ::core::ffi::c_int) as int16_t,
    12 as ::core::ffi::c_int as int16_t,
    680 as ::core::ffi::c_int as int16_t,
    272 as ::core::ffi::c_int as int16_t,
    -(476 as ::core::ffi::c_int) as int16_t,
    972 as ::core::ffi::c_int as int16_t,
    -(100 as ::core::ffi::c_int) as int16_t,
    652 as ::core::ffi::c_int as int16_t,
    368 as ::core::ffi::c_int as int16_t,
    432 as ::core::ffi::c_int as int16_t,
    -(196 as ::core::ffi::c_int) as int16_t,
    -(720 as ::core::ffi::c_int) as int16_t,
    -(192 as ::core::ffi::c_int) as int16_t,
    1000 as ::core::ffi::c_int as int16_t,
    -(332 as ::core::ffi::c_int) as int16_t,
    652 as ::core::ffi::c_int as int16_t,
    -(136 as ::core::ffi::c_int) as int16_t,
    -(552 as ::core::ffi::c_int) as int16_t,
    -(604 as ::core::ffi::c_int) as int16_t,
    -(4 as ::core::ffi::c_int) as int16_t,
    192 as ::core::ffi::c_int as int16_t,
    -(220 as ::core::ffi::c_int) as int16_t,
    -(136 as ::core::ffi::c_int) as int16_t,
    1000 as ::core::ffi::c_int as int16_t,
    -(52 as ::core::ffi::c_int) as int16_t,
    372 as ::core::ffi::c_int as int16_t,
    -(96 as ::core::ffi::c_int) as int16_t,
    -(624 as ::core::ffi::c_int) as int16_t,
    124 as ::core::ffi::c_int as int16_t,
    -(24 as ::core::ffi::c_int) as int16_t,
    396 as ::core::ffi::c_int as int16_t,
    540 as ::core::ffi::c_int as int16_t,
    -(12 as ::core::ffi::c_int) as int16_t,
    -(104 as ::core::ffi::c_int) as int16_t,
    640 as ::core::ffi::c_int as int16_t,
    464 as ::core::ffi::c_int as int16_t,
    244 as ::core::ffi::c_int as int16_t,
    -(208 as ::core::ffi::c_int) as int16_t,
    -(84 as ::core::ffi::c_int) as int16_t,
    368 as ::core::ffi::c_int as int16_t,
    -(528 as ::core::ffi::c_int) as int16_t,
    -(740 as ::core::ffi::c_int) as int16_t,
    248 as ::core::ffi::c_int as int16_t,
    -(968 as ::core::ffi::c_int) as int16_t,
    -(848 as ::core::ffi::c_int) as int16_t,
    608 as ::core::ffi::c_int as int16_t,
    376 as ::core::ffi::c_int as int16_t,
    -(60 as ::core::ffi::c_int) as int16_t,
    -(292 as ::core::ffi::c_int) as int16_t,
    -(40 as ::core::ffi::c_int) as int16_t,
    -(156 as ::core::ffi::c_int) as int16_t,
    252 as ::core::ffi::c_int as int16_t,
    -(292 as ::core::ffi::c_int) as int16_t,
    248 as ::core::ffi::c_int as int16_t,
    224 as ::core::ffi::c_int as int16_t,
    -(280 as ::core::ffi::c_int) as int16_t,
    400 as ::core::ffi::c_int as int16_t,
    -(244 as ::core::ffi::c_int) as int16_t,
    244 as ::core::ffi::c_int as int16_t,
    -(60 as ::core::ffi::c_int) as int16_t,
    76 as ::core::ffi::c_int as int16_t,
    -(80 as ::core::ffi::c_int) as int16_t,
    212 as ::core::ffi::c_int as int16_t,
    532 as ::core::ffi::c_int as int16_t,
    340 as ::core::ffi::c_int as int16_t,
    128 as ::core::ffi::c_int as int16_t,
    -(36 as ::core::ffi::c_int) as int16_t,
    824 as ::core::ffi::c_int as int16_t,
    -(352 as ::core::ffi::c_int) as int16_t,
    -(60 as ::core::ffi::c_int) as int16_t,
    -(264 as ::core::ffi::c_int) as int16_t,
    -(96 as ::core::ffi::c_int) as int16_t,
    -(612 as ::core::ffi::c_int) as int16_t,
    416 as ::core::ffi::c_int as int16_t,
    -(704 as ::core::ffi::c_int) as int16_t,
    220 as ::core::ffi::c_int as int16_t,
    -(204 as ::core::ffi::c_int) as int16_t,
    640 as ::core::ffi::c_int as int16_t,
    -(160 as ::core::ffi::c_int) as int16_t,
    1220 as ::core::ffi::c_int as int16_t,
    -(408 as ::core::ffi::c_int) as int16_t,
    900 as ::core::ffi::c_int as int16_t,
    336 as ::core::ffi::c_int as int16_t,
    20 as ::core::ffi::c_int as int16_t,
    -(336 as ::core::ffi::c_int) as int16_t,
    -(96 as ::core::ffi::c_int) as int16_t,
    -(792 as ::core::ffi::c_int) as int16_t,
    304 as ::core::ffi::c_int as int16_t,
    48 as ::core::ffi::c_int as int16_t,
    -(28 as ::core::ffi::c_int) as int16_t,
    -(1232 as ::core::ffi::c_int) as int16_t,
    -(1172 as ::core::ffi::c_int) as int16_t,
    -(448 as ::core::ffi::c_int) as int16_t,
    104 as ::core::ffi::c_int as int16_t,
    -(292 as ::core::ffi::c_int) as int16_t,
    -(520 as ::core::ffi::c_int) as int16_t,
    244 as ::core::ffi::c_int as int16_t,
    60 as ::core::ffi::c_int as int16_t,
    -(948 as ::core::ffi::c_int) as int16_t,
    0 as ::core::ffi::c_int as int16_t,
    -(708 as ::core::ffi::c_int) as int16_t,
    268 as ::core::ffi::c_int as int16_t,
    108 as ::core::ffi::c_int as int16_t,
    356 as ::core::ffi::c_int as int16_t,
    -(548 as ::core::ffi::c_int) as int16_t,
    488 as ::core::ffi::c_int as int16_t,
    -(344 as ::core::ffi::c_int) as int16_t,
    -(136 as ::core::ffi::c_int) as int16_t,
    488 as ::core::ffi::c_int as int16_t,
    -(196 as ::core::ffi::c_int) as int16_t,
    -(224 as ::core::ffi::c_int) as int16_t,
    656 as ::core::ffi::c_int as int16_t,
    -(236 as ::core::ffi::c_int) as int16_t,
    -(1128 as ::core::ffi::c_int) as int16_t,
    60 as ::core::ffi::c_int as int16_t,
    4 as ::core::ffi::c_int as int16_t,
    140 as ::core::ffi::c_int as int16_t,
    276 as ::core::ffi::c_int as int16_t,
    -(676 as ::core::ffi::c_int) as int16_t,
    -(376 as ::core::ffi::c_int) as int16_t,
    168 as ::core::ffi::c_int as int16_t,
    -(108 as ::core::ffi::c_int) as int16_t,
    464 as ::core::ffi::c_int as int16_t,
    8 as ::core::ffi::c_int as int16_t,
    564 as ::core::ffi::c_int as int16_t,
    64 as ::core::ffi::c_int as int16_t,
    240 as ::core::ffi::c_int as int16_t,
    308 as ::core::ffi::c_int as int16_t,
    -(300 as ::core::ffi::c_int) as int16_t,
    -(400 as ::core::ffi::c_int) as int16_t,
    -(456 as ::core::ffi::c_int) as int16_t,
    -(136 as ::core::ffi::c_int) as int16_t,
    56 as ::core::ffi::c_int as int16_t,
    120 as ::core::ffi::c_int as int16_t,
    -(408 as ::core::ffi::c_int) as int16_t,
    -(116 as ::core::ffi::c_int) as int16_t,
    436 as ::core::ffi::c_int as int16_t,
    504 as ::core::ffi::c_int as int16_t,
    -(232 as ::core::ffi::c_int) as int16_t,
    328 as ::core::ffi::c_int as int16_t,
    844 as ::core::ffi::c_int as int16_t,
    -(164 as ::core::ffi::c_int) as int16_t,
    -(84 as ::core::ffi::c_int) as int16_t,
    784 as ::core::ffi::c_int as int16_t,
    -(168 as ::core::ffi::c_int) as int16_t,
    232 as ::core::ffi::c_int as int16_t,
    -(224 as ::core::ffi::c_int) as int16_t,
    348 as ::core::ffi::c_int as int16_t,
    -(376 as ::core::ffi::c_int) as int16_t,
    128 as ::core::ffi::c_int as int16_t,
    568 as ::core::ffi::c_int as int16_t,
    96 as ::core::ffi::c_int as int16_t,
    -(1244 as ::core::ffi::c_int) as int16_t,
    -(288 as ::core::ffi::c_int) as int16_t,
    276 as ::core::ffi::c_int as int16_t,
    848 as ::core::ffi::c_int as int16_t,
    832 as ::core::ffi::c_int as int16_t,
    -(360 as ::core::ffi::c_int) as int16_t,
    656 as ::core::ffi::c_int as int16_t,
    464 as ::core::ffi::c_int as int16_t,
    -(384 as ::core::ffi::c_int) as int16_t,
    -(332 as ::core::ffi::c_int) as int16_t,
    -(356 as ::core::ffi::c_int) as int16_t,
    728 as ::core::ffi::c_int as int16_t,
    -(388 as ::core::ffi::c_int) as int16_t,
    160 as ::core::ffi::c_int as int16_t,
    -(192 as ::core::ffi::c_int) as int16_t,
    468 as ::core::ffi::c_int as int16_t,
    296 as ::core::ffi::c_int as int16_t,
    224 as ::core::ffi::c_int as int16_t,
    140 as ::core::ffi::c_int as int16_t,
    -(776 as ::core::ffi::c_int) as int16_t,
    -(100 as ::core::ffi::c_int) as int16_t,
    280 as ::core::ffi::c_int as int16_t,
    4 as ::core::ffi::c_int as int16_t,
    196 as ::core::ffi::c_int as int16_t,
    44 as ::core::ffi::c_int as int16_t,
    -(36 as ::core::ffi::c_int) as int16_t,
    -(648 as ::core::ffi::c_int) as int16_t,
    932 as ::core::ffi::c_int as int16_t,
    16 as ::core::ffi::c_int as int16_t,
    1428 as ::core::ffi::c_int as int16_t,
    28 as ::core::ffi::c_int as int16_t,
    528 as ::core::ffi::c_int as int16_t,
    808 as ::core::ffi::c_int as int16_t,
    772 as ::core::ffi::c_int as int16_t,
    20 as ::core::ffi::c_int as int16_t,
    268 as ::core::ffi::c_int as int16_t,
    88 as ::core::ffi::c_int as int16_t,
    -(332 as ::core::ffi::c_int) as int16_t,
    -(284 as ::core::ffi::c_int) as int16_t,
    124 as ::core::ffi::c_int as int16_t,
    -(384 as ::core::ffi::c_int) as int16_t,
    -(448 as ::core::ffi::c_int) as int16_t,
    208 as ::core::ffi::c_int as int16_t,
    -(228 as ::core::ffi::c_int) as int16_t,
    -(1044 as ::core::ffi::c_int) as int16_t,
    -(328 as ::core::ffi::c_int) as int16_t,
    660 as ::core::ffi::c_int as int16_t,
    380 as ::core::ffi::c_int as int16_t,
    -(148 as ::core::ffi::c_int) as int16_t,
    -(300 as ::core::ffi::c_int) as int16_t,
    588 as ::core::ffi::c_int as int16_t,
    240 as ::core::ffi::c_int as int16_t,
    540 as ::core::ffi::c_int as int16_t,
    28 as ::core::ffi::c_int as int16_t,
    136 as ::core::ffi::c_int as int16_t,
    -(88 as ::core::ffi::c_int) as int16_t,
    -(436 as ::core::ffi::c_int) as int16_t,
    256 as ::core::ffi::c_int as int16_t,
    296 as ::core::ffi::c_int as int16_t,
    -(1000 as ::core::ffi::c_int) as int16_t,
    1400 as ::core::ffi::c_int as int16_t,
    0 as ::core::ffi::c_int as int16_t,
    -(48 as ::core::ffi::c_int) as int16_t,
    1056 as ::core::ffi::c_int as int16_t,
    -(136 as ::core::ffi::c_int) as int16_t,
    264 as ::core::ffi::c_int as int16_t,
    -(528 as ::core::ffi::c_int) as int16_t,
    -(1108 as ::core::ffi::c_int) as int16_t,
    632 as ::core::ffi::c_int as int16_t,
    -(484 as ::core::ffi::c_int) as int16_t,
    -(592 as ::core::ffi::c_int) as int16_t,
    -(344 as ::core::ffi::c_int) as int16_t,
    796 as ::core::ffi::c_int as int16_t,
    124 as ::core::ffi::c_int as int16_t,
    -(668 as ::core::ffi::c_int) as int16_t,
    -(768 as ::core::ffi::c_int) as int16_t,
    388 as ::core::ffi::c_int as int16_t,
    1296 as ::core::ffi::c_int as int16_t,
    -(232 as ::core::ffi::c_int) as int16_t,
    -(188 as ::core::ffi::c_int) as int16_t,
    -(200 as ::core::ffi::c_int) as int16_t,
    -(288 as ::core::ffi::c_int) as int16_t,
    -(4 as ::core::ffi::c_int) as int16_t,
    308 as ::core::ffi::c_int as int16_t,
    100 as ::core::ffi::c_int as int16_t,
    -(168 as ::core::ffi::c_int) as int16_t,
    256 as ::core::ffi::c_int as int16_t,
    -(500 as ::core::ffi::c_int) as int16_t,
    204 as ::core::ffi::c_int as int16_t,
    -(508 as ::core::ffi::c_int) as int16_t,
    648 as ::core::ffi::c_int as int16_t,
    -(136 as ::core::ffi::c_int) as int16_t,
    372 as ::core::ffi::c_int as int16_t,
    -(272 as ::core::ffi::c_int) as int16_t,
    -(120 as ::core::ffi::c_int) as int16_t,
    -(1004 as ::core::ffi::c_int) as int16_t,
    -(552 as ::core::ffi::c_int) as int16_t,
    -(548 as ::core::ffi::c_int) as int16_t,
    -(384 as ::core::ffi::c_int) as int16_t,
    548 as ::core::ffi::c_int as int16_t,
    -(296 as ::core::ffi::c_int) as int16_t,
    428 as ::core::ffi::c_int as int16_t,
    -(108 as ::core::ffi::c_int) as int16_t,
    -(8 as ::core::ffi::c_int) as int16_t,
    -(912 as ::core::ffi::c_int) as int16_t,
    -(324 as ::core::ffi::c_int) as int16_t,
    -(224 as ::core::ffi::c_int) as int16_t,
    -(88 as ::core::ffi::c_int) as int16_t,
    -(112 as ::core::ffi::c_int) as int16_t,
    -(220 as ::core::ffi::c_int) as int16_t,
    -(100 as ::core::ffi::c_int) as int16_t,
    996 as ::core::ffi::c_int as int16_t,
    -(796 as ::core::ffi::c_int) as int16_t,
    548 as ::core::ffi::c_int as int16_t,
    360 as ::core::ffi::c_int as int16_t,
    -(216 as ::core::ffi::c_int) as int16_t,
    180 as ::core::ffi::c_int as int16_t,
    428 as ::core::ffi::c_int as int16_t,
    -(200 as ::core::ffi::c_int) as int16_t,
    -(212 as ::core::ffi::c_int) as int16_t,
    148 as ::core::ffi::c_int as int16_t,
    96 as ::core::ffi::c_int as int16_t,
    148 as ::core::ffi::c_int as int16_t,
    284 as ::core::ffi::c_int as int16_t,
    216 as ::core::ffi::c_int as int16_t,
    -(412 as ::core::ffi::c_int) as int16_t,
    -(320 as ::core::ffi::c_int) as int16_t,
    120 as ::core::ffi::c_int as int16_t,
    -(300 as ::core::ffi::c_int) as int16_t,
    -(384 as ::core::ffi::c_int) as int16_t,
    -(604 as ::core::ffi::c_int) as int16_t,
    -(572 as ::core::ffi::c_int) as int16_t,
    -(332 as ::core::ffi::c_int) as int16_t,
    -(8 as ::core::ffi::c_int) as int16_t,
    -(180 as ::core::ffi::c_int) as int16_t,
    -(176 as ::core::ffi::c_int) as int16_t,
    696 as ::core::ffi::c_int as int16_t,
    116 as ::core::ffi::c_int as int16_t,
    -(88 as ::core::ffi::c_int) as int16_t,
    628 as ::core::ffi::c_int as int16_t,
    76 as ::core::ffi::c_int as int16_t,
    44 as ::core::ffi::c_int as int16_t,
    -(516 as ::core::ffi::c_int) as int16_t,
    240 as ::core::ffi::c_int as int16_t,
    -(208 as ::core::ffi::c_int) as int16_t,
    -(40 as ::core::ffi::c_int) as int16_t,
    100 as ::core::ffi::c_int as int16_t,
    -(592 as ::core::ffi::c_int) as int16_t,
    344 as ::core::ffi::c_int as int16_t,
    -(308 as ::core::ffi::c_int) as int16_t,
    -(452 as ::core::ffi::c_int) as int16_t,
    -(228 as ::core::ffi::c_int) as int16_t,
    20 as ::core::ffi::c_int as int16_t,
    916 as ::core::ffi::c_int as int16_t,
    -(1752 as ::core::ffi::c_int) as int16_t,
    -(136 as ::core::ffi::c_int) as int16_t,
    -(340 as ::core::ffi::c_int) as int16_t,
    -(804 as ::core::ffi::c_int) as int16_t,
    140 as ::core::ffi::c_int as int16_t,
    40 as ::core::ffi::c_int as int16_t,
    512 as ::core::ffi::c_int as int16_t,
    340 as ::core::ffi::c_int as int16_t,
    248 as ::core::ffi::c_int as int16_t,
    184 as ::core::ffi::c_int as int16_t,
    -(492 as ::core::ffi::c_int) as int16_t,
    896 as ::core::ffi::c_int as int16_t,
    -(156 as ::core::ffi::c_int) as int16_t,
    932 as ::core::ffi::c_int as int16_t,
    -(628 as ::core::ffi::c_int) as int16_t,
    328 as ::core::ffi::c_int as int16_t,
    -(688 as ::core::ffi::c_int) as int16_t,
    -(448 as ::core::ffi::c_int) as int16_t,
    -(616 as ::core::ffi::c_int) as int16_t,
    -(752 as ::core::ffi::c_int) as int16_t,
    -(100 as ::core::ffi::c_int) as int16_t,
    560 as ::core::ffi::c_int as int16_t,
    -(1020 as ::core::ffi::c_int) as int16_t,
    180 as ::core::ffi::c_int as int16_t,
    -(800 as ::core::ffi::c_int) as int16_t,
    -(64 as ::core::ffi::c_int) as int16_t,
    76 as ::core::ffi::c_int as int16_t,
    576 as ::core::ffi::c_int as int16_t,
    1068 as ::core::ffi::c_int as int16_t,
    396 as ::core::ffi::c_int as int16_t,
    660 as ::core::ffi::c_int as int16_t,
    552 as ::core::ffi::c_int as int16_t,
    -(108 as ::core::ffi::c_int) as int16_t,
    -(28 as ::core::ffi::c_int) as int16_t,
    320 as ::core::ffi::c_int as int16_t,
    -(628 as ::core::ffi::c_int) as int16_t,
    312 as ::core::ffi::c_int as int16_t,
    -(92 as ::core::ffi::c_int) as int16_t,
    -(92 as ::core::ffi::c_int) as int16_t,
    -(472 as ::core::ffi::c_int) as int16_t,
    268 as ::core::ffi::c_int as int16_t,
    16 as ::core::ffi::c_int as int16_t,
    560 as ::core::ffi::c_int as int16_t,
    516 as ::core::ffi::c_int as int16_t,
    -(672 as ::core::ffi::c_int) as int16_t,
    -(52 as ::core::ffi::c_int) as int16_t,
    492 as ::core::ffi::c_int as int16_t,
    -(100 as ::core::ffi::c_int) as int16_t,
    260 as ::core::ffi::c_int as int16_t,
    384 as ::core::ffi::c_int as int16_t,
    284 as ::core::ffi::c_int as int16_t,
    292 as ::core::ffi::c_int as int16_t,
    304 as ::core::ffi::c_int as int16_t,
    -(148 as ::core::ffi::c_int) as int16_t,
    88 as ::core::ffi::c_int as int16_t,
    -(152 as ::core::ffi::c_int) as int16_t,
    1012 as ::core::ffi::c_int as int16_t,
    1064 as ::core::ffi::c_int as int16_t,
    -(228 as ::core::ffi::c_int) as int16_t,
    164 as ::core::ffi::c_int as int16_t,
    -(376 as ::core::ffi::c_int) as int16_t,
    -(684 as ::core::ffi::c_int) as int16_t,
    592 as ::core::ffi::c_int as int16_t,
    -(392 as ::core::ffi::c_int) as int16_t,
    156 as ::core::ffi::c_int as int16_t,
    196 as ::core::ffi::c_int as int16_t,
    -(524 as ::core::ffi::c_int) as int16_t,
    -(64 as ::core::ffi::c_int) as int16_t,
    -(884 as ::core::ffi::c_int) as int16_t,
    160 as ::core::ffi::c_int as int16_t,
    -(176 as ::core::ffi::c_int) as int16_t,
    636 as ::core::ffi::c_int as int16_t,
    648 as ::core::ffi::c_int as int16_t,
    404 as ::core::ffi::c_int as int16_t,
    -(396 as ::core::ffi::c_int) as int16_t,
    -(436 as ::core::ffi::c_int) as int16_t,
    864 as ::core::ffi::c_int as int16_t,
    424 as ::core::ffi::c_int as int16_t,
    -(728 as ::core::ffi::c_int) as int16_t,
    988 as ::core::ffi::c_int as int16_t,
    -(604 as ::core::ffi::c_int) as int16_t,
    904 as ::core::ffi::c_int as int16_t,
    -(592 as ::core::ffi::c_int) as int16_t,
    296 as ::core::ffi::c_int as int16_t,
    -(224 as ::core::ffi::c_int) as int16_t,
    536 as ::core::ffi::c_int as int16_t,
    -(176 as ::core::ffi::c_int) as int16_t,
    -(920 as ::core::ffi::c_int) as int16_t,
    436 as ::core::ffi::c_int as int16_t,
    -(48 as ::core::ffi::c_int) as int16_t,
    1176 as ::core::ffi::c_int as int16_t,
    -(884 as ::core::ffi::c_int) as int16_t,
    416 as ::core::ffi::c_int as int16_t,
    -(776 as ::core::ffi::c_int) as int16_t,
    -(824 as ::core::ffi::c_int) as int16_t,
    -(884 as ::core::ffi::c_int) as int16_t,
    524 as ::core::ffi::c_int as int16_t,
    -(548 as ::core::ffi::c_int) as int16_t,
    -(564 as ::core::ffi::c_int) as int16_t,
    -(68 as ::core::ffi::c_int) as int16_t,
    -(164 as ::core::ffi::c_int) as int16_t,
    -(96 as ::core::ffi::c_int) as int16_t,
    692 as ::core::ffi::c_int as int16_t,
    364 as ::core::ffi::c_int as int16_t,
    -(692 as ::core::ffi::c_int) as int16_t,
    -(1012 as ::core::ffi::c_int) as int16_t,
    -(68 as ::core::ffi::c_int) as int16_t,
    260 as ::core::ffi::c_int as int16_t,
    -(480 as ::core::ffi::c_int) as int16_t,
    876 as ::core::ffi::c_int as int16_t,
    -(1116 as ::core::ffi::c_int) as int16_t,
    452 as ::core::ffi::c_int as int16_t,
    -(332 as ::core::ffi::c_int) as int16_t,
    -(352 as ::core::ffi::c_int) as int16_t,
    892 as ::core::ffi::c_int as int16_t,
    -(1088 as ::core::ffi::c_int) as int16_t,
    1220 as ::core::ffi::c_int as int16_t,
    -(676 as ::core::ffi::c_int) as int16_t,
    12 as ::core::ffi::c_int as int16_t,
    -(292 as ::core::ffi::c_int) as int16_t,
    244 as ::core::ffi::c_int as int16_t,
    496 as ::core::ffi::c_int as int16_t,
    372 as ::core::ffi::c_int as int16_t,
    -(32 as ::core::ffi::c_int) as int16_t,
    280 as ::core::ffi::c_int as int16_t,
    200 as ::core::ffi::c_int as int16_t,
    112 as ::core::ffi::c_int as int16_t,
    -(440 as ::core::ffi::c_int) as int16_t,
    -(96 as ::core::ffi::c_int) as int16_t,
    24 as ::core::ffi::c_int as int16_t,
    -(644 as ::core::ffi::c_int) as int16_t,
    -(184 as ::core::ffi::c_int) as int16_t,
    56 as ::core::ffi::c_int as int16_t,
    -(432 as ::core::ffi::c_int) as int16_t,
    224 as ::core::ffi::c_int as int16_t,
    -(980 as ::core::ffi::c_int) as int16_t,
    272 as ::core::ffi::c_int as int16_t,
    -(260 as ::core::ffi::c_int) as int16_t,
    144 as ::core::ffi::c_int as int16_t,
    -(436 as ::core::ffi::c_int) as int16_t,
    420 as ::core::ffi::c_int as int16_t,
    356 as ::core::ffi::c_int as int16_t,
    364 as ::core::ffi::c_int as int16_t,
    -(528 as ::core::ffi::c_int) as int16_t,
    76 as ::core::ffi::c_int as int16_t,
    172 as ::core::ffi::c_int as int16_t,
    -(744 as ::core::ffi::c_int) as int16_t,
    -(368 as ::core::ffi::c_int) as int16_t,
    404 as ::core::ffi::c_int as int16_t,
    -(752 as ::core::ffi::c_int) as int16_t,
    -(416 as ::core::ffi::c_int) as int16_t,
    684 as ::core::ffi::c_int as int16_t,
    -(688 as ::core::ffi::c_int) as int16_t,
    72 as ::core::ffi::c_int as int16_t,
    540 as ::core::ffi::c_int as int16_t,
    416 as ::core::ffi::c_int as int16_t,
    92 as ::core::ffi::c_int as int16_t,
    444 as ::core::ffi::c_int as int16_t,
    480 as ::core::ffi::c_int as int16_t,
    -(72 as ::core::ffi::c_int) as int16_t,
    -(1416 as ::core::ffi::c_int) as int16_t,
    164 as ::core::ffi::c_int as int16_t,
    -(1172 as ::core::ffi::c_int) as int16_t,
    -(68 as ::core::ffi::c_int) as int16_t,
    24 as ::core::ffi::c_int as int16_t,
    424 as ::core::ffi::c_int as int16_t,
    264 as ::core::ffi::c_int as int16_t,
    1040 as ::core::ffi::c_int as int16_t,
    128 as ::core::ffi::c_int as int16_t,
    -(912 as ::core::ffi::c_int) as int16_t,
    -(524 as ::core::ffi::c_int) as int16_t,
    -(356 as ::core::ffi::c_int) as int16_t,
    64 as ::core::ffi::c_int as int16_t,
    876 as ::core::ffi::c_int as int16_t,
    -(12 as ::core::ffi::c_int) as int16_t,
    4 as ::core::ffi::c_int as int16_t,
    -(88 as ::core::ffi::c_int) as int16_t,
    532 as ::core::ffi::c_int as int16_t,
    272 as ::core::ffi::c_int as int16_t,
    -(524 as ::core::ffi::c_int) as int16_t,
    320 as ::core::ffi::c_int as int16_t,
    276 as ::core::ffi::c_int as int16_t,
    -(508 as ::core::ffi::c_int) as int16_t,
    940 as ::core::ffi::c_int as int16_t,
    24 as ::core::ffi::c_int as int16_t,
    -(400 as ::core::ffi::c_int) as int16_t,
    -(120 as ::core::ffi::c_int) as int16_t,
    756 as ::core::ffi::c_int as int16_t,
    60 as ::core::ffi::c_int as int16_t,
    236 as ::core::ffi::c_int as int16_t,
    -(412 as ::core::ffi::c_int) as int16_t,
    100 as ::core::ffi::c_int as int16_t,
    376 as ::core::ffi::c_int as int16_t,
    -(484 as ::core::ffi::c_int) as int16_t,
    400 as ::core::ffi::c_int as int16_t,
    -(100 as ::core::ffi::c_int) as int16_t,
    -(740 as ::core::ffi::c_int) as int16_t,
    -(108 as ::core::ffi::c_int) as int16_t,
    -(260 as ::core::ffi::c_int) as int16_t,
    328 as ::core::ffi::c_int as int16_t,
    -(268 as ::core::ffi::c_int) as int16_t,
    224 as ::core::ffi::c_int as int16_t,
    -(200 as ::core::ffi::c_int) as int16_t,
    -(416 as ::core::ffi::c_int) as int16_t,
    184 as ::core::ffi::c_int as int16_t,
    -(604 as ::core::ffi::c_int) as int16_t,
    -(564 as ::core::ffi::c_int) as int16_t,
    -(20 as ::core::ffi::c_int) as int16_t,
    296 as ::core::ffi::c_int as int16_t,
    60 as ::core::ffi::c_int as int16_t,
    892 as ::core::ffi::c_int as int16_t,
    -(888 as ::core::ffi::c_int) as int16_t,
    60 as ::core::ffi::c_int as int16_t,
    164 as ::core::ffi::c_int as int16_t,
    68 as ::core::ffi::c_int as int16_t,
    -(760 as ::core::ffi::c_int) as int16_t,
    216 as ::core::ffi::c_int as int16_t,
    -(296 as ::core::ffi::c_int) as int16_t,
    904 as ::core::ffi::c_int as int16_t,
    -(336 as ::core::ffi::c_int) as int16_t,
    -(28 as ::core::ffi::c_int) as int16_t,
    404 as ::core::ffi::c_int as int16_t,
    -(356 as ::core::ffi::c_int) as int16_t,
    -(568 as ::core::ffi::c_int) as int16_t,
    -(208 as ::core::ffi::c_int) as int16_t,
    -(1480 as ::core::ffi::c_int) as int16_t,
    -(512 as ::core::ffi::c_int) as int16_t,
    296 as ::core::ffi::c_int as int16_t,
    328 as ::core::ffi::c_int as int16_t,
    -(360 as ::core::ffi::c_int) as int16_t,
    -(164 as ::core::ffi::c_int) as int16_t,
    -(1560 as ::core::ffi::c_int) as int16_t,
    -(776 as ::core::ffi::c_int) as int16_t,
    1156 as ::core::ffi::c_int as int16_t,
    -(428 as ::core::ffi::c_int) as int16_t,
    164 as ::core::ffi::c_int as int16_t,
    -(504 as ::core::ffi::c_int) as int16_t,
    -(112 as ::core::ffi::c_int) as int16_t,
    120 as ::core::ffi::c_int as int16_t,
    -(216 as ::core::ffi::c_int) as int16_t,
    -(148 as ::core::ffi::c_int) as int16_t,
    -(264 as ::core::ffi::c_int) as int16_t,
    308 as ::core::ffi::c_int as int16_t,
    32 as ::core::ffi::c_int as int16_t,
    64 as ::core::ffi::c_int as int16_t,
    -(72 as ::core::ffi::c_int) as int16_t,
    72 as ::core::ffi::c_int as int16_t,
    116 as ::core::ffi::c_int as int16_t,
    176 as ::core::ffi::c_int as int16_t,
    -(64 as ::core::ffi::c_int) as int16_t,
    -(272 as ::core::ffi::c_int) as int16_t,
    460 as ::core::ffi::c_int as int16_t,
    -(536 as ::core::ffi::c_int) as int16_t,
    -(784 as ::core::ffi::c_int) as int16_t,
    -(280 as ::core::ffi::c_int) as int16_t,
    348 as ::core::ffi::c_int as int16_t,
    108 as ::core::ffi::c_int as int16_t,
    -(752 as ::core::ffi::c_int) as int16_t,
    -(132 as ::core::ffi::c_int) as int16_t,
    524 as ::core::ffi::c_int as int16_t,
    -(540 as ::core::ffi::c_int) as int16_t,
    -(776 as ::core::ffi::c_int) as int16_t,
    116 as ::core::ffi::c_int as int16_t,
    -(296 as ::core::ffi::c_int) as int16_t,
    -(1196 as ::core::ffi::c_int) as int16_t,
    -(288 as ::core::ffi::c_int) as int16_t,
    -(560 as ::core::ffi::c_int) as int16_t,
    1040 as ::core::ffi::c_int as int16_t,
    -(472 as ::core::ffi::c_int) as int16_t,
    116 as ::core::ffi::c_int as int16_t,
    -(848 as ::core::ffi::c_int) as int16_t,
    -(1116 as ::core::ffi::c_int) as int16_t,
    116 as ::core::ffi::c_int as int16_t,
    636 as ::core::ffi::c_int as int16_t,
    696 as ::core::ffi::c_int as int16_t,
    284 as ::core::ffi::c_int as int16_t,
    -(176 as ::core::ffi::c_int) as int16_t,
    1016 as ::core::ffi::c_int as int16_t,
    204 as ::core::ffi::c_int as int16_t,
    -(864 as ::core::ffi::c_int) as int16_t,
    -(648 as ::core::ffi::c_int) as int16_t,
    -(248 as ::core::ffi::c_int) as int16_t,
    356 as ::core::ffi::c_int as int16_t,
    972 as ::core::ffi::c_int as int16_t,
    -(584 as ::core::ffi::c_int) as int16_t,
    -(204 as ::core::ffi::c_int) as int16_t,
    264 as ::core::ffi::c_int as int16_t,
    880 as ::core::ffi::c_int as int16_t,
    528 as ::core::ffi::c_int as int16_t,
    -(24 as ::core::ffi::c_int) as int16_t,
    -(184 as ::core::ffi::c_int) as int16_t,
    116 as ::core::ffi::c_int as int16_t,
    448 as ::core::ffi::c_int as int16_t,
    -(144 as ::core::ffi::c_int) as int16_t,
    828 as ::core::ffi::c_int as int16_t,
    524 as ::core::ffi::c_int as int16_t,
    212 as ::core::ffi::c_int as int16_t,
    -(212 as ::core::ffi::c_int) as int16_t,
    52 as ::core::ffi::c_int as int16_t,
    12 as ::core::ffi::c_int as int16_t,
    200 as ::core::ffi::c_int as int16_t,
    268 as ::core::ffi::c_int as int16_t,
    -(488 as ::core::ffi::c_int) as int16_t,
    -(404 as ::core::ffi::c_int) as int16_t,
    -(880 as ::core::ffi::c_int) as int16_t,
    824 as ::core::ffi::c_int as int16_t,
    -(672 as ::core::ffi::c_int) as int16_t,
    -(40 as ::core::ffi::c_int) as int16_t,
    908 as ::core::ffi::c_int as int16_t,
    -(248 as ::core::ffi::c_int) as int16_t,
    500 as ::core::ffi::c_int as int16_t,
    716 as ::core::ffi::c_int as int16_t,
    -(576 as ::core::ffi::c_int) as int16_t,
    492 as ::core::ffi::c_int as int16_t,
    -(576 as ::core::ffi::c_int) as int16_t,
    16 as ::core::ffi::c_int as int16_t,
    720 as ::core::ffi::c_int as int16_t,
    -(108 as ::core::ffi::c_int) as int16_t,
    384 as ::core::ffi::c_int as int16_t,
    124 as ::core::ffi::c_int as int16_t,
    344 as ::core::ffi::c_int as int16_t,
    280 as ::core::ffi::c_int as int16_t,
    576 as ::core::ffi::c_int as int16_t,
    -(500 as ::core::ffi::c_int) as int16_t,
    252 as ::core::ffi::c_int as int16_t,
    104 as ::core::ffi::c_int as int16_t,
    -(308 as ::core::ffi::c_int) as int16_t,
    196 as ::core::ffi::c_int as int16_t,
    -(188 as ::core::ffi::c_int) as int16_t,
    -(8 as ::core::ffi::c_int) as int16_t,
    1268 as ::core::ffi::c_int as int16_t,
    296 as ::core::ffi::c_int as int16_t,
    1032 as ::core::ffi::c_int as int16_t,
    -(1196 as ::core::ffi::c_int) as int16_t,
    436 as ::core::ffi::c_int as int16_t,
    316 as ::core::ffi::c_int as int16_t,
    372 as ::core::ffi::c_int as int16_t,
    -(432 as ::core::ffi::c_int) as int16_t,
    -(200 as ::core::ffi::c_int) as int16_t,
    -(660 as ::core::ffi::c_int) as int16_t,
    704 as ::core::ffi::c_int as int16_t,
    -(224 as ::core::ffi::c_int) as int16_t,
    596 as ::core::ffi::c_int as int16_t,
    -(132 as ::core::ffi::c_int) as int16_t,
    268 as ::core::ffi::c_int as int16_t,
    32 as ::core::ffi::c_int as int16_t,
    -(452 as ::core::ffi::c_int) as int16_t,
    884 as ::core::ffi::c_int as int16_t,
    104 as ::core::ffi::c_int as int16_t,
    -(1008 as ::core::ffi::c_int) as int16_t,
    424 as ::core::ffi::c_int as int16_t,
    -(1348 as ::core::ffi::c_int) as int16_t,
    -(280 as ::core::ffi::c_int) as int16_t,
    4 as ::core::ffi::c_int as int16_t,
    -(1168 as ::core::ffi::c_int) as int16_t,
    368 as ::core::ffi::c_int as int16_t,
    476 as ::core::ffi::c_int as int16_t,
    696 as ::core::ffi::c_int as int16_t,
    300 as ::core::ffi::c_int as int16_t,
    -(8 as ::core::ffi::c_int) as int16_t,
    24 as ::core::ffi::c_int as int16_t,
    180 as ::core::ffi::c_int as int16_t,
    -(592 as ::core::ffi::c_int) as int16_t,
    -(196 as ::core::ffi::c_int) as int16_t,
    388 as ::core::ffi::c_int as int16_t,
    304 as ::core::ffi::c_int as int16_t,
    500 as ::core::ffi::c_int as int16_t,
    724 as ::core::ffi::c_int as int16_t,
    -(160 as ::core::ffi::c_int) as int16_t,
    244 as ::core::ffi::c_int as int16_t,
    -(84 as ::core::ffi::c_int) as int16_t,
    272 as ::core::ffi::c_int as int16_t,
    -(256 as ::core::ffi::c_int) as int16_t,
    -(420 as ::core::ffi::c_int) as int16_t,
    320 as ::core::ffi::c_int as int16_t,
    208 as ::core::ffi::c_int as int16_t,
    -(144 as ::core::ffi::c_int) as int16_t,
    -(156 as ::core::ffi::c_int) as int16_t,
    156 as ::core::ffi::c_int as int16_t,
    364 as ::core::ffi::c_int as int16_t,
    452 as ::core::ffi::c_int as int16_t,
    28 as ::core::ffi::c_int as int16_t,
    540 as ::core::ffi::c_int as int16_t,
    316 as ::core::ffi::c_int as int16_t,
    220 as ::core::ffi::c_int as int16_t,
    -(644 as ::core::ffi::c_int) as int16_t,
    -(248 as ::core::ffi::c_int) as int16_t,
    464 as ::core::ffi::c_int as int16_t,
    72 as ::core::ffi::c_int as int16_t,
    360 as ::core::ffi::c_int as int16_t,
    32 as ::core::ffi::c_int as int16_t,
    -(388 as ::core::ffi::c_int) as int16_t,
    496 as ::core::ffi::c_int as int16_t,
    -(680 as ::core::ffi::c_int) as int16_t,
    -(48 as ::core::ffi::c_int) as int16_t,
    208 as ::core::ffi::c_int as int16_t,
    -(116 as ::core::ffi::c_int) as int16_t,
    -(408 as ::core::ffi::c_int) as int16_t,
    60 as ::core::ffi::c_int as int16_t,
    -(604 as ::core::ffi::c_int) as int16_t,
    -(392 as ::core::ffi::c_int) as int16_t,
    548 as ::core::ffi::c_int as int16_t,
    -(840 as ::core::ffi::c_int) as int16_t,
    784 as ::core::ffi::c_int as int16_t,
    -(460 as ::core::ffi::c_int) as int16_t,
    656 as ::core::ffi::c_int as int16_t,
    -(544 as ::core::ffi::c_int) as int16_t,
    -(388 as ::core::ffi::c_int) as int16_t,
    -(264 as ::core::ffi::c_int) as int16_t,
    908 as ::core::ffi::c_int as int16_t,
    -(800 as ::core::ffi::c_int) as int16_t,
    -(628 as ::core::ffi::c_int) as int16_t,
    -(612 as ::core::ffi::c_int) as int16_t,
    -(568 as ::core::ffi::c_int) as int16_t,
    572 as ::core::ffi::c_int as int16_t,
    -(220 as ::core::ffi::c_int) as int16_t,
    164 as ::core::ffi::c_int as int16_t,
    288 as ::core::ffi::c_int as int16_t,
    -(16 as ::core::ffi::c_int) as int16_t,
    -(308 as ::core::ffi::c_int) as int16_t,
    308 as ::core::ffi::c_int as int16_t,
    -(112 as ::core::ffi::c_int) as int16_t,
    -(636 as ::core::ffi::c_int) as int16_t,
    -(760 as ::core::ffi::c_int) as int16_t,
    280 as ::core::ffi::c_int as int16_t,
    -(668 as ::core::ffi::c_int) as int16_t,
    432 as ::core::ffi::c_int as int16_t,
    364 as ::core::ffi::c_int as int16_t,
    240 as ::core::ffi::c_int as int16_t,
    -(196 as ::core::ffi::c_int) as int16_t,
    604 as ::core::ffi::c_int as int16_t,
    340 as ::core::ffi::c_int as int16_t,
    384 as ::core::ffi::c_int as int16_t,
    196 as ::core::ffi::c_int as int16_t,
    592 as ::core::ffi::c_int as int16_t,
    -(44 as ::core::ffi::c_int) as int16_t,
    -(500 as ::core::ffi::c_int) as int16_t,
    432 as ::core::ffi::c_int as int16_t,
    -(580 as ::core::ffi::c_int) as int16_t,
    -(132 as ::core::ffi::c_int) as int16_t,
    636 as ::core::ffi::c_int as int16_t,
    -(76 as ::core::ffi::c_int) as int16_t,
    392 as ::core::ffi::c_int as int16_t,
    4 as ::core::ffi::c_int as int16_t,
    -(412 as ::core::ffi::c_int) as int16_t,
    540 as ::core::ffi::c_int as int16_t,
    508 as ::core::ffi::c_int as int16_t,
    328 as ::core::ffi::c_int as int16_t,
    -(356 as ::core::ffi::c_int) as int16_t,
    -(36 as ::core::ffi::c_int) as int16_t,
    16 as ::core::ffi::c_int as int16_t,
    -(220 as ::core::ffi::c_int) as int16_t,
    -(64 as ::core::ffi::c_int) as int16_t,
    -(248 as ::core::ffi::c_int) as int16_t,
    -(60 as ::core::ffi::c_int) as int16_t,
    24 as ::core::ffi::c_int as int16_t,
    -(192 as ::core::ffi::c_int) as int16_t,
    368 as ::core::ffi::c_int as int16_t,
    1040 as ::core::ffi::c_int as int16_t,
    92 as ::core::ffi::c_int as int16_t,
    -(24 as ::core::ffi::c_int) as int16_t,
    -(1044 as ::core::ffi::c_int) as int16_t,
    -(32 as ::core::ffi::c_int) as int16_t,
    40 as ::core::ffi::c_int as int16_t,
    104 as ::core::ffi::c_int as int16_t,
    148 as ::core::ffi::c_int as int16_t,
    192 as ::core::ffi::c_int as int16_t,
    -(136 as ::core::ffi::c_int) as int16_t,
    -(520 as ::core::ffi::c_int) as int16_t,
    56 as ::core::ffi::c_int as int16_t,
    -(816 as ::core::ffi::c_int) as int16_t,
    -(224 as ::core::ffi::c_int) as int16_t,
    732 as ::core::ffi::c_int as int16_t,
    392 as ::core::ffi::c_int as int16_t,
    356 as ::core::ffi::c_int as int16_t,
    212 as ::core::ffi::c_int as int16_t,
    -(80 as ::core::ffi::c_int) as int16_t,
    -(424 as ::core::ffi::c_int) as int16_t,
    -(1008 as ::core::ffi::c_int) as int16_t,
    -(324 as ::core::ffi::c_int) as int16_t,
    588 as ::core::ffi::c_int as int16_t,
    -(1496 as ::core::ffi::c_int) as int16_t,
    576 as ::core::ffi::c_int as int16_t,
    460 as ::core::ffi::c_int as int16_t,
    -(816 as ::core::ffi::c_int) as int16_t,
    -(848 as ::core::ffi::c_int) as int16_t,
    56 as ::core::ffi::c_int as int16_t,
    -(580 as ::core::ffi::c_int) as int16_t,
    -(92 as ::core::ffi::c_int) as int16_t,
    -(1372 as ::core::ffi::c_int) as int16_t,
    -(112 as ::core::ffi::c_int) as int16_t,
    -(496 as ::core::ffi::c_int) as int16_t,
    200 as ::core::ffi::c_int as int16_t,
    364 as ::core::ffi::c_int as int16_t,
    52 as ::core::ffi::c_int as int16_t,
    -(140 as ::core::ffi::c_int) as int16_t,
    48 as ::core::ffi::c_int as int16_t,
    -(48 as ::core::ffi::c_int) as int16_t,
    -(60 as ::core::ffi::c_int) as int16_t,
    84 as ::core::ffi::c_int as int16_t,
    72 as ::core::ffi::c_int as int16_t,
    40 as ::core::ffi::c_int as int16_t,
    132 as ::core::ffi::c_int as int16_t,
    -(356 as ::core::ffi::c_int) as int16_t,
    -(268 as ::core::ffi::c_int) as int16_t,
    -(104 as ::core::ffi::c_int) as int16_t,
    -(284 as ::core::ffi::c_int) as int16_t,
    -(404 as ::core::ffi::c_int) as int16_t,
    732 as ::core::ffi::c_int as int16_t,
    -(520 as ::core::ffi::c_int) as int16_t,
    164 as ::core::ffi::c_int as int16_t,
    -(304 as ::core::ffi::c_int) as int16_t,
    -(540 as ::core::ffi::c_int) as int16_t,
    120 as ::core::ffi::c_int as int16_t,
    328 as ::core::ffi::c_int as int16_t,
    -(76 as ::core::ffi::c_int) as int16_t,
    -(460 as ::core::ffi::c_int) as int16_t,
    756 as ::core::ffi::c_int as int16_t,
    388 as ::core::ffi::c_int as int16_t,
    588 as ::core::ffi::c_int as int16_t,
    236 as ::core::ffi::c_int as int16_t,
    -(436 as ::core::ffi::c_int) as int16_t,
    -(72 as ::core::ffi::c_int) as int16_t,
    -(176 as ::core::ffi::c_int) as int16_t,
    -(404 as ::core::ffi::c_int) as int16_t,
    -(316 as ::core::ffi::c_int) as int16_t,
    -(148 as ::core::ffi::c_int) as int16_t,
    716 as ::core::ffi::c_int as int16_t,
    -(604 as ::core::ffi::c_int) as int16_t,
    404 as ::core::ffi::c_int as int16_t,
    -(72 as ::core::ffi::c_int) as int16_t,
    -(88 as ::core::ffi::c_int) as int16_t,
    -(888 as ::core::ffi::c_int) as int16_t,
    -(68 as ::core::ffi::c_int) as int16_t,
    944 as ::core::ffi::c_int as int16_t,
    88 as ::core::ffi::c_int as int16_t,
    -(220 as ::core::ffi::c_int) as int16_t,
    -(344 as ::core::ffi::c_int) as int16_t,
    960 as ::core::ffi::c_int as int16_t,
    472 as ::core::ffi::c_int as int16_t,
    460 as ::core::ffi::c_int as int16_t,
    -(232 as ::core::ffi::c_int) as int16_t,
    704 as ::core::ffi::c_int as int16_t,
    120 as ::core::ffi::c_int as int16_t,
    832 as ::core::ffi::c_int as int16_t,
    -(228 as ::core::ffi::c_int) as int16_t,
    692 as ::core::ffi::c_int as int16_t,
    -(508 as ::core::ffi::c_int) as int16_t,
    132 as ::core::ffi::c_int as int16_t,
    -(476 as ::core::ffi::c_int) as int16_t,
    844 as ::core::ffi::c_int as int16_t,
    -(748 as ::core::ffi::c_int) as int16_t,
    -(364 as ::core::ffi::c_int) as int16_t,
    -(44 as ::core::ffi::c_int) as int16_t,
    1116 as ::core::ffi::c_int as int16_t,
    -(1104 as ::core::ffi::c_int) as int16_t,
    -(1056 as ::core::ffi::c_int) as int16_t,
    76 as ::core::ffi::c_int as int16_t,
    428 as ::core::ffi::c_int as int16_t,
    552 as ::core::ffi::c_int as int16_t,
    -(692 as ::core::ffi::c_int) as int16_t,
    60 as ::core::ffi::c_int as int16_t,
    356 as ::core::ffi::c_int as int16_t,
    96 as ::core::ffi::c_int as int16_t,
    -(384 as ::core::ffi::c_int) as int16_t,
    -(188 as ::core::ffi::c_int) as int16_t,
    -(612 as ::core::ffi::c_int) as int16_t,
    -(576 as ::core::ffi::c_int) as int16_t,
    736 as ::core::ffi::c_int as int16_t,
    508 as ::core::ffi::c_int as int16_t,
    892 as ::core::ffi::c_int as int16_t,
    352 as ::core::ffi::c_int as int16_t,
    -(1132 as ::core::ffi::c_int) as int16_t,
    504 as ::core::ffi::c_int as int16_t,
    -(24 as ::core::ffi::c_int) as int16_t,
    -(352 as ::core::ffi::c_int) as int16_t,
    324 as ::core::ffi::c_int as int16_t,
    332 as ::core::ffi::c_int as int16_t,
    -(600 as ::core::ffi::c_int) as int16_t,
    -(312 as ::core::ffi::c_int) as int16_t,
    292 as ::core::ffi::c_int as int16_t,
    508 as ::core::ffi::c_int as int16_t,
    -(144 as ::core::ffi::c_int) as int16_t,
    -(8 as ::core::ffi::c_int) as int16_t,
    484 as ::core::ffi::c_int as int16_t,
    48 as ::core::ffi::c_int as int16_t,
    284 as ::core::ffi::c_int as int16_t,
    -(260 as ::core::ffi::c_int) as int16_t,
    -(240 as ::core::ffi::c_int) as int16_t,
    256 as ::core::ffi::c_int as int16_t,
    -(100 as ::core::ffi::c_int) as int16_t,
    -(292 as ::core::ffi::c_int) as int16_t,
    -(204 as ::core::ffi::c_int) as int16_t,
    -(44 as ::core::ffi::c_int) as int16_t,
    472 as ::core::ffi::c_int as int16_t,
    -(204 as ::core::ffi::c_int) as int16_t,
    908 as ::core::ffi::c_int as int16_t,
    -(188 as ::core::ffi::c_int) as int16_t,
    -(1000 as ::core::ffi::c_int) as int16_t,
    -(256 as ::core::ffi::c_int) as int16_t,
    92 as ::core::ffi::c_int as int16_t,
    1164 as ::core::ffi::c_int as int16_t,
    -(392 as ::core::ffi::c_int) as int16_t,
    564 as ::core::ffi::c_int as int16_t,
    356 as ::core::ffi::c_int as int16_t,
    652 as ::core::ffi::c_int as int16_t,
    -(28 as ::core::ffi::c_int) as int16_t,
    -(884 as ::core::ffi::c_int) as int16_t,
    256 as ::core::ffi::c_int as int16_t,
    484 as ::core::ffi::c_int as int16_t,
    -(192 as ::core::ffi::c_int) as int16_t,
    760 as ::core::ffi::c_int as int16_t,
    -(176 as ::core::ffi::c_int) as int16_t,
    376 as ::core::ffi::c_int as int16_t,
    -(524 as ::core::ffi::c_int) as int16_t,
    -(452 as ::core::ffi::c_int) as int16_t,
    -(436 as ::core::ffi::c_int) as int16_t,
    860 as ::core::ffi::c_int as int16_t,
    -(736 as ::core::ffi::c_int) as int16_t,
    212 as ::core::ffi::c_int as int16_t,
    124 as ::core::ffi::c_int as int16_t,
    504 as ::core::ffi::c_int as int16_t,
    -(476 as ::core::ffi::c_int) as int16_t,
    468 as ::core::ffi::c_int as int16_t,
    76 as ::core::ffi::c_int as int16_t,
    -(472 as ::core::ffi::c_int) as int16_t,
    552 as ::core::ffi::c_int as int16_t,
    -(692 as ::core::ffi::c_int) as int16_t,
    -(944 as ::core::ffi::c_int) as int16_t,
    -(620 as ::core::ffi::c_int) as int16_t,
    740 as ::core::ffi::c_int as int16_t,
    -(240 as ::core::ffi::c_int) as int16_t,
    400 as ::core::ffi::c_int as int16_t,
    132 as ::core::ffi::c_int as int16_t,
    20 as ::core::ffi::c_int as int16_t,
    192 as ::core::ffi::c_int as int16_t,
    -(196 as ::core::ffi::c_int) as int16_t,
    264 as ::core::ffi::c_int as int16_t,
    -(668 as ::core::ffi::c_int) as int16_t,
    -(1012 as ::core::ffi::c_int) as int16_t,
    -(60 as ::core::ffi::c_int) as int16_t,
    296 as ::core::ffi::c_int as int16_t,
    -(316 as ::core::ffi::c_int) as int16_t,
    -(828 as ::core::ffi::c_int) as int16_t,
    76 as ::core::ffi::c_int as int16_t,
    -(156 as ::core::ffi::c_int) as int16_t,
    284 as ::core::ffi::c_int as int16_t,
    -(768 as ::core::ffi::c_int) as int16_t,
    -(448 as ::core::ffi::c_int) as int16_t,
    -(832 as ::core::ffi::c_int) as int16_t,
    148 as ::core::ffi::c_int as int16_t,
    248 as ::core::ffi::c_int as int16_t,
    652 as ::core::ffi::c_int as int16_t,
    616 as ::core::ffi::c_int as int16_t,
    1236 as ::core::ffi::c_int as int16_t,
    288 as ::core::ffi::c_int as int16_t,
    -(328 as ::core::ffi::c_int) as int16_t,
    -(400 as ::core::ffi::c_int) as int16_t,
    -(124 as ::core::ffi::c_int) as int16_t,
    588 as ::core::ffi::c_int as int16_t,
    220 as ::core::ffi::c_int as int16_t,
    520 as ::core::ffi::c_int as int16_t,
    -(696 as ::core::ffi::c_int) as int16_t,
    1032 as ::core::ffi::c_int as int16_t,
    768 as ::core::ffi::c_int as int16_t,
    -(740 as ::core::ffi::c_int) as int16_t,
    -(92 as ::core::ffi::c_int) as int16_t,
    -(272 as ::core::ffi::c_int) as int16_t,
    296 as ::core::ffi::c_int as int16_t,
    448 as ::core::ffi::c_int as int16_t,
    -(464 as ::core::ffi::c_int) as int16_t,
    412 as ::core::ffi::c_int as int16_t,
    -(200 as ::core::ffi::c_int) as int16_t,
    392 as ::core::ffi::c_int as int16_t,
    440 as ::core::ffi::c_int as int16_t,
    -(200 as ::core::ffi::c_int) as int16_t,
    264 as ::core::ffi::c_int as int16_t,
    -(152 as ::core::ffi::c_int) as int16_t,
    -(260 as ::core::ffi::c_int) as int16_t,
    320 as ::core::ffi::c_int as int16_t,
    1032 as ::core::ffi::c_int as int16_t,
    216 as ::core::ffi::c_int as int16_t,
    320 as ::core::ffi::c_int as int16_t,
    -(8 as ::core::ffi::c_int) as int16_t,
    -(64 as ::core::ffi::c_int) as int16_t,
    156 as ::core::ffi::c_int as int16_t,
    -(1016 as ::core::ffi::c_int) as int16_t,
    1084 as ::core::ffi::c_int as int16_t,
    1172 as ::core::ffi::c_int as int16_t,
    536 as ::core::ffi::c_int as int16_t,
    484 as ::core::ffi::c_int as int16_t,
    -(432 as ::core::ffi::c_int) as int16_t,
    132 as ::core::ffi::c_int as int16_t,
    372 as ::core::ffi::c_int as int16_t,
    -(52 as ::core::ffi::c_int) as int16_t,
    -(256 as ::core::ffi::c_int) as int16_t,
    84 as ::core::ffi::c_int as int16_t,
    116 as ::core::ffi::c_int as int16_t,
    -(352 as ::core::ffi::c_int) as int16_t,
    48 as ::core::ffi::c_int as int16_t,
    116 as ::core::ffi::c_int as int16_t,
    304 as ::core::ffi::c_int as int16_t,
    -(384 as ::core::ffi::c_int) as int16_t,
    412 as ::core::ffi::c_int as int16_t,
    924 as ::core::ffi::c_int as int16_t,
    -(300 as ::core::ffi::c_int) as int16_t,
    528 as ::core::ffi::c_int as int16_t,
    628 as ::core::ffi::c_int as int16_t,
    180 as ::core::ffi::c_int as int16_t,
    648 as ::core::ffi::c_int as int16_t,
    44 as ::core::ffi::c_int as int16_t,
    -(980 as ::core::ffi::c_int) as int16_t,
    -(220 as ::core::ffi::c_int) as int16_t,
    1320 as ::core::ffi::c_int as int16_t,
    48 as ::core::ffi::c_int as int16_t,
    332 as ::core::ffi::c_int as int16_t,
    748 as ::core::ffi::c_int as int16_t,
    524 as ::core::ffi::c_int as int16_t,
    -(268 as ::core::ffi::c_int) as int16_t,
    -(720 as ::core::ffi::c_int) as int16_t,
    540 as ::core::ffi::c_int as int16_t,
    -(276 as ::core::ffi::c_int) as int16_t,
    564 as ::core::ffi::c_int as int16_t,
    -(344 as ::core::ffi::c_int) as int16_t,
    -(208 as ::core::ffi::c_int) as int16_t,
    -(196 as ::core::ffi::c_int) as int16_t,
    436 as ::core::ffi::c_int as int16_t,
    896 as ::core::ffi::c_int as int16_t,
    88 as ::core::ffi::c_int as int16_t,
    -(392 as ::core::ffi::c_int) as int16_t,
    132 as ::core::ffi::c_int as int16_t,
    80 as ::core::ffi::c_int as int16_t,
    -(964 as ::core::ffi::c_int) as int16_t,
    -(288 as ::core::ffi::c_int) as int16_t,
    568 as ::core::ffi::c_int as int16_t,
    56 as ::core::ffi::c_int as int16_t,
    -(48 as ::core::ffi::c_int) as int16_t,
    -(456 as ::core::ffi::c_int) as int16_t,
    888 as ::core::ffi::c_int as int16_t,
    8 as ::core::ffi::c_int as int16_t,
    552 as ::core::ffi::c_int as int16_t,
    -(156 as ::core::ffi::c_int) as int16_t,
    -(292 as ::core::ffi::c_int) as int16_t,
    948 as ::core::ffi::c_int as int16_t,
    288 as ::core::ffi::c_int as int16_t,
    128 as ::core::ffi::c_int as int16_t,
    -(716 as ::core::ffi::c_int) as int16_t,
    -(292 as ::core::ffi::c_int) as int16_t,
    1192 as ::core::ffi::c_int as int16_t,
    -(152 as ::core::ffi::c_int) as int16_t,
    876 as ::core::ffi::c_int as int16_t,
    352 as ::core::ffi::c_int as int16_t,
    -(600 as ::core::ffi::c_int) as int16_t,
    -(260 as ::core::ffi::c_int) as int16_t,
    -(812 as ::core::ffi::c_int) as int16_t,
    -(468 as ::core::ffi::c_int) as int16_t,
    -(28 as ::core::ffi::c_int) as int16_t,
    -(120 as ::core::ffi::c_int) as int16_t,
    -(32 as ::core::ffi::c_int) as int16_t,
    -(44 as ::core::ffi::c_int) as int16_t,
    1284 as ::core::ffi::c_int as int16_t,
    496 as ::core::ffi::c_int as int16_t,
    192 as ::core::ffi::c_int as int16_t,
    464 as ::core::ffi::c_int as int16_t,
    312 as ::core::ffi::c_int as int16_t,
    -(76 as ::core::ffi::c_int) as int16_t,
    -(516 as ::core::ffi::c_int) as int16_t,
    -(380 as ::core::ffi::c_int) as int16_t,
    -(456 as ::core::ffi::c_int) as int16_t,
    -(1012 as ::core::ffi::c_int) as int16_t,
    -(48 as ::core::ffi::c_int) as int16_t,
    308 as ::core::ffi::c_int as int16_t,
    -(156 as ::core::ffi::c_int) as int16_t,
    36 as ::core::ffi::c_int as int16_t,
    492 as ::core::ffi::c_int as int16_t,
    -(156 as ::core::ffi::c_int) as int16_t,
    -(808 as ::core::ffi::c_int) as int16_t,
    188 as ::core::ffi::c_int as int16_t,
    1652 as ::core::ffi::c_int as int16_t,
    68 as ::core::ffi::c_int as int16_t,
    -(120 as ::core::ffi::c_int) as int16_t,
    -(116 as ::core::ffi::c_int) as int16_t,
    316 as ::core::ffi::c_int as int16_t,
    160 as ::core::ffi::c_int as int16_t,
    -(140 as ::core::ffi::c_int) as int16_t,
    352 as ::core::ffi::c_int as int16_t,
    808 as ::core::ffi::c_int as int16_t,
    -(416 as ::core::ffi::c_int) as int16_t,
    592 as ::core::ffi::c_int as int16_t,
    316 as ::core::ffi::c_int as int16_t,
    -(480 as ::core::ffi::c_int) as int16_t,
    56 as ::core::ffi::c_int as int16_t,
    528 as ::core::ffi::c_int as int16_t,
    -(204 as ::core::ffi::c_int) as int16_t,
    -(568 as ::core::ffi::c_int) as int16_t,
    372 as ::core::ffi::c_int as int16_t,
    -(232 as ::core::ffi::c_int) as int16_t,
    752 as ::core::ffi::c_int as int16_t,
    -(344 as ::core::ffi::c_int) as int16_t,
    744 as ::core::ffi::c_int as int16_t,
    -(4 as ::core::ffi::c_int) as int16_t,
    324 as ::core::ffi::c_int as int16_t,
    -(416 as ::core::ffi::c_int) as int16_t,
    -(600 as ::core::ffi::c_int) as int16_t,
    768 as ::core::ffi::c_int as int16_t,
    268 as ::core::ffi::c_int as int16_t,
    -(248 as ::core::ffi::c_int) as int16_t,
    -(88 as ::core::ffi::c_int) as int16_t,
    -(132 as ::core::ffi::c_int) as int16_t,
    -(420 as ::core::ffi::c_int) as int16_t,
    -(432 as ::core::ffi::c_int) as int16_t,
    80 as ::core::ffi::c_int as int16_t,
    -(288 as ::core::ffi::c_int) as int16_t,
    404 as ::core::ffi::c_int as int16_t,
    -(316 as ::core::ffi::c_int) as int16_t,
    -(1216 as ::core::ffi::c_int) as int16_t,
    -(588 as ::core::ffi::c_int) as int16_t,
    520 as ::core::ffi::c_int as int16_t,
    -(108 as ::core::ffi::c_int) as int16_t,
    92 as ::core::ffi::c_int as int16_t,
    -(320 as ::core::ffi::c_int) as int16_t,
    368 as ::core::ffi::c_int as int16_t,
    -(480 as ::core::ffi::c_int) as int16_t,
    -(216 as ::core::ffi::c_int) as int16_t,
    -(92 as ::core::ffi::c_int) as int16_t,
    1688 as ::core::ffi::c_int as int16_t,
    -(300 as ::core::ffi::c_int) as int16_t,
    180 as ::core::ffi::c_int as int16_t,
    1020 as ::core::ffi::c_int as int16_t,
    -(176 as ::core::ffi::c_int) as int16_t,
    820 as ::core::ffi::c_int as int16_t,
    -(68 as ::core::ffi::c_int) as int16_t,
    -(228 as ::core::ffi::c_int) as int16_t,
    -(260 as ::core::ffi::c_int) as int16_t,
    436 as ::core::ffi::c_int as int16_t,
    -(904 as ::core::ffi::c_int) as int16_t,
    20 as ::core::ffi::c_int as int16_t,
    40 as ::core::ffi::c_int as int16_t,
    -(508 as ::core::ffi::c_int) as int16_t,
    440 as ::core::ffi::c_int as int16_t,
    -(736 as ::core::ffi::c_int) as int16_t,
    312 as ::core::ffi::c_int as int16_t,
    332 as ::core::ffi::c_int as int16_t,
    204 as ::core::ffi::c_int as int16_t,
    760 as ::core::ffi::c_int as int16_t,
    -(372 as ::core::ffi::c_int) as int16_t,
    728 as ::core::ffi::c_int as int16_t,
    96 as ::core::ffi::c_int as int16_t,
    -(20 as ::core::ffi::c_int) as int16_t,
    -(632 as ::core::ffi::c_int) as int16_t,
    -(520 as ::core::ffi::c_int) as int16_t,
    -(560 as ::core::ffi::c_int) as int16_t,
    336 as ::core::ffi::c_int as int16_t,
    1076 as ::core::ffi::c_int as int16_t,
    -(64 as ::core::ffi::c_int) as int16_t,
    -(532 as ::core::ffi::c_int) as int16_t,
    776 as ::core::ffi::c_int as int16_t,
    584 as ::core::ffi::c_int as int16_t,
    192 as ::core::ffi::c_int as int16_t,
    396 as ::core::ffi::c_int as int16_t,
    -(728 as ::core::ffi::c_int) as int16_t,
    -(520 as ::core::ffi::c_int) as int16_t,
    276 as ::core::ffi::c_int as int16_t,
    -(188 as ::core::ffi::c_int) as int16_t,
    80 as ::core::ffi::c_int as int16_t,
    -(52 as ::core::ffi::c_int) as int16_t,
    -(612 as ::core::ffi::c_int) as int16_t,
    -(252 as ::core::ffi::c_int) as int16_t,
    -(48 as ::core::ffi::c_int) as int16_t,
    648 as ::core::ffi::c_int as int16_t,
    212 as ::core::ffi::c_int as int16_t,
    -(688 as ::core::ffi::c_int) as int16_t,
    228 as ::core::ffi::c_int as int16_t,
    -(52 as ::core::ffi::c_int) as int16_t,
    -(260 as ::core::ffi::c_int) as int16_t,
    428 as ::core::ffi::c_int as int16_t,
    -(412 as ::core::ffi::c_int) as int16_t,
    -(272 as ::core::ffi::c_int) as int16_t,
    -(404 as ::core::ffi::c_int) as int16_t,
    180 as ::core::ffi::c_int as int16_t,
    816 as ::core::ffi::c_int as int16_t,
    -(796 as ::core::ffi::c_int) as int16_t,
    48 as ::core::ffi::c_int as int16_t,
    152 as ::core::ffi::c_int as int16_t,
    484 as ::core::ffi::c_int as int16_t,
    -(88 as ::core::ffi::c_int) as int16_t,
    -(216 as ::core::ffi::c_int) as int16_t,
    988 as ::core::ffi::c_int as int16_t,
    696 as ::core::ffi::c_int as int16_t,
    188 as ::core::ffi::c_int as int16_t,
    -(528 as ::core::ffi::c_int) as int16_t,
    648 as ::core::ffi::c_int as int16_t,
    -(116 as ::core::ffi::c_int) as int16_t,
    -(180 as ::core::ffi::c_int) as int16_t,
    316 as ::core::ffi::c_int as int16_t,
    476 as ::core::ffi::c_int as int16_t,
    12 as ::core::ffi::c_int as int16_t,
    -(564 as ::core::ffi::c_int) as int16_t,
    96 as ::core::ffi::c_int as int16_t,
    476 as ::core::ffi::c_int as int16_t,
    -(252 as ::core::ffi::c_int) as int16_t,
    -(364 as ::core::ffi::c_int) as int16_t,
    -(376 as ::core::ffi::c_int) as int16_t,
    -(392 as ::core::ffi::c_int) as int16_t,
    556 as ::core::ffi::c_int as int16_t,
    -(256 as ::core::ffi::c_int) as int16_t,
    -(576 as ::core::ffi::c_int) as int16_t,
    260 as ::core::ffi::c_int as int16_t,
    -(352 as ::core::ffi::c_int) as int16_t,
    120 as ::core::ffi::c_int as int16_t,
    -(16 as ::core::ffi::c_int) as int16_t,
    -(136 as ::core::ffi::c_int) as int16_t,
    -(260 as ::core::ffi::c_int) as int16_t,
    -(492 as ::core::ffi::c_int) as int16_t,
    72 as ::core::ffi::c_int as int16_t,
    556 as ::core::ffi::c_int as int16_t,
    660 as ::core::ffi::c_int as int16_t,
    580 as ::core::ffi::c_int as int16_t,
    616 as ::core::ffi::c_int as int16_t,
    772 as ::core::ffi::c_int as int16_t,
    436 as ::core::ffi::c_int as int16_t,
    424 as ::core::ffi::c_int as int16_t,
    -(32 as ::core::ffi::c_int) as int16_t,
    -(324 as ::core::ffi::c_int) as int16_t,
    -(1268 as ::core::ffi::c_int) as int16_t,
    416 as ::core::ffi::c_int as int16_t,
    -(324 as ::core::ffi::c_int) as int16_t,
    -(80 as ::core::ffi::c_int) as int16_t,
    920 as ::core::ffi::c_int as int16_t,
    160 as ::core::ffi::c_int as int16_t,
    228 as ::core::ffi::c_int as int16_t,
    724 as ::core::ffi::c_int as int16_t,
    32 as ::core::ffi::c_int as int16_t,
    -(516 as ::core::ffi::c_int) as int16_t,
    64 as ::core::ffi::c_int as int16_t,
    384 as ::core::ffi::c_int as int16_t,
    68 as ::core::ffi::c_int as int16_t,
    -(128 as ::core::ffi::c_int) as int16_t,
    136 as ::core::ffi::c_int as int16_t,
    240 as ::core::ffi::c_int as int16_t,
    248 as ::core::ffi::c_int as int16_t,
    -(204 as ::core::ffi::c_int) as int16_t,
    -(68 as ::core::ffi::c_int) as int16_t,
    252 as ::core::ffi::c_int as int16_t,
    -(932 as ::core::ffi::c_int) as int16_t,
    -(120 as ::core::ffi::c_int) as int16_t,
    -(480 as ::core::ffi::c_int) as int16_t,
    -(628 as ::core::ffi::c_int) as int16_t,
    -(84 as ::core::ffi::c_int) as int16_t,
    192 as ::core::ffi::c_int as int16_t,
    852 as ::core::ffi::c_int as int16_t,
    -(404 as ::core::ffi::c_int) as int16_t,
    -(288 as ::core::ffi::c_int) as int16_t,
    -(132 as ::core::ffi::c_int) as int16_t,
    204 as ::core::ffi::c_int as int16_t,
    100 as ::core::ffi::c_int as int16_t,
    168 as ::core::ffi::c_int as int16_t,
    -(68 as ::core::ffi::c_int) as int16_t,
    -(196 as ::core::ffi::c_int) as int16_t,
    -(868 as ::core::ffi::c_int) as int16_t,
    460 as ::core::ffi::c_int as int16_t,
    1080 as ::core::ffi::c_int as int16_t,
    380 as ::core::ffi::c_int as int16_t,
    -(80 as ::core::ffi::c_int) as int16_t,
    244 as ::core::ffi::c_int as int16_t,
    0 as ::core::ffi::c_int as int16_t,
    484 as ::core::ffi::c_int as int16_t,
    -(888 as ::core::ffi::c_int) as int16_t,
    64 as ::core::ffi::c_int as int16_t,
    184 as ::core::ffi::c_int as int16_t,
    352 as ::core::ffi::c_int as int16_t,
    600 as ::core::ffi::c_int as int16_t,
    460 as ::core::ffi::c_int as int16_t,
    164 as ::core::ffi::c_int as int16_t,
    604 as ::core::ffi::c_int as int16_t,
    -(196 as ::core::ffi::c_int) as int16_t,
    320 as ::core::ffi::c_int as int16_t,
    -(64 as ::core::ffi::c_int) as int16_t,
    588 as ::core::ffi::c_int as int16_t,
    -(184 as ::core::ffi::c_int) as int16_t,
    228 as ::core::ffi::c_int as int16_t,
    12 as ::core::ffi::c_int as int16_t,
    372 as ::core::ffi::c_int as int16_t,
    48 as ::core::ffi::c_int as int16_t,
    -(848 as ::core::ffi::c_int) as int16_t,
    -(344 as ::core::ffi::c_int) as int16_t,
    224 as ::core::ffi::c_int as int16_t,
    208 as ::core::ffi::c_int as int16_t,
    -(200 as ::core::ffi::c_int) as int16_t,
    484 as ::core::ffi::c_int as int16_t,
    128 as ::core::ffi::c_int as int16_t,
    -(20 as ::core::ffi::c_int) as int16_t,
    272 as ::core::ffi::c_int as int16_t,
    -(468 as ::core::ffi::c_int) as int16_t,
    -(840 as ::core::ffi::c_int) as int16_t,
    384 as ::core::ffi::c_int as int16_t,
    256 as ::core::ffi::c_int as int16_t,
    -(720 as ::core::ffi::c_int) as int16_t,
    -(520 as ::core::ffi::c_int) as int16_t,
    -(464 as ::core::ffi::c_int) as int16_t,
    -(580 as ::core::ffi::c_int) as int16_t,
    112 as ::core::ffi::c_int as int16_t,
    -(120 as ::core::ffi::c_int) as int16_t,
    644 as ::core::ffi::c_int as int16_t,
    -(356 as ::core::ffi::c_int) as int16_t,
    -(208 as ::core::ffi::c_int) as int16_t,
    -(608 as ::core::ffi::c_int) as int16_t,
    -(528 as ::core::ffi::c_int) as int16_t,
    704 as ::core::ffi::c_int as int16_t,
    560 as ::core::ffi::c_int as int16_t,
    -(424 as ::core::ffi::c_int) as int16_t,
    392 as ::core::ffi::c_int as int16_t,
    828 as ::core::ffi::c_int as int16_t,
    40 as ::core::ffi::c_int as int16_t,
    84 as ::core::ffi::c_int as int16_t,
    200 as ::core::ffi::c_int as int16_t,
    -(152 as ::core::ffi::c_int) as int16_t,
    0 as ::core::ffi::c_int as int16_t,
    -(144 as ::core::ffi::c_int) as int16_t,
    584 as ::core::ffi::c_int as int16_t,
    280 as ::core::ffi::c_int as int16_t,
    -(120 as ::core::ffi::c_int) as int16_t,
    80 as ::core::ffi::c_int as int16_t,
    -(556 as ::core::ffi::c_int) as int16_t,
    -(972 as ::core::ffi::c_int) as int16_t,
    -(196 as ::core::ffi::c_int) as int16_t,
    -(472 as ::core::ffi::c_int) as int16_t,
    724 as ::core::ffi::c_int as int16_t,
    80 as ::core::ffi::c_int as int16_t,
    168 as ::core::ffi::c_int as int16_t,
    -(32 as ::core::ffi::c_int) as int16_t,
    88 as ::core::ffi::c_int as int16_t,
    160 as ::core::ffi::c_int as int16_t,
    -(688 as ::core::ffi::c_int) as int16_t,
    0 as ::core::ffi::c_int as int16_t,
    160 as ::core::ffi::c_int as int16_t,
    356 as ::core::ffi::c_int as int16_t,
    372 as ::core::ffi::c_int as int16_t,
    -(776 as ::core::ffi::c_int) as int16_t,
    740 as ::core::ffi::c_int as int16_t,
    -(128 as ::core::ffi::c_int) as int16_t,
    676 as ::core::ffi::c_int as int16_t,
    -(248 as ::core::ffi::c_int) as int16_t,
    -(480 as ::core::ffi::c_int) as int16_t,
    4 as ::core::ffi::c_int as int16_t,
    -(364 as ::core::ffi::c_int) as int16_t,
    96 as ::core::ffi::c_int as int16_t,
    544 as ::core::ffi::c_int as int16_t,
    232 as ::core::ffi::c_int as int16_t,
    -(1032 as ::core::ffi::c_int) as int16_t,
    956 as ::core::ffi::c_int as int16_t,
    236 as ::core::ffi::c_int as int16_t,
    356 as ::core::ffi::c_int as int16_t,
    20 as ::core::ffi::c_int as int16_t,
    -(40 as ::core::ffi::c_int) as int16_t,
    300 as ::core::ffi::c_int as int16_t,
    24 as ::core::ffi::c_int as int16_t,
    -(676 as ::core::ffi::c_int) as int16_t,
    -(596 as ::core::ffi::c_int) as int16_t,
    132 as ::core::ffi::c_int as int16_t,
    1120 as ::core::ffi::c_int as int16_t,
    -(104 as ::core::ffi::c_int) as int16_t,
    532 as ::core::ffi::c_int as int16_t,
    -(1096 as ::core::ffi::c_int) as int16_t,
    568 as ::core::ffi::c_int as int16_t,
    648 as ::core::ffi::c_int as int16_t,
    444 as ::core::ffi::c_int as int16_t,
    508 as ::core::ffi::c_int as int16_t,
    380 as ::core::ffi::c_int as int16_t,
    188 as ::core::ffi::c_int as int16_t,
    -(376 as ::core::ffi::c_int) as int16_t,
    -(604 as ::core::ffi::c_int) as int16_t,
    1488 as ::core::ffi::c_int as int16_t,
    424 as ::core::ffi::c_int as int16_t,
    24 as ::core::ffi::c_int as int16_t,
    756 as ::core::ffi::c_int as int16_t,
    -(220 as ::core::ffi::c_int) as int16_t,
    -(192 as ::core::ffi::c_int) as int16_t,
    716 as ::core::ffi::c_int as int16_t,
    120 as ::core::ffi::c_int as int16_t,
    920 as ::core::ffi::c_int as int16_t,
    688 as ::core::ffi::c_int as int16_t,
    168 as ::core::ffi::c_int as int16_t,
    44 as ::core::ffi::c_int as int16_t,
    -(460 as ::core::ffi::c_int) as int16_t,
    568 as ::core::ffi::c_int as int16_t,
    284 as ::core::ffi::c_int as int16_t,
    1144 as ::core::ffi::c_int as int16_t,
    1160 as ::core::ffi::c_int as int16_t,
    600 as ::core::ffi::c_int as int16_t,
    424 as ::core::ffi::c_int as int16_t,
    888 as ::core::ffi::c_int as int16_t,
    656 as ::core::ffi::c_int as int16_t,
    -(356 as ::core::ffi::c_int) as int16_t,
    -(320 as ::core::ffi::c_int) as int16_t,
    220 as ::core::ffi::c_int as int16_t,
    316 as ::core::ffi::c_int as int16_t,
    -(176 as ::core::ffi::c_int) as int16_t,
    -(724 as ::core::ffi::c_int) as int16_t,
    -(188 as ::core::ffi::c_int) as int16_t,
    -(816 as ::core::ffi::c_int) as int16_t,
    -(628 as ::core::ffi::c_int) as int16_t,
    -(348 as ::core::ffi::c_int) as int16_t,
    -(228 as ::core::ffi::c_int) as int16_t,
    -(380 as ::core::ffi::c_int) as int16_t,
    1012 as ::core::ffi::c_int as int16_t,
    -(452 as ::core::ffi::c_int) as int16_t,
    -(660 as ::core::ffi::c_int) as int16_t,
    736 as ::core::ffi::c_int as int16_t,
    928 as ::core::ffi::c_int as int16_t,
    404 as ::core::ffi::c_int as int16_t,
    -(696 as ::core::ffi::c_int) as int16_t,
    -(72 as ::core::ffi::c_int) as int16_t,
    -(268 as ::core::ffi::c_int) as int16_t,
    -(892 as ::core::ffi::c_int) as int16_t,
    128 as ::core::ffi::c_int as int16_t,
    184 as ::core::ffi::c_int as int16_t,
    -(344 as ::core::ffi::c_int) as int16_t,
    -(780 as ::core::ffi::c_int) as int16_t,
    360 as ::core::ffi::c_int as int16_t,
    336 as ::core::ffi::c_int as int16_t,
    400 as ::core::ffi::c_int as int16_t,
    344 as ::core::ffi::c_int as int16_t,
    428 as ::core::ffi::c_int as int16_t,
    548 as ::core::ffi::c_int as int16_t,
    -(112 as ::core::ffi::c_int) as int16_t,
    136 as ::core::ffi::c_int as int16_t,
    -(228 as ::core::ffi::c_int) as int16_t,
    -(216 as ::core::ffi::c_int) as int16_t,
    -(820 as ::core::ffi::c_int) as int16_t,
    -(516 as ::core::ffi::c_int) as int16_t,
    340 as ::core::ffi::c_int as int16_t,
    92 as ::core::ffi::c_int as int16_t,
    -(136 as ::core::ffi::c_int) as int16_t,
    116 as ::core::ffi::c_int as int16_t,
    -(300 as ::core::ffi::c_int) as int16_t,
    376 as ::core::ffi::c_int as int16_t,
    -(244 as ::core::ffi::c_int) as int16_t,
    100 as ::core::ffi::c_int as int16_t,
    -(316 as ::core::ffi::c_int) as int16_t,
    -(520 as ::core::ffi::c_int) as int16_t,
    -(284 as ::core::ffi::c_int) as int16_t,
    -(12 as ::core::ffi::c_int) as int16_t,
    824 as ::core::ffi::c_int as int16_t,
    164 as ::core::ffi::c_int as int16_t,
    -(548 as ::core::ffi::c_int) as int16_t,
    -(180 as ::core::ffi::c_int) as int16_t,
    -(128 as ::core::ffi::c_int) as int16_t,
    116 as ::core::ffi::c_int as int16_t,
    -(924 as ::core::ffi::c_int) as int16_t,
    -(828 as ::core::ffi::c_int) as int16_t,
    268 as ::core::ffi::c_int as int16_t,
    -(368 as ::core::ffi::c_int) as int16_t,
    -(580 as ::core::ffi::c_int) as int16_t,
    620 as ::core::ffi::c_int as int16_t,
    192 as ::core::ffi::c_int as int16_t,
    160 as ::core::ffi::c_int as int16_t,
    0 as ::core::ffi::c_int as int16_t,
    -(1676 as ::core::ffi::c_int) as int16_t,
    1068 as ::core::ffi::c_int as int16_t,
    424 as ::core::ffi::c_int as int16_t,
    -(56 as ::core::ffi::c_int) as int16_t,
    -(360 as ::core::ffi::c_int) as int16_t,
    468 as ::core::ffi::c_int as int16_t,
    -(156 as ::core::ffi::c_int) as int16_t,
    720 as ::core::ffi::c_int as int16_t,
    288 as ::core::ffi::c_int as int16_t,
    -(528 as ::core::ffi::c_int) as int16_t,
    556 as ::core::ffi::c_int as int16_t,
    -(364 as ::core::ffi::c_int) as int16_t,
    548 as ::core::ffi::c_int as int16_t,
    -(148 as ::core::ffi::c_int) as int16_t,
    504 as ::core::ffi::c_int as int16_t,
    316 as ::core::ffi::c_int as int16_t,
    152 as ::core::ffi::c_int as int16_t,
    -(648 as ::core::ffi::c_int) as int16_t,
    -(620 as ::core::ffi::c_int) as int16_t,
    -(684 as ::core::ffi::c_int) as int16_t,
    -(24 as ::core::ffi::c_int) as int16_t,
    -(376 as ::core::ffi::c_int) as int16_t,
    -(384 as ::core::ffi::c_int) as int16_t,
    -(108 as ::core::ffi::c_int) as int16_t,
    -(920 as ::core::ffi::c_int) as int16_t,
    -(1032 as ::core::ffi::c_int) as int16_t,
    768 as ::core::ffi::c_int as int16_t,
    180 as ::core::ffi::c_int as int16_t,
    -(264 as ::core::ffi::c_int) as int16_t,
    -(508 as ::core::ffi::c_int) as int16_t,
    -(1268 as ::core::ffi::c_int) as int16_t,
    -(260 as ::core::ffi::c_int) as int16_t,
    -(60 as ::core::ffi::c_int) as int16_t,
    300 as ::core::ffi::c_int as int16_t,
    -(240 as ::core::ffi::c_int) as int16_t,
    988 as ::core::ffi::c_int as int16_t,
    724 as ::core::ffi::c_int as int16_t,
    -(376 as ::core::ffi::c_int) as int16_t,
    -(576 as ::core::ffi::c_int) as int16_t,
    -(212 as ::core::ffi::c_int) as int16_t,
    -(736 as ::core::ffi::c_int) as int16_t,
    556 as ::core::ffi::c_int as int16_t,
    192 as ::core::ffi::c_int as int16_t,
    1092 as ::core::ffi::c_int as int16_t,
    -(620 as ::core::ffi::c_int) as int16_t,
    -(880 as ::core::ffi::c_int) as int16_t,
    376 as ::core::ffi::c_int as int16_t,
    -(56 as ::core::ffi::c_int) as int16_t,
    -(4 as ::core::ffi::c_int) as int16_t,
    -(216 as ::core::ffi::c_int) as int16_t,
    -(32 as ::core::ffi::c_int) as int16_t,
    836 as ::core::ffi::c_int as int16_t,
    268 as ::core::ffi::c_int as int16_t,
    396 as ::core::ffi::c_int as int16_t,
    1332 as ::core::ffi::c_int as int16_t,
    864 as ::core::ffi::c_int as int16_t,
    -(600 as ::core::ffi::c_int) as int16_t,
    100 as ::core::ffi::c_int as int16_t,
    56 as ::core::ffi::c_int as int16_t,
    -(412 as ::core::ffi::c_int) as int16_t,
    -(92 as ::core::ffi::c_int) as int16_t,
    356 as ::core::ffi::c_int as int16_t,
    180 as ::core::ffi::c_int as int16_t,
    884 as ::core::ffi::c_int as int16_t,
    -(468 as ::core::ffi::c_int) as int16_t,
    -(436 as ::core::ffi::c_int) as int16_t,
    292 as ::core::ffi::c_int as int16_t,
    -(388 as ::core::ffi::c_int) as int16_t,
    -(804 as ::core::ffi::c_int) as int16_t,
    -(704 as ::core::ffi::c_int) as int16_t,
    -(840 as ::core::ffi::c_int) as int16_t,
    368 as ::core::ffi::c_int as int16_t,
    -(348 as ::core::ffi::c_int) as int16_t,
    140 as ::core::ffi::c_int as int16_t,
    -(724 as ::core::ffi::c_int) as int16_t,
    1536 as ::core::ffi::c_int as int16_t,
    940 as ::core::ffi::c_int as int16_t,
    372 as ::core::ffi::c_int as int16_t,
    112 as ::core::ffi::c_int as int16_t,
    -(372 as ::core::ffi::c_int) as int16_t,
    436 as ::core::ffi::c_int as int16_t,
    -(480 as ::core::ffi::c_int) as int16_t,
    1136 as ::core::ffi::c_int as int16_t,
    296 as ::core::ffi::c_int as int16_t,
    -(32 as ::core::ffi::c_int) as int16_t,
    -(228 as ::core::ffi::c_int) as int16_t,
    132 as ::core::ffi::c_int as int16_t,
    -(48 as ::core::ffi::c_int) as int16_t,
    -(220 as ::core::ffi::c_int) as int16_t,
    868 as ::core::ffi::c_int as int16_t,
    -(1016 as ::core::ffi::c_int) as int16_t,
    -(60 as ::core::ffi::c_int) as int16_t,
    -(1044 as ::core::ffi::c_int) as int16_t,
    -(464 as ::core::ffi::c_int) as int16_t,
    328 as ::core::ffi::c_int as int16_t,
    916 as ::core::ffi::c_int as int16_t,
    244 as ::core::ffi::c_int as int16_t,
    12 as ::core::ffi::c_int as int16_t,
    -(736 as ::core::ffi::c_int) as int16_t,
    -(296 as ::core::ffi::c_int) as int16_t,
    360 as ::core::ffi::c_int as int16_t,
    468 as ::core::ffi::c_int as int16_t,
    -(376 as ::core::ffi::c_int) as int16_t,
    -(108 as ::core::ffi::c_int) as int16_t,
    -(92 as ::core::ffi::c_int) as int16_t,
    788 as ::core::ffi::c_int as int16_t,
    368 as ::core::ffi::c_int as int16_t,
    -(56 as ::core::ffi::c_int) as int16_t,
    544 as ::core::ffi::c_int as int16_t,
    400 as ::core::ffi::c_int as int16_t,
    -(672 as ::core::ffi::c_int) as int16_t,
    -(420 as ::core::ffi::c_int) as int16_t,
    728 as ::core::ffi::c_int as int16_t,
    16 as ::core::ffi::c_int as int16_t,
    320 as ::core::ffi::c_int as int16_t,
    44 as ::core::ffi::c_int as int16_t,
    -(284 as ::core::ffi::c_int) as int16_t,
    -(380 as ::core::ffi::c_int) as int16_t,
    -(796 as ::core::ffi::c_int) as int16_t,
    488 as ::core::ffi::c_int as int16_t,
    132 as ::core::ffi::c_int as int16_t,
    204 as ::core::ffi::c_int as int16_t,
    -(596 as ::core::ffi::c_int) as int16_t,
    -(372 as ::core::ffi::c_int) as int16_t,
    88 as ::core::ffi::c_int as int16_t,
    -(152 as ::core::ffi::c_int) as int16_t,
    -(908 as ::core::ffi::c_int) as int16_t,
    -(636 as ::core::ffi::c_int) as int16_t,
    -(572 as ::core::ffi::c_int) as int16_t,
    -(624 as ::core::ffi::c_int) as int16_t,
    -(116 as ::core::ffi::c_int) as int16_t,
    -(692 as ::core::ffi::c_int) as int16_t,
    -(200 as ::core::ffi::c_int) as int16_t,
    -(56 as ::core::ffi::c_int) as int16_t,
    276 as ::core::ffi::c_int as int16_t,
    -(88 as ::core::ffi::c_int) as int16_t,
    484 as ::core::ffi::c_int as int16_t,
    -(324 as ::core::ffi::c_int) as int16_t,
    948 as ::core::ffi::c_int as int16_t,
    864 as ::core::ffi::c_int as int16_t,
    1000 as ::core::ffi::c_int as int16_t,
    -(456 as ::core::ffi::c_int) as int16_t,
    -(184 as ::core::ffi::c_int) as int16_t,
    -(276 as ::core::ffi::c_int) as int16_t,
    292 as ::core::ffi::c_int as int16_t,
    -(296 as ::core::ffi::c_int) as int16_t,
    156 as ::core::ffi::c_int as int16_t,
    676 as ::core::ffi::c_int as int16_t,
    320 as ::core::ffi::c_int as int16_t,
    160 as ::core::ffi::c_int as int16_t,
    908 as ::core::ffi::c_int as int16_t,
    -(84 as ::core::ffi::c_int) as int16_t,
    -(1236 as ::core::ffi::c_int) as int16_t,
    -(288 as ::core::ffi::c_int) as int16_t,
    -(116 as ::core::ffi::c_int) as int16_t,
    260 as ::core::ffi::c_int as int16_t,
    -(372 as ::core::ffi::c_int) as int16_t,
    -(644 as ::core::ffi::c_int) as int16_t,
    732 as ::core::ffi::c_int as int16_t,
    -(756 as ::core::ffi::c_int) as int16_t,
    -(96 as ::core::ffi::c_int) as int16_t,
    84 as ::core::ffi::c_int as int16_t,
    344 as ::core::ffi::c_int as int16_t,
    -(520 as ::core::ffi::c_int) as int16_t,
    348 as ::core::ffi::c_int as int16_t,
    -(688 as ::core::ffi::c_int) as int16_t,
    240 as ::core::ffi::c_int as int16_t,
    -(84 as ::core::ffi::c_int) as int16_t,
    216 as ::core::ffi::c_int as int16_t,
    -(1044 as ::core::ffi::c_int) as int16_t,
    -(136 as ::core::ffi::c_int) as int16_t,
    -(676 as ::core::ffi::c_int) as int16_t,
    -(396 as ::core::ffi::c_int) as int16_t,
    -(1500 as ::core::ffi::c_int) as int16_t,
    960 as ::core::ffi::c_int as int16_t,
    -(40 as ::core::ffi::c_int) as int16_t,
    176 as ::core::ffi::c_int as int16_t,
    168 as ::core::ffi::c_int as int16_t,
    1516 as ::core::ffi::c_int as int16_t,
    420 as ::core::ffi::c_int as int16_t,
    -(504 as ::core::ffi::c_int) as int16_t,
    -(344 as ::core::ffi::c_int) as int16_t,
    -(364 as ::core::ffi::c_int) as int16_t,
    -(360 as ::core::ffi::c_int) as int16_t,
    1216 as ::core::ffi::c_int as int16_t,
    -(940 as ::core::ffi::c_int) as int16_t,
    -(380 as ::core::ffi::c_int) as int16_t,
    -(212 as ::core::ffi::c_int) as int16_t,
    252 as ::core::ffi::c_int as int16_t,
    -(660 as ::core::ffi::c_int) as int16_t,
    -(708 as ::core::ffi::c_int) as int16_t,
    484 as ::core::ffi::c_int as int16_t,
    -(444 as ::core::ffi::c_int) as int16_t,
    -(152 as ::core::ffi::c_int) as int16_t,
    928 as ::core::ffi::c_int as int16_t,
    -(120 as ::core::ffi::c_int) as int16_t,
    1112 as ::core::ffi::c_int as int16_t,
    476 as ::core::ffi::c_int as int16_t,
    -(260 as ::core::ffi::c_int) as int16_t,
    560 as ::core::ffi::c_int as int16_t,
    -(148 as ::core::ffi::c_int) as int16_t,
    -(344 as ::core::ffi::c_int) as int16_t,
    108 as ::core::ffi::c_int as int16_t,
    -(196 as ::core::ffi::c_int) as int16_t,
    228 as ::core::ffi::c_int as int16_t,
    -(288 as ::core::ffi::c_int) as int16_t,
    504 as ::core::ffi::c_int as int16_t,
    560 as ::core::ffi::c_int as int16_t,
    -(328 as ::core::ffi::c_int) as int16_t,
    -(88 as ::core::ffi::c_int) as int16_t,
    288 as ::core::ffi::c_int as int16_t,
    -(1008 as ::core::ffi::c_int) as int16_t,
    460 as ::core::ffi::c_int as int16_t,
    -(228 as ::core::ffi::c_int) as int16_t,
    468 as ::core::ffi::c_int as int16_t,
    -(836 as ::core::ffi::c_int) as int16_t,
    -(196 as ::core::ffi::c_int) as int16_t,
    76 as ::core::ffi::c_int as int16_t,
    388 as ::core::ffi::c_int as int16_t,
    232 as ::core::ffi::c_int as int16_t,
    412 as ::core::ffi::c_int as int16_t,
    -(1168 as ::core::ffi::c_int) as int16_t,
    -(716 as ::core::ffi::c_int) as int16_t,
    -(644 as ::core::ffi::c_int) as int16_t,
    756 as ::core::ffi::c_int as int16_t,
    -(172 as ::core::ffi::c_int) as int16_t,
    -(356 as ::core::ffi::c_int) as int16_t,
    -(504 as ::core::ffi::c_int) as int16_t,
    116 as ::core::ffi::c_int as int16_t,
    432 as ::core::ffi::c_int as int16_t,
    528 as ::core::ffi::c_int as int16_t,
    48 as ::core::ffi::c_int as int16_t,
    476 as ::core::ffi::c_int as int16_t,
    -(168 as ::core::ffi::c_int) as int16_t,
    -(608 as ::core::ffi::c_int) as int16_t,
    448 as ::core::ffi::c_int as int16_t,
    160 as ::core::ffi::c_int as int16_t,
    -(532 as ::core::ffi::c_int) as int16_t,
    -(272 as ::core::ffi::c_int) as int16_t,
    28 as ::core::ffi::c_int as int16_t,
    -(676 as ::core::ffi::c_int) as int16_t,
    -(12 as ::core::ffi::c_int) as int16_t,
    828 as ::core::ffi::c_int as int16_t,
    980 as ::core::ffi::c_int as int16_t,
    456 as ::core::ffi::c_int as int16_t,
    520 as ::core::ffi::c_int as int16_t,
    104 as ::core::ffi::c_int as int16_t,
    -(104 as ::core::ffi::c_int) as int16_t,
    256 as ::core::ffi::c_int as int16_t,
    -(344 as ::core::ffi::c_int) as int16_t,
    -(4 as ::core::ffi::c_int) as int16_t,
    -(28 as ::core::ffi::c_int) as int16_t,
    -(368 as ::core::ffi::c_int) as int16_t,
    -(52 as ::core::ffi::c_int) as int16_t,
    -(524 as ::core::ffi::c_int) as int16_t,
    -(572 as ::core::ffi::c_int) as int16_t,
    -(556 as ::core::ffi::c_int) as int16_t,
    -(200 as ::core::ffi::c_int) as int16_t,
    768 as ::core::ffi::c_int as int16_t,
    1124 as ::core::ffi::c_int as int16_t,
    -(208 as ::core::ffi::c_int) as int16_t,
    -(512 as ::core::ffi::c_int) as int16_t,
    176 as ::core::ffi::c_int as int16_t,
    232 as ::core::ffi::c_int as int16_t,
    248 as ::core::ffi::c_int as int16_t,
    -(148 as ::core::ffi::c_int) as int16_t,
    -(888 as ::core::ffi::c_int) as int16_t,
    604 as ::core::ffi::c_int as int16_t,
    -(600 as ::core::ffi::c_int) as int16_t,
    -(304 as ::core::ffi::c_int) as int16_t,
    804 as ::core::ffi::c_int as int16_t,
    -(156 as ::core::ffi::c_int) as int16_t,
    -(212 as ::core::ffi::c_int) as int16_t,
    488 as ::core::ffi::c_int as int16_t,
    -(192 as ::core::ffi::c_int) as int16_t,
    -(804 as ::core::ffi::c_int) as int16_t,
    -(256 as ::core::ffi::c_int) as int16_t,
    368 as ::core::ffi::c_int as int16_t,
    -(360 as ::core::ffi::c_int) as int16_t,
    -(916 as ::core::ffi::c_int) as int16_t,
    -(328 as ::core::ffi::c_int) as int16_t,
    228 as ::core::ffi::c_int as int16_t,
    -(240 as ::core::ffi::c_int) as int16_t,
    -(448 as ::core::ffi::c_int) as int16_t,
    -(472 as ::core::ffi::c_int) as int16_t,
    856 as ::core::ffi::c_int as int16_t,
    -(556 as ::core::ffi::c_int) as int16_t,
    -(364 as ::core::ffi::c_int) as int16_t,
    572 as ::core::ffi::c_int as int16_t,
    -(12 as ::core::ffi::c_int) as int16_t,
    -(156 as ::core::ffi::c_int) as int16_t,
    -(368 as ::core::ffi::c_int) as int16_t,
    -(340 as ::core::ffi::c_int) as int16_t,
    432 as ::core::ffi::c_int as int16_t,
    252 as ::core::ffi::c_int as int16_t,
    -(752 as ::core::ffi::c_int) as int16_t,
    -(152 as ::core::ffi::c_int) as int16_t,
    288 as ::core::ffi::c_int as int16_t,
    268 as ::core::ffi::c_int as int16_t,
    -(580 as ::core::ffi::c_int) as int16_t,
    -(848 as ::core::ffi::c_int) as int16_t,
    -(592 as ::core::ffi::c_int) as int16_t,
    108 as ::core::ffi::c_int as int16_t,
    -(76 as ::core::ffi::c_int) as int16_t,
    244 as ::core::ffi::c_int as int16_t,
    312 as ::core::ffi::c_int as int16_t,
    -(716 as ::core::ffi::c_int) as int16_t,
    592 as ::core::ffi::c_int as int16_t,
    -(80 as ::core::ffi::c_int) as int16_t,
    436 as ::core::ffi::c_int as int16_t,
    360 as ::core::ffi::c_int as int16_t,
    4 as ::core::ffi::c_int as int16_t,
    -(248 as ::core::ffi::c_int) as int16_t,
    160 as ::core::ffi::c_int as int16_t,
    516 as ::core::ffi::c_int as int16_t,
    584 as ::core::ffi::c_int as int16_t,
    732 as ::core::ffi::c_int as int16_t,
    44 as ::core::ffi::c_int as int16_t,
    -(468 as ::core::ffi::c_int) as int16_t,
    -(280 as ::core::ffi::c_int) as int16_t,
    -(292 as ::core::ffi::c_int) as int16_t,
    -(156 as ::core::ffi::c_int) as int16_t,
    -(588 as ::core::ffi::c_int) as int16_t,
    28 as ::core::ffi::c_int as int16_t,
    308 as ::core::ffi::c_int as int16_t,
    912 as ::core::ffi::c_int as int16_t,
    24 as ::core::ffi::c_int as int16_t,
    124 as ::core::ffi::c_int as int16_t,
    156 as ::core::ffi::c_int as int16_t,
    180 as ::core::ffi::c_int as int16_t,
    -(252 as ::core::ffi::c_int) as int16_t,
    944 as ::core::ffi::c_int as int16_t,
    -(924 as ::core::ffi::c_int) as int16_t,
    -(772 as ::core::ffi::c_int) as int16_t,
    -(520 as ::core::ffi::c_int) as int16_t,
    -(428 as ::core::ffi::c_int) as int16_t,
    -(624 as ::core::ffi::c_int) as int16_t,
    300 as ::core::ffi::c_int as int16_t,
    -(212 as ::core::ffi::c_int) as int16_t,
    -(1144 as ::core::ffi::c_int) as int16_t,
    32 as ::core::ffi::c_int as int16_t,
    -(724 as ::core::ffi::c_int) as int16_t,
    800 as ::core::ffi::c_int as int16_t,
    -(1128 as ::core::ffi::c_int) as int16_t,
    -(212 as ::core::ffi::c_int) as int16_t,
    -(1288 as ::core::ffi::c_int) as int16_t,
    -(848 as ::core::ffi::c_int) as int16_t,
    180 as ::core::ffi::c_int as int16_t,
    -(416 as ::core::ffi::c_int) as int16_t,
    440 as ::core::ffi::c_int as int16_t,
    192 as ::core::ffi::c_int as int16_t,
    -(576 as ::core::ffi::c_int) as int16_t,
    -(792 as ::core::ffi::c_int) as int16_t,
    -(76 as ::core::ffi::c_int) as int16_t,
    -(1080 as ::core::ffi::c_int) as int16_t,
    80 as ::core::ffi::c_int as int16_t,
    -(532 as ::core::ffi::c_int) as int16_t,
    -(352 as ::core::ffi::c_int) as int16_t,
    -(132 as ::core::ffi::c_int) as int16_t,
    380 as ::core::ffi::c_int as int16_t,
    -(820 as ::core::ffi::c_int) as int16_t,
    148 as ::core::ffi::c_int as int16_t,
    1112 as ::core::ffi::c_int as int16_t,
    128 as ::core::ffi::c_int as int16_t,
    164 as ::core::ffi::c_int as int16_t,
    456 as ::core::ffi::c_int as int16_t,
    700 as ::core::ffi::c_int as int16_t,
    -(924 as ::core::ffi::c_int) as int16_t,
    144 as ::core::ffi::c_int as int16_t,
    -(668 as ::core::ffi::c_int) as int16_t,
    -(384 as ::core::ffi::c_int) as int16_t,
    648 as ::core::ffi::c_int as int16_t,
    -(832 as ::core::ffi::c_int) as int16_t,
    508 as ::core::ffi::c_int as int16_t,
    552 as ::core::ffi::c_int as int16_t,
    -(52 as ::core::ffi::c_int) as int16_t,
    -(100 as ::core::ffi::c_int) as int16_t,
    -(656 as ::core::ffi::c_int) as int16_t,
    208 as ::core::ffi::c_int as int16_t,
    -(568 as ::core::ffi::c_int) as int16_t,
    748 as ::core::ffi::c_int as int16_t,
    -(88 as ::core::ffi::c_int) as int16_t,
    680 as ::core::ffi::c_int as int16_t,
    232 as ::core::ffi::c_int as int16_t,
    300 as ::core::ffi::c_int as int16_t,
    192 as ::core::ffi::c_int as int16_t,
    -(408 as ::core::ffi::c_int) as int16_t,
    -(1012 as ::core::ffi::c_int) as int16_t,
    -(152 as ::core::ffi::c_int) as int16_t,
    -(252 as ::core::ffi::c_int) as int16_t,
    -(268 as ::core::ffi::c_int) as int16_t,
    272 as ::core::ffi::c_int as int16_t,
    -(876 as ::core::ffi::c_int) as int16_t,
    -(664 as ::core::ffi::c_int) as int16_t,
    -(648 as ::core::ffi::c_int) as int16_t,
    -(332 as ::core::ffi::c_int) as int16_t,
    -(136 as ::core::ffi::c_int) as int16_t,
    16 as ::core::ffi::c_int as int16_t,
    12 as ::core::ffi::c_int as int16_t,
    1152 as ::core::ffi::c_int as int16_t,
    -(28 as ::core::ffi::c_int) as int16_t,
    332 as ::core::ffi::c_int as int16_t,
    -(536 as ::core::ffi::c_int) as int16_t,
    320 as ::core::ffi::c_int as int16_t,
    -(672 as ::core::ffi::c_int) as int16_t,
    -(460 as ::core::ffi::c_int) as int16_t,
    -(316 as ::core::ffi::c_int) as int16_t,
    532 as ::core::ffi::c_int as int16_t,
    -(260 as ::core::ffi::c_int) as int16_t,
    228 as ::core::ffi::c_int as int16_t,
    -(40 as ::core::ffi::c_int) as int16_t,
    1052 as ::core::ffi::c_int as int16_t,
    -(816 as ::core::ffi::c_int) as int16_t,
    180 as ::core::ffi::c_int as int16_t,
    88 as ::core::ffi::c_int as int16_t,
    -(496 as ::core::ffi::c_int) as int16_t,
    -(556 as ::core::ffi::c_int) as int16_t,
    -(672 as ::core::ffi::c_int) as int16_t,
    -(368 as ::core::ffi::c_int) as int16_t,
    428 as ::core::ffi::c_int as int16_t,
    92 as ::core::ffi::c_int as int16_t,
    356 as ::core::ffi::c_int as int16_t,
    404 as ::core::ffi::c_int as int16_t,
    -(408 as ::core::ffi::c_int) as int16_t,
    252 as ::core::ffi::c_int as int16_t,
    196 as ::core::ffi::c_int as int16_t,
    -(176 as ::core::ffi::c_int) as int16_t,
    -(556 as ::core::ffi::c_int) as int16_t,
    792 as ::core::ffi::c_int as int16_t,
    268 as ::core::ffi::c_int as int16_t,
    32 as ::core::ffi::c_int as int16_t,
    372 as ::core::ffi::c_int as int16_t,
    40 as ::core::ffi::c_int as int16_t,
    96 as ::core::ffi::c_int as int16_t,
    -(332 as ::core::ffi::c_int) as int16_t,
    328 as ::core::ffi::c_int as int16_t,
    120 as ::core::ffi::c_int as int16_t,
    372 as ::core::ffi::c_int as int16_t,
    -(900 as ::core::ffi::c_int) as int16_t,
    -(40 as ::core::ffi::c_int) as int16_t,
    472 as ::core::ffi::c_int as int16_t,
    -(264 as ::core::ffi::c_int) as int16_t,
    -(592 as ::core::ffi::c_int) as int16_t,
    952 as ::core::ffi::c_int as int16_t,
    128 as ::core::ffi::c_int as int16_t,
    656 as ::core::ffi::c_int as int16_t,
    112 as ::core::ffi::c_int as int16_t,
    664 as ::core::ffi::c_int as int16_t,
    -(232 as ::core::ffi::c_int) as int16_t,
    420 as ::core::ffi::c_int as int16_t,
    4 as ::core::ffi::c_int as int16_t,
    -(344 as ::core::ffi::c_int) as int16_t,
    -(464 as ::core::ffi::c_int) as int16_t,
    556 as ::core::ffi::c_int as int16_t,
    244 as ::core::ffi::c_int as int16_t,
    -(416 as ::core::ffi::c_int) as int16_t,
    -(32 as ::core::ffi::c_int) as int16_t,
    252 as ::core::ffi::c_int as int16_t,
    0 as ::core::ffi::c_int as int16_t,
    -(412 as ::core::ffi::c_int) as int16_t,
    188 as ::core::ffi::c_int as int16_t,
    -(696 as ::core::ffi::c_int) as int16_t,
    508 as ::core::ffi::c_int as int16_t,
    -(476 as ::core::ffi::c_int) as int16_t,
    324 as ::core::ffi::c_int as int16_t,
    -(1096 as ::core::ffi::c_int) as int16_t,
    656 as ::core::ffi::c_int as int16_t,
    -(312 as ::core::ffi::c_int) as int16_t,
    560 as ::core::ffi::c_int as int16_t,
    264 as ::core::ffi::c_int as int16_t,
    -(136 as ::core::ffi::c_int) as int16_t,
    304 as ::core::ffi::c_int as int16_t,
    160 as ::core::ffi::c_int as int16_t,
    -(64 as ::core::ffi::c_int) as int16_t,
    -(580 as ::core::ffi::c_int) as int16_t,
    248 as ::core::ffi::c_int as int16_t,
    336 as ::core::ffi::c_int as int16_t,
    -(720 as ::core::ffi::c_int) as int16_t,
    560 as ::core::ffi::c_int as int16_t,
    -(348 as ::core::ffi::c_int) as int16_t,
    -(288 as ::core::ffi::c_int) as int16_t,
    -(276 as ::core::ffi::c_int) as int16_t,
    -(196 as ::core::ffi::c_int) as int16_t,
    -(500 as ::core::ffi::c_int) as int16_t,
    852 as ::core::ffi::c_int as int16_t,
    -(544 as ::core::ffi::c_int) as int16_t,
    -(236 as ::core::ffi::c_int) as int16_t,
    -(1128 as ::core::ffi::c_int) as int16_t,
    -(992 as ::core::ffi::c_int) as int16_t,
    -(776 as ::core::ffi::c_int) as int16_t,
    116 as ::core::ffi::c_int as int16_t,
    56 as ::core::ffi::c_int as int16_t,
    52 as ::core::ffi::c_int as int16_t,
    860 as ::core::ffi::c_int as int16_t,
    884 as ::core::ffi::c_int as int16_t,
    212 as ::core::ffi::c_int as int16_t,
    -(12 as ::core::ffi::c_int) as int16_t,
    168 as ::core::ffi::c_int as int16_t,
    1020 as ::core::ffi::c_int as int16_t,
    512 as ::core::ffi::c_int as int16_t,
    -(552 as ::core::ffi::c_int) as int16_t,
    924 as ::core::ffi::c_int as int16_t,
    -(148 as ::core::ffi::c_int) as int16_t,
    716 as ::core::ffi::c_int as int16_t,
    188 as ::core::ffi::c_int as int16_t,
    164 as ::core::ffi::c_int as int16_t,
    -(340 as ::core::ffi::c_int) as int16_t,
    -(520 as ::core::ffi::c_int) as int16_t,
    -(184 as ::core::ffi::c_int) as int16_t,
    880 as ::core::ffi::c_int as int16_t,
    -(152 as ::core::ffi::c_int) as int16_t,
    -(680 as ::core::ffi::c_int) as int16_t,
    -(208 as ::core::ffi::c_int) as int16_t,
    -(1156 as ::core::ffi::c_int) as int16_t,
    -(300 as ::core::ffi::c_int) as int16_t,
    -(528 as ::core::ffi::c_int) as int16_t,
    -(472 as ::core::ffi::c_int) as int16_t,
    364 as ::core::ffi::c_int as int16_t,
    100 as ::core::ffi::c_int as int16_t,
    -(744 as ::core::ffi::c_int) as int16_t,
    -(1056 as ::core::ffi::c_int) as int16_t,
    -(32 as ::core::ffi::c_int) as int16_t,
    540 as ::core::ffi::c_int as int16_t,
    280 as ::core::ffi::c_int as int16_t,
    144 as ::core::ffi::c_int as int16_t,
    -(676 as ::core::ffi::c_int) as int16_t,
    -(32 as ::core::ffi::c_int) as int16_t,
    -(232 as ::core::ffi::c_int) as int16_t,
    -(280 as ::core::ffi::c_int) as int16_t,
    -(224 as ::core::ffi::c_int) as int16_t,
    96 as ::core::ffi::c_int as int16_t,
    568 as ::core::ffi::c_int as int16_t,
    -(76 as ::core::ffi::c_int) as int16_t,
    172 as ::core::ffi::c_int as int16_t,
    148 as ::core::ffi::c_int as int16_t,
    148 as ::core::ffi::c_int as int16_t,
    104 as ::core::ffi::c_int as int16_t,
    32 as ::core::ffi::c_int as int16_t,
    -(296 as ::core::ffi::c_int) as int16_t,
    -(32 as ::core::ffi::c_int) as int16_t,
    788 as ::core::ffi::c_int as int16_t,
    -(80 as ::core::ffi::c_int) as int16_t,
    32 as ::core::ffi::c_int as int16_t,
    -(16 as ::core::ffi::c_int) as int16_t,
    280 as ::core::ffi::c_int as int16_t,
    288 as ::core::ffi::c_int as int16_t,
    944 as ::core::ffi::c_int as int16_t,
    428 as ::core::ffi::c_int as int16_t,
    -(484 as ::core::ffi::c_int) as int16_t,
];
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
unsafe extern "C" fn generate_scaling_8(
    mut points: *const [uint8_t; 2],
    num: ::core::ffi::c_int,
    mut scaling: *mut uint8_t,
) {
    let shift_x: ::core::ffi::c_int = 8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int;
    let scaling_size: ::core::ffi::c_int = (1 as ::core::ffi::c_int)
        << 8 as ::core::ffi::c_int;
    let max_value: ::core::ffi::c_int = ((*points
        .offset(
            (num - 1 as ::core::ffi::c_int) as isize,
        ))[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int) << shift_x;
    if !(scaling_size <= 256 as ::core::ffi::c_int) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"scaling_size <= 256\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavcodec/aom_film_grain_template.c\0" as *const u8
                as *const ::core::ffi::c_char,
            439 as ::core::ffi::c_int,
        );
        abort();
    }
    if num == 0 as ::core::ffi::c_int {
        memset(
            scaling as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            scaling_size as size_t,
        );
        return;
    }
    memset(
        scaling as *mut ::core::ffi::c_void,
        (*points
            .offset(0 as ::core::ffi::c_int as isize))[1 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int,
        (((*points
            .offset(0 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int) << shift_x) as size_t,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < num - 1 as ::core::ffi::c_int {
        let bx: ::core::ffi::c_int = (*points
            .offset(i as isize))[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        let by: ::core::ffi::c_int = (*points
            .offset(i as isize))[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        let ex: ::core::ffi::c_int = (*points
            .offset(
                (i + 1 as ::core::ffi::c_int) as isize,
            ))[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        let ey: ::core::ffi::c_int = (*points
            .offset(
                (i + 1 as ::core::ffi::c_int) as isize,
            ))[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        let dx: ::core::ffi::c_int = ex - bx;
        let dy: ::core::ffi::c_int = ey - by;
        let delta: ::core::ffi::c_int = dy
            * ((0x10000 as ::core::ffi::c_int + (dx >> 1 as ::core::ffi::c_int)) / dx);
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut d: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
        while x < dx {
            *scaling.offset((bx + x << shift_x) as isize) = (by
                + (d >> 16 as ::core::ffi::c_int)) as uint8_t;
            d += delta;
            x += 1;
        }
        i += 1;
    }
    memset(
        scaling.offset(max_value as isize) as *mut uint8_t as *mut ::core::ffi::c_void,
        (*points
            .offset(
                (num - 1 as ::core::ffi::c_int) as isize,
            ))[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
        (scaling_size - max_value) as size_t,
    );
}
unsafe extern "C" fn generate_scaling_16(
    mut points: *const [uint8_t; 2],
    num: ::core::ffi::c_int,
    mut scaling: *mut uint8_t,
    bitdepth: ::core::ffi::c_int,
) {
    let shift_x: ::core::ffi::c_int = bitdepth - 8 as ::core::ffi::c_int;
    let scaling_size: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << bitdepth;
    let max_value: ::core::ffi::c_int = ((*points
        .offset(
            (num - 1 as ::core::ffi::c_int) as isize,
        ))[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int) << shift_x;
    if !(scaling_size <= 4096 as ::core::ffi::c_int) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"scaling_size <= 4096\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavcodec/aom_film_grain_template.c\0" as *const u8
                as *const ::core::ffi::c_char,
            439 as ::core::ffi::c_int,
        );
        abort();
    }
    if num == 0 as ::core::ffi::c_int {
        memset(
            scaling as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            scaling_size as size_t,
        );
        return;
    }
    memset(
        scaling as *mut ::core::ffi::c_void,
        (*points
            .offset(0 as ::core::ffi::c_int as isize))[1 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int,
        (((*points
            .offset(0 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int) << shift_x) as size_t,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < num - 1 as ::core::ffi::c_int {
        let bx: ::core::ffi::c_int = (*points
            .offset(i as isize))[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        let by: ::core::ffi::c_int = (*points
            .offset(i as isize))[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        let ex: ::core::ffi::c_int = (*points
            .offset(
                (i + 1 as ::core::ffi::c_int) as isize,
            ))[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        let ey: ::core::ffi::c_int = (*points
            .offset(
                (i + 1 as ::core::ffi::c_int) as isize,
            ))[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        let dx: ::core::ffi::c_int = ex - bx;
        let dy: ::core::ffi::c_int = ey - by;
        let delta: ::core::ffi::c_int = dy
            * ((0x10000 as ::core::ffi::c_int + (dx >> 1 as ::core::ffi::c_int)) / dx);
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut d: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
        while x < dx {
            *scaling.offset((bx + x << shift_x) as isize) = (by
                + (d >> 16 as ::core::ffi::c_int)) as uint8_t;
            d += delta;
            x += 1;
        }
        i += 1;
    }
    memset(
        scaling.offset(max_value as isize) as *mut uint8_t as *mut ::core::ffi::c_void,
        (*points
            .offset(
                (num - 1 as ::core::ffi::c_int) as isize,
            ))[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
        (scaling_size - max_value) as size_t,
    );
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < num - 1 as ::core::ffi::c_int {
        let pad: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << shift_x;
        let rnd: ::core::ffi::c_int = pad >> 1 as ::core::ffi::c_int;
        let bx_0: ::core::ffi::c_int = ((*points
            .offset(i_0 as isize))[0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int) << shift_x;
        let ex_0: ::core::ffi::c_int = ((*points
            .offset(
                (i_0 + 1 as ::core::ffi::c_int) as isize,
            ))[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int) << shift_x;
        let dx_0: ::core::ffi::c_int = ex_0 - bx_0;
        let mut x_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x_0 < dx_0 {
            let range: ::core::ffi::c_int = *scaling.offset((bx_0 + x_0 + pad) as isize)
                as ::core::ffi::c_int
                - *scaling.offset((bx_0 + x_0) as isize) as ::core::ffi::c_int;
            let mut n: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            let mut r: ::core::ffi::c_int = rnd;
            while n < pad {
                r += range;
                *scaling.offset((bx_0 + x_0 + n) as isize) = (*scaling
                    .offset((bx_0 + x_0) as isize) as ::core::ffi::c_int
                    + (r >> shift_x)) as uint8_t;
                n += 1;
            }
            x_0 += pad;
        }
        i_0 += 1;
    }
}
#[inline(always)]
unsafe extern "C" fn apply_grain_row_8(
    mut out: *mut AVFrame,
    mut in_0: *const AVFrame,
    ss_x: ::core::ffi::c_int,
    ss_y: ::core::ffi::c_int,
    mut scaling: *const [uint8_t; 256],
    mut grain_lut: *const [[int8_t; 82]; 74],
    mut params: *const AVFilmGrainParams,
    row: ::core::ffi::c_int,
) {
    let data: *const AVFilmGrainAOMParams = &raw const (*params).codec.aom;
    let cpw: ::core::ffi::c_int = (*out).width + ss_x >> ss_x;
    let is_id: ::core::ffi::c_int = ((*out).colorspace as ::core::ffi::c_uint
        == AVCOL_SPC_RGB as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
    let bh: ::core::ffi::c_int = (if (*out).height
        - row * FG_BLOCK_SIZE as ::core::ffi::c_int > FG_BLOCK_SIZE as ::core::ffi::c_int
    {
        FG_BLOCK_SIZE as ::core::ffi::c_int as ::core::ffi::c_int
    } else {
        (*out).height - row * FG_BLOCK_SIZE as ::core::ffi::c_int as ::core::ffi::c_int
    }) + ss_y >> ss_y;
    let uv_off: ptrdiff_t = (row * FG_BLOCK_SIZE as ::core::ffi::c_int
        * (*out).linesize[1 as ::core::ffi::c_int as usize] >> ss_y) as ptrdiff_t;
    let luma_src: *mut uint8_t = ((*in_0).data[0 as ::core::ffi::c_int as usize]
        as *mut ::core::ffi::c_char)
        .offset(
            (row * FG_BLOCK_SIZE as ::core::ffi::c_int
                * (*in_0).linesize[0 as ::core::ffi::c_int as usize]) as isize,
        ) as *mut uint8_t;
    if (*data).num_y_points != 0 {
        let bh_0: ::core::ffi::c_int = if (*out).height
            - row * FG_BLOCK_SIZE as ::core::ffi::c_int
            > FG_BLOCK_SIZE as ::core::ffi::c_int
        {
            FG_BLOCK_SIZE as ::core::ffi::c_int as ::core::ffi::c_int
        } else {
            (*out).height
                - row * FG_BLOCK_SIZE as ::core::ffi::c_int as ::core::ffi::c_int
        };
        let off: ptrdiff_t = (row * FG_BLOCK_SIZE as ::core::ffi::c_int
            * (*out).linesize[0 as ::core::ffi::c_int as usize]) as ptrdiff_t;
        fgy_32x32xn_c_8(
            ((*out).data[0 as ::core::ffi::c_int as usize] as *mut ::core::ffi::c_char)
                .offset(off as isize) as *mut uint8_t,
            luma_src,
            (*out).linesize[0 as ::core::ffi::c_int as usize] as ptrdiff_t,
            params,
            (*out).width as size_t,
            &raw const *scaling.offset(0 as ::core::ffi::c_int as isize)
                as *const uint8_t,
            &raw const *grain_lut.offset(0 as ::core::ffi::c_int as isize)
                as *const [int8_t; 82],
            bh_0,
            row,
        );
    }
    if (*data).num_uv_points[0 as ::core::ffi::c_int as usize] == 0
        && (*data).num_uv_points[1 as ::core::ffi::c_int as usize] == 0
        && (*data).chroma_scaling_from_luma == 0
    {
        return;
    }
    if (*out).width & ss_x != 0 {
        let mut ptr: *mut uint8_t = luma_src;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < bh {
            *ptr.offset((*out).width as isize) = *ptr
                .offset(((*out).width - 1 as ::core::ffi::c_int) as isize);
            ptr = (ptr as *mut ::core::ffi::c_char)
                .offset(
                    ((*in_0).linesize[0 as ::core::ffi::c_int as usize] << ss_y) as isize,
                ) as *mut uint8_t;
            y += 1;
        }
    }
    if (*data).chroma_scaling_from_luma != 0 {
        let mut pl: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while pl < 2 as ::core::ffi::c_int {
            fguv_32x32xn_c_8(
                ((*out).data[(1 as ::core::ffi::c_int + pl) as usize]
                    as *mut ::core::ffi::c_char)
                    .offset(uv_off as isize) as *mut uint8_t,
                ((*in_0).data[(1 as ::core::ffi::c_int + pl) as usize]
                    as *const ::core::ffi::c_char)
                    .offset(uv_off as isize) as *const uint8_t,
                (*in_0).linesize[1 as ::core::ffi::c_int as usize] as ptrdiff_t,
                params,
                cpw as size_t,
                &raw const *scaling.offset(0 as ::core::ffi::c_int as isize)
                    as *const uint8_t,
                &raw const *grain_lut.offset((1 as ::core::ffi::c_int + pl) as isize)
                    as *const [int8_t; 82],
                bh,
                row,
                luma_src,
                (*in_0).linesize[0 as ::core::ffi::c_int as usize] as ptrdiff_t,
                pl,
                is_id,
                ss_x,
                ss_y,
            );
            pl += 1;
        }
    } else {
        let mut pl_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while pl_0 < 2 as ::core::ffi::c_int {
            if (*data).num_uv_points[pl_0 as usize] != 0 {
                fguv_32x32xn_c_8(
                    ((*out).data[(1 as ::core::ffi::c_int + pl_0) as usize]
                        as *mut ::core::ffi::c_char)
                        .offset(uv_off as isize) as *mut uint8_t,
                    ((*in_0).data[(1 as ::core::ffi::c_int + pl_0) as usize]
                        as *const ::core::ffi::c_char)
                        .offset(uv_off as isize) as *const uint8_t,
                    (*in_0).linesize[1 as ::core::ffi::c_int as usize] as ptrdiff_t,
                    params,
                    cpw as size_t,
                    &raw const *scaling.offset((1 as ::core::ffi::c_int + pl_0) as isize)
                        as *const uint8_t,
                    &raw const *grain_lut
                        .offset((1 as ::core::ffi::c_int + pl_0) as isize)
                        as *const [int8_t; 82],
                    bh,
                    row,
                    luma_src,
                    (*in_0).linesize[0 as ::core::ffi::c_int as usize] as ptrdiff_t,
                    pl_0,
                    is_id,
                    ss_x,
                    ss_y,
                );
            }
            pl_0 += 1;
        }
    };
}
#[inline(always)]
unsafe extern "C" fn apply_grain_row_16(
    mut out: *mut AVFrame,
    mut in_0: *const AVFrame,
    ss_x: ::core::ffi::c_int,
    ss_y: ::core::ffi::c_int,
    mut scaling: *const [uint8_t; 4096],
    mut grain_lut: *const [[int16_t; 82]; 74],
    mut params: *const AVFilmGrainParams,
    row: ::core::ffi::c_int,
    bitdepth: ::core::ffi::c_int,
) {
    let data: *const AVFilmGrainAOMParams = &raw const (*params).codec.aom;
    let cpw: ::core::ffi::c_int = (*out).width + ss_x >> ss_x;
    let is_id: ::core::ffi::c_int = ((*out).colorspace as ::core::ffi::c_uint
        == AVCOL_SPC_RGB as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
    let bh: ::core::ffi::c_int = (if (*out).height
        - row * FG_BLOCK_SIZE as ::core::ffi::c_int > FG_BLOCK_SIZE as ::core::ffi::c_int
    {
        FG_BLOCK_SIZE as ::core::ffi::c_int as ::core::ffi::c_int
    } else {
        (*out).height - row * FG_BLOCK_SIZE as ::core::ffi::c_int as ::core::ffi::c_int
    }) + ss_y >> ss_y;
    let uv_off: ptrdiff_t = (row * FG_BLOCK_SIZE as ::core::ffi::c_int
        * (*out).linesize[1 as ::core::ffi::c_int as usize] >> ss_y) as ptrdiff_t;
    let luma_src: *mut uint16_t = ((*in_0).data[0 as ::core::ffi::c_int as usize]
        as *mut ::core::ffi::c_char)
        .offset(
            (row * FG_BLOCK_SIZE as ::core::ffi::c_int
                * (*in_0).linesize[0 as ::core::ffi::c_int as usize]) as isize,
        ) as *mut uint16_t;
    if (*data).num_y_points != 0 {
        let bh_0: ::core::ffi::c_int = if (*out).height
            - row * FG_BLOCK_SIZE as ::core::ffi::c_int
            > FG_BLOCK_SIZE as ::core::ffi::c_int
        {
            FG_BLOCK_SIZE as ::core::ffi::c_int as ::core::ffi::c_int
        } else {
            (*out).height
                - row * FG_BLOCK_SIZE as ::core::ffi::c_int as ::core::ffi::c_int
        };
        let off: ptrdiff_t = (row * FG_BLOCK_SIZE as ::core::ffi::c_int
            * (*out).linesize[0 as ::core::ffi::c_int as usize]) as ptrdiff_t;
        fgy_32x32xn_c_16(
            ((*out).data[0 as ::core::ffi::c_int as usize] as *mut ::core::ffi::c_char)
                .offset(off as isize) as *mut uint16_t,
            luma_src,
            (*out).linesize[0 as ::core::ffi::c_int as usize] as ptrdiff_t,
            params,
            (*out).width as size_t,
            &raw const *scaling.offset(0 as ::core::ffi::c_int as isize)
                as *const uint8_t,
            &raw const *grain_lut.offset(0 as ::core::ffi::c_int as isize)
                as *const [int16_t; 82],
            bh_0,
            row,
            bitdepth,
        );
    }
    if (*data).num_uv_points[0 as ::core::ffi::c_int as usize] == 0
        && (*data).num_uv_points[1 as ::core::ffi::c_int as usize] == 0
        && (*data).chroma_scaling_from_luma == 0
    {
        return;
    }
    if (*out).width & ss_x != 0 {
        let mut ptr: *mut uint16_t = luma_src;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < bh {
            *ptr.offset((*out).width as isize) = *ptr
                .offset(((*out).width - 1 as ::core::ffi::c_int) as isize);
            ptr = (ptr as *mut ::core::ffi::c_char)
                .offset(
                    ((*in_0).linesize[0 as ::core::ffi::c_int as usize] << ss_y) as isize,
                ) as *mut uint16_t;
            y += 1;
        }
    }
    if (*data).chroma_scaling_from_luma != 0 {
        let mut pl: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while pl < 2 as ::core::ffi::c_int {
            fguv_32x32xn_c_16(
                ((*out).data[(1 as ::core::ffi::c_int + pl) as usize]
                    as *mut ::core::ffi::c_char)
                    .offset(uv_off as isize) as *mut uint16_t,
                ((*in_0).data[(1 as ::core::ffi::c_int + pl) as usize]
                    as *const ::core::ffi::c_char)
                    .offset(uv_off as isize) as *const uint16_t,
                (*in_0).linesize[1 as ::core::ffi::c_int as usize] as ptrdiff_t,
                params,
                cpw as size_t,
                &raw const *scaling.offset(0 as ::core::ffi::c_int as isize)
                    as *const uint8_t,
                &raw const *grain_lut.offset((1 as ::core::ffi::c_int + pl) as isize)
                    as *const [int16_t; 82],
                bh,
                row,
                luma_src,
                (*in_0).linesize[0 as ::core::ffi::c_int as usize] as ptrdiff_t,
                pl,
                is_id,
                ss_x,
                ss_y,
                bitdepth,
            );
            pl += 1;
        }
    } else {
        let mut pl_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while pl_0 < 2 as ::core::ffi::c_int {
            if (*data).num_uv_points[pl_0 as usize] != 0 {
                fguv_32x32xn_c_16(
                    ((*out).data[(1 as ::core::ffi::c_int + pl_0) as usize]
                        as *mut ::core::ffi::c_char)
                        .offset(uv_off as isize) as *mut uint16_t,
                    ((*in_0).data[(1 as ::core::ffi::c_int + pl_0) as usize]
                        as *const ::core::ffi::c_char)
                        .offset(uv_off as isize) as *const uint16_t,
                    (*in_0).linesize[1 as ::core::ffi::c_int as usize] as ptrdiff_t,
                    params,
                    cpw as size_t,
                    &raw const *scaling.offset((1 as ::core::ffi::c_int + pl_0) as isize)
                        as *const uint8_t,
                    &raw const *grain_lut
                        .offset((1 as ::core::ffi::c_int + pl_0) as isize)
                        as *const [int16_t; 82],
                    bh,
                    row,
                    luma_src,
                    (*in_0).linesize[0 as ::core::ffi::c_int as usize] as ptrdiff_t,
                    pl_0,
                    is_id,
                    ss_x,
                    ss_y,
                    bitdepth,
                );
            }
            pl_0 += 1;
        }
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
unsafe extern "C" fn apply_film_grain_16(
    mut out_frame: *mut AVFrame,
    mut in_frame: *const AVFrame,
    mut params: *const AVFilmGrainParams,
    bitdepth: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut grain_lut: [[[int16_t; 82]; 74]; 3] = [[[0; 82]; 74]; 3];
    let mut scaling: [[uint8_t; 4096]; 3] = [[0; 4096]; 3];
    let data: *const AVFilmGrainAOMParams = &raw const (*params).codec.aom;
    let mut desc: *const AVPixFmtDescriptor = av_pix_fmt_desc_get(
        (*out_frame).format as AVPixelFormat,
    );
    let rows: ::core::ffi::c_int = if 0 == 0 {
        -(-(*out_frame).height >> 5 as ::core::ffi::c_int)
    } else {
        (*out_frame).height + ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int
    };
    let subx: ::core::ffi::c_int = (*desc).log2_chroma_w as ::core::ffi::c_int;
    let suby: ::core::ffi::c_int = (*desc).log2_chroma_h as ::core::ffi::c_int;
    generate_grain_y_c_16(
        &raw mut *(&raw mut grain_lut as *mut [[int16_t; 82]; 74])
            .offset(0 as ::core::ffi::c_int as isize) as *mut [int16_t; 82],
        params,
        bitdepth,
    );
    if (*data).num_uv_points[0 as ::core::ffi::c_int as usize] != 0
        || (*data).chroma_scaling_from_luma != 0
    {
        generate_grain_uv_c_16(
            &raw mut *(&raw mut grain_lut as *mut [[int16_t; 82]; 74])
                .offset(1 as ::core::ffi::c_int as isize) as *mut [int16_t; 82],
            &raw mut *(&raw mut grain_lut as *mut [[int16_t; 82]; 74])
                .offset(0 as ::core::ffi::c_int as isize) as *mut [int16_t; 82]
                as *const [int16_t; 82],
            params,
            0 as intptr_t,
            subx,
            suby,
            bitdepth,
        );
    }
    if (*data).num_uv_points[1 as ::core::ffi::c_int as usize] != 0
        || (*data).chroma_scaling_from_luma != 0
    {
        generate_grain_uv_c_16(
            &raw mut *(&raw mut grain_lut as *mut [[int16_t; 82]; 74])
                .offset(2 as ::core::ffi::c_int as isize) as *mut [int16_t; 82],
            &raw mut *(&raw mut grain_lut as *mut [[int16_t; 82]; 74])
                .offset(0 as ::core::ffi::c_int as isize) as *mut [int16_t; 82]
                as *const [int16_t; 82],
            params,
            1 as intptr_t,
            subx,
            suby,
            bitdepth,
        );
    }
    if (*data).num_y_points != 0 || (*data).chroma_scaling_from_luma != 0 {
        generate_scaling_16(
            &raw const (*data).y_points as *const [uint8_t; 2],
            (*data).num_y_points,
            &raw mut *(&raw mut scaling as *mut [uint8_t; 4096])
                .offset(0 as ::core::ffi::c_int as isize) as *mut uint8_t,
            bitdepth,
        );
    }
    if (*data).num_uv_points[0 as ::core::ffi::c_int as usize] != 0 {
        generate_scaling_16(
            &raw const *(&raw const (*data).uv_points as *const [[uint8_t; 2]; 10])
                .offset(0 as ::core::ffi::c_int as isize) as *const [uint8_t; 2],
            (*data).num_uv_points[0 as ::core::ffi::c_int as usize],
            &raw mut *(&raw mut scaling as *mut [uint8_t; 4096])
                .offset(1 as ::core::ffi::c_int as isize) as *mut uint8_t,
            bitdepth,
        );
    }
    if (*data).num_uv_points[1 as ::core::ffi::c_int as usize] != 0 {
        generate_scaling_16(
            &raw const *(&raw const (*data).uv_points as *const [[uint8_t; 2]; 10])
                .offset(1 as ::core::ffi::c_int as isize) as *const [uint8_t; 2],
            (*data).num_uv_points[1 as ::core::ffi::c_int as usize],
            &raw mut *(&raw mut scaling as *mut [uint8_t; 4096])
                .offset(2 as ::core::ffi::c_int as isize) as *mut uint8_t,
            bitdepth,
        );
    }
    let mut row: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while row < rows {
        apply_grain_row_16(
            out_frame,
            in_frame,
            subx,
            suby,
            &raw mut scaling as *mut [uint8_t; 4096] as *const [uint8_t; 4096],
            &raw mut grain_lut as *mut [[int16_t; 82]; 74] as *const [[int16_t; 82]; 74],
            params,
            row,
            bitdepth,
        );
        row += 1;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn apply_film_grain_8(
    mut out_frame: *mut AVFrame,
    mut in_frame: *const AVFrame,
    mut params: *const AVFilmGrainParams,
) -> ::core::ffi::c_int {
    let mut grain_lut: [[[int8_t; 82]; 74]; 3] = [[[0; 82]; 74]; 3];
    let mut scaling: [[uint8_t; 256]; 3] = [[0; 256]; 3];
    let data: *const AVFilmGrainAOMParams = &raw const (*params).codec.aom;
    let mut desc: *const AVPixFmtDescriptor = av_pix_fmt_desc_get(
        (*out_frame).format as AVPixelFormat,
    );
    let rows: ::core::ffi::c_int = if 0 == 0 {
        -(-(*out_frame).height >> 5 as ::core::ffi::c_int)
    } else {
        (*out_frame).height + ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int
    };
    let subx: ::core::ffi::c_int = (*desc).log2_chroma_w as ::core::ffi::c_int;
    let suby: ::core::ffi::c_int = (*desc).log2_chroma_h as ::core::ffi::c_int;
    generate_grain_y_c_8(
        &raw mut *(&raw mut grain_lut as *mut [[int8_t; 82]; 74])
            .offset(0 as ::core::ffi::c_int as isize) as *mut [int8_t; 82],
        params,
    );
    if (*data).num_uv_points[0 as ::core::ffi::c_int as usize] != 0
        || (*data).chroma_scaling_from_luma != 0
    {
        generate_grain_uv_c_8(
            &raw mut *(&raw mut grain_lut as *mut [[int8_t; 82]; 74])
                .offset(1 as ::core::ffi::c_int as isize) as *mut [int8_t; 82],
            &raw mut *(&raw mut grain_lut as *mut [[int8_t; 82]; 74])
                .offset(0 as ::core::ffi::c_int as isize) as *mut [int8_t; 82]
                as *const [int8_t; 82],
            params,
            0 as intptr_t,
            subx,
            suby,
        );
    }
    if (*data).num_uv_points[1 as ::core::ffi::c_int as usize] != 0
        || (*data).chroma_scaling_from_luma != 0
    {
        generate_grain_uv_c_8(
            &raw mut *(&raw mut grain_lut as *mut [[int8_t; 82]; 74])
                .offset(2 as ::core::ffi::c_int as isize) as *mut [int8_t; 82],
            &raw mut *(&raw mut grain_lut as *mut [[int8_t; 82]; 74])
                .offset(0 as ::core::ffi::c_int as isize) as *mut [int8_t; 82]
                as *const [int8_t; 82],
            params,
            1 as intptr_t,
            subx,
            suby,
        );
    }
    if (*data).num_y_points != 0 || (*data).chroma_scaling_from_luma != 0 {
        generate_scaling_8(
            &raw const (*data).y_points as *const [uint8_t; 2],
            (*data).num_y_points,
            &raw mut *(&raw mut scaling as *mut [uint8_t; 256])
                .offset(0 as ::core::ffi::c_int as isize) as *mut uint8_t,
        );
    }
    if (*data).num_uv_points[0 as ::core::ffi::c_int as usize] != 0 {
        generate_scaling_8(
            &raw const *(&raw const (*data).uv_points as *const [[uint8_t; 2]; 10])
                .offset(0 as ::core::ffi::c_int as isize) as *const [uint8_t; 2],
            (*data).num_uv_points[0 as ::core::ffi::c_int as usize],
            &raw mut *(&raw mut scaling as *mut [uint8_t; 256])
                .offset(1 as ::core::ffi::c_int as isize) as *mut uint8_t,
        );
    }
    if (*data).num_uv_points[1 as ::core::ffi::c_int as usize] != 0 {
        generate_scaling_8(
            &raw const *(&raw const (*data).uv_points as *const [[uint8_t; 2]; 10])
                .offset(1 as ::core::ffi::c_int as isize) as *const [uint8_t; 2],
            (*data).num_uv_points[1 as ::core::ffi::c_int as usize],
            &raw mut *(&raw mut scaling as *mut [uint8_t; 256])
                .offset(2 as ::core::ffi::c_int as isize) as *mut uint8_t,
        );
    }
    let mut row: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while row < rows {
        apply_grain_row_8(
            out_frame,
            in_frame,
            subx,
            suby,
            &raw mut scaling as *mut [uint8_t; 256] as *const [uint8_t; 256],
            &raw mut grain_lut as *mut [[int8_t; 82]; 74] as *const [[int8_t; 82]; 74],
            params,
            row,
        );
        row += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[repr(C)]
pub struct AVBuffer {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVDictionary {
    _opaque: [u8; 0],
}
