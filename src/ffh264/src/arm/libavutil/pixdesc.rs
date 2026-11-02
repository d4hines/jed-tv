// Translated from FFmpeg 9.0.1 libavutil/pixdesc.c
// (preprocessed for armv7-unknown-linux-musleabihf) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// pixel format descriptor
// Copyright (c) 2009 Michael Niedermayer <michaelni@gmx.at>
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
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn av_strstart(
        str: *const ::core::ffi::c_char,
        pfx: *const ::core::ffi::c_char,
        ptr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn av_strlcpy(
        dst: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        size: size_t,
    ) -> size_t;
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn av_match_name(
        name: *const ::core::ffi::c_char,
        names: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
pub type size_t = ::core::ffi::c_uint;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint16_t = ::core::ffi::c_ushort;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint64_t = ::core::ffi::c_ulonglong;
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
#[repr(C, packed)]
pub union unaligned_32 {
    pub l: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union unaligned_16 {
    pub l: uint16_t,
}
#[no_mangle]
pub unsafe extern "C" fn av_read_image_line2(
    mut dst: *mut ::core::ffi::c_void,
    mut data: *mut *const uint8_t,
    mut linesize: *const ::core::ffi::c_int,
    mut desc: *const AVPixFmtDescriptor,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut c: ::core::ffi::c_int,
    mut w: ::core::ffi::c_int,
    mut read_pal_component: ::core::ffi::c_int,
    mut dst_element_size: ::core::ffi::c_int,
) {
    let mut comp: AVComponentDescriptor = (*desc).comp[c as usize];
    let mut plane: ::core::ffi::c_int = comp.plane;
    let mut depth: ::core::ffi::c_int = comp.depth;
    let mut mask: ::core::ffi::c_uint = ((1 as ::core::ffi::c_ulonglong) << depth)
        .wrapping_sub(1 as ::core::ffi::c_ulonglong) as ::core::ffi::c_uint;
    let mut shift: ::core::ffi::c_int = comp.shift;
    let mut step: ::core::ffi::c_int = comp.step;
    let mut flags: ::core::ffi::c_int = (*desc).flags as ::core::ffi::c_int;
    let mut dst16: *mut uint16_t = dst as *mut uint16_t;
    let mut dst32: *mut uint32_t = dst as *mut uint32_t;
    if depth == 0 {
        return;
    }
    if flags & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0 {
        if step > 8 as ::core::ffi::c_int {
            let mut byte_p: *const uint8_t = (*data.offset(plane as isize))
                .offset((y * *linesize.offset(plane as isize)) as isize);
            let mut p: *const uint32_t = byte_p as *mut uint32_t;
            loop {
                let fresh0 = w;
                w = w - 1;
                if !(fresh0 != 0) {
                    break;
                }
                let mut val: ::core::ffi::c_int = av_bswap32(
                    (*(p as *const unaligned_32)).l,
                ) as ::core::ffi::c_int;
                val = ((val >> comp.offset) as ::core::ffi::c_uint & mask)
                    as ::core::ffi::c_int;
                if read_pal_component != 0 {
                    val = *(*data.offset(1 as ::core::ffi::c_int as isize))
                        .offset((4 as ::core::ffi::c_int * val + c) as isize)
                        as ::core::ffi::c_int;
                }
                if dst_element_size == 4 as ::core::ffi::c_int {
                    let fresh1 = dst32;
                    dst32 = dst32.offset(1);
                    *fresh1 = val as uint32_t;
                } else {
                    let fresh2 = dst16;
                    dst16 = dst16.offset(1);
                    *fresh2 = val as uint16_t;
                }
                p = p.offset(1);
            }
        } else {
            let mut skip: ::core::ffi::c_int = x * step + comp.offset;
            let mut p_0: *const uint8_t = (*data.offset(plane as isize))
                .offset((y * *linesize.offset(plane as isize)) as isize)
                .offset((skip >> 3 as ::core::ffi::c_int) as isize);
            let mut shift_0: ::core::ffi::c_int = 8 as ::core::ffi::c_int - depth
                - (skip & 7 as ::core::ffi::c_int);
            loop {
                let fresh3 = w;
                w = w - 1;
                if !(fresh3 != 0) {
                    break;
                }
                let mut val_0: ::core::ffi::c_int = ((*p_0 as ::core::ffi::c_int
                    >> shift_0) as ::core::ffi::c_uint & mask) as ::core::ffi::c_int;
                if read_pal_component != 0 {
                    val_0 = *(*data.offset(1 as ::core::ffi::c_int as isize))
                        .offset((4 as ::core::ffi::c_int * val_0 + c) as isize)
                        as ::core::ffi::c_int;
                }
                shift_0 -= step;
                p_0 = p_0.offset(-((shift_0 >> 3 as ::core::ffi::c_int) as isize));
                shift_0 &= 7 as ::core::ffi::c_int;
                if dst_element_size == 4 as ::core::ffi::c_int {
                    let fresh4 = dst32;
                    dst32 = dst32.offset(1);
                    *fresh4 = val_0 as uint32_t;
                } else {
                    let fresh5 = dst16;
                    dst16 = dst16.offset(1);
                    *fresh5 = val_0 as uint16_t;
                }
            }
        }
    } else {
        let mut p_1: *const uint8_t = (*data.offset(plane as isize))
            .offset((y * *linesize.offset(plane as isize)) as isize)
            .offset((x * step) as isize)
            .offset(comp.offset as isize);
        let mut is_8bit: ::core::ffi::c_int = (shift + depth <= 8 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        let mut is_16bit: ::core::ffi::c_int = (shift + depth
            <= 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        if is_8bit != 0 {
            p_1 = p_1
                .offset(
                    (flags & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0)
                        as ::core::ffi::c_int as isize,
                );
        }
        loop {
            let fresh6 = w;
            w = w - 1;
            if !(fresh6 != 0) {
                break;
            }
            let mut val_1: ::core::ffi::c_uint = 0;
            if is_8bit != 0 {
                val_1 = *p_1 as ::core::ffi::c_uint;
            } else if is_16bit != 0 {
                val_1 = (if flags & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    != 0
                {
                    av_bswap16((*(p_1 as *const unaligned_16)).l) as ::core::ffi::c_int
                } else {
                    (*(p_1 as *const unaligned_16)).l as ::core::ffi::c_int
                }) as ::core::ffi::c_uint;
            } else {
                val_1 = (if flags & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    != 0
                {
                    av_bswap32((*(p_1 as *const unaligned_32)).l)
                } else {
                    (*(p_1 as *const unaligned_32)).l
                }) as ::core::ffi::c_uint;
            }
            val_1 = val_1 >> shift & mask;
            if read_pal_component != 0 {
                val_1 = *(*data.offset(1 as ::core::ffi::c_int as isize))
                    .offset(
                        (4 as ::core::ffi::c_uint)
                            .wrapping_mul(val_1)
                            .wrapping_add(c as ::core::ffi::c_uint) as isize,
                    ) as ::core::ffi::c_uint;
            }
            p_1 = p_1.offset(step as isize);
            if dst_element_size == 4 as ::core::ffi::c_int {
                let fresh7 = dst32;
                dst32 = dst32.offset(1);
                *fresh7 = val_1 as uint32_t;
            } else {
                let fresh8 = dst16;
                dst16 = dst16.offset(1);
                *fresh8 = val_1 as uint16_t;
            }
        }
    };
}
#[inline(always)]
unsafe extern "C" fn av_bswap16(mut x: uint16_t) -> uint16_t {
    x = (x as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
        | (x as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint16_t;
    return x;
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
pub unsafe extern "C" fn av_read_image_line(
    mut dst: *mut uint16_t,
    mut data: *mut *const uint8_t,
    mut linesize: *const ::core::ffi::c_int,
    mut desc: *const AVPixFmtDescriptor,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut c: ::core::ffi::c_int,
    mut w: ::core::ffi::c_int,
    mut read_pal_component: ::core::ffi::c_int,
) {
    av_read_image_line2(
        dst as *mut ::core::ffi::c_void,
        data,
        linesize,
        desc,
        x,
        y,
        c,
        w,
        read_pal_component,
        2 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_write_image_line2(
    mut src: *const ::core::ffi::c_void,
    mut data: *mut *mut uint8_t,
    mut linesize: *const ::core::ffi::c_int,
    mut desc: *const AVPixFmtDescriptor,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut c: ::core::ffi::c_int,
    mut w: ::core::ffi::c_int,
    mut src_element_size: ::core::ffi::c_int,
) {
    let mut comp: AVComponentDescriptor = (*desc).comp[c as usize];
    let mut plane: ::core::ffi::c_int = comp.plane;
    let mut depth: ::core::ffi::c_int = comp.depth;
    let mut step: ::core::ffi::c_int = comp.step;
    let mut flags: ::core::ffi::c_int = (*desc).flags as ::core::ffi::c_int;
    let mut src32: *const uint32_t = src as *const uint32_t;
    let mut src16: *const uint16_t = src as *const uint16_t;
    if depth == 0 {
        return;
    }
    if flags & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0 {
        if step > 8 as ::core::ffi::c_int {
            let mut byte_p: *const uint8_t = (*data.offset(plane as isize))
                .offset((y * *linesize.offset(plane as isize)) as isize);
            let mut p: *mut uint32_t = byte_p as *mut uint32_t;
            let mut offset: ::core::ffi::c_int = comp.offset;
            let mut mask: uint32_t = (((1 as ::core::ffi::c_ulonglong) << depth)
                .wrapping_sub(1 as ::core::ffi::c_ulonglong) << offset) as uint32_t;
            loop {
                let fresh9 = w;
                w = w - 1;
                if !(fresh9 != 0) {
                    break;
                }
                let mut val: ::core::ffi::c_uint = if src_element_size
                    == 4 as ::core::ffi::c_int
                {
                    let fresh10 = src32;
                    src32 = src32.offset(1);
                    *fresh10
                } else {
                    let fresh11 = src16;
                    src16 = src16.offset(1);
                    *fresh11 as ::core::ffi::c_uint
                };
                (*(p as *mut unaligned_32)).l = av_bswap32(
                    av_bswap32((*(p as *const unaligned_32)).l) & !mask
                        | (val as uint32_t) << offset,
                );
                p = p.offset(1);
            }
        } else {
            let mut skip: ::core::ffi::c_int = x * step + comp.offset;
            let mut p_0: *mut uint8_t = (*data.offset(plane as isize))
                .offset((y * *linesize.offset(plane as isize)) as isize)
                .offset((skip >> 3 as ::core::ffi::c_int) as isize);
            let mut shift: ::core::ffi::c_int = 8 as ::core::ffi::c_int - depth
                - (skip & 7 as ::core::ffi::c_int);
            loop {
                let fresh12 = w;
                w = w - 1;
                if !(fresh12 != 0) {
                    break;
                }
                *p_0 = (*p_0 as uint32_t
                    | (if src_element_size == 4 as ::core::ffi::c_int {
                        let fresh13 = src32;
                        src32 = src32.offset(1);
                        *fresh13
                    } else {
                        let fresh14 = src16;
                        src16 = src16.offset(1);
                        *fresh14 as uint32_t
                    }) << shift) as uint8_t;
                shift -= step;
                p_0 = p_0.offset(-((shift >> 3 as ::core::ffi::c_int) as isize));
                shift &= 7 as ::core::ffi::c_int;
            }
        }
    } else {
        let mut shift_0: ::core::ffi::c_int = comp.shift;
        let mut p_1: *mut uint8_t = (*data.offset(plane as isize))
            .offset((y * *linesize.offset(plane as isize)) as isize)
            .offset((x * step) as isize)
            .offset(comp.offset as isize);
        if shift_0 + depth <= 8 as ::core::ffi::c_int {
            p_1 = p_1
                .offset(
                    (flags & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0)
                        as ::core::ffi::c_int as isize,
                );
            loop {
                let fresh15 = w;
                w = w - 1;
                if !(fresh15 != 0) {
                    break;
                }
                *p_1 = (*p_1 as uint32_t
                    | (if src_element_size == 4 as ::core::ffi::c_int {
                        let fresh16 = src32;
                        src32 = src32.offset(1);
                        *fresh16
                    } else {
                        let fresh17 = src16;
                        src16 = src16.offset(1);
                        *fresh17 as uint32_t
                    }) << shift_0) as uint8_t;
                p_1 = p_1.offset(step as isize);
            }
        } else {
            loop {
                let fresh18 = w;
                w = w - 1;
                if !(fresh18 != 0) {
                    break;
                }
                let mut s: ::core::ffi::c_uint = if src_element_size
                    == 4 as ::core::ffi::c_int
                {
                    let fresh19 = src32;
                    src32 = src32.offset(1);
                    *fresh19
                } else {
                    let fresh20 = src16;
                    src16 = src16.offset(1);
                    *fresh20 as ::core::ffi::c_uint
                };
                if shift_0 + depth <= 16 as ::core::ffi::c_int {
                    if flags & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0
                    {
                        let mut val_0: uint16_t = (av_bswap16(
                            (*(p_1 as *const unaligned_16)).l,
                        ) as ::core::ffi::c_uint | s << shift_0) as uint16_t;
                        (*(p_1 as *mut unaligned_16)).l = av_bswap16(val_0);
                    } else {
                        let mut val_1: uint16_t = ((*(p_1 as *const unaligned_16)).l
                            as ::core::ffi::c_uint | s << shift_0) as uint16_t;
                        (*(p_1 as *mut unaligned_16)).l = val_1;
                    }
                } else if flags & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    != 0
                {
                    let mut val_2: uint32_t = av_bswap32(
                        (*(p_1 as *const unaligned_32)).l,
                    ) | (s as uint32_t) << shift_0;
                    (*(p_1 as *mut unaligned_32)).l = av_bswap32(val_2);
                } else {
                    let mut val_3: uint32_t = (*(p_1 as *const unaligned_32)).l
                        | (s as uint32_t) << shift_0;
                    (*(p_1 as *mut unaligned_32)).l = val_3;
                }
                p_1 = p_1.offset(step as isize);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_write_image_line(
    mut src: *const uint16_t,
    mut data: *mut *mut uint8_t,
    mut linesize: *const ::core::ffi::c_int,
    mut desc: *const AVPixFmtDescriptor,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut c: ::core::ffi::c_int,
    mut w: ::core::ffi::c_int,
) {
    av_write_image_line2(
        src as *const ::core::ffi::c_void,
        data,
        linesize,
        desc,
        x,
        y,
        c,
        w,
        2 as ::core::ffi::c_int,
    );
}
static mut av_pix_fmt_descriptors: [AVPixFmtDescriptor; 267] = [
    AVPixFmtDescriptor {
        name: b"yuv420p\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuyv422\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: 0,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 3 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"rgb24\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 3 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 3 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 3 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"bgr24\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 3 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 3 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 3 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv422p\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv444p\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv410p\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 2 as uint8_t,
        log2_chroma_h: 2 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv411p\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 2 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gray\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 1 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: 0,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: b"gray8,y8\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVPixFmtDescriptor {
        name: b"monow\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 1 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 1 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"monob\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 1 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 7 as ::core::ffi::c_int,
                depth: 1 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"pal8\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 1 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuvj420p\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuvj422p\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuvj444p\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"uyvy422\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: 0,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"uyyvyy411\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 2 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: 0,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 6 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 6 as ::core::ffi::c_int,
                offset: 3 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"bgr8\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 3 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 3 as ::core::ffi::c_int,
                depth: 3 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 2 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"bgr4\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 3 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 1 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 2 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 1 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"bgr4_byte\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 1 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 1 as ::core::ffi::c_int,
                depth: 2 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 3 as ::core::ffi::c_int,
                depth: 1 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"rgb8\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 5 as ::core::ffi::c_int,
                depth: 3 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 2 as ::core::ffi::c_int,
                depth: 3 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 2 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"rgb4\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 1 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 2 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 3 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 1 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"rgb4_byte\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 3 as ::core::ffi::c_int,
                depth: 1 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 1 as ::core::ffi::c_int,
                depth: 2 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 1 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"nv12\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"nv21\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"argb\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 3 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"rgba\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 3 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"abgr\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 3 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"bgra\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 3 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gray16be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 1 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: b"y16be\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVPixFmtDescriptor {
        name: b"gray16le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 1 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: 0,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: b"y16le\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVPixFmtDescriptor {
        name: b"yuv440p\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuvj440p\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuva420p\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"rgb48be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 6 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 6 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 6 as ::core::ffi::c_int,
                offset: 4 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"rgb48le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 6 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 6 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 6 as ::core::ffi::c_int,
                offset: 4 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"rgb565be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: -(1 as ::core::ffi::c_int),
                shift: 3 as ::core::ffi::c_int,
                depth: 5 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 5 as ::core::ffi::c_int,
                depth: 6 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 5 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"rgb565le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 3 as ::core::ffi::c_int,
                depth: 5 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 5 as ::core::ffi::c_int,
                depth: 6 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 5 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"rgb555be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: -(1 as ::core::ffi::c_int),
                shift: 2 as ::core::ffi::c_int,
                depth: 5 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 5 as ::core::ffi::c_int,
                depth: 5 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 5 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"rgb555le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 2 as ::core::ffi::c_int,
                depth: 5 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 5 as ::core::ffi::c_int,
                depth: 5 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 5 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"bgr565be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 5 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 5 as ::core::ffi::c_int,
                depth: 6 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: -(1 as ::core::ffi::c_int),
                shift: 3 as ::core::ffi::c_int,
                depth: 5 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"bgr565le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 5 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 5 as ::core::ffi::c_int,
                depth: 6 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 3 as ::core::ffi::c_int,
                depth: 5 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"bgr555be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 5 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 5 as ::core::ffi::c_int,
                depth: 5 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: -(1 as ::core::ffi::c_int),
                shift: 2 as ::core::ffi::c_int,
                depth: 5 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"bgr555le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 5 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 5 as ::core::ffi::c_int,
                depth: 5 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 2 as ::core::ffi::c_int,
                depth: 5 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"vaapi\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 0,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint64_t,
        comp: [AVComponentDescriptor {
            plane: 0,
            step: 0,
            offset: 0,
            shift: 0,
            depth: 0,
        }; 4],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv420p16le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv420p16be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv422p16le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv422p16be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv444p16le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv444p16be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"dxva2_vld\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 0,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint64_t,
        comp: [AVComponentDescriptor {
            plane: 0,
            step: 0,
            offset: 0,
            shift: 0,
            depth: 0,
        }; 4],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"rgb444le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"rgb444be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: -(1 as ::core::ffi::c_int),
                shift: 0 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"bgr444le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"bgr444be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: -(1 as ::core::ffi::c_int),
                shift: 0 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"ya8\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 2 as uint8_t,
        log2_chroma_w: 0,
        log2_chroma_h: 0,
        flags: ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: b"gray8a\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVPixFmtDescriptor {
        name: b"bgr48be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 6 as ::core::ffi::c_int,
                offset: 4 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 6 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 6 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"bgr48le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 6 as ::core::ffi::c_int,
                offset: 4 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 6 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 6 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv420p9be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv420p9le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv420p10be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv420p10le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv422p10be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv422p10le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv444p9be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv444p9le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv444p10be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv444p10le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv422p9be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv422p9le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrp\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrp9be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrp9le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrp10be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrp10le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrp16be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrp16le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuva422p\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuva444p\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuva420p9be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuva420p9le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuva422p9be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuva422p9le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuva444p9be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuva444p9le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuva420p10be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuva420p10le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuva422p10be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuva422p10le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuva444p10be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuva444p10le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuva420p16be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuva420p16le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuva422p16be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuva422p16le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuva444p16be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuva444p16le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"vdpau\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 0,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint64_t,
        comp: [AVComponentDescriptor {
            plane: 0,
            step: 0,
            offset: 0,
            shift: 0,
            depth: 0,
        }; 4],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"xyz12le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 6 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 6 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 6 as ::core::ffi::c_int,
                offset: 4 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"xyz12be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 6 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 6 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 6 as ::core::ffi::c_int,
                offset: 4 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"nv16\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"nv20le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"nv20be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"rgba64be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 4 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 6 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"rgba64le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 4 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 6 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"bgra64be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 4 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 6 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"bgra64le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 4 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 6 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yvyu422\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: 0,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 3 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"ya16be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 2 as uint8_t,
        log2_chroma_w: 0,
        log2_chroma_h: 0,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"ya16le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 2 as uint8_t,
        log2_chroma_w: 0,
        log2_chroma_h: 0,
        flags: ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrap\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrap16be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrap16le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"qsv\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 0,
        log2_chroma_w: 0,
        log2_chroma_h: 0,
        flags: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint64_t,
        comp: [AVComponentDescriptor {
            plane: 0,
            step: 0,
            offset: 0,
            shift: 0,
            depth: 0,
        }; 4],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"mmal\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 0,
        log2_chroma_w: 0,
        log2_chroma_h: 0,
        flags: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint64_t,
        comp: [AVComponentDescriptor {
            plane: 0,
            step: 0,
            offset: 0,
            shift: 0,
            depth: 0,
        }; 4],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"d3d11va_vld\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 0,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint64_t,
        comp: [AVComponentDescriptor {
            plane: 0,
            step: 0,
            offset: 0,
            shift: 0,
            depth: 0,
        }; 4],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"cuda\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 0,
        log2_chroma_w: 0,
        log2_chroma_h: 0,
        flags: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint64_t,
        comp: [AVComponentDescriptor {
            plane: 0,
            step: 0,
            offset: 0,
            shift: 0,
            depth: 0,
        }; 4],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"0rgb\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 3 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"rgb0\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 3 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"0bgr\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 3 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"bgr0\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 3 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv420p12be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv420p12le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv420p14be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv420p14le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv422p12be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv422p12le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv422p14be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv422p14le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv444p12be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv444p12le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv444p14be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv444p14le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrp12be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrp12le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrp14be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrp14le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuvj411p\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 2 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"bayer_bggr8\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 2 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 2 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"bayer_rggb8\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 2 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 2 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"bayer_gbrg8\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 2 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 2 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"bayer_grbg8\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 2 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 2 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"bayer_bggr16le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"bayer_bggr16be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"bayer_rggb16le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"bayer_rggb16be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"bayer_gbrg16le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"bayer_gbrg16be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"bayer_grbg16le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"bayer_grbg16be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 4 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv440p10le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv440p10be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv440p12le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv440p12be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"ayuv64le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 4 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 6 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"ayuv64be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 4 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 6 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"videotoolbox_vld\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 0,
        log2_chroma_w: 0,
        log2_chroma_h: 0,
        flags: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint64_t,
        comp: [AVComponentDescriptor {
            plane: 0,
            step: 0,
            offset: 0,
            shift: 0,
            depth: 0,
        }; 4],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"p010le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"p010be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrap12be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrap12le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrap10be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrap10le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"mediacodec\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 0,
        log2_chroma_w: 0,
        log2_chroma_h: 0,
        flags: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint64_t,
        comp: [AVComponentDescriptor {
            plane: 0,
            step: 0,
            offset: 0,
            shift: 0,
            depth: 0,
        }; 4],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gray12be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 1 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: b"y12be\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVPixFmtDescriptor {
        name: b"gray12le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 1 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: 0,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: b"y12le\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVPixFmtDescriptor {
        name: b"gray10be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 1 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: b"y10be\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVPixFmtDescriptor {
        name: b"gray10le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 1 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: 0,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: b"y10le\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVPixFmtDescriptor {
        name: b"p016le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"p016be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"d3d11\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 0,
        log2_chroma_w: 0,
        log2_chroma_h: 0,
        flags: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint64_t,
        comp: [AVComponentDescriptor {
            plane: 0,
            step: 0,
            offset: 0,
            shift: 0,
            depth: 0,
        }; 4],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gray9be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 1 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: b"y9be\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVPixFmtDescriptor {
        name: b"gray9le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 1 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: 0,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 9 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: b"y9le\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVPixFmtDescriptor {
        name: b"gbrpf32be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrpf32le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrapf32be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrapf32le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"drm_prime\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 0,
        log2_chroma_w: 0,
        log2_chroma_h: 0,
        flags: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint64_t,
        comp: [AVComponentDescriptor {
            plane: 0,
            step: 0,
            offset: 0,
            shift: 0,
            depth: 0,
        }; 4],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"opencl\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 0,
        log2_chroma_w: 0,
        log2_chroma_h: 0,
        flags: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint64_t,
        comp: [AVComponentDescriptor {
            plane: 0,
            step: 0,
            offset: 0,
            shift: 0,
            depth: 0,
        }; 4],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gray14be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 1 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: b"y14be\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVPixFmtDescriptor {
        name: b"gray14le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 1 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: 0,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: b"y14le\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVPixFmtDescriptor {
        name: b"grayf32be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 1 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: b"yf32be\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVPixFmtDescriptor {
        name: b"grayf32le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 1 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: b"yf32le\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVPixFmtDescriptor {
        name: b"yuva422p12be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuva422p12le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuva444p12be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuva444p12le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"nv24\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"nv42\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 1 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"vulkan\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 0,
        log2_chroma_w: 0,
        log2_chroma_h: 0,
        flags: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint64_t,
        comp: [AVComponentDescriptor {
            plane: 0,
            step: 0,
            offset: 0,
            shift: 0,
            depth: 0,
        }; 4],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"y210be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 6 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"y210le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: 0,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 6 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"x2rgb10le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 2 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 3 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 2 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"x2rgb10be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 2 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 3 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 2 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"x2bgr10le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 2 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 3 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 2 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"x2bgr10be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 2 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 3 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 2 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"p210be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"p210le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"p410be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"p410le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"p216be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"p216le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"p416be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"p416le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"vuya\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 3 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"rgbaf16be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 4 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 6 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"rgbaf16le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 4 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 6 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"vuyx\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: 0,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 3 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"p012le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"p012be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 1 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"y212be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 6 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"y212le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: 0,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 6 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"xv30be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 32 as ::core::ffi::c_int,
                offset: 10 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 32 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 32 as ::core::ffi::c_int,
                offset: 20 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 32 as ::core::ffi::c_int,
                offset: 30 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 2 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"xv30le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: 0,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 2 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 3 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 2 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"xv36be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 4 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 6 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"xv36le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: 0,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 4 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 6 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"rgbf32be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 12 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 12 as ::core::ffi::c_int,
                offset: 4 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 12 as ::core::ffi::c_int,
                offset: 8 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"rgbf32le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 12 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 12 as ::core::ffi::c_int,
                offset: 4 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 12 as ::core::ffi::c_int,
                offset: 8 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"rgbaf32be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 16 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 16 as ::core::ffi::c_int,
                offset: 4 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 16 as ::core::ffi::c_int,
                offset: 8 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 16 as ::core::ffi::c_int,
                offset: 12 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"rgbaf32le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 16 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 16 as ::core::ffi::c_int,
                offset: 4 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 16 as ::core::ffi::c_int,
                offset: 8 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 16 as ::core::ffi::c_int,
                offset: 12 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"p212be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"p212le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"p412be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"p412le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrap14be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrap14le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 14 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"d3d12\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 0,
        log2_chroma_w: 0,
        log2_chroma_h: 0,
        flags: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint64_t,
        comp: [AVComponentDescriptor {
            plane: 0,
            step: 0,
            offset: 0,
            shift: 0,
            depth: 0,
        }; 4],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"ayuv\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 3 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"uyva\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 3 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"vyu444\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: 0,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 3 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 3 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 3 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 8 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"v30xbe\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 32 as ::core::ffi::c_int,
                offset: 12 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 32 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 32 as ::core::ffi::c_int,
                offset: 22 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 32 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 2 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"v30xle\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: 0,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 1 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 2 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 2 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"rgbf16be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 6 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 6 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 6 as ::core::ffi::c_int,
                offset: 4 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"rgbf16le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 6 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 6 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 6 as ::core::ffi::c_int,
                offset: 4 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"rgba128be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 16 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 16 as ::core::ffi::c_int,
                offset: 4 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 16 as ::core::ffi::c_int,
                offset: 8 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 16 as ::core::ffi::c_int,
                offset: 12 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"rgba128le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 16 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 16 as ::core::ffi::c_int,
                offset: 4 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 16 as ::core::ffi::c_int,
                offset: 8 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 16 as ::core::ffi::c_int,
                offset: 12 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"rgb96be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 12 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 12 as ::core::ffi::c_int,
                offset: 4 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 12 as ::core::ffi::c_int,
                offset: 8 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"rgb96le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 12 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 12 as ::core::ffi::c_int,
                offset: 4 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 12 as ::core::ffi::c_int,
                offset: 8 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"y216be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 6 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"y216le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 1 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: 0,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 6 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"xv48be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 4 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 6 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"xv48le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: 0,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 4 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 6 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrpf16be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrpf16le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrapf16be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrapf16le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"grayf16be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 1 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"grayf16le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 1 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"amf\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 0,
        log2_chroma_w: 0,
        log2_chroma_h: 0,
        flags: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint64_t,
        comp: [AVComponentDescriptor {
            plane: 0,
            step: 0,
            offset: 0,
            shift: 0,
            depth: 0,
        }; 4],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gray32be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 1 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: b"y32be\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVPixFmtDescriptor {
        name: b"gray32le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 1 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: 0,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: b"y32le\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVPixFmtDescriptor {
        name: b"yaf32be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 2 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 4 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yaf32le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 2 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 8 as ::core::ffi::c_int,
                offset: 4 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yaf16be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 2 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yaf16le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 2 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 2 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 16 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrap32be\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrap32le\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 4 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 3 as ::core::ffi::c_int,
                step: 4 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 0 as ::core::ffi::c_int,
                depth: 32 as ::core::ffi::c_int,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv444p10msbbe\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv444p10msble\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv444p12msbbe\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"yuv444p12msble\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrp10msbbe\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrp10msble\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 6 as ::core::ffi::c_int,
                depth: 10 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrp12msbbe\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"gbrp12msble\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 3 as uint8_t,
        log2_chroma_w: 0 as uint8_t,
        log2_chroma_h: 0 as uint8_t,
        flags: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t,
        comp: [
            AVComponentDescriptor {
                plane: 2 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 1 as ::core::ffi::c_int,
                step: 2 as ::core::ffi::c_int,
                offset: 0 as ::core::ffi::c_int,
                shift: 4 as ::core::ffi::c_int,
                depth: 12 as ::core::ffi::c_int,
            },
            AVComponentDescriptor {
                plane: 0,
                step: 0,
                offset: 0,
                shift: 0,
                depth: 0,
            },
        ],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVPixFmtDescriptor {
        name: b"ohcodec\0" as *const u8 as *const ::core::ffi::c_char,
        nb_components: 0,
        log2_chroma_w: 0,
        log2_chroma_h: 0,
        flags: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint64_t,
        comp: [AVComponentDescriptor {
            plane: 0,
            step: 0,
            offset: 0,
            shift: 0,
            depth: 0,
        }; 4],
        alias: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
static mut color_range_names: [*const ::core::ffi::c_char; 3] = [
    b"unknown\0" as *const u8 as *const ::core::ffi::c_char,
    b"tv\0" as *const u8 as *const ::core::ffi::c_char,
    b"pc\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut color_primaries_names: [*const ::core::ffi::c_char; 23] = [
    b"reserved\0" as *const u8 as *const ::core::ffi::c_char,
    b"bt709\0" as *const u8 as *const ::core::ffi::c_char,
    b"unknown\0" as *const u8 as *const ::core::ffi::c_char,
    b"reserved\0" as *const u8 as *const ::core::ffi::c_char,
    b"bt470m\0" as *const u8 as *const ::core::ffi::c_char,
    b"bt470bg\0" as *const u8 as *const ::core::ffi::c_char,
    b"smpte170m\0" as *const u8 as *const ::core::ffi::c_char,
    b"smpte240m\0" as *const u8 as *const ::core::ffi::c_char,
    b"film\0" as *const u8 as *const ::core::ffi::c_char,
    b"bt2020\0" as *const u8 as *const ::core::ffi::c_char,
    b"smpte428\0" as *const u8 as *const ::core::ffi::c_char,
    b"smpte431\0" as *const u8 as *const ::core::ffi::c_char,
    b"smpte432\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
    ::core::ptr::null::<::core::ffi::c_char>(),
    ::core::ptr::null::<::core::ffi::c_char>(),
    ::core::ptr::null::<::core::ffi::c_char>(),
    ::core::ptr::null::<::core::ffi::c_char>(),
    ::core::ptr::null::<::core::ffi::c_char>(),
    ::core::ptr::null::<::core::ffi::c_char>(),
    ::core::ptr::null::<::core::ffi::c_char>(),
    ::core::ptr::null::<::core::ffi::c_char>(),
    b"ebu3213\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut color_primaries_names_ext: [*const ::core::ffi::c_char; 1] = [
    b"vgamut\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut color_transfer_names: [*const ::core::ffi::c_char; 19] = [
    b"reserved\0" as *const u8 as *const ::core::ffi::c_char,
    b"bt709\0" as *const u8 as *const ::core::ffi::c_char,
    b"unknown\0" as *const u8 as *const ::core::ffi::c_char,
    b"reserved\0" as *const u8 as *const ::core::ffi::c_char,
    b"bt470m\0" as *const u8 as *const ::core::ffi::c_char,
    b"bt470bg\0" as *const u8 as *const ::core::ffi::c_char,
    b"smpte170m\0" as *const u8 as *const ::core::ffi::c_char,
    b"smpte240m\0" as *const u8 as *const ::core::ffi::c_char,
    b"linear\0" as *const u8 as *const ::core::ffi::c_char,
    b"log100\0" as *const u8 as *const ::core::ffi::c_char,
    b"log316\0" as *const u8 as *const ::core::ffi::c_char,
    b"iec61966-2-4\0" as *const u8 as *const ::core::ffi::c_char,
    b"bt1361e\0" as *const u8 as *const ::core::ffi::c_char,
    b"iec61966-2-1\0" as *const u8 as *const ::core::ffi::c_char,
    b"bt2020-10\0" as *const u8 as *const ::core::ffi::c_char,
    b"bt2020-12\0" as *const u8 as *const ::core::ffi::c_char,
    b"smpte2084\0" as *const u8 as *const ::core::ffi::c_char,
    b"smpte428\0" as *const u8 as *const ::core::ffi::c_char,
    b"arib-std-b67\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut color_transfer_names_ext: [*const ::core::ffi::c_char; 1] = [
    b"vlog\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut color_space_names: [*const ::core::ffi::c_char; 18] = [
    b"gbr\0" as *const u8 as *const ::core::ffi::c_char,
    b"bt709\0" as *const u8 as *const ::core::ffi::c_char,
    b"unknown\0" as *const u8 as *const ::core::ffi::c_char,
    b"reserved\0" as *const u8 as *const ::core::ffi::c_char,
    b"fcc\0" as *const u8 as *const ::core::ffi::c_char,
    b"bt470bg\0" as *const u8 as *const ::core::ffi::c_char,
    b"smpte170m\0" as *const u8 as *const ::core::ffi::c_char,
    b"smpte240m\0" as *const u8 as *const ::core::ffi::c_char,
    b"ycgco\0" as *const u8 as *const ::core::ffi::c_char,
    b"bt2020nc\0" as *const u8 as *const ::core::ffi::c_char,
    b"bt2020c\0" as *const u8 as *const ::core::ffi::c_char,
    b"smpte2085\0" as *const u8 as *const ::core::ffi::c_char,
    b"chroma-derived-nc\0" as *const u8 as *const ::core::ffi::c_char,
    b"chroma-derived-c\0" as *const u8 as *const ::core::ffi::c_char,
    b"ictcp\0" as *const u8 as *const ::core::ffi::c_char,
    b"ipt-c2\0" as *const u8 as *const ::core::ffi::c_char,
    b"ycgco-re\0" as *const u8 as *const ::core::ffi::c_char,
    b"ycgco-ro\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut chroma_location_names: [*const ::core::ffi::c_char; 7] = [
    b"unspecified\0" as *const u8 as *const ::core::ffi::c_char,
    b"left\0" as *const u8 as *const ::core::ffi::c_char,
    b"center\0" as *const u8 as *const ::core::ffi::c_char,
    b"topleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"top\0" as *const u8 as *const ::core::ffi::c_char,
    b"bottomleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"bottom\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut alpha_mode_names: [*const ::core::ffi::c_char; 3] = [
    b"unspecified\0" as *const u8 as *const ::core::ffi::c_char,
    b"premultiplied\0" as *const u8 as *const ::core::ffi::c_char,
    b"straight\0" as *const u8 as *const ::core::ffi::c_char,
];
unsafe extern "C" fn get_pix_fmt_internal(
    mut name: *const ::core::ffi::c_char,
) -> AVPixelFormat {
    let mut pix_fmt: AVPixelFormat = AV_PIX_FMT_YUV420P;
    pix_fmt = AV_PIX_FMT_YUV420P;
    while (pix_fmt as ::core::ffi::c_int) < AV_PIX_FMT_NB as ::core::ffi::c_int {
        if !av_pix_fmt_descriptors[pix_fmt as usize].name.is_null()
            && (strcmp(av_pix_fmt_descriptors[pix_fmt as usize].name, name) == 0
                || av_match_name(name, av_pix_fmt_descriptors[pix_fmt as usize].alias)
                    != 0)
        {
            return pix_fmt;
        }
        pix_fmt += 1;
    }
    return AV_PIX_FMT_NONE;
}
#[no_mangle]
pub unsafe extern "C" fn av_get_pix_fmt_name(
    mut pix_fmt: AVPixelFormat,
) -> *const ::core::ffi::c_char {
    return if (pix_fmt as ::core::ffi::c_uint)
        < AV_PIX_FMT_NB as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        av_pix_fmt_descriptors[pix_fmt as usize].name
    } else {
        ::core::ptr::null::<::core::ffi::c_char>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_get_pix_fmt(
    mut name: *const ::core::ffi::c_char,
) -> AVPixelFormat {
    let mut pix_fmt: AVPixelFormat = AV_PIX_FMT_YUV420P;
    if strcmp(name, b"rgb32\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        name = b"bgra\0" as *const u8 as *const ::core::ffi::c_char;
    } else if strcmp(name, b"bgr32\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        name = b"rgba\0" as *const u8 as *const ::core::ffi::c_char;
    }
    pix_fmt = get_pix_fmt_internal(name);
    if pix_fmt as ::core::ffi::c_int == AV_PIX_FMT_NONE as ::core::ffi::c_int {
        let mut name2: [::core::ffi::c_char; 32] = [0; 32];
        snprintf(
            &raw mut name2 as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>() as size_t,
            b"%s%s\0" as *const u8 as *const ::core::ffi::c_char,
            name,
            b"le\0" as *const u8 as *const ::core::ffi::c_char,
        );
        pix_fmt = get_pix_fmt_internal(&raw mut name2 as *mut ::core::ffi::c_char);
    }
    return pix_fmt;
}
#[no_mangle]
pub unsafe extern "C" fn av_get_bits_per_pixel(
    mut pixdesc: *const AVPixFmtDescriptor,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0;
    let mut bits: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut log2_pixels: ::core::ffi::c_int = (*pixdesc).log2_chroma_w
        as ::core::ffi::c_int + (*pixdesc).log2_chroma_h as ::core::ffi::c_int;
    c = 0 as ::core::ffi::c_int;
    while c < (*pixdesc).nb_components as ::core::ffi::c_int {
        let mut s: ::core::ffi::c_int = if c == 1 as ::core::ffi::c_int
            || c == 2 as ::core::ffi::c_int
        {
            0 as ::core::ffi::c_int
        } else {
            log2_pixels
        };
        bits += (*pixdesc).comp[c as usize].depth << s;
        c += 1;
    }
    return bits >> log2_pixels;
}
#[no_mangle]
pub unsafe extern "C" fn av_get_padded_bits_per_pixel(
    mut pixdesc: *const AVPixFmtDescriptor,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0;
    let mut bits: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut log2_pixels: ::core::ffi::c_int = (*pixdesc).log2_chroma_w
        as ::core::ffi::c_int + (*pixdesc).log2_chroma_h as ::core::ffi::c_int;
    let mut steps: [::core::ffi::c_int; 4] = [0 as ::core::ffi::c_int; 4];
    c = 0 as ::core::ffi::c_int;
    while c < (*pixdesc).nb_components as ::core::ffi::c_int {
        let mut comp: *const AVComponentDescriptor = (&raw const (*pixdesc).comp
            as *const AVComponentDescriptor)
            .offset(c as isize) as *const AVComponentDescriptor;
        let mut s: ::core::ffi::c_int = if c == 1 as ::core::ffi::c_int
            || c == 2 as ::core::ffi::c_int
        {
            0 as ::core::ffi::c_int
        } else {
            log2_pixels
        };
        steps[(*comp).plane as usize] = (*comp).step << s;
        c += 1;
    }
    c = 0 as ::core::ffi::c_int;
    while c < 4 as ::core::ffi::c_int {
        bits += steps[c as usize];
        c += 1;
    }
    if (*pixdesc).flags
        & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as uint64_t == 0
    {
        bits *= 8 as ::core::ffi::c_int;
    }
    return bits >> log2_pixels;
}
#[no_mangle]
pub unsafe extern "C" fn av_get_pix_fmt_string(
    mut buf: *mut ::core::ffi::c_char,
    mut buf_size: ::core::ffi::c_int,
    mut pix_fmt: AVPixelFormat,
) -> *mut ::core::ffi::c_char {
    if (pix_fmt as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        snprintf(
            buf,
            buf_size as size_t,
            b"name nb_components nb_bits\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        let mut pixdesc: *const AVPixFmtDescriptor = (&raw const av_pix_fmt_descriptors
            as *const AVPixFmtDescriptor)
            .offset(pix_fmt as isize) as *const AVPixFmtDescriptor;
        snprintf(
            buf,
            buf_size as size_t,
            b"%-11s %7d %10d\0" as *const u8 as *const ::core::ffi::c_char,
            (*pixdesc).name,
            (*pixdesc).nb_components as ::core::ffi::c_int,
            av_get_bits_per_pixel(pixdesc),
        );
    }
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn av_pix_fmt_desc_get(
    mut pix_fmt: AVPixelFormat,
) -> *const AVPixFmtDescriptor {
    if (pix_fmt as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        || pix_fmt as ::core::ffi::c_int >= AV_PIX_FMT_NB as ::core::ffi::c_int
    {
        return ::core::ptr::null::<AVPixFmtDescriptor>();
    }
    return (&raw const av_pix_fmt_descriptors as *const AVPixFmtDescriptor)
        .offset(pix_fmt as isize) as *const AVPixFmtDescriptor;
}
#[no_mangle]
pub unsafe extern "C" fn av_pix_fmt_desc_next(
    mut prev: *const AVPixFmtDescriptor,
) -> *const AVPixFmtDescriptor {
    if prev.is_null() {
        return (&raw const av_pix_fmt_descriptors as *const AVPixFmtDescriptor)
            .offset(0 as ::core::ffi::c_int as isize) as *const AVPixFmtDescriptor;
    }
    while (prev
        .offset_from(&raw const av_pix_fmt_descriptors as *const AVPixFmtDescriptor)
        as ::core::ffi::c_int as usize)
        < (::core::mem::size_of::<[AVPixFmtDescriptor; 267]>() as usize)
            .wrapping_div(::core::mem::size_of::<AVPixFmtDescriptor>() as usize)
            .wrapping_sub(1 as usize)
    {
        prev = prev.offset(1);
        if !(*prev).name.is_null() {
            return prev;
        }
    }
    return ::core::ptr::null::<AVPixFmtDescriptor>();
}
#[no_mangle]
pub unsafe extern "C" fn av_pix_fmt_desc_get_id(
    mut desc: *const AVPixFmtDescriptor,
) -> AVPixelFormat {
    if desc < &raw const av_pix_fmt_descriptors as *const AVPixFmtDescriptor
        || desc
            >= (&raw const av_pix_fmt_descriptors as *const AVPixFmtDescriptor)
                .offset(
                    (::core::mem::size_of::<[AVPixFmtDescriptor; 267]>() as usize)
                        .wrapping_div(
                            ::core::mem::size_of::<AVPixFmtDescriptor>() as usize,
                        ) as isize,
                )
    {
        return AV_PIX_FMT_NONE;
    }
    return desc
        .offset_from(&raw const av_pix_fmt_descriptors as *const AVPixFmtDescriptor)
        as ::core::ffi::c_int as AVPixelFormat;
}
#[no_mangle]
pub unsafe extern "C" fn av_pix_fmt_get_chroma_sub_sample(
    mut pix_fmt: AVPixelFormat,
    mut h_shift: *mut ::core::ffi::c_int,
    mut v_shift: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut desc: *const AVPixFmtDescriptor = av_pix_fmt_desc_get(pix_fmt);
    if desc.is_null() {
        return -(38 as ::core::ffi::c_int);
    }
    *h_shift = (*desc).log2_chroma_w as ::core::ffi::c_int;
    *v_shift = (*desc).log2_chroma_h as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_pix_fmt_count_planes(
    mut pix_fmt: AVPixelFormat,
) -> ::core::ffi::c_int {
    let mut desc: *const AVPixFmtDescriptor = av_pix_fmt_desc_get(pix_fmt);
    let mut i: ::core::ffi::c_int = 0;
    let mut planes: [::core::ffi::c_int; 4] = [0 as ::core::ffi::c_int; 4];
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if desc.is_null() {
        return -(22 as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*desc).nb_components as ::core::ffi::c_int {
        planes[(*desc).comp[i as usize].plane as usize] = 1 as ::core::ffi::c_int;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while (i as usize)
        < (::core::mem::size_of::<[::core::ffi::c_int; 4]>() as usize)
            .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>() as usize)
    {
        ret += planes[i as usize];
        i += 1;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_pix_fmt_swap_endianness(
    mut pix_fmt: AVPixelFormat,
) -> AVPixelFormat {
    let mut desc: *const AVPixFmtDescriptor = av_pix_fmt_desc_get(pix_fmt);
    let mut name: [::core::ffi::c_char; 16] = [0; 16];
    let mut i: ::core::ffi::c_int = 0;
    if desc.is_null() || strlen((*desc).name) < 2 as size_t {
        return AV_PIX_FMT_NONE;
    }
    av_strlcpy(
        &raw mut name as *mut ::core::ffi::c_char,
        (*desc).name,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as size_t,
    );
    i = strlen(&raw mut name as *mut ::core::ffi::c_char).wrapping_sub(2 as size_t)
        as ::core::ffi::c_int;
    if strcmp(
        (&raw mut name as *mut ::core::ffi::c_char).offset(i as isize),
        b"be\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
        && strcmp(
            (&raw mut name as *mut ::core::ffi::c_char).offset(i as isize),
            b"le\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
    {
        return AV_PIX_FMT_NONE;
    }
    name[i as usize] = (name[i as usize] as ::core::ffi::c_int
        ^ ('b' as i32 ^ 'l' as i32)) as ::core::ffi::c_char;
    return get_pix_fmt_internal(&raw mut name as *mut ::core::ffi::c_char);
}
unsafe extern "C" fn get_color_type(
    mut desc: *const AVPixFmtDescriptor,
) -> ::core::ffi::c_int {
    if (*desc).flags & ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint64_t
        != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*desc).nb_components as ::core::ffi::c_int == 1 as ::core::ffi::c_int
        || (*desc).nb_components as ::core::ffi::c_int == 2 as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    }
    if !(*desc).name.is_null() {
        if av_strstart(
            (*desc).name,
            b"yuvj\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        ) != 0
        {
            return 3 as ::core::ffi::c_int;
        }
    }
    if (*desc).flags & ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as uint64_t
        != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*desc).flags
        & ((1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int) as uint64_t != 0
    {
        return 4 as ::core::ffi::c_int;
    }
    if (*desc).nb_components as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    return 2 as ::core::ffi::c_int;
}
unsafe extern "C" fn get_pix_fmt_depth(
    mut min: *mut ::core::ffi::c_int,
    mut max: *mut ::core::ffi::c_int,
    mut pix_fmt: AVPixelFormat,
) -> ::core::ffi::c_int {
    let mut desc: *const AVPixFmtDescriptor = av_pix_fmt_desc_get(pix_fmt);
    let mut i: ::core::ffi::c_int = 0;
    if desc.is_null() || (*desc).nb_components == 0 {
        *max = 0 as ::core::ffi::c_int;
        *min = *max;
        return -(22 as ::core::ffi::c_int);
    }
    *min = 2147483647 as ::core::ffi::c_int;
    *max = -(2147483647 as ::core::ffi::c_int);
    i = 0 as ::core::ffi::c_int;
    while i < (*desc).nb_components as ::core::ffi::c_int {
        *min = if (*desc).comp[i as usize].depth > *min {
            *min
        } else {
            (*desc).comp[i as usize].depth
        };
        *max = if (*desc).comp[i as usize].depth > *max {
            (*desc).comp[i as usize].depth
        } else {
            *max
        };
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn get_pix_fmt_score(
    mut dst_pix_fmt: AVPixelFormat,
    mut src_pix_fmt: AVPixelFormat,
    mut lossp: *mut ::core::ffi::c_uint,
    mut consider: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut src_desc: *const AVPixFmtDescriptor = av_pix_fmt_desc_get(src_pix_fmt);
    let mut dst_desc: *const AVPixFmtDescriptor = av_pix_fmt_desc_get(dst_pix_fmt);
    let mut src_color: ::core::ffi::c_int = 0;
    let mut dst_color: ::core::ffi::c_int = 0;
    let mut src_min_depth: ::core::ffi::c_int = 0;
    let mut src_max_depth: ::core::ffi::c_int = 0;
    let mut dst_min_depth: ::core::ffi::c_int = 0;
    let mut dst_max_depth: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut loss: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut nb_components: ::core::ffi::c_int = 0;
    let mut score: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int
        - 1 as ::core::ffi::c_int;
    if src_desc.is_null() || dst_desc.is_null() {
        return -(4 as ::core::ffi::c_int);
    }
    if (*src_desc).flags
        & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint64_t != 0
        || (*dst_desc).flags
            & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint64_t != 0
    {
        if dst_pix_fmt as ::core::ffi::c_int == src_pix_fmt as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int)
        } else {
            return -(2 as ::core::ffi::c_int)
        }
    }
    loss = 0 as ::core::ffi::c_int;
    *lossp = loss as ::core::ffi::c_uint;
    if dst_pix_fmt as ::core::ffi::c_int == src_pix_fmt as ::core::ffi::c_int {
        return 2147483647 as ::core::ffi::c_int;
    }
    ret = get_pix_fmt_depth(&raw mut src_min_depth, &raw mut src_max_depth, src_pix_fmt);
    if ret < 0 as ::core::ffi::c_int {
        return -(3 as ::core::ffi::c_int);
    }
    ret = get_pix_fmt_depth(&raw mut dst_min_depth, &raw mut dst_max_depth, dst_pix_fmt);
    if ret < 0 as ::core::ffi::c_int {
        return -(3 as ::core::ffi::c_int);
    }
    src_color = get_color_type(src_desc);
    dst_color = get_color_type(dst_desc);
    if dst_pix_fmt as ::core::ffi::c_int == AV_PIX_FMT_PAL8 as ::core::ffi::c_int {
        nb_components = if (*src_desc).nb_components as ::core::ffi::c_int
            > 4 as ::core::ffi::c_int
        {
            4 as ::core::ffi::c_int
        } else {
            (*src_desc).nb_components as ::core::ffi::c_int
        };
    } else {
        nb_components = if (*src_desc).nb_components as ::core::ffi::c_int
            > (*dst_desc).nb_components as ::core::ffi::c_int
        {
            (*dst_desc).nb_components as ::core::ffi::c_int
        } else {
            (*src_desc).nb_components as ::core::ffi::c_int
        };
    }
    i = 0 as ::core::ffi::c_int;
    while i < nb_components {
        let mut depth_minus1: ::core::ffi::c_int = if dst_pix_fmt as ::core::ffi::c_int
            == AV_PIX_FMT_PAL8 as ::core::ffi::c_int
        {
            7 as ::core::ffi::c_int / nb_components
        } else {
            (*dst_desc).comp[i as usize].depth - 1 as ::core::ffi::c_int
        };
        let mut depth_delta: ::core::ffi::c_int = (*src_desc).comp[i as usize].depth
            - 1 as ::core::ffi::c_int - depth_minus1;
        if depth_delta > 0 as ::core::ffi::c_int
            && consider & 0x2 as ::core::ffi::c_uint != 0
        {
            loss |= 0x2 as ::core::ffi::c_int;
            score -= 65536 as ::core::ffi::c_int >> depth_minus1;
        } else if depth_delta < 0 as ::core::ffi::c_int
            && consider & 0x80 as ::core::ffi::c_uint != 0
        {
            loss |= 0x80 as ::core::ffi::c_int;
            score += depth_delta;
        }
        i += 1;
    }
    if consider & 0x1 as ::core::ffi::c_uint != 0 {
        if (*dst_desc).log2_chroma_w as ::core::ffi::c_int
            > (*src_desc).log2_chroma_w as ::core::ffi::c_int
        {
            loss |= 0x1 as ::core::ffi::c_int;
            score
                -= (256 as ::core::ffi::c_int)
                    << (*dst_desc).log2_chroma_w as ::core::ffi::c_int;
        }
        if (*dst_desc).log2_chroma_h as ::core::ffi::c_int
            > (*src_desc).log2_chroma_h as ::core::ffi::c_int
        {
            loss |= 0x1 as ::core::ffi::c_int;
            score
                -= (256 as ::core::ffi::c_int)
                    << (*dst_desc).log2_chroma_h as ::core::ffi::c_int;
        }
        if (*dst_desc).log2_chroma_w as ::core::ffi::c_int == 1 as ::core::ffi::c_int
            && (*src_desc).log2_chroma_w as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && (*dst_desc).log2_chroma_h as ::core::ffi::c_int == 1 as ::core::ffi::c_int
            && (*src_desc).log2_chroma_h as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            score += 512 as ::core::ffi::c_int;
        }
    }
    if consider & 0x40 as ::core::ffi::c_uint != 0 {
        if ((*dst_desc).log2_chroma_w as ::core::ffi::c_int)
            < (*src_desc).log2_chroma_w as ::core::ffi::c_int
        {
            loss |= 0x40 as ::core::ffi::c_int;
            score
                -= (1 as ::core::ffi::c_int)
                    << (*src_desc).log2_chroma_w as ::core::ffi::c_int
                        - (*dst_desc).log2_chroma_w as ::core::ffi::c_int;
        }
        if ((*dst_desc).log2_chroma_h as ::core::ffi::c_int)
            < (*src_desc).log2_chroma_h as ::core::ffi::c_int
        {
            loss |= 0x40 as ::core::ffi::c_int;
            score
                -= (1 as ::core::ffi::c_int)
                    << (*src_desc).log2_chroma_h as ::core::ffi::c_int
                        - (*dst_desc).log2_chroma_h as ::core::ffi::c_int;
        }
        if (*dst_desc).log2_chroma_w as ::core::ffi::c_int == 1 as ::core::ffi::c_int
            && (*src_desc).log2_chroma_w as ::core::ffi::c_int == 2 as ::core::ffi::c_int
            && (*dst_desc).log2_chroma_h as ::core::ffi::c_int == 1 as ::core::ffi::c_int
            && (*src_desc).log2_chroma_h as ::core::ffi::c_int == 2 as ::core::ffi::c_int
        {
            score += 4 as ::core::ffi::c_int;
        }
    }
    if consider & 0x4 as ::core::ffi::c_uint != 0 {
        match dst_color {
            0 => {
                if src_color != 0 as ::core::ffi::c_int
                    && src_color != 1 as ::core::ffi::c_int
                {
                    loss |= 0x4 as ::core::ffi::c_int;
                }
            }
            1 => {
                if src_color != 1 as ::core::ffi::c_int {
                    loss |= 0x4 as ::core::ffi::c_int;
                }
            }
            2 => {
                if src_color != 2 as ::core::ffi::c_int {
                    loss |= 0x4 as ::core::ffi::c_int;
                }
            }
            3 => {
                if src_color != 3 as ::core::ffi::c_int
                    && src_color != 2 as ::core::ffi::c_int
                    && src_color != 1 as ::core::ffi::c_int
                {
                    loss |= 0x4 as ::core::ffi::c_int;
                }
            }
            _ => {
                if src_color != dst_color {
                    loss |= 0x4 as ::core::ffi::c_int;
                }
            }
        }
    }
    if loss & 0x4 as ::core::ffi::c_int != 0 {
        score
            -= nb_components * 65536 as ::core::ffi::c_int
                >> (if (*dst_desc).comp[0 as ::core::ffi::c_int as usize].depth
                    - 1 as ::core::ffi::c_int
                    > (*src_desc).comp[0 as ::core::ffi::c_int as usize].depth
                        - 1 as ::core::ffi::c_int
                {
                    (*src_desc).comp[0 as ::core::ffi::c_int as usize].depth
                        - 1 as ::core::ffi::c_int
                } else {
                    (*dst_desc).comp[0 as ::core::ffi::c_int as usize].depth
                        - 1 as ::core::ffi::c_int
                });
    }
    if dst_color == 1 as ::core::ffi::c_int && src_color != 1 as ::core::ffi::c_int
        && consider & 0x20 as ::core::ffi::c_uint != 0
    {
        loss |= 0x20 as ::core::ffi::c_int;
        score -= 2 as ::core::ffi::c_int * 65536 as ::core::ffi::c_int;
    }
    if (*dst_desc).flags
        & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t == 0
        && ((*src_desc).flags
            & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t != 0
            && consider & 0x8 as ::core::ffi::c_uint != 0)
    {
        loss |= 0x8 as ::core::ffi::c_int;
        score -= 65536 as ::core::ffi::c_int;
    }
    if dst_pix_fmt as ::core::ffi::c_int == AV_PIX_FMT_PAL8 as ::core::ffi::c_int
        && consider & 0x10 as ::core::ffi::c_uint != 0
        && (src_pix_fmt as ::core::ffi::c_int != AV_PIX_FMT_PAL8 as ::core::ffi::c_int
            && (src_color != 1 as ::core::ffi::c_int
                || (*src_desc).flags
                    & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint64_t
                    != 0 && consider & 0x8 as ::core::ffi::c_uint != 0))
    {
        loss |= 0x10 as ::core::ffi::c_int;
        score -= 65536 as ::core::ffi::c_int;
    }
    *lossp = loss as ::core::ffi::c_uint;
    return score;
}
#[no_mangle]
pub unsafe extern "C" fn av_get_pix_fmt_loss(
    mut dst_pix_fmt: AVPixelFormat,
    mut src_pix_fmt: AVPixelFormat,
    mut has_alpha: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut loss: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = get_pix_fmt_score(
        dst_pix_fmt,
        src_pix_fmt,
        &raw mut loss as *mut ::core::ffi::c_uint,
        (if has_alpha != 0 {
            !(0 as ::core::ffi::c_int)
        } else {
            !(0x8 as ::core::ffi::c_int)
        }) as ::core::ffi::c_uint,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    return loss;
}
#[no_mangle]
pub unsafe extern "C" fn av_find_best_pix_fmt_of_2(
    mut dst_pix_fmt1: AVPixelFormat,
    mut dst_pix_fmt2: AVPixelFormat,
    mut src_pix_fmt: AVPixelFormat,
    mut has_alpha: ::core::ffi::c_int,
    mut loss_ptr: *mut ::core::ffi::c_int,
) -> AVPixelFormat {
    let mut dst_pix_fmt: AVPixelFormat = AV_PIX_FMT_YUV420P;
    let mut loss1: ::core::ffi::c_int = 0;
    let mut loss2: ::core::ffi::c_int = 0;
    let mut loss_mask: ::core::ffi::c_int = 0;
    let mut desc1: *const AVPixFmtDescriptor = av_pix_fmt_desc_get(dst_pix_fmt1);
    let mut desc2: *const AVPixFmtDescriptor = av_pix_fmt_desc_get(dst_pix_fmt2);
    let mut score1: ::core::ffi::c_int = 0;
    let mut score2: ::core::ffi::c_int = 0;
    if desc1.is_null() {
        dst_pix_fmt = dst_pix_fmt2;
    } else if desc2.is_null() {
        dst_pix_fmt = dst_pix_fmt1;
    } else {
        loss_mask = if !loss_ptr.is_null() {
            !*loss_ptr
        } else {
            !(0 as ::core::ffi::c_int)
        };
        if has_alpha == 0 {
            loss_mask &= !(0x8 as ::core::ffi::c_int);
        }
        score1 = get_pix_fmt_score(
            dst_pix_fmt1,
            src_pix_fmt,
            &raw mut loss1 as *mut ::core::ffi::c_uint,
            loss_mask as ::core::ffi::c_uint,
        );
        score2 = get_pix_fmt_score(
            dst_pix_fmt2,
            src_pix_fmt,
            &raw mut loss2 as *mut ::core::ffi::c_uint,
            loss_mask as ::core::ffi::c_uint,
        );
        if score1 == score2 {
            if av_get_padded_bits_per_pixel(desc2) != av_get_padded_bits_per_pixel(desc1)
            {
                dst_pix_fmt = (if av_get_padded_bits_per_pixel(desc2)
                    < av_get_padded_bits_per_pixel(desc1)
                {
                    dst_pix_fmt2 as ::core::ffi::c_int
                } else {
                    dst_pix_fmt1 as ::core::ffi::c_int
                }) as AVPixelFormat;
            } else {
                dst_pix_fmt = (if ((*desc2).nb_components as ::core::ffi::c_int)
                    < (*desc1).nb_components as ::core::ffi::c_int
                {
                    dst_pix_fmt2 as ::core::ffi::c_int
                } else {
                    dst_pix_fmt1 as ::core::ffi::c_int
                }) as AVPixelFormat;
            }
        } else {
            dst_pix_fmt = (if score1 < score2 {
                dst_pix_fmt2 as ::core::ffi::c_int
            } else {
                dst_pix_fmt1 as ::core::ffi::c_int
            }) as AVPixelFormat;
        }
    }
    if !loss_ptr.is_null() {
        *loss_ptr = av_get_pix_fmt_loss(dst_pix_fmt, src_pix_fmt, has_alpha);
    }
    return dst_pix_fmt;
}
#[no_mangle]
pub unsafe extern "C" fn av_color_range_name(
    mut range: AVColorRange,
) -> *const ::core::ffi::c_char {
    return if (range as ::core::ffi::c_uint)
        < AVCOL_RANGE_NB as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        color_range_names[range as usize]
    } else {
        ::core::ptr::null::<::core::ffi::c_char>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_color_range_from_name(
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while (i as usize)
        < (::core::mem::size_of::<[*const ::core::ffi::c_char; 3]>() as usize)
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
    {
        if strcmp(name, color_range_names[i as usize]) == 0 {
            return i;
        }
        i += 1;
    }
    return -(22 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn av_color_primaries_name(
    mut primaries: AVColorPrimaries,
) -> *const ::core::ffi::c_char {
    if (primaries as ::core::ffi::c_uint)
        < AVCOL_PRI_NB as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return color_primaries_names[primaries as usize]
    } else if primaries as ::core::ffi::c_uint
        >= AVCOL_PRI_EXT_BASE as ::core::ffi::c_int as ::core::ffi::c_uint
        && (primaries as ::core::ffi::c_uint)
            < AVCOL_PRI_EXT_NB as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return color_primaries_names_ext[(primaries as ::core::ffi::c_uint)
            .wrapping_sub(
                AVCOL_TRC_EXT_BASE as ::core::ffi::c_int as ::core::ffi::c_uint,
            ) as usize]
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn av_color_primaries_from_name(
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while (i as usize)
        < (::core::mem::size_of::<[*const ::core::ffi::c_char; 23]>() as usize)
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
    {
        if !color_primaries_names[i as usize].is_null() {
            if strcmp(name, color_primaries_names[i as usize]) == 0 {
                return i;
            }
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while (i as usize)
        < (::core::mem::size_of::<[*const ::core::ffi::c_char; 1]>() as usize)
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
    {
        if !color_primaries_names_ext[i as usize].is_null() {
            if strcmp(name, color_primaries_names_ext[i as usize]) == 0 {
                return AVCOL_PRI_EXT_BASE as ::core::ffi::c_int + i;
            }
        }
        i += 1;
    }
    return -(22 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn av_color_transfer_name(
    mut transfer: AVColorTransferCharacteristic,
) -> *const ::core::ffi::c_char {
    if (transfer as ::core::ffi::c_uint)
        < AVCOL_TRC_NB as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return color_transfer_names[transfer as usize]
    } else if transfer as ::core::ffi::c_uint
        >= AVCOL_TRC_EXT_BASE as ::core::ffi::c_int as ::core::ffi::c_uint
        && (transfer as ::core::ffi::c_uint)
            < AVCOL_TRC_EXT_NB as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return color_transfer_names_ext[(transfer as ::core::ffi::c_uint)
            .wrapping_sub(
                AVCOL_TRC_EXT_BASE as ::core::ffi::c_int as ::core::ffi::c_uint,
            ) as usize]
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn av_color_transfer_from_name(
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while (i as usize)
        < (::core::mem::size_of::<[*const ::core::ffi::c_char; 19]>() as usize)
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
    {
        if !color_transfer_names[i as usize].is_null() {
            if strcmp(name, color_transfer_names[i as usize]) == 0 {
                return i;
            }
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while (i as usize)
        < (::core::mem::size_of::<[*const ::core::ffi::c_char; 1]>() as usize)
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
    {
        if !color_transfer_names_ext[i as usize].is_null() {
            if strcmp(name, color_transfer_names_ext[i as usize]) == 0 {
                return AVCOL_TRC_EXT_BASE as ::core::ffi::c_int + i;
            }
        }
        i += 1;
    }
    return -(22 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn av_color_space_name(
    mut space: AVColorSpace,
) -> *const ::core::ffi::c_char {
    return if (space as ::core::ffi::c_uint)
        < AVCOL_SPC_NB as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        color_space_names[space as usize]
    } else {
        ::core::ptr::null::<::core::ffi::c_char>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_color_space_from_name(
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while (i as usize)
        < (::core::mem::size_of::<[*const ::core::ffi::c_char; 18]>() as usize)
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
    {
        if !color_space_names[i as usize].is_null() {
            if strcmp(name, color_space_names[i as usize]) == 0 {
                return i;
            }
        }
        i += 1;
    }
    return -(22 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn av_chroma_location_name(
    mut location: AVChromaLocation,
) -> *const ::core::ffi::c_char {
    return if (location as ::core::ffi::c_uint)
        < AVCHROMA_LOC_NB as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        chroma_location_names[location as usize]
    } else {
        ::core::ptr::null::<::core::ffi::c_char>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_chroma_location_from_name(
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while (i as usize)
        < (::core::mem::size_of::<[*const ::core::ffi::c_char; 7]>() as usize)
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
    {
        if !chroma_location_names[i as usize].is_null() {
            if strcmp(name, chroma_location_names[i as usize]) == 0 {
                return i;
            }
        }
        i += 1;
    }
    return -(22 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn av_chroma_location_enum_to_pos(
    mut xpos: *mut ::core::ffi::c_int,
    mut ypos: *mut ::core::ffi::c_int,
    mut pos: AVChromaLocation,
) -> ::core::ffi::c_int {
    if pos as ::core::ffi::c_uint
        <= AVCHROMA_LOC_UNSPECIFIED as ::core::ffi::c_int as ::core::ffi::c_uint
        || pos as ::core::ffi::c_uint
            >= AVCHROMA_LOC_NB as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return -(22 as ::core::ffi::c_int);
    }
    pos -= 1;
    *xpos = (pos as ::core::ffi::c_uint & 1 as ::core::ffi::c_uint)
        .wrapping_mul(128 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    *ypos = (pos as ::core::ffi::c_uint >> 1 as ::core::ffi::c_int
        ^ ((pos as ::core::ffi::c_uint) < 4 as ::core::ffi::c_uint) as ::core::ffi::c_int
            as ::core::ffi::c_uint)
        .wrapping_mul(128 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_chroma_location_pos_to_enum(
    mut xpos: ::core::ffi::c_int,
    mut ypos: ::core::ffi::c_int,
) -> AVChromaLocation {
    let mut pos: ::core::ffi::c_int = 0;
    let mut xout: ::core::ffi::c_int = 0;
    let mut yout: ::core::ffi::c_int = 0;
    pos = AVCHROMA_LOC_UNSPECIFIED as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    while pos < AVCHROMA_LOC_NB as ::core::ffi::c_int {
        if av_chroma_location_enum_to_pos(
            &raw mut xout,
            &raw mut yout,
            pos as AVChromaLocation,
        ) == 0 as ::core::ffi::c_int && xout == xpos && yout == ypos
        {
            return pos as AVChromaLocation;
        }
        pos += 1;
    }
    return AVCHROMA_LOC_UNSPECIFIED;
}
#[no_mangle]
pub unsafe extern "C" fn av_alpha_mode_name(
    mut mode: AVAlphaMode,
) -> *const ::core::ffi::c_char {
    return if (mode as ::core::ffi::c_uint)
        < AVALPHA_MODE_NB as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        alpha_mode_names[mode as usize]
    } else {
        ::core::ptr::null::<::core::ffi::c_char>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_alpha_mode_from_name(
    mut name: *const ::core::ffi::c_char,
) -> AVAlphaMode {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (i as usize)
        < (::core::mem::size_of::<[*const ::core::ffi::c_char; 3]>() as usize)
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
    {
        if !alpha_mode_names[i as usize].is_null() {
            if strcmp(name, alpha_mode_names[i as usize]) == 0 {
                return i as AVAlphaMode;
            }
        }
        i += 1;
    }
    return 4294967274 as AVAlphaMode;
}
