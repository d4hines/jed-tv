// Translated from FFmpeg 9.0.1 libavcodec/h2645_parse.c
// (preprocessed for armv7-unknown-linux-musleabihf) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// H.264/HEVC common parsing code
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
    fn av_strerror(
        errnum: ::core::ffi::c_int,
        errbuf: *mut ::core::ffi::c_char,
        errbuf_size: size_t,
    ) -> ::core::ffi::c_int;
    fn av_mallocz(size: size_t) -> *mut ::core::ffi::c_void;
    fn av_buffer_create(
        data: *mut uint8_t,
        size: size_t,
        free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint8_t) -> ()>,
        opaque: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_int,
    ) -> *mut AVBufferRef;
    fn av_malloc_array(nmemb: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn av_buffer_unref(buf: *mut *mut AVBufferRef);
    fn av_buffer_is_writable(buf: *const AVBufferRef) -> ::core::ffi::c_int;
    fn av_reallocp_array(
        ptr: *mut ::core::ffi::c_void,
        nmemb: size_t,
        size: size_t,
    ) -> ::core::ffi::c_int;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn av_fast_realloc(
        ptr: *mut ::core::ffi::c_void,
        size: *mut ::core::ffi::c_uint,
        min_size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn av_free(ptr: *mut ::core::ffi::c_void);
    fn av_freep(ptr: *mut ::core::ffi::c_void);
}
pub type size_t = ::core::ffi::c_uint;
pub type int64_t = ::core::ffi::c_longlong;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint32_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union unaligned_32 {
    pub l: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GetByteContext {
    pub buffer: *const uint8_t,
    pub buffer_end: *const uint8_t,
    pub buffer_start: *const uint8_t,
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const H264_NAL_UNSPECIFIED31: C2RustUnnamed = 31;
pub const H264_NAL_UNSPECIFIED30: C2RustUnnamed = 30;
pub const H264_NAL_UNSPECIFIED29: C2RustUnnamed = 29;
pub const H264_NAL_UNSPECIFIED28: C2RustUnnamed = 28;
pub const H264_NAL_UNSPECIFIED27: C2RustUnnamed = 27;
pub const H264_NAL_UNSPECIFIED26: C2RustUnnamed = 26;
pub const H264_NAL_UNSPECIFIED25: C2RustUnnamed = 25;
pub const H264_NAL_UNSPECIFIED24: C2RustUnnamed = 24;
pub const H264_NAL_RESERVED23: C2RustUnnamed = 23;
pub const H264_NAL_RESERVED22: C2RustUnnamed = 22;
pub const H264_NAL_DEPTH_EXTEN_SLICE: C2RustUnnamed = 21;
pub const H264_NAL_EXTEN_SLICE: C2RustUnnamed = 20;
pub const H264_NAL_AUXILIARY_SLICE: C2RustUnnamed = 19;
pub const H264_NAL_RESERVED18: C2RustUnnamed = 18;
pub const H264_NAL_RESERVED17: C2RustUnnamed = 17;
pub const H264_NAL_DPS: C2RustUnnamed = 16;
pub const H264_NAL_SUB_SPS: C2RustUnnamed = 15;
pub const H264_NAL_PREFIX: C2RustUnnamed = 14;
pub const H264_NAL_SPS_EXT: C2RustUnnamed = 13;
pub const H264_NAL_FILLER_DATA: C2RustUnnamed = 12;
pub const H264_NAL_END_STREAM: C2RustUnnamed = 11;
pub const H264_NAL_END_SEQUENCE: C2RustUnnamed = 10;
pub const H264_NAL_AUD: C2RustUnnamed = 9;
pub const H264_NAL_PPS: C2RustUnnamed = 8;
pub const H264_NAL_SPS: C2RustUnnamed = 7;
pub const H264_NAL_SEI: C2RustUnnamed = 6;
pub const H264_NAL_IDR_SLICE: C2RustUnnamed = 5;
pub const H264_NAL_DPC: C2RustUnnamed = 4;
pub const H264_NAL_DPB: C2RustUnnamed = 3;
pub const H264_NAL_DPA: C2RustUnnamed = 2;
pub const H264_NAL_SLICE: C2RustUnnamed = 1;
pub const H264_NAL_UNSPECIFIED: C2RustUnnamed = 0;
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
pub struct GetBitContext {
    pub buffer: *const uint8_t,
    pub index: ::core::ffi::c_int,
    pub size_in_bits: ::core::ffi::c_int,
    pub size_in_bits_plus8: ::core::ffi::c_int,
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
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const H2645_FLAG_USE_REF: C2RustUnnamed_0 = 4;
pub const H2645_FLAG_SMALL_PADDING: C2RustUnnamed_0 = 2;
pub const H2645_FLAG_IS_NALFF: C2RustUnnamed_0 = 1;
pub const VVC_RSV_IRAP_11: VVCNALUnitType = 11;
pub const VVC_IDR_W_RADL: VVCNALUnitType = 7;
pub const VVC_SUFFIX_APS_NUT: VVCNALUnitType = 18;
pub const VVC_OPI_NUT: VVCNALUnitType = 12;
pub const HEVC_NAL_PPS: HEVCNALUnitType = 34;
pub const HEVC_NAL_SPS: HEVCNALUnitType = 33;
pub const HEVC_NAL_VPS: HEVCNALUnitType = 32;
pub type VVCNALUnitType = ::core::ffi::c_uint;
pub const VVC_UNSPEC_31: VVCNALUnitType = 31;
pub const VVC_UNSPEC_30: VVCNALUnitType = 30;
pub const VVC_UNSPEC_29: VVCNALUnitType = 29;
pub const VVC_UNSPEC_28: VVCNALUnitType = 28;
pub const VVC_RSV_NVCL_27: VVCNALUnitType = 27;
pub const VVC_RSV_NVCL_26: VVCNALUnitType = 26;
pub const VVC_FD_NUT: VVCNALUnitType = 25;
pub const VVC_SUFFIX_SEI_NUT: VVCNALUnitType = 24;
pub const VVC_PREFIX_SEI_NUT: VVCNALUnitType = 23;
pub const VVC_EOB_NUT: VVCNALUnitType = 22;
pub const VVC_EOS_NUT: VVCNALUnitType = 21;
pub const VVC_AUD_NUT: VVCNALUnitType = 20;
pub const VVC_PH_NUT: VVCNALUnitType = 19;
pub const VVC_PREFIX_APS_NUT: VVCNALUnitType = 17;
pub const VVC_PPS_NUT: VVCNALUnitType = 16;
pub const VVC_SPS_NUT: VVCNALUnitType = 15;
pub const VVC_VPS_NUT: VVCNALUnitType = 14;
pub const VVC_DCI_NUT: VVCNALUnitType = 13;
pub const VVC_GDR_NUT: VVCNALUnitType = 10;
pub const VVC_CRA_NUT: VVCNALUnitType = 9;
pub const VVC_IDR_N_LP: VVCNALUnitType = 8;
pub const VVC_RSV_VCL_6: VVCNALUnitType = 6;
pub const VVC_RSV_VCL_5: VVCNALUnitType = 5;
pub const VVC_RSV_VCL_4: VVCNALUnitType = 4;
pub const VVC_RASL_NUT: VVCNALUnitType = 3;
pub const VVC_RADL_NUT: VVCNALUnitType = 2;
pub const VVC_STSA_NUT: VVCNALUnitType = 1;
pub const VVC_TRAIL_NUT: VVCNALUnitType = 0;
pub type HEVCNALUnitType = ::core::ffi::c_uint;
pub const HEVC_NAL_UNSPEC63: HEVCNALUnitType = 63;
pub const HEVC_NAL_UNSPEC62: HEVCNALUnitType = 62;
pub const HEVC_NAL_UNSPEC61: HEVCNALUnitType = 61;
pub const HEVC_NAL_UNSPEC60: HEVCNALUnitType = 60;
pub const HEVC_NAL_UNSPEC59: HEVCNALUnitType = 59;
pub const HEVC_NAL_UNSPEC58: HEVCNALUnitType = 58;
pub const HEVC_NAL_UNSPEC57: HEVCNALUnitType = 57;
pub const HEVC_NAL_UNSPEC56: HEVCNALUnitType = 56;
pub const HEVC_NAL_UNSPEC55: HEVCNALUnitType = 55;
pub const HEVC_NAL_UNSPEC54: HEVCNALUnitType = 54;
pub const HEVC_NAL_UNSPEC53: HEVCNALUnitType = 53;
pub const HEVC_NAL_UNSPEC52: HEVCNALUnitType = 52;
pub const HEVC_NAL_UNSPEC51: HEVCNALUnitType = 51;
pub const HEVC_NAL_UNSPEC50: HEVCNALUnitType = 50;
pub const HEVC_NAL_UNSPEC49: HEVCNALUnitType = 49;
pub const HEVC_NAL_UNSPEC48: HEVCNALUnitType = 48;
pub const HEVC_NAL_RSV_NVCL47: HEVCNALUnitType = 47;
pub const HEVC_NAL_RSV_NVCL46: HEVCNALUnitType = 46;
pub const HEVC_NAL_RSV_NVCL45: HEVCNALUnitType = 45;
pub const HEVC_NAL_RSV_NVCL44: HEVCNALUnitType = 44;
pub const HEVC_NAL_RSV_NVCL43: HEVCNALUnitType = 43;
pub const HEVC_NAL_RSV_NVCL42: HEVCNALUnitType = 42;
pub const HEVC_NAL_RSV_NVCL41: HEVCNALUnitType = 41;
pub const HEVC_NAL_SEI_SUFFIX: HEVCNALUnitType = 40;
pub const HEVC_NAL_SEI_PREFIX: HEVCNALUnitType = 39;
pub const HEVC_NAL_FD_NUT: HEVCNALUnitType = 38;
pub const HEVC_NAL_EOB_NUT: HEVCNALUnitType = 37;
pub const HEVC_NAL_EOS_NUT: HEVCNALUnitType = 36;
pub const HEVC_NAL_AUD: HEVCNALUnitType = 35;
pub const HEVC_NAL_RSV_VCL31: HEVCNALUnitType = 31;
pub const HEVC_NAL_RSV_VCL30: HEVCNALUnitType = 30;
pub const HEVC_NAL_RSV_VCL29: HEVCNALUnitType = 29;
pub const HEVC_NAL_RSV_VCL28: HEVCNALUnitType = 28;
pub const HEVC_NAL_RSV_VCL27: HEVCNALUnitType = 27;
pub const HEVC_NAL_RSV_VCL26: HEVCNALUnitType = 26;
pub const HEVC_NAL_RSV_VCL25: HEVCNALUnitType = 25;
pub const HEVC_NAL_RSV_VCL24: HEVCNALUnitType = 24;
pub const HEVC_NAL_RSV_IRAP_VCL23: HEVCNALUnitType = 23;
pub const HEVC_NAL_RSV_IRAP_VCL22: HEVCNALUnitType = 22;
pub const HEVC_NAL_CRA_NUT: HEVCNALUnitType = 21;
pub const HEVC_NAL_IDR_N_LP: HEVCNALUnitType = 20;
pub const HEVC_NAL_IDR_W_RADL: HEVCNALUnitType = 19;
pub const HEVC_NAL_BLA_N_LP: HEVCNALUnitType = 18;
pub const HEVC_NAL_BLA_W_RADL: HEVCNALUnitType = 17;
pub const HEVC_NAL_BLA_W_LP: HEVCNALUnitType = 16;
pub const HEVC_NAL_VCL_R15: HEVCNALUnitType = 15;
pub const HEVC_NAL_VCL_N14: HEVCNALUnitType = 14;
pub const HEVC_NAL_VCL_R13: HEVCNALUnitType = 13;
pub const HEVC_NAL_VCL_N12: HEVCNALUnitType = 12;
pub const HEVC_NAL_VCL_R11: HEVCNALUnitType = 11;
pub const HEVC_NAL_VCL_N10: HEVCNALUnitType = 10;
pub const HEVC_NAL_RASL_R: HEVCNALUnitType = 9;
pub const HEVC_NAL_RASL_N: HEVCNALUnitType = 8;
pub const HEVC_NAL_RADL_R: HEVCNALUnitType = 7;
pub const HEVC_NAL_RADL_N: HEVCNALUnitType = 6;
pub const HEVC_NAL_STSA_R: HEVCNALUnitType = 5;
pub const HEVC_NAL_STSA_N: HEVCNALUnitType = 4;
pub const HEVC_NAL_TSA_R: HEVCNALUnitType = 3;
pub const HEVC_NAL_TSA_N: HEVCNALUnitType = 2;
pub const HEVC_NAL_TRAIL_R: HEVCNALUnitType = 1;
pub const HEVC_NAL_TRAIL_N: HEVCNALUnitType = 0;
#[no_mangle]
pub unsafe extern "C" fn ff_h2645_extract_rbsp(
    mut src: *const uint8_t,
    mut length: ::core::ffi::c_int,
    mut rbsp: *mut H2645RBSP,
    mut nal: *mut H2645NAL,
    mut small_padding: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut i: ::core::ffi::c_int = 0;
    let mut si: ::core::ffi::c_int = 0;
    let mut di: ::core::ffi::c_int = 0;
    let mut dst: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    (*nal).skipped_bytes = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while (i + 1 as ::core::ffi::c_int) < length {
        if !(*src.offset(i as isize) != 0) {
            if i > 0 as ::core::ffi::c_int
                && *src.offset((i - 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                i -= 1;
            }
            if (i + 2 as ::core::ffi::c_int) < length
                && *src.offset((i + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && (*src.offset((i + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int == 3 as ::core::ffi::c_int
                    || *src.offset((i + 2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int == 1 as ::core::ffi::c_int)
            {
                if *src.offset((i + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                {
                    length = i;
                }
                break;
            }
        }
        i += 2 as ::core::ffi::c_int;
    }
    if i >= length - 1 as ::core::ffi::c_int && small_padding != 0 {
        (*nal).raw_data = src;
        (*nal).data = (*nal).raw_data;
        (*nal).raw_size = length;
        (*nal).size = (*nal).raw_size;
        return length;
    } else if i > length {
        i = length;
    }
    dst = (*rbsp).rbsp_buffer.offset((*rbsp).rbsp_buffer_size as isize) as *mut uint8_t;
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        i as size_t,
    );
    di = i;
    si = di;
    loop {
        if !((si + 2 as ::core::ffi::c_int) < length) {
            current_block = 5330834795799507926;
            break;
        }
        if *src.offset((si + 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            > 3 as ::core::ffi::c_int
        {
            let fresh0 = si;
            si = si + 1;
            let fresh1 = di;
            di = di + 1;
            *dst.offset(fresh1 as isize) = *src.offset(fresh0 as isize);
            let fresh2 = si;
            si = si + 1;
            let fresh3 = di;
            di = di + 1;
            *dst.offset(fresh3 as isize) = *src.offset(fresh2 as isize);
        } else if *src.offset(si as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            && *src.offset((si + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            && *src.offset((si + 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
        {
            if !(*src.offset((si + 2 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int == 3 as ::core::ffi::c_int)
            {
                current_block = 16382535732608865551;
                break;
            }
            let fresh4 = di;
            di = di + 1;
            *dst.offset(fresh4 as isize) = 0 as uint8_t;
            let fresh5 = di;
            di = di + 1;
            *dst.offset(fresh5 as isize) = 0 as uint8_t;
            si += 3 as ::core::ffi::c_int;
            if !(*nal).skipped_bytes_pos.is_null() {
                (*nal).skipped_bytes += 1;
                if (*nal).skipped_bytes_pos_size < (*nal).skipped_bytes {
                    (*nal).skipped_bytes_pos_size *= 2 as ::core::ffi::c_int;
                    if !((*nal).skipped_bytes_pos_size >= (*nal).skipped_bytes) {
                        av_log(
                            ::core::ptr::null_mut::<::core::ffi::c_void>(),
                            0 as ::core::ffi::c_int,
                            b"Assertion %s failed at %s:%d\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"nal->skipped_bytes_pos_size >= nal->skipped_bytes\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            b"src/libavcodec/h2645_parse.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            118 as ::core::ffi::c_int,
                        );
                        abort();
                    }
                    av_reallocp_array(
                        &raw mut (*nal).skipped_bytes_pos as *mut ::core::ffi::c_void,
                        (*nal).skipped_bytes_pos_size as size_t,
                        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
                    );
                    if (*nal).skipped_bytes_pos.is_null() {
                        (*nal).skipped_bytes_pos_size = 0 as ::core::ffi::c_int;
                        return -(12 as ::core::ffi::c_int);
                    }
                }
                if !(*nal).skipped_bytes_pos.is_null() {
                    *(*nal)
                        .skipped_bytes_pos
                        .offset(
                            ((*nal).skipped_bytes - 1 as ::core::ffi::c_int) as isize,
                        ) = di - 1 as ::core::ffi::c_int;
                }
            }
            continue;
        }
        let fresh6 = si;
        si = si + 1;
        let fresh7 = di;
        di = di + 1;
        *dst.offset(fresh7 as isize) = *src.offset(fresh6 as isize);
    }
    match current_block {
        5330834795799507926 => {
            while si < length {
                let fresh8 = si;
                si = si + 1;
                let fresh9 = di;
                di = di + 1;
                *dst.offset(fresh9 as isize) = *src.offset(fresh8 as isize);
            }
        }
        _ => {}
    }
    memset(
        dst.offset(di as isize) as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        64 as size_t,
    );
    (*nal).data = dst;
    (*nal).size = di;
    (*nal).raw_data = src;
    (*nal).raw_size = si;
    (*rbsp).rbsp_buffer_size += si;
    return si;
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
#[inline(always)]
unsafe extern "C" fn bytestream2_peek_be32u(
    mut g: *const GetByteContext,
) -> ::core::ffi::c_uint {
    return av_bswap32((*((*g).buffer as *const unaligned_32)).l) as ::core::ffi::c_uint;
}
#[inline(always)]
unsafe extern "C" fn bytestream2_peek_be32(
    mut g: *const GetByteContext,
) -> ::core::ffi::c_uint {
    if ((*g).buffer_end.offset_from((*g).buffer) as ::core::ffi::c_int)
        < 4 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_uint;
    }
    return bytestream2_peek_be32u(g);
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
#[inline]
unsafe extern "C" fn get_nalsize(
    mut nal_length_size: ::core::ffi::c_int,
    mut buf: *const uint8_t,
    mut buf_size: ::core::ffi::c_int,
    mut buf_index: *mut ::core::ffi::c_int,
    mut logctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut nalsize: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if *buf_index >= buf_size - nal_length_size {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    while i < nal_length_size {
        let fresh11 = *buf_index;
        *buf_index = *buf_index + 1;
        nalsize = ((nalsize as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
            | *buf.offset(fresh11 as isize) as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
        i += 1;
    }
    if nalsize <= 0 as ::core::ffi::c_int || nalsize > buf_size - *buf_index {
        av_log(
            logctx,
            16 as ::core::ffi::c_int,
            b"Invalid NAL unit size (%d > %d).\n\0" as *const u8
                as *const ::core::ffi::c_char,
            nalsize,
            buf_size - *buf_index,
        );
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    return nalsize;
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
static mut lcevc_nal_type_name: [*const ::core::ffi::c_char; 32] = [
    b"UNSPEC0\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC1\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC2\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC3\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC4\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC5\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC6\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC7\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC8\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC9\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC10\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC11\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC12\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC13\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC14\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC15\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC16\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC17\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC18\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC19\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC20\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC21\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC22\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC23\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC24\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC25\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC26\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC27\0" as *const u8 as *const ::core::ffi::c_char,
    b"NON_IDR_NUT\0" as *const u8 as *const ::core::ffi::c_char,
    b"IDR_NUT\0" as *const u8 as *const ::core::ffi::c_char,
    b"RSV_NUT\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC31\0" as *const u8 as *const ::core::ffi::c_char,
];
#[inline(always)]
unsafe extern "C" fn bytestream2_get_bytes_left(
    mut g: *const GetByteContext,
) -> ::core::ffi::c_int {
    return (*g).buffer_end.offset_from((*g).buffer) as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn bytestream2_skip(
    mut g: *mut GetByteContext,
    mut size: ::core::ffi::c_uint,
) {
    (*g).buffer = (*g)
        .buffer
        .offset(
            (if (*g).buffer_end.offset_from((*g).buffer) as ::core::ffi::c_int
                as ::core::ffi::c_uint > size
            {
                size
            } else {
                (*g).buffer_end.offset_from((*g).buffer) as ::core::ffi::c_int
                    as ::core::ffi::c_uint
            }) as isize,
        );
}
unsafe extern "C" fn lcevc_nal_unit_name(
    mut nal_type: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    if !(nal_type >= 0 as ::core::ffi::c_int && nal_type < 32 as ::core::ffi::c_int) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"nal_type >= 0 && nal_type < 32\0" as *const u8
                as *const ::core::ffi::c_char,
            b"src/libavcodec/h2645_parse.c\0" as *const u8 as *const ::core::ffi::c_char,
            189 as ::core::ffi::c_int,
        );
        abort();
    }
    return lcevc_nal_type_name[nal_type as usize];
}
#[inline(always)]
unsafe extern "C" fn bytestream2_tell(
    mut g: *const GetByteContext,
) -> ::core::ffi::c_int {
    return (*g).buffer.offset_from((*g).buffer_start) as ::core::ffi::c_int;
}
static mut vvc_nal_type_name: [*const ::core::ffi::c_char; 32] = [
    b"TRAIL_NUT\0" as *const u8 as *const ::core::ffi::c_char,
    b"STSA_NUT\0" as *const u8 as *const ::core::ffi::c_char,
    b"RADL_NUT\0" as *const u8 as *const ::core::ffi::c_char,
    b"RASL_NUT\0" as *const u8 as *const ::core::ffi::c_char,
    b"RSV_VCL4\0" as *const u8 as *const ::core::ffi::c_char,
    b"RSV_VCL5\0" as *const u8 as *const ::core::ffi::c_char,
    b"RSV_VCL6\0" as *const u8 as *const ::core::ffi::c_char,
    b"IDR_W_RADL\0" as *const u8 as *const ::core::ffi::c_char,
    b"IDR_N_LP\0" as *const u8 as *const ::core::ffi::c_char,
    b"CRA_NUT\0" as *const u8 as *const ::core::ffi::c_char,
    b"GDR_NUT\0" as *const u8 as *const ::core::ffi::c_char,
    b"RSV_IRAP_11\0" as *const u8 as *const ::core::ffi::c_char,
    b"OPI_NUT\0" as *const u8 as *const ::core::ffi::c_char,
    b"DCI_NUT\0" as *const u8 as *const ::core::ffi::c_char,
    b"VPS_NUT\0" as *const u8 as *const ::core::ffi::c_char,
    b"SPS_NUT\0" as *const u8 as *const ::core::ffi::c_char,
    b"PPS_NUT\0" as *const u8 as *const ::core::ffi::c_char,
    b"APS_PREFIX\0" as *const u8 as *const ::core::ffi::c_char,
    b"APS_SUFFIX\0" as *const u8 as *const ::core::ffi::c_char,
    b"PH_NUT\0" as *const u8 as *const ::core::ffi::c_char,
    b"AUD_NUT\0" as *const u8 as *const ::core::ffi::c_char,
    b"EOS_NUT\0" as *const u8 as *const ::core::ffi::c_char,
    b"EOB_NUT\0" as *const u8 as *const ::core::ffi::c_char,
    b"SEI_PREFIX\0" as *const u8 as *const ::core::ffi::c_char,
    b"SEI_SUFFIX\0" as *const u8 as *const ::core::ffi::c_char,
    b"FD_NUT\0" as *const u8 as *const ::core::ffi::c_char,
    b"RSV_NVCL26\0" as *const u8 as *const ::core::ffi::c_char,
    b"RSV_NVCL27\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC28\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC29\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC30\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC31\0" as *const u8 as *const ::core::ffi::c_char,
];
unsafe extern "C" fn vvc_nal_unit_name(
    mut nal_type: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    if !(nal_type >= 0 as ::core::ffi::c_int && nal_type < 32 as ::core::ffi::c_int) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"nal_type >= 0 && nal_type < 32\0" as *const u8
                as *const ::core::ffi::c_char,
            b"src/libavcodec/h2645_parse.c\0" as *const u8 as *const ::core::ffi::c_char,
            230 as ::core::ffi::c_int,
        );
        abort();
    }
    return vvc_nal_type_name[nal_type as usize];
}
static mut hevc_nal_type_name: [*const ::core::ffi::c_char; 64] = [
    b"TRAIL_N\0" as *const u8 as *const ::core::ffi::c_char,
    b"TRAIL_R\0" as *const u8 as *const ::core::ffi::c_char,
    b"TSA_N\0" as *const u8 as *const ::core::ffi::c_char,
    b"TSA_R\0" as *const u8 as *const ::core::ffi::c_char,
    b"STSA_N\0" as *const u8 as *const ::core::ffi::c_char,
    b"STSA_R\0" as *const u8 as *const ::core::ffi::c_char,
    b"RADL_N\0" as *const u8 as *const ::core::ffi::c_char,
    b"RADL_R\0" as *const u8 as *const ::core::ffi::c_char,
    b"RASL_N\0" as *const u8 as *const ::core::ffi::c_char,
    b"RASL_R\0" as *const u8 as *const ::core::ffi::c_char,
    b"RSV_VCL_N10\0" as *const u8 as *const ::core::ffi::c_char,
    b"RSV_VCL_R11\0" as *const u8 as *const ::core::ffi::c_char,
    b"RSV_VCL_N12\0" as *const u8 as *const ::core::ffi::c_char,
    b"RSV_VLC_R13\0" as *const u8 as *const ::core::ffi::c_char,
    b"RSV_VCL_N14\0" as *const u8 as *const ::core::ffi::c_char,
    b"RSV_VCL_R15\0" as *const u8 as *const ::core::ffi::c_char,
    b"BLA_W_LP\0" as *const u8 as *const ::core::ffi::c_char,
    b"BLA_W_RADL\0" as *const u8 as *const ::core::ffi::c_char,
    b"BLA_N_LP\0" as *const u8 as *const ::core::ffi::c_char,
    b"IDR_W_RADL\0" as *const u8 as *const ::core::ffi::c_char,
    b"IDR_N_LP\0" as *const u8 as *const ::core::ffi::c_char,
    b"CRA_NUT\0" as *const u8 as *const ::core::ffi::c_char,
    b"RSV_IRAP_VCL22\0" as *const u8 as *const ::core::ffi::c_char,
    b"RSV_IRAP_VCL23\0" as *const u8 as *const ::core::ffi::c_char,
    b"RSV_VCL24\0" as *const u8 as *const ::core::ffi::c_char,
    b"RSV_VCL25\0" as *const u8 as *const ::core::ffi::c_char,
    b"RSV_VCL26\0" as *const u8 as *const ::core::ffi::c_char,
    b"RSV_VCL27\0" as *const u8 as *const ::core::ffi::c_char,
    b"RSV_VCL28\0" as *const u8 as *const ::core::ffi::c_char,
    b"RSV_VCL29\0" as *const u8 as *const ::core::ffi::c_char,
    b"RSV_VCL30\0" as *const u8 as *const ::core::ffi::c_char,
    b"RSV_VCL31\0" as *const u8 as *const ::core::ffi::c_char,
    b"VPS\0" as *const u8 as *const ::core::ffi::c_char,
    b"SPS\0" as *const u8 as *const ::core::ffi::c_char,
    b"PPS\0" as *const u8 as *const ::core::ffi::c_char,
    b"AUD\0" as *const u8 as *const ::core::ffi::c_char,
    b"EOS_NUT\0" as *const u8 as *const ::core::ffi::c_char,
    b"EOB_NUT\0" as *const u8 as *const ::core::ffi::c_char,
    b"FD_NUT\0" as *const u8 as *const ::core::ffi::c_char,
    b"SEI_PREFIX\0" as *const u8 as *const ::core::ffi::c_char,
    b"SEI_SUFFIX\0" as *const u8 as *const ::core::ffi::c_char,
    b"RSV_NVCL41\0" as *const u8 as *const ::core::ffi::c_char,
    b"RSV_NVCL42\0" as *const u8 as *const ::core::ffi::c_char,
    b"RSV_NVCL43\0" as *const u8 as *const ::core::ffi::c_char,
    b"RSV_NVCL44\0" as *const u8 as *const ::core::ffi::c_char,
    b"RSV_NVCL45\0" as *const u8 as *const ::core::ffi::c_char,
    b"RSV_NVCL46\0" as *const u8 as *const ::core::ffi::c_char,
    b"RSV_NVCL47\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC48\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC49\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC50\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC51\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC52\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC53\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC54\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC55\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC56\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC57\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC58\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC59\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC60\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC61\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC62\0" as *const u8 as *const ::core::ffi::c_char,
    b"UNSPEC63\0" as *const u8 as *const ::core::ffi::c_char,
];
unsafe extern "C" fn hevc_nal_unit_name(
    mut nal_type: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    if !(nal_type >= 0 as ::core::ffi::c_int && nal_type < 64 as ::core::ffi::c_int) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"nal_type >= 0 && nal_type < 64\0" as *const u8
                as *const ::core::ffi::c_char,
            b"src/libavcodec/h2645_parse.c\0" as *const u8 as *const ::core::ffi::c_char,
            303 as ::core::ffi::c_int,
        );
        abort();
    }
    return hevc_nal_type_name[nal_type as usize];
}
static mut h264_nal_type_name: [*const ::core::ffi::c_char; 32] = [
    b"Unspecified 0\0" as *const u8 as *const ::core::ffi::c_char,
    b"Coded slice of a non-IDR picture\0" as *const u8 as *const ::core::ffi::c_char,
    b"Coded slice data partition A\0" as *const u8 as *const ::core::ffi::c_char,
    b"Coded slice data partition B\0" as *const u8 as *const ::core::ffi::c_char,
    b"Coded slice data partition C\0" as *const u8 as *const ::core::ffi::c_char,
    b"IDR\0" as *const u8 as *const ::core::ffi::c_char,
    b"SEI\0" as *const u8 as *const ::core::ffi::c_char,
    b"SPS\0" as *const u8 as *const ::core::ffi::c_char,
    b"PPS\0" as *const u8 as *const ::core::ffi::c_char,
    b"AUD\0" as *const u8 as *const ::core::ffi::c_char,
    b"End of sequence\0" as *const u8 as *const ::core::ffi::c_char,
    b"End of stream\0" as *const u8 as *const ::core::ffi::c_char,
    b"Filler data\0" as *const u8 as *const ::core::ffi::c_char,
    b"SPS extension\0" as *const u8 as *const ::core::ffi::c_char,
    b"Prefix\0" as *const u8 as *const ::core::ffi::c_char,
    b"Subset SPS\0" as *const u8 as *const ::core::ffi::c_char,
    b"Depth parameter set\0" as *const u8 as *const ::core::ffi::c_char,
    b"Reserved 17\0" as *const u8 as *const ::core::ffi::c_char,
    b"Reserved 18\0" as *const u8 as *const ::core::ffi::c_char,
    b"Auxiliary coded picture without partitioning\0" as *const u8
        as *const ::core::ffi::c_char,
    b"Slice extension\0" as *const u8 as *const ::core::ffi::c_char,
    b"Slice extension for a depth view or a 3D-AVC texture view\0" as *const u8
        as *const ::core::ffi::c_char,
    b"Reserved 22\0" as *const u8 as *const ::core::ffi::c_char,
    b"Reserved 23\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unspecified 24\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unspecified 25\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unspecified 26\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unspecified 27\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unspecified 28\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unspecified 29\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unspecified 30\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unspecified 31\0" as *const u8 as *const ::core::ffi::c_char,
];
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
unsafe extern "C" fn h264_nal_unit_name(
    mut nal_type: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    if !(nal_type >= 0 as ::core::ffi::c_int && nal_type < 32 as ::core::ffi::c_int) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"nal_type >= 0 && nal_type < 32\0" as *const u8
                as *const ::core::ffi::c_char,
            b"src/libavcodec/h2645_parse.c\0" as *const u8 as *const ::core::ffi::c_char,
            344 as ::core::ffi::c_int,
        );
        abort();
    }
    return h264_nal_type_name[nal_type as usize];
}
unsafe extern "C" fn get_bit_length(
    mut nal: *mut H2645NAL,
    mut min_size: ::core::ffi::c_int,
    mut skip_trailing_zeros: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut size: ::core::ffi::c_int = (*nal).size;
    let mut trailing_padding: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while skip_trailing_zeros != 0 && size > 0 as ::core::ffi::c_int
        && *(*nal).data.offset((size - 1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        size -= 1;
    }
    if size == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if size <= min_size {
        if (*nal).size < min_size {
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        size = min_size;
    } else {
        let mut v: ::core::ffi::c_int = *(*nal)
            .data
            .offset((size - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int;
        if v != 0 {
            trailing_padding = ff_ctz_c(v) + 1 as ::core::ffi::c_int;
        }
    }
    if size > 2147483647 as ::core::ffi::c_int / 8 as ::core::ffi::c_int {
        return -(34 as ::core::ffi::c_int);
    }
    size *= 8 as ::core::ffi::c_int;
    return size - trailing_padding;
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
unsafe extern "C" fn lcevc_parse_nal_header(
    mut nal: *mut H2645NAL,
    mut logctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut gb: *mut GetBitContext = &raw mut (*nal).gb;
    if get_bits1(gb) != 0 as ::core::ffi::c_uint {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if get_bits1(gb) != 1 as ::core::ffi::c_uint {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    (*nal).type_0 = get_bits(gb, 5 as ::core::ffi::c_int) as ::core::ffi::c_int;
    av_log(
        logctx,
        48 as ::core::ffi::c_int,
        b"nal_unit_type: %d(%s)\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*nal).type_0,
        lcevc_nal_unit_name((*nal).type_0),
    );
    return 0 as ::core::ffi::c_int;
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
unsafe extern "C" fn vvc_parse_nal_header(
    mut nal: *mut H2645NAL,
    mut logctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut gb: *mut GetBitContext = &raw mut (*nal).gb;
    if get_bits1(gb) != 0 as ::core::ffi::c_uint {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    skip_bits1(gb);
    (*nal).nuh_layer_id = get_bits(gb, 6 as ::core::ffi::c_int) as ::core::ffi::c_int;
    (*nal).type_0 = get_bits(gb, 5 as ::core::ffi::c_int) as ::core::ffi::c_int;
    (*nal).temporal_id = get_bits(gb, 3 as ::core::ffi::c_int)
        .wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    if (*nal).temporal_id < 0 as ::core::ffi::c_int {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if (*nal).type_0 >= VVC_IDR_W_RADL as ::core::ffi::c_int
        && (*nal).type_0 <= VVC_RSV_IRAP_11 as ::core::ffi::c_int
        && (*nal).temporal_id != 0
    {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    av_log(
        logctx,
        48 as ::core::ffi::c_int,
        b"nal_unit_type: %d(%s), nuh_layer_id: %d, temporal_id: %d\n\0" as *const u8
            as *const ::core::ffi::c_char,
        (*nal).type_0,
        vvc_nal_unit_name((*nal).type_0),
        (*nal).nuh_layer_id,
        (*nal).temporal_id,
    );
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn skip_bits1(mut s: *mut GetBitContext) {
    skip_bits(s, 1 as ::core::ffi::c_int);
}
unsafe extern "C" fn hevc_parse_nal_header(
    mut nal: *mut H2645NAL,
    mut logctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut gb: *mut GetBitContext = &raw mut (*nal).gb;
    if get_bits1(gb) != 0 as ::core::ffi::c_uint {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    (*nal).type_0 = get_bits(gb, 6 as ::core::ffi::c_int) as ::core::ffi::c_int;
    (*nal).nuh_layer_id = get_bits(gb, 6 as ::core::ffi::c_int) as ::core::ffi::c_int;
    (*nal).temporal_id = get_bits(gb, 3 as ::core::ffi::c_int)
        .wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    if (*nal).temporal_id < 0 as ::core::ffi::c_int {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    av_log(
        logctx,
        48 as ::core::ffi::c_int,
        b"nal_unit_type: %d(%s), nuh_layer_id: %d, temporal_id: %d\n\0" as *const u8
            as *const ::core::ffi::c_char,
        (*nal).type_0,
        hevc_nal_unit_name((*nal).type_0),
        (*nal).nuh_layer_id,
        (*nal).temporal_id,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn h264_parse_nal_header(
    mut nal: *mut H2645NAL,
    mut logctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut gb: *mut GetBitContext = &raw mut (*nal).gb;
    if get_bits1(gb) != 0 as ::core::ffi::c_uint {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    (*nal).ref_idc = get_bits(gb, 2 as ::core::ffi::c_int) as ::core::ffi::c_int;
    (*nal).type_0 = get_bits(gb, 5 as ::core::ffi::c_int) as ::core::ffi::c_int;
    av_log(
        logctx,
        48 as ::core::ffi::c_int,
        b"nal_unit_type: %d(%s), nal_ref_idc: %d\n\0" as *const u8
            as *const ::core::ffi::c_char,
        (*nal).type_0,
        h264_nal_unit_name((*nal).type_0),
        (*nal).ref_idc,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn find_next_start_code(
    mut buf: *const uint8_t,
    mut next_avc: *const uint8_t,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if buf.offset(3 as ::core::ffi::c_int as isize) >= next_avc {
        return next_avc.offset_from(buf) as ::core::ffi::c_int;
    }
    while buf.offset(i as isize).offset(3 as ::core::ffi::c_int as isize) < next_avc {
        if *buf.offset(i as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && *buf.offset((i + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            && *buf.offset((i + 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
        {
            break;
        }
        i += 1;
    }
    return i + 3 as ::core::ffi::c_int;
}
unsafe extern "C" fn alloc_rbsp_buffer(
    mut rbsp: *mut H2645RBSP,
    mut size: ::core::ffi::c_uint,
    mut use_ref: ::core::ffi::c_int,
) {
    let mut current_block: u64;
    let mut min_size: ::core::ffi::c_int = size as ::core::ffi::c_int;
    if !(size
        > (2147483647 as ::core::ffi::c_int - 64 as ::core::ffi::c_int)
            as ::core::ffi::c_uint)
    {
        size = size.wrapping_add(64 as ::core::ffi::c_uint);
        if (*rbsp).rbsp_buffer_alloc_size as ::core::ffi::c_uint >= size
            && ((*rbsp).rbsp_buffer_ref.is_null()
                || av_buffer_is_writable((*rbsp).rbsp_buffer_ref) != 0)
        {
            if (*rbsp).rbsp_buffer.is_null() {
                av_log(
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    0 as ::core::ffi::c_int,
                    b"Assertion %s failed at %s:%d\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"rbsp->rbsp_buffer\0" as *const u8 as *const ::core::ffi::c_char,
                    b"src/libavcodec/h2645_parse.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    490 as ::core::ffi::c_int,
                );
                abort();
            }
            memset(
                (*rbsp).rbsp_buffer.offset(min_size as isize)
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                64 as size_t,
            );
            return;
        }
        size = if size
            .wrapping_add(size.wrapping_div(16 as ::core::ffi::c_uint))
            .wrapping_add(32 as ::core::ffi::c_uint)
            > 2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint
        } else {
            size.wrapping_add(size.wrapping_div(16 as ::core::ffi::c_uint))
                .wrapping_add(32 as ::core::ffi::c_uint)
        };
        if !(*rbsp).rbsp_buffer_ref.is_null() {
            av_buffer_unref(&raw mut (*rbsp).rbsp_buffer_ref);
        } else {
            av_free((*rbsp).rbsp_buffer as *mut ::core::ffi::c_void);
        }
        (*rbsp).rbsp_buffer = av_mallocz(size as size_t) as *mut uint8_t;
        if !(*rbsp).rbsp_buffer.is_null() {
            (*rbsp).rbsp_buffer_alloc_size = size as ::core::ffi::c_int;
            if use_ref != 0 {
                (*rbsp).rbsp_buffer_ref = av_buffer_create(
                    (*rbsp).rbsp_buffer,
                    size as size_t,
                    None,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    0 as ::core::ffi::c_int,
                );
                if (*rbsp).rbsp_buffer_ref.is_null() {
                    current_block = 6615848536380219125;
                } else {
                    current_block = 4956146061682418353;
                }
            } else {
                current_block = 4956146061682418353;
            }
            match current_block {
                6615848536380219125 => {}
                _ => return,
            }
        }
    }
    (*rbsp).rbsp_buffer_alloc_size = 0 as ::core::ffi::c_int;
    if !(*rbsp).rbsp_buffer_ref.is_null() {
        av_buffer_unref(&raw mut (*rbsp).rbsp_buffer_ref);
        (*rbsp).rbsp_buffer = ::core::ptr::null_mut::<uint8_t>();
    } else {
        av_freep(&raw mut (*rbsp).rbsp_buffer as *mut ::core::ffi::c_void);
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
#[no_mangle]
pub unsafe extern "C" fn ff_h2645_packet_split(
    mut pkt: *mut H2645Packet,
    mut buf: *const uint8_t,
    mut length: ::core::ffi::c_int,
    mut logctx: *mut ::core::ffi::c_void,
    mut nal_length_size: ::core::ffi::c_int,
    mut codec_id: AVCodecID,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut bc: GetByteContext = GetByteContext {
        buffer: ::core::ptr::null::<uint8_t>(),
        buffer_end: ::core::ptr::null::<uint8_t>(),
        buffer_start: ::core::ptr::null::<uint8_t>(),
    };
    let mut consumed: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut next_avc: ::core::ffi::c_int = if flags
        & H2645_FLAG_IS_NALFF as ::core::ffi::c_int != 0
    {
        0 as ::core::ffi::c_int
    } else {
        length
    };
    let mut padding: int64_t = (if flags & H2645_FLAG_SMALL_PADDING as ::core::ffi::c_int
        != 0
    {
        0 as ::core::ffi::c_int
    } else {
        256 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int
    }) as int64_t;
    bytestream2_init(&raw mut bc, buf, length);
    alloc_rbsp_buffer(
        &raw mut (*pkt).rbsp,
        (length as int64_t + padding) as ::core::ffi::c_uint,
        (flags & H2645_FLAG_USE_REF as ::core::ffi::c_int != 0) as ::core::ffi::c_int,
    );
    if (*pkt).rbsp.rbsp_buffer.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    (*pkt).rbsp.rbsp_buffer_size = 0 as ::core::ffi::c_int;
    (*pkt).nb_nals = 0 as ::core::ffi::c_int;
    while bytestream2_get_bytes_left(&raw mut bc) >= 4 as ::core::ffi::c_int {
        let mut nal: *mut H2645NAL = ::core::ptr::null_mut::<H2645NAL>();
        let mut extract_length: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut skip_trailing_zeros: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        if bytestream2_tell(&raw mut bc) == next_avc {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            extract_length = get_nalsize(
                nal_length_size,
                bc.buffer,
                bytestream2_get_bytes_left(&raw mut bc),
                &raw mut i,
                logctx,
            );
            if extract_length < 0 as ::core::ffi::c_int {
                return extract_length;
            }
            bytestream2_skip(&raw mut bc, nal_length_size as ::core::ffi::c_uint);
            next_avc = bytestream2_tell(&raw mut bc) + extract_length;
        } else {
            let mut buf_index: ::core::ffi::c_int = 0;
            if bytestream2_tell(&raw mut bc) > next_avc {
                av_log(
                    logctx,
                    24 as ::core::ffi::c_int,
                    b"Exceeded next NALFF position, re-syncing.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            buf_index = find_next_start_code(bc.buffer, buf.offset(next_avc as isize));
            bytestream2_skip(&raw mut bc, buf_index as ::core::ffi::c_uint);
            if bytestream2_get_bytes_left(&raw mut bc) == 0 {
                if (*pkt).nb_nals > 0 as ::core::ffi::c_int {
                    return 0 as ::core::ffi::c_int
                } else {
                    av_log(
                        logctx,
                        16 as ::core::ffi::c_int,
                        b"No start code is found.\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                        | ('D' as i32) << 16 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint
                        | ('A' as i32 as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                }
            }
            extract_length = if bytestream2_get_bytes_left(&raw mut bc)
                > next_avc - bytestream2_tell(&raw mut bc)
            {
                next_avc - bytestream2_tell(&raw mut bc)
            } else {
                bytestream2_get_bytes_left(&raw mut bc)
            };
            if bytestream2_tell(&raw mut bc) >= next_avc {
                bytestream2_skip(
                    &raw mut bc,
                    (next_avc - bytestream2_tell(&raw mut bc)) as ::core::ffi::c_uint,
                );
                continue;
            }
        }
        if (*pkt).nals_allocated < (*pkt).nb_nals + 1 as ::core::ffi::c_int {
            let mut new_size: ::core::ffi::c_int = (*pkt).nals_allocated
                + 1 as ::core::ffi::c_int;
            let mut tmp: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
                ::core::ffi::c_void,
            >();
            if new_size as usize
                >= (2147483647 as ::core::ffi::c_int as usize)
                    .wrapping_div(::core::mem::size_of::<H2645NAL>() as usize)
            {
                return -(12 as ::core::ffi::c_int);
            }
            tmp = av_fast_realloc(
                (*pkt).nals as *mut ::core::ffi::c_void,
                &raw mut (*pkt).nal_buffer_size,
                (new_size as size_t)
                    .wrapping_mul(::core::mem::size_of::<H2645NAL>() as size_t),
            );
            if tmp.is_null() {
                return -(12 as ::core::ffi::c_int);
            }
            (*pkt).nals = tmp as *mut H2645NAL;
            memset(
                (*pkt).nals.offset((*pkt).nals_allocated as isize)
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<H2645NAL>() as size_t,
            );
            nal = (*pkt).nals.offset((*pkt).nb_nals as isize) as *mut H2645NAL;
            (*nal).skipped_bytes_pos_size = if 1024 as ::core::ffi::c_int
                > extract_length / 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            {
                extract_length / 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            } else {
                1024 as ::core::ffi::c_int
            };
            (*nal).skipped_bytes_pos = av_malloc_array(
                (*nal).skipped_bytes_pos_size as size_t,
                ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
            ) as *mut ::core::ffi::c_int;
            if (*nal).skipped_bytes_pos.is_null() {
                return -(12 as ::core::ffi::c_int);
            }
            (*pkt).nals_allocated = new_size;
        }
        nal = (*pkt).nals.offset((*pkt).nb_nals as isize) as *mut H2645NAL;
        consumed = ff_h2645_extract_rbsp(
            bc.buffer,
            extract_length,
            &raw mut (*pkt).rbsp,
            nal,
            (flags & H2645_FLAG_SMALL_PADDING as ::core::ffi::c_int != 0)
                as ::core::ffi::c_int,
        );
        if consumed < 0 as ::core::ffi::c_int {
            return consumed;
        }
        if flags & H2645_FLAG_IS_NALFF as ::core::ffi::c_int != 0
            && extract_length != consumed && extract_length != 0
        {
            av_log(
                logctx,
                48 as ::core::ffi::c_int,
                b"NALFF: Consumed only %d bytes instead of %d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                consumed,
                extract_length,
            );
        }
        bytestream2_skip(&raw mut bc, consumed as ::core::ffi::c_uint);
        if bytestream2_get_bytes_left(&raw mut bc) >= 4 as ::core::ffi::c_int
            && bytestream2_peek_be32(&raw mut bc) == 0x1e0 as ::core::ffi::c_uint
        {
            skip_trailing_zeros = 0 as ::core::ffi::c_int;
        }
        (*nal).size_bits = get_bit_length(
            nal,
            1 as ::core::ffi::c_int
                + (codec_id as ::core::ffi::c_uint
                    == AV_CODEC_ID_HEVC as ::core::ffi::c_int as ::core::ffi::c_uint)
                    as ::core::ffi::c_int,
            skip_trailing_zeros,
        );
        if (*nal).size <= 0 as ::core::ffi::c_int
            || (*nal).size_bits <= 0 as ::core::ffi::c_int
        {
            continue;
        }
        ret = init_get_bits(&raw mut (*nal).gb, (*nal).data, (*nal).size_bits);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        (*nal).type_0 = 0 as ::core::ffi::c_int;
        if codec_id as ::core::ffi::c_uint
            == AV_CODEC_ID_VVC as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            ret = vvc_parse_nal_header(nal, logctx);
        } else if codec_id as ::core::ffi::c_uint
            == AV_CODEC_ID_LCEVC as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            ret = lcevc_parse_nal_header(nal, logctx);
        } else if codec_id as ::core::ffi::c_uint
            == AV_CODEC_ID_HEVC as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            ret = hevc_parse_nal_header(nal, logctx);
            if (*nal).nuh_layer_id == 63 as ::core::ffi::c_int {
                continue;
            }
        } else {
            ret = h264_parse_nal_header(nal, logctx);
        }
        if ret < 0 as ::core::ffi::c_int {
            let mut fresh10: [::core::ffi::c_char; 64] = [
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
                logctx,
                24 as ::core::ffi::c_int,
                b"Failed to parse header of NALU (type %d): \"%s\". Skipping NALU.\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                (*nal).type_0,
                av_make_error_string(
                    &raw mut fresh10 as *mut ::core::ffi::c_char,
                    64 as size_t,
                    ret,
                ),
            );
        } else {
            (*pkt).nb_nals += 1;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_h2645_unit_requires_zero_byte(
    mut codec_id: AVCodecID,
    mut type_0: ::core::ffi::c_uint,
    mut nal_unit_index: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if nal_unit_index == 0 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    if codec_id as ::core::ffi::c_uint
        == AV_CODEC_ID_H264 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (type_0 == H264_NAL_SPS as ::core::ffi::c_int as ::core::ffi::c_uint
            || type_0 == H264_NAL_PPS as ::core::ffi::c_int as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
    }
    if codec_id as ::core::ffi::c_uint
        == AV_CODEC_ID_HEVC as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (type_0 == HEVC_NAL_VPS as ::core::ffi::c_int as ::core::ffi::c_uint
            || type_0 == HEVC_NAL_SPS as ::core::ffi::c_int as ::core::ffi::c_uint
            || type_0 == HEVC_NAL_PPS as ::core::ffi::c_int as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
    }
    if codec_id as ::core::ffi::c_uint
        == AV_CODEC_ID_VVC as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (type_0 >= VVC_OPI_NUT as ::core::ffi::c_int as ::core::ffi::c_uint
            && type_0 <= VVC_SUFFIX_APS_NUT as ::core::ffi::c_int as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_h2645_packet_uninit(mut pkt: *mut H2645Packet) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*pkt).nals_allocated {
        av_freep(
            &raw mut (*(*pkt).nals.offset(i as isize)).skipped_bytes_pos
                as *mut ::core::ffi::c_void,
        );
        i += 1;
    }
    av_freep(&raw mut (*pkt).nals as *mut ::core::ffi::c_void);
    (*pkt).nal_buffer_size = 0 as ::core::ffi::c_uint;
    (*pkt).nals_allocated = (*pkt).nal_buffer_size as ::core::ffi::c_int;
    if !(*pkt).rbsp.rbsp_buffer_ref.is_null() {
        av_buffer_unref(&raw mut (*pkt).rbsp.rbsp_buffer_ref);
        (*pkt).rbsp.rbsp_buffer = ::core::ptr::null_mut::<uint8_t>();
    } else {
        av_freep(&raw mut (*pkt).rbsp.rbsp_buffer as *mut ::core::ffi::c_void);
    }
    (*pkt).rbsp.rbsp_buffer_size = 0 as ::core::ffi::c_int;
    (*pkt).rbsp.rbsp_buffer_alloc_size = (*pkt).rbsp.rbsp_buffer_size;
}
#[repr(C)]
pub struct AVBuffer {
    _opaque: [u8; 0],
}
