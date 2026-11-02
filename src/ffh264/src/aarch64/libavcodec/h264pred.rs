// Translated from FFmpeg 9.0.1 libavcodec/h264pred.c
// (preprocessed for aarch64-unknown-linux-musl) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// H.26L/H.264/AVC/JVT/14496-10/... encoder/decoder
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
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    static ff_crop_tab: [uint8_t; 2304];
    fn abort() -> !;
    fn ff_h264_pred_init_aarch64(
        h: *mut H264PredContext,
        codec_id: ::core::ffi::c_int,
        bit_depth: ::core::ffi::c_int,
        chroma_format_idc: ::core::ffi::c_int,
    );
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
pub type ptrdiff_t = ::core::ffi::c_long;
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

unsafe extern "C" fn pred4x4_vertical_8_c(
    mut _src: *mut uint8_t,
    mut topright: *const uint8_t,
    mut _stride: ptrdiff_t,
) {
    let mut src: *mut uint8_t = _src;
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    let a: uint32_t = (*(src.offset(-(stride as isize)) as *const av_alias32)).u32_0;
    (*(src.offset((0 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
    (*(src.offset((1 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
    (*(src.offset((2 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
    (*(src.offset((3 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
}



unsafe extern "C" fn pred4x4_horizontal_8_c(
    mut _src: *mut uint8_t,
    mut topright: *const uint8_t,
    mut _stride: ptrdiff_t,
) {
    let mut src: *mut uint8_t = _src;
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    (*(src.offset((0 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = (*src
        .offset((-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint)
        .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
    (*(src.offset((1 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = (*src
        .offset((-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint)
        .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
    (*(src.offset((2 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = (*src
        .offset((-(1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint)
        .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
    (*(src.offset((3 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = (*src
        .offset((-(1 as ::core::ffi::c_int) + 3 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint)
        .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
}




unsafe extern "C" fn pred4x4_127_dc_c(
    mut src: *mut uint8_t,
    mut topright: *const uint8_t,
    mut _stride: ptrdiff_t,
) {
    let mut stride: ::core::ffi::c_int = _stride as ::core::ffi::c_int;
    let a: uint32_t = 0x7f7f7f7f as uint32_t;
    (*(src.offset((0 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
    (*(src.offset((1 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
    (*(src.offset((2 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
    (*(src.offset((3 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
}




unsafe extern "C" fn pred4x4_dc_8_c(
    mut _src: *mut uint8_t,
    mut topright: *const uint8_t,
    mut _stride: ptrdiff_t,
) {
    let mut src: *mut uint8_t = _src;
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    let dc: ::core::ffi::c_int = *src.offset(-stride as isize) as ::core::ffi::c_int
        + *src.offset((1 as ::core::ffi::c_int - stride) as isize) as ::core::ffi::c_int
        + *src.offset((2 as ::core::ffi::c_int - stride) as isize) as ::core::ffi::c_int
        + *src.offset((3 as ::core::ffi::c_int - stride) as isize) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * stride) as isize,
            ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * stride) as isize,
            ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int * stride) as isize,
            ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int) + 3 as ::core::ffi::c_int * stride) as isize,
            ) as ::core::ffi::c_int + 4 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int;
    let a: uint32_t = (dc as uint32_t).wrapping_mul(0x1010101 as uint32_t);
    (*(src.offset((0 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
    (*(src.offset((1 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
    (*(src.offset((2 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
    (*(src.offset((3 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
}
unsafe extern "C" fn pred4x4_129_dc_c(
    mut src: *mut uint8_t,
    mut topright: *const uint8_t,
    mut _stride: ptrdiff_t,
) {
    let mut stride: ::core::ffi::c_int = _stride as ::core::ffi::c_int;
    let a: uint32_t = 0x81818181 as uint32_t;
    (*(src.offset((0 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
    (*(src.offset((1 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
    (*(src.offset((2 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
    (*(src.offset((3 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
}




unsafe extern "C" fn pred4x4_left_dc_8_c(
    mut _src: *mut uint8_t,
    mut topright: *const uint8_t,
    mut _stride: ptrdiff_t,
) {
    let mut src: *mut uint8_t = _src;
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    let dc: ::core::ffi::c_int = *src
        .offset((-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * stride) as isize,
            ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int * stride) as isize,
            ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int) + 3 as ::core::ffi::c_int * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let a: uint32_t = (dc as uint32_t).wrapping_mul(0x1010101 as uint32_t);
    (*(src.offset((0 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
    (*(src.offset((1 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
    (*(src.offset((2 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
    (*(src.offset((3 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
}
unsafe extern "C" fn pred4x4_vertical_vp8_c(
    mut src: *mut uint8_t,
    mut topright: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let lt: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t - 1 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let t0: ::core::ffi::c_uint = *src
        .offset((0 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let t1: ::core::ffi::c_uint = *src
        .offset((1 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let t2: ::core::ffi::c_uint = *src
        .offset((2 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let t3: ::core::ffi::c_uint = *src
        .offset((3 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let t4: ::core::ffi::c_uint = *topright.offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    let t5: ::core::ffi::c_uint = *topright.offset(1 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    let t6: ::core::ffi::c_uint = *topright.offset(2 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    let t7: ::core::ffi::c_uint = *topright.offset(3 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    let mut v: uint32_t = ((t2 as uint32_t)
        .wrapping_add((2 as uint32_t).wrapping_mul(t3 as uint32_t))
        .wrapping_add(t4 as uint32_t)
        .wrapping_add(2 as uint32_t) >> 2 as ::core::ffi::c_int)
        << 24 as ::core::ffi::c_int
        | ((t1 as uint32_t)
            .wrapping_add((2 as uint32_t).wrapping_mul(t2 as uint32_t))
            .wrapping_add(t3 as uint32_t)
            .wrapping_add(2 as uint32_t) >> 2 as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int
        | ((t0 as uint32_t)
            .wrapping_add((2 as uint32_t).wrapping_mul(t1 as uint32_t))
            .wrapping_add(t2 as uint32_t)
            .wrapping_add(2 as uint32_t) >> 2 as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
        | (lt as uint32_t)
            .wrapping_add((2 as uint32_t).wrapping_mul(t0 as uint32_t))
            .wrapping_add(t1 as uint32_t)
            .wrapping_add(2 as uint32_t) >> 2 as ::core::ffi::c_int;
    (*(src.offset((0 as ptrdiff_t * stride) as isize) as *mut av_alias32)).u32_0 = v;
    (*(src.offset((1 as ptrdiff_t * stride) as isize) as *mut av_alias32)).u32_0 = v;
    (*(src.offset((2 as ptrdiff_t * stride) as isize) as *mut av_alias32)).u32_0 = v;
    (*(src.offset((3 as ptrdiff_t * stride) as isize) as *mut av_alias32)).u32_0 = v;
}




unsafe extern "C" fn pred4x4_top_dc_8_c(
    mut _src: *mut uint8_t,
    mut topright: *const uint8_t,
    mut _stride: ptrdiff_t,
) {
    let mut src: *mut uint8_t = _src;
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    let dc: ::core::ffi::c_int = *src.offset(-stride as isize) as ::core::ffi::c_int
        + *src.offset((1 as ::core::ffi::c_int - stride) as isize) as ::core::ffi::c_int
        + *src.offset((2 as ::core::ffi::c_int - stride) as isize) as ::core::ffi::c_int
        + *src.offset((3 as ::core::ffi::c_int - stride) as isize) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let a: uint32_t = (dc as uint32_t).wrapping_mul(0x1010101 as uint32_t);
    (*(src.offset((0 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
    (*(src.offset((1 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
    (*(src.offset((2 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
    (*(src.offset((3 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
}
unsafe extern "C" fn pred4x4_horizontal_vp8_c(
    mut src: *mut uint8_t,
    mut topright: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let lt: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t - 1 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l0: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 0 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l1: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 1 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l2: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 2 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l3: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 3 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    (*(src.offset((0 as ptrdiff_t * stride) as isize) as *mut av_alias32)).u32_0 = (lt
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l0))
        .wrapping_add(l1)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int)
        .wrapping_mul(0x1010101 as ::core::ffi::c_int as ::core::ffi::c_uint)
        as uint32_t;
    (*(src.offset((1 as ptrdiff_t * stride) as isize) as *mut av_alias32)).u32_0 = (l0
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l1))
        .wrapping_add(l2)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int)
        .wrapping_mul(0x1010101 as ::core::ffi::c_int as ::core::ffi::c_uint)
        as uint32_t;
    (*(src.offset((2 as ptrdiff_t * stride) as isize) as *mut av_alias32)).u32_0 = (l1
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l2))
        .wrapping_add(l3)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int)
        .wrapping_mul(0x1010101 as ::core::ffi::c_int as ::core::ffi::c_uint)
        as uint32_t;
    (*(src.offset((3 as ptrdiff_t * stride) as isize) as *mut av_alias32)).u32_0 = (l2
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l3))
        .wrapping_add(l3)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int)
        .wrapping_mul(0x1010101 as ::core::ffi::c_int as ::core::ffi::c_uint)
        as uint32_t;
}




unsafe extern "C" fn pred4x4_128_dc_8_c(
    mut _src: *mut uint8_t,
    mut topright: *const uint8_t,
    mut _stride: ptrdiff_t,
) {
    let mut src: *mut uint8_t = _src;
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    let a: uint32_t = (((1 as ::core::ffi::c_int)
        << 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as uint32_t)
        .wrapping_mul(0x1010101 as uint32_t);
    (*(src.offset((0 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
    (*(src.offset((1 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
    (*(src.offset((2 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
    (*(src.offset((3 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
}
unsafe extern "C" fn pred4x4_down_left_svq3_c(
    mut src: *mut uint8_t,
    mut topright: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let t0: ::core::ffi::c_uint = *src
        .offset((0 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let t1: ::core::ffi::c_uint = *src
        .offset((1 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let t2: ::core::ffi::c_uint = *src
        .offset((2 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let t3: ::core::ffi::c_uint = *src
        .offset((3 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let l0: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 0 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l1: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 1 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l2: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 2 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l3: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 3 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    *src.offset((0 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = (l1
        .wrapping_add(t1) >> 1 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh43 = *src
        .offset((0 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize);
    *fresh43 = (l2.wrapping_add(t2) >> 1 as ::core::ffi::c_int) as uint8_t;
    *src.offset((1 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh43;
    let ref mut fresh44 = *src
        .offset((3 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
    *fresh44 = (l3.wrapping_add(t3) >> 1 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh45 = *src
        .offset((2 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
    *fresh45 = *fresh44;
    let ref mut fresh46 = *src
        .offset((3 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
    *fresh46 = *fresh45;
    let ref mut fresh47 = *src
        .offset((1 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
    *fresh47 = *fresh46;
    let ref mut fresh48 = *src
        .offset((2 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
    *fresh48 = *fresh47;
    let ref mut fresh49 = *src
        .offset((3 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize);
    *fresh49 = *fresh48;
    let ref mut fresh50 = *src
        .offset((0 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
    *fresh50 = *fresh49;
    let ref mut fresh51 = *src
        .offset((1 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
    *fresh51 = *fresh50;
    let ref mut fresh52 = *src
        .offset((2 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize);
    *fresh52 = *fresh51;
    let ref mut fresh53 = *src
        .offset((3 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize);
    *fresh53 = *fresh52;
    let ref mut fresh54 = *src
        .offset((0 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
    *fresh54 = *fresh53;
    let ref mut fresh55 = *src
        .offset((1 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize);
    *fresh55 = *fresh54;
    *src.offset((2 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh55;
}
unsafe extern "C" fn pred4x4_down_left_rv40_c(
    mut src: *mut uint8_t,
    mut topright: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let t0: ::core::ffi::c_uint = *src
        .offset((0 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let t1: ::core::ffi::c_uint = *src
        .offset((1 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let t2: ::core::ffi::c_uint = *src
        .offset((2 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let t3: ::core::ffi::c_uint = *src
        .offset((3 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let t4: ::core::ffi::c_uint = *topright.offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    let t5: ::core::ffi::c_uint = *topright.offset(1 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    let t6: ::core::ffi::c_uint = *topright.offset(2 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    let t7: ::core::ffi::c_uint = *topright.offset(3 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    let l0: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 0 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l1: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 1 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l2: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 2 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l3: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 3 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l4: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 4 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l5: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 5 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l6: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 6 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l7: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 7 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    *src.offset((0 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = (t0
        .wrapping_add(t2)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t1))
        .wrapping_add(2 as ::core::ffi::c_uint)
        .wrapping_add(l0)
        .wrapping_add(l2)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l1))
        .wrapping_add(2 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh34 = *src
        .offset((0 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize);
    *fresh34 = (t1
        .wrapping_add(t3)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t2))
        .wrapping_add(2 as ::core::ffi::c_uint)
        .wrapping_add(l1)
        .wrapping_add(l3)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l2))
        .wrapping_add(2 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int) as uint8_t;
    *src.offset((1 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh34;
    let ref mut fresh35 = *src
        .offset((0 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
    *fresh35 = (t2
        .wrapping_add(t4)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t3))
        .wrapping_add(2 as ::core::ffi::c_uint)
        .wrapping_add(l2)
        .wrapping_add(l4)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l3))
        .wrapping_add(2 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh36 = *src
        .offset((1 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize);
    *fresh36 = *fresh35;
    *src.offset((2 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh36;
    let ref mut fresh37 = *src
        .offset((0 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
    *fresh37 = (t3
        .wrapping_add(t5)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t4))
        .wrapping_add(2 as ::core::ffi::c_uint)
        .wrapping_add(l3)
        .wrapping_add(l5)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l4))
        .wrapping_add(2 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh38 = *src
        .offset((1 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
    *fresh38 = *fresh37;
    let ref mut fresh39 = *src
        .offset((2 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize);
    *fresh39 = *fresh38;
    *src.offset((3 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh39;
    let ref mut fresh40 = *src
        .offset((1 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
    *fresh40 = (t4
        .wrapping_add(t6)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t5))
        .wrapping_add(2 as ::core::ffi::c_uint)
        .wrapping_add(l4)
        .wrapping_add(l6)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l5))
        .wrapping_add(2 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh41 = *src
        .offset((2 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
    *fresh41 = *fresh40;
    *src.offset((3 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) = *fresh41;
    let ref mut fresh42 = *src
        .offset((2 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
    *fresh42 = (t5
        .wrapping_add(t7)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t6))
        .wrapping_add(2 as ::core::ffi::c_uint)
        .wrapping_add(l5)
        .wrapping_add(l7)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l6))
        .wrapping_add(2 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int) as uint8_t;
    *src.offset((3 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize) = *fresh42;
    *src.offset((3 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize) = (t6
        .wrapping_add(t7)
        .wrapping_add(1 as ::core::ffi::c_uint)
        .wrapping_add(l6)
        .wrapping_add(l7)
        .wrapping_add(1 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
}

unsafe extern "C" fn pred4x4_down_right_8_c(
    mut _src: *mut uint8_t,
    mut topright: *const uint8_t,
    mut _stride: ptrdiff_t,
) {
    let mut src: *mut uint8_t = _src;
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    let lt: ::core::ffi::c_int = *src
        .offset((-(1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_int;
    let t0: ::core::ffi::c_uint = *src
        .offset((0 as ::core::ffi::c_int - 1 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let t1: ::core::ffi::c_uint = *src
        .offset((1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let t2: ::core::ffi::c_uint = *src
        .offset((2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let t3: ::core::ffi::c_uint = *src
        .offset((3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let l0: ::core::ffi::c_uint = *src
        .offset((-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let l1: ::core::ffi::c_uint = *src
        .offset((-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let l2: ::core::ffi::c_uint = *src
        .offset((-(1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let l3: ::core::ffi::c_uint = *src
        .offset((-(1 as ::core::ffi::c_int) + 3 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    *src.offset((0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize) = (l3
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l2))
        .wrapping_add(l1)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh358 = *src
        .offset((1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh358 = (l2
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l1))
        .wrapping_add(l0)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize) = *fresh358;
    let ref mut fresh359 = *src
        .offset((2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh359 = (l1
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l0))
        .wrapping_add(lt as ::core::ffi::c_uint)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh360 = *src
        .offset((1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh360 = *fresh359;
    *src.offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize) = *fresh360;
    let ref mut fresh361 = *src
        .offset((3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh361 = (l0
        .wrapping_add((2 as ::core::ffi::c_int * lt) as ::core::ffi::c_uint)
        .wrapping_add(t0)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh362 = *src
        .offset((2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh362 = *fresh361;
    let ref mut fresh363 = *src
        .offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh363 = *fresh362;
    *src.offset((0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh363;
    let ref mut fresh364 = *src
        .offset((3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh364 = ((lt as ::core::ffi::c_uint)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t0))
        .wrapping_add(t1)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh365 = *src
        .offset((2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh365 = *fresh364;
    *src.offset((1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh365;
    let ref mut fresh366 = *src
        .offset((3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh366 = (t0
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t1))
        .wrapping_add(t2)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh366;
    *src.offset((3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = (t1
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t2))
        .wrapping_add(t3)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
}



unsafe extern "C" fn pred4x4_down_left_rv40_nodown_c(
    mut src: *mut uint8_t,
    mut topright: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let t0: ::core::ffi::c_uint = *src
        .offset((0 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let t1: ::core::ffi::c_uint = *src
        .offset((1 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let t2: ::core::ffi::c_uint = *src
        .offset((2 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let t3: ::core::ffi::c_uint = *src
        .offset((3 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let t4: ::core::ffi::c_uint = *topright.offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    let t5: ::core::ffi::c_uint = *topright.offset(1 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    let t6: ::core::ffi::c_uint = *topright.offset(2 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    let t7: ::core::ffi::c_uint = *topright.offset(3 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    let l0: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 0 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l1: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 1 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l2: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 2 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l3: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 3 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    *src.offset((0 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = (t0
        .wrapping_add(t2)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t1))
        .wrapping_add(2 as ::core::ffi::c_uint)
        .wrapping_add(l0)
        .wrapping_add(l2)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l1))
        .wrapping_add(2 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh19 = *src
        .offset((0 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize);
    *fresh19 = (t1
        .wrapping_add(t3)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t2))
        .wrapping_add(2 as ::core::ffi::c_uint)
        .wrapping_add(l1)
        .wrapping_add(l3)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l2))
        .wrapping_add(2 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int) as uint8_t;
    *src.offset((1 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh19;
    let ref mut fresh20 = *src
        .offset((0 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
    *fresh20 = (t2
        .wrapping_add(t4)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t3))
        .wrapping_add(2 as ::core::ffi::c_uint)
        .wrapping_add(l2)
        .wrapping_add((3 as ::core::ffi::c_uint).wrapping_mul(l3))
        .wrapping_add(2 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh21 = *src
        .offset((1 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize);
    *fresh21 = *fresh20;
    *src.offset((2 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh21;
    let ref mut fresh22 = *src
        .offset((0 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
    *fresh22 = (t3
        .wrapping_add(t5)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t4))
        .wrapping_add(2 as ::core::ffi::c_uint)
        .wrapping_add(l3.wrapping_mul(4 as ::core::ffi::c_uint))
        .wrapping_add(2 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh23 = *src
        .offset((1 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
    *fresh23 = *fresh22;
    let ref mut fresh24 = *src
        .offset((2 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize);
    *fresh24 = *fresh23;
    *src.offset((3 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh24;
    let ref mut fresh25 = *src
        .offset((1 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
    *fresh25 = (t4
        .wrapping_add(t6)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t5))
        .wrapping_add(2 as ::core::ffi::c_uint)
        .wrapping_add(l3.wrapping_mul(4 as ::core::ffi::c_uint))
        .wrapping_add(2 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh26 = *src
        .offset((2 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
    *fresh26 = *fresh25;
    *src.offset((3 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) = *fresh26;
    let ref mut fresh27 = *src
        .offset((2 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
    *fresh27 = (t5
        .wrapping_add(t7)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t6))
        .wrapping_add(2 as ::core::ffi::c_uint)
        .wrapping_add(l3.wrapping_mul(4 as ::core::ffi::c_uint))
        .wrapping_add(2 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int) as uint8_t;
    *src.offset((3 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize) = *fresh27;
    *src.offset((3 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize) = (t6
        .wrapping_add(t7)
        .wrapping_add(1 as ::core::ffi::c_uint)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l3))
        .wrapping_add(1 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
}


unsafe extern "C" fn pred4x4_down_left_8_c(
    mut _src: *mut uint8_t,
    mut _topright: *const uint8_t,
    mut _stride: ptrdiff_t,
) {
    let mut src: *mut uint8_t = _src;
    let mut topright: *const uint8_t = _topright;
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    let t0: ::core::ffi::c_uint = *src
        .offset((0 as ::core::ffi::c_int - 1 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let t1: ::core::ffi::c_uint = *src
        .offset((1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let t2: ::core::ffi::c_uint = *src
        .offset((2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let t3: ::core::ffi::c_uint = *src
        .offset((3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let t4: ::core::ffi::c_uint = *topright.offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    let t5: ::core::ffi::c_uint = *topright.offset(1 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    let t6: ::core::ffi::c_uint = *topright.offset(2 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    let t7: ::core::ffi::c_uint = *topright.offset(3 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    *src.offset((0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = (t0
        .wrapping_add(t2)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t1))
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh367 = *src
        .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh367 = (t1
        .wrapping_add(t3)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t2))
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh367;
    let ref mut fresh368 = *src
        .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh368 = (t2
        .wrapping_add(t4)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t3))
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh369 = *src
        .offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh369 = *fresh368;
    *src.offset((2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh369;
    let ref mut fresh370 = *src
        .offset((0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh370 = (t3
        .wrapping_add(t5)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t4))
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh371 = *src
        .offset((1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh371 = *fresh370;
    let ref mut fresh372 = *src
        .offset((2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh372 = *fresh371;
    *src.offset((3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh372;
    let ref mut fresh373 = *src
        .offset((1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh373 = (t4
        .wrapping_add(t6)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t5))
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh374 = *src
        .offset((2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh374 = *fresh373;
    *src.offset((3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize) = *fresh374;
    let ref mut fresh375 = *src
        .offset((2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh375 = (t5
        .wrapping_add(t7)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t6))
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize) = *fresh375;
    *src.offset((3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize) = (t6
        .wrapping_add((3 as ::core::ffi::c_uint).wrapping_mul(t7))
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
}


unsafe extern "C" fn pred4x4_vertical_left_rv40(
    mut src: *mut uint8_t,
    mut topright: *const uint8_t,
    mut stride: ptrdiff_t,
    l0: ::core::ffi::c_int,
    l1: ::core::ffi::c_int,
    l2: ::core::ffi::c_int,
    l3: ::core::ffi::c_int,
    l4: ::core::ffi::c_int,
) {
    let t0: ::core::ffi::c_uint = *src
        .offset((0 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let t1: ::core::ffi::c_uint = *src
        .offset((1 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let t2: ::core::ffi::c_uint = *src
        .offset((2 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let t3: ::core::ffi::c_uint = *src
        .offset((3 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let t4: ::core::ffi::c_uint = *topright.offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    let t5: ::core::ffi::c_uint = *topright.offset(1 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    let t6: ::core::ffi::c_uint = *topright.offset(2 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    let t7: ::core::ffi::c_uint = *topright.offset(3 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    *src.offset((0 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = ((2
        as ::core::ffi::c_uint)
        .wrapping_mul(t0)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t1))
        .wrapping_add(l1 as ::core::ffi::c_uint)
        .wrapping_add((2 as ::core::ffi::c_int * l2) as ::core::ffi::c_uint)
        .wrapping_add(l3 as ::core::ffi::c_uint)
        .wrapping_add(4 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh6 = *src
        .offset((0 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
    *fresh6 = (t1.wrapping_add(t2).wrapping_add(1 as ::core::ffi::c_uint)
        >> 1 as ::core::ffi::c_int) as uint8_t;
    *src.offset((1 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh6;
    let ref mut fresh7 = *src
        .offset((1 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
    *fresh7 = (t2.wrapping_add(t3).wrapping_add(1 as ::core::ffi::c_uint)
        >> 1 as ::core::ffi::c_int) as uint8_t;
    *src.offset((2 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh7;
    let ref mut fresh8 = *src
        .offset((2 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
    *fresh8 = (t3.wrapping_add(t4).wrapping_add(1 as ::core::ffi::c_uint)
        >> 1 as ::core::ffi::c_int) as uint8_t;
    *src.offset((3 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh8;
    *src.offset((3 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize) = (t4
        .wrapping_add(t5)
        .wrapping_add(1 as ::core::ffi::c_uint) >> 1 as ::core::ffi::c_int) as uint8_t;
    *src.offset((0 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) = (t0
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t1))
        .wrapping_add(t2)
        .wrapping_add(l2 as ::core::ffi::c_uint)
        .wrapping_add((2 as ::core::ffi::c_int * l3) as ::core::ffi::c_uint)
        .wrapping_add(l4 as ::core::ffi::c_uint)
        .wrapping_add(4 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh9 = *src
        .offset((0 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
    *fresh9 = (t1
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t2))
        .wrapping_add(t3)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((1 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) = *fresh9;
    let ref mut fresh10 = *src
        .offset((1 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
    *fresh10 = (t2
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t3))
        .wrapping_add(t4)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((2 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) = *fresh10;
    let ref mut fresh11 = *src
        .offset((2 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
    *fresh11 = (t3
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t4))
        .wrapping_add(t5)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((3 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) = *fresh11;
    *src.offset((3 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize) = (t4
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t5))
        .wrapping_add(t6)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
}

unsafe extern "C" fn pred4x4_vertical_right_8_c(
    mut _src: *mut uint8_t,
    mut topright: *const uint8_t,
    mut _stride: ptrdiff_t,
) {
    let mut src: *mut uint8_t = _src;
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    let lt: ::core::ffi::c_int = *src
        .offset((-(1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_int;
    let t0: ::core::ffi::c_uint = *src
        .offset((0 as ::core::ffi::c_int - 1 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let t1: ::core::ffi::c_uint = *src
        .offset((1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let t2: ::core::ffi::c_uint = *src
        .offset((2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let t3: ::core::ffi::c_uint = *src
        .offset((3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let l0: ::core::ffi::c_uint = *src
        .offset((-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let l1: ::core::ffi::c_uint = *src
        .offset((-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let l2: ::core::ffi::c_uint = *src
        .offset((-(1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let l3: ::core::ffi::c_uint = *src
        .offset((-(1 as ::core::ffi::c_int) + 3 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let ref mut fresh352 = *src
        .offset((1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh352 = ((lt as ::core::ffi::c_uint)
        .wrapping_add(t0)
        .wrapping_add(1 as ::core::ffi::c_uint) >> 1 as ::core::ffi::c_int) as uint8_t;
    *src.offset((0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh352;
    let ref mut fresh353 = *src
        .offset((2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh353 = (t0.wrapping_add(t1).wrapping_add(1 as ::core::ffi::c_uint)
        >> 1 as ::core::ffi::c_int) as uint8_t;
    *src.offset((1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh353;
    let ref mut fresh354 = *src
        .offset((3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh354 = (t1.wrapping_add(t2).wrapping_add(1 as ::core::ffi::c_uint)
        >> 1 as ::core::ffi::c_int) as uint8_t;
    *src.offset((2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh354;
    *src.offset((3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = (t2
        .wrapping_add(t3)
        .wrapping_add(1 as ::core::ffi::c_uint) >> 1 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh355 = *src
        .offset((1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh355 = (l0
        .wrapping_add((2 as ::core::ffi::c_int * lt) as ::core::ffi::c_uint)
        .wrapping_add(t0)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize) = *fresh355;
    let ref mut fresh356 = *src
        .offset((2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh356 = ((lt as ::core::ffi::c_uint)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t0))
        .wrapping_add(t1)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize) = *fresh356;
    let ref mut fresh357 = *src
        .offset((3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh357 = (t0
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t1))
        .wrapping_add(t2)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize) = *fresh357;
    *src.offset((3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize) = (t1
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t2))
        .wrapping_add(t3)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize) = ((lt
        as ::core::ffi::c_uint)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l0))
        .wrapping_add(l1)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize) = (l0
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l1))
        .wrapping_add(l2)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
}



#[inline(always)]
unsafe extern "C" fn av_clip_uint8_c(mut a: ::core::ffi::c_int) -> uint8_t {
    if a & !(0xff as ::core::ffi::c_int) != 0 {
        return (!a >> 31 as ::core::ffi::c_int) as uint8_t
    } else {
        return a as uint8_t
    };
}
unsafe extern "C" fn pred4x4_vertical_left_rv40_c(
    mut src: *mut uint8_t,
    mut topright: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let l0: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 0 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l1: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 1 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l2: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 2 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l3: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 3 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l4: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 4 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l5: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 5 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l6: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 6 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l7: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 7 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    pred4x4_vertical_left_rv40(
        src,
        topright,
        stride,
        l0 as ::core::ffi::c_int,
        l1 as ::core::ffi::c_int,
        l2 as ::core::ffi::c_int,
        l3 as ::core::ffi::c_int,
        l4 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn pred4x4_vertical_left_rv40_nodown_c(
    mut src: *mut uint8_t,
    mut topright: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let l0: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 0 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l1: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 1 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l2: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 2 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l3: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 3 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    pred4x4_vertical_left_rv40(
        src,
        topright,
        stride,
        l0 as ::core::ffi::c_int,
        l1 as ::core::ffi::c_int,
        l2 as ::core::ffi::c_int,
        l3 as ::core::ffi::c_int,
        l3 as ::core::ffi::c_int,
    );
}

unsafe extern "C" fn pred4x4_vertical_left_8_c(
    mut _src: *mut uint8_t,
    mut _topright: *const uint8_t,
    mut _stride: ptrdiff_t,
) {
    let mut src: *mut uint8_t = _src;
    let mut topright: *const uint8_t = _topright;
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    let t0: ::core::ffi::c_uint = *src
        .offset((0 as ::core::ffi::c_int - 1 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let t1: ::core::ffi::c_uint = *src
        .offset((1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let t2: ::core::ffi::c_uint = *src
        .offset((2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let t3: ::core::ffi::c_uint = *src
        .offset((3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let t4: ::core::ffi::c_uint = *topright.offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    let t5: ::core::ffi::c_uint = *topright.offset(1 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    let t6: ::core::ffi::c_uint = *topright.offset(2 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    let t7: ::core::ffi::c_uint = *topright.offset(3 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    *src.offset((0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = (t0
        .wrapping_add(t1)
        .wrapping_add(1 as ::core::ffi::c_uint) >> 1 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh340 = *src
        .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh340 = (t1.wrapping_add(t2).wrapping_add(1 as ::core::ffi::c_uint)
        >> 1 as ::core::ffi::c_int) as uint8_t;
    *src.offset((1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh340;
    let ref mut fresh341 = *src
        .offset((1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh341 = (t2.wrapping_add(t3).wrapping_add(1 as ::core::ffi::c_uint)
        >> 1 as ::core::ffi::c_int) as uint8_t;
    *src.offset((2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh341;
    let ref mut fresh342 = *src
        .offset((2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh342 = (t3.wrapping_add(t4).wrapping_add(1 as ::core::ffi::c_uint)
        >> 1 as ::core::ffi::c_int) as uint8_t;
    *src.offset((3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh342;
    *src.offset((3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize) = (t4
        .wrapping_add(t5)
        .wrapping_add(1 as ::core::ffi::c_uint) >> 1 as ::core::ffi::c_int) as uint8_t;
    *src.offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize) = (t0
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t1))
        .wrapping_add(t2)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh343 = *src
        .offset((0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh343 = (t1
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t2))
        .wrapping_add(t3)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize) = *fresh343;
    let ref mut fresh344 = *src
        .offset((1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh344 = (t2
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t3))
        .wrapping_add(t4)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize) = *fresh344;
    let ref mut fresh345 = *src
        .offset((2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh345 = (t3
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t4))
        .wrapping_add(t5)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize) = *fresh345;
    *src.offset((3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize) = (t4
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t5))
        .wrapping_add(t6)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
}



unsafe extern "C" fn pred4x4_vertical_left_vp8_c(
    mut src: *mut uint8_t,
    mut topright: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let t0: ::core::ffi::c_uint = *src
        .offset((0 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let t1: ::core::ffi::c_uint = *src
        .offset((1 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let t2: ::core::ffi::c_uint = *src
        .offset((2 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let t3: ::core::ffi::c_uint = *src
        .offset((3 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let t4: ::core::ffi::c_uint = *topright.offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    let t5: ::core::ffi::c_uint = *topright.offset(1 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    let t6: ::core::ffi::c_uint = *topright.offset(2 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    let t7: ::core::ffi::c_uint = *topright.offset(3 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    *src.offset((0 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = (t0
        .wrapping_add(t1)
        .wrapping_add(1 as ::core::ffi::c_uint) >> 1 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh0 = *src
        .offset((0 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
    *fresh0 = (t1.wrapping_add(t2).wrapping_add(1 as ::core::ffi::c_uint)
        >> 1 as ::core::ffi::c_int) as uint8_t;
    *src.offset((1 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh0;
    let ref mut fresh1 = *src
        .offset((1 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
    *fresh1 = (t2.wrapping_add(t3).wrapping_add(1 as ::core::ffi::c_uint)
        >> 1 as ::core::ffi::c_int) as uint8_t;
    *src.offset((2 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh1;
    let ref mut fresh2 = *src
        .offset((2 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
    *fresh2 = (t3.wrapping_add(t4).wrapping_add(1 as ::core::ffi::c_uint)
        >> 1 as ::core::ffi::c_int) as uint8_t;
    *src.offset((3 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh2;
    *src.offset((0 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) = (t0
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t1))
        .wrapping_add(t2)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh3 = *src
        .offset((0 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
    *fresh3 = (t1
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t2))
        .wrapping_add(t3)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((1 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) = *fresh3;
    let ref mut fresh4 = *src
        .offset((1 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
    *fresh4 = (t2
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t3))
        .wrapping_add(t4)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((2 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) = *fresh4;
    let ref mut fresh5 = *src
        .offset((2 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
    *fresh5 = (t3
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t4))
        .wrapping_add(t5)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((3 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) = *fresh5;
    *src.offset((3 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize) = (t4
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t5))
        .wrapping_add(t6)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((3 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize) = (t5
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t6))
        .wrapping_add(t7)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
}



unsafe extern "C" fn pred4x4_horizontal_up_8_c(
    mut _src: *mut uint8_t,
    mut topright: *const uint8_t,
    mut _stride: ptrdiff_t,
) {
    let mut src: *mut uint8_t = _src;
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    let l0: ::core::ffi::c_uint = *src
        .offset((-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let l1: ::core::ffi::c_uint = *src
        .offset((-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let l2: ::core::ffi::c_uint = *src
        .offset((-(1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let l3: ::core::ffi::c_uint = *src
        .offset((-(1 as ::core::ffi::c_int) + 3 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    *src.offset((0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = (l0
        .wrapping_add(l1)
        .wrapping_add(1 as ::core::ffi::c_uint) >> 1 as ::core::ffi::c_int) as uint8_t;
    *src.offset((1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = (l0
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l1))
        .wrapping_add(l2)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh331 = *src
        .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh331 = (l1.wrapping_add(l2).wrapping_add(1 as ::core::ffi::c_uint)
        >> 1 as ::core::ffi::c_int) as uint8_t;
    *src.offset((2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh331;
    let ref mut fresh332 = *src
        .offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh332 = (l1
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l2))
        .wrapping_add(l3)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh332;
    let ref mut fresh333 = *src
        .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh333 = (l2.wrapping_add(l3).wrapping_add(1 as ::core::ffi::c_uint)
        >> 1 as ::core::ffi::c_int) as uint8_t;
    *src.offset((2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize) = *fresh333;
    let ref mut fresh334 = *src
        .offset((1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh334 = (l2
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l3))
        .wrapping_add(l3)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize) = *fresh334;
    let ref mut fresh335 = *src
        .offset((3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh335 = l3 as uint8_t;
    let ref mut fresh336 = *src
        .offset((2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh336 = *fresh335;
    let ref mut fresh337 = *src
        .offset((2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh337 = *fresh336;
    let ref mut fresh338 = *src
        .offset((0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh338 = *fresh337;
    let ref mut fresh339 = *src
        .offset((1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh339 = *fresh338;
    *src.offset((3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize) = *fresh339;
}

unsafe extern "C" fn pred4x4_horizontal_up_rv40_c(
    mut src: *mut uint8_t,
    mut topright: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let l0: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 0 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l1: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 1 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l2: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 2 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l3: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 3 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l4: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 4 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l5: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 5 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l6: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 6 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l7: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 7 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let t0: ::core::ffi::c_uint = *src
        .offset((0 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let t1: ::core::ffi::c_uint = *src
        .offset((1 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let t2: ::core::ffi::c_uint = *src
        .offset((2 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let t3: ::core::ffi::c_uint = *src
        .offset((3 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let t4: ::core::ffi::c_uint = *topright.offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    let t5: ::core::ffi::c_uint = *topright.offset(1 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    let t6: ::core::ffi::c_uint = *topright.offset(2 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    let t7: ::core::ffi::c_uint = *topright.offset(3 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    *src.offset((0 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = (t1
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t2))
        .wrapping_add(t3)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l0))
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l1))
        .wrapping_add(4 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int) as uint8_t;
    *src.offset((1 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = (t2
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t3))
        .wrapping_add(t4)
        .wrapping_add(l0)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l1))
        .wrapping_add(l2)
        .wrapping_add(4 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh28 = *src
        .offset((0 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize);
    *fresh28 = (t3
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t4))
        .wrapping_add(t5)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l1))
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l2))
        .wrapping_add(4 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int) as uint8_t;
    *src.offset((2 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh28;
    let ref mut fresh29 = *src
        .offset((1 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize);
    *fresh29 = (t4
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t5))
        .wrapping_add(t6)
        .wrapping_add(l1)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l2))
        .wrapping_add(l3)
        .wrapping_add(4 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int) as uint8_t;
    *src.offset((3 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh29;
    let ref mut fresh30 = *src
        .offset((0 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
    *fresh30 = (t5
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t6))
        .wrapping_add(t7)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l2))
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l3))
        .wrapping_add(4 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int) as uint8_t;
    *src.offset((2 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) = *fresh30;
    let ref mut fresh31 = *src
        .offset((1 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
    *fresh31 = (t6
        .wrapping_add((3 as ::core::ffi::c_uint).wrapping_mul(t7))
        .wrapping_add(l2)
        .wrapping_add((3 as ::core::ffi::c_uint).wrapping_mul(l3))
        .wrapping_add(4 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int) as uint8_t;
    *src.offset((3 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) = *fresh31;
    let ref mut fresh32 = *src
        .offset((1 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
    *fresh32 = (l3
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l4))
        .wrapping_add(l5)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((3 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize) = *fresh32;
    let ref mut fresh33 = *src
        .offset((2 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
    *fresh33 = (t6
        .wrapping_add(t7)
        .wrapping_add(l3)
        .wrapping_add(l4)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((0 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize) = *fresh33;
    *src.offset((2 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize) = (l4
        .wrapping_add(l5)
        .wrapping_add(1 as ::core::ffi::c_uint) >> 1 as ::core::ffi::c_int) as uint8_t;
    *src.offset((3 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize) = (l4
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l5))
        .wrapping_add(l6)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
}

unsafe extern "C" fn pred4x4_horizontal_down_8_c(
    mut _src: *mut uint8_t,
    mut topright: *const uint8_t,
    mut _stride: ptrdiff_t,
) {
    let mut src: *mut uint8_t = _src;
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    let lt: ::core::ffi::c_int = *src
        .offset((-(1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_int;
    let t0: ::core::ffi::c_uint = *src
        .offset((0 as ::core::ffi::c_int - 1 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let t1: ::core::ffi::c_uint = *src
        .offset((1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let t2: ::core::ffi::c_uint = *src
        .offset((2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let t3: ::core::ffi::c_uint = *src
        .offset((3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let l0: ::core::ffi::c_uint = *src
        .offset((-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let l1: ::core::ffi::c_uint = *src
        .offset((-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let l2: ::core::ffi::c_uint = *src
        .offset((-(1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let l3: ::core::ffi::c_uint = *src
        .offset((-(1 as ::core::ffi::c_int) + 3 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_uint;
    let ref mut fresh346 = *src
        .offset((2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh346 = ((lt as ::core::ffi::c_uint)
        .wrapping_add(l0)
        .wrapping_add(1 as ::core::ffi::c_uint) >> 1 as ::core::ffi::c_int) as uint8_t;
    *src.offset((0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh346;
    let ref mut fresh347 = *src
        .offset((3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh347 = (l0
        .wrapping_add((2 as ::core::ffi::c_int * lt) as ::core::ffi::c_uint)
        .wrapping_add(t0)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh347;
    *src.offset((2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = ((lt
        as ::core::ffi::c_uint)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t0))
        .wrapping_add(t1)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = (t0
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t1))
        .wrapping_add(t2)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh348 = *src
        .offset((2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh348 = (l0.wrapping_add(l1).wrapping_add(1 as ::core::ffi::c_uint)
        >> 1 as ::core::ffi::c_int) as uint8_t;
    *src.offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize) = *fresh348;
    let ref mut fresh349 = *src
        .offset((3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh349 = ((lt as ::core::ffi::c_uint)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l0))
        .wrapping_add(l1)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize) = *fresh349;
    let ref mut fresh350 = *src
        .offset((2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh350 = (l1.wrapping_add(l2).wrapping_add(1 as ::core::ffi::c_uint)
        >> 1 as ::core::ffi::c_int) as uint8_t;
    *src.offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize) = *fresh350;
    let ref mut fresh351 = *src
        .offset((3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh351 = (l0
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l1))
        .wrapping_add(l2)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize) = *fresh351;
    *src.offset((0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize) = (l2
        .wrapping_add(l3)
        .wrapping_add(1 as ::core::ffi::c_uint) >> 1 as ::core::ffi::c_int) as uint8_t;
    *src.offset((1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize) = (l1
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l2))
        .wrapping_add(l3)
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
}



unsafe extern "C" fn pred4x4_horizontal_up_rv40_nodown_c(
    mut src: *mut uint8_t,
    mut topright: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let l0: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 0 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l1: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 1 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l2: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 2 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let l3: ::core::ffi::c_uint = *src
        .offset(
            (-(1 as ::core::ffi::c_int) as ptrdiff_t + 3 as ptrdiff_t * stride) as isize,
        ) as ::core::ffi::c_uint;
    let t0: ::core::ffi::c_uint = *src
        .offset((0 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let t1: ::core::ffi::c_uint = *src
        .offset((1 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let t2: ::core::ffi::c_uint = *src
        .offset((2 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let t3: ::core::ffi::c_uint = *src
        .offset((3 as ptrdiff_t - 1 as ptrdiff_t * stride) as isize)
        as ::core::ffi::c_uint;
    let t4: ::core::ffi::c_uint = *topright.offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    let t5: ::core::ffi::c_uint = *topright.offset(1 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    let t6: ::core::ffi::c_uint = *topright.offset(2 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    let t7: ::core::ffi::c_uint = *topright.offset(3 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_uint;
    *src.offset((0 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = (t1
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t2))
        .wrapping_add(t3)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l0))
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l1))
        .wrapping_add(4 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int) as uint8_t;
    *src.offset((1 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = (t2
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t3))
        .wrapping_add(t4)
        .wrapping_add(l0)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l1))
        .wrapping_add(l2)
        .wrapping_add(4 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh12 = *src
        .offset((0 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize);
    *fresh12 = (t3
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t4))
        .wrapping_add(t5)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l1))
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l2))
        .wrapping_add(4 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int) as uint8_t;
    *src.offset((2 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh12;
    let ref mut fresh13 = *src
        .offset((1 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize);
    *fresh13 = (t4
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t5))
        .wrapping_add(t6)
        .wrapping_add(l1)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l2))
        .wrapping_add(l3)
        .wrapping_add(4 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int) as uint8_t;
    *src.offset((3 as ptrdiff_t + 0 as ptrdiff_t * stride) as isize) = *fresh13;
    let ref mut fresh14 = *src
        .offset((0 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
    *fresh14 = (t5
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(t6))
        .wrapping_add(t7)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l2))
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l3))
        .wrapping_add(4 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int) as uint8_t;
    *src.offset((2 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) = *fresh14;
    let ref mut fresh15 = *src
        .offset((1 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
    *fresh15 = (t6
        .wrapping_add((3 as ::core::ffi::c_uint).wrapping_mul(t7))
        .wrapping_add(l2)
        .wrapping_add((3 as ::core::ffi::c_uint).wrapping_mul(l3))
        .wrapping_add(4 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int) as uint8_t;
    *src.offset((3 as ptrdiff_t + 1 as ptrdiff_t * stride) as isize) = *fresh15;
    let ref mut fresh16 = *src
        .offset((1 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
    *fresh16 = l3 as uint8_t;
    *src.offset((3 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize) = *fresh16;
    let ref mut fresh17 = *src
        .offset((2 as ptrdiff_t + 2 as ptrdiff_t * stride) as isize);
    *fresh17 = (t6
        .wrapping_add(t7)
        .wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(l3))
        .wrapping_add(2 as ::core::ffi::c_uint) >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((0 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize) = *fresh17;
    let ref mut fresh18 = *src
        .offset((3 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize);
    *fresh18 = l3 as uint8_t;
    *src.offset((2 as ptrdiff_t + 3 as ptrdiff_t * stride) as isize) = *fresh18;
}
#[inline(always)]
unsafe extern "C" fn av_clip_uintp2_c(
    mut a: ::core::ffi::c_int,
    mut p: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    if a as ::core::ffi::c_uint
        & !((1 as ::core::ffi::c_uint) << p).wrapping_sub(1 as ::core::ffi::c_uint) != 0
    {
        return (!a >> 31 as ::core::ffi::c_int) as ::core::ffi::c_uint
            & ((1 as ::core::ffi::c_uint) << p).wrapping_sub(1 as ::core::ffi::c_uint)
    } else {
        return a as ::core::ffi::c_uint
    };
}



unsafe extern "C" fn pred16x16_vertical_8_c(
    mut _src: *mut uint8_t,
    mut _stride: ptrdiff_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut src: *mut uint8_t = _src;
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    let a: uint32_t = (*((src.offset(-(stride as isize)) as *mut uint32_t)
        .offset(0 as ::core::ffi::c_int as isize) as *const av_alias32))
        .u32_0;
    let b: uint32_t = (*((src.offset(-(stride as isize)) as *mut uint32_t)
        .offset(1 as ::core::ffi::c_int as isize) as *const av_alias32))
        .u32_0;
    let c: uint32_t = (*((src.offset(-(stride as isize)) as *mut uint32_t)
        .offset(2 as ::core::ffi::c_int as isize) as *const av_alias32))
        .u32_0;
    let d: uint32_t = (*((src.offset(-(stride as isize)) as *mut uint32_t)
        .offset(3 as ::core::ffi::c_int as isize) as *const av_alias32))
        .u32_0;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        (*((src.offset((i * stride) as isize) as *mut uint32_t)
            .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = a;
        (*((src.offset((i * stride) as isize) as *mut uint32_t)
            .offset(1 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = b;
        (*((src.offset((i * stride) as isize) as *mut uint32_t)
            .offset(2 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = c;
        (*((src.offset((i * stride) as isize) as *mut uint32_t)
            .offset(3 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = d;
        i += 1;
    }
}

unsafe extern "C" fn pred4x4_tm_vp8_c(
    mut src: *mut uint8_t,
    mut topright: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut cm: *const uint8_t = (&raw const ff_crop_tab as *const uint8_t)
        .offset(1024 as ::core::ffi::c_int as isize)
        .offset(
            -(*src.offset((-(1 as ::core::ffi::c_int) as ptrdiff_t - stride) as isize)
                as ::core::ffi::c_int as isize),
        );
    let mut top: *mut uint8_t = src.offset(-(stride as isize));
    let mut y: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y < 4 as ::core::ffi::c_int {
        let mut cm_in: *const uint8_t = cm
            .offset(
                *src.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    as isize,
            );
        *src.offset(0 as ::core::ffi::c_int as isize) = *cm_in
            .offset(*top.offset(0 as ::core::ffi::c_int as isize) as isize);
        *src.offset(1 as ::core::ffi::c_int as isize) = *cm_in
            .offset(*top.offset(1 as ::core::ffi::c_int as isize) as isize);
        *src.offset(2 as ::core::ffi::c_int as isize) = *cm_in
            .offset(*top.offset(2 as ::core::ffi::c_int as isize) as isize);
        *src.offset(3 as ::core::ffi::c_int as isize) = *cm_in
            .offset(*top.offset(3 as ::core::ffi::c_int as isize) as isize);
        src = src.offset(stride as isize);
        y += 1;
    }
}




unsafe extern "C" fn pred16x16_horizontal_8_c(
    mut _src: *mut uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut src: *mut uint8_t = _src;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        let a: uint32_t = (*src
            .offset(
                (-(1 as ::core::ffi::c_int) as ptrdiff_t + i as ptrdiff_t * stride)
                    as isize,
            ) as uint32_t)
            .wrapping_mul(0x1010101 as uint32_t);
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = a;
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(1 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = a;
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(2 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = a;
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(3 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = a;
        i += 1;
    }
}
unsafe extern "C" fn pred16x16_plane_svq3_c(
    mut src: *mut uint8_t,
    mut stride: ptrdiff_t,
) {
    pred16x16_plane_compat_8_c(
        src,
        stride,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn pred16x16_plane_rv40_c(
    mut src: *mut uint8_t,
    mut stride: ptrdiff_t,
) {
    pred16x16_plane_compat_8_c(
        src,
        stride,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn pred16x16_tm_vp8_c(mut src: *mut uint8_t, mut stride: ptrdiff_t) {
    let mut cm: *const uint8_t = (&raw const ff_crop_tab as *const uint8_t)
        .offset(1024 as ::core::ffi::c_int as isize)
        .offset(
            -(*src.offset((-(1 as ::core::ffi::c_int) as ptrdiff_t - stride) as isize)
                as ::core::ffi::c_int as isize),
        );
    let mut top: *mut uint8_t = src.offset(-(stride as isize));
    let mut y: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y < 16 as ::core::ffi::c_int {
        let mut cm_in: *const uint8_t = cm
            .offset(
                *src.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    as isize,
            );
        *src.offset(0 as ::core::ffi::c_int as isize) = *cm_in
            .offset(*top.offset(0 as ::core::ffi::c_int as isize) as isize);
        *src.offset(1 as ::core::ffi::c_int as isize) = *cm_in
            .offset(*top.offset(1 as ::core::ffi::c_int as isize) as isize);
        *src.offset(2 as ::core::ffi::c_int as isize) = *cm_in
            .offset(*top.offset(2 as ::core::ffi::c_int as isize) as isize);
        *src.offset(3 as ::core::ffi::c_int as isize) = *cm_in
            .offset(*top.offset(3 as ::core::ffi::c_int as isize) as isize);
        *src.offset(4 as ::core::ffi::c_int as isize) = *cm_in
            .offset(*top.offset(4 as ::core::ffi::c_int as isize) as isize);
        *src.offset(5 as ::core::ffi::c_int as isize) = *cm_in
            .offset(*top.offset(5 as ::core::ffi::c_int as isize) as isize);
        *src.offset(6 as ::core::ffi::c_int as isize) = *cm_in
            .offset(*top.offset(6 as ::core::ffi::c_int as isize) as isize);
        *src.offset(7 as ::core::ffi::c_int as isize) = *cm_in
            .offset(*top.offset(7 as ::core::ffi::c_int as isize) as isize);
        *src.offset(8 as ::core::ffi::c_int as isize) = *cm_in
            .offset(*top.offset(8 as ::core::ffi::c_int as isize) as isize);
        *src.offset(9 as ::core::ffi::c_int as isize) = *cm_in
            .offset(*top.offset(9 as ::core::ffi::c_int as isize) as isize);
        *src.offset(10 as ::core::ffi::c_int as isize) = *cm_in
            .offset(*top.offset(10 as ::core::ffi::c_int as isize) as isize);
        *src.offset(11 as ::core::ffi::c_int as isize) = *cm_in
            .offset(*top.offset(11 as ::core::ffi::c_int as isize) as isize);
        *src.offset(12 as ::core::ffi::c_int as isize) = *cm_in
            .offset(*top.offset(12 as ::core::ffi::c_int as isize) as isize);
        *src.offset(13 as ::core::ffi::c_int as isize) = *cm_in
            .offset(*top.offset(13 as ::core::ffi::c_int as isize) as isize);
        *src.offset(14 as ::core::ffi::c_int as isize) = *cm_in
            .offset(*top.offset(14 as ::core::ffi::c_int as isize) as isize);
        *src.offset(15 as ::core::ffi::c_int as isize) = *cm_in
            .offset(*top.offset(15 as ::core::ffi::c_int as isize) as isize);
        src = src.offset(stride as isize);
        y += 1;
    }
}
unsafe extern "C" fn pred16x16_dc_8_c(mut _src: *mut uint8_t, mut stride: ptrdiff_t) {
    let mut i: ::core::ffi::c_int = 0;
    let mut dc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut src: *mut uint8_t = _src;
    let mut dcsplat: uint32_t = 0;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        dc
            += *src
                .offset(
                    (-(1 as ::core::ffi::c_int) as ptrdiff_t + i as ptrdiff_t * stride)
                        as isize,
                ) as ::core::ffi::c_int;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        dc += *src.offset((i as ptrdiff_t - stride) as isize) as ::core::ffi::c_int;
        i += 1;
    }
    dcsplat = ((dc + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int)
        as ::core::ffi::c_uint)
        .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut av_alias32)).u32_0 = dcsplat;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut av_alias32)).u32_0 = dcsplat;
        (*(src.offset(8 as ::core::ffi::c_int as isize) as *mut av_alias32)).u32_0 = dcsplat;
        (*(src.offset(12 as ::core::ffi::c_int as isize) as *mut av_alias32)).u32_0 = dcsplat;
        src = src.offset(stride as isize);
        i += 1;
    }
}




unsafe extern "C" fn pred8x8_left_dc_rv40_c(
    mut src: *mut uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut dc0: ::core::ffi::c_uint = 0;
    dc0 = 0 as ::core::ffi::c_uint;
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        dc0 = dc0
            .wrapping_add(
                *src
                    .offset(
                        (-(1 as ::core::ffi::c_int) as ptrdiff_t
                            + i as ptrdiff_t * stride) as isize,
                    ) as ::core::ffi::c_uint,
            );
        i += 1;
    }
    dc0 = (0x1010101 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            dc0.wrapping_add(4 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int,
        );
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        let ref mut fresh57 = *(src.offset((i as ptrdiff_t * stride) as isize)
            as *mut uint32_t)
            .offset(1 as ::core::ffi::c_int as isize);
        *fresh57 = dc0 as uint32_t;
        *(src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(0 as ::core::ffi::c_int as isize) = *fresh57;
        i += 1;
    }
}


unsafe extern "C" fn pred16x16_left_dc_8_c(
    mut _src: *mut uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut dc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut src: *mut uint8_t = _src;
    let mut dcsplat: uint32_t = 0;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        dc
            += *src
                .offset(
                    (-(1 as ::core::ffi::c_int) as ptrdiff_t + i as ptrdiff_t * stride)
                        as isize,
                ) as ::core::ffi::c_int;
        i += 1;
    }
    dcsplat = ((dc + 8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
        as ::core::ffi::c_uint)
        .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut av_alias32)).u32_0 = dcsplat;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut av_alias32)).u32_0 = dcsplat;
        (*(src.offset(8 as ::core::ffi::c_int as isize) as *mut av_alias32)).u32_0 = dcsplat;
        (*(src.offset(12 as ::core::ffi::c_int as isize) as *mut av_alias32)).u32_0 = dcsplat;
        src = src.offset(stride as isize);
        i += 1;
    }
}


unsafe extern "C" fn pred8x8_top_dc_rv40_c(
    mut src: *mut uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut dc0: ::core::ffi::c_uint = 0;
    dc0 = 0 as ::core::ffi::c_uint;
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        dc0 = dc0
            .wrapping_add(
                *src.offset((i as ptrdiff_t - stride) as isize) as ::core::ffi::c_uint,
            );
        i += 1;
    }
    dc0 = (0x1010101 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            dc0.wrapping_add(4 as ::core::ffi::c_uint) >> 3 as ::core::ffi::c_int,
        );
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        let ref mut fresh56 = *(src.offset((i as ptrdiff_t * stride) as isize)
            as *mut uint32_t)
            .offset(1 as ::core::ffi::c_int as isize);
        *fresh56 = dc0 as uint32_t;
        *(src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(0 as ::core::ffi::c_int as isize) = *fresh56;
        i += 1;
    }
}
unsafe extern "C" fn pred16x16_top_dc_8_c(
    mut _src: *mut uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut dc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut src: *mut uint8_t = _src;
    let mut dcsplat: uint32_t = 0;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        dc += *src.offset((i as ptrdiff_t - stride) as isize) as ::core::ffi::c_int;
        i += 1;
    }
    dcsplat = ((dc + 8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
        as ::core::ffi::c_uint)
        .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut av_alias32)).u32_0 = dcsplat;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut av_alias32)).u32_0 = dcsplat;
        (*(src.offset(8 as ::core::ffi::c_int as isize) as *mut av_alias32)).u32_0 = dcsplat;
        (*(src.offset(12 as ::core::ffi::c_int as isize) as *mut av_alias32)).u32_0 = dcsplat;
        src = src.offset(stride as isize);
        i += 1;
    }
}




unsafe extern "C" fn pred8x8_dc_rv40_c(mut src: *mut uint8_t, mut stride: ptrdiff_t) {
    let mut i: ::core::ffi::c_int = 0;
    let mut dc0: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        dc0 = dc0
            .wrapping_add(
                (*src
                    .offset(
                        (-(1 as ::core::ffi::c_int) as ptrdiff_t
                            + i as ptrdiff_t * stride) as isize,
                    ) as ::core::ffi::c_int
                    + *src.offset((i as ptrdiff_t - stride) as isize)
                        as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
        dc0 = dc0
            .wrapping_add(
                *src
                    .offset(
                        ((4 as ::core::ffi::c_int + i) as ptrdiff_t - stride) as isize,
                    ) as ::core::ffi::c_uint,
            );
        dc0 = dc0
            .wrapping_add(
                *src
                    .offset(
                        (-(1 as ::core::ffi::c_int) as ptrdiff_t
                            + (i + 4 as ::core::ffi::c_int) as ptrdiff_t * stride)
                            as isize,
                    ) as ::core::ffi::c_uint,
            );
        i += 1;
    }
    dc0 = (0x1010101 as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_mul(
            dc0.wrapping_add(8 as ::core::ffi::c_uint) >> 4 as ::core::ffi::c_int,
        );
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        *(src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(0 as ::core::ffi::c_int as isize) = dc0 as uint32_t;
        *(src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(1 as ::core::ffi::c_int as isize) = dc0 as uint32_t;
        i += 1;
    }
    i = 4 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        *(src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(0 as ::core::ffi::c_int as isize) = dc0 as uint32_t;
        *(src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(1 as ::core::ffi::c_int as isize) = dc0 as uint32_t;
        i += 1;
    }
}




unsafe extern "C" fn pred16x16_128_dc_8_c(
    mut _src: *mut uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut src: *mut uint8_t = _src;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut av_alias32)).u32_0 = ((((1
            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            + 0 as ::core::ffi::c_int) as ::core::ffi::c_uint)
            .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut av_alias32)).u32_0 = ((((1
            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            + 0 as ::core::ffi::c_int) as ::core::ffi::c_uint)
            .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
        (*(src.offset(8 as ::core::ffi::c_int as isize) as *mut av_alias32)).u32_0 = ((((1
            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            + 0 as ::core::ffi::c_int) as ::core::ffi::c_uint)
            .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
        (*(src.offset(12 as ::core::ffi::c_int as isize) as *mut av_alias32)).u32_0 = ((((1
            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            + 0 as ::core::ffi::c_int) as ::core::ffi::c_uint)
            .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
        src = src.offset(stride as isize);
        i += 1;
    }
}
unsafe extern "C" fn pred16x16_127_dc_8_c(
    mut _src: *mut uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut src: *mut uint8_t = _src;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut av_alias32)).u32_0 = ((((1
            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint)
            .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut av_alias32)).u32_0 = ((((1
            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint)
            .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
        (*(src.offset(8 as ::core::ffi::c_int as isize) as *mut av_alias32)).u32_0 = ((((1
            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint)
            .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
        (*(src.offset(12 as ::core::ffi::c_int as isize) as *mut av_alias32)).u32_0 = ((((1
            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint)
            .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
        src = src.offset(stride as isize);
        i += 1;
    }
}
unsafe extern "C" fn pred16x16_129_dc_8_c(
    mut _src: *mut uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut src: *mut uint8_t = _src;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut av_alias32)).u32_0 = ((((1
            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            + 1 as ::core::ffi::c_int) as ::core::ffi::c_uint)
            .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut av_alias32)).u32_0 = ((((1
            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            + 1 as ::core::ffi::c_int) as ::core::ffi::c_uint)
            .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
        (*(src.offset(8 as ::core::ffi::c_int as isize) as *mut av_alias32)).u32_0 = ((((1
            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            + 1 as ::core::ffi::c_int) as ::core::ffi::c_uint)
            .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
        (*(src.offset(12 as ::core::ffi::c_int as isize) as *mut av_alias32)).u32_0 = ((((1
            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            + 1 as ::core::ffi::c_int) as ::core::ffi::c_uint)
            .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
        src = src.offset(stride as isize);
        i += 1;
    }
}




#[inline]
unsafe extern "C" fn pred16x16_plane_compat_8_c(
    mut _src: *mut uint8_t,
    mut _stride: ptrdiff_t,
    svq3: ::core::ffi::c_int,
    rv40: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut a: ::core::ffi::c_int = 0;
    let mut src: *mut uint8_t = _src;
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    let src0: *const uint8_t = src
        .offset(7 as ::core::ffi::c_int as isize)
        .offset(-(stride as isize));
    let mut src1: *const uint8_t = src
        .offset((8 as ::core::ffi::c_int * stride) as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    let mut src2: *const uint8_t = src1
        .offset(-((2 as ::core::ffi::c_int * stride) as isize));
    let mut H: ::core::ffi::c_int = *src0.offset(1 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        - *src0.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int;
    let mut V: ::core::ffi::c_int = *src1.offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        - *src2.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
    k = 2 as ::core::ffi::c_int;
    while k <= 8 as ::core::ffi::c_int {
        src1 = src1.offset(stride as isize);
        src2 = src2.offset(-(stride as isize));
        H
            += k
                * (*src0.offset(k as isize) as ::core::ffi::c_int
                    - *src0.offset(-k as isize) as ::core::ffi::c_int);
        V
            += k
                * (*src1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    - *src2.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int);
        k += 1;
    }
    if svq3 != 0 {
        H = 5 as ::core::ffi::c_int * (H / 4 as ::core::ffi::c_int)
            / 16 as ::core::ffi::c_int;
        V = 5 as ::core::ffi::c_int * (V / 4 as ::core::ffi::c_int)
            / 16 as ::core::ffi::c_int;
        i = H;
        H = V;
        V = i;
    } else if rv40 != 0 {
        H = H + (H >> 2 as ::core::ffi::c_int) >> 4 as ::core::ffi::c_int;
        V = V + (V >> 2 as ::core::ffi::c_int) >> 4 as ::core::ffi::c_int;
    } else {
        H = 5 as ::core::ffi::c_int * H + 32 as ::core::ffi::c_int
            >> 6 as ::core::ffi::c_int;
        V = 5 as ::core::ffi::c_int * V + 32 as ::core::ffi::c_int
            >> 6 as ::core::ffi::c_int;
    }
    a = 16 as ::core::ffi::c_int
        * (*src1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *src2.offset(16 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) - 7 as ::core::ffi::c_int * (V + H);
    j = 16 as ::core::ffi::c_int;
    while j > 0 as ::core::ffi::c_int {
        let mut b: ::core::ffi::c_int = a;
        a += V;
        i = -(16 as ::core::ffi::c_int);
        while i < 0 as ::core::ffi::c_int {
            *src.offset((16 as ::core::ffi::c_int + i) as isize) = av_clip_uint8_c(
                b >> 5 as ::core::ffi::c_int,
            );
            *src.offset((17 as ::core::ffi::c_int + i) as isize) = av_clip_uint8_c(
                b + H >> 5 as ::core::ffi::c_int,
            );
            *src.offset((18 as ::core::ffi::c_int + i) as isize) = av_clip_uint8_c(
                b + 2 as ::core::ffi::c_int * H >> 5 as ::core::ffi::c_int,
            );
            *src.offset((19 as ::core::ffi::c_int + i) as isize) = av_clip_uint8_c(
                b + 3 as ::core::ffi::c_int * H >> 5 as ::core::ffi::c_int,
            );
            b += 4 as ::core::ffi::c_int * H;
            i += 4 as ::core::ffi::c_int;
        }
        src = src.offset(stride as isize);
        j -= 1;
    }
}
unsafe extern "C" fn pred8x8_tm_vp8_c(mut src: *mut uint8_t, mut stride: ptrdiff_t) {
    let mut cm: *const uint8_t = (&raw const ff_crop_tab as *const uint8_t)
        .offset(1024 as ::core::ffi::c_int as isize)
        .offset(
            -(*src.offset((-(1 as ::core::ffi::c_int) as ptrdiff_t - stride) as isize)
                as ::core::ffi::c_int as isize),
        );
    let mut top: *mut uint8_t = src.offset(-(stride as isize));
    let mut y: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y < 8 as ::core::ffi::c_int {
        let mut cm_in: *const uint8_t = cm
            .offset(
                *src.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    as isize,
            );
        *src.offset(0 as ::core::ffi::c_int as isize) = *cm_in
            .offset(*top.offset(0 as ::core::ffi::c_int as isize) as isize);
        *src.offset(1 as ::core::ffi::c_int as isize) = *cm_in
            .offset(*top.offset(1 as ::core::ffi::c_int as isize) as isize);
        *src.offset(2 as ::core::ffi::c_int as isize) = *cm_in
            .offset(*top.offset(2 as ::core::ffi::c_int as isize) as isize);
        *src.offset(3 as ::core::ffi::c_int as isize) = *cm_in
            .offset(*top.offset(3 as ::core::ffi::c_int as isize) as isize);
        *src.offset(4 as ::core::ffi::c_int as isize) = *cm_in
            .offset(*top.offset(4 as ::core::ffi::c_int as isize) as isize);
        *src.offset(5 as ::core::ffi::c_int as isize) = *cm_in
            .offset(*top.offset(5 as ::core::ffi::c_int as isize) as isize);
        *src.offset(6 as ::core::ffi::c_int as isize) = *cm_in
            .offset(*top.offset(6 as ::core::ffi::c_int as isize) as isize);
        *src.offset(7 as ::core::ffi::c_int as isize) = *cm_in
            .offset(*top.offset(7 as ::core::ffi::c_int as isize) as isize);
        src = src.offset(stride as isize);
        y += 1;
    }
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ff_h264_pred_init(
    mut h: *mut H264PredContext,
    mut codec_id: ::core::ffi::c_int,
    bit_depth: ::core::ffi::c_int,
    mut chroma_format_idc: ::core::ffi::c_int,
) {
    {
            if !(bit_depth <= 8 as ::core::ffi::c_int) {
                av_log(
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    0 as ::core::ffi::c_int,
                    b"Assertion %s failed at %s:%d\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"bit_depth<=8\0" as *const u8 as *const ::core::ffi::c_char,
                    b"src/libavcodec/h264pred.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    554 as ::core::ffi::c_int,
                );
                abort();
            }
            (*h).pred4x4[0 as ::core::ffi::c_int as usize] = Some(
                pred4x4_vertical_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
                >;
            (*h).pred4x4[1 as ::core::ffi::c_int as usize] = Some(
                pred4x4_horizontal_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
                >;
            (*h).pred4x4[2 as ::core::ffi::c_int as usize] = Some(
                pred4x4_dc_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
                >;
            (*h).pred4x4[3 as ::core::ffi::c_int as usize] = Some(
                pred4x4_down_left_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
                >;
            (*h).pred4x4[4 as ::core::ffi::c_int as usize] = Some(
                pred4x4_down_right_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
                >;
            (*h).pred4x4[5 as ::core::ffi::c_int as usize] = Some(
                pred4x4_vertical_right_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
                >;
            (*h).pred4x4[6 as ::core::ffi::c_int as usize] = Some(
                pred4x4_horizontal_down_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
                >;
            (*h).pred4x4[7 as ::core::ffi::c_int as usize] = Some(
                pred4x4_vertical_left_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
                >;
            (*h).pred4x4[8 as ::core::ffi::c_int as usize] = Some(
                pred4x4_horizontal_up_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
                >;
            (*h).pred4x4[9 as ::core::ffi::c_int as usize] = Some(
                pred4x4_left_dc_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
                >;
            (*h).pred4x4[10 as ::core::ffi::c_int as usize] = Some(
                pred4x4_top_dc_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
                >;
            if 8 as ::core::ffi::c_int > 8 as ::core::ffi::c_int
                || codec_id != AV_CODEC_ID_VP8 as ::core::ffi::c_int
            {
                (*h).pred4x4[11 as ::core::ffi::c_int as usize] = Some(
                    pred4x4_128_dc_8_c
                        as unsafe extern "C" fn(
                            *mut uint8_t,
                            *const uint8_t,
                            ptrdiff_t,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut uint8_t,
                            *const uint8_t,
                            ptrdiff_t,
                        ) -> (),
                    >;
            }
            (*h).pred8x8l[0 as ::core::ffi::c_int as usize] = Some(
                pred8x8l_vertical_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ptrdiff_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut uint8_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ptrdiff_t,
                    ) -> (),
                >;
            (*h).pred8x8l[1 as ::core::ffi::c_int as usize] = Some(
                pred8x8l_horizontal_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ptrdiff_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut uint8_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ptrdiff_t,
                    ) -> (),
                >;
            (*h).pred8x8l[2 as ::core::ffi::c_int as usize] = Some(
                pred8x8l_dc_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ptrdiff_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut uint8_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ptrdiff_t,
                    ) -> (),
                >;
            (*h).pred8x8l[3 as ::core::ffi::c_int as usize] = Some(
                pred8x8l_down_left_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ptrdiff_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut uint8_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ptrdiff_t,
                    ) -> (),
                >;
            (*h).pred8x8l[4 as ::core::ffi::c_int as usize] = Some(
                pred8x8l_down_right_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ptrdiff_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut uint8_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ptrdiff_t,
                    ) -> (),
                >;
            (*h).pred8x8l[5 as ::core::ffi::c_int as usize] = Some(
                pred8x8l_vertical_right_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ptrdiff_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut uint8_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ptrdiff_t,
                    ) -> (),
                >;
            (*h).pred8x8l[6 as ::core::ffi::c_int as usize] = Some(
                pred8x8l_horizontal_down_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ptrdiff_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut uint8_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ptrdiff_t,
                    ) -> (),
                >;
            (*h).pred8x8l[7 as ::core::ffi::c_int as usize] = Some(
                pred8x8l_vertical_left_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ptrdiff_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut uint8_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ptrdiff_t,
                    ) -> (),
                >;
            (*h).pred8x8l[8 as ::core::ffi::c_int as usize] = Some(
                pred8x8l_horizontal_up_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ptrdiff_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut uint8_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ptrdiff_t,
                    ) -> (),
                >;
            (*h).pred8x8l[9 as ::core::ffi::c_int as usize] = Some(
                pred8x8l_left_dc_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ptrdiff_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut uint8_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ptrdiff_t,
                    ) -> (),
                >;
            (*h).pred8x8l[10 as ::core::ffi::c_int as usize] = Some(
                pred8x8l_top_dc_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ptrdiff_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut uint8_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ptrdiff_t,
                    ) -> (),
                >;
            (*h).pred8x8l[11 as ::core::ffi::c_int as usize] = Some(
                pred8x8l_128_dc_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ptrdiff_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut uint8_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ptrdiff_t,
                    ) -> (),
                >;
            if chroma_format_idc <= 1 as ::core::ffi::c_int {
                (*h).pred8x8[2 as ::core::ffi::c_int as usize] = Some(
                    pred8x8_vertical_8_c
                        as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
                (*h).pred8x8[1 as ::core::ffi::c_int as usize] = Some(
                    pred8x8_horizontal_8_c
                        as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
                (*h).pred8x8[3 as ::core::ffi::c_int as usize] = Some(
                    pred8x8_plane_8_c
                        as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
            } else {
                (*h).pred8x8[2 as ::core::ffi::c_int as usize] = Some(
                    pred8x16_vertical_8_c
                        as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
                (*h).pred8x8[1 as ::core::ffi::c_int as usize] = Some(
                    pred8x16_horizontal_8_c
                        as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
                (*h).pred8x8[3 as ::core::ffi::c_int as usize] = Some(
                    pred8x16_plane_8_c
                        as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
            }
            if 8 as ::core::ffi::c_int > 8 as ::core::ffi::c_int
                || codec_id != AV_CODEC_ID_RV40 as ::core::ffi::c_int
                    && codec_id != AV_CODEC_ID_VP7 as ::core::ffi::c_int
                    && codec_id != AV_CODEC_ID_VP8 as ::core::ffi::c_int
            {
                if chroma_format_idc <= 1 as ::core::ffi::c_int {
                    (*h).pred8x8[0 as ::core::ffi::c_int as usize] = Some(
                        pred8x8_dc_8_c
                            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                    ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
                    (*h).pred8x8[4 as ::core::ffi::c_int as usize] = Some(
                        pred8x8_left_dc_8_c
                            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                    ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
                    (*h).pred8x8[5 as ::core::ffi::c_int as usize] = Some(
                        pred8x8_top_dc_8_c
                            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                    ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
                    (*h).pred8x8[7 as ::core::ffi::c_int as usize] = Some(
                        pred8x8_mad_cow_dc_l0t_8
                            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                    ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
                    (*h).pred8x8[8 as ::core::ffi::c_int as usize] = Some(
                        pred8x8_mad_cow_dc_0lt_8
                            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                    ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
                    (*h).pred8x8[9 as ::core::ffi::c_int as usize] = Some(
                        pred8x8_mad_cow_dc_l00_8
                            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                    ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
                    (*h).pred8x8[10 as ::core::ffi::c_int as usize] = Some(
                        pred8x8_mad_cow_dc_0l0_8
                            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                    ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
                } else {
                    (*h).pred8x8[0 as ::core::ffi::c_int as usize] = Some(
                        pred8x16_dc_8_c
                            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                    ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
                    (*h).pred8x8[4 as ::core::ffi::c_int as usize] = Some(
                        pred8x16_left_dc_8_c
                            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                    ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
                    (*h).pred8x8[5 as ::core::ffi::c_int as usize] = Some(
                        pred8x16_top_dc_8_c
                            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                    ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
                    (*h).pred8x8[7 as ::core::ffi::c_int as usize] = Some(
                        pred8x16_mad_cow_dc_l0t_8
                            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                    ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
                    (*h).pred8x8[8 as ::core::ffi::c_int as usize] = Some(
                        pred8x16_mad_cow_dc_0lt_8
                            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                    ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
                    (*h).pred8x8[9 as ::core::ffi::c_int as usize] = Some(
                        pred8x16_mad_cow_dc_l00_8
                            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                    ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
                    (*h).pred8x8[10 as ::core::ffi::c_int as usize] = Some(
                        pred8x16_mad_cow_dc_0l0_8
                            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                    ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
                }
            } else {
                (*h).pred8x8[0 as ::core::ffi::c_int as usize] = Some(
                    pred8x8_dc_rv40_c
                        as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
                (*h).pred8x8[4 as ::core::ffi::c_int as usize] = Some(
                    pred8x8_left_dc_rv40_c
                        as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
                (*h).pred8x8[5 as ::core::ffi::c_int as usize] = Some(
                    pred8x8_top_dc_rv40_c
                        as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
            }
            if chroma_format_idc <= 1 as ::core::ffi::c_int {
                (*h).pred8x8[6 as ::core::ffi::c_int as usize] = Some(
                    pred8x8_128_dc_8_c
                        as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
            } else {
                (*h).pred8x8[6 as ::core::ffi::c_int as usize] = Some(
                    pred8x16_128_dc_8_c
                        as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
            }
            (*h).pred16x16[0 as ::core::ffi::c_int as usize] = Some(
                pred16x16_dc_8_c as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
            ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
            (*h).pred16x16[2 as ::core::ffi::c_int as usize] = Some(
                pred16x16_vertical_8_c
                    as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
            ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
            (*h).pred16x16[1 as ::core::ffi::c_int as usize] = Some(
                pred16x16_horizontal_8_c
                    as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
            ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
            (*h).pred16x16[3 as ::core::ffi::c_int as usize] = Some(
                pred16x16_plane_8_c
                    as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
            ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
            (*h).pred16x16[4 as ::core::ffi::c_int as usize] = Some(
                pred16x16_left_dc_8_c
                    as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
            ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
            (*h).pred16x16[5 as ::core::ffi::c_int as usize] = Some(
                pred16x16_top_dc_8_c
                    as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
            ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
            (*h).pred16x16[6 as ::core::ffi::c_int as usize] = Some(
                pred16x16_128_dc_8_c
                    as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
            ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
            (*h).pred4x4_add[0 as ::core::ffi::c_int as usize] = Some(
                pred4x4_vertical_add_8_c
                    as unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ptrdiff_t) -> (),
            )
                as Option<
                    unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ptrdiff_t) -> (),
                >;
            (*h).pred4x4_add[1 as ::core::ffi::c_int as usize] = Some(
                pred4x4_horizontal_add_8_c
                    as unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ptrdiff_t) -> (),
            )
                as Option<
                    unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ptrdiff_t) -> (),
                >;
            (*h).pred8x8l_add[0 as ::core::ffi::c_int as usize] = Some(
                pred8x8l_vertical_add_8_c
                    as unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ptrdiff_t) -> (),
            )
                as Option<
                    unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ptrdiff_t) -> (),
                >;
            (*h).pred8x8l_add[1 as ::core::ffi::c_int as usize] = Some(
                pred8x8l_horizontal_add_8_c
                    as unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ptrdiff_t) -> (),
            )
                as Option<
                    unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ptrdiff_t) -> (),
                >;
            (*h).pred8x8l_filter_add[0 as ::core::ffi::c_int as usize] = Some(
                pred8x8l_vertical_filter_add_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *mut int16_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ptrdiff_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut uint8_t,
                        *mut int16_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ptrdiff_t,
                    ) -> (),
                >;
            (*h).pred8x8l_filter_add[1 as ::core::ffi::c_int as usize] = Some(
                pred8x8l_horizontal_filter_add_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *mut int16_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ptrdiff_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut uint8_t,
                        *mut int16_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ptrdiff_t,
                    ) -> (),
                >;
            if chroma_format_idc <= 1 as ::core::ffi::c_int {
                (*h).pred8x8_add[2 as ::core::ffi::c_int as usize] = Some(
                    pred8x8_vertical_add_8_c
                        as unsafe extern "C" fn(
                            *mut uint8_t,
                            *const ::core::ffi::c_int,
                            *mut int16_t,
                            ptrdiff_t,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut uint8_t,
                            *const ::core::ffi::c_int,
                            *mut int16_t,
                            ptrdiff_t,
                        ) -> (),
                    >;
                (*h).pred8x8_add[1 as ::core::ffi::c_int as usize] = Some(
                    pred8x8_horizontal_add_8_c
                        as unsafe extern "C" fn(
                            *mut uint8_t,
                            *const ::core::ffi::c_int,
                            *mut int16_t,
                            ptrdiff_t,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut uint8_t,
                            *const ::core::ffi::c_int,
                            *mut int16_t,
                            ptrdiff_t,
                        ) -> (),
                    >;
            } else {
                (*h).pred8x8_add[2 as ::core::ffi::c_int as usize] = Some(
                    pred8x16_vertical_add_8_c
                        as unsafe extern "C" fn(
                            *mut uint8_t,
                            *const ::core::ffi::c_int,
                            *mut int16_t,
                            ptrdiff_t,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut uint8_t,
                            *const ::core::ffi::c_int,
                            *mut int16_t,
                            ptrdiff_t,
                        ) -> (),
                    >;
                (*h).pred8x8_add[1 as ::core::ffi::c_int as usize] = Some(
                    pred8x16_horizontal_add_8_c
                        as unsafe extern "C" fn(
                            *mut uint8_t,
                            *const ::core::ffi::c_int,
                            *mut int16_t,
                            ptrdiff_t,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut uint8_t,
                            *const ::core::ffi::c_int,
                            *mut int16_t,
                            ptrdiff_t,
                        ) -> (),
                    >;
            }
            (*h).pred16x16_add[2 as ::core::ffi::c_int as usize] = Some(
                pred16x16_vertical_add_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const ::core::ffi::c_int,
                        *mut int16_t,
                        ptrdiff_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut uint8_t,
                        *const ::core::ffi::c_int,
                        *mut int16_t,
                        ptrdiff_t,
                    ) -> (),
                >;
            (*h).pred16x16_add[1 as ::core::ffi::c_int as usize] = Some(
                pred16x16_horizontal_add_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const ::core::ffi::c_int,
                        *mut int16_t,
                        ptrdiff_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut uint8_t,
                        *const ::core::ffi::c_int,
                        *mut int16_t,
                        ptrdiff_t,
                    ) -> (),
                >;
            match codec_id {
                23 => {
                    (*h).pred4x4[3 as ::core::ffi::c_int as usize] = Some(
                        pred4x4_down_left_svq3_c
                            as unsafe extern "C" fn(
                                *mut uint8_t,
                                *const uint8_t,
                                ptrdiff_t,
                            ) -> (),
                    )
                        as Option<
                            unsafe extern "C" fn(
                                *mut uint8_t,
                                *const uint8_t,
                                ptrdiff_t,
                            ) -> (),
                        >;
                    (*h).pred16x16[3 as ::core::ffi::c_int as usize] = Some(
                        pred16x16_plane_svq3_c
                            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                    ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
                }
                69 => {
                    (*h).pred4x4[3 as ::core::ffi::c_int as usize] = Some(
                        pred4x4_down_left_rv40_c
                            as unsafe extern "C" fn(
                                *mut uint8_t,
                                *const uint8_t,
                                ptrdiff_t,
                            ) -> (),
                    )
                        as Option<
                            unsafe extern "C" fn(
                                *mut uint8_t,
                                *const uint8_t,
                                ptrdiff_t,
                            ) -> (),
                        >;
                    (*h).pred4x4[7 as ::core::ffi::c_int as usize] = Some(
                        pred4x4_vertical_left_rv40_c
                            as unsafe extern "C" fn(
                                *mut uint8_t,
                                *const uint8_t,
                                ptrdiff_t,
                            ) -> (),
                    )
                        as Option<
                            unsafe extern "C" fn(
                                *mut uint8_t,
                                *const uint8_t,
                                ptrdiff_t,
                            ) -> (),
                        >;
                    (*h).pred4x4[8 as ::core::ffi::c_int as usize] = Some(
                        pred4x4_horizontal_up_rv40_c
                            as unsafe extern "C" fn(
                                *mut uint8_t,
                                *const uint8_t,
                                ptrdiff_t,
                            ) -> (),
                    )
                        as Option<
                            unsafe extern "C" fn(
                                *mut uint8_t,
                                *const uint8_t,
                                ptrdiff_t,
                            ) -> (),
                        >;
                    (*h).pred4x4[12 as ::core::ffi::c_int as usize] = Some(
                        pred4x4_down_left_rv40_nodown_c
                            as unsafe extern "C" fn(
                                *mut uint8_t,
                                *const uint8_t,
                                ptrdiff_t,
                            ) -> (),
                    )
                        as Option<
                            unsafe extern "C" fn(
                                *mut uint8_t,
                                *const uint8_t,
                                ptrdiff_t,
                            ) -> (),
                        >;
                    (*h).pred4x4[13 as ::core::ffi::c_int as usize] = Some(
                        pred4x4_horizontal_up_rv40_nodown_c
                            as unsafe extern "C" fn(
                                *mut uint8_t,
                                *const uint8_t,
                                ptrdiff_t,
                            ) -> (),
                    )
                        as Option<
                            unsafe extern "C" fn(
                                *mut uint8_t,
                                *const uint8_t,
                                ptrdiff_t,
                            ) -> (),
                        >;
                    (*h).pred4x4[14 as ::core::ffi::c_int as usize] = Some(
                        pred4x4_vertical_left_rv40_nodown_c
                            as unsafe extern "C" fn(
                                *mut uint8_t,
                                *const uint8_t,
                                ptrdiff_t,
                            ) -> (),
                    )
                        as Option<
                            unsafe extern "C" fn(
                                *mut uint8_t,
                                *const uint8_t,
                                ptrdiff_t,
                            ) -> (),
                        >;
                    (*h).pred16x16[3 as ::core::ffi::c_int as usize] = Some(
                        pred16x16_plane_rv40_c
                            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                    ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
                }
                178 | 139 => {
                    (*h).pred4x4[0 as ::core::ffi::c_int as usize] = Some(
                        pred4x4_vertical_vp8_c
                            as unsafe extern "C" fn(
                                *mut uint8_t,
                                *const uint8_t,
                                ptrdiff_t,
                            ) -> (),
                    )
                        as Option<
                            unsafe extern "C" fn(
                                *mut uint8_t,
                                *const uint8_t,
                                ptrdiff_t,
                            ) -> (),
                        >;
                    (*h).pred4x4[1 as ::core::ffi::c_int as usize] = Some(
                        pred4x4_horizontal_vp8_c
                            as unsafe extern "C" fn(
                                *mut uint8_t,
                                *const uint8_t,
                                ptrdiff_t,
                            ) -> (),
                    )
                        as Option<
                            unsafe extern "C" fn(
                                *mut uint8_t,
                                *const uint8_t,
                                ptrdiff_t,
                            ) -> (),
                        >;
                    (*h).pred4x4[7 as ::core::ffi::c_int as usize] = Some(
                        pred4x4_vertical_left_vp8_c
                            as unsafe extern "C" fn(
                                *mut uint8_t,
                                *const uint8_t,
                                ptrdiff_t,
                            ) -> (),
                    )
                        as Option<
                            unsafe extern "C" fn(
                                *mut uint8_t,
                                *const uint8_t,
                                ptrdiff_t,
                            ) -> (),
                        >;
                    (*h).pred4x4[9 as ::core::ffi::c_int as usize] = Some(
                        pred4x4_tm_vp8_c
                            as unsafe extern "C" fn(
                                *mut uint8_t,
                                *const uint8_t,
                                ptrdiff_t,
                            ) -> (),
                    )
                        as Option<
                            unsafe extern "C" fn(
                                *mut uint8_t,
                                *const uint8_t,
                                ptrdiff_t,
                            ) -> (),
                        >;
                    (*h).pred4x4[10 as ::core::ffi::c_int as usize] = Some(
                        pred4x4_vertical_8_c
                            as unsafe extern "C" fn(
                                *mut uint8_t,
                                *const uint8_t,
                                ptrdiff_t,
                            ) -> (),
                    )
                        as Option<
                            unsafe extern "C" fn(
                                *mut uint8_t,
                                *const uint8_t,
                                ptrdiff_t,
                            ) -> (),
                        >;
                    (*h).pred4x4[12 as ::core::ffi::c_int as usize] = Some(
                        pred4x4_127_dc_c
                            as unsafe extern "C" fn(
                                *mut uint8_t,
                                *const uint8_t,
                                ptrdiff_t,
                            ) -> (),
                    )
                        as Option<
                            unsafe extern "C" fn(
                                *mut uint8_t,
                                *const uint8_t,
                                ptrdiff_t,
                            ) -> (),
                        >;
                    (*h).pred4x4[13 as ::core::ffi::c_int as usize] = Some(
                        pred4x4_129_dc_c
                            as unsafe extern "C" fn(
                                *mut uint8_t,
                                *const uint8_t,
                                ptrdiff_t,
                            ) -> (),
                    )
                        as Option<
                            unsafe extern "C" fn(
                                *mut uint8_t,
                                *const uint8_t,
                                ptrdiff_t,
                            ) -> (),
                        >;
                    (*h).pred4x4[14 as ::core::ffi::c_int as usize] = Some(
                        pred4x4_horizontal_8_c
                            as unsafe extern "C" fn(
                                *mut uint8_t,
                                *const uint8_t,
                                ptrdiff_t,
                            ) -> (),
                    )
                        as Option<
                            unsafe extern "C" fn(
                                *mut uint8_t,
                                *const uint8_t,
                                ptrdiff_t,
                            ) -> (),
                        >;
                    (*h).pred8x8[3 as ::core::ffi::c_int as usize] = Some(
                        pred8x8_tm_vp8_c
                            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                    ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
                    (*h).pred8x8[7 as ::core::ffi::c_int as usize] = Some(
                        pred8x8_127_dc_8_c
                            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                    ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
                    (*h).pred8x8[8 as ::core::ffi::c_int as usize] = Some(
                        pred8x8_129_dc_8_c
                            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                    ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
                    (*h).pred16x16[3 as ::core::ffi::c_int as usize] = Some(
                        pred16x16_tm_vp8_c
                            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                    ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
                    (*h).pred16x16[7 as ::core::ffi::c_int as usize] = Some(
                        pred16x16_127_dc_8_c
                            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                    ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
                    (*h).pred16x16[8 as ::core::ffi::c_int as usize] = Some(
                        pred16x16_129_dc_8_c
                            as unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> (),
                    ) as Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>;
                }
                _ => {}
            }
        }
    ff_h264_pred_init_aarch64(h, codec_id, bit_depth, chroma_format_idc);
}




unsafe extern "C" fn pred16x16_plane_8_c(mut src: *mut uint8_t, mut stride: ptrdiff_t) {
    pred16x16_plane_compat_8_c(
        src,
        stride,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
}

unsafe extern "C" fn pred8x8_vertical_8_c(
    mut _src: *mut uint8_t,
    mut _stride: ptrdiff_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut src: *mut uint8_t = _src;
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    let a: uint32_t = (*((src.offset(-(stride as isize)) as *mut uint32_t)
        .offset(0 as ::core::ffi::c_int as isize) as *const av_alias32))
        .u32_0;
    let b: uint32_t = (*((src.offset(-(stride as isize)) as *mut uint32_t)
        .offset(1 as ::core::ffi::c_int as isize) as *const av_alias32))
        .u32_0;
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        (*((src.offset((i * stride) as isize) as *mut uint32_t)
            .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = a;
        (*((src.offset((i * stride) as isize) as *mut uint32_t)
            .offset(1 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = b;
        i += 1;
    }
}




unsafe extern "C" fn pred8x16_vertical_8_c(
    mut _src: *mut uint8_t,
    mut _stride: ptrdiff_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut src: *mut uint8_t = _src;
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    let a: uint32_t = (*((src.offset(-(stride as isize)) as *mut uint32_t)
        .offset(0 as ::core::ffi::c_int as isize) as *const av_alias32))
        .u32_0;
    let b: uint32_t = (*((src.offset(-(stride as isize)) as *mut uint32_t)
        .offset(1 as ::core::ffi::c_int as isize) as *const av_alias32))
        .u32_0;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        (*((src.offset((i * stride) as isize) as *mut uint32_t)
            .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = a;
        (*((src.offset((i * stride) as isize) as *mut uint32_t)
            .offset(1 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = b;
        i += 1;
    }
}






unsafe extern "C" fn pred8x8_horizontal_8_c(
    mut _src: *mut uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut src: *mut uint8_t = _src;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        let a: uint32_t = (*src
            .offset(
                (-(1 as ::core::ffi::c_int) as ptrdiff_t + i as ptrdiff_t * stride)
                    as isize,
            ) as uint32_t)
            .wrapping_mul(0x1010101 as uint32_t);
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = a;
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(1 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = a;
        i += 1;
    }
}

unsafe extern "C" fn pred8x16_horizontal_8_c(
    mut _src: *mut uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut src: *mut uint8_t = _src;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        let a: uint32_t = (*src
            .offset(
                (-(1 as ::core::ffi::c_int) as ptrdiff_t + i as ptrdiff_t * stride)
                    as isize,
            ) as uint32_t)
            .wrapping_mul(0x1010101 as uint32_t);
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = a;
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(1 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = a;
        i += 1;
    }
}






unsafe extern "C" fn pred8x8_128_dc_8_c(mut _src: *mut uint8_t, mut stride: ptrdiff_t) {
    let mut i: ::core::ffi::c_int = 0;
    let a: uint32_t = ((((1 as ::core::ffi::c_int)
        << 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int)
        as uint32_t)
        .wrapping_mul(0x1010101 as uint32_t);
    let mut src: *mut uint8_t = _src;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = a;
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(1 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = a;
        i += 1;
    }
}


unsafe extern "C" fn pred8x8_127_dc_8_c(mut _src: *mut uint8_t, mut stride: ptrdiff_t) {
    let mut i: ::core::ffi::c_int = 0;
    let a: uint32_t = ((((1 as ::core::ffi::c_int)
        << 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
        as uint32_t)
        .wrapping_mul(0x1010101 as uint32_t);
    let mut src: *mut uint8_t = _src;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = a;
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(1 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = a;
        i += 1;
    }
}
unsafe extern "C" fn pred8x8_129_dc_8_c(mut _src: *mut uint8_t, mut stride: ptrdiff_t) {
    let mut i: ::core::ffi::c_int = 0;
    let a: uint32_t = ((((1 as ::core::ffi::c_int)
        << 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int)
        as uint32_t)
        .wrapping_mul(0x1010101 as uint32_t);
    let mut src: *mut uint8_t = _src;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = a;
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(1 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = a;
        i += 1;
    }
}


unsafe extern "C" fn pred8x16_128_dc_8_c(mut _src: *mut uint8_t, mut stride: ptrdiff_t) {
    pred8x8_128_dc_8_c(_src, stride);
    pred8x8_128_dc_8_c(_src.offset((8 as ptrdiff_t * stride) as isize), stride);
}



unsafe extern "C" fn pred8x8_left_dc_8_c(mut _src: *mut uint8_t, mut stride: ptrdiff_t) {
    let mut i: ::core::ffi::c_int = 0;
    let mut dc0: ::core::ffi::c_int = 0;
    let mut dc2: ::core::ffi::c_int = 0;
    let mut dc0splat: uint32_t = 0;
    let mut dc2splat: uint32_t = 0;
    let mut src: *mut uint8_t = _src;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    dc2 = 0 as ::core::ffi::c_int;
    dc0 = dc2;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        dc0
            += *src
                .offset(
                    (-(1 as ::core::ffi::c_int) as ptrdiff_t + i as ptrdiff_t * stride)
                        as isize,
                ) as ::core::ffi::c_int;
        dc2
            += *src
                .offset(
                    (-(1 as ::core::ffi::c_int) as ptrdiff_t
                        + (i + 4 as ::core::ffi::c_int) as ptrdiff_t * stride) as isize,
                ) as ::core::ffi::c_int;
        i += 1;
    }
    dc0splat = ((dc0 + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
        as ::core::ffi::c_uint)
        .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
    dc2splat = ((dc2 + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
        as ::core::ffi::c_uint)
        .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = dc0splat;
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(1 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = dc0splat;
        i += 1;
    }
    i = 4 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = dc2splat;
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(1 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = dc2splat;
        i += 1;
    }
}




unsafe extern "C" fn pred8x16_left_dc_8_c(
    mut _src: *mut uint8_t,
    mut stride: ptrdiff_t,
) {
    pred8x8_left_dc_8_c(_src, stride);
    pred8x8_left_dc_8_c(_src.offset((8 as ptrdiff_t * stride) as isize), stride);
}




unsafe extern "C" fn pred8x8_top_dc_8_c(mut _src: *mut uint8_t, mut stride: ptrdiff_t) {
    let mut i: ::core::ffi::c_int = 0;
    let mut dc0: ::core::ffi::c_int = 0;
    let mut dc1: ::core::ffi::c_int = 0;
    let mut dc0splat: uint32_t = 0;
    let mut dc1splat: uint32_t = 0;
    let mut src: *mut uint8_t = _src;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    dc1 = 0 as ::core::ffi::c_int;
    dc0 = dc1;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        dc0 += *src.offset((i as ptrdiff_t - stride) as isize) as ::core::ffi::c_int;
        dc1
            += *src
                .offset(((4 as ::core::ffi::c_int + i) as ptrdiff_t - stride) as isize)
                as ::core::ffi::c_int;
        i += 1;
    }
    dc0splat = ((dc0 + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
        as ::core::ffi::c_uint)
        .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
    dc1splat = ((dc1 + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
        as ::core::ffi::c_uint)
        .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = dc0splat;
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(1 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = dc1splat;
        i += 1;
    }
    i = 4 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = dc0splat;
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(1 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = dc1splat;
        i += 1;
    }
}




unsafe extern "C" fn pred8x16_top_dc_8_c(mut _src: *mut uint8_t, mut stride: ptrdiff_t) {
    let mut i: ::core::ffi::c_int = 0;
    let mut dc0: ::core::ffi::c_int = 0;
    let mut dc1: ::core::ffi::c_int = 0;
    let mut dc0splat: uint32_t = 0;
    let mut dc1splat: uint32_t = 0;
    let mut src: *mut uint8_t = _src;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    dc1 = 0 as ::core::ffi::c_int;
    dc0 = dc1;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        dc0 += *src.offset((i as ptrdiff_t - stride) as isize) as ::core::ffi::c_int;
        dc1
            += *src
                .offset(((4 as ::core::ffi::c_int + i) as ptrdiff_t - stride) as isize)
                as ::core::ffi::c_int;
        i += 1;
    }
    dc0splat = ((dc0 + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
        as ::core::ffi::c_uint)
        .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
    dc1splat = ((dc1 + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
        as ::core::ffi::c_uint)
        .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = dc0splat;
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(1 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = dc1splat;
        i += 1;
    }
}





unsafe extern "C" fn pred8x8_dc_8_c(mut _src: *mut uint8_t, mut stride: ptrdiff_t) {
    let mut i: ::core::ffi::c_int = 0;
    let mut dc0: ::core::ffi::c_int = 0;
    let mut dc1: ::core::ffi::c_int = 0;
    let mut dc2: ::core::ffi::c_int = 0;
    let mut dc0splat: uint32_t = 0;
    let mut dc1splat: uint32_t = 0;
    let mut dc2splat: uint32_t = 0;
    let mut dc3splat: uint32_t = 0;
    let mut src: *mut uint8_t = _src;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    dc2 = 0 as ::core::ffi::c_int;
    dc1 = dc2;
    dc0 = dc1;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        dc0
            += *src
                .offset(
                    (-(1 as ::core::ffi::c_int) as ptrdiff_t + i as ptrdiff_t * stride)
                        as isize,
                ) as ::core::ffi::c_int
                + *src.offset((i as ptrdiff_t - stride) as isize) as ::core::ffi::c_int;
        dc1
            += *src
                .offset(((4 as ::core::ffi::c_int + i) as ptrdiff_t - stride) as isize)
                as ::core::ffi::c_int;
        dc2
            += *src
                .offset(
                    (-(1 as ::core::ffi::c_int) as ptrdiff_t
                        + (i + 4 as ::core::ffi::c_int) as ptrdiff_t * stride) as isize,
                ) as ::core::ffi::c_int;
        i += 1;
    }
    dc0splat = ((dc0 + 4 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int)
        as ::core::ffi::c_uint)
        .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
    dc1splat = ((dc1 + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
        as ::core::ffi::c_uint)
        .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
    dc2splat = ((dc2 + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
        as ::core::ffi::c_uint)
        .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
    dc3splat = ((dc1 + dc2 + 4 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int)
        as ::core::ffi::c_uint)
        .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = dc0splat;
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(1 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = dc1splat;
        i += 1;
    }
    i = 4 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = dc2splat;
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(1 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = dc3splat;
        i += 1;
    }
}





unsafe extern "C" fn pred8x16_dc_8_c(mut _src: *mut uint8_t, mut stride: ptrdiff_t) {
    let mut i: ::core::ffi::c_int = 0;
    let mut dc0: ::core::ffi::c_int = 0;
    let mut dc1: ::core::ffi::c_int = 0;
    let mut dc2: ::core::ffi::c_int = 0;
    let mut dc3: ::core::ffi::c_int = 0;
    let mut dc4: ::core::ffi::c_int = 0;
    let mut dc0splat: uint32_t = 0;
    let mut dc1splat: uint32_t = 0;
    let mut dc2splat: uint32_t = 0;
    let mut dc3splat: uint32_t = 0;
    let mut dc4splat: uint32_t = 0;
    let mut dc5splat: uint32_t = 0;
    let mut dc6splat: uint32_t = 0;
    let mut dc7splat: uint32_t = 0;
    let mut src: *mut uint8_t = _src;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    dc4 = 0 as ::core::ffi::c_int;
    dc3 = dc4;
    dc2 = dc3;
    dc1 = dc2;
    dc0 = dc1;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        dc0
            += *src
                .offset(
                    (-(1 as ::core::ffi::c_int) as ptrdiff_t + i as ptrdiff_t * stride)
                        as isize,
                ) as ::core::ffi::c_int
                + *src.offset((i as ptrdiff_t - stride) as isize) as ::core::ffi::c_int;
        dc1
            += *src
                .offset(((4 as ::core::ffi::c_int + i) as ptrdiff_t - stride) as isize)
                as ::core::ffi::c_int;
        dc2
            += *src
                .offset(
                    (-(1 as ::core::ffi::c_int) as ptrdiff_t
                        + (i + 4 as ::core::ffi::c_int) as ptrdiff_t * stride) as isize,
                ) as ::core::ffi::c_int;
        dc3
            += *src
                .offset(
                    (-(1 as ::core::ffi::c_int) as ptrdiff_t
                        + (i + 8 as ::core::ffi::c_int) as ptrdiff_t * stride) as isize,
                ) as ::core::ffi::c_int;
        dc4
            += *src
                .offset(
                    (-(1 as ::core::ffi::c_int) as ptrdiff_t
                        + (i + 12 as ::core::ffi::c_int) as ptrdiff_t * stride) as isize,
                ) as ::core::ffi::c_int;
        i += 1;
    }
    dc0splat = ((dc0 + 4 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int)
        as ::core::ffi::c_uint)
        .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
    dc1splat = ((dc1 + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
        as ::core::ffi::c_uint)
        .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
    dc2splat = ((dc2 + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
        as ::core::ffi::c_uint)
        .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
    dc3splat = ((dc1 + dc2 + 4 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int)
        as ::core::ffi::c_uint)
        .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
    dc4splat = ((dc3 + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
        as ::core::ffi::c_uint)
        .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
    dc5splat = ((dc1 + dc3 + 4 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int)
        as ::core::ffi::c_uint)
        .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
    dc6splat = ((dc4 + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
        as ::core::ffi::c_uint)
        .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
    dc7splat = ((dc1 + dc4 + 4 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int)
        as ::core::ffi::c_uint)
        .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = dc0splat;
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(1 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = dc1splat;
        i += 1;
    }
    i = 4 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = dc2splat;
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(1 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = dc3splat;
        i += 1;
    }
    i = 8 as ::core::ffi::c_int;
    while i < 12 as ::core::ffi::c_int {
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = dc4splat;
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(1 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = dc5splat;
        i += 1;
    }
    i = 12 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = dc6splat;
        (*((src.offset((i as ptrdiff_t * stride) as isize) as *mut uint32_t)
            .offset(1 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = dc7splat;
        i += 1;
    }
}


unsafe extern "C" fn pred8x8_mad_cow_dc_l0t_8(
    mut src: *mut uint8_t,
    mut stride: ptrdiff_t,
) {
    pred8x8_top_dc_8_c(src, stride);
    pred4x4_dc_8_c(src, ::core::ptr::null::<uint8_t>(), stride);
}



unsafe extern "C" fn pred8x16_mad_cow_dc_l0t_8(
    mut src: *mut uint8_t,
    mut stride: ptrdiff_t,
) {
    pred8x16_top_dc_8_c(src, stride);
    pred4x4_dc_8_c(src, ::core::ptr::null::<uint8_t>(), stride);
}






unsafe extern "C" fn pred8x8_mad_cow_dc_0lt_8(
    mut src: *mut uint8_t,
    mut stride: ptrdiff_t,
) {
    pred8x8_dc_8_c(src, stride);
    pred4x4_top_dc_8_c(src, ::core::ptr::null::<uint8_t>(), stride);
}



unsafe extern "C" fn pred8x16_mad_cow_dc_0lt_8(
    mut src: *mut uint8_t,
    mut stride: ptrdiff_t,
) {
    pred8x16_dc_8_c(src, stride);
    pred4x4_top_dc_8_c(src, ::core::ptr::null::<uint8_t>(), stride);
}



unsafe extern "C" fn pred8x8_mad_cow_dc_l00_8(
    mut src: *mut uint8_t,
    mut stride: ptrdiff_t,
) {
    pred8x8_left_dc_8_c(src, stride);
    pred4x4_128_dc_8_c(
        src.offset((4 as ptrdiff_t * stride) as isize),
        ::core::ptr::null::<uint8_t>(),
        stride,
    );
    pred4x4_128_dc_8_c(
        src
            .offset((4 as ptrdiff_t * stride) as isize)
            .offset(
                (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        ::core::ptr::null::<uint8_t>(),
        stride,
    );
}




unsafe extern "C" fn pred8x16_mad_cow_dc_l00_8(
    mut src: *mut uint8_t,
    mut stride: ptrdiff_t,
) {
    pred8x16_left_dc_8_c(src, stride);
    pred4x4_128_dc_8_c(
        src.offset((4 as ptrdiff_t * stride) as isize),
        ::core::ptr::null::<uint8_t>(),
        stride,
    );
    pred4x4_128_dc_8_c(
        src
            .offset((4 as ptrdiff_t * stride) as isize)
            .offset(
                (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        ::core::ptr::null::<uint8_t>(),
        stride,
    );
}








unsafe extern "C" fn pred8x8_mad_cow_dc_0l0_8(
    mut src: *mut uint8_t,
    mut stride: ptrdiff_t,
) {
    pred8x8_left_dc_8_c(src, stride);
    pred4x4_128_dc_8_c(src, ::core::ptr::null::<uint8_t>(), stride);
    pred4x4_128_dc_8_c(
        src
            .offset(
                (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        ::core::ptr::null::<uint8_t>(),
        stride,
    );
}



unsafe extern "C" fn pred8x16_mad_cow_dc_0l0_8(
    mut src: *mut uint8_t,
    mut stride: ptrdiff_t,
) {
    pred8x16_left_dc_8_c(src, stride);
    pred4x4_128_dc_8_c(src, ::core::ptr::null::<uint8_t>(), stride);
    pred4x4_128_dc_8_c(
        src
            .offset(
                (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        ::core::ptr::null::<uint8_t>(),
        stride,
    );
}

unsafe extern "C" fn pred8x8_plane_8_c(mut _src: *mut uint8_t, mut _stride: ptrdiff_t) {
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut a: ::core::ffi::c_int = 0;
    let mut src: *mut uint8_t = _src;
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    let src0: *const uint8_t = src
        .offset(3 as ::core::ffi::c_int as isize)
        .offset(-(stride as isize));
    let mut src1: *const uint8_t = src
        .offset((4 as ::core::ffi::c_int * stride) as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    let mut src2: *const uint8_t = src1
        .offset(-((2 as ::core::ffi::c_int * stride) as isize));
    let mut H: ::core::ffi::c_int = *src0.offset(1 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        - *src0.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int;
    let mut V: ::core::ffi::c_int = *src1.offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        - *src2.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
    k = 2 as ::core::ffi::c_int;
    while k <= 4 as ::core::ffi::c_int {
        src1 = src1.offset(stride as isize);
        src2 = src2.offset(-(stride as isize));
        H
            += k
                * (*src0.offset(k as isize) as ::core::ffi::c_int
                    - *src0.offset(-k as isize) as ::core::ffi::c_int);
        V
            += k
                * (*src1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    - *src2.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int);
        k += 1;
    }
    H = 17 as ::core::ffi::c_int * H + 16 as ::core::ffi::c_int
        >> 5 as ::core::ffi::c_int;
    V = 17 as ::core::ffi::c_int * V + 16 as ::core::ffi::c_int
        >> 5 as ::core::ffi::c_int;
    a = 16 as ::core::ffi::c_int
        * (*src1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *src2.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) - 3 as ::core::ffi::c_int * (V + H);
    j = 8 as ::core::ffi::c_int;
    while j > 0 as ::core::ffi::c_int {
        let mut b: ::core::ffi::c_int = a;
        a += V;
        *src.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            b >> 5 as ::core::ffi::c_int,
        );
        *src.offset(1 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            b + H >> 5 as ::core::ffi::c_int,
        );
        *src.offset(2 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            b + 2 as ::core::ffi::c_int * H >> 5 as ::core::ffi::c_int,
        );
        *src.offset(3 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            b + 3 as ::core::ffi::c_int * H >> 5 as ::core::ffi::c_int,
        );
        *src.offset(4 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            b + 4 as ::core::ffi::c_int * H >> 5 as ::core::ffi::c_int,
        );
        *src.offset(5 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            b + 5 as ::core::ffi::c_int * H >> 5 as ::core::ffi::c_int,
        );
        *src.offset(6 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            b + 6 as ::core::ffi::c_int * H >> 5 as ::core::ffi::c_int,
        );
        *src.offset(7 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            b + 7 as ::core::ffi::c_int * H >> 5 as ::core::ffi::c_int,
        );
        src = src.offset(stride as isize);
        j -= 1;
    }
}





unsafe extern "C" fn pred8x16_plane_8_c(mut _src: *mut uint8_t, mut _stride: ptrdiff_t) {
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut a: ::core::ffi::c_int = 0;
    let mut src: *mut uint8_t = _src;
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    let src0: *const uint8_t = src
        .offset(3 as ::core::ffi::c_int as isize)
        .offset(-(stride as isize));
    let mut src1: *const uint8_t = src
        .offset((8 as ::core::ffi::c_int * stride) as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    let mut src2: *const uint8_t = src1
        .offset(-((2 as ::core::ffi::c_int * stride) as isize));
    let mut H: ::core::ffi::c_int = *src0.offset(1 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        - *src0.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int;
    let mut V: ::core::ffi::c_int = *src1.offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        - *src2.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
    k = 2 as ::core::ffi::c_int;
    while k <= 4 as ::core::ffi::c_int {
        src1 = src1.offset(stride as isize);
        src2 = src2.offset(-(stride as isize));
        H
            += k
                * (*src0.offset(k as isize) as ::core::ffi::c_int
                    - *src0.offset(-k as isize) as ::core::ffi::c_int);
        V
            += k
                * (*src1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    - *src2.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int);
        k += 1;
    }
    while k <= 8 as ::core::ffi::c_int {
        src1 = src1.offset(stride as isize);
        src2 = src2.offset(-(stride as isize));
        V
            += k
                * (*src1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    - *src2.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int);
        k += 1;
    }
    H = 17 as ::core::ffi::c_int * H + 16 as ::core::ffi::c_int
        >> 5 as ::core::ffi::c_int;
    V = 5 as ::core::ffi::c_int * V + 32 as ::core::ffi::c_int
        >> 6 as ::core::ffi::c_int;
    a = 16 as ::core::ffi::c_int
        * (*src1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *src2.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) - 7 as ::core::ffi::c_int * V
        - 3 as ::core::ffi::c_int * H;
    j = 16 as ::core::ffi::c_int;
    while j > 0 as ::core::ffi::c_int {
        let mut b: ::core::ffi::c_int = a;
        a += V;
        *src.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            b >> 5 as ::core::ffi::c_int,
        );
        *src.offset(1 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            b + H >> 5 as ::core::ffi::c_int,
        );
        *src.offset(2 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            b + 2 as ::core::ffi::c_int * H >> 5 as ::core::ffi::c_int,
        );
        *src.offset(3 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            b + 3 as ::core::ffi::c_int * H >> 5 as ::core::ffi::c_int,
        );
        *src.offset(4 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            b + 4 as ::core::ffi::c_int * H >> 5 as ::core::ffi::c_int,
        );
        *src.offset(5 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            b + 5 as ::core::ffi::c_int * H >> 5 as ::core::ffi::c_int,
        );
        *src.offset(6 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            b + 6 as ::core::ffi::c_int * H >> 5 as ::core::ffi::c_int,
        );
        *src.offset(7 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            b + 7 as ::core::ffi::c_int * H >> 5 as ::core::ffi::c_int,
        );
        src = src.offset(stride as isize);
        j -= 1;
    }
}





unsafe extern "C" fn pred8x8l_128_dc_8_c(
    mut _src: *mut uint8_t,
    mut has_topleft: ::core::ffi::c_int,
    mut has_topright: ::core::ffi::c_int,
    mut _stride: ptrdiff_t,
) {
    let mut src: *mut uint8_t = _src;
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y < 8 as ::core::ffi::c_int {
        (*((src as *mut uint32_t).offset(0 as ::core::ffi::c_int as isize)
            as *mut av_alias32))
            .u32_0 = (((1 as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint)
            .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
        (*((src as *mut uint32_t).offset(1 as ::core::ffi::c_int as isize)
            as *mut av_alias32))
            .u32_0 = (((1 as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint)
            .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
        src = src.offset(stride as isize);
        y += 1;
    }
}


unsafe extern "C" fn pred8x8l_left_dc_8_c(
    mut _src: *mut uint8_t,
    mut has_topleft: ::core::ffi::c_int,
    mut has_topright: ::core::ffi::c_int,
    mut _stride: ptrdiff_t,
) {
    let mut src: *mut uint8_t = _src;
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    let l0: ::core::ffi::c_int = (if has_topleft != 0 {
        *src
            .offset(
                (-(1 as ::core::ffi::c_int) + -(1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int
    } else {
        *src
            .offset(
                (-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * stride) as isize,
            ) as ::core::ffi::c_int
    })
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l1: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l2: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l3: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 3 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l4: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 4 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l5: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 5 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l6: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 6 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l7: ::core::ffi::c_int = *src
        .offset((-(1 as ::core::ffi::c_int) + 6 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_int
        + 3 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 7 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int;
    let dc: uint32_t = ((l0 + l1 + l2 + l3 + l4 + l5 + l6 + l7 + 4 as ::core::ffi::c_int
        >> 3 as ::core::ffi::c_int) as uint32_t)
        .wrapping_mul(0x1010101 as uint32_t);
    let mut y: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y < 8 as ::core::ffi::c_int {
        (*((src as *mut uint32_t).offset(0 as ::core::ffi::c_int as isize)
            as *mut av_alias32))
            .u32_0 = dc;
        (*((src as *mut uint32_t).offset(1 as ::core::ffi::c_int as isize)
            as *mut av_alias32))
            .u32_0 = dc;
        src = src.offset(stride as isize);
        y += 1;
    }
}





unsafe extern "C" fn pred8x8l_top_dc_8_c(
    mut _src: *mut uint8_t,
    mut has_topleft: ::core::ffi::c_int,
    mut has_topright: ::core::ffi::c_int,
    mut _stride: ptrdiff_t,
) {
    let mut src: *mut uint8_t = _src;
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    let t0: ::core::ffi::c_int = (if has_topleft != 0 {
        *src
            .offset(
                (-(1 as ::core::ffi::c_int) + -(1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int
    } else {
        *src
            .offset(
                (0 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
    })
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (0 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (1 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t1: ::core::ffi::c_int = *src
        .offset(
            (1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (1 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t2: ::core::ffi::c_int = *src
        .offset(
            (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (2 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t3: ::core::ffi::c_int = *src
        .offset(
            (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (3 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t4: ::core::ffi::c_int = *src
        .offset(
            (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (4 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t5: ::core::ffi::c_int = *src
        .offset(
            (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (5 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t6: ::core::ffi::c_int = *src
        .offset(
            (6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (6 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t7: ::core::ffi::c_int = (if has_topright != 0 {
        *src
            .offset(
                (8 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
    } else {
        *src
            .offset(
                (7 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
    })
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (7 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (6 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let dc: uint32_t = ((t0 + t1 + t2 + t3 + t4 + t5 + t6 + t7 + 4 as ::core::ffi::c_int
        >> 3 as ::core::ffi::c_int) as uint32_t)
        .wrapping_mul(0x1010101 as uint32_t);
    let mut y: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y < 8 as ::core::ffi::c_int {
        (*((src as *mut uint32_t).offset(0 as ::core::ffi::c_int as isize)
            as *mut av_alias32))
            .u32_0 = dc;
        (*((src as *mut uint32_t).offset(1 as ::core::ffi::c_int as isize)
            as *mut av_alias32))
            .u32_0 = dc;
        src = src.offset(stride as isize);
        y += 1;
    }
}







unsafe extern "C" fn pred8x8l_dc_8_c(
    mut _src: *mut uint8_t,
    mut has_topleft: ::core::ffi::c_int,
    mut has_topright: ::core::ffi::c_int,
    mut _stride: ptrdiff_t,
) {
    let mut src: *mut uint8_t = _src;
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    let l0: ::core::ffi::c_int = (if has_topleft != 0 {
        *src
            .offset(
                (-(1 as ::core::ffi::c_int) + -(1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int
    } else {
        *src
            .offset(
                (-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * stride) as isize,
            ) as ::core::ffi::c_int
    })
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l1: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l2: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l3: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 3 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l4: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 4 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l5: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 5 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l6: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 6 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l7: ::core::ffi::c_int = *src
        .offset((-(1 as ::core::ffi::c_int) + 6 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_int
        + 3 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 7 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int;
    let t0: ::core::ffi::c_int = (if has_topleft != 0 {
        *src
            .offset(
                (-(1 as ::core::ffi::c_int) + -(1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int
    } else {
        *src
            .offset(
                (0 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
    })
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (0 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (1 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t1: ::core::ffi::c_int = *src
        .offset(
            (1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (1 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t2: ::core::ffi::c_int = *src
        .offset(
            (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (2 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t3: ::core::ffi::c_int = *src
        .offset(
            (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (3 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t4: ::core::ffi::c_int = *src
        .offset(
            (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (4 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t5: ::core::ffi::c_int = *src
        .offset(
            (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (5 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t6: ::core::ffi::c_int = *src
        .offset(
            (6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (6 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t7: ::core::ffi::c_int = (if has_topright != 0 {
        *src
            .offset(
                (8 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
    } else {
        *src
            .offset(
                (7 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
    })
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (7 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (6 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let dc: uint32_t = ((l0 + l1 + l2 + l3 + l4 + l5 + l6 + l7 + t0 + t1 + t2 + t3 + t4
        + t5 + t6 + t7 + 8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as uint32_t)
        .wrapping_mul(0x1010101 as uint32_t);
    let mut y: ::core::ffi::c_int = 0;
    y = 0 as ::core::ffi::c_int;
    while y < 8 as ::core::ffi::c_int {
        (*((src as *mut uint32_t).offset(0 as ::core::ffi::c_int as isize)
            as *mut av_alias32))
            .u32_0 = dc;
        (*((src as *mut uint32_t).offset(1 as ::core::ffi::c_int as isize)
            as *mut av_alias32))
            .u32_0 = dc;
        src = src.offset(stride as isize);
        y += 1;
    }
}


unsafe extern "C" fn pred8x8l_horizontal_8_c(
    mut _src: *mut uint8_t,
    mut has_topleft: ::core::ffi::c_int,
    mut has_topright: ::core::ffi::c_int,
    mut _stride: ptrdiff_t,
) {
    let mut src: *mut uint8_t = _src;
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    let mut a: uint32_t = 0;
    let l0: ::core::ffi::c_int = (if has_topleft != 0 {
        *src
            .offset(
                (-(1 as ::core::ffi::c_int) + -(1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int
    } else {
        *src
            .offset(
                (-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * stride) as isize,
            ) as ::core::ffi::c_int
    })
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l1: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l2: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l3: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 3 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l4: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 4 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l5: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 5 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l6: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 6 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l7: ::core::ffi::c_int = *src
        .offset((-(1 as ::core::ffi::c_int) + 6 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_int
        + 3 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 7 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int;
    a = (l0 as ::core::ffi::c_uint).wrapping_mul(0x1010101 as ::core::ffi::c_uint)
        as uint32_t;
    (*(src.offset((0 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
    (*(src
        .offset((0 as ::core::ffi::c_int * stride) as isize)
        .offset(4 as ::core::ffi::c_int as isize) as *mut av_alias32))
        .u32_0 = a;
    a = (l1 as ::core::ffi::c_uint).wrapping_mul(0x1010101 as ::core::ffi::c_uint)
        as uint32_t;
    (*(src.offset((1 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
    (*(src
        .offset((1 as ::core::ffi::c_int * stride) as isize)
        .offset(4 as ::core::ffi::c_int as isize) as *mut av_alias32))
        .u32_0 = a;
    a = (l2 as ::core::ffi::c_uint).wrapping_mul(0x1010101 as ::core::ffi::c_uint)
        as uint32_t;
    (*(src.offset((2 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
    (*(src
        .offset((2 as ::core::ffi::c_int * stride) as isize)
        .offset(4 as ::core::ffi::c_int as isize) as *mut av_alias32))
        .u32_0 = a;
    a = (l3 as ::core::ffi::c_uint).wrapping_mul(0x1010101 as ::core::ffi::c_uint)
        as uint32_t;
    (*(src.offset((3 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
    (*(src
        .offset((3 as ::core::ffi::c_int * stride) as isize)
        .offset(4 as ::core::ffi::c_int as isize) as *mut av_alias32))
        .u32_0 = a;
    a = (l4 as ::core::ffi::c_uint).wrapping_mul(0x1010101 as ::core::ffi::c_uint)
        as uint32_t;
    (*(src.offset((4 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
    (*(src
        .offset((4 as ::core::ffi::c_int * stride) as isize)
        .offset(4 as ::core::ffi::c_int as isize) as *mut av_alias32))
        .u32_0 = a;
    a = (l5 as ::core::ffi::c_uint).wrapping_mul(0x1010101 as ::core::ffi::c_uint)
        as uint32_t;
    (*(src.offset((5 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
    (*(src
        .offset((5 as ::core::ffi::c_int * stride) as isize)
        .offset(4 as ::core::ffi::c_int as isize) as *mut av_alias32))
        .u32_0 = a;
    a = (l6 as ::core::ffi::c_uint).wrapping_mul(0x1010101 as ::core::ffi::c_uint)
        as uint32_t;
    (*(src.offset((6 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
    (*(src
        .offset((6 as ::core::ffi::c_int * stride) as isize)
        .offset(4 as ::core::ffi::c_int as isize) as *mut av_alias32))
        .u32_0 = a;
    a = (l7 as ::core::ffi::c_uint).wrapping_mul(0x1010101 as ::core::ffi::c_uint)
        as uint32_t;
    (*(src.offset((7 as ::core::ffi::c_int * stride) as isize) as *mut av_alias32))
        .u32_0 = a;
    (*(src
        .offset((7 as ::core::ffi::c_int * stride) as isize)
        .offset(4 as ::core::ffi::c_int as isize) as *mut av_alias32))
        .u32_0 = a;
}





unsafe extern "C" fn pred8x8l_vertical_8_c(
    mut _src: *mut uint8_t,
    mut has_topleft: ::core::ffi::c_int,
    mut has_topright: ::core::ffi::c_int,
    mut _stride: ptrdiff_t,
) {
    let mut y: ::core::ffi::c_int = 0;
    let mut src: *mut uint8_t = _src;
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    let mut a: uint32_t = 0;
    let mut b: uint32_t = 0;
    let t0: ::core::ffi::c_int = (if has_topleft != 0 {
        *src
            .offset(
                (-(1 as ::core::ffi::c_int) + -(1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int
    } else {
        *src
            .offset(
                (0 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
    })
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (0 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (1 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t1: ::core::ffi::c_int = *src
        .offset(
            (1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (1 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t2: ::core::ffi::c_int = *src
        .offset(
            (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (2 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t3: ::core::ffi::c_int = *src
        .offset(
            (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (3 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t4: ::core::ffi::c_int = *src
        .offset(
            (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (4 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t5: ::core::ffi::c_int = *src
        .offset(
            (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (5 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t6: ::core::ffi::c_int = *src
        .offset(
            (6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (6 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t7: ::core::ffi::c_int = (if has_topright != 0 {
        *src
            .offset(
                (8 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
    } else {
        *src
            .offset(
                (7 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
    })
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (7 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (6 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    *src.offset(0 as ::core::ffi::c_int as isize) = t0 as uint8_t;
    *src.offset(1 as ::core::ffi::c_int as isize) = t1 as uint8_t;
    *src.offset(2 as ::core::ffi::c_int as isize) = t2 as uint8_t;
    *src.offset(3 as ::core::ffi::c_int as isize) = t3 as uint8_t;
    *src.offset(4 as ::core::ffi::c_int as isize) = t4 as uint8_t;
    *src.offset(5 as ::core::ffi::c_int as isize) = t5 as uint8_t;
    *src.offset(6 as ::core::ffi::c_int as isize) = t6 as uint8_t;
    *src.offset(7 as ::core::ffi::c_int as isize) = t7 as uint8_t;
    a = (*((src as *mut uint32_t).offset(0 as ::core::ffi::c_int as isize)
        as *const av_alias32))
        .u32_0;
    b = (*((src as *mut uint32_t).offset(1 as ::core::ffi::c_int as isize)
        as *const av_alias32))
        .u32_0;
    y = 1 as ::core::ffi::c_int;
    while y < 8 as ::core::ffi::c_int {
        (*((src.offset((y * stride) as isize) as *mut uint32_t)
            .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = a;
        (*((src.offset((y * stride) as isize) as *mut uint32_t)
            .offset(1 as ::core::ffi::c_int as isize) as *mut av_alias32))
            .u32_0 = b;
        y += 1;
    }
}

unsafe extern "C" fn pred8x8l_down_left_8_c(
    mut _src: *mut uint8_t,
    mut has_topleft: ::core::ffi::c_int,
    mut has_topright: ::core::ffi::c_int,
    mut _stride: ptrdiff_t,
) {
    let mut src: *mut uint8_t = _src;
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    let t0: ::core::ffi::c_int = (if has_topleft != 0 {
        *src
            .offset(
                (-(1 as ::core::ffi::c_int) + -(1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int
    } else {
        *src
            .offset(
                (0 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
    })
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (0 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (1 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t1: ::core::ffi::c_int = *src
        .offset(
            (1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (1 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t2: ::core::ffi::c_int = *src
        .offset(
            (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (2 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t3: ::core::ffi::c_int = *src
        .offset(
            (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (3 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t4: ::core::ffi::c_int = *src
        .offset(
            (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (4 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t5: ::core::ffi::c_int = *src
        .offset(
            (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (5 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t6: ::core::ffi::c_int = *src
        .offset(
            (6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (6 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t7: ::core::ffi::c_int = (if has_topright != 0 {
        *src
            .offset(
                (8 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
    } else {
        *src
            .offset(
                (7 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
    })
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (7 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (6 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let mut t8: ::core::ffi::c_int = 0;
    let mut t9: ::core::ffi::c_int = 0;
    let mut t10: ::core::ffi::c_int = 0;
    let mut t11: ::core::ffi::c_int = 0;
    let mut t12: ::core::ffi::c_int = 0;
    let mut t13: ::core::ffi::c_int = 0;
    let mut t14: ::core::ffi::c_int = 0;
    let mut t15: ::core::ffi::c_int = 0;
    if has_topright != 0 {
        t8 = *src
            .offset(
                (8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *src
                    .offset(
                        (8 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                            as isize,
                    ) as ::core::ffi::c_int
            + *src
                .offset(
                    (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        + -(1 as ::core::ffi::c_int) * stride) as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int;
        t9 = *src
            .offset(
                (9 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *src
                    .offset(
                        (9 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                            as isize,
                    ) as ::core::ffi::c_int
            + *src
                .offset(
                    (9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        + -(1 as ::core::ffi::c_int) * stride) as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int;
        t10 = *src
            .offset(
                (10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *src
                    .offset(
                        (10 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                            as isize,
                    ) as ::core::ffi::c_int
            + *src
                .offset(
                    (10 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        + -(1 as ::core::ffi::c_int) * stride) as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int;
        t11 = *src
            .offset(
                (11 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *src
                    .offset(
                        (11 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                            as isize,
                    ) as ::core::ffi::c_int
            + *src
                .offset(
                    (11 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        + -(1 as ::core::ffi::c_int) * stride) as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int;
        t12 = *src
            .offset(
                (12 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *src
                    .offset(
                        (12 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                            as isize,
                    ) as ::core::ffi::c_int
            + *src
                .offset(
                    (12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        + -(1 as ::core::ffi::c_int) * stride) as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int;
        t13 = *src
            .offset(
                (13 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *src
                    .offset(
                        (13 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                            as isize,
                    ) as ::core::ffi::c_int
            + *src
                .offset(
                    (13 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        + -(1 as ::core::ffi::c_int) * stride) as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int;
        t14 = *src
            .offset(
                (14 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *src
                    .offset(
                        (14 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                            as isize,
                    ) as ::core::ffi::c_int
            + *src
                .offset(
                    (14 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        + -(1 as ::core::ffi::c_int) * stride) as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int;
        t15 = *src
            .offset(
                (14 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int
                * *src
                    .offset(
                        (15 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                            as isize,
                    ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int;
    } else {
        t15 = *src
            .offset(
                (7 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int;
        t14 = t15;
        t13 = t14;
        t12 = t13;
        t11 = t12;
        t10 = t11;
        t9 = t10;
        t8 = t9;
    }
    *src.offset((0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = (t0
        + 2 as ::core::ffi::c_int * t1 + t2 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh282 = *src
        .offset((1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize);
    *fresh282 = (t1 + 2 as ::core::ffi::c_int * t2 + t3 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize) = *fresh282;
    let ref mut fresh283 = *src
        .offset((2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize);
    *fresh283 = (t2 + 2 as ::core::ffi::c_int * t3 + t4 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh284 = *src
        .offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh284 = *fresh283;
    *src.offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize) = *fresh284;
    let ref mut fresh285 = *src
        .offset((3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize);
    *fresh285 = (t3 + 2 as ::core::ffi::c_int * t4 + t5 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh286 = *src
        .offset((2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh286 = *fresh285;
    let ref mut fresh287 = *src
        .offset((1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh287 = *fresh286;
    *src.offset((0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize) = *fresh287;
    let ref mut fresh288 = *src
        .offset((4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize);
    *fresh288 = (t4 + 2 as ::core::ffi::c_int * t5 + t6 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh289 = *src
        .offset((3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh289 = *fresh288;
    let ref mut fresh290 = *src
        .offset((2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh290 = *fresh289;
    let ref mut fresh291 = *src
        .offset((1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh291 = *fresh290;
    *src.offset((0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize) = *fresh291;
    let ref mut fresh292 = *src
        .offset((5 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize);
    *fresh292 = (t5 + 2 as ::core::ffi::c_int * t6 + t7 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh293 = *src
        .offset((4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh293 = *fresh292;
    let ref mut fresh294 = *src
        .offset((3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh294 = *fresh293;
    let ref mut fresh295 = *src
        .offset((2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh295 = *fresh294;
    let ref mut fresh296 = *src
        .offset((1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh296 = *fresh295;
    *src.offset((0 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize) = *fresh296;
    let ref mut fresh297 = *src
        .offset((6 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize);
    *fresh297 = (t6 + 2 as ::core::ffi::c_int * t7 + t8 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh298 = *src
        .offset((5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh298 = *fresh297;
    let ref mut fresh299 = *src
        .offset((4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh299 = *fresh298;
    let ref mut fresh300 = *src
        .offset((3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh300 = *fresh299;
    let ref mut fresh301 = *src
        .offset((2 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh301 = *fresh300;
    let ref mut fresh302 = *src
        .offset((1 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh302 = *fresh301;
    *src.offset((0 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize) = *fresh302;
    let ref mut fresh303 = *src
        .offset((7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize);
    *fresh303 = (t7 + 2 as ::core::ffi::c_int * t8 + t9 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh304 = *src
        .offset((6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh304 = *fresh303;
    let ref mut fresh305 = *src
        .offset((5 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh305 = *fresh304;
    let ref mut fresh306 = *src
        .offset((4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh306 = *fresh305;
    let ref mut fresh307 = *src
        .offset((3 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh307 = *fresh306;
    let ref mut fresh308 = *src
        .offset((2 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh308 = *fresh307;
    let ref mut fresh309 = *src
        .offset((1 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh309 = *fresh308;
    *src.offset((0 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize) = *fresh309;
    let ref mut fresh310 = *src
        .offset((7 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh310 = (t8 + 2 as ::core::ffi::c_int * t9 + t10 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh311 = *src
        .offset((6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh311 = *fresh310;
    let ref mut fresh312 = *src
        .offset((5 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh312 = *fresh311;
    let ref mut fresh313 = *src
        .offset((4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh313 = *fresh312;
    let ref mut fresh314 = *src
        .offset((3 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh314 = *fresh313;
    let ref mut fresh315 = *src
        .offset((2 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh315 = *fresh314;
    *src.offset((1 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize) = *fresh315;
    let ref mut fresh316 = *src
        .offset((7 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh316 = (t9 + 2 as ::core::ffi::c_int * t10 + t11 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh317 = *src
        .offset((6 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh317 = *fresh316;
    let ref mut fresh318 = *src
        .offset((5 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh318 = *fresh317;
    let ref mut fresh319 = *src
        .offset((4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh319 = *fresh318;
    let ref mut fresh320 = *src
        .offset((3 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh320 = *fresh319;
    *src.offset((2 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize) = *fresh320;
    let ref mut fresh321 = *src
        .offset((7 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh321 = (t10 + 2 as ::core::ffi::c_int * t11 + t12 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh322 = *src
        .offset((6 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh322 = *fresh321;
    let ref mut fresh323 = *src
        .offset((5 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh323 = *fresh322;
    let ref mut fresh324 = *src
        .offset((4 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh324 = *fresh323;
    *src.offset((3 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize) = *fresh324;
    let ref mut fresh325 = *src
        .offset((7 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh325 = (t11 + 2 as ::core::ffi::c_int * t12 + t13 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh326 = *src
        .offset((6 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh326 = *fresh325;
    let ref mut fresh327 = *src
        .offset((5 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh327 = *fresh326;
    *src.offset((4 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize) = *fresh327;
    let ref mut fresh328 = *src
        .offset((7 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh328 = (t12 + 2 as ::core::ffi::c_int * t13 + t14 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh329 = *src
        .offset((6 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh329 = *fresh328;
    *src.offset((5 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize) = *fresh329;
    let ref mut fresh330 = *src
        .offset((7 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh330 = (t13 + 2 as ::core::ffi::c_int * t14 + t15 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((6 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize) = *fresh330;
    *src.offset((7 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize) = (t14
        + 3 as ::core::ffi::c_int * t15 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
}





unsafe extern "C" fn pred8x8l_down_right_8_c(
    mut _src: *mut uint8_t,
    mut has_topleft: ::core::ffi::c_int,
    mut has_topright: ::core::ffi::c_int,
    mut _stride: ptrdiff_t,
) {
    let mut src: *mut uint8_t = _src;
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    let t0: ::core::ffi::c_int = (if has_topleft != 0 {
        *src
            .offset(
                (-(1 as ::core::ffi::c_int) + -(1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int
    } else {
        *src
            .offset(
                (0 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
    })
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (0 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (1 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t1: ::core::ffi::c_int = *src
        .offset(
            (1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (1 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t2: ::core::ffi::c_int = *src
        .offset(
            (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (2 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t3: ::core::ffi::c_int = *src
        .offset(
            (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (3 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t4: ::core::ffi::c_int = *src
        .offset(
            (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (4 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t5: ::core::ffi::c_int = *src
        .offset(
            (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (5 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t6: ::core::ffi::c_int = *src
        .offset(
            (6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (6 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t7: ::core::ffi::c_int = (if has_topright != 0 {
        *src
            .offset(
                (8 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
    } else {
        *src
            .offset(
                (7 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
    })
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (7 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (6 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l0: ::core::ffi::c_int = (if has_topleft != 0 {
        *src
            .offset(
                (-(1 as ::core::ffi::c_int) + -(1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int
    } else {
        *src
            .offset(
                (-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * stride) as isize,
            ) as ::core::ffi::c_int
    })
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l1: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l2: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l3: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 3 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l4: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 4 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l5: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 5 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l6: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 6 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l7: ::core::ffi::c_int = *src
        .offset((-(1 as ::core::ffi::c_int) + 6 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_int
        + 3 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 7 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int;
    let lt: ::core::ffi::c_int = *src
        .offset((-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (0 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    *src.offset((0 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize) = (l7
        + 2 as ::core::ffi::c_int * l6 + l5 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh233 = *src
        .offset((1 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize);
    *fresh233 = (l6 + 2 as ::core::ffi::c_int * l5 + l4 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((0 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize) = *fresh233;
    let ref mut fresh234 = *src
        .offset((2 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize);
    *fresh234 = (l5 + 2 as ::core::ffi::c_int * l4 + l3 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh235 = *src
        .offset((1 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh235 = *fresh234;
    *src.offset((0 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize) = *fresh235;
    let ref mut fresh236 = *src
        .offset((3 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize);
    *fresh236 = (l4 + 2 as ::core::ffi::c_int * l3 + l2 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh237 = *src
        .offset((2 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh237 = *fresh236;
    let ref mut fresh238 = *src
        .offset((1 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh238 = *fresh237;
    *src.offset((0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize) = *fresh238;
    let ref mut fresh239 = *src
        .offset((4 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize);
    *fresh239 = (l3 + 2 as ::core::ffi::c_int * l2 + l1 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh240 = *src
        .offset((3 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh240 = *fresh239;
    let ref mut fresh241 = *src
        .offset((2 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh241 = *fresh240;
    let ref mut fresh242 = *src
        .offset((1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh242 = *fresh241;
    *src.offset((0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize) = *fresh242;
    let ref mut fresh243 = *src
        .offset((5 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize);
    *fresh243 = (l2 + 2 as ::core::ffi::c_int * l1 + l0 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh244 = *src
        .offset((4 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh244 = *fresh243;
    let ref mut fresh245 = *src
        .offset((3 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh245 = *fresh244;
    let ref mut fresh246 = *src
        .offset((2 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh246 = *fresh245;
    let ref mut fresh247 = *src
        .offset((1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh247 = *fresh246;
    *src.offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize) = *fresh247;
    let ref mut fresh248 = *src
        .offset((6 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize);
    *fresh248 = (l1 + 2 as ::core::ffi::c_int * l0 + lt + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh249 = *src
        .offset((5 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh249 = *fresh248;
    let ref mut fresh250 = *src
        .offset((4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh250 = *fresh249;
    let ref mut fresh251 = *src
        .offset((3 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh251 = *fresh250;
    let ref mut fresh252 = *src
        .offset((2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh252 = *fresh251;
    let ref mut fresh253 = *src
        .offset((1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh253 = *fresh252;
    *src.offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize) = *fresh253;
    let ref mut fresh254 = *src
        .offset((7 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize);
    *fresh254 = (l0 + 2 as ::core::ffi::c_int * lt + t0 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh255 = *src
        .offset((6 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh255 = *fresh254;
    let ref mut fresh256 = *src
        .offset((5 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh256 = *fresh255;
    let ref mut fresh257 = *src
        .offset((4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh257 = *fresh256;
    let ref mut fresh258 = *src
        .offset((3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh258 = *fresh257;
    let ref mut fresh259 = *src
        .offset((2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh259 = *fresh258;
    let ref mut fresh260 = *src
        .offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh260 = *fresh259;
    *src.offset((0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh260;
    let ref mut fresh261 = *src
        .offset((7 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh261 = (lt + 2 as ::core::ffi::c_int * t0 + t1 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh262 = *src
        .offset((6 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh262 = *fresh261;
    let ref mut fresh263 = *src
        .offset((5 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh263 = *fresh262;
    let ref mut fresh264 = *src
        .offset((4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh264 = *fresh263;
    let ref mut fresh265 = *src
        .offset((3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh265 = *fresh264;
    let ref mut fresh266 = *src
        .offset((2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh266 = *fresh265;
    *src.offset((1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh266;
    let ref mut fresh267 = *src
        .offset((7 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh267 = (t0 + 2 as ::core::ffi::c_int * t1 + t2 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh268 = *src
        .offset((6 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh268 = *fresh267;
    let ref mut fresh269 = *src
        .offset((5 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh269 = *fresh268;
    let ref mut fresh270 = *src
        .offset((4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh270 = *fresh269;
    let ref mut fresh271 = *src
        .offset((3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh271 = *fresh270;
    *src.offset((2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh271;
    let ref mut fresh272 = *src
        .offset((7 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh272 = (t1 + 2 as ::core::ffi::c_int * t2 + t3 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh273 = *src
        .offset((6 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh273 = *fresh272;
    let ref mut fresh274 = *src
        .offset((5 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh274 = *fresh273;
    let ref mut fresh275 = *src
        .offset((4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh275 = *fresh274;
    *src.offset((3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh275;
    let ref mut fresh276 = *src
        .offset((7 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh276 = (t2 + 2 as ::core::ffi::c_int * t3 + t4 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh277 = *src
        .offset((6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh277 = *fresh276;
    let ref mut fresh278 = *src
        .offset((5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh278 = *fresh277;
    *src.offset((4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh278;
    let ref mut fresh279 = *src
        .offset((7 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh279 = (t3 + 2 as ::core::ffi::c_int * t4 + t5 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh280 = *src
        .offset((6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh280 = *fresh279;
    *src.offset((5 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh280;
    let ref mut fresh281 = *src
        .offset((7 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh281 = (t4 + 2 as ::core::ffi::c_int * t5 + t6 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((6 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh281;
    *src.offset((7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = (t5
        + 2 as ::core::ffi::c_int * t6 + t7 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
}




unsafe extern "C" fn pred8x8l_vertical_right_8_c(
    mut _src: *mut uint8_t,
    mut has_topleft: ::core::ffi::c_int,
    mut has_topright: ::core::ffi::c_int,
    mut _stride: ptrdiff_t,
) {
    let mut src: *mut uint8_t = _src;
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    let t0: ::core::ffi::c_int = (if has_topleft != 0 {
        *src
            .offset(
                (-(1 as ::core::ffi::c_int) + -(1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int
    } else {
        *src
            .offset(
                (0 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
    })
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (0 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (1 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t1: ::core::ffi::c_int = *src
        .offset(
            (1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (1 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t2: ::core::ffi::c_int = *src
        .offset(
            (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (2 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t3: ::core::ffi::c_int = *src
        .offset(
            (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (3 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t4: ::core::ffi::c_int = *src
        .offset(
            (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (4 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t5: ::core::ffi::c_int = *src
        .offset(
            (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (5 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t6: ::core::ffi::c_int = *src
        .offset(
            (6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (6 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t7: ::core::ffi::c_int = (if has_topright != 0 {
        *src
            .offset(
                (8 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
    } else {
        *src
            .offset(
                (7 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
    })
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (7 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (6 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l0: ::core::ffi::c_int = (if has_topleft != 0 {
        *src
            .offset(
                (-(1 as ::core::ffi::c_int) + -(1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int
    } else {
        *src
            .offset(
                (-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * stride) as isize,
            ) as ::core::ffi::c_int
    })
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l1: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l2: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l3: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 3 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l4: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 4 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l5: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 5 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l6: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 6 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l7: ::core::ffi::c_int = *src
        .offset((-(1 as ::core::ffi::c_int) + 6 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_int
        + 3 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 7 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int;
    let lt: ::core::ffi::c_int = *src
        .offset((-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (0 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    *src.offset((0 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize) = (l5
        + 2 as ::core::ffi::c_int * l4 + l3 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((0 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize) = (l6
        + 2 as ::core::ffi::c_int * l5 + l4 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh191 = *src
        .offset((1 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh191 = (l3 + 2 as ::core::ffi::c_int * l2 + l1 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize) = *fresh191;
    let ref mut fresh192 = *src
        .offset((1 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize);
    *fresh192 = (l4 + 2 as ::core::ffi::c_int * l3 + l2 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((0 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize) = *fresh192;
    let ref mut fresh193 = *src
        .offset((2 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh193 = (l1 + 2 as ::core::ffi::c_int * l0 + lt + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh194 = *src
        .offset((1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh194 = *fresh193;
    *src.offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize) = *fresh194;
    let ref mut fresh195 = *src
        .offset((2 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize);
    *fresh195 = (l2 + 2 as ::core::ffi::c_int * l1 + l0 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh196 = *src
        .offset((1 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh196 = *fresh195;
    *src.offset((0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize) = *fresh196;
    let ref mut fresh197 = *src
        .offset((3 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize);
    *fresh197 = (l0 + 2 as ::core::ffi::c_int * lt + t0 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh198 = *src
        .offset((2 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh198 = *fresh197;
    let ref mut fresh199 = *src
        .offset((1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh199 = *fresh198;
    *src.offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize) = *fresh199;
    let ref mut fresh200 = *src
        .offset((3 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh200 = (lt + t0 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as uint8_t;
    let ref mut fresh201 = *src
        .offset((2 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh201 = *fresh200;
    let ref mut fresh202 = *src
        .offset((1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh202 = *fresh201;
    *src.offset((0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh202;
    let ref mut fresh203 = *src
        .offset((4 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize);
    *fresh203 = (lt + 2 as ::core::ffi::c_int * t0 + t1 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh204 = *src
        .offset((3 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh204 = *fresh203;
    let ref mut fresh205 = *src
        .offset((2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh205 = *fresh204;
    *src.offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize) = *fresh205;
    let ref mut fresh206 = *src
        .offset((4 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh206 = (t0 + t1 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as uint8_t;
    let ref mut fresh207 = *src
        .offset((3 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh207 = *fresh206;
    let ref mut fresh208 = *src
        .offset((2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh208 = *fresh207;
    *src.offset((1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh208;
    let ref mut fresh209 = *src
        .offset((5 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize);
    *fresh209 = (t0 + 2 as ::core::ffi::c_int * t1 + t2 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh210 = *src
        .offset((4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh210 = *fresh209;
    let ref mut fresh211 = *src
        .offset((3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh211 = *fresh210;
    *src.offset((2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize) = *fresh211;
    let ref mut fresh212 = *src
        .offset((5 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh212 = (t1 + t2 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as uint8_t;
    let ref mut fresh213 = *src
        .offset((4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh213 = *fresh212;
    let ref mut fresh214 = *src
        .offset((3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh214 = *fresh213;
    *src.offset((2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh214;
    let ref mut fresh215 = *src
        .offset((6 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize);
    *fresh215 = (t1 + 2 as ::core::ffi::c_int * t2 + t3 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh216 = *src
        .offset((5 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh216 = *fresh215;
    let ref mut fresh217 = *src
        .offset((4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh217 = *fresh216;
    *src.offset((3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize) = *fresh217;
    let ref mut fresh218 = *src
        .offset((6 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh218 = (t2 + t3 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as uint8_t;
    let ref mut fresh219 = *src
        .offset((5 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh219 = *fresh218;
    let ref mut fresh220 = *src
        .offset((4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh220 = *fresh219;
    *src.offset((3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh220;
    let ref mut fresh221 = *src
        .offset((7 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize);
    *fresh221 = (t2 + 2 as ::core::ffi::c_int * t3 + t4 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh222 = *src
        .offset((6 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh222 = *fresh221;
    let ref mut fresh223 = *src
        .offset((5 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh223 = *fresh222;
    *src.offset((4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize) = *fresh223;
    let ref mut fresh224 = *src
        .offset((7 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh224 = (t3 + t4 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as uint8_t;
    let ref mut fresh225 = *src
        .offset((6 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh225 = *fresh224;
    let ref mut fresh226 = *src
        .offset((5 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh226 = *fresh225;
    *src.offset((4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh226;
    let ref mut fresh227 = *src
        .offset((7 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh227 = (t3 + 2 as ::core::ffi::c_int * t4 + t5 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh228 = *src
        .offset((6 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh228 = *fresh227;
    *src.offset((5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize) = *fresh228;
    let ref mut fresh229 = *src
        .offset((7 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh229 = (t4 + t5 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as uint8_t;
    let ref mut fresh230 = *src
        .offset((6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh230 = *fresh229;
    *src.offset((5 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh230;
    let ref mut fresh231 = *src
        .offset((7 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh231 = (t4 + 2 as ::core::ffi::c_int * t5 + t6 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize) = *fresh231;
    let ref mut fresh232 = *src
        .offset((7 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh232 = (t5 + t6 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as uint8_t;
    *src.offset((6 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh232;
    *src.offset((7 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize) = (t5
        + 2 as ::core::ffi::c_int * t6 + t7 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = (t6
        + t7 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
}




unsafe extern "C" fn pred8x8l_horizontal_down_8_c(
    mut _src: *mut uint8_t,
    mut has_topleft: ::core::ffi::c_int,
    mut has_topright: ::core::ffi::c_int,
    mut _stride: ptrdiff_t,
) {
    let mut src: *mut uint8_t = _src;
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    let t0: ::core::ffi::c_int = (if has_topleft != 0 {
        *src
            .offset(
                (-(1 as ::core::ffi::c_int) + -(1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int
    } else {
        *src
            .offset(
                (0 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
    })
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (0 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (1 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t1: ::core::ffi::c_int = *src
        .offset(
            (1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (1 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t2: ::core::ffi::c_int = *src
        .offset(
            (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (2 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t3: ::core::ffi::c_int = *src
        .offset(
            (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (3 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t4: ::core::ffi::c_int = *src
        .offset(
            (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (4 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t5: ::core::ffi::c_int = *src
        .offset(
            (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (5 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t6: ::core::ffi::c_int = *src
        .offset(
            (6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (6 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t7: ::core::ffi::c_int = (if has_topright != 0 {
        *src
            .offset(
                (8 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
    } else {
        *src
            .offset(
                (7 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
    })
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (7 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (6 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l0: ::core::ffi::c_int = (if has_topleft != 0 {
        *src
            .offset(
                (-(1 as ::core::ffi::c_int) + -(1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int
    } else {
        *src
            .offset(
                (-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * stride) as isize,
            ) as ::core::ffi::c_int
    })
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l1: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l2: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l3: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 3 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l4: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 4 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l5: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 5 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l6: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 6 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l7: ::core::ffi::c_int = *src
        .offset((-(1 as ::core::ffi::c_int) + 6 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_int
        + 3 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 7 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int;
    let lt: ::core::ffi::c_int = *src
        .offset((-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (0 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    *src.offset((0 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize) = (l6
        + l7 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
    *src.offset((1 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize) = (l5
        + 2 as ::core::ffi::c_int * l6 + l7 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh149 = *src
        .offset((2 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize);
    *fresh149 = (l5 + l6 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as uint8_t;
    *src.offset((0 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize) = *fresh149;
    let ref mut fresh150 = *src
        .offset((3 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize);
    *fresh150 = (l4 + 2 as ::core::ffi::c_int * l5 + l6 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((1 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize) = *fresh150;
    let ref mut fresh151 = *src
        .offset((4 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize);
    *fresh151 = (l4 + l5 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as uint8_t;
    let ref mut fresh152 = *src
        .offset((2 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh152 = *fresh151;
    *src.offset((0 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize) = *fresh152;
    let ref mut fresh153 = *src
        .offset((5 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize);
    *fresh153 = (l3 + 2 as ::core::ffi::c_int * l4 + l5 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh154 = *src
        .offset((3 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh154 = *fresh153;
    *src.offset((1 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize) = *fresh154;
    let ref mut fresh155 = *src
        .offset((6 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize);
    *fresh155 = (l3 + l4 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as uint8_t;
    let ref mut fresh156 = *src
        .offset((4 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh156 = *fresh155;
    let ref mut fresh157 = *src
        .offset((2 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh157 = *fresh156;
    *src.offset((0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize) = *fresh157;
    let ref mut fresh158 = *src
        .offset((7 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize);
    *fresh158 = (l2 + 2 as ::core::ffi::c_int * l3 + l4 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh159 = *src
        .offset((5 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh159 = *fresh158;
    let ref mut fresh160 = *src
        .offset((3 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh160 = *fresh159;
    *src.offset((1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize) = *fresh160;
    let ref mut fresh161 = *src
        .offset((6 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh161 = (l2 + l3 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as uint8_t;
    let ref mut fresh162 = *src
        .offset((4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh162 = *fresh161;
    let ref mut fresh163 = *src
        .offset((2 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh163 = *fresh162;
    *src.offset((0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize) = *fresh163;
    let ref mut fresh164 = *src
        .offset((7 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh164 = (l1 + 2 as ::core::ffi::c_int * l2 + l3 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh165 = *src
        .offset((5 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh165 = *fresh164;
    let ref mut fresh166 = *src
        .offset((3 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh166 = *fresh165;
    *src.offset((1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize) = *fresh166;
    let ref mut fresh167 = *src
        .offset((6 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh167 = (l1 + l2 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as uint8_t;
    let ref mut fresh168 = *src
        .offset((4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh168 = *fresh167;
    let ref mut fresh169 = *src
        .offset((2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh169 = *fresh168;
    *src.offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize) = *fresh169;
    let ref mut fresh170 = *src
        .offset((7 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh170 = (l0 + 2 as ::core::ffi::c_int * l1 + l2 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh171 = *src
        .offset((5 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh171 = *fresh170;
    let ref mut fresh172 = *src
        .offset((3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh172 = *fresh171;
    *src.offset((1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize) = *fresh172;
    let ref mut fresh173 = *src
        .offset((6 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh173 = (l0 + l1 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as uint8_t;
    let ref mut fresh174 = *src
        .offset((4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh174 = *fresh173;
    let ref mut fresh175 = *src
        .offset((2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh175 = *fresh174;
    *src.offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize) = *fresh175;
    let ref mut fresh176 = *src
        .offset((7 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh176 = (lt + 2 as ::core::ffi::c_int * l0 + l1 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh177 = *src
        .offset((5 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh177 = *fresh176;
    let ref mut fresh178 = *src
        .offset((3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh178 = *fresh177;
    *src.offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize) = *fresh178;
    let ref mut fresh179 = *src
        .offset((6 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh179 = (lt + l0 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as uint8_t;
    let ref mut fresh180 = *src
        .offset((4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh180 = *fresh179;
    let ref mut fresh181 = *src
        .offset((2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh181 = *fresh180;
    *src.offset((0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh181;
    let ref mut fresh182 = *src
        .offset((7 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh182 = (l0 + 2 as ::core::ffi::c_int * lt + t0 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh183 = *src
        .offset((5 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh183 = *fresh182;
    let ref mut fresh184 = *src
        .offset((3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh184 = *fresh183;
    *src.offset((1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh184;
    let ref mut fresh185 = *src
        .offset((6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh185 = (t1 + 2 as ::core::ffi::c_int * t0 + lt + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh186 = *src
        .offset((4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh186 = *fresh185;
    *src.offset((2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh186;
    let ref mut fresh187 = *src
        .offset((7 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh187 = (t2 + 2 as ::core::ffi::c_int * t1 + t0 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh188 = *src
        .offset((5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh188 = *fresh187;
    *src.offset((3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh188;
    let ref mut fresh189 = *src
        .offset((6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh189 = (t3 + 2 as ::core::ffi::c_int * t2 + t1 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh189;
    let ref mut fresh190 = *src
        .offset((7 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh190 = (t4 + 2 as ::core::ffi::c_int * t3 + t2 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((5 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = *fresh190;
    *src.offset((6 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = (t5
        + 2 as ::core::ffi::c_int * t4 + t3 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = (t6
        + 2 as ::core::ffi::c_int * t5 + t4 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
}






unsafe extern "C" fn pred8x8l_vertical_left_8_c(
    mut _src: *mut uint8_t,
    mut has_topleft: ::core::ffi::c_int,
    mut has_topright: ::core::ffi::c_int,
    mut _stride: ptrdiff_t,
) {
    let mut src: *mut uint8_t = _src;
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    let t0: ::core::ffi::c_int = (if has_topleft != 0 {
        *src
            .offset(
                (-(1 as ::core::ffi::c_int) + -(1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int
    } else {
        *src
            .offset(
                (0 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
    })
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (0 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (1 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t1: ::core::ffi::c_int = *src
        .offset(
            (1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (1 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t2: ::core::ffi::c_int = *src
        .offset(
            (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (2 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t3: ::core::ffi::c_int = *src
        .offset(
            (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (3 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t4: ::core::ffi::c_int = *src
        .offset(
            (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (4 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t5: ::core::ffi::c_int = *src
        .offset(
            (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (5 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t6: ::core::ffi::c_int = *src
        .offset(
            (6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (6 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t7: ::core::ffi::c_int = (if has_topright != 0 {
        *src
            .offset(
                (8 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
    } else {
        *src
            .offset(
                (7 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
    })
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (7 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (6 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let mut t8: ::core::ffi::c_int = 0;
    let mut t9: ::core::ffi::c_int = 0;
    let mut t10: ::core::ffi::c_int = 0;
    let mut t11: ::core::ffi::c_int = 0;
    let mut t12: ::core::ffi::c_int = 0;
    let mut t13: ::core::ffi::c_int = 0;
    let mut t14: ::core::ffi::c_int = 0;
    let mut t15: ::core::ffi::c_int = 0;
    if has_topright != 0 {
        t8 = *src
            .offset(
                (8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *src
                    .offset(
                        (8 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                            as isize,
                    ) as ::core::ffi::c_int
            + *src
                .offset(
                    (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        + -(1 as ::core::ffi::c_int) * stride) as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int;
        t9 = *src
            .offset(
                (9 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *src
                    .offset(
                        (9 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                            as isize,
                    ) as ::core::ffi::c_int
            + *src
                .offset(
                    (9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        + -(1 as ::core::ffi::c_int) * stride) as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int;
        t10 = *src
            .offset(
                (10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *src
                    .offset(
                        (10 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                            as isize,
                    ) as ::core::ffi::c_int
            + *src
                .offset(
                    (10 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        + -(1 as ::core::ffi::c_int) * stride) as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int;
        t11 = *src
            .offset(
                (11 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *src
                    .offset(
                        (11 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                            as isize,
                    ) as ::core::ffi::c_int
            + *src
                .offset(
                    (11 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        + -(1 as ::core::ffi::c_int) * stride) as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int;
        t12 = *src
            .offset(
                (12 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *src
                    .offset(
                        (12 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                            as isize,
                    ) as ::core::ffi::c_int
            + *src
                .offset(
                    (12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        + -(1 as ::core::ffi::c_int) * stride) as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int;
        t13 = *src
            .offset(
                (13 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *src
                    .offset(
                        (13 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                            as isize,
                    ) as ::core::ffi::c_int
            + *src
                .offset(
                    (13 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        + -(1 as ::core::ffi::c_int) * stride) as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int;
        t14 = *src
            .offset(
                (14 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *src
                    .offset(
                        (14 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                            as isize,
                    ) as ::core::ffi::c_int
            + *src
                .offset(
                    (14 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        + -(1 as ::core::ffi::c_int) * stride) as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int;
        t15 = *src
            .offset(
                (14 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int
                * *src
                    .offset(
                        (15 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                            as isize,
                    ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int;
    } else {
        t15 = *src
            .offset(
                (7 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int;
        t14 = t15;
        t13 = t14;
        t12 = t13;
        t11 = t12;
        t10 = t11;
        t9 = t10;
        t8 = t9;
    }
    *src.offset((0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = (t0
        + t1 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
    *src.offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize) = (t0
        + 2 as ::core::ffi::c_int * t1 + t2 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh107 = *src
        .offset((1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize);
    *fresh107 = (t1 + t2 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as uint8_t;
    *src.offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize) = *fresh107;
    let ref mut fresh108 = *src
        .offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh108 = (t1 + 2 as ::core::ffi::c_int * t2 + t3 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize) = *fresh108;
    let ref mut fresh109 = *src
        .offset((2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize);
    *fresh109 = (t2 + t3 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as uint8_t;
    let ref mut fresh110 = *src
        .offset((1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh110 = *fresh109;
    *src.offset((0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize) = *fresh110;
    let ref mut fresh111 = *src
        .offset((2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh111 = (t2 + 2 as ::core::ffi::c_int * t3 + t4 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh112 = *src
        .offset((1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh112 = *fresh111;
    *src.offset((0 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize) = *fresh112;
    let ref mut fresh113 = *src
        .offset((3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize);
    *fresh113 = (t3 + t4 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as uint8_t;
    let ref mut fresh114 = *src
        .offset((2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh114 = *fresh113;
    let ref mut fresh115 = *src
        .offset((1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh115 = *fresh114;
    *src.offset((0 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize) = *fresh115;
    let ref mut fresh116 = *src
        .offset((3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh116 = (t3 + 2 as ::core::ffi::c_int * t4 + t5 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh117 = *src
        .offset((2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh117 = *fresh116;
    let ref mut fresh118 = *src
        .offset((1 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh118 = *fresh117;
    *src.offset((0 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize) = *fresh118;
    let ref mut fresh119 = *src
        .offset((4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize);
    *fresh119 = (t4 + t5 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as uint8_t;
    let ref mut fresh120 = *src
        .offset((3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh120 = *fresh119;
    let ref mut fresh121 = *src
        .offset((2 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh121 = *fresh120;
    *src.offset((1 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize) = *fresh121;
    let ref mut fresh122 = *src
        .offset((4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh122 = (t4 + 2 as ::core::ffi::c_int * t5 + t6 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh123 = *src
        .offset((3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh123 = *fresh122;
    let ref mut fresh124 = *src
        .offset((2 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh124 = *fresh123;
    *src.offset((1 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize) = *fresh124;
    let ref mut fresh125 = *src
        .offset((5 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize);
    *fresh125 = (t5 + t6 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as uint8_t;
    let ref mut fresh126 = *src
        .offset((4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh126 = *fresh125;
    let ref mut fresh127 = *src
        .offset((3 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh127 = *fresh126;
    *src.offset((2 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize) = *fresh127;
    let ref mut fresh128 = *src
        .offset((5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh128 = (t5 + 2 as ::core::ffi::c_int * t6 + t7 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh129 = *src
        .offset((4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh129 = *fresh128;
    let ref mut fresh130 = *src
        .offset((3 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh130 = *fresh129;
    *src.offset((2 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize) = *fresh130;
    let ref mut fresh131 = *src
        .offset((6 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize);
    *fresh131 = (t6 + t7 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as uint8_t;
    let ref mut fresh132 = *src
        .offset((5 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh132 = *fresh131;
    let ref mut fresh133 = *src
        .offset((4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh133 = *fresh132;
    *src.offset((3 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize) = *fresh133;
    let ref mut fresh134 = *src
        .offset((6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh134 = (t6 + 2 as ::core::ffi::c_int * t7 + t8 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh135 = *src
        .offset((5 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh135 = *fresh134;
    let ref mut fresh136 = *src
        .offset((4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh136 = *fresh135;
    *src.offset((3 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize) = *fresh136;
    let ref mut fresh137 = *src
        .offset((7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize);
    *fresh137 = (t7 + t8 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as uint8_t;
    let ref mut fresh138 = *src
        .offset((6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh138 = *fresh137;
    let ref mut fresh139 = *src
        .offset((5 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh139 = *fresh138;
    *src.offset((4 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize) = *fresh139;
    let ref mut fresh140 = *src
        .offset((7 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh140 = (t7 + 2 as ::core::ffi::c_int * t8 + t9 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh141 = *src
        .offset((6 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh141 = *fresh140;
    let ref mut fresh142 = *src
        .offset((5 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh142 = *fresh141;
    *src.offset((4 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize) = *fresh142;
    let ref mut fresh143 = *src
        .offset((7 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh143 = (t8 + t9 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as uint8_t;
    let ref mut fresh144 = *src
        .offset((6 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh144 = *fresh143;
    *src.offset((5 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize) = *fresh144;
    let ref mut fresh145 = *src
        .offset((7 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh145 = (t8 + 2 as ::core::ffi::c_int * t9 + t10 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh146 = *src
        .offset((6 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh146 = *fresh145;
    *src.offset((5 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize) = *fresh146;
    let ref mut fresh147 = *src
        .offset((7 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh147 = (t9 + t10 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as uint8_t;
    *src.offset((6 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize) = *fresh147;
    let ref mut fresh148 = *src
        .offset((7 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh148 = (t9 + 2 as ::core::ffi::c_int * t10 + t11 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((6 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize) = *fresh148;
    *src.offset((7 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize) = (t10
        + t11 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
    *src.offset((7 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize) = (t10
        + 2 as ::core::ffi::c_int * t11 + t12 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
}


unsafe extern "C" fn pred8x8l_horizontal_up_8_c(
    mut _src: *mut uint8_t,
    mut has_topleft: ::core::ffi::c_int,
    mut has_topright: ::core::ffi::c_int,
    mut _stride: ptrdiff_t,
) {
    let mut src: *mut uint8_t = _src;
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    let l0: ::core::ffi::c_int = (if has_topleft != 0 {
        *src
            .offset(
                (-(1 as ::core::ffi::c_int) + -(1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int
    } else {
        *src
            .offset(
                (-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * stride) as isize,
            ) as ::core::ffi::c_int
    })
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l1: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l2: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l3: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 3 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l4: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 4 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l5: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 5 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l6: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 6 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l7: ::core::ffi::c_int = *src
        .offset((-(1 as ::core::ffi::c_int) + 6 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_int
        + 3 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 7 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int;
    *src.offset((0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = (l0
        + l1 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
    *src.offset((1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize) = (l0
        + 2 as ::core::ffi::c_int * l1 + l2 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh58 = *src
        .offset((2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize);
    *fresh58 = (l1 + l2 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
    *src.offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize) = *fresh58;
    let ref mut fresh59 = *src
        .offset((3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize);
    *fresh59 = (l1 + 2 as ::core::ffi::c_int * l2 + l3 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    *src.offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize) = *fresh59;
    let ref mut fresh60 = *src
        .offset((4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize);
    *fresh60 = (l2 + l3 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh61 = *src
        .offset((2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh61 = *fresh60;
    *src.offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize) = *fresh61;
    let ref mut fresh62 = *src
        .offset((5 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize);
    *fresh62 = (l2 + 2 as ::core::ffi::c_int * l3 + l4 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh63 = *src
        .offset((3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh63 = *fresh62;
    *src.offset((1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize) = *fresh63;
    let ref mut fresh64 = *src
        .offset((6 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize);
    *fresh64 = (l3 + l4 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh65 = *src
        .offset((4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh65 = *fresh64;
    let ref mut fresh66 = *src
        .offset((2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh66 = *fresh65;
    *src.offset((0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize) = *fresh66;
    let ref mut fresh67 = *src
        .offset((7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * stride) as isize);
    *fresh67 = (l3 + 2 as ::core::ffi::c_int * l4 + l5 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh68 = *src
        .offset((5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh68 = *fresh67;
    let ref mut fresh69 = *src
        .offset((3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh69 = *fresh68;
    *src.offset((1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize) = *fresh69;
    let ref mut fresh70 = *src
        .offset((6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh70 = (l4 + l5 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh71 = *src
        .offset((4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh71 = *fresh70;
    let ref mut fresh72 = *src
        .offset((2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh72 = *fresh71;
    *src.offset((0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize) = *fresh72;
    let ref mut fresh73 = *src
        .offset((7 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * stride) as isize);
    *fresh73 = (l4 + 2 as ::core::ffi::c_int * l5 + l6 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh74 = *src
        .offset((5 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh74 = *fresh73;
    let ref mut fresh75 = *src
        .offset((3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh75 = *fresh74;
    *src.offset((1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize) = *fresh75;
    let ref mut fresh76 = *src
        .offset((6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh76 = (l5 + l6 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh77 = *src
        .offset((4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh77 = *fresh76;
    let ref mut fresh78 = *src
        .offset((2 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh78 = *fresh77;
    *src.offset((0 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize) = *fresh78;
    let ref mut fresh79 = *src
        .offset((7 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * stride) as isize);
    *fresh79 = (l5 + 2 as ::core::ffi::c_int * l6 + l7 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh80 = *src
        .offset((5 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh80 = *fresh79;
    let ref mut fresh81 = *src
        .offset((3 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh81 = *fresh80;
    *src.offset((1 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize) = *fresh81;
    let ref mut fresh82 = *src
        .offset((6 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh82 = (l6 + l7 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh83 = *src
        .offset((4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh83 = *fresh82;
    let ref mut fresh84 = *src
        .offset((2 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh84 = *fresh83;
    *src.offset((0 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize) = *fresh84;
    let ref mut fresh85 = *src
        .offset((7 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * stride) as isize);
    *fresh85 = (l6 + 3 as ::core::ffi::c_int * l7 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as uint8_t;
    let ref mut fresh86 = *src
        .offset((5 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh86 = *fresh85;
    let ref mut fresh87 = *src
        .offset((3 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh87 = *fresh86;
    *src.offset((1 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize) = *fresh87;
    let ref mut fresh88 = *src
        .offset((7 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize);
    *fresh88 = l7 as uint8_t;
    let ref mut fresh89 = *src
        .offset((7 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh89 = *fresh88;
    let ref mut fresh90 = *src
        .offset((7 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh90 = *fresh89;
    let ref mut fresh91 = *src
        .offset((7 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh91 = *fresh90;
    let ref mut fresh92 = *src
        .offset((6 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize);
    *fresh92 = *fresh91;
    let ref mut fresh93 = *src
        .offset((6 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh93 = *fresh92;
    let ref mut fresh94 = *src
        .offset((6 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh94 = *fresh93;
    let ref mut fresh95 = *src
        .offset((6 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * stride) as isize);
    *fresh95 = *fresh94;
    let ref mut fresh96 = *src
        .offset((5 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize);
    *fresh96 = *fresh95;
    let ref mut fresh97 = *src
        .offset((5 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh97 = *fresh96;
    let ref mut fresh98 = *src
        .offset((5 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh98 = *fresh97;
    let ref mut fresh99 = *src
        .offset((4 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize);
    *fresh99 = *fresh98;
    let ref mut fresh100 = *src
        .offset((4 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh100 = *fresh99;
    let ref mut fresh101 = *src
        .offset((4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * stride) as isize);
    *fresh101 = *fresh100;
    let ref mut fresh102 = *src
        .offset((3 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize);
    *fresh102 = *fresh101;
    let ref mut fresh103 = *src
        .offset((3 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh103 = *fresh102;
    let ref mut fresh104 = *src
        .offset((2 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize);
    *fresh104 = *fresh103;
    let ref mut fresh105 = *src
        .offset((2 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * stride) as isize);
    *fresh105 = *fresh104;
    let ref mut fresh106 = *src
        .offset((1 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize);
    *fresh106 = *fresh105;
    *src.offset((0 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * stride) as isize) = *fresh106;
}




unsafe extern "C" fn pred8x8l_vertical_filter_add_8_c(
    mut _src: *mut uint8_t,
    mut _block: *mut int16_t,
    mut has_topleft: ::core::ffi::c_int,
    mut has_topright: ::core::ffi::c_int,
    mut _stride: ptrdiff_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut src: *mut uint8_t = _src;
    let mut block: *const int16_t = _block as *const int16_t;
    let mut pix: [uint8_t; 8] = [0; 8];
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    let t0: ::core::ffi::c_int = (if has_topleft != 0 {
        *src
            .offset(
                (-(1 as ::core::ffi::c_int) + -(1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int
    } else {
        *src
            .offset(
                (0 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
    })
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (0 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (1 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t1: ::core::ffi::c_int = *src
        .offset(
            (1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (1 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t2: ::core::ffi::c_int = *src
        .offset(
            (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (2 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t3: ::core::ffi::c_int = *src
        .offset(
            (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (3 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t4: ::core::ffi::c_int = *src
        .offset(
            (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (4 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t5: ::core::ffi::c_int = *src
        .offset(
            (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (5 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t6: ::core::ffi::c_int = *src
        .offset(
            (6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (6 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let t7: ::core::ffi::c_int = (if has_topright != 0 {
        *src
            .offset(
                (8 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
    } else {
        *src
            .offset(
                (7 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int
    })
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (7 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (6 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    pix[0 as ::core::ffi::c_int as usize] = t0 as uint8_t;
    pix[1 as ::core::ffi::c_int as usize] = t1 as uint8_t;
    pix[2 as ::core::ffi::c_int as usize] = t2 as uint8_t;
    pix[3 as ::core::ffi::c_int as usize] = t3 as uint8_t;
    pix[4 as ::core::ffi::c_int as usize] = t4 as uint8_t;
    pix[5 as ::core::ffi::c_int as usize] = t5 as uint8_t;
    pix[6 as ::core::ffi::c_int as usize] = t6 as uint8_t;
    pix[7 as ::core::ffi::c_int as usize] = t7 as uint8_t;
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        let mut v: uint8_t = pix[i as usize];
        v = (v as ::core::ffi::c_int
            + *block.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *src.offset((0 as ::core::ffi::c_int * stride) as isize) = v;
        v = (v as ::core::ffi::c_int
            + *block.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *src.offset((1 as ::core::ffi::c_int * stride) as isize) = v;
        v = (v as ::core::ffi::c_int
            + *block.offset(16 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *src.offset((2 as ::core::ffi::c_int * stride) as isize) = v;
        v = (v as ::core::ffi::c_int
            + *block.offset(24 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *src.offset((3 as ::core::ffi::c_int * stride) as isize) = v;
        v = (v as ::core::ffi::c_int
            + *block.offset(32 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *src.offset((4 as ::core::ffi::c_int * stride) as isize) = v;
        v = (v as ::core::ffi::c_int
            + *block.offset(40 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *src.offset((5 as ::core::ffi::c_int * stride) as isize) = v;
        v = (v as ::core::ffi::c_int
            + *block.offset(48 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *src.offset((6 as ::core::ffi::c_int * stride) as isize) = v;
        *src.offset((7 as ::core::ffi::c_int * stride) as isize) = (v
            as ::core::ffi::c_int
            + *block.offset(56 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        src = src.offset(1);
        block = block.offset(1);
        i += 1;
    }
    memset(
        _block as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<int16_t>() as size_t).wrapping_mul(64 as size_t),
    );
}




unsafe extern "C" fn pred8x8l_horizontal_filter_add_8_c(
    mut _src: *mut uint8_t,
    mut _block: *mut int16_t,
    mut has_topleft: ::core::ffi::c_int,
    mut has_topright: ::core::ffi::c_int,
    mut _stride: ptrdiff_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut src: *mut uint8_t = _src;
    let mut block: *const int16_t = _block as *const int16_t;
    let mut pix: [uint8_t; 8] = [0; 8];
    let mut stride: ::core::ffi::c_int = (_stride
        >> (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize))
        as ::core::ffi::c_int;
    let l0: ::core::ffi::c_int = (if has_topleft != 0 {
        *src
            .offset(
                (-(1 as ::core::ffi::c_int) + -(1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int
    } else {
        *src
            .offset(
                (-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * stride) as isize,
            ) as ::core::ffi::c_int
    })
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * stride) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l1: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l2: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l3: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 3 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l4: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 4 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l5: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 5 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l6: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + (6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * stride) as isize,
        ) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 6 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    let l7: ::core::ffi::c_int = *src
        .offset((-(1 as ::core::ffi::c_int) + 6 as ::core::ffi::c_int * stride) as isize)
        as ::core::ffi::c_int
        + 3 as ::core::ffi::c_int
            * *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 7 as ::core::ffi::c_int * stride)
                        as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int;
    pix[0 as ::core::ffi::c_int as usize] = l0 as uint8_t;
    pix[1 as ::core::ffi::c_int as usize] = l1 as uint8_t;
    pix[2 as ::core::ffi::c_int as usize] = l2 as uint8_t;
    pix[3 as ::core::ffi::c_int as usize] = l3 as uint8_t;
    pix[4 as ::core::ffi::c_int as usize] = l4 as uint8_t;
    pix[5 as ::core::ffi::c_int as usize] = l5 as uint8_t;
    pix[6 as ::core::ffi::c_int as usize] = l6 as uint8_t;
    pix[7 as ::core::ffi::c_int as usize] = l7 as uint8_t;
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        let mut v: uint8_t = pix[i as usize];
        v = (v as ::core::ffi::c_int
            + *block.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *src.offset(0 as ::core::ffi::c_int as isize) = v;
        v = (v as ::core::ffi::c_int
            + *block.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *src.offset(1 as ::core::ffi::c_int as isize) = v;
        v = (v as ::core::ffi::c_int
            + *block.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *src.offset(2 as ::core::ffi::c_int as isize) = v;
        v = (v as ::core::ffi::c_int
            + *block.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *src.offset(3 as ::core::ffi::c_int as isize) = v;
        v = (v as ::core::ffi::c_int
            + *block.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *src.offset(4 as ::core::ffi::c_int as isize) = v;
        v = (v as ::core::ffi::c_int
            + *block.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *src.offset(5 as ::core::ffi::c_int as isize) = v;
        v = (v as ::core::ffi::c_int
            + *block.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *src.offset(6 as ::core::ffi::c_int as isize) = v;
        *src.offset(7 as ::core::ffi::c_int as isize) = (v as ::core::ffi::c_int
            + *block.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        src = src.offset(stride as isize);
        block = block.offset(8 as ::core::ffi::c_int as isize);
        i += 1;
    }
    memset(
        _block as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<int16_t>() as size_t).wrapping_mul(64 as size_t),
    );
}



unsafe extern "C" fn pred4x4_vertical_add_8_c(
    mut _pix: *mut uint8_t,
    mut _block: *mut int16_t,
    mut stride: ptrdiff_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut pix: *mut uint8_t = _pix;
    let mut block: *const int16_t = _block as *const int16_t;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    pix = pix.offset(-(stride as isize));
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        let mut v: uint8_t = *pix.offset(0 as ::core::ffi::c_int as isize);
        v = (v as ::core::ffi::c_int
            + *block.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *pix.offset((1 as ptrdiff_t * stride) as isize) = v;
        v = (v as ::core::ffi::c_int
            + *block.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *pix.offset((2 as ptrdiff_t * stride) as isize) = v;
        v = (v as ::core::ffi::c_int
            + *block.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *pix.offset((3 as ptrdiff_t * stride) as isize) = v;
        *pix.offset((4 as ptrdiff_t * stride) as isize) = (v as ::core::ffi::c_int
            + *block.offset(12 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        pix = pix.offset(1);
        block = block.offset(1);
        i += 1;
    }
    memset(
        _block as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<int16_t>() as size_t).wrapping_mul(16 as size_t),
    );
}






unsafe extern "C" fn pred4x4_horizontal_add_8_c(
    mut _pix: *mut uint8_t,
    mut _block: *mut int16_t,
    mut stride: ptrdiff_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut pix: *mut uint8_t = _pix;
    let mut block: *const int16_t = _block as *const int16_t;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        let mut v: uint8_t = *pix.offset(-(1 as ::core::ffi::c_int) as isize);
        v = (v as ::core::ffi::c_int
            + *block.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *pix.offset(0 as ::core::ffi::c_int as isize) = v;
        v = (v as ::core::ffi::c_int
            + *block.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *pix.offset(1 as ::core::ffi::c_int as isize) = v;
        v = (v as ::core::ffi::c_int
            + *block.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *pix.offset(2 as ::core::ffi::c_int as isize) = v;
        *pix.offset(3 as ::core::ffi::c_int as isize) = (v as ::core::ffi::c_int
            + *block.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        pix = pix.offset(stride as isize);
        block = block.offset(4 as ::core::ffi::c_int as isize);
        i += 1;
    }
    memset(
        _block as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<int16_t>() as size_t).wrapping_mul(16 as size_t),
    );
}






unsafe extern "C" fn pred8x8l_vertical_add_8_c(
    mut _pix: *mut uint8_t,
    mut _block: *mut int16_t,
    mut stride: ptrdiff_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut pix: *mut uint8_t = _pix;
    let mut block: *const int16_t = _block as *const int16_t;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    pix = pix.offset(-(stride as isize));
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        let mut v: uint8_t = *pix.offset(0 as ::core::ffi::c_int as isize);
        v = (v as ::core::ffi::c_int
            + *block.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *pix.offset((1 as ptrdiff_t * stride) as isize) = v;
        v = (v as ::core::ffi::c_int
            + *block.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *pix.offset((2 as ptrdiff_t * stride) as isize) = v;
        v = (v as ::core::ffi::c_int
            + *block.offset(16 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *pix.offset((3 as ptrdiff_t * stride) as isize) = v;
        v = (v as ::core::ffi::c_int
            + *block.offset(24 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *pix.offset((4 as ptrdiff_t * stride) as isize) = v;
        v = (v as ::core::ffi::c_int
            + *block.offset(32 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *pix.offset((5 as ptrdiff_t * stride) as isize) = v;
        v = (v as ::core::ffi::c_int
            + *block.offset(40 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *pix.offset((6 as ptrdiff_t * stride) as isize) = v;
        v = (v as ::core::ffi::c_int
            + *block.offset(48 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *pix.offset((7 as ptrdiff_t * stride) as isize) = v;
        *pix.offset((8 as ptrdiff_t * stride) as isize) = (v as ::core::ffi::c_int
            + *block.offset(56 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        pix = pix.offset(1);
        block = block.offset(1);
        i += 1;
    }
    memset(
        _block as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<int16_t>() as size_t).wrapping_mul(64 as size_t),
    );
}


unsafe extern "C" fn pred8x8l_horizontal_add_8_c(
    mut _pix: *mut uint8_t,
    mut _block: *mut int16_t,
    mut stride: ptrdiff_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut pix: *mut uint8_t = _pix;
    let mut block: *const int16_t = _block as *const int16_t;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        let mut v: uint8_t = *pix.offset(-(1 as ::core::ffi::c_int) as isize);
        v = (v as ::core::ffi::c_int
            + *block.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *pix.offset(0 as ::core::ffi::c_int as isize) = v;
        v = (v as ::core::ffi::c_int
            + *block.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *pix.offset(1 as ::core::ffi::c_int as isize) = v;
        v = (v as ::core::ffi::c_int
            + *block.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *pix.offset(2 as ::core::ffi::c_int as isize) = v;
        v = (v as ::core::ffi::c_int
            + *block.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *pix.offset(3 as ::core::ffi::c_int as isize) = v;
        v = (v as ::core::ffi::c_int
            + *block.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *pix.offset(4 as ::core::ffi::c_int as isize) = v;
        v = (v as ::core::ffi::c_int
            + *block.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *pix.offset(5 as ::core::ffi::c_int as isize) = v;
        v = (v as ::core::ffi::c_int
            + *block.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        *pix.offset(6 as ::core::ffi::c_int as isize) = v;
        *pix.offset(7 as ::core::ffi::c_int as isize) = (v as ::core::ffi::c_int
            + *block.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint8_t;
        pix = pix.offset(stride as isize);
        block = block.offset(8 as ::core::ffi::c_int as isize);
        i += 1;
    }
    memset(
        _block as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<int16_t>() as size_t).wrapping_mul(64 as size_t),
    );
}



unsafe extern "C" fn pred16x16_vertical_add_8_c(
    mut pix: *mut uint8_t,
    mut block_offset: *const ::core::ffi::c_int,
    mut block: *mut int16_t,
    mut stride: ptrdiff_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        pred4x4_vertical_add_8_c(
            pix.offset(*block_offset.offset(i as isize) as isize),
            block
                .offset(
                    ((i * 16 as ::core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                        as isize,
                ),
            stride,
        );
        i += 1;
    }
}





unsafe extern "C" fn pred16x16_horizontal_add_8_c(
    mut pix: *mut uint8_t,
    mut block_offset: *const ::core::ffi::c_int,
    mut block: *mut int16_t,
    mut stride: ptrdiff_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        pred4x4_horizontal_add_8_c(
            pix.offset(*block_offset.offset(i as isize) as isize),
            block
                .offset(
                    ((i * 16 as ::core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                        as isize,
                ),
            stride,
        );
        i += 1;
    }
}




unsafe extern "C" fn pred8x8_vertical_add_8_c(
    mut pix: *mut uint8_t,
    mut block_offset: *const ::core::ffi::c_int,
    mut block: *mut int16_t,
    mut stride: ptrdiff_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        pred4x4_vertical_add_8_c(
            pix.offset(*block_offset.offset(i as isize) as isize),
            block
                .offset(
                    ((i * 16 as ::core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                        as isize,
                ),
            stride,
        );
        i += 1;
    }
}






unsafe extern "C" fn pred8x16_vertical_add_8_c(
    mut pix: *mut uint8_t,
    mut block_offset: *const ::core::ffi::c_int,
    mut block: *mut int16_t,
    mut stride: ptrdiff_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        pred4x4_vertical_add_8_c(
            pix.offset(*block_offset.offset(i as isize) as isize),
            block
                .offset(
                    ((i * 16 as ::core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                        as isize,
                ),
            stride,
        );
        i += 1;
    }
    i = 4 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        pred4x4_vertical_add_8_c(
            pix
                .offset(
                    *block_offset.offset((i + 4 as ::core::ffi::c_int) as isize) as isize,
                ),
            block
                .offset(
                    ((i * 16 as ::core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                        as isize,
                ),
            stride,
        );
        i += 1;
    }
}



unsafe extern "C" fn pred8x8_horizontal_add_8_c(
    mut pix: *mut uint8_t,
    mut block_offset: *const ::core::ffi::c_int,
    mut block: *mut int16_t,
    mut stride: ptrdiff_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        pred4x4_horizontal_add_8_c(
            pix.offset(*block_offset.offset(i as isize) as isize),
            block
                .offset(
                    ((i * 16 as ::core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                        as isize,
                ),
            stride,
        );
        i += 1;
    }
}





unsafe extern "C" fn pred8x16_horizontal_add_8_c(
    mut pix: *mut uint8_t,
    mut block_offset: *const ::core::ffi::c_int,
    mut block: *mut int16_t,
    mut stride: ptrdiff_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        pred4x4_horizontal_add_8_c(
            pix.offset(*block_offset.offset(i as isize) as isize),
            block
                .offset(
                    ((i * 16 as ::core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                        as isize,
                ),
            stride,
        );
        i += 1;
    }
    i = 4 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        pred4x4_horizontal_add_8_c(
            pix
                .offset(
                    *block_offset.offset((i + 4 as ::core::ffi::c_int) as isize) as isize,
                ),
            block
                .offset(
                    ((i * 16 as ::core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                        as isize,
                ),
            stride,
        );
        i += 1;
    }
}
