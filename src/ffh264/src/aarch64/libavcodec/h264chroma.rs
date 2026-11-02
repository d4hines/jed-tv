// Translated from FFmpeg 9.0.1 libavcodec/h264chroma.c
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

extern "C" {
    fn ff_h264chroma_init_aarch64(
        c: *mut H264ChromaContext,
        bit_depth: ::core::ffi::c_int,
    );
}
pub type ptrdiff_t = ::core::ffi::c_long;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint16_t = ::core::ffi::c_ushort;
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
pub unsafe extern "C" fn ff_h264chroma_init(
    mut c: *mut H264ChromaContext,
    mut bit_depth: ::core::ffi::c_int,
) {
    {
        (*c).put_h264_chroma_pixels_tab[0 as ::core::ffi::c_int as usize] = Some(
            put_h264_chroma_mc8_8_c
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
            put_h264_chroma_mc4_8_c
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
            ff_put_h264_chroma_mc2_8_c
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
            avg_h264_chroma_mc8_8_c
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
            avg_h264_chroma_mc4_8_c
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
            ff_avg_h264_chroma_mc2_8_c
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
    ff_h264chroma_init_aarch64(c, bit_depth);
}

#[no_mangle]
pub unsafe extern "C" fn ff_put_h264_chroma_mc2_8_c(
    mut _dst: *mut uint8_t,
    mut _src: *const uint8_t,
    mut stride: ptrdiff_t,
    mut h: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) {
    let mut dst: *mut uint8_t = _dst;
    let mut src: *const uint8_t = _src;
    let A: ::core::ffi::c_int = (8 as ::core::ffi::c_int - x)
        * (8 as ::core::ffi::c_int - y);
    let B: ::core::ffi::c_int = x * (8 as ::core::ffi::c_int - y);
    let C: ::core::ffi::c_int = (8 as ::core::ffi::c_int - x) * y;
    let D: ::core::ffi::c_int = x * y;
    let mut i: ::core::ffi::c_int = 0;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    if D != 0 {
        i = 0 as ::core::ffi::c_int;
        while i < h {
            *dst.offset(0 as ::core::ffi::c_int as isize) = (A
                * *src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + B * *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + C
                    * *src.offset((stride + 0 as ptrdiff_t) as isize)
                        as ::core::ffi::c_int
                + D
                    * *src.offset((stride + 1 as ptrdiff_t) as isize)
                        as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(1 as ::core::ffi::c_int as isize) = (A
                * *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + B * *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + C
                    * *src.offset((stride + 1 as ptrdiff_t) as isize)
                        as ::core::ffi::c_int
                + D
                    * *src.offset((stride + 2 as ptrdiff_t) as isize)
                        as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                >> 6 as ::core::ffi::c_int) as uint8_t;
            dst = dst.offset(stride as isize);
            src = src.offset(stride as isize);
            i += 1;
        }
    } else if B + C != 0 {
        let E: ::core::ffi::c_int = B + C;
        let step: ptrdiff_t = if C != 0 { stride } else { 1 as ptrdiff_t };
        i = 0 as ::core::ffi::c_int;
        while i < h {
            *dst.offset(0 as ::core::ffi::c_int as isize) = (A
                * *src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + E * *src.offset((step + 0 as ptrdiff_t) as isize) as ::core::ffi::c_int
                + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(1 as ::core::ffi::c_int as isize) = (A
                * *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + E * *src.offset((step + 1 as ptrdiff_t) as isize) as ::core::ffi::c_int
                + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as uint8_t;
            dst = dst.offset(stride as isize);
            src = src.offset(stride as isize);
            i += 1;
        }
    } else {
        i = 0 as ::core::ffi::c_int;
        while i < h {
            *dst.offset(0 as ::core::ffi::c_int as isize) = (A
                * *src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(1 as ::core::ffi::c_int as isize) = (A
                * *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as uint8_t;
            dst = dst.offset(stride as isize);
            src = src.offset(stride as isize);
            i += 1;
        }
    };
}
unsafe extern "C" fn put_h264_chroma_mc4_8_c(
    mut _dst: *mut uint8_t,
    mut _src: *const uint8_t,
    mut stride: ptrdiff_t,
    mut h: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) {
    let mut dst: *mut uint8_t = _dst;
    let mut src: *const uint8_t = _src;
    let A: ::core::ffi::c_int = (8 as ::core::ffi::c_int - x)
        * (8 as ::core::ffi::c_int - y);
    let B: ::core::ffi::c_int = x * (8 as ::core::ffi::c_int - y);
    let C: ::core::ffi::c_int = (8 as ::core::ffi::c_int - x) * y;
    let D: ::core::ffi::c_int = x * y;
    let mut i: ::core::ffi::c_int = 0;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    if D != 0 {
        i = 0 as ::core::ffi::c_int;
        while i < h {
            *dst.offset(0 as ::core::ffi::c_int as isize) = (A
                * *src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + B * *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + C
                    * *src.offset((stride + 0 as ptrdiff_t) as isize)
                        as ::core::ffi::c_int
                + D
                    * *src.offset((stride + 1 as ptrdiff_t) as isize)
                        as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(1 as ::core::ffi::c_int as isize) = (A
                * *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + B * *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + C
                    * *src.offset((stride + 1 as ptrdiff_t) as isize)
                        as ::core::ffi::c_int
                + D
                    * *src.offset((stride + 2 as ptrdiff_t) as isize)
                        as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(2 as ::core::ffi::c_int as isize) = (A
                * *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + B * *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + C
                    * *src.offset((stride + 2 as ptrdiff_t) as isize)
                        as ::core::ffi::c_int
                + D
                    * *src.offset((stride + 3 as ptrdiff_t) as isize)
                        as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(3 as ::core::ffi::c_int as isize) = (A
                * *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + B * *src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + C
                    * *src.offset((stride + 3 as ptrdiff_t) as isize)
                        as ::core::ffi::c_int
                + D
                    * *src.offset((stride + 4 as ptrdiff_t) as isize)
                        as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                >> 6 as ::core::ffi::c_int) as uint8_t;
            dst = dst.offset(stride as isize);
            src = src.offset(stride as isize);
            i += 1;
        }
    } else if B + C != 0 {
        let E: ::core::ffi::c_int = B + C;
        let step: ptrdiff_t = if C != 0 { stride } else { 1 as ptrdiff_t };
        i = 0 as ::core::ffi::c_int;
        while i < h {
            *dst.offset(0 as ::core::ffi::c_int as isize) = (A
                * *src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + E * *src.offset((step + 0 as ptrdiff_t) as isize) as ::core::ffi::c_int
                + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(1 as ::core::ffi::c_int as isize) = (A
                * *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + E * *src.offset((step + 1 as ptrdiff_t) as isize) as ::core::ffi::c_int
                + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(2 as ::core::ffi::c_int as isize) = (A
                * *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + E * *src.offset((step + 2 as ptrdiff_t) as isize) as ::core::ffi::c_int
                + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(3 as ::core::ffi::c_int as isize) = (A
                * *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + E * *src.offset((step + 3 as ptrdiff_t) as isize) as ::core::ffi::c_int
                + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as uint8_t;
            dst = dst.offset(stride as isize);
            src = src.offset(stride as isize);
            i += 1;
        }
    } else {
        i = 0 as ::core::ffi::c_int;
        while i < h {
            *dst.offset(0 as ::core::ffi::c_int as isize) = (A
                * *src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(1 as ::core::ffi::c_int as isize) = (A
                * *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(2 as ::core::ffi::c_int as isize) = (A
                * *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(3 as ::core::ffi::c_int as isize) = (A
                * *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as uint8_t;
            dst = dst.offset(stride as isize);
            src = src.offset(stride as isize);
            i += 1;
        }
    };
}

unsafe extern "C" fn put_h264_chroma_mc8_8_c(
    mut _dst: *mut uint8_t,
    mut _src: *const uint8_t,
    mut stride: ptrdiff_t,
    mut h: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) {
    let mut dst: *mut uint8_t = _dst;
    let mut src: *const uint8_t = _src;
    let A: ::core::ffi::c_int = (8 as ::core::ffi::c_int - x)
        * (8 as ::core::ffi::c_int - y);
    let B: ::core::ffi::c_int = x * (8 as ::core::ffi::c_int - y);
    let C: ::core::ffi::c_int = (8 as ::core::ffi::c_int - x) * y;
    let D: ::core::ffi::c_int = x * y;
    let mut i: ::core::ffi::c_int = 0;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    if D != 0 {
        i = 0 as ::core::ffi::c_int;
        while i < h {
            *dst.offset(0 as ::core::ffi::c_int as isize) = (A
                * *src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + B * *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + C
                    * *src.offset((stride + 0 as ptrdiff_t) as isize)
                        as ::core::ffi::c_int
                + D
                    * *src.offset((stride + 1 as ptrdiff_t) as isize)
                        as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(1 as ::core::ffi::c_int as isize) = (A
                * *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + B * *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + C
                    * *src.offset((stride + 1 as ptrdiff_t) as isize)
                        as ::core::ffi::c_int
                + D
                    * *src.offset((stride + 2 as ptrdiff_t) as isize)
                        as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(2 as ::core::ffi::c_int as isize) = (A
                * *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + B * *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + C
                    * *src.offset((stride + 2 as ptrdiff_t) as isize)
                        as ::core::ffi::c_int
                + D
                    * *src.offset((stride + 3 as ptrdiff_t) as isize)
                        as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(3 as ::core::ffi::c_int as isize) = (A
                * *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + B * *src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + C
                    * *src.offset((stride + 3 as ptrdiff_t) as isize)
                        as ::core::ffi::c_int
                + D
                    * *src.offset((stride + 4 as ptrdiff_t) as isize)
                        as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(4 as ::core::ffi::c_int as isize) = (A
                * *src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + B * *src.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + C
                    * *src.offset((stride + 4 as ptrdiff_t) as isize)
                        as ::core::ffi::c_int
                + D
                    * *src.offset((stride + 5 as ptrdiff_t) as isize)
                        as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(5 as ::core::ffi::c_int as isize) = (A
                * *src.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + B * *src.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + C
                    * *src.offset((stride + 5 as ptrdiff_t) as isize)
                        as ::core::ffi::c_int
                + D
                    * *src.offset((stride + 6 as ptrdiff_t) as isize)
                        as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(6 as ::core::ffi::c_int as isize) = (A
                * *src.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + B * *src.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + C
                    * *src.offset((stride + 6 as ptrdiff_t) as isize)
                        as ::core::ffi::c_int
                + D
                    * *src.offset((stride + 7 as ptrdiff_t) as isize)
                        as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(7 as ::core::ffi::c_int as isize) = (A
                * *src.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + B * *src.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + C
                    * *src.offset((stride + 7 as ptrdiff_t) as isize)
                        as ::core::ffi::c_int
                + D
                    * *src.offset((stride + 8 as ptrdiff_t) as isize)
                        as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                >> 6 as ::core::ffi::c_int) as uint8_t;
            dst = dst.offset(stride as isize);
            src = src.offset(stride as isize);
            i += 1;
        }
    } else if B + C != 0 {
        let E: ::core::ffi::c_int = B + C;
        let step: ptrdiff_t = if C != 0 { stride } else { 1 as ptrdiff_t };
        i = 0 as ::core::ffi::c_int;
        while i < h {
            *dst.offset(0 as ::core::ffi::c_int as isize) = (A
                * *src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + E * *src.offset((step + 0 as ptrdiff_t) as isize) as ::core::ffi::c_int
                + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(1 as ::core::ffi::c_int as isize) = (A
                * *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + E * *src.offset((step + 1 as ptrdiff_t) as isize) as ::core::ffi::c_int
                + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(2 as ::core::ffi::c_int as isize) = (A
                * *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + E * *src.offset((step + 2 as ptrdiff_t) as isize) as ::core::ffi::c_int
                + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(3 as ::core::ffi::c_int as isize) = (A
                * *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + E * *src.offset((step + 3 as ptrdiff_t) as isize) as ::core::ffi::c_int
                + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(4 as ::core::ffi::c_int as isize) = (A
                * *src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + E * *src.offset((step + 4 as ptrdiff_t) as isize) as ::core::ffi::c_int
                + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(5 as ::core::ffi::c_int as isize) = (A
                * *src.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + E * *src.offset((step + 5 as ptrdiff_t) as isize) as ::core::ffi::c_int
                + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(6 as ::core::ffi::c_int as isize) = (A
                * *src.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + E * *src.offset((step + 6 as ptrdiff_t) as isize) as ::core::ffi::c_int
                + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(7 as ::core::ffi::c_int as isize) = (A
                * *src.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + E * *src.offset((step + 7 as ptrdiff_t) as isize) as ::core::ffi::c_int
                + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as uint8_t;
            dst = dst.offset(stride as isize);
            src = src.offset(stride as isize);
            i += 1;
        }
    } else {
        i = 0 as ::core::ffi::c_int;
        while i < h {
            *dst.offset(0 as ::core::ffi::c_int as isize) = (A
                * *src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(1 as ::core::ffi::c_int as isize) = (A
                * *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(2 as ::core::ffi::c_int as isize) = (A
                * *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(3 as ::core::ffi::c_int as isize) = (A
                * *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(4 as ::core::ffi::c_int as isize) = (A
                * *src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(5 as ::core::ffi::c_int as isize) = (A
                * *src.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(6 as ::core::ffi::c_int as isize) = (A
                * *src.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(7 as ::core::ffi::c_int as isize) = (A
                * *src.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as uint8_t;
            dst = dst.offset(stride as isize);
            src = src.offset(stride as isize);
            i += 1;
        }
    };
}


#[no_mangle]
pub unsafe extern "C" fn ff_avg_h264_chroma_mc2_8_c(
    mut _dst: *mut uint8_t,
    mut _src: *const uint8_t,
    mut stride: ptrdiff_t,
    mut h: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) {
    let mut dst: *mut uint8_t = _dst;
    let mut src: *const uint8_t = _src;
    let A: ::core::ffi::c_int = (8 as ::core::ffi::c_int - x)
        * (8 as ::core::ffi::c_int - y);
    let B: ::core::ffi::c_int = x * (8 as ::core::ffi::c_int - y);
    let C: ::core::ffi::c_int = (8 as ::core::ffi::c_int - x) * y;
    let D: ::core::ffi::c_int = x * y;
    let mut i: ::core::ffi::c_int = 0;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    if D != 0 {
        i = 0 as ::core::ffi::c_int;
        while i < h {
            *dst.offset(0 as ::core::ffi::c_int as isize) = (*dst
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + B
                        * *src.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                    + C
                        * *src.offset((stride + 0 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int
                    + D
                        * *src.offset((stride + 1 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(1 as ::core::ffi::c_int as isize) = (*dst
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + B
                        * *src.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                    + C
                        * *src.offset((stride + 1 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int
                    + D
                        * *src.offset((stride + 2 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            dst = dst.offset(stride as isize);
            src = src.offset(stride as isize);
            i += 1;
        }
    } else if B + C != 0 {
        let E: ::core::ffi::c_int = B + C;
        let step: ptrdiff_t = if C != 0 { stride } else { 1 as ptrdiff_t };
        i = 0 as ::core::ffi::c_int;
        while i < h {
            *dst.offset(0 as ::core::ffi::c_int as isize) = (*dst
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + E
                        * *src.offset((step + 0 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(1 as ::core::ffi::c_int as isize) = (*dst
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + E
                        * *src.offset((step + 1 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            dst = dst.offset(stride as isize);
            src = src.offset(stride as isize);
            i += 1;
        }
    } else {
        i = 0 as ::core::ffi::c_int;
        while i < h {
            *dst.offset(0 as ::core::ffi::c_int as isize) = (*dst
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(1 as ::core::ffi::c_int as isize) = (*dst
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
            dst = dst.offset(stride as isize);
            src = src.offset(stride as isize);
            i += 1;
        }
    };
}
unsafe extern "C" fn avg_h264_chroma_mc4_8_c(
    mut _dst: *mut uint8_t,
    mut _src: *const uint8_t,
    mut stride: ptrdiff_t,
    mut h: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) {
    let mut dst: *mut uint8_t = _dst;
    let mut src: *const uint8_t = _src;
    let A: ::core::ffi::c_int = (8 as ::core::ffi::c_int - x)
        * (8 as ::core::ffi::c_int - y);
    let B: ::core::ffi::c_int = x * (8 as ::core::ffi::c_int - y);
    let C: ::core::ffi::c_int = (8 as ::core::ffi::c_int - x) * y;
    let D: ::core::ffi::c_int = x * y;
    let mut i: ::core::ffi::c_int = 0;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    if D != 0 {
        i = 0 as ::core::ffi::c_int;
        while i < h {
            *dst.offset(0 as ::core::ffi::c_int as isize) = (*dst
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + B
                        * *src.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                    + C
                        * *src.offset((stride + 0 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int
                    + D
                        * *src.offset((stride + 1 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(1 as ::core::ffi::c_int as isize) = (*dst
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + B
                        * *src.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                    + C
                        * *src.offset((stride + 1 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int
                    + D
                        * *src.offset((stride + 2 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(2 as ::core::ffi::c_int as isize) = (*dst
                .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + B
                        * *src.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                    + C
                        * *src.offset((stride + 2 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int
                    + D
                        * *src.offset((stride + 3 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(3 as ::core::ffi::c_int as isize) = (*dst
                .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + B
                        * *src.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                    + C
                        * *src.offset((stride + 3 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int
                    + D
                        * *src.offset((stride + 4 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            dst = dst.offset(stride as isize);
            src = src.offset(stride as isize);
            i += 1;
        }
    } else if B + C != 0 {
        let E: ::core::ffi::c_int = B + C;
        let step: ptrdiff_t = if C != 0 { stride } else { 1 as ptrdiff_t };
        i = 0 as ::core::ffi::c_int;
        while i < h {
            *dst.offset(0 as ::core::ffi::c_int as isize) = (*dst
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + E
                        * *src.offset((step + 0 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(1 as ::core::ffi::c_int as isize) = (*dst
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + E
                        * *src.offset((step + 1 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(2 as ::core::ffi::c_int as isize) = (*dst
                .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + E
                        * *src.offset((step + 2 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(3 as ::core::ffi::c_int as isize) = (*dst
                .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + E
                        * *src.offset((step + 3 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            dst = dst.offset(stride as isize);
            src = src.offset(stride as isize);
            i += 1;
        }
    } else {
        i = 0 as ::core::ffi::c_int;
        while i < h {
            *dst.offset(0 as ::core::ffi::c_int as isize) = (*dst
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(1 as ::core::ffi::c_int as isize) = (*dst
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(2 as ::core::ffi::c_int as isize) = (*dst
                .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(3 as ::core::ffi::c_int as isize) = (*dst
                .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
            dst = dst.offset(stride as isize);
            src = src.offset(stride as isize);
            i += 1;
        }
    };
}

unsafe extern "C" fn avg_h264_chroma_mc8_8_c(
    mut _dst: *mut uint8_t,
    mut _src: *const uint8_t,
    mut stride: ptrdiff_t,
    mut h: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) {
    let mut dst: *mut uint8_t = _dst;
    let mut src: *const uint8_t = _src;
    let A: ::core::ffi::c_int = (8 as ::core::ffi::c_int - x)
        * (8 as ::core::ffi::c_int - y);
    let B: ::core::ffi::c_int = x * (8 as ::core::ffi::c_int - y);
    let C: ::core::ffi::c_int = (8 as ::core::ffi::c_int - x) * y;
    let D: ::core::ffi::c_int = x * y;
    let mut i: ::core::ffi::c_int = 0;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    if D != 0 {
        i = 0 as ::core::ffi::c_int;
        while i < h {
            *dst.offset(0 as ::core::ffi::c_int as isize) = (*dst
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + B
                        * *src.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                    + C
                        * *src.offset((stride + 0 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int
                    + D
                        * *src.offset((stride + 1 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(1 as ::core::ffi::c_int as isize) = (*dst
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + B
                        * *src.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                    + C
                        * *src.offset((stride + 1 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int
                    + D
                        * *src.offset((stride + 2 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(2 as ::core::ffi::c_int as isize) = (*dst
                .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + B
                        * *src.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                    + C
                        * *src.offset((stride + 2 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int
                    + D
                        * *src.offset((stride + 3 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(3 as ::core::ffi::c_int as isize) = (*dst
                .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + B
                        * *src.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                    + C
                        * *src.offset((stride + 3 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int
                    + D
                        * *src.offset((stride + 4 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(4 as ::core::ffi::c_int as isize) = (*dst
                .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + B
                        * *src.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                    + C
                        * *src.offset((stride + 4 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int
                    + D
                        * *src.offset((stride + 5 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(5 as ::core::ffi::c_int as isize) = (*dst
                .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + B
                        * *src.offset(6 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                    + C
                        * *src.offset((stride + 5 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int
                    + D
                        * *src.offset((stride + 6 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(6 as ::core::ffi::c_int as isize) = (*dst
                .offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + B
                        * *src.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                    + C
                        * *src.offset((stride + 6 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int
                    + D
                        * *src.offset((stride + 7 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(7 as ::core::ffi::c_int as isize) = (*dst
                .offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + B
                        * *src.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                    + C
                        * *src.offset((stride + 7 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int
                    + D
                        * *src.offset((stride + 8 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            dst = dst.offset(stride as isize);
            src = src.offset(stride as isize);
            i += 1;
        }
    } else if B + C != 0 {
        let E: ::core::ffi::c_int = B + C;
        let step: ptrdiff_t = if C != 0 { stride } else { 1 as ptrdiff_t };
        i = 0 as ::core::ffi::c_int;
        while i < h {
            *dst.offset(0 as ::core::ffi::c_int as isize) = (*dst
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + E
                        * *src.offset((step + 0 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(1 as ::core::ffi::c_int as isize) = (*dst
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + E
                        * *src.offset((step + 1 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(2 as ::core::ffi::c_int as isize) = (*dst
                .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + E
                        * *src.offset((step + 2 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(3 as ::core::ffi::c_int as isize) = (*dst
                .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + E
                        * *src.offset((step + 3 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(4 as ::core::ffi::c_int as isize) = (*dst
                .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + E
                        * *src.offset((step + 4 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(5 as ::core::ffi::c_int as isize) = (*dst
                .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + E
                        * *src.offset((step + 5 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(6 as ::core::ffi::c_int as isize) = (*dst
                .offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + E
                        * *src.offset((step + 6 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(7 as ::core::ffi::c_int as isize) = (*dst
                .offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + E
                        * *src.offset((step + 7 as ptrdiff_t) as isize)
                            as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            dst = dst.offset(stride as isize);
            src = src.offset(stride as isize);
            i += 1;
        }
    } else {
        i = 0 as ::core::ffi::c_int;
        while i < h {
            *dst.offset(0 as ::core::ffi::c_int as isize) = (*dst
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(1 as ::core::ffi::c_int as isize) = (*dst
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(2 as ::core::ffi::c_int as isize) = (*dst
                .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(3 as ::core::ffi::c_int as isize) = (*dst
                .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(4 as ::core::ffi::c_int as isize) = (*dst
                .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(5 as ::core::ffi::c_int as isize) = (*dst
                .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(6 as ::core::ffi::c_int as isize) = (*dst
                .offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
            *dst.offset(7 as ::core::ffi::c_int as isize) = (*dst
                .offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + (A
                    * *src.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint8_t;
            dst = dst.offset(stride as isize);
            src = src.offset(stride as isize);
            i += 1;
        }
    };
}
