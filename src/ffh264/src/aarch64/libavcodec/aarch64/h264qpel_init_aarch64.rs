// Translated from FFmpeg 9.0.1 libavcodec/aarch64/h264qpel_init_aarch64.c
// (preprocessed for aarch64-unknown-linux-musl) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// ARM NEON optimised DSP functions
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
    fn ff_put_h264_qpel16_mc00_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel16_mc10_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel16_mc20_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel16_mc30_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel16_mc01_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel16_mc11_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel16_mc21_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel16_mc31_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel16_mc02_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel16_mc12_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel16_mc22_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel16_mc32_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel16_mc03_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel16_mc13_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel16_mc23_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel16_mc33_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel8_mc00_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel8_mc10_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel8_mc20_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel8_mc30_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel8_mc01_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel8_mc11_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel8_mc21_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel8_mc31_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel8_mc02_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel8_mc12_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel8_mc22_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel8_mc32_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel8_mc03_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel8_mc13_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel8_mc23_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel8_mc33_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel16_mc00_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel16_mc10_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel16_mc20_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel16_mc30_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel16_mc01_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel16_mc11_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel16_mc21_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel16_mc31_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel16_mc02_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel16_mc12_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel16_mc22_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel16_mc32_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel16_mc03_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel16_mc13_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel16_mc23_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel16_mc33_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel8_mc00_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel8_mc10_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel8_mc20_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel8_mc30_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel8_mc01_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel8_mc11_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel8_mc21_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel8_mc31_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel8_mc02_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel8_mc12_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel8_mc22_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel8_mc32_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel8_mc03_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel8_mc13_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel8_mc23_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel8_mc33_neon(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel16_mc10_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel16_mc20_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel16_mc30_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel16_mc01_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel16_mc11_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel16_mc31_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel16_mc02_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel16_mc03_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel16_mc13_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel16_mc33_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel8_mc10_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel8_mc20_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel8_mc30_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel8_mc01_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel8_mc11_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel8_mc31_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn av_get_cpu_flags() -> ::core::ffi::c_int;
    fn ff_put_h264_qpel8_mc02_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel8_mc03_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel8_mc13_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_put_h264_qpel8_mc33_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel16_mc10_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel16_mc20_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel16_mc30_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel16_mc01_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel16_mc11_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel16_mc31_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel16_mc02_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel16_mc03_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel16_mc13_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel16_mc33_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel8_mc10_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel8_mc20_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel8_mc30_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel8_mc01_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel8_mc11_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel8_mc31_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel8_mc02_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel8_mc03_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel8_mc13_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
    fn ff_avg_h264_qpel8_mc33_neon_10(
        dst: *mut uint8_t,
        src: *const uint8_t,
        stride: ptrdiff_t,
    );
}
pub type uint8_t = ::core::ffi::c_uchar;
pub type ptrdiff_t = ::core::ffi::c_long;
pub type qpel_mc_func = Option<
    unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264QpelContext {
    pub put_h264_qpel_pixels_tab: [[qpel_mc_func; 16]; 3],
    pub avg_h264_qpel_pixels_tab: [[qpel_mc_func; 16]; 3],
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ff_h264qpel_init_aarch64(
    mut c: *mut H264QpelContext,
    mut bit_depth: ::core::ffi::c_int,
) {
    let mut cpu_flags: ::core::ffi::c_int = av_get_cpu_flags();
    if 1 as ::core::ffi::c_int != 0
        && cpu_flags & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0
        && bit_depth <= 8 as ::core::ffi::c_int
    {
        (*c)
            .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][0 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel16_mc00_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][1 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel16_mc10_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][2 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel16_mc20_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][3 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel16_mc30_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][4 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel16_mc01_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][5 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel16_mc11_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][6 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel16_mc21_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][7 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel16_mc31_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][8 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel16_mc02_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][9 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel16_mc12_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][10 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel16_mc22_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][11 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel16_mc32_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][12 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel16_mc03_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][13 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel16_mc13_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][14 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel16_mc23_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][15 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel16_mc33_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][0 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel8_mc00_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][1 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel8_mc10_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][2 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel8_mc20_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][3 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel8_mc30_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][4 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel8_mc01_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][5 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel8_mc11_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][6 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel8_mc21_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][7 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel8_mc31_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][8 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel8_mc02_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][9 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel8_mc12_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][10 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel8_mc22_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][11 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel8_mc32_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][12 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel8_mc03_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][13 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel8_mc13_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][14 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel8_mc23_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][15 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel8_mc33_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][0 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel16_mc00_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][1 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel16_mc10_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][2 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel16_mc20_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][3 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel16_mc30_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][4 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel16_mc01_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][5 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel16_mc11_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][6 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel16_mc21_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][7 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel16_mc31_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][8 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel16_mc02_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][9 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel16_mc12_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][10 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel16_mc22_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][11 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel16_mc32_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][12 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel16_mc03_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][13 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel16_mc13_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][14 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel16_mc23_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][15 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel16_mc33_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][0 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel8_mc00_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][1 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel8_mc10_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][2 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel8_mc20_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][3 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel8_mc30_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][4 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel8_mc01_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][5 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel8_mc11_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][6 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel8_mc21_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][7 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel8_mc31_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][8 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel8_mc02_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][9 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel8_mc12_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][10 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel8_mc22_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][11 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel8_mc32_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][12 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel8_mc03_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][13 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel8_mc13_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][14 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel8_mc23_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][15 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel8_mc33_neon
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
    } else if 1 as ::core::ffi::c_int != 0
        && cpu_flags & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0
        && bit_depth == 10 as ::core::ffi::c_int
    {
        (*c)
            .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][1 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel16_mc10_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][2 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel16_mc20_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][3 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel16_mc30_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][4 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel16_mc01_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][5 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel16_mc11_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][7 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel16_mc31_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][8 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel16_mc02_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][12 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel16_mc03_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][13 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel16_mc13_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][15 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel16_mc33_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][1 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel8_mc10_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][2 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel8_mc20_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][3 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel8_mc30_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][4 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel8_mc01_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][5 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel8_mc11_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][7 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel8_mc31_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][8 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel8_mc02_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][12 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel8_mc03_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][13 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel8_mc13_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][15 as ::core::ffi::c_int as usize] = Some(
            ff_put_h264_qpel8_mc33_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][1 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel16_mc10_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][2 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel16_mc20_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][3 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel16_mc30_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][4 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel16_mc01_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][5 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel16_mc11_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][7 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel16_mc31_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][8 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel16_mc02_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][12 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel16_mc03_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][13 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel16_mc13_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
            as usize][15 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel16_mc33_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][1 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel8_mc10_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][2 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel8_mc20_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][3 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel8_mc30_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][4 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel8_mc01_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][5 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel8_mc11_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][7 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel8_mc31_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][8 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel8_mc02_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][12 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel8_mc03_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][13 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel8_mc13_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
        (*c)
            .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
            as usize][15 as ::core::ffi::c_int as usize] = Some(
            ff_avg_h264_qpel8_mc33_neon_10
                as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
        ) as qpel_mc_func;
    }
}
