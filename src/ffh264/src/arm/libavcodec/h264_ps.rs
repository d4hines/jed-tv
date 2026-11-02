// Specialized to 8-bit 4:2:0 by tools/ffh264/shrink.py.
// Translated from FFmpeg 9.0.1 libavcodec/h264_ps.c
// (preprocessed for armv7-unknown-linux-musleabihf) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// H.26L/H.264/AVC/JVT/14496-10/... parameter set decoding
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
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        _: *const ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> ::core::ffi::c_int;
    static ff_zigzag_direct: [uint8_t; 64];
    static ff_zigzag_scan: [uint8_t; 17];
    static ff_golomb_vlc_len: [uint8_t; 512];
    static ff_ue_golomb_vlc_code: [uint8_t; 512];
    static ff_se_golomb_vlc_code: [int8_t; 512];
    static ff_log2_tab: [uint8_t; 256];
    fn ff_h2645_decode_common_vui_params(
        gb: *mut GetBitContext,
        vui: *mut H2645VUI,
        logctx: *mut ::core::ffi::c_void,
    );
    static ff_h264_dequant4_coeff_init: [[uint8_t; 3]; 6];
    static ff_h264_dequant8_coeff_init_scan: [uint8_t; 16];
    static ff_h264_dequant8_coeff_init: [[uint8_t; 6]; 6];
    static ff_h264_quant_rem6: [uint8_t; 88];
    static ff_h264_quant_div6: [uint8_t; 88];
    static ff_h264_chroma_qp: [[uint8_t; 88]; 7];
    fn av_refstruct_alloc_ext_c(
        size: size_t,
        flags: ::core::ffi::c_uint,
        opaque: AVRefStructOpaque,
        free_cb: Option<
            unsafe extern "C" fn(AVRefStructOpaque, *mut ::core::ffi::c_void) -> (),
        >,
    ) -> *mut ::core::ffi::c_void;
    fn avpriv_report_missing_feature(
        avc: *mut ::core::ffi::c_void,
        msg: *const ::core::ffi::c_char,
        ...
    );
    fn avpriv_request_sample(
        avc: *mut ::core::ffi::c_void,
        msg: *const ::core::ffi::c_char,
        ...
    );
    fn av_refstruct_unref(objp: *mut ::core::ffi::c_void);
    fn av_refstruct_ref_c(obj: *const ::core::ffi::c_void) -> *const ::core::ffi::c_void;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn av_image_check_size(
        w: ::core::ffi::c_uint,
        h: ::core::ffi::c_uint,
        log_offset: ::core::ffi::c_int,
        log_ctx: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn av_log_once(
        avcl: *mut ::core::ffi::c_void,
        initial_level: ::core::ffi::c_int,
        subsequent_level: ::core::ffi::c_int,
        state: *mut ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
}
pub type int8_t = ::core::ffi::c_schar;
pub type int32_t = ::core::ffi::c_int;
pub type int64_t = ::core::ffi::c_longlong;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint16_t = ::core::ffi::c_ushort;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint64_t = ::core::ffi::c_ulonglong;
pub type size_t = ::core::ffi::c_uint;
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
pub struct AVRational {
    pub num: ::core::ffi::c_int,
    pub den: ::core::ffi::c_int,
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
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const H264_MAX_HEIGHT: C2RustUnnamed_0 = 16880;
pub const H264_MAX_WIDTH: C2RustUnnamed_0 = 16880;
pub const H264_MAX_MB_HEIGHT: C2RustUnnamed_0 = 1055;
pub const H264_MAX_MB_WIDTH: C2RustUnnamed_0 = 1055;
pub const H264_MAX_MB_PIC_SIZE: C2RustUnnamed_0 = 139264;
pub const H264_MAX_CPB_CNT: C2RustUnnamed_0 = 32;
pub const H264_MAX_SLICE_GROUPS: C2RustUnnamed_0 = 8;
pub const H264_MAX_MMCO_COUNT: C2RustUnnamed_0 = 67;
pub const H264_MAX_RPLM_COUNT: C2RustUnnamed_0 = 33;
pub const H264_MAX_REFS: C2RustUnnamed_0 = 32;
pub const H264_MAX_DPB_FRAMES: C2RustUnnamed_0 = 16;
pub const H264_MAX_PPS_COUNT: C2RustUnnamed_0 = 256;
pub const H264_MAX_SPS_COUNT: C2RustUnnamed_0 = 32;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union AVRefStructOpaque {
    pub nc: *mut ::core::ffi::c_void,
    pub c: *const ::core::ffi::c_void,
}
static mut default_scaling4: [[uint8_t; 16]; 2] = [
    [
        6 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        20 as ::core::ffi::c_int as uint8_t,
        28 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        20 as ::core::ffi::c_int as uint8_t,
        28 as ::core::ffi::c_int as uint8_t,
        32 as ::core::ffi::c_int as uint8_t,
        20 as ::core::ffi::c_int as uint8_t,
        28 as ::core::ffi::c_int as uint8_t,
        32 as ::core::ffi::c_int as uint8_t,
        37 as ::core::ffi::c_int as uint8_t,
        28 as ::core::ffi::c_int as uint8_t,
        32 as ::core::ffi::c_int as uint8_t,
        37 as ::core::ffi::c_int as uint8_t,
        42 as ::core::ffi::c_int as uint8_t,
    ],
    [
        10 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
        20 as ::core::ffi::c_int as uint8_t,
        24 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
        20 as ::core::ffi::c_int as uint8_t,
        24 as ::core::ffi::c_int as uint8_t,
        27 as ::core::ffi::c_int as uint8_t,
        20 as ::core::ffi::c_int as uint8_t,
        24 as ::core::ffi::c_int as uint8_t,
        27 as ::core::ffi::c_int as uint8_t,
        30 as ::core::ffi::c_int as uint8_t,
        24 as ::core::ffi::c_int as uint8_t,
        27 as ::core::ffi::c_int as uint8_t,
        30 as ::core::ffi::c_int as uint8_t,
        34 as ::core::ffi::c_int as uint8_t,
    ],
];
static mut default_scaling8: [[uint8_t; 64]; 2] = [
    [
        6 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        16 as ::core::ffi::c_int as uint8_t,
        18 as ::core::ffi::c_int as uint8_t,
        23 as ::core::ffi::c_int as uint8_t,
        25 as ::core::ffi::c_int as uint8_t,
        27 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        16 as ::core::ffi::c_int as uint8_t,
        18 as ::core::ffi::c_int as uint8_t,
        23 as ::core::ffi::c_int as uint8_t,
        25 as ::core::ffi::c_int as uint8_t,
        27 as ::core::ffi::c_int as uint8_t,
        29 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        16 as ::core::ffi::c_int as uint8_t,
        18 as ::core::ffi::c_int as uint8_t,
        23 as ::core::ffi::c_int as uint8_t,
        25 as ::core::ffi::c_int as uint8_t,
        27 as ::core::ffi::c_int as uint8_t,
        29 as ::core::ffi::c_int as uint8_t,
        31 as ::core::ffi::c_int as uint8_t,
        16 as ::core::ffi::c_int as uint8_t,
        18 as ::core::ffi::c_int as uint8_t,
        23 as ::core::ffi::c_int as uint8_t,
        25 as ::core::ffi::c_int as uint8_t,
        27 as ::core::ffi::c_int as uint8_t,
        29 as ::core::ffi::c_int as uint8_t,
        31 as ::core::ffi::c_int as uint8_t,
        33 as ::core::ffi::c_int as uint8_t,
        18 as ::core::ffi::c_int as uint8_t,
        23 as ::core::ffi::c_int as uint8_t,
        25 as ::core::ffi::c_int as uint8_t,
        27 as ::core::ffi::c_int as uint8_t,
        29 as ::core::ffi::c_int as uint8_t,
        31 as ::core::ffi::c_int as uint8_t,
        33 as ::core::ffi::c_int as uint8_t,
        36 as ::core::ffi::c_int as uint8_t,
        23 as ::core::ffi::c_int as uint8_t,
        25 as ::core::ffi::c_int as uint8_t,
        27 as ::core::ffi::c_int as uint8_t,
        29 as ::core::ffi::c_int as uint8_t,
        31 as ::core::ffi::c_int as uint8_t,
        33 as ::core::ffi::c_int as uint8_t,
        36 as ::core::ffi::c_int as uint8_t,
        38 as ::core::ffi::c_int as uint8_t,
        25 as ::core::ffi::c_int as uint8_t,
        27 as ::core::ffi::c_int as uint8_t,
        29 as ::core::ffi::c_int as uint8_t,
        31 as ::core::ffi::c_int as uint8_t,
        33 as ::core::ffi::c_int as uint8_t,
        36 as ::core::ffi::c_int as uint8_t,
        38 as ::core::ffi::c_int as uint8_t,
        40 as ::core::ffi::c_int as uint8_t,
        27 as ::core::ffi::c_int as uint8_t,
        29 as ::core::ffi::c_int as uint8_t,
        31 as ::core::ffi::c_int as uint8_t,
        33 as ::core::ffi::c_int as uint8_t,
        36 as ::core::ffi::c_int as uint8_t,
        38 as ::core::ffi::c_int as uint8_t,
        40 as ::core::ffi::c_int as uint8_t,
        42 as ::core::ffi::c_int as uint8_t,
    ],
    [
        9 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        17 as ::core::ffi::c_int as uint8_t,
        19 as ::core::ffi::c_int as uint8_t,
        21 as ::core::ffi::c_int as uint8_t,
        22 as ::core::ffi::c_int as uint8_t,
        24 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        17 as ::core::ffi::c_int as uint8_t,
        19 as ::core::ffi::c_int as uint8_t,
        21 as ::core::ffi::c_int as uint8_t,
        22 as ::core::ffi::c_int as uint8_t,
        24 as ::core::ffi::c_int as uint8_t,
        25 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        17 as ::core::ffi::c_int as uint8_t,
        19 as ::core::ffi::c_int as uint8_t,
        21 as ::core::ffi::c_int as uint8_t,
        22 as ::core::ffi::c_int as uint8_t,
        24 as ::core::ffi::c_int as uint8_t,
        25 as ::core::ffi::c_int as uint8_t,
        27 as ::core::ffi::c_int as uint8_t,
        17 as ::core::ffi::c_int as uint8_t,
        19 as ::core::ffi::c_int as uint8_t,
        21 as ::core::ffi::c_int as uint8_t,
        22 as ::core::ffi::c_int as uint8_t,
        24 as ::core::ffi::c_int as uint8_t,
        25 as ::core::ffi::c_int as uint8_t,
        27 as ::core::ffi::c_int as uint8_t,
        28 as ::core::ffi::c_int as uint8_t,
        19 as ::core::ffi::c_int as uint8_t,
        21 as ::core::ffi::c_int as uint8_t,
        22 as ::core::ffi::c_int as uint8_t,
        24 as ::core::ffi::c_int as uint8_t,
        25 as ::core::ffi::c_int as uint8_t,
        27 as ::core::ffi::c_int as uint8_t,
        28 as ::core::ffi::c_int as uint8_t,
        30 as ::core::ffi::c_int as uint8_t,
        21 as ::core::ffi::c_int as uint8_t,
        22 as ::core::ffi::c_int as uint8_t,
        24 as ::core::ffi::c_int as uint8_t,
        25 as ::core::ffi::c_int as uint8_t,
        27 as ::core::ffi::c_int as uint8_t,
        28 as ::core::ffi::c_int as uint8_t,
        30 as ::core::ffi::c_int as uint8_t,
        32 as ::core::ffi::c_int as uint8_t,
        22 as ::core::ffi::c_int as uint8_t,
        24 as ::core::ffi::c_int as uint8_t,
        25 as ::core::ffi::c_int as uint8_t,
        27 as ::core::ffi::c_int as uint8_t,
        28 as ::core::ffi::c_int as uint8_t,
        30 as ::core::ffi::c_int as uint8_t,
        32 as ::core::ffi::c_int as uint8_t,
        33 as ::core::ffi::c_int as uint8_t,
        24 as ::core::ffi::c_int as uint8_t,
        25 as ::core::ffi::c_int as uint8_t,
        27 as ::core::ffi::c_int as uint8_t,
        28 as ::core::ffi::c_int as uint8_t,
        30 as ::core::ffi::c_int as uint8_t,
        32 as ::core::ffi::c_int as uint8_t,
        33 as ::core::ffi::c_int as uint8_t,
        35 as ::core::ffi::c_int as uint8_t,
    ],
];
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
static mut level_max_dpb_mbs: [[::core::ffi::c_int; 2]; 16] = [
    [10 as ::core::ffi::c_int, 396 as ::core::ffi::c_int],
    [11 as ::core::ffi::c_int, 900 as ::core::ffi::c_int],
    [12 as ::core::ffi::c_int, 2376 as ::core::ffi::c_int],
    [13 as ::core::ffi::c_int, 2376 as ::core::ffi::c_int],
    [20 as ::core::ffi::c_int, 2376 as ::core::ffi::c_int],
    [21 as ::core::ffi::c_int, 4752 as ::core::ffi::c_int],
    [22 as ::core::ffi::c_int, 8100 as ::core::ffi::c_int],
    [30 as ::core::ffi::c_int, 8100 as ::core::ffi::c_int],
    [31 as ::core::ffi::c_int, 18000 as ::core::ffi::c_int],
    [32 as ::core::ffi::c_int, 20480 as ::core::ffi::c_int],
    [40 as ::core::ffi::c_int, 32768 as ::core::ffi::c_int],
    [41 as ::core::ffi::c_int, 32768 as ::core::ffi::c_int],
    [42 as ::core::ffi::c_int, 34816 as ::core::ffi::c_int],
    [50 as ::core::ffi::c_int, 110400 as ::core::ffi::c_int],
    [51 as ::core::ffi::c_int, 184320 as ::core::ffi::c_int],
    [52 as ::core::ffi::c_int, 184320 as ::core::ffi::c_int],
];
unsafe extern "C" fn remove_pps(mut s: *mut H264ParamSets, mut id: ::core::ffi::c_int) {
    av_refstruct_unref(
        (&raw mut (*s).pps_list as *mut *const PPS).offset(id as isize)
            as *mut *const PPS as *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" fn remove_sps(mut s: *mut H264ParamSets, mut id: ::core::ffi::c_int) {
    av_refstruct_unref(
        (&raw mut (*s).sps_list as *mut *const SPS).offset(id as isize)
            as *mut *const SPS as *mut ::core::ffi::c_void,
    );
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
unsafe extern "C" fn decode_hrd_parameters(
    mut gb: *mut GetBitContext,
    mut logctx: *mut ::core::ffi::c_void,
    mut sps: *mut SPS,
) -> ::core::ffi::c_int {
    let mut cpb_count: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    cpb_count = get_ue_golomb_31(gb) + 1 as ::core::ffi::c_int;
    if cpb_count as ::core::ffi::c_uint > 32 as ::core::ffi::c_uint {
        av_log(
            logctx,
            16 as ::core::ffi::c_int,
            b"cpb_count %d invalid\n\0" as *const u8 as *const ::core::ffi::c_char,
            cpb_count,
        );
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    (*sps).cpr_flag = 0 as uint32_t;
    (*sps).bit_rate_scale = get_bits(gb, 4 as ::core::ffi::c_int) as ::core::ffi::c_int;
    get_bits(gb, 4 as ::core::ffi::c_int);
    i = 0 as ::core::ffi::c_int;
    while i < cpb_count {
        (*sps).bit_rate_value[i as usize] = get_ue_golomb_long(gb)
            .wrapping_add(1 as ::core::ffi::c_uint) as uint32_t;
        (*sps).cpb_size_value[i as usize] = get_ue_golomb_long(gb)
            .wrapping_add(1 as ::core::ffi::c_uint) as uint32_t;
        (*sps).cpr_flag |= get_bits1(gb) << i;
        i += 1;
    }
    (*sps).initial_cpb_removal_delay_length = get_bits(gb, 5 as ::core::ffi::c_int)
        .wrapping_add(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    (*sps).cpb_removal_delay_length = get_bits(gb, 5 as ::core::ffi::c_int)
        .wrapping_add(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    (*sps).dpb_output_delay_length = get_bits(gb, 5 as ::core::ffi::c_int)
        .wrapping_add(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    (*sps).time_offset_length = get_bits(gb, 5 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    (*sps).cpb_cnt = cpb_count;
    return 0 as ::core::ffi::c_int;
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
unsafe extern "C" fn decode_vui_parameters(
    mut gb: *mut GetBitContext,
    mut logctx: *mut ::core::ffi::c_void,
    mut sps: *mut SPS,
) -> ::core::ffi::c_int {
    ff_h2645_decode_common_vui_params(gb, &raw mut (*sps).vui, logctx);
    if show_bits1(gb) != 0 && get_bits_left(gb) < 10 as ::core::ffi::c_int {
        av_log(
            logctx,
            24 as ::core::ffi::c_int,
            b"Truncated VUI (%d)\n\0" as *const u8 as *const ::core::ffi::c_char,
            get_bits_left(gb),
        );
        return 0 as ::core::ffi::c_int;
    }
    (*sps).timing_info_present_flag = get_bits1(gb) as ::core::ffi::c_int;
    if (*sps).timing_info_present_flag != 0 {
        let mut num_units_in_tick: ::core::ffi::c_uint = get_bits_long(
            gb,
            32 as ::core::ffi::c_int,
        );
        let mut time_scale: ::core::ffi::c_uint = get_bits_long(
            gb,
            32 as ::core::ffi::c_int,
        );
        if num_units_in_tick == 0 || time_scale == 0 {
            av_log(
                logctx,
                16 as ::core::ffi::c_int,
                b"time_scale/num_units_in_tick invalid or unsupported (%u/%u)\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                time_scale,
                num_units_in_tick,
            );
            (*sps).timing_info_present_flag = 0 as ::core::ffi::c_int;
        } else {
            (*sps).num_units_in_tick = num_units_in_tick as uint32_t;
            (*sps).time_scale = time_scale as uint32_t;
        }
        (*sps).fixed_frame_rate_flag = get_bits1(gb) as ::core::ffi::c_int;
    }
    (*sps).nal_hrd_parameters_present_flag = get_bits1(gb) as ::core::ffi::c_int;
    if (*sps).nal_hrd_parameters_present_flag != 0 {
        if decode_hrd_parameters(gb, logctx, sps) < 0 as ::core::ffi::c_int {
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
    }
    (*sps).vcl_hrd_parameters_present_flag = get_bits1(gb) as ::core::ffi::c_int;
    if (*sps).vcl_hrd_parameters_present_flag != 0 {
        if decode_hrd_parameters(gb, logctx, sps) < 0 as ::core::ffi::c_int {
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
    }
    if (*sps).nal_hrd_parameters_present_flag != 0
        || (*sps).vcl_hrd_parameters_present_flag != 0
    {
        get_bits1(gb);
    }
    (*sps).pic_struct_present_flag = get_bits1(gb) as ::core::ffi::c_int;
    if get_bits_left(gb) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    (*sps).bitstream_restriction_flag = get_bits1(gb) as ::core::ffi::c_int;
    if (*sps).bitstream_restriction_flag != 0 {
        get_bits1(gb);
        get_ue_golomb_31(gb);
        get_ue_golomb_31(gb);
        get_ue_golomb_31(gb);
        get_ue_golomb_31(gb);
        (*sps).num_reorder_frames = get_ue_golomb_31(gb);
        (*sps).max_dec_frame_buffering = get_ue_golomb_31(gb);
        if get_bits_left(gb) < 0 as ::core::ffi::c_int {
            (*sps).num_reorder_frames = 0 as ::core::ffi::c_int;
            (*sps).bitstream_restriction_flag = 0 as ::core::ffi::c_int;
        }
        if (*sps).num_reorder_frames as ::core::ffi::c_uint > 16 as ::core::ffi::c_uint {
            av_log(
                logctx,
                16 as ::core::ffi::c_int,
                b"Clipping illegal num_reorder_frames %d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*sps).num_reorder_frames,
            );
            (*sps).num_reorder_frames = 16 as ::core::ffi::c_int;
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
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
unsafe extern "C" fn decode_scaling_list(
    mut gb: *mut GetBitContext,
    mut factors: *mut uint8_t,
    mut size: ::core::ffi::c_int,
    mut jvt_list: *const uint8_t,
    mut fallback_list: *const uint8_t,
    mut mask: *mut uint16_t,
    mut pos: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut last: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    let mut next: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    let mut scan: *const uint8_t = if size == 16 as ::core::ffi::c_int {
        &raw const ff_zigzag_scan as *const uint8_t
    } else {
        &raw const ff_zigzag_direct as *const uint8_t
    };
    let mut seq_scaling_list_present_flag: uint16_t = get_bits1(gb) as uint16_t;
    *mask = (*mask as ::core::ffi::c_int
        | (seq_scaling_list_present_flag as ::core::ffi::c_int) << pos) as uint16_t;
    if seq_scaling_list_present_flag == 0 {
        memcpy(
            factors as *mut ::core::ffi::c_void,
            fallback_list as *const ::core::ffi::c_void,
            (size as size_t).wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        );
    } else {
        i = 0 as ::core::ffi::c_int;
        while i < size {
            if next != 0 {
                let mut v: ::core::ffi::c_int = get_se_golomb(gb);
                if v < -(128 as ::core::ffi::c_int) || v > 127 as ::core::ffi::c_int {
                    av_log(
                        ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        16 as ::core::ffi::c_int,
                        b"delta scale %d is invalid\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        v,
                    );
                    return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                        | ('D' as i32) << 16 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint
                        | ('A' as i32 as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                }
                next = last + v & 0xff as ::core::ffi::c_int;
            }
            if i == 0 && next == 0 {
                memcpy(
                    factors as *mut ::core::ffi::c_void,
                    jvt_list as *const ::core::ffi::c_void,
                    (size as size_t)
                        .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
                );
                break;
            } else {
                let ref mut fresh1 = *factors.offset(*scan.offset(i as isize) as isize);
                *fresh1 = (if next != 0 { next } else { last }) as uint8_t;
                last = *fresh1 as ::core::ffi::c_int;
                i += 1;
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn decode_scaling_matrices(
    mut gb: *mut GetBitContext,
    mut sps: *const SPS,
    mut pps: *const PPS,
    mut is_sps: ::core::ffi::c_int,
    mut present_flag: ::core::ffi::c_int,
    mut mask: *mut uint16_t,
    mut scaling_matrix4: *mut [uint8_t; 16],
    mut scaling_matrix8: *mut [uint8_t; 64],
) -> ::core::ffi::c_int {
    let mut fallback_sps: ::core::ffi::c_int = (is_sps == 0
        && (*sps).scaling_matrix_present != 0) as ::core::ffi::c_int;
    let mut fallback: [*const uint8_t; 4] = [
        if fallback_sps != 0 {
            &raw const *(&raw const (*sps).scaling_matrix4 as *const [uint8_t; 16])
                .offset(0 as ::core::ffi::c_int as isize) as *const uint8_t
        } else {
            &raw const *(&raw const default_scaling4 as *const [uint8_t; 16])
                .offset(0 as ::core::ffi::c_int as isize) as *const uint8_t
        },
        if fallback_sps != 0 {
            &raw const *(&raw const (*sps).scaling_matrix4 as *const [uint8_t; 16])
                .offset(3 as ::core::ffi::c_int as isize) as *const uint8_t
        } else {
            &raw const *(&raw const default_scaling4 as *const [uint8_t; 16])
                .offset(1 as ::core::ffi::c_int as isize) as *const uint8_t
        },
        if fallback_sps != 0 {
            &raw const *(&raw const (*sps).scaling_matrix8 as *const [uint8_t; 64])
                .offset(0 as ::core::ffi::c_int as isize) as *const uint8_t
        } else {
            &raw const *(&raw const default_scaling8 as *const [uint8_t; 64])
                .offset(0 as ::core::ffi::c_int as isize) as *const uint8_t
        },
        if fallback_sps != 0 {
            &raw const *(&raw const (*sps).scaling_matrix8 as *const [uint8_t; 64])
                .offset(3 as ::core::ffi::c_int as isize) as *const uint8_t
        } else {
            &raw const *(&raw const default_scaling8 as *const [uint8_t; 64])
                .offset(1 as ::core::ffi::c_int as isize) as *const uint8_t
        },
    ];
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    *mask = 0 as uint16_t;
    if present_flag != 0 {
        ret
            |= decode_scaling_list(
                gb,
                &raw mut *scaling_matrix4.offset(0 as ::core::ffi::c_int as isize)
                    as *mut uint8_t,
                16 as ::core::ffi::c_int,
                &raw const *(&raw const default_scaling4 as *const [uint8_t; 16])
                    .offset(0 as ::core::ffi::c_int as isize) as *const uint8_t,
                fallback[0 as ::core::ffi::c_int as usize],
                mask,
                0 as ::core::ffi::c_int,
            );
        ret
            |= decode_scaling_list(
                gb,
                &raw mut *scaling_matrix4.offset(1 as ::core::ffi::c_int as isize)
                    as *mut uint8_t,
                16 as ::core::ffi::c_int,
                &raw const *(&raw const default_scaling4 as *const [uint8_t; 16])
                    .offset(0 as ::core::ffi::c_int as isize) as *const uint8_t,
                &raw mut *scaling_matrix4.offset(0 as ::core::ffi::c_int as isize)
                    as *mut uint8_t,
                mask,
                1 as ::core::ffi::c_int,
            );
        ret
            |= decode_scaling_list(
                gb,
                &raw mut *scaling_matrix4.offset(2 as ::core::ffi::c_int as isize)
                    as *mut uint8_t,
                16 as ::core::ffi::c_int,
                &raw const *(&raw const default_scaling4 as *const [uint8_t; 16])
                    .offset(0 as ::core::ffi::c_int as isize) as *const uint8_t,
                &raw mut *scaling_matrix4.offset(1 as ::core::ffi::c_int as isize)
                    as *mut uint8_t,
                mask,
                2 as ::core::ffi::c_int,
            );
        ret
            |= decode_scaling_list(
                gb,
                &raw mut *scaling_matrix4.offset(3 as ::core::ffi::c_int as isize)
                    as *mut uint8_t,
                16 as ::core::ffi::c_int,
                &raw const *(&raw const default_scaling4 as *const [uint8_t; 16])
                    .offset(1 as ::core::ffi::c_int as isize) as *const uint8_t,
                fallback[1 as ::core::ffi::c_int as usize],
                mask,
                3 as ::core::ffi::c_int,
            );
        ret
            |= decode_scaling_list(
                gb,
                &raw mut *scaling_matrix4.offset(4 as ::core::ffi::c_int as isize)
                    as *mut uint8_t,
                16 as ::core::ffi::c_int,
                &raw const *(&raw const default_scaling4 as *const [uint8_t; 16])
                    .offset(1 as ::core::ffi::c_int as isize) as *const uint8_t,
                &raw mut *scaling_matrix4.offset(3 as ::core::ffi::c_int as isize)
                    as *mut uint8_t,
                mask,
                4 as ::core::ffi::c_int,
            );
        ret
            |= decode_scaling_list(
                gb,
                &raw mut *scaling_matrix4.offset(5 as ::core::ffi::c_int as isize)
                    as *mut uint8_t,
                16 as ::core::ffi::c_int,
                &raw const *(&raw const default_scaling4 as *const [uint8_t; 16])
                    .offset(1 as ::core::ffi::c_int as isize) as *const uint8_t,
                &raw mut *scaling_matrix4.offset(4 as ::core::ffi::c_int as isize)
                    as *mut uint8_t,
                mask,
                5 as ::core::ffi::c_int,
            );
        if is_sps != 0 || (*pps).transform_8x8_mode != 0 {
            ret
                |= decode_scaling_list(
                    gb,
                    &raw mut *scaling_matrix8.offset(0 as ::core::ffi::c_int as isize)
                        as *mut uint8_t,
                    64 as ::core::ffi::c_int,
                    &raw const *(&raw const default_scaling8 as *const [uint8_t; 64])
                        .offset(0 as ::core::ffi::c_int as isize) as *const uint8_t,
                    fallback[2 as ::core::ffi::c_int as usize],
                    mask,
                    6 as ::core::ffi::c_int,
                );
            ret
                |= decode_scaling_list(
                    gb,
                    &raw mut *scaling_matrix8.offset(3 as ::core::ffi::c_int as isize)
                        as *mut uint8_t,
                    64 as ::core::ffi::c_int,
                    &raw const *(&raw const default_scaling8 as *const [uint8_t; 64])
                        .offset(1 as ::core::ffi::c_int as isize) as *const uint8_t,
                    fallback[3 as ::core::ffi::c_int as usize],
                    mask,
                    7 as ::core::ffi::c_int,
                );
            if (*sps).chroma_format_idc == 3 as ::core::ffi::c_int {
                ret
                    |= decode_scaling_list(
                        gb,
                        &raw mut *scaling_matrix8
                            .offset(1 as ::core::ffi::c_int as isize) as *mut uint8_t,
                        64 as ::core::ffi::c_int,
                        &raw const *(&raw const default_scaling8 as *const [uint8_t; 64])
                            .offset(0 as ::core::ffi::c_int as isize) as *const uint8_t,
                        &raw mut *scaling_matrix8
                            .offset(0 as ::core::ffi::c_int as isize) as *mut uint8_t,
                        mask,
                        8 as ::core::ffi::c_int,
                    );
                ret
                    |= decode_scaling_list(
                        gb,
                        &raw mut *scaling_matrix8
                            .offset(4 as ::core::ffi::c_int as isize) as *mut uint8_t,
                        64 as ::core::ffi::c_int,
                        &raw const *(&raw const default_scaling8 as *const [uint8_t; 64])
                            .offset(1 as ::core::ffi::c_int as isize) as *const uint8_t,
                        &raw mut *scaling_matrix8
                            .offset(3 as ::core::ffi::c_int as isize) as *mut uint8_t,
                        mask,
                        9 as ::core::ffi::c_int,
                    );
                ret
                    |= decode_scaling_list(
                        gb,
                        &raw mut *scaling_matrix8
                            .offset(2 as ::core::ffi::c_int as isize) as *mut uint8_t,
                        64 as ::core::ffi::c_int,
                        &raw const *(&raw const default_scaling8 as *const [uint8_t; 64])
                            .offset(0 as ::core::ffi::c_int as isize) as *const uint8_t,
                        &raw mut *scaling_matrix8
                            .offset(1 as ::core::ffi::c_int as isize) as *mut uint8_t,
                        mask,
                        10 as ::core::ffi::c_int,
                    );
                ret
                    |= decode_scaling_list(
                        gb,
                        &raw mut *scaling_matrix8
                            .offset(5 as ::core::ffi::c_int as isize) as *mut uint8_t,
                        64 as ::core::ffi::c_int,
                        &raw const *(&raw const default_scaling8 as *const [uint8_t; 64])
                            .offset(1 as ::core::ffi::c_int as isize) as *const uint8_t,
                        &raw mut *scaling_matrix8
                            .offset(4 as ::core::ffi::c_int as isize) as *mut uint8_t,
                        mask,
                        11 as ::core::ffi::c_int,
                    );
            }
        }
        if ret == 0 {
            ret = is_sps;
        }
    }
    return ret;
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
        let mut log: ::core::ffi::c_int = ff_log2_c(buf);
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
#[inline]
unsafe extern "C" fn get_bits_bytesize(
    mut s: *const GetBitContext,
    mut round_up: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (*s).size_in_bits
        + (if round_up != 0 { 7 as ::core::ffi::c_int } else { 0 as ::core::ffi::c_int })
        >> 3 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_h264_ps_uninit(mut ps: *mut H264ParamSets) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 32 as ::core::ffi::c_int {
        av_refstruct_unref(
            (&raw mut (*ps).sps_list as *mut *const SPS).offset(i as isize)
                as *mut *const SPS as *mut ::core::ffi::c_void,
        );
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 256 as ::core::ffi::c_int {
        av_refstruct_unref(
            (&raw mut (*ps).pps_list as *mut *const PPS).offset(i as isize)
                as *mut *const PPS as *mut ::core::ffi::c_void,
        );
        i += 1;
    }
    av_refstruct_unref(&raw mut (*ps).pps as *mut ::core::ffi::c_void);
    (*ps).sps = ::core::ptr::null::<SPS>();
}
#[inline]
unsafe extern "C" fn skip_bits_long(
    mut s: *mut GetBitContext,
    mut n: ::core::ffi::c_int,
) {
    (*s).index += av_clip_c(n, -(*s).index, (*s).size_in_bits_plus8 - (*s).index);
}
#[no_mangle]
pub unsafe extern "C" fn ff_h264_decode_seq_parameter_set(
    mut gb: *mut GetBitContext,
    mut avctx: *mut AVCodecContext,
    mut ps: *mut H264ParamSets,
    mut ignore_truncation: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut profile_idc: ::core::ffi::c_int = 0;
    let mut level_idc: ::core::ffi::c_int = 0;
    let mut constraint_set_flags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sps_id: ::core::ffi::c_uint = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut log2_max_frame_num_minus4: ::core::ffi::c_int = 0;
    let mut sps: *mut SPS = ::core::ptr::null_mut::<SPS>();
    let mut ret: ::core::ffi::c_int = 0;
    sps = av_refstruct_allocz(::core::mem::size_of::<SPS>() as size_t) as *mut SPS;
    if sps.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    (*sps).data_size = get_bits_bytesize(gb, 1 as ::core::ffi::c_int) as size_t;
    if (*sps).data_size as usize > ::core::mem::size_of::<[uint8_t; 4096]>() as usize {
        av_log(
            avctx as *mut ::core::ffi::c_void,
            48 as ::core::ffi::c_int,
            b"Truncating likely oversized SPS\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        (*sps).data_size = ::core::mem::size_of::<[uint8_t; 4096]>() as usize as size_t;
    }
    memcpy(
        &raw mut (*sps).data as *mut uint8_t as *mut ::core::ffi::c_void,
        (*gb).buffer as *const ::core::ffi::c_void,
        (*sps).data_size,
    );
    if (*gb).size_in_bits & 7 as ::core::ffi::c_int == 0
        && ((*sps).data_size as usize)
            < ::core::mem::size_of::<[uint8_t; 4096]>() as usize
    {
        let fresh0 = (*sps).data_size;
        (*sps).data_size = (*sps).data_size.wrapping_add(1);
        (*sps).data[fresh0 as usize] = 0x80 as uint8_t;
    }
    profile_idc = get_bits(gb, 8 as ::core::ffi::c_int) as ::core::ffi::c_int;
    constraint_set_flags = (constraint_set_flags as ::core::ffi::c_uint
        | get_bits1(gb) << 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    constraint_set_flags = (constraint_set_flags as ::core::ffi::c_uint
        | get_bits1(gb) << 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    constraint_set_flags = (constraint_set_flags as ::core::ffi::c_uint
        | get_bits1(gb) << 2 as ::core::ffi::c_int) as ::core::ffi::c_int;
    constraint_set_flags = (constraint_set_flags as ::core::ffi::c_uint
        | get_bits1(gb) << 3 as ::core::ffi::c_int) as ::core::ffi::c_int;
    constraint_set_flags = (constraint_set_flags as ::core::ffi::c_uint
        | get_bits1(gb) << 4 as ::core::ffi::c_int) as ::core::ffi::c_int;
    constraint_set_flags = (constraint_set_flags as ::core::ffi::c_uint
        | get_bits1(gb) << 5 as ::core::ffi::c_int) as ::core::ffi::c_int;
    skip_bits(gb, 2 as ::core::ffi::c_int);
    level_idc = get_bits(gb, 8 as ::core::ffi::c_int) as ::core::ffi::c_int;
    sps_id = get_ue_golomb_31(gb) as ::core::ffi::c_uint;
    if sps_id >= 32 as ::core::ffi::c_uint {
        av_log(
            avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"sps_id %u out of range\n\0" as *const u8 as *const ::core::ffi::c_char,
            sps_id,
        );
    } else {
        (*sps).sps_id = sps_id;
        (*sps).time_offset_length = 24 as ::core::ffi::c_int;
        (*sps).profile_idc = profile_idc;
        (*sps).constraint_set_flags = constraint_set_flags;
        (*sps).level_idc = level_idc;
        (*sps).vui.video_full_range_flag = -(1 as ::core::ffi::c_int);
        memset(
            &raw mut (*sps).scaling_matrix4 as *mut [uint8_t; 16]
                as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            ::core::mem::size_of::<[[uint8_t; 16]; 6]>() as size_t,
        );
        memset(
            &raw mut (*sps).scaling_matrix8 as *mut [uint8_t; 64]
                as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            ::core::mem::size_of::<[[uint8_t; 64]; 6]>() as size_t,
        );
        (*sps).scaling_matrix_present = 0 as ::core::ffi::c_int;
        (*sps).vui.matrix_coeffs = AVCOL_SPC_UNSPECIFIED;
        if (*sps).profile_idc == 100 as ::core::ffi::c_int
            || (*sps).profile_idc == 110 as ::core::ffi::c_int
            || (*sps).profile_idc == 122 as ::core::ffi::c_int
            || (*sps).profile_idc == 244 as ::core::ffi::c_int
            || (*sps).profile_idc == 44 as ::core::ffi::c_int
            || (*sps).profile_idc == 83 as ::core::ffi::c_int
            || (*sps).profile_idc == 86 as ::core::ffi::c_int
            || (*sps).profile_idc == 118 as ::core::ffi::c_int
            || (*sps).profile_idc == 128 as ::core::ffi::c_int
            || (*sps).profile_idc == 138 as ::core::ffi::c_int
            || (*sps).profile_idc == 144 as ::core::ffi::c_int
        {
            (*sps).chroma_format_idc = get_ue_golomb_31(gb);
            if (*sps).chroma_format_idc != 1 as ::core::ffi::c_int
            {
                avpriv_request_sample(
                    avctx as *mut ::core::ffi::c_void,
                    b"chroma_format_idc %u\0" as *const u8 as *const ::core::ffi::c_char,
                    (*sps).chroma_format_idc,
                );
                current_block = 2932924877315219625;
            } else {
                if (*sps).chroma_format_idc == 3 as ::core::ffi::c_int {
                    (*sps).residual_color_transform_flag = get_bits1(gb)
                        as ::core::ffi::c_int;
                    if (*sps).residual_color_transform_flag != 0 {
                        av_log(
                            avctx as *mut ::core::ffi::c_void,
                            16 as ::core::ffi::c_int,
                            b"separate color planes are not supported\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        current_block = 2932924877315219625;
                    } else {
                        current_block = 2569451025026770673;
                    }
                } else {
                    current_block = 2569451025026770673;
                }
                match current_block {
                    2932924877315219625 => {}
                    _ => {
                        (*sps).bit_depth_luma = get_ue_golomb_31(gb)
                            + 8 as ::core::ffi::c_int;
                        (*sps).bit_depth_chroma = get_ue_golomb_31(gb)
                            + 8 as ::core::ffi::c_int;
                        if (*sps).bit_depth_chroma != (*sps).bit_depth_luma {
                            avpriv_request_sample(
                                avctx as *mut ::core::ffi::c_void,
                                b"Different chroma and luma bit depth\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            current_block = 2932924877315219625;
                        } else if (*sps).bit_depth_luma != 8 as ::core::ffi::c_int
                            || (*sps).bit_depth_chroma != 8 as ::core::ffi::c_int
                        {
                            av_log(
                                avctx as *mut ::core::ffi::c_void,
                                16 as ::core::ffi::c_int,
                                b"illegal bit depth value (%d, %d)\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                (*sps).bit_depth_luma,
                                (*sps).bit_depth_chroma,
                            );
                            current_block = 2932924877315219625;
                        } else {
                            (*sps).transform_bypass = get_bits1(gb)
                                as ::core::ffi::c_int;
                            ret = decode_scaling_matrices(
                                gb,
                                sps,
                                ::core::ptr::null::<PPS>(),
                                1 as ::core::ffi::c_int,
                                get_bits1(gb) as ::core::ffi::c_int,
                                &raw mut (*sps).scaling_matrix_present_mask,
                                &raw mut (*sps).scaling_matrix4 as *mut [uint8_t; 16],
                                &raw mut (*sps).scaling_matrix8 as *mut [uint8_t; 64],
                            );
                            if ret < 0 as ::core::ffi::c_int {
                                current_block = 2932924877315219625;
                            } else {
                                (*sps).scaling_matrix_present |= ret;
                                current_block = 15512526488502093901;
                            }
                        }
                    }
                }
            }
        } else {
            (*sps).chroma_format_idc = 1 as ::core::ffi::c_int;
            (*sps).bit_depth_luma = 8 as ::core::ffi::c_int;
            (*sps).bit_depth_chroma = 8 as ::core::ffi::c_int;
            current_block = 15512526488502093901;
        }
        match current_block {
            2932924877315219625 => {}
            _ => {
                log2_max_frame_num_minus4 = get_ue_golomb_31(gb);
                if log2_max_frame_num_minus4
                    < 4 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                    || log2_max_frame_num_minus4
                        > 12 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                            - 4 as ::core::ffi::c_int
                {
                    av_log(
                        avctx as *mut ::core::ffi::c_void,
                        16 as ::core::ffi::c_int,
                        b"log2_max_frame_num_minus4 out of range (0-12): %d\n\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        log2_max_frame_num_minus4,
                    );
                } else {
                    (*sps).log2_max_frame_num = log2_max_frame_num_minus4
                        + 4 as ::core::ffi::c_int;
                    (*sps).poc_type = get_ue_golomb_31(gb);
                    if (*sps).poc_type == 0 as ::core::ffi::c_int {
                        let mut t: ::core::ffi::c_uint = get_ue_golomb_31(gb)
                            as ::core::ffi::c_uint;
                        if t > 12 as ::core::ffi::c_uint {
                            av_log(
                                avctx as *mut ::core::ffi::c_void,
                                16 as ::core::ffi::c_int,
                                b"log2_max_poc_lsb (%d) is out of range\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                t,
                            );
                            current_block = 2932924877315219625;
                        } else {
                            (*sps).log2_max_poc_lsb = t
                                .wrapping_add(4 as ::core::ffi::c_uint)
                                as ::core::ffi::c_int;
                            current_block = 5372832139739605200;
                        }
                    } else if (*sps).poc_type == 1 as ::core::ffi::c_int {
                        (*sps).delta_pic_order_always_zero_flag = get_bits1(gb)
                            as ::core::ffi::c_int;
                        (*sps).offset_for_non_ref_pic = get_se_golomb_long(gb);
                        (*sps).offset_for_top_to_bottom_field = get_se_golomb_long(gb);
                        if (*sps).offset_for_non_ref_pic
                            == -(1 as ::core::ffi::c_int)
                                - 0x7fffffff as ::core::ffi::c_int
                            || (*sps).offset_for_top_to_bottom_field
                                == -(1 as ::core::ffi::c_int)
                                    - 0x7fffffff as ::core::ffi::c_int
                        {
                            av_log(
                                avctx as *mut ::core::ffi::c_void,
                                16 as ::core::ffi::c_int,
                                b"offset_for_non_ref_pic or offset_for_top_to_bottom_field is out of range\n\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                            );
                            current_block = 2932924877315219625;
                        } else {
                            (*sps).poc_cycle_length = get_ue_golomb(gb);
                            if (*sps).poc_cycle_length as usize
                                >= (::core::mem::size_of::<[int32_t; 256]>() as usize)
                                    .wrapping_div(::core::mem::size_of::<int32_t>() as usize)
                            {
                                av_log(
                                    avctx as *mut ::core::ffi::c_void,
                                    16 as ::core::ffi::c_int,
                                    b"poc_cycle_length overflow %d\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    (*sps).poc_cycle_length,
                                );
                                current_block = 2932924877315219625;
                            } else {
                                i = 0 as ::core::ffi::c_int;
                                loop {
                                    if !(i < (*sps).poc_cycle_length) {
                                        current_block = 5372832139739605200;
                                        break;
                                    }
                                    (*sps).offset_for_ref_frame[i as usize] = get_se_golomb_long(
                                        gb,
                                    ) as int32_t;
                                    if (*sps).offset_for_ref_frame[i as usize]
                                        == -(1 as ::core::ffi::c_int)
                                            - 0x7fffffff as ::core::ffi::c_int
                                    {
                                        av_log(
                                            avctx as *mut ::core::ffi::c_void,
                                            16 as ::core::ffi::c_int,
                                            b"offset_for_ref_frame is out of range\n\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                        current_block = 2932924877315219625;
                                        break;
                                    } else {
                                        i += 1;
                                    }
                                }
                            }
                        }
                    } else if (*sps).poc_type != 2 as ::core::ffi::c_int {
                        av_log(
                            avctx as *mut ::core::ffi::c_void,
                            16 as ::core::ffi::c_int,
                            b"illegal POC type %d\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                            (*sps).poc_type,
                        );
                        current_block = 2932924877315219625;
                    } else {
                        current_block = 5372832139739605200;
                    }
                    match current_block {
                        2932924877315219625 => {}
                        _ => {
                            (*sps).ref_frame_count = get_ue_golomb_31(gb);
                            if (*avctx).codec_tag
                                == ('S' as i32 | ('M' as i32) << 8 as ::core::ffi::c_int
                                    | ('V' as i32) << 16 as ::core::ffi::c_int)
                                    as ::core::ffi::c_uint
                                    | ('2' as i32 as ::core::ffi::c_uint)
                                        << 24 as ::core::ffi::c_int
                            {
                                (*sps).ref_frame_count = if 2 as ::core::ffi::c_int
                                    > (*sps).ref_frame_count
                                {
                                    2 as ::core::ffi::c_int
                                } else {
                                    (*sps).ref_frame_count
                                };
                            }
                            if (*sps).ref_frame_count
                                > H264_MAX_DPB_FRAMES as ::core::ffi::c_int
                            {
                                av_log(
                                    avctx as *mut ::core::ffi::c_void,
                                    16 as ::core::ffi::c_int,
                                    b"too many reference frames %d\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    (*sps).ref_frame_count,
                                );
                            } else {
                                (*sps).gaps_in_frame_num_allowed_flag = get_bits1(gb)
                                    as ::core::ffi::c_int;
                                (*sps).mb_width = get_ue_golomb(gb)
                                    + 1 as ::core::ffi::c_int;
                                (*sps).mb_height = get_ue_golomb(gb)
                                    + 1 as ::core::ffi::c_int;
                                (*sps).frame_mbs_only_flag = get_bits1(gb)
                                    as ::core::ffi::c_int;
                                if (*sps).mb_height as ::core::ffi::c_uint
                                    >= (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                                        .wrapping_div(2 as ::core::ffi::c_uint)
                                {
                                    av_log(
                                        avctx as *mut ::core::ffi::c_void,
                                        16 as ::core::ffi::c_int,
                                        b"height overflow\n\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                } else {
                                    (*sps).mb_height
                                        *= 2 as ::core::ffi::c_int - (*sps).frame_mbs_only_flag;
                                    if (*sps).frame_mbs_only_flag == 0 {
                                        (*sps).mb_aff = get_bits1(gb) as ::core::ffi::c_int;
                                    } else {
                                        (*sps).mb_aff = 0 as ::core::ffi::c_int;
                                    }
                                    if (*sps).mb_width as ::core::ffi::c_uint
                                        >= (2147483647 as ::core::ffi::c_int
                                            / 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                                        || (*sps).mb_height as ::core::ffi::c_uint
                                            >= (2147483647 as ::core::ffi::c_int
                                                / 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                                        || av_image_check_size(
                                            (16 as ::core::ffi::c_int * (*sps).mb_width)
                                                as ::core::ffi::c_uint,
                                            (16 as ::core::ffi::c_int * (*sps).mb_height)
                                                as ::core::ffi::c_uint,
                                            0 as ::core::ffi::c_int,
                                            avctx as *mut ::core::ffi::c_void,
                                        ) != 0
                                    {
                                        av_log(
                                            avctx as *mut ::core::ffi::c_void,
                                            16 as ::core::ffi::c_int,
                                            b"mb_width/height overflow\n\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    } else {
                                        (*sps).direct_8x8_inference_flag = get_bits1(gb)
                                            as ::core::ffi::c_int;
                                        (*sps).crop = get_bits1(gb) as ::core::ffi::c_int;
                                        if (*sps).crop != 0 {
                                            let mut crop_left: ::core::ffi::c_uint = get_ue_golomb(gb)
                                                as ::core::ffi::c_uint;
                                            let mut crop_right: ::core::ffi::c_uint = get_ue_golomb(gb)
                                                as ::core::ffi::c_uint;
                                            let mut crop_top: ::core::ffi::c_uint = get_ue_golomb(gb)
                                                as ::core::ffi::c_uint;
                                            let mut crop_bottom: ::core::ffi::c_uint = get_ue_golomb(gb)
                                                as ::core::ffi::c_uint;
                                            let mut width: ::core::ffi::c_int = 16 as ::core::ffi::c_int
                                                * (*sps).mb_width;
                                            let mut height: ::core::ffi::c_int = 16
                                                as ::core::ffi::c_int * (*sps).mb_height;
                                            if (*avctx).flags2
                                                & (1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int != 0
                                            {
                                                av_log(
                                                    avctx as *mut ::core::ffi::c_void,
                                                    48 as ::core::ffi::c_int,
                                                    b"discarding sps cropping, original values are l:%d r:%d t:%d b:%d\n\0"
                                                        as *const u8 as *const ::core::ffi::c_char,
                                                    crop_left,
                                                    crop_right,
                                                    crop_top,
                                                    crop_bottom,
                                                );
                                                (*sps).crop_bottom = 0 as ::core::ffi::c_uint;
                                                (*sps).crop_top = (*sps).crop_bottom;
                                                (*sps).crop_right = (*sps).crop_top;
                                                (*sps).crop_left = (*sps).crop_right;
                                                current_block = 18425699056680496821;
                                            } else {
                                                let mut vsub: ::core::ffi::c_int = if (*sps)
                                                    .chroma_format_idc == 1 as ::core::ffi::c_int
                                                {
                                                    1 as ::core::ffi::c_int
                                                } else {
                                                    0 as ::core::ffi::c_int
                                                };
                                                let mut hsub: ::core::ffi::c_int = if (*sps)
                                                    .chroma_format_idc == 1 as ::core::ffi::c_int
                                                    || (*sps).chroma_format_idc == 2 as ::core::ffi::c_int
                                                {
                                                    1 as ::core::ffi::c_int
                                                } else {
                                                    0 as ::core::ffi::c_int
                                                };
                                                let mut step_x: ::core::ffi::c_int = (1
                                                    as ::core::ffi::c_int) << hsub;
                                                let mut step_y: ::core::ffi::c_int = 2 as ::core::ffi::c_int
                                                    - (*sps).frame_mbs_only_flag << vsub;
                                                if crop_left
                                                    > (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                                                        .wrapping_div(4 as ::core::ffi::c_uint)
                                                        .wrapping_div(step_x as ::core::ffi::c_uint)
                                                    || crop_right
                                                        > (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                                                            .wrapping_div(4 as ::core::ffi::c_uint)
                                                            .wrapping_div(step_x as ::core::ffi::c_uint)
                                                    || crop_top
                                                        > (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                                                            .wrapping_div(4 as ::core::ffi::c_uint)
                                                            .wrapping_div(step_y as ::core::ffi::c_uint)
                                                    || crop_bottom
                                                        > (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                                                            .wrapping_div(4 as ::core::ffi::c_uint)
                                                            .wrapping_div(step_y as ::core::ffi::c_uint)
                                                    || crop_left
                                                        .wrapping_add(crop_right)
                                                        .wrapping_mul(step_x as ::core::ffi::c_uint)
                                                        >= width as ::core::ffi::c_uint
                                                    || crop_top
                                                        .wrapping_add(crop_bottom)
                                                        .wrapping_mul(step_y as ::core::ffi::c_uint)
                                                        >= height as ::core::ffi::c_uint
                                                {
                                                    av_log(
                                                        avctx as *mut ::core::ffi::c_void,
                                                        16 as ::core::ffi::c_int,
                                                        b"crop values invalid %d %d %d %d / %d %d\n\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        crop_left,
                                                        crop_right,
                                                        crop_top,
                                                        crop_bottom,
                                                        width,
                                                        height,
                                                    );
                                                    current_block = 2932924877315219625;
                                                } else {
                                                    (*sps).crop_left = crop_left
                                                        .wrapping_mul(step_x as ::core::ffi::c_uint);
                                                    (*sps).crop_right = crop_right
                                                        .wrapping_mul(step_x as ::core::ffi::c_uint);
                                                    (*sps).crop_top = crop_top
                                                        .wrapping_mul(step_y as ::core::ffi::c_uint);
                                                    (*sps).crop_bottom = crop_bottom
                                                        .wrapping_mul(step_y as ::core::ffi::c_uint);
                                                    current_block = 18425699056680496821;
                                                }
                                            }
                                        } else {
                                            (*sps).crop = 0 as ::core::ffi::c_int;
                                            (*sps).crop_bottom = (*sps).crop as ::core::ffi::c_uint;
                                            (*sps).crop_top = (*sps).crop_bottom;
                                            (*sps).crop_right = (*sps).crop_top;
                                            (*sps).crop_left = (*sps).crop_right;
                                            current_block = 18425699056680496821;
                                        }
                                        match current_block {
                                            2932924877315219625 => {}
                                            _ => {
                                                (*sps).vui.sar = AVRational {
                                                    num: 0 as ::core::ffi::c_int,
                                                    den: 1 as ::core::ffi::c_int,
                                                };
                                                (*sps).vui_parameters_present_flag = get_bits1(gb)
                                                    as ::core::ffi::c_int;
                                                if (*sps).vui_parameters_present_flag != 0 {
                                                    ret = decode_vui_parameters(
                                                        gb,
                                                        avctx as *mut ::core::ffi::c_void,
                                                        sps,
                                                    );
                                                    if ret < 0 as ::core::ffi::c_int {
                                                        current_block = 2932924877315219625;
                                                    } else {
                                                        current_block = 1765866445182206997;
                                                    }
                                                } else {
                                                    current_block = 1765866445182206997;
                                                }
                                                match current_block {
                                                    2932924877315219625 => {}
                                                    _ => {
                                                        if get_bits_left(gb) < 0 as ::core::ffi::c_int {
                                                            av_log_once(
                                                                avctx as *mut ::core::ffi::c_void,
                                                                if ignore_truncation != 0 {
                                                                    24 as ::core::ffi::c_int
                                                                } else {
                                                                    16 as ::core::ffi::c_int
                                                                },
                                                                48 as ::core::ffi::c_int,
                                                                (&raw mut (*ps).overread_warning_printed
                                                                    as *mut ::core::ffi::c_int)
                                                                    .offset((*sps).vui_parameters_present_flag as isize)
                                                                    as *mut ::core::ffi::c_int,
                                                                b"Overread %s by %d bits\n\0" as *const u8
                                                                    as *const ::core::ffi::c_char,
                                                                if (*sps).vui_parameters_present_flag != 0 {
                                                                    b"VUI\0" as *const u8 as *const ::core::ffi::c_char
                                                                } else {
                                                                    b"SPS\0" as *const u8 as *const ::core::ffi::c_char
                                                                },
                                                                -get_bits_left(gb),
                                                            );
                                                            if ignore_truncation == 0 {
                                                                current_block = 2932924877315219625;
                                                            } else {
                                                                current_block = 15447629348493591490;
                                                            }
                                                        } else {
                                                            current_block = 15447629348493591490;
                                                        }
                                                        match current_block {
                                                            2932924877315219625 => {}
                                                            _ => {
                                                                if (*sps).bitstream_restriction_flag == 0
                                                                    && ((*sps).ref_frame_count != 0
                                                                        || (*avctx).strict_std_compliance
                                                                            >= 1 as ::core::ffi::c_int)
                                                                {
                                                                    (*sps).num_reorder_frames = H264_MAX_DPB_FRAMES
                                                                        as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
                                                                    i = 0 as ::core::ffi::c_int;
                                                                    while (i as usize)
                                                                        < (::core::mem::size_of::<[[::core::ffi::c_int; 2]; 16]>()
                                                                            as usize)
                                                                            .wrapping_div(
                                                                                ::core::mem::size_of::<[::core::ffi::c_int; 2]>() as usize,
                                                                            )
                                                                    {
                                                                        if level_max_dpb_mbs[i
                                                                            as usize][0 as ::core::ffi::c_int as usize]
                                                                            == (*sps).level_idc
                                                                        {
                                                                            (*sps).num_reorder_frames = if level_max_dpb_mbs[i
                                                                                as usize][1 as ::core::ffi::c_int as usize]
                                                                                / ((*sps).mb_width * (*sps).mb_height)
                                                                                > (*sps).num_reorder_frames
                                                                            {
                                                                                (*sps).num_reorder_frames
                                                                            } else {
                                                                                level_max_dpb_mbs[i
                                                                                    as usize][1 as ::core::ffi::c_int as usize]
                                                                                    / ((*sps).mb_width * (*sps).mb_height)
                                                                            };
                                                                            break;
                                                                        } else {
                                                                            i += 1;
                                                                        }
                                                                    }
                                                                }
                                                                if (*avctx).debug & 1 as ::core::ffi::c_int != 0 {
                                                                    static mut csp: [[::core::ffi::c_char; 5]; 4] = unsafe {
                                                                        [
                                                                            ::core::mem::transmute::<
                                                                                [u8; 5],
                                                                                [::core::ffi::c_char; 5],
                                                                            >(*b"Gray\0"),
                                                                            ::core::mem::transmute::<
                                                                                [u8; 5],
                                                                                [::core::ffi::c_char; 5],
                                                                            >(*b"420\0\0"),
                                                                            ::core::mem::transmute::<
                                                                                [u8; 5],
                                                                                [::core::ffi::c_char; 5],
                                                                            >(*b"422\0\0"),
                                                                            ::core::mem::transmute::<
                                                                                [u8; 5],
                                                                                [::core::ffi::c_char; 5],
                                                                            >(*b"444\0\0"),
                                                                        ]
                                                                    };
                                                                    av_log(
                                                                        avctx as *mut ::core::ffi::c_void,
                                                                        48 as ::core::ffi::c_int,
                                                                        b"sps:%u profile:%d/%d poc:%d ref:%d %dx%d %s %s crop:%u/%u/%u/%u %s %s %d/%d b%d reo:%d\n\0"
                                                                            as *const u8 as *const ::core::ffi::c_char,
                                                                        sps_id,
                                                                        (*sps).profile_idc,
                                                                        (*sps).level_idc,
                                                                        (*sps).poc_type,
                                                                        (*sps).ref_frame_count,
                                                                        (*sps).mb_width,
                                                                        (*sps).mb_height,
                                                                        if (*sps).frame_mbs_only_flag != 0 {
                                                                            b"FRM\0" as *const u8 as *const ::core::ffi::c_char
                                                                        } else if (*sps).mb_aff != 0 {
                                                                            b"MB-AFF\0" as *const u8 as *const ::core::ffi::c_char
                                                                        } else {
                                                                            b"PIC-AFF\0" as *const u8 as *const ::core::ffi::c_char
                                                                        },
                                                                        if (*sps).direct_8x8_inference_flag != 0 {
                                                                            b"8B8\0" as *const u8 as *const ::core::ffi::c_char
                                                                        } else {
                                                                            b"\0" as *const u8 as *const ::core::ffi::c_char
                                                                        },
                                                                        (*sps).crop_left,
                                                                        (*sps).crop_right,
                                                                        (*sps).crop_top,
                                                                        (*sps).crop_bottom,
                                                                        if (*sps).vui_parameters_present_flag != 0 {
                                                                            b"VUI\0" as *const u8 as *const ::core::ffi::c_char
                                                                        } else {
                                                                            b"\0" as *const u8 as *const ::core::ffi::c_char
                                                                        },
                                                                        &raw const *(&raw const csp
                                                                            as *const [::core::ffi::c_char; 5])
                                                                            .offset((*sps).chroma_format_idc as isize)
                                                                            as *const ::core::ffi::c_char,
                                                                        if (*sps).timing_info_present_flag != 0 {
                                                                            (*sps).num_units_in_tick
                                                                        } else {
                                                                            0 as uint32_t
                                                                        },
                                                                        if (*sps).timing_info_present_flag != 0 {
                                                                            (*sps).time_scale
                                                                        } else {
                                                                            0 as uint32_t
                                                                        },
                                                                        (*sps).bit_depth_luma,
                                                                        if (*sps).bitstream_restriction_flag != 0 {
                                                                            (*sps).num_reorder_frames
                                                                        } else {
                                                                            -(1 as ::core::ffi::c_int)
                                                                        },
                                                                    );
                                                                }
                                                                if !(*ps).sps_list[sps_id as usize].is_null()
                                                                    && memcmp(
                                                                        (*ps).sps_list[sps_id as usize]
                                                                            as *const ::core::ffi::c_void,
                                                                        sps as *const ::core::ffi::c_void,
                                                                        ::core::mem::size_of::<SPS>() as size_t,
                                                                    ) == 0
                                                                {
                                                                    av_refstruct_unref(
                                                                        &raw mut sps as *mut ::core::ffi::c_void,
                                                                    );
                                                                } else {
                                                                    remove_sps(ps, sps_id as ::core::ffi::c_int);
                                                                    (*ps).sps_list[sps_id as usize] = sps;
                                                                }
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
                }
            }
        }
    }
    av_refstruct_unref(&raw mut sps as *mut ::core::ffi::c_void);
    return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
        | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
        | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
        as ::core::ffi::c_int);
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
unsafe extern "C" fn show_bits1(mut s: *mut GetBitContext) -> ::core::ffi::c_uint {
    return show_bits(s, 1 as ::core::ffi::c_int);
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
unsafe extern "C" fn init_dequant8_coeff_table(mut pps: *mut PPS, mut sps: *const SPS) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut q: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let max_qp: ::core::ffi::c_int = 51 as ::core::ffi::c_int
        + 6 as ::core::ffi::c_int * ((*sps).bit_depth_luma - 8 as ::core::ffi::c_int);
    i = 0 as ::core::ffi::c_int;
    while i < 6 as ::core::ffi::c_int {
        (*pps).dequant8_coeff[i as usize] = &raw mut *(&raw mut (*pps).dequant8_buffer
            as *mut [[uint32_t; 64]; 88])
            .offset(i as isize) as *mut [uint32_t; 64] as *mut [uint32_t; 64];
        j = 0 as ::core::ffi::c_int;
        while j < i {
            if memcmp(
                &raw mut *(&raw mut (*pps).scaling_matrix8 as *mut [uint8_t; 64])
                    .offset(j as isize) as *mut uint8_t as *const ::core::ffi::c_void,
                &raw mut *(&raw mut (*pps).scaling_matrix8 as *mut [uint8_t; 64])
                    .offset(i as isize) as *mut uint8_t as *const ::core::ffi::c_void,
                (64 as size_t).wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
            ) == 0
            {
                (*pps).dequant8_coeff[i as usize] = &raw mut *(&raw mut (*pps)
                    .dequant8_buffer as *mut [[uint32_t; 64]; 88])
                    .offset(j as isize) as *mut [uint32_t; 64] as *mut [uint32_t; 64];
                break;
            } else {
                j += 1;
            }
        }
        if !(j < i) {
            q = 0 as ::core::ffi::c_int;
            while q < max_qp + 1 as ::core::ffi::c_int {
                let mut shift: ::core::ffi::c_int = ff_h264_quant_div6[q as usize]
                    as ::core::ffi::c_int;
                let mut idx: ::core::ffi::c_int = ff_h264_quant_rem6[q as usize]
                    as ::core::ffi::c_int;
                x = 0 as ::core::ffi::c_int;
                while x < 64 as ::core::ffi::c_int {
                    (*(*pps)
                        .dequant8_coeff[i as usize]
                        .offset(
                            q as isize,
                        ))[(x >> 3 as ::core::ffi::c_int
                        | (x & 7 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                        as usize] = (ff_h264_dequant8_coeff_init[idx
                        as usize][ff_h264_dequant8_coeff_init_scan[(x
                        >> 1 as ::core::ffi::c_int & 12 as ::core::ffi::c_int
                        | x & 3 as ::core::ffi::c_int) as usize] as usize] as uint32_t)
                        .wrapping_mul(
                            (*pps).scaling_matrix8[i as usize][x as usize] as uint32_t,
                        ) << shift;
                    x += 1;
                }
                q += 1;
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn init_dequant4_coeff_table(mut pps: *mut PPS, mut sps: *const SPS) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut q: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let max_qp: ::core::ffi::c_int = 51 as ::core::ffi::c_int
        + 6 as ::core::ffi::c_int * ((*sps).bit_depth_luma - 8 as ::core::ffi::c_int);
    i = 0 as ::core::ffi::c_int;
    while i < 6 as ::core::ffi::c_int {
        (*pps).dequant4_coeff[i as usize] = &raw mut *(&raw mut (*pps).dequant4_buffer
            as *mut [[uint32_t; 16]; 88])
            .offset(i as isize) as *mut [uint32_t; 16] as *mut [uint32_t; 16];
        j = 0 as ::core::ffi::c_int;
        while j < i {
            if memcmp(
                &raw mut *(&raw mut (*pps).scaling_matrix4 as *mut [uint8_t; 16])
                    .offset(j as isize) as *mut uint8_t as *const ::core::ffi::c_void,
                &raw mut *(&raw mut (*pps).scaling_matrix4 as *mut [uint8_t; 16])
                    .offset(i as isize) as *mut uint8_t as *const ::core::ffi::c_void,
                (16 as size_t).wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
            ) == 0
            {
                (*pps).dequant4_coeff[i as usize] = &raw mut *(&raw mut (*pps)
                    .dequant4_buffer as *mut [[uint32_t; 16]; 88])
                    .offset(j as isize) as *mut [uint32_t; 16] as *mut [uint32_t; 16];
                break;
            } else {
                j += 1;
            }
        }
        if !(j < i) {
            q = 0 as ::core::ffi::c_int;
            while q < max_qp + 1 as ::core::ffi::c_int {
                let mut shift: ::core::ffi::c_int = ff_h264_quant_div6[q as usize]
                    as ::core::ffi::c_int + 2 as ::core::ffi::c_int;
                let mut idx: ::core::ffi::c_int = ff_h264_quant_rem6[q as usize]
                    as ::core::ffi::c_int;
                x = 0 as ::core::ffi::c_int;
                while x < 16 as ::core::ffi::c_int {
                    (*(*pps)
                        .dequant4_coeff[i as usize]
                        .offset(
                            q as isize,
                        ))[(x >> 2 as ::core::ffi::c_int
                        | x << 2 as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                        as usize] = (ff_h264_dequant4_coeff_init[idx
                        as usize][((x & 1 as ::core::ffi::c_int)
                        + (x >> 2 as ::core::ffi::c_int & 1 as ::core::ffi::c_int))
                        as usize] as uint32_t)
                        .wrapping_mul(
                            (*pps).scaling_matrix4[i as usize][x as usize] as uint32_t,
                        ) << shift;
                    x += 1;
                }
                q += 1;
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn init_dequant_tables(mut pps: *mut PPS, mut sps: *const SPS) {
    let mut i: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    init_dequant4_coeff_table(pps, sps);
    memset(
        &raw mut (*pps).dequant8_coeff as *mut *mut [uint32_t; 64]
            as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[*mut [uint32_t; 64]; 6]>() as size_t,
    );
    if (*pps).transform_8x8_mode != 0 {
        init_dequant8_coeff_table(pps, sps);
    }
    if (*sps).transform_bypass != 0 {
        i = 0 as ::core::ffi::c_int;
        while i < 6 as ::core::ffi::c_int {
            x = 0 as ::core::ffi::c_int;
            while x < 16 as ::core::ffi::c_int {
                (*(*pps)
                    .dequant4_coeff[i as usize]
                    .offset(0 as ::core::ffi::c_int as isize))[x as usize] = ((1
                    as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as uint32_t;
                x += 1;
            }
            i += 1;
        }
        if (*pps).transform_8x8_mode != 0 {
            i = 0 as ::core::ffi::c_int;
            while i < 6 as ::core::ffi::c_int {
                x = 0 as ::core::ffi::c_int;
                while x < 64 as ::core::ffi::c_int {
                    (*(*pps)
                        .dequant8_coeff[i as usize]
                        .offset(0 as ::core::ffi::c_int as isize))[x as usize] = ((1
                        as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as uint32_t;
                    x += 1;
                }
                i += 1;
            }
        }
    }
}
unsafe extern "C" fn build_qp_table(
    mut pps: *mut PPS,
    mut t: ::core::ffi::c_int,
    mut index: ::core::ffi::c_int,
    depth: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let max_qp: ::core::ffi::c_int = 51 as ::core::ffi::c_int
        + 6 as ::core::ffi::c_int * (depth - 8 as ::core::ffi::c_int);
    i = 0 as ::core::ffi::c_int;
    while i < max_qp + 1 as ::core::ffi::c_int {
        (*pps).chroma_qp_table[t as usize][i as usize] = ff_h264_chroma_qp[(depth
            - 8 as ::core::ffi::c_int)
            as usize][av_clip_c(i + index, 0 as ::core::ffi::c_int, max_qp) as usize];
        i += 1;
    }
}
unsafe extern "C" fn more_rbsp_data_in_pps(
    mut sps: *const SPS,
    mut logctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut profile_idc: ::core::ffi::c_int = (*sps).profile_idc;
    if (profile_idc == 66 as ::core::ffi::c_int
        || profile_idc == 77 as ::core::ffi::c_int
        || profile_idc == 88 as ::core::ffi::c_int)
        && (*sps).constraint_set_flags & 7 as ::core::ffi::c_int != 0
    {
        av_log(
            logctx,
            40 as ::core::ffi::c_int,
            b"Current profile doesn't provide more RBSP data in PPS, skipping\n\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn get_bits_left(mut gb: *mut GetBitContext) -> ::core::ffi::c_int {
    return (*gb).size_in_bits - get_bits_count(gb);
}
unsafe extern "C" fn pps_free(
    mut unused: AVRefStructOpaque,
    mut obj: *mut ::core::ffi::c_void,
) {
    let mut pps: *mut PPS = obj as *mut PPS;
    av_refstruct_unref(&raw mut (*pps).sps as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ff_h264_decode_picture_parameter_set(
    mut gb: *mut GetBitContext,
    mut avctx: *mut AVCodecContext,
    mut ps: *mut H264ParamSets,
    mut bit_length: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut sps: *const SPS = ::core::ptr::null::<SPS>();
    let mut pps_id: ::core::ffi::c_uint = get_ue_golomb(gb) as ::core::ffi::c_uint;
    let mut pps: *mut PPS = ::core::ptr::null_mut::<PPS>();
    let mut qp_bd_offset: ::core::ffi::c_int = 0;
    let mut bits_left: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    if pps_id >= 256 as ::core::ffi::c_uint {
        av_log(
            avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"pps_id %u out of range\n\0" as *const u8 as *const ::core::ffi::c_char,
            pps_id,
        );
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    pps = av_refstruct_alloc_ext(
        ::core::mem::size_of::<PPS>() as size_t,
        0 as ::core::ffi::c_uint,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        Some(
            pps_free
                as unsafe extern "C" fn(
                    AVRefStructOpaque,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
    ) as *mut PPS;
    if pps.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    (*pps).data_size = get_bits_bytesize(gb, 1 as ::core::ffi::c_int) as size_t;
    if (*pps).data_size as usize > ::core::mem::size_of::<[uint8_t; 4096]>() as usize {
        av_log(
            avctx as *mut ::core::ffi::c_void,
            48 as ::core::ffi::c_int,
            b"Truncating likely oversized PPS (%zu > %zu)\n\0" as *const u8
                as *const ::core::ffi::c_char,
            (*pps).data_size,
            ::core::mem::size_of::<[uint8_t; 4096]>(),
        );
        (*pps).data_size = ::core::mem::size_of::<[uint8_t; 4096]>() as usize as size_t;
    }
    memcpy(
        &raw mut (*pps).data as *mut uint8_t as *mut ::core::ffi::c_void,
        (*gb).buffer as *const ::core::ffi::c_void,
        (*pps).data_size,
    );
    if bit_length & 7 as ::core::ffi::c_int == 0
        && ((*pps).data_size as usize)
            < ::core::mem::size_of::<[uint8_t; 4096]>() as usize
    {
        let fresh2 = (*pps).data_size;
        (*pps).data_size = (*pps).data_size.wrapping_add(1);
        (*pps).data[fresh2 as usize] = 0x80 as uint8_t;
    }
    (*pps).pps_id = pps_id;
    (*pps).sps_id = get_ue_golomb_31(gb) as ::core::ffi::c_uint;
    if (*pps).sps_id >= 32 as ::core::ffi::c_uint
        || (*ps).sps_list[(*pps).sps_id as usize].is_null()
    {
        av_log(
            avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"sps_id %u out of range\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*pps).sps_id,
        );
        ret = -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    } else {
        (*pps).sps = av_refstruct_ref_c(
            (*ps).sps_list[(*pps).sps_id as usize] as *const ::core::ffi::c_void,
        ) as *const SPS;
        sps = (*pps).sps;
        if (*sps).bit_depth_luma > 14 as ::core::ffi::c_int {
            av_log(
                avctx as *mut ::core::ffi::c_void,
                16 as ::core::ffi::c_int,
                b"Invalid luma bit depth=%d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*sps).bit_depth_luma,
            );
            ret = -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        } else if (*sps).bit_depth_luma == 11 as ::core::ffi::c_int
            || (*sps).bit_depth_luma == 13 as ::core::ffi::c_int
        {
            avpriv_report_missing_feature(
                avctx as *mut ::core::ffi::c_void,
                b"Unimplemented luma bit depth=%d\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*sps).bit_depth_luma,
            );
            ret = -((('P' as i32 | ('A' as i32) << 8 as ::core::ffi::c_int
                | ('W' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('E' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        } else {
            (*pps).cabac = get_bits1(gb) as ::core::ffi::c_int;
            (*pps).pic_order_present = get_bits1(gb) as ::core::ffi::c_int;
            (*pps).slice_group_count = get_ue_golomb(gb) + 1 as ::core::ffi::c_int;
            if (*pps).slice_group_count > 1 as ::core::ffi::c_int {
                (*pps).mb_slice_group_map_type = get_ue_golomb(gb);
                avpriv_report_missing_feature(
                    avctx as *mut ::core::ffi::c_void,
                    b"FMO\0" as *const u8 as *const ::core::ffi::c_char,
                );
                ret = -((('P' as i32 | ('A' as i32) << 8 as ::core::ffi::c_int
                    | ('W' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('E' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
            } else {
                (*pps).ref_count[0 as ::core::ffi::c_int as usize] = (get_ue_golomb(gb)
                    + 1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
                (*pps).ref_count[1 as ::core::ffi::c_int as usize] = (get_ue_golomb(gb)
                    + 1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
                if (*pps)
                    .ref_count[0 as ::core::ffi::c_int as usize]
                    .wrapping_sub(1 as ::core::ffi::c_uint)
                    > (32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint
                    || (*pps)
                        .ref_count[1 as ::core::ffi::c_int as usize]
                        .wrapping_sub(1 as ::core::ffi::c_uint)
                        > (32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                            as ::core::ffi::c_uint
                {
                    av_log(
                        avctx as *mut ::core::ffi::c_void,
                        16 as ::core::ffi::c_int,
                        b"reference overflow (pps)\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    ret = -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                        | ('D' as i32) << 16 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint
                        | ('A' as i32 as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                } else {
                    qp_bd_offset = 6 as ::core::ffi::c_int
                        * ((*sps).bit_depth_luma - 8 as ::core::ffi::c_int);
                    (*pps).weighted_pred = get_bits1(gb) as ::core::ffi::c_int;
                    (*pps).weighted_bipred_idc = get_bits(gb, 2 as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                    (*pps).init_qp = (get_se_golomb(gb) as ::core::ffi::c_uint)
                        .wrapping_add(26 as ::core::ffi::c_uint)
                        .wrapping_add(qp_bd_offset as ::core::ffi::c_uint)
                        as ::core::ffi::c_int;
                    (*pps).init_qs = (get_se_golomb(gb) as ::core::ffi::c_uint)
                        .wrapping_add(26 as ::core::ffi::c_uint)
                        .wrapping_add(qp_bd_offset as ::core::ffi::c_uint)
                        as ::core::ffi::c_int;
                    (*pps).chroma_qp_index_offset[0 as ::core::ffi::c_int as usize] = get_se_golomb(
                        gb,
                    );
                    if (*pps).chroma_qp_index_offset[0 as ::core::ffi::c_int as usize]
                        < -(12 as ::core::ffi::c_int)
                        || (*pps)
                            .chroma_qp_index_offset[0 as ::core::ffi::c_int as usize]
                            > 12 as ::core::ffi::c_int
                    {
                        ret = -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                            | ('D' as i32) << 16 as ::core::ffi::c_int)
                            as ::core::ffi::c_uint
                            | ('A' as i32 as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                    } else {
                        (*pps).deblocking_filter_parameters_present = get_bits1(gb)
                            as ::core::ffi::c_int;
                        (*pps).constrained_intra_pred = get_bits1(gb)
                            as ::core::ffi::c_int;
                        (*pps).redundant_pic_cnt_present = get_bits1(gb)
                            as ::core::ffi::c_int;
                        (*pps).transform_8x8_mode = 0 as ::core::ffi::c_int;
                        memcpy(
                            &raw mut (*pps).scaling_matrix4 as *mut [uint8_t; 16]
                                as *mut ::core::ffi::c_void,
                            &raw const (*sps).scaling_matrix4 as *const [uint8_t; 16]
                                as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<[[uint8_t; 16]; 6]>() as size_t,
                        );
                        memcpy(
                            &raw mut (*pps).scaling_matrix8 as *mut [uint8_t; 64]
                                as *mut ::core::ffi::c_void,
                            &raw const (*sps).scaling_matrix8 as *const [uint8_t; 64]
                                as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<[[uint8_t; 64]; 6]>() as size_t,
                        );
                        bits_left = bit_length - get_bits_count(gb);
                        if bits_left > 0 as ::core::ffi::c_int
                            && more_rbsp_data_in_pps(
                                sps,
                                avctx as *mut ::core::ffi::c_void,
                            ) != 0
                        {
                            (*pps).transform_8x8_mode = get_bits1(gb)
                                as ::core::ffi::c_int;
                            (*pps).pic_scaling_matrix_present_flag = get_bits1(gb)
                                as ::core::ffi::c_int;
                            ret = decode_scaling_matrices(
                                gb,
                                sps,
                                pps,
                                0 as ::core::ffi::c_int,
                                (*pps).pic_scaling_matrix_present_flag,
                                &raw mut (*pps).pic_scaling_matrix_present_mask,
                                &raw mut (*pps).scaling_matrix4 as *mut [uint8_t; 16],
                                &raw mut (*pps).scaling_matrix8 as *mut [uint8_t; 64],
                            );
                            if ret < 0 as ::core::ffi::c_int {
                                current_block = 16648951783327420416;
                            } else {
                                (*pps)
                                    .chroma_qp_index_offset[1 as ::core::ffi::c_int as usize] = get_se_golomb(
                                    gb,
                                );
                                if (*pps)
                                    .chroma_qp_index_offset[1 as ::core::ffi::c_int as usize]
                                    < -(12 as ::core::ffi::c_int)
                                    || (*pps)
                                        .chroma_qp_index_offset[1 as ::core::ffi::c_int as usize]
                                        > 12 as ::core::ffi::c_int
                                {
                                    ret = -((('I' as i32
                                        | ('N' as i32) << 8 as ::core::ffi::c_int
                                        | ('D' as i32) << 16 as ::core::ffi::c_int)
                                        as ::core::ffi::c_uint
                                        | ('A' as i32 as ::core::ffi::c_uint)
                                            << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                                    current_block = 16648951783327420416;
                                } else {
                                    current_block = 13678349939556791712;
                                }
                            }
                        } else {
                            (*pps)
                                .chroma_qp_index_offset[1 as ::core::ffi::c_int as usize] = (*pps)
                                .chroma_qp_index_offset[0 as ::core::ffi::c_int as usize];
                            current_block = 13678349939556791712;
                        }
                        match current_block {
                            16648951783327420416 => {}
                            _ => {
                                build_qp_table(
                                    pps,
                                    0 as ::core::ffi::c_int,
                                    (*pps)
                                        .chroma_qp_index_offset[0 as ::core::ffi::c_int as usize],
                                    (*sps).bit_depth_luma,
                                );
                                build_qp_table(
                                    pps,
                                    1 as ::core::ffi::c_int,
                                    (*pps)
                                        .chroma_qp_index_offset[1 as ::core::ffi::c_int as usize],
                                    (*sps).bit_depth_luma,
                                );
                                init_dequant_tables(pps, sps);
                                if (*pps)
                                    .chroma_qp_index_offset[0 as ::core::ffi::c_int as usize]
                                    != (*pps)
                                        .chroma_qp_index_offset[1 as ::core::ffi::c_int as usize]
                                {
                                    (*pps).chroma_qp_diff = 1 as ::core::ffi::c_int;
                                }
                                if (*avctx).debug & 1 as ::core::ffi::c_int != 0 {
                                    av_log(
                                        avctx as *mut ::core::ffi::c_void,
                                        48 as ::core::ffi::c_int,
                                        b"pps:%u sps:%u %s slice_groups:%d ref:%u/%u %s qp:%d/%d/%d/%d %s %s %s %s\n\0"
                                            as *const u8 as *const ::core::ffi::c_char,
                                        pps_id,
                                        (*pps).sps_id,
                                        if (*pps).cabac != 0 {
                                            b"CABAC\0" as *const u8 as *const ::core::ffi::c_char
                                        } else {
                                            b"CAVLC\0" as *const u8 as *const ::core::ffi::c_char
                                        },
                                        (*pps).slice_group_count,
                                        (*pps).ref_count[0 as ::core::ffi::c_int as usize],
                                        (*pps).ref_count[1 as ::core::ffi::c_int as usize],
                                        if (*pps).weighted_pred != 0 {
                                            b"weighted\0" as *const u8 as *const ::core::ffi::c_char
                                        } else {
                                            b"\0" as *const u8 as *const ::core::ffi::c_char
                                        },
                                        (*pps).init_qp,
                                        (*pps).init_qs,
                                        (*pps)
                                            .chroma_qp_index_offset[0 as ::core::ffi::c_int as usize],
                                        (*pps)
                                            .chroma_qp_index_offset[1 as ::core::ffi::c_int as usize],
                                        if (*pps).deblocking_filter_parameters_present != 0 {
                                            b"LPAR\0" as *const u8 as *const ::core::ffi::c_char
                                        } else {
                                            b"\0" as *const u8 as *const ::core::ffi::c_char
                                        },
                                        if (*pps).constrained_intra_pred != 0 {
                                            b"CONSTR\0" as *const u8 as *const ::core::ffi::c_char
                                        } else {
                                            b"\0" as *const u8 as *const ::core::ffi::c_char
                                        },
                                        if (*pps).redundant_pic_cnt_present != 0 {
                                            b"REDU\0" as *const u8 as *const ::core::ffi::c_char
                                        } else {
                                            b"\0" as *const u8 as *const ::core::ffi::c_char
                                        },
                                        if (*pps).transform_8x8_mode != 0 {
                                            b"8x8DCT\0" as *const u8 as *const ::core::ffi::c_char
                                        } else {
                                            b"\0" as *const u8 as *const ::core::ffi::c_char
                                        },
                                    );
                                }
                                remove_pps(ps, pps_id as ::core::ffi::c_int);
                                (*ps).pps_list[pps_id as usize] = pps;
                                return 0 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
        }
    }
    av_refstruct_unref(&raw mut pps as *mut ::core::ffi::c_void);
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
