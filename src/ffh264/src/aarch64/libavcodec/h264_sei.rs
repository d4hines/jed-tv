// Translated from FFmpeg 9.0.1 libavcodec/h264_sei.c
// (preprocessed for aarch64-unknown-linux-musl) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// H.26L/H.264/AVC/JVT/14496-10/... SEI decoding
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
    static ff_golomb_vlc_len: [uint8_t; 512];
    static ff_ue_golomb_vlc_code: [uint8_t; 512];
    fn abort() -> !;
    fn ff_h2645_sei_message_decode(
        h: *mut H2645SEI,
        type_0: SEIType,
        codec_id: AVCodecID,
        gb: *mut GetBitContext,
        gbyte: *mut GetByteContext,
        logctx: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn ff_h2645_sei_reset(s: *mut H2645SEI);
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
}
pub type size_t = ::core::ffi::c_ulong;
pub type int16_t = ::core::ffi::c_short;
pub type int32_t = ::core::ffi::c_int;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint16_t = ::core::ffi::c_ushort;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint64_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: ::core::ffi::c_int,
    pub den: ::core::ffi::c_int,
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
#[repr(C, packed)]
pub union unaligned_16 {
    pub l: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GetByteContext {
    pub buffer: *const uint8_t,
    pub buffer_end: *const uint8_t,
    pub buffer_start: *const uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GetBitContext {
    pub buffer: *const uint8_t,
    pub index: ::core::ffi::c_int,
    pub size_in_bits: ::core::ffi::c_int,
    pub size_in_bits_plus8: ::core::ffi::c_int,
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
pub type AVChromaLocation = ::core::ffi::c_uint;
pub const AVCHROMA_LOC_NB: AVChromaLocation = 7;
pub const AVCHROMA_LOC_BOTTOM: AVChromaLocation = 6;
pub const AVCHROMA_LOC_BOTTOMLEFT: AVChromaLocation = 5;
pub const AVCHROMA_LOC_TOP: AVChromaLocation = 4;
pub const AVCHROMA_LOC_TOPLEFT: AVChromaLocation = 3;
pub const AVCHROMA_LOC_CENTER: AVChromaLocation = 2;
pub const AVCHROMA_LOC_LEFT: AVChromaLocation = 1;
pub const AVCHROMA_LOC_UNSPECIFIED: AVChromaLocation = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBufferRef {
    pub buffer: *mut AVBuffer,
    pub data: *mut uint8_t,
    pub size: size_t,
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
pub type SEIType = ::core::ffi::c_uint;
pub const SEI_TYPE_SAMPLE_ASPECT_RATIO_INFO: SEIType = 204;
pub const SEI_TYPE_SUBPIC_LEVEL_INFO: SEIType = 203;
pub const SEI_TYPE_ANNOTATED_REGIONS: SEIType = 202;
pub const SEI_TYPE_SEI_PREFIX_INDICATION: SEIType = 201;
pub const SEI_TYPE_SEI_MANIFEST: SEIType = 200;
pub const SEI_TYPE_ALTERNATIVE_DEPTH_INFO: SEIType = 181;
pub const SEI_TYPE_MULTIVIEW_VIEW_POSITION_5: SEIType = 180;
pub const SEI_TYPE_MULTIVIEW_ACQUISITION_INFO_5: SEIType = 179;
pub const SEI_TYPE_MULTIVIEW_SCENE_INFO_5: SEIType = 178;
pub const SEI_TYPE_DEPTH_REPRESENTATION_INFO_5: SEIType = 177;
pub const SEI_TYPE_THREE_DIMENSIONAL_REFERENCE_DISPLAYS_INFO: SEIType = 176;
pub const SEI_TYPE_FRAME_FIELD_INFO: SEIType = 168;
pub const SEI_TYPE_TEMPORAL_MV_PREDICTION_CONSTRAINTS: SEIType = 167;
pub const SEI_TYPE_OVERLAY_INFO: SEIType = 166;
pub const SEI_TYPE_ALPHA_CHANNEL_INFO: SEIType = 165;
pub const SEI_TYPE_SUB_BITSTREAM_PROPERTY: SEIType = 164;
pub const SEI_TYPE_BSP_INITIAL_ARRIVAL_TIME: SEIType = 163;
pub const SEI_TYPE_BSP_NESTING: SEIType = 162;
pub const SEI_TYPE_INTER_LAYER_CONSTRAINED_TILE_SETS: SEIType = 161;
pub const SEI_TYPE_LAYERS_NOT_PRESENT_5: SEIType = 160;
pub const SEI_TYPE_MCTS_EXTRACTION_INFO_NESTING: SEIType = 159;
pub const SEI_TYPE_MCTS_EXTRACTION_INFO_SETS: SEIType = 158;
pub const SEI_TYPE_REGIONAL_NESTING: SEIType = 157;
pub const SEI_TYPE_OMNI_VIEWPORT: SEIType = 156;
pub const SEI_TYPE_REGIONWISE_PACKING: SEIType = 155;
pub const SEI_TYPE_SPHERE_ROTATION: SEIType = 154;
pub const SEI_TYPE_FISHEYE_VIDEO_INFO: SEIType = 152;
pub const SEI_TYPE_CUBEMAP_PROJECTION: SEIType = 151;
pub const SEI_TYPE_EQUIRECTANGULAR_PROJECTION: SEIType = 150;
pub const SEI_TYPE_CONTENT_COLOUR_VOLUME: SEIType = 149;
pub const SEI_TYPE_AMBIENT_VIEWING_ENVIRONMENT: SEIType = 148;
pub const SEI_TYPE_ALTERNATIVE_TRANSFER_CHARACTERISTICS: SEIType = 147;
pub const SEI_TYPE_CODED_REGION_COMPLETION: SEIType = 146;
pub const SEI_TYPE_DEPENDENT_RAP_INDICATION: SEIType = 145;
pub const SEI_TYPE_CONTENT_LIGHT_LEVEL_INFO: SEIType = 144;
pub const SEI_TYPE_DEINTERLACED_FIELD_IDENTIFICATION: SEIType = 143;
pub const SEI_TYPE_COLOUR_REMAPPING_INFO: SEIType = 142;
pub const SEI_TYPE_KNEE_FUNCTION_INFO: SEIType = 141;
pub const SEI_TYPE_CHROMA_RESAMPLING_FILTER_HINT: SEIType = 140;
pub const SEI_TYPE_TEMPORAL_MOTION_CONSTRAINED_TILE_SETS: SEIType = 139;
pub const SEI_TYPE_SEGMENTED_RECT_FRAME_PACKING_ARRANGEMENT: SEIType = 138;
pub const SEI_TYPE_MASTERING_DISPLAY_COLOUR_VOLUME: SEIType = 137;
pub const SEI_TYPE_TIME_CODE: SEIType = 136;
pub const SEI_TYPE_NO_DISPLAY: SEIType = 135;
pub const SEI_TYPE_REGION_REFRESH_INFO: SEIType = 134;
pub const SEI_TYPE_SCALABLE_NESTING_5: SEIType = 133;
pub const SEI_TYPE_DECODED_PICTURE_HASH: SEIType = 132;
pub const SEI_TYPE_TEMPORAL_SUB_LAYER_ZERO_IDX: SEIType = 131;
pub const SEI_TYPE_DECODING_UNIT_INFO: SEIType = 130;
pub const SEI_TYPE_PARAMETER_SETS_INCLUSION_INDICATION: SEIType = 129;
pub const SEI_TYPE_ACTIVE_PARAMETER_SETS: SEIType = 129;
pub const SEI_TYPE_STRUCTURE_OF_PICTURES_INFO: SEIType = 128;
pub const SEI_TYPE_GREEN_METADATA: SEIType = 56;
pub const SEI_TYPE_CONSTRAINED_DEPTH_PARAMETER_SET_IDENTIFIER: SEIType = 54;
pub const SEI_TYPE_DEPTH_SAMPLING_INFO: SEIType = 53;
pub const SEI_TYPE_DEPTH_TIMING: SEIType = 52;
pub const SEI_TYPE_THREE_DIMENSIONAL_REFERENCE_DISPLAYS_INFO_4: SEIType = 51;
pub const SEI_TYPE_DEPTH_REPRESENTATION_INFO_4: SEIType = 50;
pub const SEI_TYPE_MVCD_VIEW_SCALABILITY_INFO: SEIType = 49;
pub const SEI_TYPE_MVCD_SCALABLE_NESTING: SEIType = 48;
pub const SEI_TYPE_DISPLAY_ORIENTATION: SEIType = 47;
pub const SEI_TYPE_MULTIVIEW_VIEW_POSITION_4: SEIType = 46;
pub const SEI_TYPE_FRAME_PACKING_ARRANGEMENT: SEIType = 45;
pub const SEI_TYPE_BASE_VIEW_TEMPORAL_HRD: SEIType = 44;
pub const SEI_TYPE_OPERATION_POINTS_NOT_PRESENT: SEIType = 43;
pub const SEI_TYPE_VIEW_DEPENDENCY_CHANGE: SEIType = 42;
pub const SEI_TYPE_NON_REQUIRED_VIEW_COMPONENT: SEIType = 41;
pub const SEI_TYPE_MULTIVIEW_ACQUISITION_INFO_4: SEIType = 40;
pub const SEI_TYPE_MULTIVIEW_SCENE_INFO_4: SEIType = 39;
pub const SEI_TYPE_VIEW_SCALABILITY_INFO: SEIType = 38;
pub const SEI_TYPE_MVC_SCALABLE_NESTING: SEIType = 37;
pub const SEI_TYPE_PARALLEL_DECODING_INFO: SEIType = 36;
pub const SEI_TYPE_TL_SWITCHING_POINT: SEIType = 35;
pub const SEI_TYPE_TL0_DEP_REP_INDEX: SEIType = 34;
pub const SEI_TYPE_REDUNDANT_PIC_PROPERTY: SEIType = 33;
pub const SEI_TYPE_QUALITY_LAYER_INTEGRITY_CHECK: SEIType = 32;
pub const SEI_TYPE_BASE_LAYER_TEMPORAL_HRD: SEIType = 31;
pub const SEI_TYPE_SCALABLE_NESTING_4: SEIType = 30;
pub const SEI_TYPE_LAYER_DEPENDENCY_CHANGE: SEIType = 29;
pub const SEI_TYPE_LAYERS_NOT_PRESENT_4: SEIType = 28;
pub const SEI_TYPE_PRIORITY_LAYER_INFO: SEIType = 27;
pub const SEI_TYPE_NON_REQUIRED_LAYER_REP: SEIType = 26;
pub const SEI_TYPE_SUB_PIC_SCALABLE_LAYER: SEIType = 25;
pub const SEI_TYPE_SCALABILITY_INFO: SEIType = 24;
pub const SEI_TYPE_TONE_MAPPING_INFO: SEIType = 23;
pub const SEI_TYPE_POST_FILTER_HINT: SEIType = 22;
pub const SEI_TYPE_STEREO_VIDEO_INFO: SEIType = 21;
pub const SEI_TYPE_DEBLOCKING_FILTER_DISPLAY_PREFERENCE: SEIType = 20;
pub const SEI_TYPE_FILM_GRAIN_CHARACTERISTICS: SEIType = 19;
pub const SEI_TYPE_MOTION_CONSTRAINED_SLICE_GROUP_SET: SEIType = 18;
pub const SEI_TYPE_PROGRESSIVE_REFINEMENT_SEGMENT_END: SEIType = 17;
pub const SEI_TYPE_PROGRESSIVE_REFINEMENT_SEGMENT_START: SEIType = 16;
pub const SEI_TYPE_FULL_FRAME_SNAPSHOT: SEIType = 15;
pub const SEI_TYPE_FULL_FRAME_FREEZE_RELEASE: SEIType = 14;
pub const SEI_TYPE_FULL_FRAME_FREEZE: SEIType = 13;
pub const SEI_TYPE_SUB_SEQ_CHARACTERISTICS: SEIType = 12;
pub const SEI_TYPE_SUB_SEQ_LAYER_CHARACTERISTICS: SEIType = 11;
pub const SEI_TYPE_SUB_SEQ_INFO: SEIType = 10;
pub const SEI_TYPE_SCENE_INFO: SEIType = 9;
pub const SEI_TYPE_SPARE_PIC: SEIType = 8;
pub const SEI_TYPE_DEC_REF_PIC_MARKING_REPETITION: SEIType = 7;
pub const SEI_TYPE_RECOVERY_POINT: SEIType = 6;
pub const SEI_TYPE_USER_DATA_UNREGISTERED: SEIType = 5;
pub const SEI_TYPE_USER_DATA_REGISTERED_ITU_T_T35: SEIType = 4;
pub const SEI_TYPE_FILLER_PAYLOAD: SEIType = 3;
pub const SEI_TYPE_PAN_SCAN_RECT: SEIType = 2;
pub const SEI_TYPE_PIC_TIMING: SEIType = 1;
pub const SEI_TYPE_BUFFERING_PERIOD: SEIType = 0;
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
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const FF_H2645_SEI_MESSAGE_UNHANDLED: C2RustUnnamed = 1;
pub const FF_H2645_SEI_MESSAGE_HANDLED: C2RustUnnamed = 0;
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
static mut sei_num_clock_ts_table: [uint8_t; 9] = [
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
pub unsafe extern "C" fn ff_h264_sei_uninit(mut h: *mut H264SEIContext) {
    (*h).recovery_point.recovery_frame_cnt = -(1 as ::core::ffi::c_int);
    (*h).picture_timing.dpb_output_delay = 0 as ::core::ffi::c_int;
    (*h).picture_timing.cpb_removal_delay = -(1 as ::core::ffi::c_int);
    (*h).picture_timing.present = 0 as ::core::ffi::c_int;
    (*h).buffering_period.present = 0 as ::core::ffi::c_int;
    (*h).common.frame_packing.present = 0 as ::core::ffi::c_int;
    (*h).common.display_orientation.present = 0 as ::core::ffi::c_int;
    ff_h2645_sei_reset(&raw mut (*h).common);
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
pub unsafe extern "C" fn ff_h264_sei_process_picture_timing(
    mut h: *mut H264SEIPictureTiming,
    mut sps: *const SPS,
    mut logctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut gb: GetBitContext = GetBitContext {
        buffer: ::core::ptr::null::<uint8_t>(),
        index: 0,
        size_in_bits: 0,
        size_in_bits_plus8: 0,
    };
    let mut ret: ::core::ffi::c_int = 0;
    ret = init_get_bits8(
        &raw mut gb,
        &raw mut (*h).payload as *mut uint8_t,
        (*h).payload_size_bytes,
    );
    if (*sps).nal_hrd_parameters_present_flag != 0
        || (*sps).vcl_hrd_parameters_present_flag != 0
    {
        (*h).cpb_removal_delay = get_bits_long(
            &raw mut gb,
            (*sps).cpb_removal_delay_length,
        ) as ::core::ffi::c_int;
        (*h).dpb_output_delay = get_bits_long(
            &raw mut gb,
            (*sps).dpb_output_delay_length,
        ) as ::core::ffi::c_int;
    }
    if (*sps).pic_struct_present_flag != 0 {
        let mut i: ::core::ffi::c_uint = 0;
        let mut num_clock_ts: ::core::ffi::c_uint = 0;
        (*h).pic_struct = get_bits(&raw mut gb, 4 as ::core::ffi::c_int)
            as H264_SEI_PicStructType;
        (*h).ct_type = 0 as ::core::ffi::c_int;
        if (*h).pic_struct as ::core::ffi::c_uint
            > H264_SEI_PIC_STRUCT_FRAME_TRIPLING as ::core::ffi::c_int
                as ::core::ffi::c_uint
        {
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        num_clock_ts = sei_num_clock_ts_table[(*h).pic_struct as usize]
            as ::core::ffi::c_uint;
        (*h).timecode_cnt = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_uint;
        while i < num_clock_ts {
            if get_bits(&raw mut gb, 1 as ::core::ffi::c_int) != 0 {
                let fresh0 = (*h).timecode_cnt;
                (*h).timecode_cnt = (*h).timecode_cnt + 1;
                let mut tc: *mut H264SEITimeCode = (&raw mut (*h).timecode
                    as *mut H264SEITimeCode)
                    .offset(fresh0 as isize) as *mut H264SEITimeCode;
                let mut full_timestamp_flag: ::core::ffi::c_uint = 0;
                let mut counting_type: ::core::ffi::c_uint = 0;
                let mut cnt_dropped_flag: ::core::ffi::c_uint = 0;
                (*h).ct_type
                    |= (1 as ::core::ffi::c_int)
                        << get_bits(&raw mut gb, 2 as ::core::ffi::c_int);
                skip_bits(&raw mut gb, 1 as ::core::ffi::c_int);
                counting_type = get_bits(&raw mut gb, 5 as ::core::ffi::c_int);
                full_timestamp_flag = get_bits(&raw mut gb, 1 as ::core::ffi::c_int);
                skip_bits(&raw mut gb, 1 as ::core::ffi::c_int);
                cnt_dropped_flag = get_bits(&raw mut gb, 1 as ::core::ffi::c_int);
                if cnt_dropped_flag != 0 && counting_type > 1 as ::core::ffi::c_uint
                    && counting_type < 7 as ::core::ffi::c_uint
                {
                    (*tc).dropframe = 1 as ::core::ffi::c_int;
                }
                (*tc).frame = get_bits(&raw mut gb, 8 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
                if full_timestamp_flag != 0 {
                    (*tc).full = 1 as ::core::ffi::c_int;
                    (*tc).seconds = get_bits(&raw mut gb, 6 as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                    (*tc).minutes = get_bits(&raw mut gb, 6 as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                    (*tc).hours = get_bits(&raw mut gb, 5 as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                } else {
                    (*tc).full = 0 as ::core::ffi::c_int;
                    (*tc).hours = (*tc).full;
                    (*tc).minutes = (*tc).hours;
                    (*tc).seconds = (*tc).minutes;
                    if get_bits(&raw mut gb, 1 as ::core::ffi::c_int) != 0 {
                        (*tc).seconds = get_bits(&raw mut gb, 6 as ::core::ffi::c_int)
                            as ::core::ffi::c_int;
                        if get_bits(&raw mut gb, 1 as ::core::ffi::c_int) != 0 {
                            (*tc).minutes = get_bits(
                                &raw mut gb,
                                6 as ::core::ffi::c_int,
                            ) as ::core::ffi::c_int;
                            if get_bits(&raw mut gb, 1 as ::core::ffi::c_int) != 0 {
                                (*tc).hours = get_bits(&raw mut gb, 5 as ::core::ffi::c_int)
                                    as ::core::ffi::c_int;
                            }
                        }
                    }
                }
                if (*sps).time_offset_length > 0 as ::core::ffi::c_int {
                    skip_bits(&raw mut gb, (*sps).time_offset_length);
                }
            }
            i = i.wrapping_add(1);
        }
        av_log(
            logctx,
            48 as ::core::ffi::c_int,
            b"ct_type:%X pic_struct:%d\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*h).ct_type,
            (*h).pic_struct as ::core::ffi::c_uint,
        );
    }
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn av_bswap64(mut x: uint64_t) -> uint64_t {
    return (av_bswap32(x as uint32_t) as uint64_t) << 32 as ::core::ffi::c_int
        | av_bswap32((x >> 32 as ::core::ffi::c_int) as uint32_t) as uint64_t;
}
#[inline(always)]
unsafe extern "C" fn bytestream2_peek_le16u(
    mut g: *const GetByteContext,
) -> ::core::ffi::c_uint {
    return (*((*g).buffer as *const unaligned_16)).l as ::core::ffi::c_uint;
}
#[inline(always)]
unsafe extern "C" fn bytestream2_peek_le16(
    mut g: *const GetByteContext,
) -> ::core::ffi::c_uint {
    if ((*g).buffer_end.offset_from((*g).buffer) as ::core::ffi::c_long)
        < 2 as ::core::ffi::c_long
    {
        return 0 as ::core::ffi::c_uint;
    }
    return bytestream2_peek_le16u(g);
}
#[inline(always)]
unsafe extern "C" fn bytestream_get_be16(
    mut b: *mut *const uint8_t,
) -> ::core::ffi::c_uint {
    *b = (*b).offset(2 as ::core::ffi::c_int as isize);
    return av_bswap16(
        (*((*b).offset(-(2 as ::core::ffi::c_int as isize)) as *const unaligned_16)).l,
    ) as ::core::ffi::c_uint;
}
#[inline(always)]
unsafe extern "C" fn bytestream2_get_be16u(
    mut g: *mut GetByteContext,
) -> ::core::ffi::c_uint {
    return bytestream_get_be16(&raw mut (*g).buffer);
}
#[inline(always)]
unsafe extern "C" fn bytestream2_get_be16(
    mut g: *mut GetByteContext,
) -> ::core::ffi::c_uint {
    if ((*g).buffer_end.offset_from((*g).buffer) as ::core::ffi::c_long)
        < 2 as ::core::ffi::c_long
    {
        (*g).buffer = (*g).buffer_end;
        return 0 as ::core::ffi::c_uint;
    }
    return bytestream2_get_be16u(g);
}
#[inline(always)]
unsafe extern "C" fn bytestream_get_byte(
    mut b: *mut *const uint8_t,
) -> ::core::ffi::c_uint {
    *b = (*b).offset(1 as ::core::ffi::c_int as isize);
    return *(*b)
        .offset(-(1 as ::core::ffi::c_int as isize))
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
}
#[inline(always)]
unsafe extern "C" fn bytestream2_get_byteu(
    mut g: *mut GetByteContext,
) -> ::core::ffi::c_uint {
    return bytestream_get_byte(&raw mut (*g).buffer);
}
#[inline(always)]
unsafe extern "C" fn bytestream2_get_byte(
    mut g: *mut GetByteContext,
) -> ::core::ffi::c_uint {
    if ((*g).buffer_end.offset_from((*g).buffer) as ::core::ffi::c_long)
        < 1 as ::core::ffi::c_long
    {
        (*g).buffer = (*g).buffer_end;
        return 0 as ::core::ffi::c_uint;
    }
    return bytestream2_get_byteu(g);
}
#[inline(always)]
unsafe extern "C" fn bytestream2_peek_byteu(
    mut g: *const GetByteContext,
) -> ::core::ffi::c_uint {
    return *(*g).buffer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
}
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
unsafe extern "C" fn decode_picture_timing(
    mut h: *mut H264SEIPictureTiming,
    mut gb: *mut GetByteContext,
    mut logctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut size: ::core::ffi::c_int = bytestream2_get_bytes_left(gb);
    if size as usize > ::core::mem::size_of::<[uint8_t; 40]>() as usize {
        av_log(
            logctx,
            16 as ::core::ffi::c_int,
            b"Picture timing SEI payload too large\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    bytestream2_get_bufferu(
        gb,
        &raw mut (*h).payload as *mut uint8_t,
        size as ::core::ffi::c_uint,
    );
    (*h).payload_size_bytes = size;
    (*h).present = 1 as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn bytestream2_init(
    mut g: *mut GetByteContext,
    mut buf: *const uint8_t,
    mut buf_size: ::core::ffi::c_int,
) {
    if !(!buf.is_null() && buf_size >= 0 as ::core::ffi::c_int) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"buf && buf_size >= 0\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavcodec/bytestream.h\0" as *const u8 as *const ::core::ffi::c_char,
            141 as ::core::ffi::c_int,
        );
        abort();
    }
    (*g).buffer = buf;
    (*g).buffer_start = buf;
    (*g).buffer_end = buf.offset(buf_size as isize);
}
unsafe extern "C" fn decode_recovery_point(
    mut h: *mut H264SEIRecoveryPoint,
    mut gb: *mut GetBitContext,
    mut logctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut recovery_frame_cnt: ::core::ffi::c_uint = get_ue_golomb_long(gb);
    if recovery_frame_cnt
        >= ((1 as ::core::ffi::c_int)
            << 12 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as ::core::ffi::c_uint
    {
        av_log(
            logctx,
            16 as ::core::ffi::c_int,
            b"recovery_frame_cnt %u is out of range\n\0" as *const u8
                as *const ::core::ffi::c_char,
            recovery_frame_cnt,
        );
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    (*h).recovery_frame_cnt = recovery_frame_cnt as ::core::ffi::c_int;
    skip_bits(gb, 4 as ::core::ffi::c_int);
    return 0 as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn bytestream2_get_bytes_left(
    mut g: *const GetByteContext,
) -> ::core::ffi::c_int {
    return (*g).buffer_end.offset_from((*g).buffer) as ::core::ffi::c_long
        as ::core::ffi::c_int;
}
unsafe extern "C" fn decode_buffering_period(
    mut h: *mut H264SEIBufferingPeriod,
    mut gb: *mut GetBitContext,
    mut ps: *const H264ParamSets,
    mut logctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut sps_id: ::core::ffi::c_uint = 0;
    let mut sched_sel_idx: ::core::ffi::c_int = 0;
    let mut sps: *const SPS = ::core::ptr::null::<SPS>();
    sps_id = get_ue_golomb_31(gb) as ::core::ffi::c_uint;
    if sps_id > 31 as ::core::ffi::c_uint || (*ps).sps_list[sps_id as usize].is_null() {
        av_log(
            logctx,
            16 as ::core::ffi::c_int,
            b"non-existing SPS %d referenced in buffering period\n\0" as *const u8
                as *const ::core::ffi::c_char,
            sps_id,
        );
        return if sps_id > 31 as ::core::ffi::c_uint {
            -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int)
        } else {
            -(((0xf8 as ::core::ffi::c_int | ('?' as i32) << 8 as ::core::ffi::c_int
                | ('P' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('S' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int)
        };
    }
    sps = (*ps).sps_list[sps_id as usize];
    if (*sps).nal_hrd_parameters_present_flag != 0 {
        sched_sel_idx = 0 as ::core::ffi::c_int;
        while sched_sel_idx < (*sps).cpb_cnt {
            (*h).initial_cpb_removal_delay[sched_sel_idx as usize] = get_bits_long(
                gb,
                (*sps).initial_cpb_removal_delay_length,
            ) as ::core::ffi::c_int;
            skip_bits(gb, (*sps).initial_cpb_removal_delay_length);
            sched_sel_idx += 1;
        }
    }
    if (*sps).vcl_hrd_parameters_present_flag != 0 {
        sched_sel_idx = 0 as ::core::ffi::c_int;
        while sched_sel_idx < (*sps).cpb_cnt {
            (*h).initial_cpb_removal_delay[sched_sel_idx as usize] = get_bits_long(
                gb,
                (*sps).initial_cpb_removal_delay_length,
            ) as ::core::ffi::c_int;
            skip_bits(gb, (*sps).initial_cpb_removal_delay_length);
            sched_sel_idx += 1;
        }
    }
    (*h).present = 1 as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn bytestream2_skipu(
    mut g: *mut GetByteContext,
    mut size: ::core::ffi::c_uint,
) {
    (*g).buffer = (*g).buffer.offset(size as isize);
}
#[inline(always)]
unsafe extern "C" fn av_clip_c(
    mut a: ::core::ffi::c_int,
    mut amin: ::core::ffi::c_int,
    mut amax: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if a < amin { return amin } else if a > amax { return amax } else { return a };
}
unsafe extern "C" fn decode_green_metadata(
    mut h: *mut H264SEIGreenMetaData,
    mut gb: *mut GetByteContext,
) -> ::core::ffi::c_int {
    (*h).green_metadata_type = bytestream2_get_byte(gb) as uint8_t;
    if (*h).green_metadata_type as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        (*h).period_type = bytestream2_get_byte(gb) as uint8_t;
        if (*h).period_type as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (*h).num_seconds = bytestream2_get_be16(gb) as uint16_t;
        } else if (*h).period_type as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (*h).num_pictures = bytestream2_get_be16(gb) as uint16_t;
        }
        (*h).percent_non_zero_macroblocks = bytestream2_get_byte(gb) as uint8_t;
        (*h).percent_intra_coded_macroblocks = bytestream2_get_byte(gb) as uint8_t;
        (*h).percent_six_tap_filtering = bytestream2_get_byte(gb) as uint8_t;
        (*h).percent_alpha_point_deblocking_instance = bytestream2_get_byte(gb)
            as uint8_t;
    } else if (*h).green_metadata_type as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        (*h).xsd_metric_type = bytestream2_get_byte(gb) as uint8_t;
        (*h).xsd_metric_value = bytestream2_get_be16(gb) as uint16_t;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_h264_sei_decode(
    mut h: *mut H264SEIContext,
    mut gb: *mut GetBitContext,
    mut ps: *const H264ParamSets,
    mut logctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut gbyte: GetByteContext = GetByteContext {
        buffer: ::core::ptr::null::<uint8_t>(),
        buffer_end: ::core::ptr::null::<uint8_t>(),
        buffer_start: ::core::ptr::null::<uint8_t>(),
    };
    let mut master_ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    bytestream2_init(
        &raw mut gbyte,
        (*gb).buffer.offset((get_bits_count(gb) / 8 as ::core::ffi::c_int) as isize),
        get_bits_left(gb) / 8 as ::core::ffi::c_int,
    );
    while bytestream2_get_bytes_left(&raw mut gbyte) > 2 as ::core::ffi::c_int
        && bytestream2_peek_le16(&raw mut gbyte) != 0
    {
        let mut gbyte_payload: GetByteContext = GetByteContext {
            buffer: ::core::ptr::null::<uint8_t>(),
            buffer_end: ::core::ptr::null::<uint8_t>(),
            buffer_start: ::core::ptr::null::<uint8_t>(),
        };
        let mut gb_payload: GetBitContext = GetBitContext {
            buffer: ::core::ptr::null::<uint8_t>(),
            index: 0,
            size_in_bits: 0,
            size_in_bits_plus8: 0,
        };
        let mut type_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut size: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        loop {
            if bytestream2_get_bytes_left(&raw mut gbyte) <= 0 as ::core::ffi::c_int {
                return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                    | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
            }
            type_0 = (type_0 as ::core::ffi::c_uint)
                .wrapping_add(bytestream2_peek_byteu(&raw mut gbyte))
                as ::core::ffi::c_int as ::core::ffi::c_int;
            if !(bytestream2_get_byteu(&raw mut gbyte) == 255 as ::core::ffi::c_uint) {
                break;
            }
        }
        loop {
            if bytestream2_get_bytes_left(&raw mut gbyte) <= 0 as ::core::ffi::c_int {
                return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                    | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
            }
            size = size.wrapping_add(bytestream2_peek_byteu(&raw mut gbyte));
            if !(bytestream2_get_byteu(&raw mut gbyte) == 255 as ::core::ffi::c_uint) {
                break;
            }
        }
        if size > bytestream2_get_bytes_left(&raw mut gbyte) as ::core::ffi::c_uint {
            av_log(
                logctx,
                16 as ::core::ffi::c_int,
                b"SEI type %d size %d truncated at %d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                type_0,
                size,
                bytestream2_get_bytes_left(&raw mut gbyte),
            );
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        bytestream2_init(
            &raw mut gbyte_payload,
            gbyte.buffer,
            size as ::core::ffi::c_int,
        );
        ret = init_get_bits8(
            &raw mut gb_payload,
            gbyte.buffer,
            size as ::core::ffi::c_int,
        );
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        match type_0 {
            1 => {
                ret = decode_picture_timing(
                    &raw mut (*h).picture_timing,
                    &raw mut gbyte_payload,
                    logctx,
                );
            }
            6 => {
                ret = decode_recovery_point(
                    &raw mut (*h).recovery_point,
                    &raw mut gb_payload,
                    logctx,
                );
            }
            0 => {
                ret = decode_buffering_period(
                    &raw mut (*h).buffering_period,
                    &raw mut gb_payload,
                    ps,
                    logctx,
                );
            }
            56 => {
                ret = decode_green_metadata(
                    &raw mut (*h).green_metadata,
                    &raw mut gbyte_payload,
                );
            }
            _ => {
                ret = ff_h2645_sei_message_decode(
                    &raw mut (*h).common,
                    type_0 as SEIType,
                    AV_CODEC_ID_H264,
                    &raw mut gb_payload,
                    &raw mut gbyte_payload,
                    logctx,
                );
                if ret == FF_H2645_SEI_MESSAGE_UNHANDLED as ::core::ffi::c_int {
                    av_log(
                        logctx,
                        48 as ::core::ffi::c_int,
                        b"unknown SEI type %d\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        type_0,
                    );
                }
            }
        }
        if ret < 0 as ::core::ffi::c_int
            && ret
                != -(((0xf8 as ::core::ffi::c_int
                    | ('?' as i32) << 8 as ::core::ffi::c_int
                    | ('P' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('S' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int)
        {
            return ret;
        }
        if ret < 0 as ::core::ffi::c_int {
            master_ret = ret;
        }
        if get_bits_left(&raw mut gb_payload) < 0 as ::core::ffi::c_int {
            av_log(
                logctx,
                24 as ::core::ffi::c_int,
                b"SEI type %d overread by %d bits\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                type_0,
                -get_bits_left(&raw mut gb_payload),
            );
        }
        bytestream2_skipu(&raw mut gbyte, size);
    }
    return master_ret;
}
#[inline]
unsafe extern "C" fn get_bits_count(mut s: *const GetBitContext) -> ::core::ffi::c_int {
    return (*s).index;
}
#[inline(always)]
unsafe extern "C" fn bytestream2_get_bufferu(
    mut g: *mut GetByteContext,
    mut dst: *mut uint8_t,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        (*g).buffer as *const ::core::ffi::c_void,
        size as size_t,
    );
    (*g).buffer = (*g).buffer.offset(size as isize);
    return size;
}
#[inline]
unsafe extern "C" fn skip_bits_long(
    mut s: *mut GetBitContext,
    mut n: ::core::ffi::c_int,
) {
    (*s).index += av_clip_c(n, -(*s).index, (*s).size_in_bits_plus8 - (*s).index);
}
#[no_mangle]
pub unsafe extern "C" fn ff_h264_sei_stereo_mode(
    mut h: *const H2645SEIFramePacking,
) -> *const ::core::ffi::c_char {
    if (*h).arrangement_cancel_flag == 0 as ::core::ffi::c_int {
        match (*h).arrangement_type as ::core::ffi::c_uint {
            0 => {
                if (*h).content_interpretation_type == 2 as ::core::ffi::c_int {
                    return b"checkerboard_rl\0" as *const u8
                        as *const ::core::ffi::c_char
                } else {
                    return b"checkerboard_lr\0" as *const u8
                        as *const ::core::ffi::c_char
                }
            }
            1 => {
                if (*h).content_interpretation_type == 2 as ::core::ffi::c_int {
                    return b"col_interleaved_rl\0" as *const u8
                        as *const ::core::ffi::c_char
                } else {
                    return b"col_interleaved_lr\0" as *const u8
                        as *const ::core::ffi::c_char
                }
            }
            2 => {
                if (*h).content_interpretation_type == 2 as ::core::ffi::c_int {
                    return b"row_interleaved_rl\0" as *const u8
                        as *const ::core::ffi::c_char
                } else {
                    return b"row_interleaved_lr\0" as *const u8
                        as *const ::core::ffi::c_char
                }
            }
            3 => {
                if (*h).content_interpretation_type == 2 as ::core::ffi::c_int {
                    return b"right_left\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    return b"left_right\0" as *const u8 as *const ::core::ffi::c_char
                }
            }
            4 => {
                if (*h).content_interpretation_type == 2 as ::core::ffi::c_int {
                    return b"bottom_top\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    return b"top_bottom\0" as *const u8 as *const ::core::ffi::c_char
                }
            }
            5 => {
                if (*h).content_interpretation_type == 2 as ::core::ffi::c_int {
                    return b"block_rl\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    return b"block_lr\0" as *const u8 as *const ::core::ffi::c_char
                }
            }
            6 | _ => return b"mono\0" as *const u8 as *const ::core::ffi::c_char,
        }
    } else if (*h).arrangement_cancel_flag == 1 as ::core::ffi::c_int {
        return b"mono\0" as *const u8 as *const ::core::ffi::c_char
    } else {
        return ::core::ptr::null::<::core::ffi::c_char>()
    };
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
unsafe extern "C" fn get_bits_left(mut gb: *mut GetBitContext) -> ::core::ffi::c_int {
    return (*gb).size_in_bits - get_bits_count(gb);
}
#[repr(C)]
pub struct AVBuffer {
    _opaque: [u8; 0],
}
