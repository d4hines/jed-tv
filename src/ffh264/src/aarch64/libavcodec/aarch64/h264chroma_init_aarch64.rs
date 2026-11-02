// Translated from FFmpeg 9.0.1 libavcodec/aarch64/h264chroma_init_aarch64.c
// (preprocessed for aarch64-unknown-linux-musl) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// ARM NEON optimised H.264 chroma functions
// Copyright (c) 2008 Mans Rullgard <mans@mansr.com>
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
    fn ff_put_h264_chroma_mc8_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
        h: ::core::ffi::c_int,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    );
    fn ff_put_h264_chroma_mc4_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
        h: ::core::ffi::c_int,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    );
    fn ff_put_h264_chroma_mc2_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
        h: ::core::ffi::c_int,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    );
    fn ff_avg_h264_chroma_mc8_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
        h: ::core::ffi::c_int,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    );
    fn ff_avg_h264_chroma_mc4_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
        h: ::core::ffi::c_int,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    );
    fn ff_avg_h264_chroma_mc2_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
        h: ::core::ffi::c_int,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    );
    fn av_get_cpu_flags() -> ::core::ffi::c_int;
}
pub type uint8_t = ::core::ffi::c_uchar;
pub type ptrdiff_t = ::core::ffi::c_long;
pub type h264_chroma_mc_func = Option<
    unsafe extern "C" fn(
        *mut uint8_t,
        *const uint8_t,
        ptrdiff_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264ChromaContext {
    pub put_h264_chroma_pixels_tab: [h264_chroma_mc_func; 4],
    pub avg_h264_chroma_pixels_tab: [h264_chroma_mc_func; 4],
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ff_h264chroma_init_aarch64(
    mut c: *mut H264ChromaContext,
    mut bit_depth: ::core::ffi::c_int,
) {
    let high_bit_depth: ::core::ffi::c_int = (bit_depth > 8 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    let mut cpu_flags: ::core::ffi::c_int = av_get_cpu_flags();
    if 1 as ::core::ffi::c_int != 0
        && cpu_flags & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0
        && high_bit_depth == 0
    {
        (*c).put_h264_chroma_pixels_tab[0 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_chroma_mc8_neon
                as unsafe extern "C" fn(
                    *mut uint8_t,
                    *const uint8_t,
                    ptrdiff_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        ) as h264_chroma_mc_func;
        (*c).put_h264_chroma_pixels_tab[1 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_chroma_mc4_neon
                as unsafe extern "C" fn(
                    *mut uint8_t,
                    *const uint8_t,
                    ptrdiff_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        ) as h264_chroma_mc_func;
        (*c).put_h264_chroma_pixels_tab[2 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_chroma_mc2_neon
                as unsafe extern "C" fn(
                    *mut uint8_t,
                    *const uint8_t,
                    ptrdiff_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        ) as h264_chroma_mc_func;
        (*c).avg_h264_chroma_pixels_tab[0 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_chroma_mc8_neon
                as unsafe extern "C" fn(
                    *mut uint8_t,
                    *const uint8_t,
                    ptrdiff_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        ) as h264_chroma_mc_func;
        (*c).avg_h264_chroma_pixels_tab[1 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_chroma_mc4_neon
                as unsafe extern "C" fn(
                    *mut uint8_t,
                    *const uint8_t,
                    ptrdiff_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        ) as h264_chroma_mc_func;
        (*c).avg_h264_chroma_pixels_tab[2 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_chroma_mc2_neon
                as unsafe extern "C" fn(
                    *mut uint8_t,
                    *const uint8_t,
                    ptrdiff_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        ) as h264_chroma_mc_func;
    }
}
