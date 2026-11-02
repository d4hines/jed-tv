// Translated from FFmpeg 9.0.1 libavcodec/h264qpel.c
// (preprocessed for aarch64-unknown-linux-musl) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// H.26L/H.264/AVC/JVT/14496-10/... encoder/decoder
// Copyright (c) 2003-2010 Michael Niedermayer <michaelni@gmx.at>
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
    fn ff_h264qpel_init_aarch64(c: *mut H264QpelContext, bit_depth: ::core::ffi::c_int);
}
pub type int16_t = ::core::ffi::c_short;
pub type int32_t = ::core::ffi::c_int;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint16_t = ::core::ffi::c_ushort;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint64_t = ::core::ffi::c_ulong;
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
#[inline]
unsafe extern "C" fn rnd_avg32(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    return ((a | b) as ::core::ffi::c_ulong)
        .wrapping_sub(
            ((a ^ b) as ::core::ffi::c_ulong
                & !(0x1 as ::core::ffi::c_ulong)
                    .wrapping_mul(0x1010101 as ::core::ffi::c_ulong))
                >> 1 as ::core::ffi::c_int,
        ) as uint32_t;
}



#[inline]
unsafe extern "C" fn copy_block4_8(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut dstStride: ::core::ffi::c_int,
    mut srcStride: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < h {
        (*(dst as *mut unaligned_32)).l = (*(src as *const unaligned_32)).l;
        dst = dst.offset(dstStride as isize);
        src = src.offset(srcStride as isize);
        i += 1;
    }
}

#[inline]
unsafe extern "C" fn rnd_avg64(mut a: uint64_t, mut b: uint64_t) -> uint64_t {
    return (a | b)
        .wrapping_sub(
            ((a ^ b) & !(0x1 as uint64_t).wrapping_mul(0x1000100010001 as uint64_t))
                >> 1 as ::core::ffi::c_int,
        );
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ff_h264qpel_init(
    mut c: *mut H264QpelContext,
    mut bit_depth: ::core::ffi::c_int,
) {
    {
            (*c)
                .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][0 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel16_mc00_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][1 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel16_mc10_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][2 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel16_mc20_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][3 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel16_mc30_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][4 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel16_mc01_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][5 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel16_mc11_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][6 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel16_mc21_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][7 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel16_mc31_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][8 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel16_mc02_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][9 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel16_mc12_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][10 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel16_mc22_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][11 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel16_mc32_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][12 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel16_mc03_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][13 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel16_mc13_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][14 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel16_mc23_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][15 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel16_mc33_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][0 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel8_mc00_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][1 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel8_mc10_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][2 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel8_mc20_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][3 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel8_mc30_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][4 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel8_mc01_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][5 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel8_mc11_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][6 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel8_mc21_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][7 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel8_mc31_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][8 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel8_mc02_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][9 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel8_mc12_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][10 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel8_mc22_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][11 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel8_mc32_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][12 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel8_mc03_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][13 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel8_mc13_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][14 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel8_mc23_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][15 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel8_mc33_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][0 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel4_mc00_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][1 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel4_mc10_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][2 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel4_mc20_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][3 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel4_mc30_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][4 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel4_mc01_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][5 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel4_mc11_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][6 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel4_mc21_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][7 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel4_mc31_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][8 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel4_mc02_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][9 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel4_mc12_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][10 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel4_mc22_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][11 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel4_mc32_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][12 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel4_mc03_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][13 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel4_mc13_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][14 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel4_mc23_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .put_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][15 as ::core::ffi::c_int as usize] = Some(
                put_h264_qpel4_mc33_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][0 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel16_mc00_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][1 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel16_mc10_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][2 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel16_mc20_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][3 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel16_mc30_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][4 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel16_mc01_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][5 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel16_mc11_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][6 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel16_mc21_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][7 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel16_mc31_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][8 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel16_mc02_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][9 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel16_mc12_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][10 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel16_mc22_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][11 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel16_mc32_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][12 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel16_mc03_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][13 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel16_mc13_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][14 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel16_mc23_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[0 as ::core::ffi::c_int
                as usize][15 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel16_mc33_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][0 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel8_mc00_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][1 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel8_mc10_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][2 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel8_mc20_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][3 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel8_mc30_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][4 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel8_mc01_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][5 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel8_mc11_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][6 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel8_mc21_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][7 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel8_mc31_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][8 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel8_mc02_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][9 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel8_mc12_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][10 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel8_mc22_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][11 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel8_mc32_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][12 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel8_mc03_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][13 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel8_mc13_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][14 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel8_mc23_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[1 as ::core::ffi::c_int
                as usize][15 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel8_mc33_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][0 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel4_mc00_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][1 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel4_mc10_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][2 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel4_mc20_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][3 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel4_mc30_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][4 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel4_mc01_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][5 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel4_mc11_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][6 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel4_mc21_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][7 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel4_mc31_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][8 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel4_mc02_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][9 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel4_mc12_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][10 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel4_mc22_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][11 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel4_mc32_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][12 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel4_mc03_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][13 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel4_mc13_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][14 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel4_mc23_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
            (*c)
                .avg_h264_qpel_pixels_tab[2 as ::core::ffi::c_int
                as usize][15 as ::core::ffi::c_int as usize] = Some(
                avg_h264_qpel4_mc33_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const uint8_t,
                        ptrdiff_t,
                    ) -> (),
            ) as qpel_mc_func;
        }
    ff_h264qpel_init_aarch64(c, bit_depth);
}




#[inline]
unsafe extern "C" fn copy_block8_8(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut dstStride: ::core::ffi::c_int,
    mut srcStride: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < h {
        (*(dst as *mut unaligned_32)).l = (*(src as *const unaligned_32)).l;
        (*(dst
            .offset(
                (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ) as *mut unaligned_32))
            .l = (*(src
            .offset(
                (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ) as *const unaligned_32))
            .l;
        dst = dst.offset(dstStride as isize);
        src = src.offset(srcStride as isize);
        i += 1;
    }
}




#[inline]
unsafe extern "C" fn copy_block16_8(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut dstStride: ::core::ffi::c_int,
    mut srcStride: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < h {
        (*(dst as *mut unaligned_32)).l = (*(src as *const unaligned_32)).l;
        (*(dst
            .offset(
                (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ) as *mut unaligned_32))
            .l = (*(src
            .offset(
                (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ) as *const unaligned_32))
            .l;
        (*(dst
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ) as *mut unaligned_32))
            .l = (*(src
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ) as *const unaligned_32))
            .l;
        (*(dst
            .offset(
                (12 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ) as *mut unaligned_32))
            .l = (*(src
            .offset(
                (12 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ) as *const unaligned_32))
            .l;
        dst = dst.offset(dstStride as isize);
        src = src.offset(srcStride as isize);
        i += 1;
    }
}
#[inline]
unsafe extern "C" fn avg_pixels4_8_c(
    mut block: *mut uint8_t,
    mut pixels: *const uint8_t,
    mut line_size: ptrdiff_t,
    mut h: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < h {
        *(block as *mut uint32_t) = rnd_avg32(
            *(block as *mut uint32_t),
            (*(pixels as *const unaligned_32)).l,
        );
        pixels = pixels.offset(line_size as isize);
        block = block.offset(line_size as isize);
        i += 1;
    }
}





#[inline]
unsafe extern "C" fn avg_pixels8_8_c(
    mut block: *mut uint8_t,
    mut pixels: *const uint8_t,
    mut line_size: ptrdiff_t,
    mut h: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < h {
        *(block as *mut uint32_t) = rnd_avg32(
            *(block as *mut uint32_t),
            (*(pixels as *const unaligned_32)).l,
        );
        *(block
            .offset(
                (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ) as *mut uint32_t) = rnd_avg32(
            *(block
                .offset(
                    (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                        as isize,
                ) as *mut uint32_t),
            (*(pixels
                .offset(
                    (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                        as isize,
                ) as *const unaligned_32))
                .l,
        );
        pixels = pixels.offset(line_size as isize);
        block = block.offset(line_size as isize);
        i += 1;
    }
}



unsafe extern "C" fn avg_pixels16_8_c(
    mut block: *mut uint8_t,
    mut pixels: *const uint8_t,
    mut line_size: ptrdiff_t,
    mut h: ::core::ffi::c_int,
) {
    avg_pixels8_8_c(block, pixels, line_size, h);
    avg_pixels8_8_c(
        block
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        pixels
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        line_size,
        h,
    );
}





#[inline]
unsafe extern "C" fn put_pixels4_8_c(
    mut block: *mut uint8_t,
    mut pixels: *const uint8_t,
    mut line_size: ptrdiff_t,
    mut h: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < h {
        *(block as *mut uint32_t) = (*(pixels as *const unaligned_32)).l;
        pixels = pixels.offset(line_size as isize);
        block = block.offset(line_size as isize);
        i += 1;
    }
}




#[inline]
unsafe extern "C" fn put_pixels8_8_c(
    mut block: *mut uint8_t,
    mut pixels: *const uint8_t,
    mut line_size: ptrdiff_t,
    mut h: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < h {
        *(block as *mut uint32_t) = (*(pixels as *const unaligned_32)).l;
        *(block
            .offset(
                (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ) as *mut uint32_t) = (*(pixels
            .offset(
                (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ) as *const unaligned_32))
            .l;
        pixels = pixels.offset(line_size as isize);
        block = block.offset(line_size as isize);
        i += 1;
    }
}



unsafe extern "C" fn put_pixels16_8_c(
    mut block: *mut uint8_t,
    mut pixels: *const uint8_t,
    mut line_size: ptrdiff_t,
    mut h: ::core::ffi::c_int,
) {
    put_pixels8_8_c(block, pixels, line_size, h);
    put_pixels8_8_c(
        block
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        pixels
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        line_size,
        h,
    );
}








#[inline]
unsafe extern "C" fn avg_pixels8_l2_8(
    mut dst: *mut uint8_t,
    mut src1: *const uint8_t,
    mut src2: *const uint8_t,
    mut dst_stride: ptrdiff_t,
    mut src_stride1: ptrdiff_t,
    mut src_stride2: ptrdiff_t,
    mut h: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < h {
        let mut a: uint32_t = 0;
        let mut b: uint32_t = 0;
        a = (*(src1.offset((i as ptrdiff_t * src_stride1) as isize) as *const uint8_t
            as *const unaligned_32))
            .l;
        b = (*(src2.offset((i as ptrdiff_t * src_stride2) as isize) as *const uint8_t
            as *const unaligned_32))
            .l;
        *(dst.offset((i as ptrdiff_t * dst_stride) as isize) as *mut uint8_t
            as *mut uint32_t) = rnd_avg32(
            *(dst.offset((i as ptrdiff_t * dst_stride) as isize) as *mut uint8_t
                as *mut uint32_t),
            rnd_avg32(a, b),
        );
        a = (*(src1
            .offset(
                (i as ptrdiff_t * src_stride1
                    + (4 as ::core::ffi::c_int
                        * ::core::mem::size_of::<uint8_t>() as ::core::ffi::c_int)
                        as ptrdiff_t) as isize,
            ) as *const uint8_t as *const unaligned_32))
            .l;
        b = (*(src2
            .offset(
                (i as ptrdiff_t * src_stride2
                    + (4 as ::core::ffi::c_int
                        * ::core::mem::size_of::<uint8_t>() as ::core::ffi::c_int)
                        as ptrdiff_t) as isize,
            ) as *const uint8_t as *const unaligned_32))
            .l;
        *(dst
            .offset(
                (i as ptrdiff_t * dst_stride
                    + (4 as ::core::ffi::c_int
                        * ::core::mem::size_of::<uint8_t>() as ::core::ffi::c_int)
                        as ptrdiff_t) as isize,
            ) as *mut uint8_t as *mut uint32_t) = rnd_avg32(
            *(dst
                .offset(
                    (i as ptrdiff_t * dst_stride
                        + (4 as ::core::ffi::c_int
                            * ::core::mem::size_of::<uint8_t>() as ::core::ffi::c_int)
                            as ptrdiff_t) as isize,
                ) as *mut uint8_t as *mut uint32_t),
            rnd_avg32(a, b),
        );
        i += 1;
    }
}
#[inline]
unsafe extern "C" fn avg_pixels4_l2_8(
    mut dst: *mut uint8_t,
    mut src1: *const uint8_t,
    mut src2: *const uint8_t,
    mut dst_stride: ptrdiff_t,
    mut src_stride1: ptrdiff_t,
    mut src_stride2: ptrdiff_t,
    mut h: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < h {
        let mut a: uint32_t = 0;
        let mut b: uint32_t = 0;
        a = (*(src1.offset((i as ptrdiff_t * src_stride1) as isize) as *const uint8_t
            as *const unaligned_32))
            .l;
        b = (*(src2.offset((i as ptrdiff_t * src_stride2) as isize) as *const uint8_t
            as *const unaligned_32))
            .l;
        *(dst.offset((i as ptrdiff_t * dst_stride) as isize) as *mut uint8_t
            as *mut uint32_t) = rnd_avg32(
            *(dst.offset((i as ptrdiff_t * dst_stride) as isize) as *mut uint8_t
                as *mut uint32_t),
            rnd_avg32(a, b),
        );
        i += 1;
    }
}




#[inline]
unsafe extern "C" fn avg_pixels16_l2_8(
    mut dst: *mut uint8_t,
    mut src1: *const uint8_t,
    mut src2: *const uint8_t,
    mut dst_stride: ptrdiff_t,
    mut src_stride1: ptrdiff_t,
    mut src_stride2: ptrdiff_t,
    mut h: ::core::ffi::c_int,
) {
    avg_pixels8_l2_8(dst, src1, src2, dst_stride, src_stride1, src_stride2, h);
    avg_pixels8_l2_8(
        dst
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        src1
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        src2
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        dst_stride,
        src_stride1,
        src_stride2,
        h,
    );
}






#[inline]
unsafe extern "C" fn put_pixels8_l2_8(
    mut dst: *mut uint8_t,
    mut src1: *const uint8_t,
    mut src2: *const uint8_t,
    mut dst_stride: ptrdiff_t,
    mut src_stride1: ptrdiff_t,
    mut src_stride2: ptrdiff_t,
    mut h: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < h {
        let mut a: uint32_t = 0;
        let mut b: uint32_t = 0;
        a = (*(src1.offset((i as ptrdiff_t * src_stride1) as isize) as *const uint8_t
            as *const unaligned_32))
            .l;
        b = (*(src2.offset((i as ptrdiff_t * src_stride2) as isize) as *const uint8_t
            as *const unaligned_32))
            .l;
        *(dst.offset((i as ptrdiff_t * dst_stride) as isize) as *mut uint8_t
            as *mut uint32_t) = rnd_avg32(a, b);
        a = (*(src1
            .offset(
                (i as ptrdiff_t * src_stride1
                    + (4 as ::core::ffi::c_int
                        * ::core::mem::size_of::<uint8_t>() as ::core::ffi::c_int)
                        as ptrdiff_t) as isize,
            ) as *const uint8_t as *const unaligned_32))
            .l;
        b = (*(src2
            .offset(
                (i as ptrdiff_t * src_stride2
                    + (4 as ::core::ffi::c_int
                        * ::core::mem::size_of::<uint8_t>() as ::core::ffi::c_int)
                        as ptrdiff_t) as isize,
            ) as *const uint8_t as *const unaligned_32))
            .l;
        *(dst
            .offset(
                (i as ptrdiff_t * dst_stride
                    + (4 as ::core::ffi::c_int
                        * ::core::mem::size_of::<uint8_t>() as ::core::ffi::c_int)
                        as ptrdiff_t) as isize,
            ) as *mut uint8_t as *mut uint32_t) = rnd_avg32(a, b);
        i += 1;
    }
}


#[inline]
unsafe extern "C" fn put_pixels4_l2_8(
    mut dst: *mut uint8_t,
    mut src1: *const uint8_t,
    mut src2: *const uint8_t,
    mut dst_stride: ptrdiff_t,
    mut src_stride1: ptrdiff_t,
    mut src_stride2: ptrdiff_t,
    mut h: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < h {
        let mut a: uint32_t = 0;
        let mut b: uint32_t = 0;
        a = (*(src1.offset((i as ptrdiff_t * src_stride1) as isize) as *const uint8_t
            as *const unaligned_32))
            .l;
        b = (*(src2.offset((i as ptrdiff_t * src_stride2) as isize) as *const uint8_t
            as *const unaligned_32))
            .l;
        *(dst.offset((i as ptrdiff_t * dst_stride) as isize) as *mut uint8_t
            as *mut uint32_t) = rnd_avg32(a, b);
        i += 1;
    }
}




#[inline]
unsafe extern "C" fn put_pixels16_l2_8(
    mut dst: *mut uint8_t,
    mut src1: *const uint8_t,
    mut src2: *const uint8_t,
    mut dst_stride: ptrdiff_t,
    mut src_stride1: ptrdiff_t,
    mut src_stride2: ptrdiff_t,
    mut h: ::core::ffi::c_int,
) {
    put_pixels8_l2_8(dst, src1, src2, dst_stride, src_stride1, src_stride2, h);
    put_pixels8_l2_8(
        dst
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        src1
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        src2
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        dst_stride,
        src_stride1,
        src_stride2,
        h,
    );
}




#[inline(always)]
unsafe extern "C" fn av_clip_uint8_c(mut a: ::core::ffi::c_int) -> uint8_t {
    if a & !(0xff as ::core::ffi::c_int) != 0 {
        return (!a >> 31 as ::core::ffi::c_int) as uint8_t
    } else {
        return a as uint8_t
    };
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


unsafe extern "C" fn put_h264_qpel4_h_lowpass_8(
    mut _dst: *mut uint8_t,
    mut _src: *const uint8_t,
    mut dstStride: ::core::ffi::c_int,
    mut srcStride: ::core::ffi::c_int,
) {
    let h: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut dst: *mut uint8_t = _dst;
    let mut src: *const uint8_t = _src;
    dstStride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    srcStride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    i = 0 as ::core::ffi::c_int;
    while i < h {
        *dst.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            (*src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 20 as ::core::ffi::c_int
                - (*src.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    + *src.offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) * 5 as ::core::ffi::c_int
                + (*src.offset(-(2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    + *src.offset(3 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) + 16 as ::core::ffi::c_int
                >> 5 as ::core::ffi::c_int,
        );
        *dst.offset(1 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            (*src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 20 as ::core::ffi::c_int
                - (*src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src.offset(3 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) * 5 as ::core::ffi::c_int
                + (*src.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    + *src.offset(4 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) + 16 as ::core::ffi::c_int
                >> 5 as ::core::ffi::c_int,
        );
        *dst.offset(2 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            (*src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 20 as ::core::ffi::c_int
                - (*src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src.offset(4 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) * 5 as ::core::ffi::c_int
                + (*src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src.offset(5 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) + 16 as ::core::ffi::c_int
                >> 5 as ::core::ffi::c_int,
        );
        *dst.offset(3 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            (*src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 20 as ::core::ffi::c_int
                - (*src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src.offset(5 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) * 5 as ::core::ffi::c_int
                + (*src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src.offset(6 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) + 16 as ::core::ffi::c_int
                >> 5 as ::core::ffi::c_int,
        );
        dst = dst.offset(dstStride as isize);
        src = src.offset(srcStride as isize);
        i += 1;
    }
}


unsafe extern "C" fn put_h264_qpel4_v_lowpass_8(
    mut _dst: *mut uint8_t,
    mut _src: *const uint8_t,
    mut dstStride: ::core::ffi::c_int,
    mut srcStride: ::core::ffi::c_int,
) {
    let w: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut dst: *mut uint8_t = _dst;
    let mut src: *const uint8_t = _src;
    dstStride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    srcStride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    i = 0 as ::core::ffi::c_int;
    while i < w {
        let srcB: ::core::ffi::c_int = *src
            .offset((-(2 as ::core::ffi::c_int) * srcStride) as isize)
            as ::core::ffi::c_int;
        let srcA: ::core::ffi::c_int = *src
            .offset((-(1 as ::core::ffi::c_int) * srcStride) as isize)
            as ::core::ffi::c_int;
        let src0: ::core::ffi::c_int = *src
            .offset((0 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src1: ::core::ffi::c_int = *src
            .offset((1 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src2: ::core::ffi::c_int = *src
            .offset((2 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src3: ::core::ffi::c_int = *src
            .offset((3 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src4: ::core::ffi::c_int = *src
            .offset((4 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src5: ::core::ffi::c_int = *src
            .offset((5 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src6: ::core::ffi::c_int = *src
            .offset((6 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        *dst.offset((0 as ::core::ffi::c_int * dstStride) as isize) = av_clip_uint8_c(
            (src0 + src1) * 20 as ::core::ffi::c_int
                - (srcA + src2) * 5 as ::core::ffi::c_int + (srcB + src3)
                + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int,
        );
        *dst.offset((1 as ::core::ffi::c_int * dstStride) as isize) = av_clip_uint8_c(
            (src1 + src2) * 20 as ::core::ffi::c_int
                - (src0 + src3) * 5 as ::core::ffi::c_int + (srcA + src4)
                + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int,
        );
        *dst.offset((2 as ::core::ffi::c_int * dstStride) as isize) = av_clip_uint8_c(
            (src2 + src3) * 20 as ::core::ffi::c_int
                - (src1 + src4) * 5 as ::core::ffi::c_int + (src0 + src5)
                + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int,
        );
        *dst.offset((3 as ::core::ffi::c_int * dstStride) as isize) = av_clip_uint8_c(
            (src3 + src4) * 20 as ::core::ffi::c_int
                - (src2 + src5) * 5 as ::core::ffi::c_int + (src1 + src6)
                + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int,
        );
        dst = dst.offset(1);
        src = src.offset(1);
        i += 1;
    }
}




unsafe extern "C" fn put_h264_qpel4_hv_lowpass_8(
    mut _dst: *mut uint8_t,
    mut tmp: *mut int16_t,
    mut _src: *const uint8_t,
    mut dstStride: ::core::ffi::c_int,
    mut tmpStride: ::core::ffi::c_int,
    mut srcStride: ::core::ffi::c_int,
) {
    let h: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    let w: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    let pad: ::core::ffi::c_int = if 8 as ::core::ffi::c_int == 10 as ::core::ffi::c_int
    {
        -(10 as ::core::ffi::c_int)
            * (((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int)
    } else {
        0 as ::core::ffi::c_int
    };
    let mut i: ::core::ffi::c_int = 0;
    let mut dst: *mut uint8_t = _dst;
    let mut src: *const uint8_t = _src;
    dstStride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    srcStride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    src = src.offset(-((2 as ::core::ffi::c_int * srcStride) as isize));
    i = 0 as ::core::ffi::c_int;
    while i < h + 5 as ::core::ffi::c_int {
        *tmp.offset(0 as ::core::ffi::c_int as isize) = ((*src
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            * 20 as ::core::ffi::c_int
            - (*src.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 5 as ::core::ffi::c_int
            + (*src.offset(-(2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            + pad) as int16_t;
        *tmp.offset(1 as ::core::ffi::c_int as isize) = ((*src
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            * 20 as ::core::ffi::c_int
            - (*src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 5 as ::core::ffi::c_int
            + (*src.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            + pad) as int16_t;
        *tmp.offset(2 as ::core::ffi::c_int as isize) = ((*src
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            * 20 as ::core::ffi::c_int
            - (*src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 5 as ::core::ffi::c_int
            + (*src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            + pad) as int16_t;
        *tmp.offset(3 as ::core::ffi::c_int as isize) = ((*src
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            * 20 as ::core::ffi::c_int
            - (*src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 5 as ::core::ffi::c_int
            + (*src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            + pad) as int16_t;
        tmp = tmp.offset(tmpStride as isize);
        src = src.offset(srcStride as isize);
        i += 1;
    }
    tmp = tmp
        .offset(
            -((tmpStride * (h + 5 as ::core::ffi::c_int - 2 as ::core::ffi::c_int))
                as isize),
        );
    i = 0 as ::core::ffi::c_int;
    while i < w {
        let tmpB: ::core::ffi::c_int = *tmp
            .offset((-(2 as ::core::ffi::c_int) * tmpStride) as isize)
            as ::core::ffi::c_int - pad;
        let tmpA: ::core::ffi::c_int = *tmp
            .offset((-(1 as ::core::ffi::c_int) * tmpStride) as isize)
            as ::core::ffi::c_int - pad;
        let tmp0: ::core::ffi::c_int = *tmp
            .offset((0 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp1: ::core::ffi::c_int = *tmp
            .offset((1 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp2: ::core::ffi::c_int = *tmp
            .offset((2 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp3: ::core::ffi::c_int = *tmp
            .offset((3 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp4: ::core::ffi::c_int = *tmp
            .offset((4 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp5: ::core::ffi::c_int = *tmp
            .offset((5 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp6: ::core::ffi::c_int = *tmp
            .offset((6 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        *dst.offset((0 as ::core::ffi::c_int * dstStride) as isize) = av_clip_uint8_c(
            (tmp0 + tmp1) * 20 as ::core::ffi::c_int
                - (tmpA + tmp2) * 5 as ::core::ffi::c_int + (tmpB + tmp3)
                + 512 as ::core::ffi::c_int >> 10 as ::core::ffi::c_int,
        );
        *dst.offset((1 as ::core::ffi::c_int * dstStride) as isize) = av_clip_uint8_c(
            (tmp1 + tmp2) * 20 as ::core::ffi::c_int
                - (tmp0 + tmp3) * 5 as ::core::ffi::c_int + (tmpA + tmp4)
                + 512 as ::core::ffi::c_int >> 10 as ::core::ffi::c_int,
        );
        *dst.offset((2 as ::core::ffi::c_int * dstStride) as isize) = av_clip_uint8_c(
            (tmp2 + tmp3) * 20 as ::core::ffi::c_int
                - (tmp1 + tmp4) * 5 as ::core::ffi::c_int + (tmp0 + tmp5)
                + 512 as ::core::ffi::c_int >> 10 as ::core::ffi::c_int,
        );
        *dst.offset((3 as ::core::ffi::c_int * dstStride) as isize) = av_clip_uint8_c(
            (tmp3 + tmp4) * 20 as ::core::ffi::c_int
                - (tmp2 + tmp5) * 5 as ::core::ffi::c_int + (tmp1 + tmp6)
                + 512 as ::core::ffi::c_int >> 10 as ::core::ffi::c_int,
        );
        dst = dst.offset(1);
        tmp = tmp.offset(1);
        i += 1;
    }
}




unsafe extern "C" fn put_h264_qpel8_h_lowpass_8(
    mut _dst: *mut uint8_t,
    mut _src: *const uint8_t,
    mut dstStride: ::core::ffi::c_int,
    mut srcStride: ::core::ffi::c_int,
) {
    let h: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut dst: *mut uint8_t = _dst;
    let mut src: *const uint8_t = _src;
    dstStride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    srcStride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    i = 0 as ::core::ffi::c_int;
    while i < h {
        *dst.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            (*src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 20 as ::core::ffi::c_int
                - (*src.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    + *src.offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) * 5 as ::core::ffi::c_int
                + (*src.offset(-(2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    + *src.offset(3 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) + 16 as ::core::ffi::c_int
                >> 5 as ::core::ffi::c_int,
        );
        *dst.offset(1 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            (*src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 20 as ::core::ffi::c_int
                - (*src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src.offset(3 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) * 5 as ::core::ffi::c_int
                + (*src.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    + *src.offset(4 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) + 16 as ::core::ffi::c_int
                >> 5 as ::core::ffi::c_int,
        );
        *dst.offset(2 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            (*src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 20 as ::core::ffi::c_int
                - (*src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src.offset(4 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) * 5 as ::core::ffi::c_int
                + (*src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src.offset(5 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) + 16 as ::core::ffi::c_int
                >> 5 as ::core::ffi::c_int,
        );
        *dst.offset(3 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            (*src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 20 as ::core::ffi::c_int
                - (*src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src.offset(5 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) * 5 as ::core::ffi::c_int
                + (*src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src.offset(6 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) + 16 as ::core::ffi::c_int
                >> 5 as ::core::ffi::c_int,
        );
        *dst.offset(4 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            (*src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 20 as ::core::ffi::c_int
                - (*src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src.offset(6 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) * 5 as ::core::ffi::c_int
                + (*src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src.offset(7 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) + 16 as ::core::ffi::c_int
                >> 5 as ::core::ffi::c_int,
        );
        *dst.offset(5 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            (*src.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 20 as ::core::ffi::c_int
                - (*src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src.offset(7 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) * 5 as ::core::ffi::c_int
                + (*src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src.offset(8 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) + 16 as ::core::ffi::c_int
                >> 5 as ::core::ffi::c_int,
        );
        *dst.offset(6 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            (*src.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 20 as ::core::ffi::c_int
                - (*src.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src.offset(8 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) * 5 as ::core::ffi::c_int
                + (*src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src.offset(9 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) + 16 as ::core::ffi::c_int
                >> 5 as ::core::ffi::c_int,
        );
        *dst.offset(7 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            (*src.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 20 as ::core::ffi::c_int
                - (*src.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src.offset(9 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) * 5 as ::core::ffi::c_int
                + (*src.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src.offset(10 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) + 16 as ::core::ffi::c_int
                >> 5 as ::core::ffi::c_int,
        );
        dst = dst.offset(dstStride as isize);
        src = src.offset(srcStride as isize);
        i += 1;
    }
}




unsafe extern "C" fn put_h264_qpel8_v_lowpass_8(
    mut _dst: *mut uint8_t,
    mut _src: *const uint8_t,
    mut dstStride: ::core::ffi::c_int,
    mut srcStride: ::core::ffi::c_int,
) {
    let w: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut dst: *mut uint8_t = _dst;
    let mut src: *const uint8_t = _src;
    dstStride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    srcStride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    i = 0 as ::core::ffi::c_int;
    while i < w {
        let srcB: ::core::ffi::c_int = *src
            .offset((-(2 as ::core::ffi::c_int) * srcStride) as isize)
            as ::core::ffi::c_int;
        let srcA: ::core::ffi::c_int = *src
            .offset((-(1 as ::core::ffi::c_int) * srcStride) as isize)
            as ::core::ffi::c_int;
        let src0: ::core::ffi::c_int = *src
            .offset((0 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src1: ::core::ffi::c_int = *src
            .offset((1 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src2: ::core::ffi::c_int = *src
            .offset((2 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src3: ::core::ffi::c_int = *src
            .offset((3 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src4: ::core::ffi::c_int = *src
            .offset((4 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src5: ::core::ffi::c_int = *src
            .offset((5 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src6: ::core::ffi::c_int = *src
            .offset((6 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src7: ::core::ffi::c_int = *src
            .offset((7 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src8: ::core::ffi::c_int = *src
            .offset((8 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src9: ::core::ffi::c_int = *src
            .offset((9 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src10: ::core::ffi::c_int = *src
            .offset((10 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        *dst.offset((0 as ::core::ffi::c_int * dstStride) as isize) = av_clip_uint8_c(
            (src0 + src1) * 20 as ::core::ffi::c_int
                - (srcA + src2) * 5 as ::core::ffi::c_int + (srcB + src3)
                + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int,
        );
        *dst.offset((1 as ::core::ffi::c_int * dstStride) as isize) = av_clip_uint8_c(
            (src1 + src2) * 20 as ::core::ffi::c_int
                - (src0 + src3) * 5 as ::core::ffi::c_int + (srcA + src4)
                + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int,
        );
        *dst.offset((2 as ::core::ffi::c_int * dstStride) as isize) = av_clip_uint8_c(
            (src2 + src3) * 20 as ::core::ffi::c_int
                - (src1 + src4) * 5 as ::core::ffi::c_int + (src0 + src5)
                + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int,
        );
        *dst.offset((3 as ::core::ffi::c_int * dstStride) as isize) = av_clip_uint8_c(
            (src3 + src4) * 20 as ::core::ffi::c_int
                - (src2 + src5) * 5 as ::core::ffi::c_int + (src1 + src6)
                + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int,
        );
        *dst.offset((4 as ::core::ffi::c_int * dstStride) as isize) = av_clip_uint8_c(
            (src4 + src5) * 20 as ::core::ffi::c_int
                - (src3 + src6) * 5 as ::core::ffi::c_int + (src2 + src7)
                + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int,
        );
        *dst.offset((5 as ::core::ffi::c_int * dstStride) as isize) = av_clip_uint8_c(
            (src5 + src6) * 20 as ::core::ffi::c_int
                - (src4 + src7) * 5 as ::core::ffi::c_int + (src3 + src8)
                + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int,
        );
        *dst.offset((6 as ::core::ffi::c_int * dstStride) as isize) = av_clip_uint8_c(
            (src6 + src7) * 20 as ::core::ffi::c_int
                - (src5 + src8) * 5 as ::core::ffi::c_int + (src4 + src9)
                + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int,
        );
        *dst.offset((7 as ::core::ffi::c_int * dstStride) as isize) = av_clip_uint8_c(
            (src7 + src8) * 20 as ::core::ffi::c_int
                - (src6 + src9) * 5 as ::core::ffi::c_int + (src5 + src10)
                + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int,
        );
        dst = dst.offset(1);
        src = src.offset(1);
        i += 1;
    }
}




unsafe extern "C" fn put_h264_qpel8_hv_lowpass_8(
    mut _dst: *mut uint8_t,
    mut tmp: *mut int16_t,
    mut _src: *const uint8_t,
    mut dstStride: ::core::ffi::c_int,
    mut tmpStride: ::core::ffi::c_int,
    mut srcStride: ::core::ffi::c_int,
) {
    let h: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    let w: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    let pad: ::core::ffi::c_int = if 8 as ::core::ffi::c_int == 10 as ::core::ffi::c_int
    {
        -(10 as ::core::ffi::c_int)
            * (((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int)
    } else {
        0 as ::core::ffi::c_int
    };
    let mut i: ::core::ffi::c_int = 0;
    let mut dst: *mut uint8_t = _dst;
    let mut src: *const uint8_t = _src;
    dstStride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    srcStride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    src = src.offset(-((2 as ::core::ffi::c_int * srcStride) as isize));
    i = 0 as ::core::ffi::c_int;
    while i < h + 5 as ::core::ffi::c_int {
        *tmp.offset(0 as ::core::ffi::c_int as isize) = ((*src
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            * 20 as ::core::ffi::c_int
            - (*src.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 5 as ::core::ffi::c_int
            + (*src.offset(-(2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            + pad) as int16_t;
        *tmp.offset(1 as ::core::ffi::c_int as isize) = ((*src
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            * 20 as ::core::ffi::c_int
            - (*src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 5 as ::core::ffi::c_int
            + (*src.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            + pad) as int16_t;
        *tmp.offset(2 as ::core::ffi::c_int as isize) = ((*src
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            * 20 as ::core::ffi::c_int
            - (*src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 5 as ::core::ffi::c_int
            + (*src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            + pad) as int16_t;
        *tmp.offset(3 as ::core::ffi::c_int as isize) = ((*src
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            * 20 as ::core::ffi::c_int
            - (*src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 5 as ::core::ffi::c_int
            + (*src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            + pad) as int16_t;
        *tmp.offset(4 as ::core::ffi::c_int as isize) = ((*src
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *src.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            * 20 as ::core::ffi::c_int
            - (*src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 5 as ::core::ffi::c_int
            + (*src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            + pad) as int16_t;
        *tmp.offset(5 as ::core::ffi::c_int as isize) = ((*src
            .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *src.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            * 20 as ::core::ffi::c_int
            - (*src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 5 as ::core::ffi::c_int
            + (*src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            + pad) as int16_t;
        *tmp.offset(6 as ::core::ffi::c_int as isize) = ((*src
            .offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *src.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            * 20 as ::core::ffi::c_int
            - (*src.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 5 as ::core::ffi::c_int
            + (*src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            + pad) as int16_t;
        *tmp.offset(7 as ::core::ffi::c_int as isize) = ((*src
            .offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *src.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            * 20 as ::core::ffi::c_int
            - (*src.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 5 as ::core::ffi::c_int
            + (*src.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            + pad) as int16_t;
        tmp = tmp.offset(tmpStride as isize);
        src = src.offset(srcStride as isize);
        i += 1;
    }
    tmp = tmp
        .offset(
            -((tmpStride * (h + 5 as ::core::ffi::c_int - 2 as ::core::ffi::c_int))
                as isize),
        );
    i = 0 as ::core::ffi::c_int;
    while i < w {
        let tmpB: ::core::ffi::c_int = *tmp
            .offset((-(2 as ::core::ffi::c_int) * tmpStride) as isize)
            as ::core::ffi::c_int - pad;
        let tmpA: ::core::ffi::c_int = *tmp
            .offset((-(1 as ::core::ffi::c_int) * tmpStride) as isize)
            as ::core::ffi::c_int - pad;
        let tmp0: ::core::ffi::c_int = *tmp
            .offset((0 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp1: ::core::ffi::c_int = *tmp
            .offset((1 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp2: ::core::ffi::c_int = *tmp
            .offset((2 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp3: ::core::ffi::c_int = *tmp
            .offset((3 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp4: ::core::ffi::c_int = *tmp
            .offset((4 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp5: ::core::ffi::c_int = *tmp
            .offset((5 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp6: ::core::ffi::c_int = *tmp
            .offset((6 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp7: ::core::ffi::c_int = *tmp
            .offset((7 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp8: ::core::ffi::c_int = *tmp
            .offset((8 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp9: ::core::ffi::c_int = *tmp
            .offset((9 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp10: ::core::ffi::c_int = *tmp
            .offset((10 as ::core::ffi::c_int * tmpStride) as isize)
            as ::core::ffi::c_int - pad;
        *dst.offset((0 as ::core::ffi::c_int * dstStride) as isize) = av_clip_uint8_c(
            (tmp0 + tmp1) * 20 as ::core::ffi::c_int
                - (tmpA + tmp2) * 5 as ::core::ffi::c_int + (tmpB + tmp3)
                + 512 as ::core::ffi::c_int >> 10 as ::core::ffi::c_int,
        );
        *dst.offset((1 as ::core::ffi::c_int * dstStride) as isize) = av_clip_uint8_c(
            (tmp1 + tmp2) * 20 as ::core::ffi::c_int
                - (tmp0 + tmp3) * 5 as ::core::ffi::c_int + (tmpA + tmp4)
                + 512 as ::core::ffi::c_int >> 10 as ::core::ffi::c_int,
        );
        *dst.offset((2 as ::core::ffi::c_int * dstStride) as isize) = av_clip_uint8_c(
            (tmp2 + tmp3) * 20 as ::core::ffi::c_int
                - (tmp1 + tmp4) * 5 as ::core::ffi::c_int + (tmp0 + tmp5)
                + 512 as ::core::ffi::c_int >> 10 as ::core::ffi::c_int,
        );
        *dst.offset((3 as ::core::ffi::c_int * dstStride) as isize) = av_clip_uint8_c(
            (tmp3 + tmp4) * 20 as ::core::ffi::c_int
                - (tmp2 + tmp5) * 5 as ::core::ffi::c_int + (tmp1 + tmp6)
                + 512 as ::core::ffi::c_int >> 10 as ::core::ffi::c_int,
        );
        *dst.offset((4 as ::core::ffi::c_int * dstStride) as isize) = av_clip_uint8_c(
            (tmp4 + tmp5) * 20 as ::core::ffi::c_int
                - (tmp3 + tmp6) * 5 as ::core::ffi::c_int + (tmp2 + tmp7)
                + 512 as ::core::ffi::c_int >> 10 as ::core::ffi::c_int,
        );
        *dst.offset((5 as ::core::ffi::c_int * dstStride) as isize) = av_clip_uint8_c(
            (tmp5 + tmp6) * 20 as ::core::ffi::c_int
                - (tmp4 + tmp7) * 5 as ::core::ffi::c_int + (tmp3 + tmp8)
                + 512 as ::core::ffi::c_int >> 10 as ::core::ffi::c_int,
        );
        *dst.offset((6 as ::core::ffi::c_int * dstStride) as isize) = av_clip_uint8_c(
            (tmp6 + tmp7) * 20 as ::core::ffi::c_int
                - (tmp5 + tmp8) * 5 as ::core::ffi::c_int + (tmp4 + tmp9)
                + 512 as ::core::ffi::c_int >> 10 as ::core::ffi::c_int,
        );
        *dst.offset((7 as ::core::ffi::c_int * dstStride) as isize) = av_clip_uint8_c(
            (tmp7 + tmp8) * 20 as ::core::ffi::c_int
                - (tmp6 + tmp9) * 5 as ::core::ffi::c_int + (tmp5 + tmp10)
                + 512 as ::core::ffi::c_int >> 10 as ::core::ffi::c_int,
        );
        dst = dst.offset(1);
        tmp = tmp.offset(1);
        i += 1;
    }
}




unsafe extern "C" fn put_h264_qpel16_v_lowpass_8(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut dstStride: ::core::ffi::c_int,
    mut srcStride: ::core::ffi::c_int,
) {
    put_h264_qpel8_v_lowpass_8(dst, src, dstStride, srcStride);
    put_h264_qpel8_v_lowpass_8(
        dst
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        src
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        dstStride,
        srcStride,
    );
    src = src.offset((8 as ::core::ffi::c_int * srcStride) as isize);
    dst = dst.offset((8 as ::core::ffi::c_int * dstStride) as isize);
    put_h264_qpel8_v_lowpass_8(dst, src, dstStride, srcStride);
    put_h264_qpel8_v_lowpass_8(
        dst
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        src
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        dstStride,
        srcStride,
    );
}




unsafe extern "C" fn put_h264_qpel16_h_lowpass_8(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut dstStride: ::core::ffi::c_int,
    mut srcStride: ::core::ffi::c_int,
) {
    put_h264_qpel8_h_lowpass_8(dst, src, dstStride, srcStride);
    put_h264_qpel8_h_lowpass_8(
        dst
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        src
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        dstStride,
        srcStride,
    );
    src = src.offset((8 as ::core::ffi::c_int * srcStride) as isize);
    dst = dst.offset((8 as ::core::ffi::c_int * dstStride) as isize);
    put_h264_qpel8_h_lowpass_8(dst, src, dstStride, srcStride);
    put_h264_qpel8_h_lowpass_8(
        dst
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        src
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        dstStride,
        srcStride,
    );
}




unsafe extern "C" fn put_h264_qpel16_hv_lowpass_8(
    mut dst: *mut uint8_t,
    mut tmp: *mut int16_t,
    mut src: *const uint8_t,
    mut dstStride: ::core::ffi::c_int,
    mut tmpStride: ::core::ffi::c_int,
    mut srcStride: ::core::ffi::c_int,
) {
    put_h264_qpel8_hv_lowpass_8(dst, tmp, src, dstStride, tmpStride, srcStride);
    put_h264_qpel8_hv_lowpass_8(
        dst
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        tmp.offset(8 as ::core::ffi::c_int as isize),
        src
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        dstStride,
        tmpStride,
        srcStride,
    );
    src = src.offset((8 as ::core::ffi::c_int * srcStride) as isize);
    dst = dst.offset((8 as ::core::ffi::c_int * dstStride) as isize);
    put_h264_qpel8_hv_lowpass_8(dst, tmp, src, dstStride, tmpStride, srcStride);
    put_h264_qpel8_hv_lowpass_8(
        dst
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        tmp.offset(8 as ::core::ffi::c_int as isize),
        src
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        dstStride,
        tmpStride,
        srcStride,
    );
}






unsafe extern "C" fn avg_h264_qpel4_h_lowpass_8(
    mut _dst: *mut uint8_t,
    mut _src: *const uint8_t,
    mut dstStride: ::core::ffi::c_int,
    mut srcStride: ::core::ffi::c_int,
) {
    let h: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut dst: *mut uint8_t = _dst;
    let mut src: *const uint8_t = _src;
    dstStride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    srcStride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    i = 0 as ::core::ffi::c_int;
    while i < h {
        *dst.offset(0 as ::core::ffi::c_int as isize) = (*dst
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (*src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) * 20 as ::core::ffi::c_int
                    - (*src.offset(-(1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        + *src.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) * 5 as ::core::ffi::c_int
                    + (*src.offset(-(2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        + *src.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) + 16 as ::core::ffi::c_int
                    >> 5 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        *dst.offset(1 as ::core::ffi::c_int as isize) = (*dst
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (*src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src.offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) * 20 as ::core::ffi::c_int
                    - (*src.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        + *src.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) * 5 as ::core::ffi::c_int
                    + (*src.offset(-(1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        + *src.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) + 16 as ::core::ffi::c_int
                    >> 5 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        *dst.offset(2 as ::core::ffi::c_int as isize) = (*dst
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (*src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src.offset(3 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) * 20 as ::core::ffi::c_int
                    - (*src.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        + *src.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) * 5 as ::core::ffi::c_int
                    + (*src.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        + *src.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) + 16 as ::core::ffi::c_int
                    >> 5 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        *dst.offset(3 as ::core::ffi::c_int as isize) = (*dst
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (*src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src.offset(4 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) * 20 as ::core::ffi::c_int
                    - (*src.offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        + *src.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) * 5 as ::core::ffi::c_int
                    + (*src.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        + *src.offset(6 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) + 16 as ::core::ffi::c_int
                    >> 5 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        dst = dst.offset(dstStride as isize);
        src = src.offset(srcStride as isize);
        i += 1;
    }
}


unsafe extern "C" fn avg_h264_qpel4_v_lowpass_8(
    mut _dst: *mut uint8_t,
    mut _src: *const uint8_t,
    mut dstStride: ::core::ffi::c_int,
    mut srcStride: ::core::ffi::c_int,
) {
    let w: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut dst: *mut uint8_t = _dst;
    let mut src: *const uint8_t = _src;
    dstStride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    srcStride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    i = 0 as ::core::ffi::c_int;
    while i < w {
        let srcB: ::core::ffi::c_int = *src
            .offset((-(2 as ::core::ffi::c_int) * srcStride) as isize)
            as ::core::ffi::c_int;
        let srcA: ::core::ffi::c_int = *src
            .offset((-(1 as ::core::ffi::c_int) * srcStride) as isize)
            as ::core::ffi::c_int;
        let src0: ::core::ffi::c_int = *src
            .offset((0 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src1: ::core::ffi::c_int = *src
            .offset((1 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src2: ::core::ffi::c_int = *src
            .offset((2 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src3: ::core::ffi::c_int = *src
            .offset((3 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src4: ::core::ffi::c_int = *src
            .offset((4 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src5: ::core::ffi::c_int = *src
            .offset((5 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src6: ::core::ffi::c_int = *src
            .offset((6 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        *dst.offset((0 as ::core::ffi::c_int * dstStride) as isize) = (*dst
            .offset((0 as ::core::ffi::c_int * dstStride) as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (src0 + src1) * 20 as ::core::ffi::c_int
                    - (srcA + src2) * 5 as ::core::ffi::c_int + (srcB + src3)
                    + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        *dst.offset((1 as ::core::ffi::c_int * dstStride) as isize) = (*dst
            .offset((1 as ::core::ffi::c_int * dstStride) as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (src1 + src2) * 20 as ::core::ffi::c_int
                    - (src0 + src3) * 5 as ::core::ffi::c_int + (srcA + src4)
                    + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        *dst.offset((2 as ::core::ffi::c_int * dstStride) as isize) = (*dst
            .offset((2 as ::core::ffi::c_int * dstStride) as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (src2 + src3) * 20 as ::core::ffi::c_int
                    - (src1 + src4) * 5 as ::core::ffi::c_int + (src0 + src5)
                    + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        *dst.offset((3 as ::core::ffi::c_int * dstStride) as isize) = (*dst
            .offset((3 as ::core::ffi::c_int * dstStride) as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (src3 + src4) * 20 as ::core::ffi::c_int
                    - (src2 + src5) * 5 as ::core::ffi::c_int + (src1 + src6)
                    + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        dst = dst.offset(1);
        src = src.offset(1);
        i += 1;
    }
}




unsafe extern "C" fn avg_h264_qpel4_hv_lowpass_8(
    mut _dst: *mut uint8_t,
    mut tmp: *mut int16_t,
    mut _src: *const uint8_t,
    mut dstStride: ::core::ffi::c_int,
    mut tmpStride: ::core::ffi::c_int,
    mut srcStride: ::core::ffi::c_int,
) {
    let h: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    let w: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    let pad: ::core::ffi::c_int = if 8 as ::core::ffi::c_int == 10 as ::core::ffi::c_int
    {
        -(10 as ::core::ffi::c_int)
            * (((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int)
    } else {
        0 as ::core::ffi::c_int
    };
    let mut i: ::core::ffi::c_int = 0;
    let mut dst: *mut uint8_t = _dst;
    let mut src: *const uint8_t = _src;
    dstStride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    srcStride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    src = src.offset(-((2 as ::core::ffi::c_int * srcStride) as isize));
    i = 0 as ::core::ffi::c_int;
    while i < h + 5 as ::core::ffi::c_int {
        *tmp.offset(0 as ::core::ffi::c_int as isize) = ((*src
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            * 20 as ::core::ffi::c_int
            - (*src.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 5 as ::core::ffi::c_int
            + (*src.offset(-(2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            + pad) as int16_t;
        *tmp.offset(1 as ::core::ffi::c_int as isize) = ((*src
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            * 20 as ::core::ffi::c_int
            - (*src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 5 as ::core::ffi::c_int
            + (*src.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            + pad) as int16_t;
        *tmp.offset(2 as ::core::ffi::c_int as isize) = ((*src
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            * 20 as ::core::ffi::c_int
            - (*src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 5 as ::core::ffi::c_int
            + (*src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            + pad) as int16_t;
        *tmp.offset(3 as ::core::ffi::c_int as isize) = ((*src
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            * 20 as ::core::ffi::c_int
            - (*src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 5 as ::core::ffi::c_int
            + (*src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            + pad) as int16_t;
        tmp = tmp.offset(tmpStride as isize);
        src = src.offset(srcStride as isize);
        i += 1;
    }
    tmp = tmp
        .offset(
            -((tmpStride * (h + 5 as ::core::ffi::c_int - 2 as ::core::ffi::c_int))
                as isize),
        );
    i = 0 as ::core::ffi::c_int;
    while i < w {
        let tmpB: ::core::ffi::c_int = *tmp
            .offset((-(2 as ::core::ffi::c_int) * tmpStride) as isize)
            as ::core::ffi::c_int - pad;
        let tmpA: ::core::ffi::c_int = *tmp
            .offset((-(1 as ::core::ffi::c_int) * tmpStride) as isize)
            as ::core::ffi::c_int - pad;
        let tmp0: ::core::ffi::c_int = *tmp
            .offset((0 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp1: ::core::ffi::c_int = *tmp
            .offset((1 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp2: ::core::ffi::c_int = *tmp
            .offset((2 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp3: ::core::ffi::c_int = *tmp
            .offset((3 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp4: ::core::ffi::c_int = *tmp
            .offset((4 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp5: ::core::ffi::c_int = *tmp
            .offset((5 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp6: ::core::ffi::c_int = *tmp
            .offset((6 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        *dst.offset((0 as ::core::ffi::c_int * dstStride) as isize) = (*dst
            .offset((0 as ::core::ffi::c_int * dstStride) as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (tmp0 + tmp1) * 20 as ::core::ffi::c_int
                    - (tmpA + tmp2) * 5 as ::core::ffi::c_int + (tmpB + tmp3)
                    + 512 as ::core::ffi::c_int >> 10 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        *dst.offset((1 as ::core::ffi::c_int * dstStride) as isize) = (*dst
            .offset((1 as ::core::ffi::c_int * dstStride) as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (tmp1 + tmp2) * 20 as ::core::ffi::c_int
                    - (tmp0 + tmp3) * 5 as ::core::ffi::c_int + (tmpA + tmp4)
                    + 512 as ::core::ffi::c_int >> 10 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        *dst.offset((2 as ::core::ffi::c_int * dstStride) as isize) = (*dst
            .offset((2 as ::core::ffi::c_int * dstStride) as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (tmp2 + tmp3) * 20 as ::core::ffi::c_int
                    - (tmp1 + tmp4) * 5 as ::core::ffi::c_int + (tmp0 + tmp5)
                    + 512 as ::core::ffi::c_int >> 10 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        *dst.offset((3 as ::core::ffi::c_int * dstStride) as isize) = (*dst
            .offset((3 as ::core::ffi::c_int * dstStride) as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (tmp3 + tmp4) * 20 as ::core::ffi::c_int
                    - (tmp2 + tmp5) * 5 as ::core::ffi::c_int + (tmp1 + tmp6)
                    + 512 as ::core::ffi::c_int >> 10 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        dst = dst.offset(1);
        tmp = tmp.offset(1);
        i += 1;
    }
}




unsafe extern "C" fn avg_h264_qpel8_h_lowpass_8(
    mut _dst: *mut uint8_t,
    mut _src: *const uint8_t,
    mut dstStride: ::core::ffi::c_int,
    mut srcStride: ::core::ffi::c_int,
) {
    let h: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut dst: *mut uint8_t = _dst;
    let mut src: *const uint8_t = _src;
    dstStride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    srcStride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    i = 0 as ::core::ffi::c_int;
    while i < h {
        *dst.offset(0 as ::core::ffi::c_int as isize) = (*dst
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (*src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) * 20 as ::core::ffi::c_int
                    - (*src.offset(-(1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        + *src.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) * 5 as ::core::ffi::c_int
                    + (*src.offset(-(2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        + *src.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) + 16 as ::core::ffi::c_int
                    >> 5 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        *dst.offset(1 as ::core::ffi::c_int as isize) = (*dst
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (*src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src.offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) * 20 as ::core::ffi::c_int
                    - (*src.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        + *src.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) * 5 as ::core::ffi::c_int
                    + (*src.offset(-(1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        + *src.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) + 16 as ::core::ffi::c_int
                    >> 5 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        *dst.offset(2 as ::core::ffi::c_int as isize) = (*dst
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (*src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src.offset(3 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) * 20 as ::core::ffi::c_int
                    - (*src.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        + *src.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) * 5 as ::core::ffi::c_int
                    + (*src.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        + *src.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) + 16 as ::core::ffi::c_int
                    >> 5 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        *dst.offset(3 as ::core::ffi::c_int as isize) = (*dst
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (*src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src.offset(4 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) * 20 as ::core::ffi::c_int
                    - (*src.offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        + *src.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) * 5 as ::core::ffi::c_int
                    + (*src.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        + *src.offset(6 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) + 16 as ::core::ffi::c_int
                    >> 5 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        *dst.offset(4 as ::core::ffi::c_int as isize) = (*dst
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (*src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src.offset(5 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) * 20 as ::core::ffi::c_int
                    - (*src.offset(3 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        + *src.offset(6 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) * 5 as ::core::ffi::c_int
                    + (*src.offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        + *src.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) + 16 as ::core::ffi::c_int
                    >> 5 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        *dst.offset(5 as ::core::ffi::c_int as isize) = (*dst
            .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (*src.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src.offset(6 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) * 20 as ::core::ffi::c_int
                    - (*src.offset(4 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        + *src.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) * 5 as ::core::ffi::c_int
                    + (*src.offset(3 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        + *src.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) + 16 as ::core::ffi::c_int
                    >> 5 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        *dst.offset(6 as ::core::ffi::c_int as isize) = (*dst
            .offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (*src.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src.offset(7 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) * 20 as ::core::ffi::c_int
                    - (*src.offset(5 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        + *src.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) * 5 as ::core::ffi::c_int
                    + (*src.offset(4 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        + *src.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) + 16 as ::core::ffi::c_int
                    >> 5 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        *dst.offset(7 as ::core::ffi::c_int as isize) = (*dst
            .offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (*src.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *src.offset(8 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) * 20 as ::core::ffi::c_int
                    - (*src.offset(6 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        + *src.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) * 5 as ::core::ffi::c_int
                    + (*src.offset(5 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        + *src.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) + 16 as ::core::ffi::c_int
                    >> 5 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        dst = dst.offset(dstStride as isize);
        src = src.offset(srcStride as isize);
        i += 1;
    }
}




unsafe extern "C" fn avg_h264_qpel8_v_lowpass_8(
    mut _dst: *mut uint8_t,
    mut _src: *const uint8_t,
    mut dstStride: ::core::ffi::c_int,
    mut srcStride: ::core::ffi::c_int,
) {
    let w: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut dst: *mut uint8_t = _dst;
    let mut src: *const uint8_t = _src;
    dstStride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    srcStride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    i = 0 as ::core::ffi::c_int;
    while i < w {
        let srcB: ::core::ffi::c_int = *src
            .offset((-(2 as ::core::ffi::c_int) * srcStride) as isize)
            as ::core::ffi::c_int;
        let srcA: ::core::ffi::c_int = *src
            .offset((-(1 as ::core::ffi::c_int) * srcStride) as isize)
            as ::core::ffi::c_int;
        let src0: ::core::ffi::c_int = *src
            .offset((0 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src1: ::core::ffi::c_int = *src
            .offset((1 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src2: ::core::ffi::c_int = *src
            .offset((2 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src3: ::core::ffi::c_int = *src
            .offset((3 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src4: ::core::ffi::c_int = *src
            .offset((4 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src5: ::core::ffi::c_int = *src
            .offset((5 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src6: ::core::ffi::c_int = *src
            .offset((6 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src7: ::core::ffi::c_int = *src
            .offset((7 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src8: ::core::ffi::c_int = *src
            .offset((8 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src9: ::core::ffi::c_int = *src
            .offset((9 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        let src10: ::core::ffi::c_int = *src
            .offset((10 as ::core::ffi::c_int * srcStride) as isize)
            as ::core::ffi::c_int;
        *dst.offset((0 as ::core::ffi::c_int * dstStride) as isize) = (*dst
            .offset((0 as ::core::ffi::c_int * dstStride) as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (src0 + src1) * 20 as ::core::ffi::c_int
                    - (srcA + src2) * 5 as ::core::ffi::c_int + (srcB + src3)
                    + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        *dst.offset((1 as ::core::ffi::c_int * dstStride) as isize) = (*dst
            .offset((1 as ::core::ffi::c_int * dstStride) as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (src1 + src2) * 20 as ::core::ffi::c_int
                    - (src0 + src3) * 5 as ::core::ffi::c_int + (srcA + src4)
                    + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        *dst.offset((2 as ::core::ffi::c_int * dstStride) as isize) = (*dst
            .offset((2 as ::core::ffi::c_int * dstStride) as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (src2 + src3) * 20 as ::core::ffi::c_int
                    - (src1 + src4) * 5 as ::core::ffi::c_int + (src0 + src5)
                    + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        *dst.offset((3 as ::core::ffi::c_int * dstStride) as isize) = (*dst
            .offset((3 as ::core::ffi::c_int * dstStride) as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (src3 + src4) * 20 as ::core::ffi::c_int
                    - (src2 + src5) * 5 as ::core::ffi::c_int + (src1 + src6)
                    + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        *dst.offset((4 as ::core::ffi::c_int * dstStride) as isize) = (*dst
            .offset((4 as ::core::ffi::c_int * dstStride) as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (src4 + src5) * 20 as ::core::ffi::c_int
                    - (src3 + src6) * 5 as ::core::ffi::c_int + (src2 + src7)
                    + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        *dst.offset((5 as ::core::ffi::c_int * dstStride) as isize) = (*dst
            .offset((5 as ::core::ffi::c_int * dstStride) as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (src5 + src6) * 20 as ::core::ffi::c_int
                    - (src4 + src7) * 5 as ::core::ffi::c_int + (src3 + src8)
                    + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        *dst.offset((6 as ::core::ffi::c_int * dstStride) as isize) = (*dst
            .offset((6 as ::core::ffi::c_int * dstStride) as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (src6 + src7) * 20 as ::core::ffi::c_int
                    - (src5 + src8) * 5 as ::core::ffi::c_int + (src4 + src9)
                    + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        *dst.offset((7 as ::core::ffi::c_int * dstStride) as isize) = (*dst
            .offset((7 as ::core::ffi::c_int * dstStride) as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (src7 + src8) * 20 as ::core::ffi::c_int
                    - (src6 + src9) * 5 as ::core::ffi::c_int + (src5 + src10)
                    + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        dst = dst.offset(1);
        src = src.offset(1);
        i += 1;
    }
}




unsafe extern "C" fn avg_h264_qpel8_hv_lowpass_8(
    mut _dst: *mut uint8_t,
    mut tmp: *mut int16_t,
    mut _src: *const uint8_t,
    mut dstStride: ::core::ffi::c_int,
    mut tmpStride: ::core::ffi::c_int,
    mut srcStride: ::core::ffi::c_int,
) {
    let h: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    let w: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    let pad: ::core::ffi::c_int = if 8 as ::core::ffi::c_int == 10 as ::core::ffi::c_int
    {
        -(10 as ::core::ffi::c_int)
            * (((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int)
    } else {
        0 as ::core::ffi::c_int
    };
    let mut i: ::core::ffi::c_int = 0;
    let mut dst: *mut uint8_t = _dst;
    let mut src: *const uint8_t = _src;
    dstStride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    srcStride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    src = src.offset(-((2 as ::core::ffi::c_int * srcStride) as isize));
    i = 0 as ::core::ffi::c_int;
    while i < h + 5 as ::core::ffi::c_int {
        *tmp.offset(0 as ::core::ffi::c_int as isize) = ((*src
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            * 20 as ::core::ffi::c_int
            - (*src.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 5 as ::core::ffi::c_int
            + (*src.offset(-(2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            + pad) as int16_t;
        *tmp.offset(1 as ::core::ffi::c_int as isize) = ((*src
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            * 20 as ::core::ffi::c_int
            - (*src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 5 as ::core::ffi::c_int
            + (*src.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            + pad) as int16_t;
        *tmp.offset(2 as ::core::ffi::c_int as isize) = ((*src
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            * 20 as ::core::ffi::c_int
            - (*src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 5 as ::core::ffi::c_int
            + (*src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            + pad) as int16_t;
        *tmp.offset(3 as ::core::ffi::c_int as isize) = ((*src
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            * 20 as ::core::ffi::c_int
            - (*src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 5 as ::core::ffi::c_int
            + (*src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            + pad) as int16_t;
        *tmp.offset(4 as ::core::ffi::c_int as isize) = ((*src
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *src.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            * 20 as ::core::ffi::c_int
            - (*src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 5 as ::core::ffi::c_int
            + (*src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            + pad) as int16_t;
        *tmp.offset(5 as ::core::ffi::c_int as isize) = ((*src
            .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *src.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            * 20 as ::core::ffi::c_int
            - (*src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 5 as ::core::ffi::c_int
            + (*src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            + pad) as int16_t;
        *tmp.offset(6 as ::core::ffi::c_int as isize) = ((*src
            .offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *src.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            * 20 as ::core::ffi::c_int
            - (*src.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 5 as ::core::ffi::c_int
            + (*src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            + pad) as int16_t;
        *tmp.offset(7 as ::core::ffi::c_int as isize) = ((*src
            .offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *src.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            * 20 as ::core::ffi::c_int
            - (*src.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                * 5 as ::core::ffi::c_int
            + (*src.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *src.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            + pad) as int16_t;
        tmp = tmp.offset(tmpStride as isize);
        src = src.offset(srcStride as isize);
        i += 1;
    }
    tmp = tmp
        .offset(
            -((tmpStride * (h + 5 as ::core::ffi::c_int - 2 as ::core::ffi::c_int))
                as isize),
        );
    i = 0 as ::core::ffi::c_int;
    while i < w {
        let tmpB: ::core::ffi::c_int = *tmp
            .offset((-(2 as ::core::ffi::c_int) * tmpStride) as isize)
            as ::core::ffi::c_int - pad;
        let tmpA: ::core::ffi::c_int = *tmp
            .offset((-(1 as ::core::ffi::c_int) * tmpStride) as isize)
            as ::core::ffi::c_int - pad;
        let tmp0: ::core::ffi::c_int = *tmp
            .offset((0 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp1: ::core::ffi::c_int = *tmp
            .offset((1 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp2: ::core::ffi::c_int = *tmp
            .offset((2 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp3: ::core::ffi::c_int = *tmp
            .offset((3 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp4: ::core::ffi::c_int = *tmp
            .offset((4 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp5: ::core::ffi::c_int = *tmp
            .offset((5 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp6: ::core::ffi::c_int = *tmp
            .offset((6 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp7: ::core::ffi::c_int = *tmp
            .offset((7 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp8: ::core::ffi::c_int = *tmp
            .offset((8 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp9: ::core::ffi::c_int = *tmp
            .offset((9 as ::core::ffi::c_int * tmpStride) as isize) as ::core::ffi::c_int
            - pad;
        let tmp10: ::core::ffi::c_int = *tmp
            .offset((10 as ::core::ffi::c_int * tmpStride) as isize)
            as ::core::ffi::c_int - pad;
        *dst.offset((0 as ::core::ffi::c_int * dstStride) as isize) = (*dst
            .offset((0 as ::core::ffi::c_int * dstStride) as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (tmp0 + tmp1) * 20 as ::core::ffi::c_int
                    - (tmpA + tmp2) * 5 as ::core::ffi::c_int + (tmpB + tmp3)
                    + 512 as ::core::ffi::c_int >> 10 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        *dst.offset((1 as ::core::ffi::c_int * dstStride) as isize) = (*dst
            .offset((1 as ::core::ffi::c_int * dstStride) as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (tmp1 + tmp2) * 20 as ::core::ffi::c_int
                    - (tmp0 + tmp3) * 5 as ::core::ffi::c_int + (tmpA + tmp4)
                    + 512 as ::core::ffi::c_int >> 10 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        *dst.offset((2 as ::core::ffi::c_int * dstStride) as isize) = (*dst
            .offset((2 as ::core::ffi::c_int * dstStride) as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (tmp2 + tmp3) * 20 as ::core::ffi::c_int
                    - (tmp1 + tmp4) * 5 as ::core::ffi::c_int + (tmp0 + tmp5)
                    + 512 as ::core::ffi::c_int >> 10 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        *dst.offset((3 as ::core::ffi::c_int * dstStride) as isize) = (*dst
            .offset((3 as ::core::ffi::c_int * dstStride) as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (tmp3 + tmp4) * 20 as ::core::ffi::c_int
                    - (tmp2 + tmp5) * 5 as ::core::ffi::c_int + (tmp1 + tmp6)
                    + 512 as ::core::ffi::c_int >> 10 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        *dst.offset((4 as ::core::ffi::c_int * dstStride) as isize) = (*dst
            .offset((4 as ::core::ffi::c_int * dstStride) as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (tmp4 + tmp5) * 20 as ::core::ffi::c_int
                    - (tmp3 + tmp6) * 5 as ::core::ffi::c_int + (tmp2 + tmp7)
                    + 512 as ::core::ffi::c_int >> 10 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        *dst.offset((5 as ::core::ffi::c_int * dstStride) as isize) = (*dst
            .offset((5 as ::core::ffi::c_int * dstStride) as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (tmp5 + tmp6) * 20 as ::core::ffi::c_int
                    - (tmp4 + tmp7) * 5 as ::core::ffi::c_int + (tmp3 + tmp8)
                    + 512 as ::core::ffi::c_int >> 10 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        *dst.offset((6 as ::core::ffi::c_int * dstStride) as isize) = (*dst
            .offset((6 as ::core::ffi::c_int * dstStride) as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (tmp6 + tmp7) * 20 as ::core::ffi::c_int
                    - (tmp5 + tmp8) * 5 as ::core::ffi::c_int + (tmp4 + tmp9)
                    + 512 as ::core::ffi::c_int >> 10 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        *dst.offset((7 as ::core::ffi::c_int * dstStride) as isize) = (*dst
            .offset((7 as ::core::ffi::c_int * dstStride) as isize) as ::core::ffi::c_int
            + av_clip_uint8_c(
                (tmp7 + tmp8) * 20 as ::core::ffi::c_int
                    - (tmp6 + tmp9) * 5 as ::core::ffi::c_int + (tmp5 + tmp10)
                    + 512 as ::core::ffi::c_int >> 10 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as uint8_t;
        dst = dst.offset(1);
        tmp = tmp.offset(1);
        i += 1;
    }
}




unsafe extern "C" fn avg_h264_qpel16_v_lowpass_8(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut dstStride: ::core::ffi::c_int,
    mut srcStride: ::core::ffi::c_int,
) {
    avg_h264_qpel8_v_lowpass_8(dst, src, dstStride, srcStride);
    avg_h264_qpel8_v_lowpass_8(
        dst
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        src
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        dstStride,
        srcStride,
    );
    src = src.offset((8 as ::core::ffi::c_int * srcStride) as isize);
    dst = dst.offset((8 as ::core::ffi::c_int * dstStride) as isize);
    avg_h264_qpel8_v_lowpass_8(dst, src, dstStride, srcStride);
    avg_h264_qpel8_v_lowpass_8(
        dst
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        src
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        dstStride,
        srcStride,
    );
}




unsafe extern "C" fn avg_h264_qpel16_h_lowpass_8(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut dstStride: ::core::ffi::c_int,
    mut srcStride: ::core::ffi::c_int,
) {
    avg_h264_qpel8_h_lowpass_8(dst, src, dstStride, srcStride);
    avg_h264_qpel8_h_lowpass_8(
        dst
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        src
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        dstStride,
        srcStride,
    );
    src = src.offset((8 as ::core::ffi::c_int * srcStride) as isize);
    dst = dst.offset((8 as ::core::ffi::c_int * dstStride) as isize);
    avg_h264_qpel8_h_lowpass_8(dst, src, dstStride, srcStride);
    avg_h264_qpel8_h_lowpass_8(
        dst
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        src
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        dstStride,
        srcStride,
    );
}




unsafe extern "C" fn avg_h264_qpel16_hv_lowpass_8(
    mut dst: *mut uint8_t,
    mut tmp: *mut int16_t,
    mut src: *const uint8_t,
    mut dstStride: ::core::ffi::c_int,
    mut tmpStride: ::core::ffi::c_int,
    mut srcStride: ::core::ffi::c_int,
) {
    avg_h264_qpel8_hv_lowpass_8(dst, tmp, src, dstStride, tmpStride, srcStride);
    avg_h264_qpel8_hv_lowpass_8(
        dst
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        tmp.offset(8 as ::core::ffi::c_int as isize),
        src
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        dstStride,
        tmpStride,
        srcStride,
    );
    src = src.offset((8 as ::core::ffi::c_int * srcStride) as isize);
    dst = dst.offset((8 as ::core::ffi::c_int * dstStride) as isize);
    avg_h264_qpel8_hv_lowpass_8(dst, tmp, src, dstStride, tmpStride, srcStride);
    avg_h264_qpel8_hv_lowpass_8(
        dst
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        tmp.offset(8 as ::core::ffi::c_int as isize),
        src
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        dstStride,
        tmpStride,
        srcStride,
    );
}




unsafe extern "C" fn put_h264_qpel4_mc00_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    put_pixels4_8_c(dst, src, stride, 4 as ::core::ffi::c_int);
}





unsafe extern "C" fn put_h264_qpel4_mc10_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut half: [uint8_t; 16] = [0; 16];
    put_h264_qpel4_h_lowpass_8(
        &raw mut half as *mut uint8_t,
        src,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    put_pixels4_l2_8(
        dst,
        src,
        &raw mut half as *mut uint8_t,
        stride,
        stride,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        4 as ::core::ffi::c_int,
    );
}



unsafe extern "C" fn put_h264_qpel4_mc20_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    put_h264_qpel4_h_lowpass_8(
        dst,
        src,
        stride as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel4_mc30_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut half: [uint8_t; 16] = [0; 16];
    put_h264_qpel4_h_lowpass_8(
        &raw mut half as *mut uint8_t,
        src,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    put_pixels4_l2_8(
        dst,
        src.offset(::core::mem::size_of::<uint8_t>() as usize as isize),
        &raw mut half as *mut uint8_t,
        stride,
        stride,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        4 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel4_mc01_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 36] = [0; 36];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut half: [uint8_t; 16] = [0; 16];
    copy_block4_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel4_v_lowpass_8(
        &raw mut half as *mut uint8_t,
        full_mid,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    put_pixels4_l2_8(
        dst,
        full_mid,
        &raw mut half as *mut uint8_t,
        stride,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        4 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel4_mc02_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 36] = [0; 36];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    copy_block4_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel4_v_lowpass_8(
        dst,
        full_mid,
        stride as ::core::ffi::c_int,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel4_mc03_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 36] = [0; 36];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut half: [uint8_t; 16] = [0; 16];
    copy_block4_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel4_v_lowpass_8(
        &raw mut half as *mut uint8_t,
        full_mid,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    put_pixels4_l2_8(
        dst,
        full_mid
            .offset(
                (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        &raw mut half as *mut uint8_t,
        stride,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        4 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel4_mc11_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 36] = [0; 36];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut halfH: [uint8_t; 16] = [0; 16];
    let mut halfV: [uint8_t; 16] = [0; 16];
    put_h264_qpel4_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    copy_block4_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel4_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    put_pixels4_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfV as *mut uint8_t,
        stride,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        4 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel4_mc31_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 36] = [0; 36];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut halfH: [uint8_t; 16] = [0; 16];
    let mut halfV: [uint8_t; 16] = [0; 16];
    put_h264_qpel4_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    copy_block4_8(
        &raw mut full as *mut uint8_t,
        src
            .offset(-((stride * 2 as ptrdiff_t) as isize))
            .offset(::core::mem::size_of::<uint8_t>() as usize as isize),
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel4_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    put_pixels4_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfV as *mut uint8_t,
        stride,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        4 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel4_mc13_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 36] = [0; 36];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut halfH: [uint8_t; 16] = [0; 16];
    let mut halfV: [uint8_t; 16] = [0; 16];
    put_h264_qpel4_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src.offset(stride as isize),
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    copy_block4_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel4_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    put_pixels4_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfV as *mut uint8_t,
        stride,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        4 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel4_mc33_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 36] = [0; 36];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut halfH: [uint8_t; 16] = [0; 16];
    let mut halfV: [uint8_t; 16] = [0; 16];
    put_h264_qpel4_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src.offset(stride as isize),
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    copy_block4_8(
        &raw mut full as *mut uint8_t,
        src
            .offset(-((stride * 2 as ptrdiff_t) as isize))
            .offset(::core::mem::size_of::<uint8_t>() as usize as isize),
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel4_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    put_pixels4_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfV as *mut uint8_t,
        stride,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        4 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel4_mc22_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut tmp: [int16_t; 36] = [0; 36];
    put_h264_qpel4_hv_lowpass_8(
        dst,
        &raw mut tmp as *mut int16_t,
        src,
        stride as ::core::ffi::c_int,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel4_mc21_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut tmp: [int16_t; 36] = [0; 36];
    let mut halfH: [uint8_t; 16] = [0; 16];
    let mut halfHV: [uint8_t; 16] = [0; 16];
    put_h264_qpel4_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    put_h264_qpel4_hv_lowpass_8(
        &raw mut halfHV as *mut uint8_t,
        &raw mut tmp as *mut int16_t,
        src,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    put_pixels4_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfHV as *mut uint8_t,
        stride,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        4 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel4_mc23_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut tmp: [int16_t; 36] = [0; 36];
    let mut halfH: [uint8_t; 16] = [0; 16];
    let mut halfHV: [uint8_t; 16] = [0; 16];
    put_h264_qpel4_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src.offset(stride as isize),
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    put_h264_qpel4_hv_lowpass_8(
        &raw mut halfHV as *mut uint8_t,
        &raw mut tmp as *mut int16_t,
        src,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    put_pixels4_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfHV as *mut uint8_t,
        stride,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        4 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel4_mc12_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 36] = [0; 36];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut tmp: [int16_t; 36] = [0; 36];
    let mut halfV: [uint8_t; 16] = [0; 16];
    let mut halfHV: [uint8_t; 16] = [0; 16];
    copy_block4_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel4_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    put_h264_qpel4_hv_lowpass_8(
        &raw mut halfHV as *mut uint8_t,
        &raw mut tmp as *mut int16_t,
        src,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    put_pixels4_l2_8(
        dst,
        &raw mut halfV as *mut uint8_t,
        &raw mut halfHV as *mut uint8_t,
        stride,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        4 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel4_mc32_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 36] = [0; 36];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut tmp: [int16_t; 36] = [0; 36];
    let mut halfV: [uint8_t; 16] = [0; 16];
    let mut halfHV: [uint8_t; 16] = [0; 16];
    copy_block4_8(
        &raw mut full as *mut uint8_t,
        src
            .offset(-((stride * 2 as ptrdiff_t) as isize))
            .offset(::core::mem::size_of::<uint8_t>() as usize as isize),
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel4_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    put_h264_qpel4_hv_lowpass_8(
        &raw mut halfHV as *mut uint8_t,
        &raw mut tmp as *mut int16_t,
        src,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    put_pixels4_l2_8(
        dst,
        &raw mut halfV as *mut uint8_t,
        &raw mut halfHV as *mut uint8_t,
        stride,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        4 as ::core::ffi::c_int,
    );
}








unsafe extern "C" fn put_h264_qpel8_mc00_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    put_pixels8_8_c(dst, src, stride, 8 as ::core::ffi::c_int);
}

unsafe extern "C" fn put_h264_qpel8_mc10_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut half: [uint8_t; 64] = [0; 64];
    put_h264_qpel8_h_lowpass_8(
        &raw mut half as *mut uint8_t,
        src,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    put_pixels8_l2_8(
        dst,
        src,
        &raw mut half as *mut uint8_t,
        stride,
        stride,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        8 as ::core::ffi::c_int,
    );
}



unsafe extern "C" fn put_h264_qpel8_mc20_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    put_h264_qpel8_h_lowpass_8(
        dst,
        src,
        stride as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel8_mc30_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut half: [uint8_t; 64] = [0; 64];
    put_h264_qpel8_h_lowpass_8(
        &raw mut half as *mut uint8_t,
        src,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    put_pixels8_l2_8(
        dst,
        src.offset(::core::mem::size_of::<uint8_t>() as usize as isize),
        &raw mut half as *mut uint8_t,
        stride,
        stride,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        8 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel8_mc01_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 104] = [0; 104];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut half: [uint8_t; 64] = [0; 64];
    copy_block8_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel8_v_lowpass_8(
        &raw mut half as *mut uint8_t,
        full_mid,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    put_pixels8_l2_8(
        dst,
        full_mid,
        &raw mut half as *mut uint8_t,
        stride,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        8 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel8_mc02_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 104] = [0; 104];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    copy_block8_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel8_v_lowpass_8(
        dst,
        full_mid,
        stride as ::core::ffi::c_int,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel8_mc03_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 104] = [0; 104];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut half: [uint8_t; 64] = [0; 64];
    copy_block8_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel8_v_lowpass_8(
        &raw mut half as *mut uint8_t,
        full_mid,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    put_pixels8_l2_8(
        dst,
        full_mid
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        &raw mut half as *mut uint8_t,
        stride,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        8 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel8_mc11_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 104] = [0; 104];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut halfH: [uint8_t; 64] = [0; 64];
    let mut halfV: [uint8_t; 64] = [0; 64];
    put_h264_qpel8_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    copy_block8_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel8_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    put_pixels8_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfV as *mut uint8_t,
        stride,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        8 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel8_mc31_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 104] = [0; 104];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut halfH: [uint8_t; 64] = [0; 64];
    let mut halfV: [uint8_t; 64] = [0; 64];
    put_h264_qpel8_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    copy_block8_8(
        &raw mut full as *mut uint8_t,
        src
            .offset(-((stride * 2 as ptrdiff_t) as isize))
            .offset(::core::mem::size_of::<uint8_t>() as usize as isize),
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel8_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    put_pixels8_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfV as *mut uint8_t,
        stride,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        8 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel8_mc13_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 104] = [0; 104];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut halfH: [uint8_t; 64] = [0; 64];
    let mut halfV: [uint8_t; 64] = [0; 64];
    put_h264_qpel8_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src.offset(stride as isize),
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    copy_block8_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel8_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    put_pixels8_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfV as *mut uint8_t,
        stride,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        8 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel8_mc33_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 104] = [0; 104];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut halfH: [uint8_t; 64] = [0; 64];
    let mut halfV: [uint8_t; 64] = [0; 64];
    put_h264_qpel8_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src.offset(stride as isize),
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    copy_block8_8(
        &raw mut full as *mut uint8_t,
        src
            .offset(-((stride * 2 as ptrdiff_t) as isize))
            .offset(::core::mem::size_of::<uint8_t>() as usize as isize),
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel8_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    put_pixels8_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfV as *mut uint8_t,
        stride,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        8 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel8_mc22_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut tmp: [int16_t; 104] = [0; 104];
    put_h264_qpel8_hv_lowpass_8(
        dst,
        &raw mut tmp as *mut int16_t,
        src,
        stride as ::core::ffi::c_int,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel8_mc21_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut tmp: [int16_t; 104] = [0; 104];
    let mut halfH: [uint8_t; 64] = [0; 64];
    let mut halfHV: [uint8_t; 64] = [0; 64];
    put_h264_qpel8_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    put_h264_qpel8_hv_lowpass_8(
        &raw mut halfHV as *mut uint8_t,
        &raw mut tmp as *mut int16_t,
        src,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    put_pixels8_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfHV as *mut uint8_t,
        stride,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        8 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel8_mc23_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut tmp: [int16_t; 104] = [0; 104];
    let mut halfH: [uint8_t; 64] = [0; 64];
    let mut halfHV: [uint8_t; 64] = [0; 64];
    put_h264_qpel8_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src.offset(stride as isize),
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    put_h264_qpel8_hv_lowpass_8(
        &raw mut halfHV as *mut uint8_t,
        &raw mut tmp as *mut int16_t,
        src,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    put_pixels8_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfHV as *mut uint8_t,
        stride,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        8 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel8_mc12_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 104] = [0; 104];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut tmp: [int16_t; 104] = [0; 104];
    let mut halfV: [uint8_t; 64] = [0; 64];
    let mut halfHV: [uint8_t; 64] = [0; 64];
    copy_block8_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel8_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    put_h264_qpel8_hv_lowpass_8(
        &raw mut halfHV as *mut uint8_t,
        &raw mut tmp as *mut int16_t,
        src,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    put_pixels8_l2_8(
        dst,
        &raw mut halfV as *mut uint8_t,
        &raw mut halfHV as *mut uint8_t,
        stride,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        8 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel8_mc32_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 104] = [0; 104];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut tmp: [int16_t; 104] = [0; 104];
    let mut halfV: [uint8_t; 64] = [0; 64];
    let mut halfHV: [uint8_t; 64] = [0; 64];
    copy_block8_8(
        &raw mut full as *mut uint8_t,
        src
            .offset(-((stride * 2 as ptrdiff_t) as isize))
            .offset(::core::mem::size_of::<uint8_t>() as usize as isize),
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel8_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    put_h264_qpel8_hv_lowpass_8(
        &raw mut halfHV as *mut uint8_t,
        &raw mut tmp as *mut int16_t,
        src,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    put_pixels8_l2_8(
        dst,
        &raw mut halfV as *mut uint8_t,
        &raw mut halfHV as *mut uint8_t,
        stride,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        8 as ::core::ffi::c_int,
    );
}








unsafe extern "C" fn put_h264_qpel16_mc00_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    put_pixels16_8_c(dst, src, stride, 16 as ::core::ffi::c_int);
}

unsafe extern "C" fn put_h264_qpel16_mc10_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut half: [uint8_t; 256] = [0; 256];
    put_h264_qpel16_h_lowpass_8(
        &raw mut half as *mut uint8_t,
        src,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    put_pixels16_l2_8(
        dst,
        src,
        &raw mut half as *mut uint8_t,
        stride,
        stride,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        16 as ::core::ffi::c_int,
    );
}



unsafe extern "C" fn put_h264_qpel16_mc20_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    put_h264_qpel16_h_lowpass_8(
        dst,
        src,
        stride as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel16_mc30_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut half: [uint8_t; 256] = [0; 256];
    put_h264_qpel16_h_lowpass_8(
        &raw mut half as *mut uint8_t,
        src,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    put_pixels16_l2_8(
        dst,
        src.offset(::core::mem::size_of::<uint8_t>() as usize as isize),
        &raw mut half as *mut uint8_t,
        stride,
        stride,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        16 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel16_mc01_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 336] = [0; 336];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut half: [uint8_t; 256] = [0; 256];
    copy_block16_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel16_v_lowpass_8(
        &raw mut half as *mut uint8_t,
        full_mid,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    put_pixels16_l2_8(
        dst,
        full_mid,
        &raw mut half as *mut uint8_t,
        stride,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        16 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel16_mc02_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 336] = [0; 336];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    copy_block16_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel16_v_lowpass_8(
        dst,
        full_mid,
        stride as ::core::ffi::c_int,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel16_mc03_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 336] = [0; 336];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut half: [uint8_t; 256] = [0; 256];
    copy_block16_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel16_v_lowpass_8(
        &raw mut half as *mut uint8_t,
        full_mid,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    put_pixels16_l2_8(
        dst,
        full_mid
            .offset(
                (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        &raw mut half as *mut uint8_t,
        stride,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        16 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel16_mc11_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 336] = [0; 336];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut halfH: [uint8_t; 256] = [0; 256];
    let mut halfV: [uint8_t; 256] = [0; 256];
    put_h264_qpel16_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    copy_block16_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel16_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    put_pixels16_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfV as *mut uint8_t,
        stride,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        16 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel16_mc31_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 336] = [0; 336];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut halfH: [uint8_t; 256] = [0; 256];
    let mut halfV: [uint8_t; 256] = [0; 256];
    put_h264_qpel16_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    copy_block16_8(
        &raw mut full as *mut uint8_t,
        src
            .offset(-((stride * 2 as ptrdiff_t) as isize))
            .offset(::core::mem::size_of::<uint8_t>() as usize as isize),
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel16_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    put_pixels16_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfV as *mut uint8_t,
        stride,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        16 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel16_mc13_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 336] = [0; 336];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut halfH: [uint8_t; 256] = [0; 256];
    let mut halfV: [uint8_t; 256] = [0; 256];
    put_h264_qpel16_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src.offset(stride as isize),
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    copy_block16_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel16_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    put_pixels16_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfV as *mut uint8_t,
        stride,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        16 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel16_mc33_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 336] = [0; 336];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut halfH: [uint8_t; 256] = [0; 256];
    let mut halfV: [uint8_t; 256] = [0; 256];
    put_h264_qpel16_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src.offset(stride as isize),
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    copy_block16_8(
        &raw mut full as *mut uint8_t,
        src
            .offset(-((stride * 2 as ptrdiff_t) as isize))
            .offset(::core::mem::size_of::<uint8_t>() as usize as isize),
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel16_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    put_pixels16_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfV as *mut uint8_t,
        stride,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        16 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel16_mc22_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut tmp: [int16_t; 336] = [0; 336];
    put_h264_qpel16_hv_lowpass_8(
        dst,
        &raw mut tmp as *mut int16_t,
        src,
        stride as ::core::ffi::c_int,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel16_mc21_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut tmp: [int16_t; 336] = [0; 336];
    let mut halfH: [uint8_t; 256] = [0; 256];
    let mut halfHV: [uint8_t; 256] = [0; 256];
    put_h264_qpel16_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    put_h264_qpel16_hv_lowpass_8(
        &raw mut halfHV as *mut uint8_t,
        &raw mut tmp as *mut int16_t,
        src,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    put_pixels16_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfHV as *mut uint8_t,
        stride,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        16 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel16_mc23_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut tmp: [int16_t; 336] = [0; 336];
    let mut halfH: [uint8_t; 256] = [0; 256];
    let mut halfHV: [uint8_t; 256] = [0; 256];
    put_h264_qpel16_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src.offset(stride as isize),
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    put_h264_qpel16_hv_lowpass_8(
        &raw mut halfHV as *mut uint8_t,
        &raw mut tmp as *mut int16_t,
        src,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    put_pixels16_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfHV as *mut uint8_t,
        stride,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        16 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel16_mc12_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 336] = [0; 336];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut tmp: [int16_t; 336] = [0; 336];
    let mut halfV: [uint8_t; 256] = [0; 256];
    let mut halfHV: [uint8_t; 256] = [0; 256];
    copy_block16_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel16_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    put_h264_qpel16_hv_lowpass_8(
        &raw mut halfHV as *mut uint8_t,
        &raw mut tmp as *mut int16_t,
        src,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    put_pixels16_l2_8(
        dst,
        &raw mut halfV as *mut uint8_t,
        &raw mut halfHV as *mut uint8_t,
        stride,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        16 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn put_h264_qpel16_mc32_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 336] = [0; 336];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut tmp: [int16_t; 336] = [0; 336];
    let mut halfV: [uint8_t; 256] = [0; 256];
    let mut halfHV: [uint8_t; 256] = [0; 256];
    copy_block16_8(
        &raw mut full as *mut uint8_t,
        src
            .offset(-((stride * 2 as ptrdiff_t) as isize))
            .offset(::core::mem::size_of::<uint8_t>() as usize as isize),
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel16_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    put_h264_qpel16_hv_lowpass_8(
        &raw mut halfHV as *mut uint8_t,
        &raw mut tmp as *mut int16_t,
        src,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    put_pixels16_l2_8(
        dst,
        &raw mut halfV as *mut uint8_t,
        &raw mut halfHV as *mut uint8_t,
        stride,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        16 as ::core::ffi::c_int,
    );
}






unsafe extern "C" fn avg_h264_qpel4_mc00_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    avg_pixels4_8_c(dst, src, stride, 4 as ::core::ffi::c_int);
}



unsafe extern "C" fn avg_h264_qpel4_mc10_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut half: [uint8_t; 16] = [0; 16];
    put_h264_qpel4_h_lowpass_8(
        &raw mut half as *mut uint8_t,
        src,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    avg_pixels4_l2_8(
        dst,
        src,
        &raw mut half as *mut uint8_t,
        stride,
        stride,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        4 as ::core::ffi::c_int,
    );
}



unsafe extern "C" fn avg_h264_qpel4_mc20_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    avg_h264_qpel4_h_lowpass_8(
        dst,
        src,
        stride as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel4_mc30_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut half: [uint8_t; 16] = [0; 16];
    put_h264_qpel4_h_lowpass_8(
        &raw mut half as *mut uint8_t,
        src,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    avg_pixels4_l2_8(
        dst,
        src.offset(::core::mem::size_of::<uint8_t>() as usize as isize),
        &raw mut half as *mut uint8_t,
        stride,
        stride,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        4 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel4_mc01_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 36] = [0; 36];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut half: [uint8_t; 16] = [0; 16];
    copy_block4_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel4_v_lowpass_8(
        &raw mut half as *mut uint8_t,
        full_mid,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    avg_pixels4_l2_8(
        dst,
        full_mid,
        &raw mut half as *mut uint8_t,
        stride,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        4 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel4_mc02_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 36] = [0; 36];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    copy_block4_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    avg_h264_qpel4_v_lowpass_8(
        dst,
        full_mid,
        stride as ::core::ffi::c_int,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel4_mc03_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 36] = [0; 36];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut half: [uint8_t; 16] = [0; 16];
    copy_block4_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel4_v_lowpass_8(
        &raw mut half as *mut uint8_t,
        full_mid,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    avg_pixels4_l2_8(
        dst,
        full_mid
            .offset(
                (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        &raw mut half as *mut uint8_t,
        stride,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        4 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel4_mc11_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 36] = [0; 36];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut halfH: [uint8_t; 16] = [0; 16];
    let mut halfV: [uint8_t; 16] = [0; 16];
    put_h264_qpel4_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    copy_block4_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel4_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    avg_pixels4_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfV as *mut uint8_t,
        stride,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        4 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel4_mc31_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 36] = [0; 36];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut halfH: [uint8_t; 16] = [0; 16];
    let mut halfV: [uint8_t; 16] = [0; 16];
    put_h264_qpel4_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    copy_block4_8(
        &raw mut full as *mut uint8_t,
        src
            .offset(-((stride * 2 as ptrdiff_t) as isize))
            .offset(::core::mem::size_of::<uint8_t>() as usize as isize),
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel4_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    avg_pixels4_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfV as *mut uint8_t,
        stride,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        4 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel4_mc13_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 36] = [0; 36];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut halfH: [uint8_t; 16] = [0; 16];
    let mut halfV: [uint8_t; 16] = [0; 16];
    put_h264_qpel4_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src.offset(stride as isize),
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    copy_block4_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel4_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    avg_pixels4_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfV as *mut uint8_t,
        stride,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        4 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel4_mc33_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 36] = [0; 36];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut halfH: [uint8_t; 16] = [0; 16];
    let mut halfV: [uint8_t; 16] = [0; 16];
    put_h264_qpel4_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src.offset(stride as isize),
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    copy_block4_8(
        &raw mut full as *mut uint8_t,
        src
            .offset(-((stride * 2 as ptrdiff_t) as isize))
            .offset(::core::mem::size_of::<uint8_t>() as usize as isize),
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel4_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    avg_pixels4_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfV as *mut uint8_t,
        stride,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        4 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel4_mc22_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut tmp: [int16_t; 36] = [0; 36];
    avg_h264_qpel4_hv_lowpass_8(
        dst,
        &raw mut tmp as *mut int16_t,
        src,
        stride as ::core::ffi::c_int,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel4_mc21_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut tmp: [int16_t; 36] = [0; 36];
    let mut halfH: [uint8_t; 16] = [0; 16];
    let mut halfHV: [uint8_t; 16] = [0; 16];
    put_h264_qpel4_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    put_h264_qpel4_hv_lowpass_8(
        &raw mut halfHV as *mut uint8_t,
        &raw mut tmp as *mut int16_t,
        src,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    avg_pixels4_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfHV as *mut uint8_t,
        stride,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        4 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel4_mc23_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut tmp: [int16_t; 36] = [0; 36];
    let mut halfH: [uint8_t; 16] = [0; 16];
    let mut halfHV: [uint8_t; 16] = [0; 16];
    put_h264_qpel4_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src.offset(stride as isize),
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    put_h264_qpel4_hv_lowpass_8(
        &raw mut halfHV as *mut uint8_t,
        &raw mut tmp as *mut int16_t,
        src,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    avg_pixels4_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfHV as *mut uint8_t,
        stride,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        4 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel4_mc12_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 36] = [0; 36];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut tmp: [int16_t; 36] = [0; 36];
    let mut halfV: [uint8_t; 16] = [0; 16];
    let mut halfHV: [uint8_t; 16] = [0; 16];
    copy_block4_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel4_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    put_h264_qpel4_hv_lowpass_8(
        &raw mut halfHV as *mut uint8_t,
        &raw mut tmp as *mut int16_t,
        src,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    avg_pixels4_l2_8(
        dst,
        &raw mut halfV as *mut uint8_t,
        &raw mut halfHV as *mut uint8_t,
        stride,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        4 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel4_mc32_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 36] = [0; 36];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut tmp: [int16_t; 36] = [0; 36];
    let mut halfV: [uint8_t; 16] = [0; 16];
    let mut halfHV: [uint8_t; 16] = [0; 16];
    copy_block4_8(
        &raw mut full as *mut uint8_t,
        src
            .offset(-((stride * 2 as ptrdiff_t) as isize))
            .offset(::core::mem::size_of::<uint8_t>() as usize as isize),
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel4_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    put_h264_qpel4_hv_lowpass_8(
        &raw mut halfHV as *mut uint8_t,
        &raw mut tmp as *mut int16_t,
        src,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    avg_pixels4_l2_8(
        dst,
        &raw mut halfV as *mut uint8_t,
        &raw mut halfHV as *mut uint8_t,
        stride,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (4 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        4 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel8_mc00_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    avg_pixels8_8_c(dst, src, stride, 8 as ::core::ffi::c_int);
}




unsafe extern "C" fn avg_h264_qpel8_mc10_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut half: [uint8_t; 64] = [0; 64];
    put_h264_qpel8_h_lowpass_8(
        &raw mut half as *mut uint8_t,
        src,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    avg_pixels8_l2_8(
        dst,
        src,
        &raw mut half as *mut uint8_t,
        stride,
        stride,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        8 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel8_mc20_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    avg_h264_qpel8_h_lowpass_8(
        dst,
        src,
        stride as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel8_mc30_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut half: [uint8_t; 64] = [0; 64];
    put_h264_qpel8_h_lowpass_8(
        &raw mut half as *mut uint8_t,
        src,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    avg_pixels8_l2_8(
        dst,
        src.offset(::core::mem::size_of::<uint8_t>() as usize as isize),
        &raw mut half as *mut uint8_t,
        stride,
        stride,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        8 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel8_mc01_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 104] = [0; 104];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut half: [uint8_t; 64] = [0; 64];
    copy_block8_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel8_v_lowpass_8(
        &raw mut half as *mut uint8_t,
        full_mid,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    avg_pixels8_l2_8(
        dst,
        full_mid,
        &raw mut half as *mut uint8_t,
        stride,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        8 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel8_mc02_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 104] = [0; 104];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    copy_block8_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    avg_h264_qpel8_v_lowpass_8(
        dst,
        full_mid,
        stride as ::core::ffi::c_int,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel8_mc03_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 104] = [0; 104];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut half: [uint8_t; 64] = [0; 64];
    copy_block8_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel8_v_lowpass_8(
        &raw mut half as *mut uint8_t,
        full_mid,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    avg_pixels8_l2_8(
        dst,
        full_mid
            .offset(
                (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        &raw mut half as *mut uint8_t,
        stride,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        8 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel8_mc11_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 104] = [0; 104];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut halfH: [uint8_t; 64] = [0; 64];
    let mut halfV: [uint8_t; 64] = [0; 64];
    put_h264_qpel8_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    copy_block8_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel8_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    avg_pixels8_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfV as *mut uint8_t,
        stride,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        8 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel8_mc31_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 104] = [0; 104];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut halfH: [uint8_t; 64] = [0; 64];
    let mut halfV: [uint8_t; 64] = [0; 64];
    put_h264_qpel8_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    copy_block8_8(
        &raw mut full as *mut uint8_t,
        src
            .offset(-((stride * 2 as ptrdiff_t) as isize))
            .offset(::core::mem::size_of::<uint8_t>() as usize as isize),
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel8_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    avg_pixels8_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfV as *mut uint8_t,
        stride,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        8 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel8_mc13_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 104] = [0; 104];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut halfH: [uint8_t; 64] = [0; 64];
    let mut halfV: [uint8_t; 64] = [0; 64];
    put_h264_qpel8_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src.offset(stride as isize),
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    copy_block8_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel8_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    avg_pixels8_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfV as *mut uint8_t,
        stride,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        8 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel8_mc33_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 104] = [0; 104];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut halfH: [uint8_t; 64] = [0; 64];
    let mut halfV: [uint8_t; 64] = [0; 64];
    put_h264_qpel8_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src.offset(stride as isize),
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    copy_block8_8(
        &raw mut full as *mut uint8_t,
        src
            .offset(-((stride * 2 as ptrdiff_t) as isize))
            .offset(::core::mem::size_of::<uint8_t>() as usize as isize),
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel8_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    avg_pixels8_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfV as *mut uint8_t,
        stride,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        8 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel8_mc22_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut tmp: [int16_t; 104] = [0; 104];
    avg_h264_qpel8_hv_lowpass_8(
        dst,
        &raw mut tmp as *mut int16_t,
        src,
        stride as ::core::ffi::c_int,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel8_mc21_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut tmp: [int16_t; 104] = [0; 104];
    let mut halfH: [uint8_t; 64] = [0; 64];
    let mut halfHV: [uint8_t; 64] = [0; 64];
    put_h264_qpel8_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    put_h264_qpel8_hv_lowpass_8(
        &raw mut halfHV as *mut uint8_t,
        &raw mut tmp as *mut int16_t,
        src,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    avg_pixels8_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfHV as *mut uint8_t,
        stride,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        8 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel8_mc23_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut tmp: [int16_t; 104] = [0; 104];
    let mut halfH: [uint8_t; 64] = [0; 64];
    let mut halfHV: [uint8_t; 64] = [0; 64];
    put_h264_qpel8_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src.offset(stride as isize),
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    put_h264_qpel8_hv_lowpass_8(
        &raw mut halfHV as *mut uint8_t,
        &raw mut tmp as *mut int16_t,
        src,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    avg_pixels8_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfHV as *mut uint8_t,
        stride,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        8 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel8_mc12_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 104] = [0; 104];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut tmp: [int16_t; 104] = [0; 104];
    let mut halfV: [uint8_t; 64] = [0; 64];
    let mut halfHV: [uint8_t; 64] = [0; 64];
    copy_block8_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel8_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    put_h264_qpel8_hv_lowpass_8(
        &raw mut halfHV as *mut uint8_t,
        &raw mut tmp as *mut int16_t,
        src,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    avg_pixels8_l2_8(
        dst,
        &raw mut halfV as *mut uint8_t,
        &raw mut halfHV as *mut uint8_t,
        stride,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        8 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel8_mc32_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 104] = [0; 104];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut tmp: [int16_t; 104] = [0; 104];
    let mut halfV: [uint8_t; 64] = [0; 64];
    let mut halfHV: [uint8_t; 64] = [0; 64];
    copy_block8_8(
        &raw mut full as *mut uint8_t,
        src
            .offset(-((stride * 2 as ptrdiff_t) as isize))
            .offset(::core::mem::size_of::<uint8_t>() as usize as isize),
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel8_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    put_h264_qpel8_hv_lowpass_8(
        &raw mut halfHV as *mut uint8_t,
        &raw mut tmp as *mut int16_t,
        src,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    avg_pixels8_l2_8(
        dst,
        &raw mut halfV as *mut uint8_t,
        &raw mut halfHV as *mut uint8_t,
        stride,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (8 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        8 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel16_mc00_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    avg_pixels16_8_c(dst, src, stride, 16 as ::core::ffi::c_int);
}




unsafe extern "C" fn avg_h264_qpel16_mc10_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut half: [uint8_t; 256] = [0; 256];
    put_h264_qpel16_h_lowpass_8(
        &raw mut half as *mut uint8_t,
        src,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    avg_pixels16_l2_8(
        dst,
        src,
        &raw mut half as *mut uint8_t,
        stride,
        stride,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        16 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel16_mc20_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    avg_h264_qpel16_h_lowpass_8(
        dst,
        src,
        stride as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel16_mc30_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut half: [uint8_t; 256] = [0; 256];
    put_h264_qpel16_h_lowpass_8(
        &raw mut half as *mut uint8_t,
        src,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    avg_pixels16_l2_8(
        dst,
        src.offset(::core::mem::size_of::<uint8_t>() as usize as isize),
        &raw mut half as *mut uint8_t,
        stride,
        stride,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        16 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel16_mc01_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 336] = [0; 336];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut half: [uint8_t; 256] = [0; 256];
    copy_block16_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel16_v_lowpass_8(
        &raw mut half as *mut uint8_t,
        full_mid,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    avg_pixels16_l2_8(
        dst,
        full_mid,
        &raw mut half as *mut uint8_t,
        stride,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        16 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel16_mc02_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 336] = [0; 336];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    copy_block16_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    avg_h264_qpel16_v_lowpass_8(
        dst,
        full_mid,
        stride as ::core::ffi::c_int,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel16_mc03_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 336] = [0; 336];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut half: [uint8_t; 256] = [0; 256];
    copy_block16_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel16_v_lowpass_8(
        &raw mut half as *mut uint8_t,
        full_mid,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    avg_pixels16_l2_8(
        dst,
        full_mid
            .offset(
                (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                    as isize,
            ),
        &raw mut half as *mut uint8_t,
        stride,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        16 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel16_mc11_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 336] = [0; 336];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut halfH: [uint8_t; 256] = [0; 256];
    let mut halfV: [uint8_t; 256] = [0; 256];
    put_h264_qpel16_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    copy_block16_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel16_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    avg_pixels16_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfV as *mut uint8_t,
        stride,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        16 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel16_mc31_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 336] = [0; 336];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut halfH: [uint8_t; 256] = [0; 256];
    let mut halfV: [uint8_t; 256] = [0; 256];
    put_h264_qpel16_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    copy_block16_8(
        &raw mut full as *mut uint8_t,
        src
            .offset(-((stride * 2 as ptrdiff_t) as isize))
            .offset(::core::mem::size_of::<uint8_t>() as usize as isize),
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel16_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    avg_pixels16_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfV as *mut uint8_t,
        stride,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        16 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel16_mc13_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 336] = [0; 336];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut halfH: [uint8_t; 256] = [0; 256];
    let mut halfV: [uint8_t; 256] = [0; 256];
    put_h264_qpel16_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src.offset(stride as isize),
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    copy_block16_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel16_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    avg_pixels16_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfV as *mut uint8_t,
        stride,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        16 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel16_mc33_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 336] = [0; 336];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut halfH: [uint8_t; 256] = [0; 256];
    let mut halfV: [uint8_t; 256] = [0; 256];
    put_h264_qpel16_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src.offset(stride as isize),
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    copy_block16_8(
        &raw mut full as *mut uint8_t,
        src
            .offset(-((stride * 2 as ptrdiff_t) as isize))
            .offset(::core::mem::size_of::<uint8_t>() as usize as isize),
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel16_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    avg_pixels16_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfV as *mut uint8_t,
        stride,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        16 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel16_mc22_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut tmp: [int16_t; 336] = [0; 336];
    avg_h264_qpel16_hv_lowpass_8(
        dst,
        &raw mut tmp as *mut int16_t,
        src,
        stride as ::core::ffi::c_int,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel16_mc21_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut tmp: [int16_t; 336] = [0; 336];
    let mut halfH: [uint8_t; 256] = [0; 256];
    let mut halfHV: [uint8_t; 256] = [0; 256];
    put_h264_qpel16_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    put_h264_qpel16_hv_lowpass_8(
        &raw mut halfHV as *mut uint8_t,
        &raw mut tmp as *mut int16_t,
        src,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    avg_pixels16_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfHV as *mut uint8_t,
        stride,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        16 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel16_mc23_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut tmp: [int16_t; 336] = [0; 336];
    let mut halfH: [uint8_t; 256] = [0; 256];
    let mut halfHV: [uint8_t; 256] = [0; 256];
    put_h264_qpel16_h_lowpass_8(
        &raw mut halfH as *mut uint8_t,
        src.offset(stride as isize),
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    put_h264_qpel16_hv_lowpass_8(
        &raw mut halfHV as *mut uint8_t,
        &raw mut tmp as *mut int16_t,
        src,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    avg_pixels16_l2_8(
        dst,
        &raw mut halfH as *mut uint8_t,
        &raw mut halfHV as *mut uint8_t,
        stride,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        16 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel16_mc12_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 336] = [0; 336];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut tmp: [int16_t; 336] = [0; 336];
    let mut halfV: [uint8_t; 256] = [0; 256];
    let mut halfHV: [uint8_t; 256] = [0; 256];
    copy_block16_8(
        &raw mut full as *mut uint8_t,
        src.offset(-((stride * 2 as ptrdiff_t) as isize)),
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel16_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    put_h264_qpel16_hv_lowpass_8(
        &raw mut halfHV as *mut uint8_t,
        &raw mut tmp as *mut int16_t,
        src,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    avg_pixels16_l2_8(
        dst,
        &raw mut halfV as *mut uint8_t,
        &raw mut halfHV as *mut uint8_t,
        stride,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        16 as ::core::ffi::c_int,
    );
}




unsafe extern "C" fn avg_h264_qpel16_mc32_8_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut stride: ptrdiff_t,
) {
    let mut full: [uint8_t; 336] = [0; 336];
    let full_mid: *mut uint8_t = (&raw mut full as *mut uint8_t)
        .offset(
            ((16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
        );
    let mut tmp: [int16_t; 336] = [0; 336];
    let mut halfV: [uint8_t; 256] = [0; 256];
    let mut halfHV: [uint8_t; 256] = [0; 256];
    copy_block16_8(
        &raw mut full as *mut uint8_t,
        src
            .offset(-((stride * 2 as ptrdiff_t) as isize))
            .offset(::core::mem::size_of::<uint8_t>() as usize as isize),
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
    );
    put_h264_qpel16_v_lowpass_8(
        &raw mut halfV as *mut uint8_t,
        full_mid,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
    );
    put_h264_qpel16_hv_lowpass_8(
        &raw mut halfHV as *mut uint8_t,
        &raw mut tmp as *mut int16_t,
        src,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
    );
    avg_pixels16_l2_8(
        dst,
        &raw mut halfV as *mut uint8_t,
        &raw mut halfHV as *mut uint8_t,
        stride,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        (16 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
            as ptrdiff_t,
        16 as ::core::ffi::c_int,
    );
}
