// Translated from FFmpeg 9.0.1 libavcodec/profiles.c
// (preprocessed for aarch64-unknown-linux-musl) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVProfile {
    pub profile: ::core::ffi::c_int,
    pub name: *const ::core::ffi::c_char,
}
#[no_mangle]
pub static mut ff_aac_profiles: [AVProfile; 10] = [
    AVProfile {
        profile: 1 as ::core::ffi::c_int,
        name: b"LC\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 4 as ::core::ffi::c_int,
        name: b"HE-AAC\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 28 as ::core::ffi::c_int,
        name: b"HE-AACv2\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 22 as ::core::ffi::c_int,
        name: b"LD\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 38 as ::core::ffi::c_int,
        name: b"ELD\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 0 as ::core::ffi::c_int,
        name: b"Main\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 2 as ::core::ffi::c_int,
        name: b"SSR\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 3 as ::core::ffi::c_int,
        name: b"LTP\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 41 as ::core::ffi::c_int,
        name: b"xHE-AAC\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: -(99 as ::core::ffi::c_int),
        name: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
#[no_mangle]
pub static mut ff_dca_profiles: [AVProfile; 9] = [
    AVProfile {
        profile: 20 as ::core::ffi::c_int,
        name: b"DTS\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 30 as ::core::ffi::c_int,
        name: b"DTS-ES\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 40 as ::core::ffi::c_int,
        name: b"DTS 96/24\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 50 as ::core::ffi::c_int,
        name: b"DTS-HD HRA\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 60 as ::core::ffi::c_int,
        name: b"DTS-HD MA\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 61 as ::core::ffi::c_int,
        name: b"DTS-HD MA + DTS:X\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 62 as ::core::ffi::c_int,
        name: b"DTS-HD MA + DTS:X IMAX\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 70 as ::core::ffi::c_int,
        name: b"DTS Express\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: -(99 as ::core::ffi::c_int),
        name: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
#[no_mangle]
pub static mut ff_eac3_profiles: [AVProfile; 2] = [
    AVProfile {
        profile: 30 as ::core::ffi::c_int,
        name: b"Dolby Digital Plus + Dolby Atmos\0" as *const u8
            as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: -(99 as ::core::ffi::c_int),
        name: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
#[no_mangle]
pub static mut ff_truehd_profiles: [AVProfile; 2] = [
    AVProfile {
        profile: 30 as ::core::ffi::c_int,
        name: b"Dolby TrueHD + Dolby Atmos\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: -(99 as ::core::ffi::c_int),
        name: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
#[no_mangle]
pub static mut ff_dnxhd_profiles: [AVProfile; 7] = [
    AVProfile {
        profile: 0 as ::core::ffi::c_int,
        name: b"DNXHD\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 1 as ::core::ffi::c_int,
        name: b"DNXHR LB\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 2 as ::core::ffi::c_int,
        name: b"DNXHR SQ\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 3 as ::core::ffi::c_int,
        name: b"DNXHR HQ\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 4 as ::core::ffi::c_int,
        name: b"DNXHR HQX\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 5 as ::core::ffi::c_int,
        name: b"DNXHR 444\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: -(99 as ::core::ffi::c_int),
        name: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
#[no_mangle]
pub static mut ff_h264_profiles: [AVProfile; 16] = [
    AVProfile {
        profile: 66 as ::core::ffi::c_int,
        name: b"Baseline\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 66 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int,
        name: b"Constrained Baseline\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 77 as ::core::ffi::c_int,
        name: b"Main\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 88 as ::core::ffi::c_int,
        name: b"Extended\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 100 as ::core::ffi::c_int,
        name: b"High\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 110 as ::core::ffi::c_int,
        name: b"High 10\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 110 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int,
        name: b"High 10 Intra\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 122 as ::core::ffi::c_int,
        name: b"High 4:2:2\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 122 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int,
        name: b"High 4:2:2 Intra\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 144 as ::core::ffi::c_int,
        name: b"High 4:4:4\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 244 as ::core::ffi::c_int,
        name: b"High 4:4:4 Predictive\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 244 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int,
        name: b"High 4:4:4 Intra\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 44 as ::core::ffi::c_int,
        name: b"CAVLC 4:4:4\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 118 as ::core::ffi::c_int,
        name: b"Multiview High\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 128 as ::core::ffi::c_int,
        name: b"Stereo High\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: -(99 as ::core::ffi::c_int),
        name: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
#[no_mangle]
pub static mut ff_vvc_profiles: [AVProfile; 3] = [
    AVProfile {
        profile: 1 as ::core::ffi::c_int,
        name: b"Main 10\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 33 as ::core::ffi::c_int,
        name: b"Main 10 4:4:4\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: -(99 as ::core::ffi::c_int),
        name: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
#[no_mangle]
pub static mut ff_hevc_profiles: [AVProfile; 7] = [
    AVProfile {
        profile: 1 as ::core::ffi::c_int,
        name: b"Main\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 2 as ::core::ffi::c_int,
        name: b"Main 10\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 3 as ::core::ffi::c_int,
        name: b"Main Still Picture\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 4 as ::core::ffi::c_int,
        name: b"Rext\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 6 as ::core::ffi::c_int,
        name: b"Multiview Main\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 9 as ::core::ffi::c_int,
        name: b"Scc\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: -(99 as ::core::ffi::c_int),
        name: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
#[no_mangle]
pub static mut ff_jpeg2000_profiles: [AVProfile; 6] = [
    AVProfile {
        profile: 1 as ::core::ffi::c_int,
        name: b"JPEG 2000 codestream restriction 0\0" as *const u8
            as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 2 as ::core::ffi::c_int,
        name: b"JPEG 2000 codestream restriction 1\0" as *const u8
            as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 32768 as ::core::ffi::c_int,
        name: b"JPEG 2000 no codestream restrictions\0" as *const u8
            as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 3 as ::core::ffi::c_int,
        name: b"JPEG 2000 digital cinema 2K\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 4 as ::core::ffi::c_int,
        name: b"JPEG 2000 digital cinema 4K\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: -(99 as ::core::ffi::c_int),
        name: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
#[no_mangle]
pub static mut ff_mpeg2_video_profiles: [AVProfile; 8] = [
    AVProfile {
        profile: 0 as ::core::ffi::c_int,
        name: b"4:2:2\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 1 as ::core::ffi::c_int,
        name: b"High\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 2 as ::core::ffi::c_int,
        name: b"Spatially Scalable\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 3 as ::core::ffi::c_int,
        name: b"SNR Scalable\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 4 as ::core::ffi::c_int,
        name: b"Main\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 5 as ::core::ffi::c_int,
        name: b"Simple\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: -(100 as ::core::ffi::c_int),
        name: b"Reserved\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: -(99 as ::core::ffi::c_int),
        name: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
#[no_mangle]
pub static mut ff_mpeg4_video_profiles: [AVProfile; 17] = [
    AVProfile {
        profile: 0 as ::core::ffi::c_int,
        name: b"Simple Profile\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 1 as ::core::ffi::c_int,
        name: b"Simple Scalable Profile\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 2 as ::core::ffi::c_int,
        name: b"Core Profile\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 3 as ::core::ffi::c_int,
        name: b"Main Profile\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 4 as ::core::ffi::c_int,
        name: b"N-bit Profile\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 5 as ::core::ffi::c_int,
        name: b"Scalable Texture Profile\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 6 as ::core::ffi::c_int,
        name: b"Simple Face Animation Profile\0" as *const u8
            as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 7 as ::core::ffi::c_int,
        name: b"Basic Animated Texture Profile\0" as *const u8
            as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 8 as ::core::ffi::c_int,
        name: b"Hybrid Profile\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 9 as ::core::ffi::c_int,
        name: b"Advanced Real Time Simple Profile\0" as *const u8
            as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 10 as ::core::ffi::c_int,
        name: b"Code Scalable Profile\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 11 as ::core::ffi::c_int,
        name: b"Advanced Coding Profile\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 12 as ::core::ffi::c_int,
        name: b"Advanced Core Profile\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 13 as ::core::ffi::c_int,
        name: b"Advanced Scalable Texture Profile\0" as *const u8
            as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 14 as ::core::ffi::c_int,
        name: b"Simple Studio Profile\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 15 as ::core::ffi::c_int,
        name: b"Advanced Simple Profile\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: -(99 as ::core::ffi::c_int),
        name: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
#[no_mangle]
pub static mut ff_vc1_profiles: [AVProfile; 5] = [
    AVProfile {
        profile: 0 as ::core::ffi::c_int,
        name: b"Simple\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 1 as ::core::ffi::c_int,
        name: b"Main\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 2 as ::core::ffi::c_int,
        name: b"Complex\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 3 as ::core::ffi::c_int,
        name: b"Advanced\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: -(99 as ::core::ffi::c_int),
        name: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
#[no_mangle]
pub static mut ff_vp9_profiles: [AVProfile; 5] = [
    AVProfile {
        profile: 0 as ::core::ffi::c_int,
        name: b"Profile 0\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 1 as ::core::ffi::c_int,
        name: b"Profile 1\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 2 as ::core::ffi::c_int,
        name: b"Profile 2\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 3 as ::core::ffi::c_int,
        name: b"Profile 3\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: -(99 as ::core::ffi::c_int),
        name: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
#[no_mangle]
pub static mut ff_av1_profiles: [AVProfile; 4] = [
    AVProfile {
        profile: 0 as ::core::ffi::c_int,
        name: b"Main\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 1 as ::core::ffi::c_int,
        name: b"High\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 2 as ::core::ffi::c_int,
        name: b"Professional\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: -(99 as ::core::ffi::c_int),
        name: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
#[no_mangle]
pub static mut ff_sbc_profiles: [AVProfile; 2] = [
    AVProfile {
        profile: 1 as ::core::ffi::c_int,
        name: b"mSBC\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: -(99 as ::core::ffi::c_int),
        name: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
#[no_mangle]
pub static mut ff_prores_profiles: [AVProfile; 7] = [
    AVProfile {
        profile: 0 as ::core::ffi::c_int,
        name: b"Proxy\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 1 as ::core::ffi::c_int,
        name: b"LT\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 2 as ::core::ffi::c_int,
        name: b"Standard\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 3 as ::core::ffi::c_int,
        name: b"HQ\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 4 as ::core::ffi::c_int,
        name: b"4444\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 5 as ::core::ffi::c_int,
        name: b"XQ\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: -(99 as ::core::ffi::c_int),
        name: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
#[no_mangle]
pub static mut ff_prores_raw_profiles: [AVProfile; 3] = [
    AVProfile {
        profile: 0 as ::core::ffi::c_int,
        name: b"RAW\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 1 as ::core::ffi::c_int,
        name: b"RAW HQ\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: -(99 as ::core::ffi::c_int),
        name: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
#[no_mangle]
pub static mut ff_mjpeg_profiles: [AVProfile; 6] = [
    AVProfile {
        profile: 0xc0 as ::core::ffi::c_int,
        name: b"Baseline\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 0xc1 as ::core::ffi::c_int,
        name: b"Sequential\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 0xc2 as ::core::ffi::c_int,
        name: b"Progressive\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 0xc3 as ::core::ffi::c_int,
        name: b"Lossless\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 0xf7 as ::core::ffi::c_int,
        name: b"JPEG LS\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: -(99 as ::core::ffi::c_int),
        name: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
#[no_mangle]
pub static mut ff_arib_caption_profiles: [AVProfile; 3] = [
    AVProfile {
        profile: 0 as ::core::ffi::c_int,
        name: b"Profile A\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 1 as ::core::ffi::c_int,
        name: b"Profile C\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: -(99 as ::core::ffi::c_int),
        name: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
#[no_mangle]
pub static mut ff_evc_profiles: [AVProfile; 3] = [
    AVProfile {
        profile: 0 as ::core::ffi::c_int,
        name: b"Baseline\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 1 as ::core::ffi::c_int,
        name: b"Main\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: -(99 as ::core::ffi::c_int),
        name: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
#[no_mangle]
pub static mut ff_apv_profiles: [AVProfile; 8] = [
    AVProfile {
        profile: 33 as ::core::ffi::c_int,
        name: b"422-10\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 44 as ::core::ffi::c_int,
        name: b"422-12\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 55 as ::core::ffi::c_int,
        name: b"444-10\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 66 as ::core::ffi::c_int,
        name: b"444-12\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 77 as ::core::ffi::c_int,
        name: b"4444-10\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 88 as ::core::ffi::c_int,
        name: b"4444-12\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: 99 as ::core::ffi::c_int,
        name: b"400-10\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVProfile {
        profile: -(99 as ::core::ffi::c_int),
        name: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
