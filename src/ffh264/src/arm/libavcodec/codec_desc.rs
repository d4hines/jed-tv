// Translated from FFmpeg 9.0.1 libavcodec/codec_desc.c
// (preprocessed for armv7-unknown-linux-musleabihf) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// This file is part of FFmpeg.
//
// This table was generated from the long and short names of AVCodecs
// please see the respective codec sources for authorship
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
    fn bsearch(
        _: *const ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
        _: size_t,
        _: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    ) -> *mut ::core::ffi::c_void;
    static ff_aac_profiles: [AVProfile; 0];
    static ff_dca_profiles: [AVProfile; 0];
    static ff_eac3_profiles: [AVProfile; 0];
    static ff_truehd_profiles: [AVProfile; 0];
    static ff_dnxhd_profiles: [AVProfile; 0];
    static ff_h264_profiles: [AVProfile; 0];
    static ff_hevc_profiles: [AVProfile; 0];
    static ff_vvc_profiles: [AVProfile; 0];
    static ff_jpeg2000_profiles: [AVProfile; 0];
    static ff_mpeg2_video_profiles: [AVProfile; 0];
    static ff_mpeg4_video_profiles: [AVProfile; 0];
    static ff_vc1_profiles: [AVProfile; 0];
    static ff_vp9_profiles: [AVProfile; 0];
    static ff_av1_profiles: [AVProfile; 0];
    static ff_prores_profiles: [AVProfile; 0];
    static ff_prores_raw_profiles: [AVProfile; 0];
    static ff_mjpeg_profiles: [AVProfile; 0];
    static ff_arib_caption_profiles: [AVProfile; 0];
    static ff_evc_profiles: [AVProfile; 0];
}
pub type size_t = ::core::ffi::c_uint;
pub type AVMediaType = ::core::ffi::c_int;
pub const AVMEDIA_TYPE_NB: AVMediaType = 5;
pub const AVMEDIA_TYPE_ATTACHMENT: AVMediaType = 4;
pub const AVMEDIA_TYPE_SUBTITLE: AVMediaType = 3;
pub const AVMEDIA_TYPE_DATA: AVMediaType = 2;
pub const AVMEDIA_TYPE_AUDIO: AVMediaType = 1;
pub const AVMEDIA_TYPE_VIDEO: AVMediaType = 0;
pub const AVMEDIA_TYPE_UNKNOWN: AVMediaType = -1;
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
pub struct AVProfile {
    pub profile: ::core::ffi::c_int,
    pub name: *const ::core::ffi::c_char,
}
static mut codec_descriptors: [AVCodecDescriptor; 539] = unsafe {
    [
        AVCodecDescriptor {
            id: AV_CODEC_ID_MPEG1VIDEO,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"mpeg1video\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MPEG-1 video\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MPEG2VIDEO,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"mpeg2video\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MPEG-2 video\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: &raw const ff_mpeg2_video_profiles as *const AVProfile,
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_H261,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"h261\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"H.261\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_H263,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"h263\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"H.263 / H.263-1996, H.263+ / H.263-1998 / H.263 version 2\0"
                as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_RV10,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"rv10\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"RealVideo 1.0\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_RV20,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"rv20\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"RealVideo 2.0\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MJPEG,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"mjpeg\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Motion JPEG\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: [
                b"image/jpeg\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ]
                .as_ptr(),
            profiles: &raw const ff_mjpeg_profiles as *const AVProfile,
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MJPEGB,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"mjpegb\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Apple MJPEG-B\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_LJPEG,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"ljpeg\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Lossless JPEG\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SP5X,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"sp5x\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Sunplus JPEG (SP5X)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_JPEGLS,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"jpegls\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"JPEG-LS\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MPEG4,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"mpeg4\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MPEG-4 part 2\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: &raw const ff_mpeg4_video_profiles as *const AVProfile,
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_RAWVIDEO,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"rawvideo\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"raw video\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MSMPEG4V1,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"msmpeg4v1\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MPEG-4 part 2 Microsoft variant version 1\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MSMPEG4V2,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"msmpeg4v2\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MPEG-4 part 2 Microsoft variant version 2\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MSMPEG4V3,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"msmpeg4v3\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MPEG-4 part 2 Microsoft variant version 3\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_WMV1,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"wmv1\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Windows Media Video 7\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_WMV2,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"wmv2\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Windows Media Video 8\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_H263P,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"h263p\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"H.263+ / H.263-1998 / H.263 version 2\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_H263I,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"h263i\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Intel H.263\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_FLV1,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"flv1\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"FLV / Sorenson Spark / Sorenson H.263 (Flash Video)\0"
                as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SVQ1,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"svq1\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Sorenson Vector Quantizer 1 / Sorenson Video 1 / SVQ1\0"
                as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SVQ3,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"svq3\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Sorenson Vector Quantizer 3 / Sorenson Video 3 / SVQ3\0"
                as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_DVVIDEO,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"dvvideo\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"DV (Digital Video)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_HUFFYUV,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"huffyuv\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"HuffYUV\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_CYUV,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"cyuv\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Creative YUV (CYUV)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_H264,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"h264\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"H.264 / AVC / MPEG-4 AVC / MPEG-4 part 10\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: &raw const ff_h264_profiles as *const AVProfile,
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_INDEO3,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"indeo3\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Intel Indeo 3\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_VP3,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"vp3\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"On2 VP3\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_THEORA,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"theora\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Theora\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ASV1,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"asv1\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ASUS V1\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ASV2,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"asv2\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ASUS V2\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_FFV1,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"ffv1\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"FFmpeg video codec #1\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_4XM,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"4xm\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"4X Movie\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_VCR1,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"vcr1\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ATI VCR1\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_CLJR,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"cljr\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Cirrus Logic AccuPak\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MDEC,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"mdec\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Sony PlayStation MDEC (Motion DECoder)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ROQ,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"roq\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"id RoQ video\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_INTERPLAY_VIDEO,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"interplayvideo\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Interplay MVE video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_XAN_WC3,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"xan_wc3\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Wing Commander III / Xan\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_XAN_WC4,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"xan_wc4\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Wing Commander IV / Xxan\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_RPZA,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"rpza\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"QuickTime video (RPZA)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_CINEPAK,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"cinepak\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Cinepak\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_WS_VQA,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"ws_vqa\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Westwood Studios VQA (Vector Quantized Animation) video\0"
                as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MSRLE,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"msrle\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Microsoft RLE\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MSVIDEO1,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"msvideo1\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Microsoft Video 1\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_IDCIN,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"idcin\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"id Quake II CIN video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_8BPS,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"8bps\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"QuickTime 8BPS video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SMC,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"smc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"QuickTime Graphics (SMC)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_FLIC,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"flic\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Autodesk Animator Flic video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_TRUEMOTION1,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"truemotion1\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Duck TrueMotion 1.0\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_VMDVIDEO,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"vmdvideo\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Sierra VMD video\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MSZH,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"mszh\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"LCL (LossLess Codec Library) MSZH\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ZLIB,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"zlib\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"LCL (LossLess Codec Library) ZLIB\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_QTRLE,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"qtrle\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"QuickTime Animation (RLE) video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_TSCC,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"tscc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"TechSmith Screen Capture Codec\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ULTI,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"ulti\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"IBM UltiMotion\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_QDRAW,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"qdraw\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Apple QuickDraw\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_VIXL,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"vixl\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Miro VideoXL\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_QPEG,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"qpeg\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Q-team QPEG\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PNG,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"png\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PNG (Portable Network Graphics) image\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: [
                b"image/png\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ]
                .as_ptr(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PPM,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"ppm\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PPM (Portable PixelMap) image\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PBM,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"pbm\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PBM (Portable BitMap) image\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PGM,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"pgm\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PGM (Portable GrayMap) image\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PGMYUV,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"pgmyuv\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PGMYUV (Portable GrayMap YUV) image\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PAM,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"pam\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PAM (Portable AnyMap) image\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: [
                b"image/x-portable-pixmap\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ]
                .as_ptr(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_FFVHUFF,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"ffvhuff\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Huffyuv FFmpeg variant\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_RV30,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"rv30\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"RealVideo 3.0\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_RV40,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"rv40\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"RealVideo 4.0\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_VC1,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"vc1\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"SMPTE VC-1\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: &raw const ff_vc1_profiles as *const AVProfile,
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_WMV3,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"wmv3\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Windows Media Video 9\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: &raw const ff_vc1_profiles as *const AVProfile,
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_LOCO,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"loco\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"LOCO\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_WNV1,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"wnv1\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Winnov WNV1\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_AASC,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"aasc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Autodesk RLE\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_INDEO2,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"indeo2\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Intel Indeo 2\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_FRAPS,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"fraps\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Fraps\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_TRUEMOTION2,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"truemotion2\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Duck TrueMotion 2.0\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_BMP,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"bmp\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"BMP (Windows and OS/2 bitmap)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: [
                b"image/x-ms-bmp\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ]
                .as_ptr(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_CSCD,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"cscd\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"CamStudio\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MMVIDEO,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"mmvideo\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"American Laser Games MM Video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ZMBV,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"zmbv\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Zip Motion Blocks Video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_AVS,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"avs\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"AVS (Audio Video Standard) video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SMACKVIDEO,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"smackvideo\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Smacker video\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_NUV,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"nuv\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"NuppelVideo/RTJPEG\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_KMVC,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"kmvc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Karl Morton's video codec\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_FLASHSV,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"flashsv\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Flash Screen Video v1\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_CAVS,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"cavs\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Chinese AVS (Audio Video Standard) (AVS1-P2, JiZhun profile)\0"
                as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_JPEG2000,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"jpeg2000\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"JPEG 2000\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: [
                b"image/jp2\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ]
                .as_ptr(),
            profiles: &raw const ff_jpeg2000_profiles as *const AVProfile,
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_VMNC,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"vmnc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"VMware Screen Codec / VMware Video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_VP5,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"vp5\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"On2 VP5\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_VP6,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"vp6\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"On2 VP6\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_VP6F,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"vp6f\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"On2 VP6 (Flash version)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_TARGA,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"targa\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Truevision Targa image\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: [
                b"image/x-targa\0" as *const u8 as *const ::core::ffi::c_char,
                b"image/x-tga\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ]
                .as_ptr(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_DSICINVIDEO,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"dsicinvideo\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Delphine Software International CIN video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_TIERTEXSEQVIDEO,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"tiertexseqvideo\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Tiertex Limited SEQ video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_TIFF,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"tiff\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"TIFF image\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: [
                b"image/tiff\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ]
                .as_ptr(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_GIF,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"gif\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"CompuServe GIF (Graphics Interchange Format)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: [
                b"image/gif\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ]
                .as_ptr(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_DXA,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"dxa\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Feeble Files/ScummVM DXA\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_DNXHD,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"dnxhd\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"VC3/DNxHD\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: &raw const ff_dnxhd_profiles as *const AVProfile,
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_THP,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"thp\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Nintendo Gamecube THP video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SGI,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"sgi\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"SGI image\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_C93,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"c93\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Interplay C93\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_BETHSOFTVID,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"bethsoftvid\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Bethesda VID video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PTX,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"ptx\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"V.Flash PTX image\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_TXD,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"txd\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Renderware TXD (TeXture Dictionary) image\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_VP6A,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"vp6a\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"On2 VP6 (Flash version, with alpha channel)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_AMV,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"amv\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"AMV Video\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_VB,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"vb\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Beam Software VB\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCX,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"pcx\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PC Paintbrush PCX image\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: [
                b"image/x-pcx\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ]
                .as_ptr(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SUNRAST,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"sunrast\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Sun Rasterfile image\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_INDEO4,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"indeo4\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Intel Indeo Video Interactive 4\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_INDEO5,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"indeo5\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Intel Indeo Video Interactive 5\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MIMIC,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"mimic\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Mimic\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_RL2,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"rl2\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"RL2 video\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ESCAPE124,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"escape124\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Escape 124\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_DIRAC,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"dirac\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Dirac\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_BFI,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"bfi\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Brute Force & Ignorance\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_CMV,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"cmv\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Electronic Arts CMV video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MOTIONPIXELS,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"motionpixels\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Motion Pixels video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_TGV,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"tgv\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Electronic Arts TGV video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_TGQ,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"tgq\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Electronic Arts TGQ video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_TQI,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"tqi\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Electronic Arts TQI video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_AURA,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"aura\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Auravision AURA\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_AURA2,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"aura2\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Auravision Aura 2\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_V210X,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"v210x\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Uncompressed 4:2:2 10-bit\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_TMV,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"tmv\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"8088flex TMV\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_V210,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"v210\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Uncompressed 4:2:2 10-bit\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_DPX,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"dpx\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"DPX (Digital Picture Exchange) image\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MAD,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"mad\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Electronic Arts Madcow Video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_FRWU,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"frwu\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Forward Uncompressed\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_FLASHSV2,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"flashsv2\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Flash Screen Video v2\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_CDGRAPHICS,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"cdgraphics\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"CD Graphics video\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_R210,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"r210\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Uncompressed RGB 10-bit\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ANM,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"anm\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Deluxe Paint Animation\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_BINKVIDEO,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"binkvideo\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Bink video\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_IFF_ILBM,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"iff_ilbm\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"IFF ACBM/ANIM/DEEP/ILBM/PBM/RGB8/RGBN\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_KGV1,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"kgv1\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Kega Game Video\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_YOP,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"yop\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Psygnosis YOP Video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_VP8,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"vp8\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"On2 VP8\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PICTOR,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"pictor\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Pictor/PC Paint\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ANSI,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"ansi\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ASCII/ANSI art\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_A64_MULTI,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"a64_multi\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Multicolor charset for Commodore 64\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_A64_MULTI5,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"a64_multi5\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Multicolor charset for Commodore 64, extended with 5th color (colram)\0"
                as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_R10K,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"r10k\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"AJA Kona 10-bit RGB Codec\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MXPEG,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"mxpeg\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Mobotix MxPEG video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_LAGARITH,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"lagarith\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Lagarith lossless\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PRORES,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"prores\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Apple ProRes (iCodec Pro)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: &raw const ff_prores_profiles as *const AVProfile,
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_JV,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"jv\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Bitmap Brothers JV video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_DFA,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"dfa\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Chronomaster DFA\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_WMV3IMAGE,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"wmv3image\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Windows Media Video 9 Image\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_VC1IMAGE,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"vc1image\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Windows Media Video 9 Image v2\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_UTVIDEO,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"utvideo\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Ut Video\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_BMV_VIDEO,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"bmv_video\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Discworld II BMV video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_VBLE,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"vble\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"VBLE Lossless Codec\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_DXTORY,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"dxtory\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Dxtory\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_XWD,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"xwd\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"XWD (X Window Dump) image\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: [
                b"image/x-xwindowdump\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ]
                .as_ptr(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_CDXL,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"cdxl\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Commodore CDXL video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_XBM,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"xbm\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"XBM (X BitMap) image\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: [
                b"image/x-xbitmap\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ]
                .as_ptr(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ZEROCODEC,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"zerocodec\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ZeroCodec Lossless Video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MSS1,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"mss1\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MS Screen 1\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MSA1,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"msa1\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MS ATC Screen\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_TSCC2,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"tscc2\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"TechSmith Screen Codec 2\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MTS2,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"mts2\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MS Expression Encoder Screen\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_CLLC,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"cllc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Canopus Lossless Codec\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MSS2,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"mss2\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MS Windows Media Video V9 Screen\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_VP9,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"vp9\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Google VP9\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: &raw const ff_vp9_profiles as *const AVProfile,
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_AIC,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"aic\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Apple Intermediate Codec\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ESCAPE130,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"escape130\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Escape 130\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_G2M,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"g2m\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Go2Meeting\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_WEBP,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"webp\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"WebP\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: [
                b"image/webp\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ]
                .as_ptr(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_HNM4_VIDEO,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"hnm4video\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"HNM 4 video\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_HEVC,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"hevc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"H.265 / HEVC (High Efficiency Video Coding)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: &raw const ff_hevc_profiles as *const AVProfile,
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_FIC,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"fic\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Mirillis FIC\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ALIAS_PIX,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"alias_pix\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Alias/Wavefront PIX image\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_BRENDER_PIX,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"brender_pix\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"BRender PIX image\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PAF_VIDEO,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"paf_video\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Amazing Studio Packed Animation File Video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_EXR,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"exr\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"OpenEXR image\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_VP7,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"vp7\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"On2 VP7\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SANM,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"sanm\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"LucasArts SANM/SMUSH video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SGIRLE,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"sgirle\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"SGI RLE 8-bit\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MVC1,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"mvc1\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Silicon Graphics Motion Video Compressor 1\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MVC2,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"mvc2\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Silicon Graphics Motion Video Compressor 2\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_HQX,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"hqx\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Canopus HQX\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_TDSC,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"tdsc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"TDSC\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_HQ_HQA,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"hq_hqa\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Canopus HQ/HQA\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_HAP,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"hap\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Vidvox Hap\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_DDS,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"dds\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"DirectDraw Surface image decoder\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_DXV,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"dxv\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Resolume DXV\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SCREENPRESSO,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"screenpresso\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Screenpresso\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_RSCC,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"rscc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"innoHeim/Rsupport Screen Capture Codec\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_AVS2,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"avs2\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"AVS2-P2/IEEE1857.4\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PGX,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"pgx\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PGX (JPEG2000 Test Format)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_AVS3,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"avs3\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"AVS3-P2/IEEE1857.10\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MSP2,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"msp2\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Microsoft Paint (MSP) version 2\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_VVC,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"vvc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"H.266 / VVC (Versatile Video Coding)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: &raw const ff_vvc_profiles as *const AVProfile,
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_Y41P,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"y41p\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Uncompressed YUV 4:1:1 12-bit\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_AVRP,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"avrp\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Avid 1:1 10-bit RGB Packer\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_012V,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"012v\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Uncompressed 4:2:2 10-bit\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_AVUI,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"avui\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Avid Meridien Uncompressed\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_TARGA_Y216,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"targa_y216\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Pinnacle TARGA CineWave YUV16\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_YUV4,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"yuv4\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Uncompressed packed 4:2:0\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_AVRN,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"avrn\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Avid AVI Codec\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_CPIA,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"cpia\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"CPiA video format\0" as *const u8 as *const ::core::ffi::c_char,
            props: 0,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_XFACE,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"xface\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"X-face image\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SNOW,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"snow\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Snow\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SMVJPEG,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"smvjpeg\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Sigmatel Motion Video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_APNG,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"apng\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"APNG (Animated Portable Network Graphics) image\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: [
                b"image/png\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ]
                .as_ptr(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_DAALA,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"daala\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Daala\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_CFHD,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"cfhd\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"GoPro CineForm HD\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_TRUEMOTION2RT,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"truemotion2rt\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Duck TrueMotion 2.0 Real Time\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_M101,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"m101\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Matrox Uncompressed SD\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MAGICYUV,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"magicyuv\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MagicYUV video\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SHEERVIDEO,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"sheervideo\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"BitJazz SheerVideo\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_YLC,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"ylc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"YUY2 Lossless Codec\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PSD,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"psd\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Photoshop PSD file\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PIXLET,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"pixlet\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Apple Pixlet\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SPEEDHQ,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"speedhq\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"NewTek SpeedHQ\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_FMVC,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"fmvc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"FM Screen Capture Codec\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SCPR,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"scpr\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ScreenPressor\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_CLEARVIDEO,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"clearvideo\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Iterated Systems ClearVideo\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_XPM,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"xpm\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"XPM (X PixMap) image\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: [
                b"image/x-xpixmap\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ]
                .as_ptr(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_AV1,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"av1\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Alliance for Open Media AV1\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: &raw const ff_av1_profiles as *const AVProfile,
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_BITPACKED,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"bitpacked\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Bitpacked\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MSCC,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"mscc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Mandsoft Screen Capture Codec\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SRGC,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"srgc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Screen Recorder Gold Codec\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SVG,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"svg\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Scalable Vector Graphics\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: [
                b"image/svg+xml\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ]
                .as_ptr(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_GDV,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"gdv\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Gremlin Digital Video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_FITS,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"fits\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"FITS (Flexible Image Transport System)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_IMM4,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"imm4\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Infinity IMM4\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PROSUMER,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"prosumer\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Brooktree ProSumer Video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MWSC,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"mwsc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MatchWare Screen Capture Codec\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_WCMV,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"wcmv\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"WinCAM Motion Video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_RASC,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"rasc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"RemotelyAnywhere Screen Capture\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_HYMT,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"hymt\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"HuffYUV MT\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ARBC,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"arbc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Gryphon's Anim Compressor\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_AGM,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"agm\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Amuse Graphics Movie\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_LSCR,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"lscr\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"LEAD Screen Capture\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_VP4,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"vp4\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"On2 VP4\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_IMM5,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"imm5\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Infinity IMM5\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MVDV,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"mvdv\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MidiVid VQ\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MVHA,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"mvha\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MidiVid Archive Codec\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_CDTOONS,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"cdtoons\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"CDToons video\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MV30,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"mv30\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MidiVid 3.0\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_NOTCHLC,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"notchlc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"NotchLC\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PFM,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"pfm\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PFM (Portable FloatMap) image\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MOBICLIP,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"mobiclip\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MobiClip Video\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PHOTOCD,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"photocd\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Kodak Photo CD\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_IPU,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"ipu\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"IPU Video\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ARGO,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"argo\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Argonaut Games Video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_CRI,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"cri\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Cintel RAW\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SIMBIOSIS_IMX,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"simbiosis_imx\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Simbiosis Interactive IMX Video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SGA_VIDEO,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"sga\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Digital Pictures SGA Video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_GEM,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"gem\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"GEM Raster image\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_VBN,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"vbn\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Vizrt Binary Image\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_JPEGXL,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"jpegxl\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"JPEG XL\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: [
                b"image/jxl\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ]
                .as_ptr(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_QOI,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"qoi\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"QOI (Quite OK Image)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PHM,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"phm\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PHM (Portable HalfFloatMap) image\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_RADIANCE_HDR,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"hdr\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"HDR (Radiance RGBE format) image\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_WBMP,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"wbmp\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"WBMP (Wireless Application Protocol Bitmap) image\0"
                as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MEDIA100,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"media100\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Media 100i\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_VQC,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"vqc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ViewQuest VQC\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PDV,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"pdv\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PDV (PlayDate Video)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_EVC,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"evc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MPEG-5 EVC (Essential Video Coding)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: &raw const ff_evc_profiles as *const AVProfile,
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_RTV1,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"rtv1\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"RTV1 (RivaTuner Video)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_VMIX,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"vmix\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"vMix Video\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_LEAD,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"lead\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"LEAD MCMP\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_DNXUC,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"dnxuc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"DNxUncompressed / SMPTE RDD 50\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_RV60,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"rv60\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"RealVideo 6.0\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_JPEGXL_ANIM,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"jpegxl_anim\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"JPEG XL animated\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: [
                b"image/jxl\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ]
                .as_ptr(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_APV,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"apv\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Advanced Professional Video\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PRORES_RAW,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"prores_raw\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Apple ProRes RAW\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: &raw const ff_prores_raw_profiles as *const AVProfile,
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_JPEGXS,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"jpegxs\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"JPEG XS\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: [
                b"image/jxs\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ]
                .as_ptr(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_WEBP_ANIM,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"webp_anim\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Animated WebP\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: [
                b"image/webp\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ]
                .as_ptr(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_S16LE,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_s16le\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM signed 16-bit little-endian\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_S16BE,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_s16be\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM signed 16-bit big-endian\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_U16LE,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_u16le\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM unsigned 16-bit little-endian\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_U16BE,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_u16be\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM unsigned 16-bit big-endian\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_S8,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_s8\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM signed 8-bit\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_U8,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_u8\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM unsigned 8-bit\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_MULAW,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_mulaw\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM mu-law / G.711 mu-law\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_ALAW,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_alaw\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM A-law / G.711 A-law\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_S32LE,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_s32le\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM signed 32-bit little-endian\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_S32BE,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_s32be\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM signed 32-bit big-endian\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_U32LE,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_u32le\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM unsigned 32-bit little-endian\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_U32BE,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_u32be\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM unsigned 32-bit big-endian\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_S24LE,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_s24le\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM signed 24-bit little-endian\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_S24BE,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_s24be\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM signed 24-bit big-endian\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_U24LE,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_u24le\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM unsigned 24-bit little-endian\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_U24BE,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_u24be\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM unsigned 24-bit big-endian\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_S24DAUD,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_s24daud\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM D-Cinema audio signed 24-bit\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_S16LE_PLANAR,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_s16le_planar\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM signed 16-bit little-endian planar\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_DVD,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_dvd\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM signed 20|24-bit big-endian\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_F32BE,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_f32be\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM 32-bit floating point big-endian\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_F32LE,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_f32le\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM 32-bit floating point little-endian\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_F64BE,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_f64be\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM 64-bit floating point big-endian\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_F64LE,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_f64le\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM 64-bit floating point little-endian\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_BLURAY,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_bluray\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM signed 16|20|24-bit big-endian for Blu-ray media\0"
                as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_LXF,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_lxf\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM signed 20-bit little-endian planar\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_S302M,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"s302m\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"SMPTE 302M\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_S8_PLANAR,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_s8_planar\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM signed 8-bit planar\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_S24LE_PLANAR,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_s24le_planar\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM signed 24-bit little-endian planar\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_S32LE_PLANAR,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_s32le_planar\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM signed 32-bit little-endian planar\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_S16BE_PLANAR,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_s16be_planar\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM signed 16-bit big-endian planar\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_S64LE,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_s64le\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM signed 64-bit little-endian\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_S64BE,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_s64be\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM signed 64-bit big-endian\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_F16LE,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_f16le\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM 16.8 floating point little-endian\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_F24LE,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_f24le\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM 24.0 floating point little-endian\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_VIDC,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_vidc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM Archimedes VIDC\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PCM_SGA,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"pcm_sga\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PCM SGA\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_IMA_QT,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ima_qt\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM IMA QuickTime\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_IMA_WAV,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ima_wav\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM IMA WAV\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_IMA_DK3,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ima_dk3\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM IMA Duck DK3\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_IMA_DK4,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ima_dk4\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM IMA Duck DK4\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_IMA_WS,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ima_ws\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM IMA Westwood\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_IMA_SMJPEG,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ima_smjpeg\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM IMA Loki SDL MJPEG\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_MS,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ms\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM Microsoft\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_4XM,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_4xm\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM 4X Movie\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_XA,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_xa\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM CDROM XA\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_ADX,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_adx\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"SEGA CRI ADX ADPCM\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_EA,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ea\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM Electronic Arts\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_G726,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_g726\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"G.726 ADPCM\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_CT,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ct\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM Creative Technology\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_SWF,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_swf\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM Shockwave Flash\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_YAMAHA,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_yamaha\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM Yamaha\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_SBPRO_4,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_sbpro_4\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM Sound Blaster Pro 4-bit\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_SBPRO_3,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_sbpro_3\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM Sound Blaster Pro 2.6-bit\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_SBPRO_2,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_sbpro_2\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM Sound Blaster Pro 2-bit\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_THP,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_thp\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM Nintendo THP\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_IMA_AMV,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ima_amv\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM IMA AMV\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_EA_R1,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ea_r1\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM Electronic Arts R1\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_EA_R3,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ea_r3\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM Electronic Arts R3\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_EA_R2,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ea_r2\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM Electronic Arts R2\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_IMA_EA_SEAD,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ima_ea_sead\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM IMA Electronic Arts SEAD\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_IMA_EA_EACS,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ima_ea_eacs\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM IMA Electronic Arts EACS\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_EA_XAS,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ea_xas\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM Electronic Arts XAS\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_EA_MAXIS_XA,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ea_maxis_xa\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM Electronic Arts Maxis CDROM XA\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_IMA_ISS,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ima_iss\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM IMA Funcom ISS\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_G722,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_g722\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"G.722 ADPCM\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_IMA_APC,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ima_apc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM IMA CRYO APC\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_VIMA,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_vima\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"LucasArts VIMA audio\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_AFC,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_afc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM Nintendo Gamecube AFC\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_IMA_OKI,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ima_oki\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM IMA Dialogic OKI\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_DTK,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_dtk\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM Nintendo Gamecube DTK\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_IMA_RAD,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ima_rad\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM IMA Radical\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_G726LE,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_g726le\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"G.726 ADPCM little-endian\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_THP_LE,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_thp_le\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM Nintendo THP (Little-Endian)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_PSX,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_psx\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM Playstation\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_AICA,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_aica\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM Yamaha AICA\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_IMA_DAT4,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ima_dat4\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM IMA Eurocom DAT4\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_MTAF,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_mtaf\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM MTAF\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_AGM,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_agm\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM AmuseGraphics Movie AGM\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_ARGO,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_argo\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM Argonaut Games\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_IMA_SSI,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ima_ssi\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM IMA Simon & Schuster Interactive\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_ZORK,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_zork\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM Zork\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_IMA_APM,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ima_apm\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM IMA Ubisoft APM\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_IMA_ALP,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ima_alp\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM IMA High Voltage Software ALP\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_IMA_MTF,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ima_mtf\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM IMA Capcom's MT Framework\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_IMA_CUNNING,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ima_cunning\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM IMA Cunning Developments\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_IMA_MOFLEX,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ima_moflex\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM IMA MobiClip MOFLEX\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_IMA_ACORN,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ima_acorn\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM IMA Acorn Replay\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_XMD,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_xmd\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM Konami XMD\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_IMA_XBOX,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ima_xbox\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM IMA Xbox\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_SANYO,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_sanyo\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM Sanyo\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_IMA_HVQM4,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ima_hvqm4\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM IMA HVQM4\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_IMA_PDA,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ima_pda\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM IMA PlayDate\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_N64,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_n64\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM Silicon Graphics N64\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_IMA_HVQM2,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ima_hvqm2\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM IMA HVQM2\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_IMA_MAGIX,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ima_magix\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM IMA Magix\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_PSXC,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_psxc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM Playstation C\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_CIRCUS,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_circus\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM Circus\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ADPCM_IMA_ESCAPE,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"adpcm_ima_escape\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADPCM IMA Acorn Escape\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_AMR_NB,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"amr_nb\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"AMR-NB (Adaptive Multi-Rate NarrowBand)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_AMR_WB,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"amr_wb\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"AMR-WB (Adaptive Multi-Rate WideBand)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_RA_144,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"ra_144\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"RealAudio 1.0 (14.4K)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_RA_288,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"ra_288\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"RealAudio 2.0 (28.8K)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ROQ_DPCM,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"roq_dpcm\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"DPCM id RoQ\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_INTERPLAY_DPCM,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"interplay_dpcm\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"DPCM Interplay\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_XAN_DPCM,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"xan_dpcm\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"DPCM Xan\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SOL_DPCM,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"sol_dpcm\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"DPCM Sol\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SDX2_DPCM,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"sdx2_dpcm\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"DPCM Squareroot-Delta-Exact\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_GREMLIN_DPCM,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"gremlin_dpcm\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"DPCM Gremlin\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_DERF_DPCM,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"derf_dpcm\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"DPCM Xilam DERF\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_WADY_DPCM,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"wady_dpcm\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"DPCM Marble WADY\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_CBD2_DPCM,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"cbd2_dpcm\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"DPCM Cuberoot-Delta-Exact\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MP2,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"mp2\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MP2 (MPEG audio layer 2)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MP3,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"mp3\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MP3 (MPEG audio layer 3)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_AAC,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"aac\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"AAC (Advanced Audio Coding)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: &raw const ff_aac_profiles as *const AVProfile,
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_AC3,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"ac3\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ATSC A/52A (AC-3)\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_DTS,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"dts\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"DCA (DTS Coherent Acoustics)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: &raw const ff_dca_profiles as *const AVProfile,
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_VORBIS,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"vorbis\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Vorbis\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_DVAUDIO,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"dvaudio\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"DV audio\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_WMAV1,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"wmav1\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Windows Media Audio 1\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_WMAV2,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"wmav2\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Windows Media Audio 2\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MACE3,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"mace3\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MACE (Macintosh Audio Compression/Expansion) 3:1\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MACE6,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"mace6\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MACE (Macintosh Audio Compression/Expansion) 6:1\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_VMDAUDIO,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"vmdaudio\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Sierra VMD audio\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_FLAC,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"flac\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"FLAC (Free Lossless Audio Codec)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MP3ADU,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"mp3adu\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ADU (Application Data Unit) MP3 (MPEG audio layer 3)\0"
                as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MP3ON4,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"mp3on4\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MP3onMP4\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SHORTEN,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"shorten\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Shorten\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ALAC,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"alac\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ALAC (Apple Lossless Audio Codec)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_WESTWOOD_SND1,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"westwood_snd1\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Westwood Audio (SND1)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_GSM,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"gsm\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"GSM\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_QDM2,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"qdm2\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"QDesign Music Codec 2\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_COOK,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"cook\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Cook / Cooker / Gecko (RealAudio G2)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_TRUESPEECH,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"truespeech\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"DSP Group TrueSpeech\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_TTA,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"tta\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"TTA (True Audio)\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SMACKAUDIO,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"smackaudio\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Smacker audio\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_QCELP,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"qcelp\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"QCELP / PureVoice\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_WAVPACK,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"wavpack\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"WavPack\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_DSICINAUDIO,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"dsicinaudio\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Delphine Software International CIN audio\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_IMC,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"imc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"IMC (Intel Music Coder)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MUSEPACK7,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"musepack7\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Musepack SV7\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MLP,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"mlp\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MLP (Meridian Lossless Packing)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_GSM_MS,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"gsm_ms\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"GSM Microsoft variant\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ATRAC3,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"atrac3\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ATRAC3 (Adaptive TRansform Acoustic Coding 3)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_APE,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"ape\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Monkey's Audio\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_NELLYMOSER,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"nellymoser\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Nellymoser Asao\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MUSEPACK8,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"musepack8\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Musepack SV8\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SPEEX,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"speex\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Speex\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_WMAVOICE,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"wmavoice\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Windows Media Audio Voice\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_WMAPRO,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"wmapro\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Windows Media Audio 9 Professional\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_WMALOSSLESS,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"wmalossless\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Windows Media Audio Lossless\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ATRAC3P,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"atrac3p\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ATRAC3+ (Adaptive TRansform Acoustic Coding 3+)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_EAC3,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"eac3\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ATSC A/52B (AC-3, E-AC-3)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: &raw const ff_eac3_profiles as *const AVProfile,
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SIPR,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"sipr\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"RealAudio SIPR / ACELP.NET\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MP1,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"mp1\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MP1 (MPEG audio layer 1)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_TWINVQ,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"twinvq\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"VQF TwinVQ\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_TRUEHD,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"truehd\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"TrueHD\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: &raw const ff_truehd_profiles as *const AVProfile,
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MP4ALS,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"mp4als\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MPEG-4 Audio Lossless Coding (ALS)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ATRAC1,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"atrac1\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ATRAC1 (Adaptive TRansform Acoustic Coding)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_BINKAUDIO_RDFT,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"binkaudio_rdft\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Bink Audio (RDFT)\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_BINKAUDIO_DCT,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"binkaudio_dct\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Bink Audio (DCT)\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_AAC_LATM,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"aac_latm\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"AAC LATM (Advanced Audio Coding LATM syntax)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: &raw const ff_aac_profiles as *const AVProfile,
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_QDMC,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"qdmc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"QDesign Music\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_CELT,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"celt\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Constrained Energy Lapped Transform (CELT)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_G723_1,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"g723_1\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"G.723.1\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_G729,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"g729\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"G.729\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_8SVX_EXP,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"8svx_exp\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"8SVX exponential\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_8SVX_FIB,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"8svx_fib\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"8SVX fibonacci\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_BMV_AUDIO,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"bmv_audio\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Discworld II BMV audio\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_RALF,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"ralf\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"RealAudio Lossless\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_IAC,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"iac\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"IAC (Indeo Audio Coder)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ILBC,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"ilbc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"iLBC (Internet Low Bitrate Codec)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_OPUS,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"opus\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Opus (Opus Interactive Audio Codec)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_COMFORT_NOISE,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"comfortnoise\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"RFC 3389 Comfort Noise\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_TAK,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"tak\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"TAK (Tom's lossless Audio Kompressor)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_METASOUND,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"metasound\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Voxware MetaSound\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PAF_AUDIO,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"paf_audio\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Amazing Studio Packed Animation File Audio\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ON2AVC,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"avc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"On2 Audio for Video Codec\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_DSS_SP,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"dss_sp\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Digital Speech Standard - Standard Play mode (DSS SP)\0"
                as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_CODEC2,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"codec2\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"codec2 (very low bitrate speech codec)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_FFWAVESYNTH,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"wavesynth\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Wave synthesis pseudo-codec\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SONIC,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"sonic\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Sonic\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SONIC_LS,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"sonicls\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Sonic lossless\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_EVRC,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"evrc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"EVRC (Enhanced Variable Rate Codec)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SMV,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"smv\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"SMV (Selectable Mode Vocoder)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_DSD_LSBF,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"dsd_lsbf\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"DSD (Direct Stream Digital), least significant bit first\0"
                as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_DSD_MSBF,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"dsd_msbf\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"DSD (Direct Stream Digital), most significant bit first\0"
                as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_DSD_LSBF_PLANAR,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"dsd_lsbf_planar\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"DSD (Direct Stream Digital), least significant bit first, planar\0"
                as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_DSD_MSBF_PLANAR,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"dsd_msbf_planar\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"DSD (Direct Stream Digital), most significant bit first, planar\0"
                as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_4GV,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"4gv\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"4GV (Fourth Generation Vocoder)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_INTERPLAY_ACM,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"interplayacm\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Interplay ACM\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_XMA1,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"xma1\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Xbox Media Audio 1\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_XMA2,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"xma2\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Xbox Media Audio 2\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_DST,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"dst\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"DST (Direct Stream Transfer)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ATRAC3AL,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"atrac3al\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ATRAC3 AL (Adaptive TRansform Acoustic Coding 3 Advanced Lossless)\0"
                as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ATRAC3PAL,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"atrac3pal\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ATRAC3+ AL (Adaptive TRansform Acoustic Coding 3+ Advanced Lossless)\0"
                as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_DOLBY_E,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"dolby_e\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Dolby E\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_APTX,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"aptx\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"aptX (Audio Processing Technology for Bluetooth)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_APTX_HD,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"aptx_hd\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"aptX HD (Audio Processing Technology for Bluetooth)\0"
                as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SBC,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"sbc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"SBC (low-complexity subband codec)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ATRAC9,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"atrac9\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ATRAC9 (Adaptive TRansform Acoustic Coding 9)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_HCOM,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"hcom\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"HCOM Audio\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ACELP_KELVIN,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"acelp.kelvin\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Sipro ACELP.KELVIN\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MPEGH_3D_AUDIO,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"mpegh_3d_audio\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MPEG-H 3D Audio\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SIREN,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"siren\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Siren\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_HCA,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"hca\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"CRI HCA\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_FASTAUDIO,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"fastaudio\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MobiClip FastAudio\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MSNSIREN,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"msnsiren\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MSN Siren\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_DFPWM,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"dfpwm\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"DFPWM (Dynamic Filter Pulse Width Modulation)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_BONK,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"bonk\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Bonk audio\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MISC4,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"misc4\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Micronas SC-4 Audio\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_APAC,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"apac\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Marian's A-pac audio\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_FTR,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"ftr\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"FTR Voice\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_WAVARC,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"wavarc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Waveform Archiver\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_RKA,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"rka\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"RKA (RK Audio)\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_AC4,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"ac4\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"AC-4\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_OSQ,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"osq\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"OSQ (Original Sound Quality)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_QOA,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"qoa\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"QOA (Quite OK Audio)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_LC3,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"lc3\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"LC3 (Low Complexity Communication Codec)\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_G728,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"g728\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"G.728\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_AHX,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"ahx\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"CRI AHX\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_APPLE_APAC,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"apple_apac\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Apple Positional Audio Codec\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_DVD_SUBTITLE,
            type_0: AVMEDIA_TYPE_SUBTITLE,
            name: b"dvd_subtitle\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"DVD subtitles\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_DVB_SUBTITLE,
            type_0: AVMEDIA_TYPE_SUBTITLE,
            name: b"dvb_subtitle\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"DVB subtitles\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_TEXT,
            type_0: AVMEDIA_TYPE_SUBTITLE,
            name: b"text\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"raw UTF-8 text\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_XSUB,
            type_0: AVMEDIA_TYPE_SUBTITLE,
            name: b"xsub\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"XSUB\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SSA,
            type_0: AVMEDIA_TYPE_SUBTITLE,
            name: b"ssa\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"SSA (SubStation Alpha) subtitle\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MOV_TEXT,
            type_0: AVMEDIA_TYPE_SUBTITLE,
            name: b"mov_text\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MOV text\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_HDMV_PGS_SUBTITLE,
            type_0: AVMEDIA_TYPE_SUBTITLE,
            name: b"hdmv_pgs_subtitle\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"HDMV Presentation Graphic Stream subtitles\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_DVB_TELETEXT,
            type_0: AVMEDIA_TYPE_SUBTITLE,
            name: b"dvb_teletext\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"DVB teletext\0" as *const u8 as *const ::core::ffi::c_char,
            props: 0,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SRT,
            type_0: AVMEDIA_TYPE_SUBTITLE,
            name: b"srt\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"SubRip subtitle with embedded timing\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MICRODVD,
            type_0: AVMEDIA_TYPE_SUBTITLE,
            name: b"microdvd\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MicroDVD subtitle\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_EIA_608,
            type_0: AVMEDIA_TYPE_SUBTITLE,
            name: b"eia_608\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"EIA-608 closed captions\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_JACOSUB,
            type_0: AVMEDIA_TYPE_SUBTITLE,
            name: b"jacosub\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"JACOsub subtitle\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SAMI,
            type_0: AVMEDIA_TYPE_SUBTITLE,
            name: b"sami\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"SAMI subtitle\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_REALTEXT,
            type_0: AVMEDIA_TYPE_SUBTITLE,
            name: b"realtext\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"RealText subtitle\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_STL,
            type_0: AVMEDIA_TYPE_SUBTITLE,
            name: b"stl\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Spruce subtitle format\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SUBVIEWER1,
            type_0: AVMEDIA_TYPE_SUBTITLE,
            name: b"subviewer1\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"SubViewer v1 subtitle\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SUBVIEWER,
            type_0: AVMEDIA_TYPE_SUBTITLE,
            name: b"subviewer\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"SubViewer subtitle\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SUBRIP,
            type_0: AVMEDIA_TYPE_SUBTITLE,
            name: b"subrip\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"SubRip subtitle\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_WEBVTT,
            type_0: AVMEDIA_TYPE_SUBTITLE,
            name: b"webvtt\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"WebVTT subtitle\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MPL2,
            type_0: AVMEDIA_TYPE_SUBTITLE,
            name: b"mpl2\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MPL2 subtitle\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_VPLAYER,
            type_0: AVMEDIA_TYPE_SUBTITLE,
            name: b"vplayer\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"VPlayer subtitle\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_PJS,
            type_0: AVMEDIA_TYPE_SUBTITLE,
            name: b"pjs\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"PJS (Phoenix Japanimation Society) subtitle\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ASS,
            type_0: AVMEDIA_TYPE_SUBTITLE,
            name: b"ass\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ASS (Advanced SSA) subtitle\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_HDMV_TEXT_SUBTITLE,
            type_0: AVMEDIA_TYPE_SUBTITLE,
            name: b"hdmv_text_subtitle\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"HDMV Text subtitle\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_TTML,
            type_0: AVMEDIA_TYPE_SUBTITLE,
            name: b"ttml\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Timed Text Markup Language\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ARIB_CAPTION,
            type_0: AVMEDIA_TYPE_SUBTITLE,
            name: b"arib_caption\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ARIB STD-B24 caption\0" as *const u8
                as *const ::core::ffi::c_char,
            props: 0,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: &raw const ff_arib_caption_profiles as *const AVProfile,
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_IVTV_VBI,
            type_0: AVMEDIA_TYPE_SUBTITLE,
            name: b"ivtv_vbi\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"ivtv VBI captions\0" as *const u8 as *const ::core::ffi::c_char,
            props: 0,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_TTF,
            type_0: AVMEDIA_TYPE_DATA,
            name: b"ttf\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"TrueType font\0" as *const u8 as *const ::core::ffi::c_char,
            props: 0,
            mime_types: [
                b"application/x-truetype-font\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"application/x-font\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ]
                .as_ptr(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SCTE_35,
            type_0: AVMEDIA_TYPE_DATA,
            name: b"scte_35\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"SCTE 35 Message Queue\0" as *const u8
                as *const ::core::ffi::c_char,
            props: 0,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_EPG,
            type_0: AVMEDIA_TYPE_DATA,
            name: b"epg\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Electronic Program Guide\0" as *const u8
                as *const ::core::ffi::c_char,
            props: 0,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_BINTEXT,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"bintext\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Binary text\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_XBIN,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"xbin\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"eXtended BINary text\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_IDF,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"idf\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"iCEDraw text\0" as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_OTF,
            type_0: AVMEDIA_TYPE_DATA,
            name: b"otf\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"OpenType font\0" as *const u8 as *const ::core::ffi::c_char,
            props: 0,
            mime_types: [
                b"application/vnd.ms-opentype\0" as *const u8
                    as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ]
                .as_ptr(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SMPTE_KLV,
            type_0: AVMEDIA_TYPE_DATA,
            name: b"klv\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"SMPTE 336M Key-Length-Value (KLV) metadata\0" as *const u8
                as *const ::core::ffi::c_char,
            props: 0,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_DVD_NAV,
            type_0: AVMEDIA_TYPE_DATA,
            name: b"dvd_nav_packet\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"DVD Nav packet\0" as *const u8 as *const ::core::ffi::c_char,
            props: 0,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_TIMED_ID3,
            type_0: AVMEDIA_TYPE_DATA,
            name: b"timed_id3\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"timed ID3 metadata\0" as *const u8
                as *const ::core::ffi::c_char,
            props: 0,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_BIN_DATA,
            type_0: AVMEDIA_TYPE_DATA,
            name: b"bin_data\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"binary data\0" as *const u8 as *const ::core::ffi::c_char,
            props: 0,
            mime_types: [
                b"application/octet-stream\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ]
                .as_ptr(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SMPTE_2038,
            type_0: AVMEDIA_TYPE_DATA,
            name: b"smpte_2038\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"SMPTE ST 2038 VANC in MPEG-2 TS\0" as *const u8
                as *const ::core::ffi::c_char,
            props: 0,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_LCEVC,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"lcevc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"LCEVC (Low Complexity Enhancement Video Coding) / MPEG-5 LCEVC / MPEG-5 part 2\0"
                as *const u8 as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_SMPTE_436M_ANC,
            type_0: AVMEDIA_TYPE_DATA,
            name: b"smpte_436m_anc\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"MXF SMPTE-436M ANC\0" as *const u8
                as *const ::core::ffi::c_char,
            props: 0,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_MPEG2TS,
            type_0: AVMEDIA_TYPE_DATA,
            name: b"mpegts\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"raw MPEG-TS stream\0" as *const u8
                as *const ::core::ffi::c_char,
            props: 0,
            mime_types: [
                b"application/MP2T\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ]
                .as_ptr(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_WRAPPED_AVFRAME,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"wrapped_avframe\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"AVFrame to AVPacket passthrough\0" as *const u8
                as *const ::core::ffi::c_char,
            props: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_VNULL,
            type_0: AVMEDIA_TYPE_VIDEO,
            name: b"vnull\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Null video codec\0" as *const u8 as *const ::core::ffi::c_char,
            props: 0,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
        AVCodecDescriptor {
            id: AV_CODEC_ID_ANULL,
            type_0: AVMEDIA_TYPE_AUDIO,
            name: b"anull\0" as *const u8 as *const ::core::ffi::c_char,
            long_name: b"Null audio codec\0" as *const u8 as *const ::core::ffi::c_char,
            props: 0,
            mime_types: ::core::ptr::null::<*const ::core::ffi::c_char>(),
            profiles: ::core::ptr::null::<AVProfile>(),
        },
    ]
};
unsafe extern "C" fn descriptor_compare(
    mut key: *const ::core::ffi::c_void,
    mut member: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut id: AVCodecID = *(key as *const AVCodecID);
    let mut desc: *const AVCodecDescriptor = member as *const AVCodecDescriptor;
    return (id as ::core::ffi::c_uint).wrapping_sub((*desc).id as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn avcodec_descriptor_get(
    mut id: AVCodecID,
) -> *const AVCodecDescriptor {
    return bsearch(
        &raw mut id as *const ::core::ffi::c_void,
        &raw const codec_descriptors as *const AVCodecDescriptor
            as *const ::core::ffi::c_void,
        (::core::mem::size_of::<[AVCodecDescriptor; 539]>() as size_t)
            .wrapping_div(::core::mem::size_of::<AVCodecDescriptor>() as size_t),
        ::core::mem::size_of::<AVCodecDescriptor>() as size_t,
        Some(
            descriptor_compare
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    ) as *const AVCodecDescriptor;
}
#[no_mangle]
pub unsafe extern "C" fn avcodec_descriptor_next(
    mut prev: *const AVCodecDescriptor,
) -> *const AVCodecDescriptor {
    if prev.is_null() {
        return (&raw const codec_descriptors as *const AVCodecDescriptor)
            .offset(0 as ::core::ffi::c_int as isize) as *const AVCodecDescriptor;
    }
    if (prev.offset_from(&raw const codec_descriptors as *const AVCodecDescriptor)
        as ::core::ffi::c_int as usize)
        < (::core::mem::size_of::<[AVCodecDescriptor; 539]>() as usize)
            .wrapping_div(::core::mem::size_of::<AVCodecDescriptor>() as usize)
            .wrapping_sub(1 as usize)
    {
        return prev.offset(1 as ::core::ffi::c_int as isize);
    }
    return ::core::ptr::null::<AVCodecDescriptor>();
}
#[no_mangle]
pub unsafe extern "C" fn avcodec_descriptor_get_by_name(
    mut name: *const ::core::ffi::c_char,
) -> *const AVCodecDescriptor {
    let mut desc: *const AVCodecDescriptor = ::core::ptr::null::<AVCodecDescriptor>();
    loop {
        desc = avcodec_descriptor_next(desc);
        if desc.is_null() {
            break;
        }
        if strcmp((*desc).name, name) == 0 {
            return desc;
        }
    }
    return ::core::ptr::null::<AVCodecDescriptor>();
}
#[no_mangle]
pub unsafe extern "C" fn avcodec_get_type(mut codec_id: AVCodecID) -> AVMediaType {
    let mut desc: *const AVCodecDescriptor = avcodec_descriptor_get(codec_id);
    return (if !desc.is_null() {
        (*desc).type_0 as ::core::ffi::c_int
    } else {
        AVMEDIA_TYPE_UNKNOWN as ::core::ffi::c_int
    }) as AVMediaType;
}
