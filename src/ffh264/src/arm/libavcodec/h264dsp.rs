// Translated from FFmpeg 9.0.1 libavcodec/h264dsp.c
// (preprocessed for armv7-unknown-linux-musleabihf) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
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
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn ff_startcode_find_candidate_c(
        buf: *const uint8_t,
        size: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ff_h264_idct8_add_8_c(
        dst: *mut uint8_t,
        block: *mut int16_t,
        stride: ::core::ffi::c_int,
    );
    fn ff_h264_idct_add_8_c(
        dst: *mut uint8_t,
        block: *mut int16_t,
        stride: ::core::ffi::c_int,
    );
    fn ff_h264_idct8_dc_add_8_c(
        dst: *mut uint8_t,
        block: *mut int16_t,
        stride: ::core::ffi::c_int,
    );
    fn ff_h264_idct_dc_add_8_c(
        dst: *mut uint8_t,
        block: *mut int16_t,
        stride: ::core::ffi::c_int,
    );
    fn ff_h264_idct_add16_8_c(
        dst: *mut uint8_t,
        blockoffset: *const ::core::ffi::c_int,
        block: *mut int16_t,
        stride: ::core::ffi::c_int,
        nnzc: *const uint8_t,
    );
    fn ff_h264_idct_add16intra_8_c(
        dst: *mut uint8_t,
        blockoffset: *const ::core::ffi::c_int,
        block: *mut int16_t,
        stride: ::core::ffi::c_int,
        nnzc: *const uint8_t,
    );
    fn ff_h264_idct8_add4_8_c(
        dst: *mut uint8_t,
        blockoffset: *const ::core::ffi::c_int,
        block: *mut int16_t,
        stride: ::core::ffi::c_int,
        nnzc: *const uint8_t,
    );
    fn ff_h264_idct_add8_422_8_c(
        dest: *mut *mut uint8_t,
        blockoffset: *const ::core::ffi::c_int,
        block: *mut int16_t,
        stride: ::core::ffi::c_int,
        nnzc: *const uint8_t,
    );
    fn ff_h264_idct_add8_8_c(
        dest: *mut *mut uint8_t,
        blockoffset: *const ::core::ffi::c_int,
        block: *mut int16_t,
        stride: ::core::ffi::c_int,
        nnzc: *const uint8_t,
    );
    fn ff_h264_luma_dc_dequant_idct_8_c(
        output: *mut int16_t,
        input: *mut int16_t,
        qmul: ::core::ffi::c_int,
    );
    fn abort() -> !;
    fn ff_h264_chroma422_dc_dequant_idct_8_c(
        block: *mut int16_t,
        qmul: ::core::ffi::c_int,
    );
    fn ff_h264_chroma_dc_dequant_idct_8_c(block: *mut int16_t, qmul: ::core::ffi::c_int);
    fn ff_h264dsp_init_arm(
        c: *mut H264DSPContext,
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
pub type int8_t = ::core::ffi::c_schar;
pub type int16_t = ::core::ffi::c_short;
pub type int32_t = ::core::ffi::c_int;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint16_t = ::core::ffi::c_ushort;
pub type size_t = ::core::ffi::c_uint;
pub type ptrdiff_t = ::core::ffi::c_int;
pub type h264_weight_func = Option<
    unsafe extern "C" fn(
        *mut uint8_t,
        ptrdiff_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> (),
>;
pub type h264_biweight_func = Option<
    unsafe extern "C" fn(
        *mut uint8_t,
        *mut uint8_t,
        ptrdiff_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264DSPContext {
    pub weight_pixels_tab: [h264_weight_func; 4],
    pub biweight_pixels_tab: [h264_biweight_func; 4],
    pub v_loop_filter_luma: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut int8_t,
        ) -> (),
    >,
    pub h_loop_filter_luma: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut int8_t,
        ) -> (),
    >,
    pub h_loop_filter_luma_mbaff: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut int8_t,
        ) -> (),
    >,
    pub v_loop_filter_luma_intra: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub h_loop_filter_luma_intra: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub h_loop_filter_luma_mbaff_intra: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub v_loop_filter_chroma: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut int8_t,
        ) -> (),
    >,
    pub h_loop_filter_chroma: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut int8_t,
        ) -> (),
    >,
    pub h_loop_filter_chroma_mbaff: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut int8_t,
        ) -> (),
    >,
    pub v_loop_filter_chroma_intra: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub h_loop_filter_chroma_intra: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub h_loop_filter_chroma_mbaff_intra: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub loop_filter_strength: Option<
        unsafe extern "C" fn(
            *mut [[int16_t; 4]; 4],
            *mut uint8_t,
            *mut [int8_t; 40],
            *mut [[int16_t; 2]; 40],
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub idct_add: Option<
        unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ::core::ffi::c_int) -> (),
    >,
    pub idct8_add: Option<
        unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ::core::ffi::c_int) -> (),
    >,
    pub idct_dc_add: Option<
        unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ::core::ffi::c_int) -> (),
    >,
    pub idct8_dc_add: Option<
        unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ::core::ffi::c_int) -> (),
    >,
    pub idct_add16: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            *const ::core::ffi::c_int,
            *mut int16_t,
            ::core::ffi::c_int,
            *const uint8_t,
        ) -> (),
    >,
    pub idct8_add4: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            *const ::core::ffi::c_int,
            *mut int16_t,
            ::core::ffi::c_int,
            *const uint8_t,
        ) -> (),
    >,
    pub idct_add8: Option<
        unsafe extern "C" fn(
            *mut *mut uint8_t,
            *const ::core::ffi::c_int,
            *mut int16_t,
            ::core::ffi::c_int,
            *const uint8_t,
        ) -> (),
    >,
    pub idct_add16intra: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            *const ::core::ffi::c_int,
            *mut int16_t,
            ::core::ffi::c_int,
            *const uint8_t,
        ) -> (),
    >,
    pub luma_dc_dequant_idct: Option<
        unsafe extern "C" fn(*mut int16_t, *mut int16_t, ::core::ffi::c_int) -> (),
    >,
    pub chroma_dc_dequant_idct: Option<
        unsafe extern "C" fn(*mut int16_t, ::core::ffi::c_int) -> (),
    >,
    pub add_pixels8_clear: Option<
        unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ::core::ffi::c_int) -> (),
    >,
    pub add_pixels4_clear: Option<
        unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ::core::ffi::c_int) -> (),
    >,
    pub startcode_find_candidate: Option<
        unsafe extern "C" fn(*const uint8_t, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
}

unsafe extern "C" fn ff_h264_add_pixels4_8_c(
    mut _dst: *mut uint8_t,
    mut _src: *mut int16_t,
    mut stride: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut dst: *mut uint8_t = _dst;
    let mut src: *mut int16_t = _src;
    stride = (stride as ::core::ffi::c_uint)
        .wrapping_div(::core::mem::size_of::<uint8_t>() as usize as ::core::ffi::c_uint)
        as ::core::ffi::c_int as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        let ref mut fresh8 = *dst.offset(0 as ::core::ffi::c_int as isize);
        *fresh8 = (*fresh8 as ::core::ffi::c_uint)
            .wrapping_add(
                *src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
            ) as uint8_t as uint8_t;
        let ref mut fresh9 = *dst.offset(1 as ::core::ffi::c_int as isize);
        *fresh9 = (*fresh9 as ::core::ffi::c_uint)
            .wrapping_add(
                *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
            ) as uint8_t as uint8_t;
        let ref mut fresh10 = *dst.offset(2 as ::core::ffi::c_int as isize);
        *fresh10 = (*fresh10 as ::core::ffi::c_uint)
            .wrapping_add(
                *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
            ) as uint8_t as uint8_t;
        let ref mut fresh11 = *dst.offset(3 as ::core::ffi::c_int as isize);
        *fresh11 = (*fresh11 as ::core::ffi::c_uint)
            .wrapping_add(
                *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
            ) as uint8_t as uint8_t;
        dst = dst.offset(stride as isize);
        src = src.offset(4 as ::core::ffi::c_int as isize);
        i += 1;
    }
    memset(
        _src as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<int16_t>() as size_t).wrapping_mul(16 as size_t),
    );
}
unsafe extern "C" fn ff_h264_add_pixels8_8_c(
    mut _dst: *mut uint8_t,
    mut _src: *mut int16_t,
    mut stride: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut dst: *mut uint8_t = _dst;
    let mut src: *mut int16_t = _src;
    stride = (stride as ::core::ffi::c_uint)
        .wrapping_div(::core::mem::size_of::<uint8_t>() as usize as ::core::ffi::c_uint)
        as ::core::ffi::c_int as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        let ref mut fresh0 = *dst.offset(0 as ::core::ffi::c_int as isize);
        *fresh0 = (*fresh0 as ::core::ffi::c_uint)
            .wrapping_add(
                *src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
            ) as uint8_t as uint8_t;
        let ref mut fresh1 = *dst.offset(1 as ::core::ffi::c_int as isize);
        *fresh1 = (*fresh1 as ::core::ffi::c_uint)
            .wrapping_add(
                *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
            ) as uint8_t as uint8_t;
        let ref mut fresh2 = *dst.offset(2 as ::core::ffi::c_int as isize);
        *fresh2 = (*fresh2 as ::core::ffi::c_uint)
            .wrapping_add(
                *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
            ) as uint8_t as uint8_t;
        let ref mut fresh3 = *dst.offset(3 as ::core::ffi::c_int as isize);
        *fresh3 = (*fresh3 as ::core::ffi::c_uint)
            .wrapping_add(
                *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
            ) as uint8_t as uint8_t;
        let ref mut fresh4 = *dst.offset(4 as ::core::ffi::c_int as isize);
        *fresh4 = (*fresh4 as ::core::ffi::c_uint)
            .wrapping_add(
                *src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
            ) as uint8_t as uint8_t;
        let ref mut fresh5 = *dst.offset(5 as ::core::ffi::c_int as isize);
        *fresh5 = (*fresh5 as ::core::ffi::c_uint)
            .wrapping_add(
                *src.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
            ) as uint8_t as uint8_t;
        let ref mut fresh6 = *dst.offset(6 as ::core::ffi::c_int as isize);
        *fresh6 = (*fresh6 as ::core::ffi::c_uint)
            .wrapping_add(
                *src.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
            ) as uint8_t as uint8_t;
        let ref mut fresh7 = *dst.offset(7 as ::core::ffi::c_int as isize);
        *fresh7 = (*fresh7 as ::core::ffi::c_uint)
            .wrapping_add(
                *src.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
            ) as uint8_t as uint8_t;
        dst = dst.offset(stride as isize);
        src = src.offset(8 as ::core::ffi::c_int as isize);
        i += 1;
    }
    memset(
        _src as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<int16_t>() as size_t).wrapping_mul(64 as size_t),
    );
}

#[no_mangle]
#[cold]
pub unsafe extern "C" fn ff_h264dsp_init(
    mut c: *mut H264DSPContext,
    bit_depth: ::core::ffi::c_int,
    chroma_format_idc: ::core::ffi::c_int,
) {
    {
        (*c).luma_dc_dequant_idct = Some(
            ff_h264_luma_dc_dequant_idct_8_c
                as unsafe extern "C" fn(
                    *mut int16_t,
                    *mut int16_t,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut int16_t,
                    *mut int16_t,
                    ::core::ffi::c_int,
                ) -> (),
            >;
        if chroma_format_idc <= 1 as ::core::ffi::c_int {
            (*c).chroma_dc_dequant_idct = Some(
                ff_h264_chroma_dc_dequant_idct_8_c
                    as unsafe extern "C" fn(*mut int16_t, ::core::ffi::c_int) -> (),
            ) as Option<unsafe extern "C" fn(*mut int16_t, ::core::ffi::c_int) -> ()>;
        } else {
            (*c).chroma_dc_dequant_idct = Some(
                ff_h264_chroma422_dc_dequant_idct_8_c
                    as unsafe extern "C" fn(*mut int16_t, ::core::ffi::c_int) -> (),
            ) as Option<unsafe extern "C" fn(*mut int16_t, ::core::ffi::c_int) -> ()>;
        }
        (*c).add_pixels4_clear = Some(
            ff_h264_add_pixels4_8_c
                as unsafe extern "C" fn(
                    *mut uint8_t,
                    *mut int16_t,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut uint8_t,
                    *mut int16_t,
                    ::core::ffi::c_int,
                ) -> (),
            >;
        (*c).add_pixels8_clear = Some(
            ff_h264_add_pixels8_8_c
                as unsafe extern "C" fn(
                    *mut uint8_t,
                    *mut int16_t,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut uint8_t,
                    *mut int16_t,
                    ::core::ffi::c_int,
                ) -> (),
            >;
    }
    {
            if !(bit_depth <= 8 as ::core::ffi::c_int) {
                av_log(
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    0 as ::core::ffi::c_int,
                    b"Assertion %s failed at %s:%d\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"bit_depth<=8\0" as *const u8 as *const ::core::ffi::c_char,
                    b"src/libavcodec/h264dsp.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    149 as ::core::ffi::c_int,
                );
                abort();
            }
            (*c).idct_add = Some(
                ff_h264_idct_add_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *mut int16_t,
                        ::core::ffi::c_int,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut uint8_t,
                        *mut int16_t,
                        ::core::ffi::c_int,
                    ) -> (),
                >;
            (*c).idct8_add = Some(
                ff_h264_idct8_add_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *mut int16_t,
                        ::core::ffi::c_int,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut uint8_t,
                        *mut int16_t,
                        ::core::ffi::c_int,
                    ) -> (),
                >;
            (*c).idct_dc_add = Some(
                ff_h264_idct_dc_add_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *mut int16_t,
                        ::core::ffi::c_int,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut uint8_t,
                        *mut int16_t,
                        ::core::ffi::c_int,
                    ) -> (),
                >;
            (*c).idct8_dc_add = Some(
                ff_h264_idct8_dc_add_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *mut int16_t,
                        ::core::ffi::c_int,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut uint8_t,
                        *mut int16_t,
                        ::core::ffi::c_int,
                    ) -> (),
                >;
            (*c).idct_add16 = Some(
                ff_h264_idct_add16_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const ::core::ffi::c_int,
                        *mut int16_t,
                        ::core::ffi::c_int,
                        *const uint8_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut uint8_t,
                        *const ::core::ffi::c_int,
                        *mut int16_t,
                        ::core::ffi::c_int,
                        *const uint8_t,
                    ) -> (),
                >;
            (*c).idct8_add4 = Some(
                ff_h264_idct8_add4_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const ::core::ffi::c_int,
                        *mut int16_t,
                        ::core::ffi::c_int,
                        *const uint8_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut uint8_t,
                        *const ::core::ffi::c_int,
                        *mut int16_t,
                        ::core::ffi::c_int,
                        *const uint8_t,
                    ) -> (),
                >;
            if chroma_format_idc <= 1 as ::core::ffi::c_int {
                (*c).idct_add8 = Some(
                    ff_h264_idct_add8_8_c
                        as unsafe extern "C" fn(
                            *mut *mut uint8_t,
                            *const ::core::ffi::c_int,
                            *mut int16_t,
                            ::core::ffi::c_int,
                            *const uint8_t,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut *mut uint8_t,
                            *const ::core::ffi::c_int,
                            *mut int16_t,
                            ::core::ffi::c_int,
                            *const uint8_t,
                        ) -> (),
                    >;
            } else {
                (*c).idct_add8 = Some(
                    ff_h264_idct_add8_422_8_c
                        as unsafe extern "C" fn(
                            *mut *mut uint8_t,
                            *const ::core::ffi::c_int,
                            *mut int16_t,
                            ::core::ffi::c_int,
                            *const uint8_t,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut *mut uint8_t,
                            *const ::core::ffi::c_int,
                            *mut int16_t,
                            ::core::ffi::c_int,
                            *const uint8_t,
                        ) -> (),
                    >;
            }
            (*c).idct_add16intra = Some(
                ff_h264_idct_add16intra_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *const ::core::ffi::c_int,
                        *mut int16_t,
                        ::core::ffi::c_int,
                        *const uint8_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut uint8_t,
                        *const ::core::ffi::c_int,
                        *mut int16_t,
                        ::core::ffi::c_int,
                        *const uint8_t,
                    ) -> (),
                >;
            (*c).weight_pixels_tab[0 as ::core::ffi::c_int as usize] = Some(
                weight_h264_pixels16_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        ptrdiff_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> (),
            ) as h264_weight_func;
            (*c).weight_pixels_tab[1 as ::core::ffi::c_int as usize] = Some(
                weight_h264_pixels8_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        ptrdiff_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> (),
            ) as h264_weight_func;
            (*c).weight_pixels_tab[2 as ::core::ffi::c_int as usize] = Some(
                weight_h264_pixels4_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        ptrdiff_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> (),
            ) as h264_weight_func;
            (*c).weight_pixels_tab[3 as ::core::ffi::c_int as usize] = Some(
                weight_h264_pixels2_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        ptrdiff_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> (),
            ) as h264_weight_func;
            (*c).biweight_pixels_tab[0 as ::core::ffi::c_int as usize] = Some(
                biweight_h264_pixels16_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *mut uint8_t,
                        ptrdiff_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> (),
            ) as h264_biweight_func;
            (*c).biweight_pixels_tab[1 as ::core::ffi::c_int as usize] = Some(
                biweight_h264_pixels8_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *mut uint8_t,
                        ptrdiff_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> (),
            ) as h264_biweight_func;
            (*c).biweight_pixels_tab[2 as ::core::ffi::c_int as usize] = Some(
                biweight_h264_pixels4_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *mut uint8_t,
                        ptrdiff_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> (),
            ) as h264_biweight_func;
            (*c).biweight_pixels_tab[3 as ::core::ffi::c_int as usize] = Some(
                biweight_h264_pixels2_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        *mut uint8_t,
                        ptrdiff_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> (),
            ) as h264_biweight_func;
            (*c).v_loop_filter_luma = Some(
                h264_v_loop_filter_luma_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        ptrdiff_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut int8_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut uint8_t,
                        ptrdiff_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut int8_t,
                    ) -> (),
                >;
            (*c).h_loop_filter_luma = Some(
                h264_h_loop_filter_luma_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        ptrdiff_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut int8_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut uint8_t,
                        ptrdiff_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut int8_t,
                    ) -> (),
                >;
            (*c).h_loop_filter_luma_mbaff = Some(
                h264_h_loop_filter_luma_mbaff_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        ptrdiff_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut int8_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut uint8_t,
                        ptrdiff_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut int8_t,
                    ) -> (),
                >;
            (*c).v_loop_filter_luma_intra = Some(
                h264_v_loop_filter_luma_intra_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        ptrdiff_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut uint8_t,
                        ptrdiff_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> (),
                >;
            (*c).h_loop_filter_luma_intra = Some(
                h264_h_loop_filter_luma_intra_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        ptrdiff_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut uint8_t,
                        ptrdiff_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> (),
                >;
            (*c).h_loop_filter_luma_mbaff_intra = Some(
                h264_h_loop_filter_luma_mbaff_intra_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        ptrdiff_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut uint8_t,
                        ptrdiff_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> (),
                >;
            (*c).v_loop_filter_chroma = Some(
                h264_v_loop_filter_chroma_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        ptrdiff_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut int8_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut uint8_t,
                        ptrdiff_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut int8_t,
                    ) -> (),
                >;
            if chroma_format_idc <= 1 as ::core::ffi::c_int {
                (*c).h_loop_filter_chroma = Some(
                    h264_h_loop_filter_chroma_8_c
                        as unsafe extern "C" fn(
                            *mut uint8_t,
                            ptrdiff_t,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                            *mut int8_t,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut uint8_t,
                            ptrdiff_t,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                            *mut int8_t,
                        ) -> (),
                    >;
            } else {
                (*c).h_loop_filter_chroma = Some(
                    h264_h_loop_filter_chroma422_8_c
                        as unsafe extern "C" fn(
                            *mut uint8_t,
                            ptrdiff_t,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                            *mut int8_t,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut uint8_t,
                            ptrdiff_t,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                            *mut int8_t,
                        ) -> (),
                    >;
            }
            if chroma_format_idc <= 1 as ::core::ffi::c_int {
                (*c).h_loop_filter_chroma_mbaff = Some(
                    h264_h_loop_filter_chroma_mbaff_8_c
                        as unsafe extern "C" fn(
                            *mut uint8_t,
                            ptrdiff_t,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                            *mut int8_t,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut uint8_t,
                            ptrdiff_t,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                            *mut int8_t,
                        ) -> (),
                    >;
            } else {
                (*c).h_loop_filter_chroma_mbaff = Some(
                    h264_h_loop_filter_chroma422_mbaff_8_c
                        as unsafe extern "C" fn(
                            *mut uint8_t,
                            ptrdiff_t,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                            *mut int8_t,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut uint8_t,
                            ptrdiff_t,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                            *mut int8_t,
                        ) -> (),
                    >;
            }
            (*c).v_loop_filter_chroma_intra = Some(
                h264_v_loop_filter_chroma_intra_8_c
                    as unsafe extern "C" fn(
                        *mut uint8_t,
                        ptrdiff_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut uint8_t,
                        ptrdiff_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> (),
                >;
            if chroma_format_idc <= 1 as ::core::ffi::c_int {
                (*c).h_loop_filter_chroma_intra = Some(
                    h264_h_loop_filter_chroma_intra_8_c
                        as unsafe extern "C" fn(
                            *mut uint8_t,
                            ptrdiff_t,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut uint8_t,
                            ptrdiff_t,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                        ) -> (),
                    >;
            } else {
                (*c).h_loop_filter_chroma_intra = Some(
                    h264_h_loop_filter_chroma422_intra_8_c
                        as unsafe extern "C" fn(
                            *mut uint8_t,
                            ptrdiff_t,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut uint8_t,
                            ptrdiff_t,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                        ) -> (),
                    >;
            }
            if chroma_format_idc <= 1 as ::core::ffi::c_int {
                (*c).h_loop_filter_chroma_mbaff_intra = Some(
                    h264_h_loop_filter_chroma_mbaff_intra_8_c
                        as unsafe extern "C" fn(
                            *mut uint8_t,
                            ptrdiff_t,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut uint8_t,
                            ptrdiff_t,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                        ) -> (),
                    >;
            } else {
                (*c).h_loop_filter_chroma_mbaff_intra = Some(
                    h264_h_loop_filter_chroma422_mbaff_intra_8_c
                        as unsafe extern "C" fn(
                            *mut uint8_t,
                            ptrdiff_t,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut uint8_t,
                            ptrdiff_t,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                        ) -> (),
                    >;
            }
            (*c).loop_filter_strength = None;
        }
    (*c).startcode_find_candidate = Some(
        ff_startcode_find_candidate_c
            as unsafe extern "C" fn(
                *const uint8_t,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *const uint8_t,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >;
    ff_h264dsp_init_arm(c, bit_depth, chroma_format_idc);
}




unsafe extern "C" fn weight_h264_pixels16_8_c(
    mut _block: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut height: ::core::ffi::c_int,
    mut log2_denom: ::core::ffi::c_int,
    mut weight: ::core::ffi::c_int,
    mut offset: ::core::ffi::c_int,
) {
    let mut y: ::core::ffi::c_int = 0;
    let mut block: *mut uint8_t = _block;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    offset = ((offset as ::core::ffi::c_uint)
        << log2_denom + (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int))
        as ::core::ffi::c_int;
    if log2_denom != 0 {
        offset += (1 as ::core::ffi::c_int) << log2_denom - 1 as ::core::ffi::c_int;
    }
    y = 0 as ::core::ffi::c_int;
    while y < height {
        *block.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            *block.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                * weight + offset >> log2_denom,
        );
        *block.offset(1 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            *block.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                * weight + offset >> log2_denom,
        );
        if !(16 as ::core::ffi::c_int == 2 as ::core::ffi::c_int) {
            *block.offset(2 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                *block.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    * weight + offset >> log2_denom,
            );
            *block.offset(3 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                *block.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    * weight + offset >> log2_denom,
            );
            if !(16 as ::core::ffi::c_int == 4 as ::core::ffi::c_int) {
                *block.offset(4 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *block.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weight + offset >> log2_denom,
                );
                *block.offset(5 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *block.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weight + offset >> log2_denom,
                );
                *block.offset(6 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *block.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weight + offset >> log2_denom,
                );
                *block.offset(7 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *block.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weight + offset >> log2_denom,
                );
                if !(16 as ::core::ffi::c_int == 8 as ::core::ffi::c_int) {
                    *block.offset(8 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                    *block.offset(9 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                    *block.offset(10 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                    *block.offset(11 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(11 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                    *block.offset(12 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(12 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                    *block.offset(13 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(13 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                    *block.offset(14 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(14 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                    *block.offset(15 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(15 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                }
            }
        }
        y += 1;
        block = block.offset(stride as isize);
    }
}
unsafe extern "C" fn biweight_h264_pixels16_8_c(
    mut _dst: *mut uint8_t,
    mut _src: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut height: ::core::ffi::c_int,
    mut log2_denom: ::core::ffi::c_int,
    mut weightd: ::core::ffi::c_int,
    mut weights: ::core::ffi::c_int,
    mut offset: ::core::ffi::c_int,
) {
    let mut y: ::core::ffi::c_int = 0;
    let mut dst: *mut uint8_t = _dst;
    let mut src: *mut uint8_t = _src;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    offset = ((offset as ::core::ffi::c_uint)
        << 8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int) as ::core::ffi::c_int;
    offset = (((offset + 1 as ::core::ffi::c_int | 1 as ::core::ffi::c_int)
        as ::core::ffi::c_uint) << log2_denom) as ::core::ffi::c_int;
    y = 0 as ::core::ffi::c_int;
    while y < height {
        *dst.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            *src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int * weights
                + *dst.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    * weightd + offset >> log2_denom + 1 as ::core::ffi::c_int,
        );
        *dst.offset(1 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int * weights
                + *dst.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    * weightd + offset >> log2_denom + 1 as ::core::ffi::c_int,
        );
        if !(16 as ::core::ffi::c_int == 2 as ::core::ffi::c_int) {
            *dst.offset(2 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    * weights
                    + *dst.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weightd + offset >> log2_denom + 1 as ::core::ffi::c_int,
            );
            *dst.offset(3 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    * weights
                    + *dst.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weightd + offset >> log2_denom + 1 as ::core::ffi::c_int,
            );
            if !(16 as ::core::ffi::c_int == 4 as ::core::ffi::c_int) {
                *dst.offset(4 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weights
                        + *dst.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weightd + offset
                        >> log2_denom + 1 as ::core::ffi::c_int,
                );
                *dst.offset(5 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *src.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weights
                        + *dst.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weightd + offset
                        >> log2_denom + 1 as ::core::ffi::c_int,
                );
                *dst.offset(6 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *src.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weights
                        + *dst.offset(6 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weightd + offset
                        >> log2_denom + 1 as ::core::ffi::c_int,
                );
                *dst.offset(7 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *src.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weights
                        + *dst.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weightd + offset
                        >> log2_denom + 1 as ::core::ffi::c_int,
                );
                if !(16 as ::core::ffi::c_int == 8 as ::core::ffi::c_int) {
                    *dst.offset(8 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                    *dst.offset(9 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                    *dst.offset(10 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                    *dst.offset(11 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(11 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(11 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                    *dst.offset(12 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(12 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(12 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                    *dst.offset(13 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(13 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(13 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                    *dst.offset(14 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(14 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(14 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                    *dst.offset(15 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(15 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(15 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                }
            }
        }
        y += 1;
        dst = dst.offset(stride as isize);
        src = src.offset(stride as isize);
    }
}







unsafe extern "C" fn weight_h264_pixels8_8_c(
    mut _block: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut height: ::core::ffi::c_int,
    mut log2_denom: ::core::ffi::c_int,
    mut weight: ::core::ffi::c_int,
    mut offset: ::core::ffi::c_int,
) {
    let mut y: ::core::ffi::c_int = 0;
    let mut block: *mut uint8_t = _block;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    offset = ((offset as ::core::ffi::c_uint)
        << log2_denom + (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int))
        as ::core::ffi::c_int;
    if log2_denom != 0 {
        offset += (1 as ::core::ffi::c_int) << log2_denom - 1 as ::core::ffi::c_int;
    }
    y = 0 as ::core::ffi::c_int;
    while y < height {
        *block.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            *block.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                * weight + offset >> log2_denom,
        );
        *block.offset(1 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            *block.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                * weight + offset >> log2_denom,
        );
        if !(8 as ::core::ffi::c_int == 2 as ::core::ffi::c_int) {
            *block.offset(2 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                *block.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    * weight + offset >> log2_denom,
            );
            *block.offset(3 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                *block.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    * weight + offset >> log2_denom,
            );
            if !(8 as ::core::ffi::c_int == 4 as ::core::ffi::c_int) {
                *block.offset(4 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *block.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weight + offset >> log2_denom,
                );
                *block.offset(5 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *block.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weight + offset >> log2_denom,
                );
                *block.offset(6 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *block.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weight + offset >> log2_denom,
                );
                *block.offset(7 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *block.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weight + offset >> log2_denom,
                );
                if !(8 as ::core::ffi::c_int == 8 as ::core::ffi::c_int) {
                    *block.offset(8 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                    *block.offset(9 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                    *block.offset(10 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                    *block.offset(11 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(11 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                    *block.offset(12 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(12 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                    *block.offset(13 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(13 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                    *block.offset(14 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(14 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                    *block.offset(15 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(15 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                }
            }
        }
        y += 1;
        block = block.offset(stride as isize);
    }
}

unsafe extern "C" fn biweight_h264_pixels8_8_c(
    mut _dst: *mut uint8_t,
    mut _src: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut height: ::core::ffi::c_int,
    mut log2_denom: ::core::ffi::c_int,
    mut weightd: ::core::ffi::c_int,
    mut weights: ::core::ffi::c_int,
    mut offset: ::core::ffi::c_int,
) {
    let mut y: ::core::ffi::c_int = 0;
    let mut dst: *mut uint8_t = _dst;
    let mut src: *mut uint8_t = _src;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    offset = ((offset as ::core::ffi::c_uint)
        << 8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int) as ::core::ffi::c_int;
    offset = (((offset + 1 as ::core::ffi::c_int | 1 as ::core::ffi::c_int)
        as ::core::ffi::c_uint) << log2_denom) as ::core::ffi::c_int;
    y = 0 as ::core::ffi::c_int;
    while y < height {
        *dst.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            *src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int * weights
                + *dst.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    * weightd + offset >> log2_denom + 1 as ::core::ffi::c_int,
        );
        *dst.offset(1 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int * weights
                + *dst.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    * weightd + offset >> log2_denom + 1 as ::core::ffi::c_int,
        );
        if !(8 as ::core::ffi::c_int == 2 as ::core::ffi::c_int) {
            *dst.offset(2 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    * weights
                    + *dst.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weightd + offset >> log2_denom + 1 as ::core::ffi::c_int,
            );
            *dst.offset(3 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    * weights
                    + *dst.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weightd + offset >> log2_denom + 1 as ::core::ffi::c_int,
            );
            if !(8 as ::core::ffi::c_int == 4 as ::core::ffi::c_int) {
                *dst.offset(4 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weights
                        + *dst.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weightd + offset
                        >> log2_denom + 1 as ::core::ffi::c_int,
                );
                *dst.offset(5 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *src.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weights
                        + *dst.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weightd + offset
                        >> log2_denom + 1 as ::core::ffi::c_int,
                );
                *dst.offset(6 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *src.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weights
                        + *dst.offset(6 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weightd + offset
                        >> log2_denom + 1 as ::core::ffi::c_int,
                );
                *dst.offset(7 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *src.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weights
                        + *dst.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weightd + offset
                        >> log2_denom + 1 as ::core::ffi::c_int,
                );
                if !(8 as ::core::ffi::c_int == 8 as ::core::ffi::c_int) {
                    *dst.offset(8 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                    *dst.offset(9 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                    *dst.offset(10 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                    *dst.offset(11 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(11 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(11 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                    *dst.offset(12 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(12 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(12 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                    *dst.offset(13 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(13 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(13 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                    *dst.offset(14 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(14 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(14 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                    *dst.offset(15 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(15 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(15 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                }
            }
        }
        y += 1;
        dst = dst.offset(stride as isize);
        src = src.offset(stride as isize);
    }
}







unsafe extern "C" fn weight_h264_pixels4_8_c(
    mut _block: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut height: ::core::ffi::c_int,
    mut log2_denom: ::core::ffi::c_int,
    mut weight: ::core::ffi::c_int,
    mut offset: ::core::ffi::c_int,
) {
    let mut y: ::core::ffi::c_int = 0;
    let mut block: *mut uint8_t = _block;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    offset = ((offset as ::core::ffi::c_uint)
        << log2_denom + (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int))
        as ::core::ffi::c_int;
    if log2_denom != 0 {
        offset += (1 as ::core::ffi::c_int) << log2_denom - 1 as ::core::ffi::c_int;
    }
    y = 0 as ::core::ffi::c_int;
    while y < height {
        *block.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            *block.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                * weight + offset >> log2_denom,
        );
        *block.offset(1 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            *block.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                * weight + offset >> log2_denom,
        );
        if !(4 as ::core::ffi::c_int == 2 as ::core::ffi::c_int) {
            *block.offset(2 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                *block.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    * weight + offset >> log2_denom,
            );
            *block.offset(3 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                *block.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    * weight + offset >> log2_denom,
            );
            if !(4 as ::core::ffi::c_int == 4 as ::core::ffi::c_int) {
                *block.offset(4 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *block.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weight + offset >> log2_denom,
                );
                *block.offset(5 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *block.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weight + offset >> log2_denom,
                );
                *block.offset(6 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *block.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weight + offset >> log2_denom,
                );
                *block.offset(7 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *block.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weight + offset >> log2_denom,
                );
                if !(4 as ::core::ffi::c_int == 8 as ::core::ffi::c_int) {
                    *block.offset(8 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                    *block.offset(9 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                    *block.offset(10 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                    *block.offset(11 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(11 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                    *block.offset(12 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(12 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                    *block.offset(13 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(13 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                    *block.offset(14 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(14 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                    *block.offset(15 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(15 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                }
            }
        }
        y += 1;
        block = block.offset(stride as isize);
    }
}

unsafe extern "C" fn biweight_h264_pixels4_8_c(
    mut _dst: *mut uint8_t,
    mut _src: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut height: ::core::ffi::c_int,
    mut log2_denom: ::core::ffi::c_int,
    mut weightd: ::core::ffi::c_int,
    mut weights: ::core::ffi::c_int,
    mut offset: ::core::ffi::c_int,
) {
    let mut y: ::core::ffi::c_int = 0;
    let mut dst: *mut uint8_t = _dst;
    let mut src: *mut uint8_t = _src;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    offset = ((offset as ::core::ffi::c_uint)
        << 8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int) as ::core::ffi::c_int;
    offset = (((offset + 1 as ::core::ffi::c_int | 1 as ::core::ffi::c_int)
        as ::core::ffi::c_uint) << log2_denom) as ::core::ffi::c_int;
    y = 0 as ::core::ffi::c_int;
    while y < height {
        *dst.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            *src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int * weights
                + *dst.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    * weightd + offset >> log2_denom + 1 as ::core::ffi::c_int,
        );
        *dst.offset(1 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int * weights
                + *dst.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    * weightd + offset >> log2_denom + 1 as ::core::ffi::c_int,
        );
        if !(4 as ::core::ffi::c_int == 2 as ::core::ffi::c_int) {
            *dst.offset(2 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    * weights
                    + *dst.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weightd + offset >> log2_denom + 1 as ::core::ffi::c_int,
            );
            *dst.offset(3 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    * weights
                    + *dst.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weightd + offset >> log2_denom + 1 as ::core::ffi::c_int,
            );
            if !(4 as ::core::ffi::c_int == 4 as ::core::ffi::c_int) {
                *dst.offset(4 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weights
                        + *dst.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weightd + offset
                        >> log2_denom + 1 as ::core::ffi::c_int,
                );
                *dst.offset(5 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *src.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weights
                        + *dst.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weightd + offset
                        >> log2_denom + 1 as ::core::ffi::c_int,
                );
                *dst.offset(6 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *src.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weights
                        + *dst.offset(6 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weightd + offset
                        >> log2_denom + 1 as ::core::ffi::c_int,
                );
                *dst.offset(7 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *src.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weights
                        + *dst.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weightd + offset
                        >> log2_denom + 1 as ::core::ffi::c_int,
                );
                if !(4 as ::core::ffi::c_int == 8 as ::core::ffi::c_int) {
                    *dst.offset(8 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                    *dst.offset(9 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                    *dst.offset(10 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                    *dst.offset(11 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(11 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(11 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                    *dst.offset(12 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(12 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(12 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                    *dst.offset(13 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(13 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(13 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                    *dst.offset(14 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(14 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(14 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                    *dst.offset(15 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(15 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(15 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                }
            }
        }
        y += 1;
        dst = dst.offset(stride as isize);
        src = src.offset(stride as isize);
    }
}





unsafe extern "C" fn weight_h264_pixels2_8_c(
    mut _block: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut height: ::core::ffi::c_int,
    mut log2_denom: ::core::ffi::c_int,
    mut weight: ::core::ffi::c_int,
    mut offset: ::core::ffi::c_int,
) {
    let mut y: ::core::ffi::c_int = 0;
    let mut block: *mut uint8_t = _block;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    offset = ((offset as ::core::ffi::c_uint)
        << log2_denom + (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int))
        as ::core::ffi::c_int;
    if log2_denom != 0 {
        offset += (1 as ::core::ffi::c_int) << log2_denom - 1 as ::core::ffi::c_int;
    }
    y = 0 as ::core::ffi::c_int;
    while y < height {
        *block.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            *block.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                * weight + offset >> log2_denom,
        );
        *block.offset(1 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            *block.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                * weight + offset >> log2_denom,
        );
        if !(2 as ::core::ffi::c_int == 2 as ::core::ffi::c_int) {
            *block.offset(2 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                *block.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    * weight + offset >> log2_denom,
            );
            *block.offset(3 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                *block.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    * weight + offset >> log2_denom,
            );
            if !(2 as ::core::ffi::c_int == 4 as ::core::ffi::c_int) {
                *block.offset(4 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *block.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weight + offset >> log2_denom,
                );
                *block.offset(5 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *block.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weight + offset >> log2_denom,
                );
                *block.offset(6 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *block.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weight + offset >> log2_denom,
                );
                *block.offset(7 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *block.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weight + offset >> log2_denom,
                );
                if !(2 as ::core::ffi::c_int == 8 as ::core::ffi::c_int) {
                    *block.offset(8 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                    *block.offset(9 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                    *block.offset(10 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                    *block.offset(11 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(11 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                    *block.offset(12 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(12 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                    *block.offset(13 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(13 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                    *block.offset(14 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(14 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                    *block.offset(15 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *block.offset(15 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weight + offset >> log2_denom,
                    );
                }
            }
        }
        y += 1;
        block = block.offset(stride as isize);
    }
}



unsafe extern "C" fn biweight_h264_pixels2_8_c(
    mut _dst: *mut uint8_t,
    mut _src: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut height: ::core::ffi::c_int,
    mut log2_denom: ::core::ffi::c_int,
    mut weightd: ::core::ffi::c_int,
    mut weights: ::core::ffi::c_int,
    mut offset: ::core::ffi::c_int,
) {
    let mut y: ::core::ffi::c_int = 0;
    let mut dst: *mut uint8_t = _dst;
    let mut src: *mut uint8_t = _src;
    stride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    offset = ((offset as ::core::ffi::c_uint)
        << 8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int) as ::core::ffi::c_int;
    offset = (((offset + 1 as ::core::ffi::c_int | 1 as ::core::ffi::c_int)
        as ::core::ffi::c_uint) << log2_denom) as ::core::ffi::c_int;
    y = 0 as ::core::ffi::c_int;
    while y < height {
        *dst.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            *src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int * weights
                + *dst.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    * weightd + offset >> log2_denom + 1 as ::core::ffi::c_int,
        );
        *dst.offset(1 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
            *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int * weights
                + *dst.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    * weightd + offset >> log2_denom + 1 as ::core::ffi::c_int,
        );
        if !(2 as ::core::ffi::c_int == 2 as ::core::ffi::c_int) {
            *dst.offset(2 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    * weights
                    + *dst.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weightd + offset >> log2_denom + 1 as ::core::ffi::c_int,
            );
            *dst.offset(3 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    * weights
                    + *dst.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weightd + offset >> log2_denom + 1 as ::core::ffi::c_int,
            );
            if !(2 as ::core::ffi::c_int == 4 as ::core::ffi::c_int) {
                *dst.offset(4 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *src.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weights
                        + *dst.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weightd + offset
                        >> log2_denom + 1 as ::core::ffi::c_int,
                );
                *dst.offset(5 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *src.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weights
                        + *dst.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weightd + offset
                        >> log2_denom + 1 as ::core::ffi::c_int,
                );
                *dst.offset(6 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *src.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weights
                        + *dst.offset(6 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weightd + offset
                        >> log2_denom + 1 as ::core::ffi::c_int,
                );
                *dst.offset(7 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                    *src.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        * weights
                        + *dst.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weightd + offset
                        >> log2_denom + 1 as ::core::ffi::c_int,
                );
                if !(2 as ::core::ffi::c_int == 8 as ::core::ffi::c_int) {
                    *dst.offset(8 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                    *dst.offset(9 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                    *dst.offset(10 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                    *dst.offset(11 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(11 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(11 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                    *dst.offset(12 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(12 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(12 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                    *dst.offset(13 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(13 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(13 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                    *dst.offset(14 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(14 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(14 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                    *dst.offset(15 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        *src.offset(15 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int * weights
                            + *dst.offset(15 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int * weightd + offset
                            >> log2_denom + 1 as ::core::ffi::c_int,
                    );
                }
            }
        }
        y += 1;
        dst = dst.offset(stride as isize);
        src = src.offset(stride as isize);
    }
}







#[inline(always)]
unsafe extern "C" fn h264_loop_filter_luma_8_c(
    mut p_pix: *mut uint8_t,
    mut xstride: ptrdiff_t,
    mut ystride: ptrdiff_t,
    mut inner_iters: ::core::ffi::c_int,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc0: *mut int8_t,
) {
    let mut pix: *mut uint8_t = p_pix;
    let mut i: ::core::ffi::c_int = 0;
    let mut d: ::core::ffi::c_int = 0;
    xstride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    ystride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    alpha <<= 8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int;
    beta <<= 8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        let tc_orig: ::core::ffi::c_int = *tc0.offset(i as isize) as ::core::ffi::c_int
            * ((1 as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int);
        if tc_orig < 0 as ::core::ffi::c_int {
            pix = pix.offset((inner_iters as ptrdiff_t * ystride) as isize);
        } else {
            d = 0 as ::core::ffi::c_int;
            while d < inner_iters {
                let p0: ::core::ffi::c_int = *pix
                    .offset((-(1 as ptrdiff_t) * xstride) as isize)
                    as ::core::ffi::c_int;
                let p1: ::core::ffi::c_int = *pix
                    .offset((-(2 as ptrdiff_t) * xstride) as isize)
                    as ::core::ffi::c_int;
                let p2: ::core::ffi::c_int = *pix
                    .offset((-(3 as ptrdiff_t) * xstride) as isize)
                    as ::core::ffi::c_int;
                let q0: ::core::ffi::c_int = *pix
                    .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
                let q1: ::core::ffi::c_int = *pix
                    .offset((1 as ptrdiff_t * xstride) as isize) as ::core::ffi::c_int;
                let q2: ::core::ffi::c_int = *pix
                    .offset((2 as ptrdiff_t * xstride) as isize) as ::core::ffi::c_int;
                if (if p0 - q0 >= 0 as ::core::ffi::c_int {
                    p0 - q0
                } else {
                    -(p0 - q0)
                }) < alpha
                    && (if p1 - p0 >= 0 as ::core::ffi::c_int {
                        p1 - p0
                    } else {
                        -(p1 - p0)
                    }) < beta
                    && (if q1 - q0 >= 0 as ::core::ffi::c_int {
                        q1 - q0
                    } else {
                        -(q1 - q0)
                    }) < beta
                {
                    let mut tc: ::core::ffi::c_int = tc_orig;
                    let mut i_delta: ::core::ffi::c_int = 0;
                    if (if p2 - p0 >= 0 as ::core::ffi::c_int {
                        p2 - p0
                    } else {
                        -(p2 - p0)
                    }) < beta
                    {
                        if tc_orig != 0 {
                            *pix.offset((-(2 as ptrdiff_t) * xstride) as isize) = (p1
                                + av_clip_c(
                                    (p2
                                        + (p0 + q0 + 1 as ::core::ffi::c_int
                                            >> 1 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int)
                                        - p1,
                                    -tc_orig,
                                    tc_orig,
                                )) as uint8_t;
                        }
                        tc += 1;
                    }
                    if (if q2 - q0 >= 0 as ::core::ffi::c_int {
                        q2 - q0
                    } else {
                        -(q2 - q0)
                    }) < beta
                    {
                        if tc_orig != 0 {
                            *pix.offset(xstride as isize) = (q1
                                + av_clip_c(
                                    (q2
                                        + (p0 + q0 + 1 as ::core::ffi::c_int
                                            >> 1 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int)
                                        - q1,
                                    -tc_orig,
                                    tc_orig,
                                )) as uint8_t;
                        }
                        tc += 1;
                    }
                    i_delta = av_clip_c(
                        (q0 - p0) * 4 as ::core::ffi::c_int + (p1 - q1)
                            + 4 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int,
                        -tc,
                        tc,
                    );
                    *pix.offset(-xstride as isize) = av_clip_uint8_c(p0 + i_delta);
                    *pix.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        q0 - i_delta,
                    );
                }
                pix = pix.offset(ystride as isize);
                d += 1;
            }
        }
        i += 1;
    }
}



unsafe extern "C" fn h264_v_loop_filter_luma_8_c(
    mut pix: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc0: *mut int8_t,
) {
    h264_loop_filter_luma_8_c(
        pix,
        stride,
        ::core::mem::size_of::<uint8_t>() as ptrdiff_t,
        4 as ::core::ffi::c_int,
        alpha,
        beta,
        tc0,
    );
}





unsafe extern "C" fn h264_h_loop_filter_luma_8_c(
    mut pix: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc0: *mut int8_t,
) {
    h264_loop_filter_luma_8_c(
        pix,
        ::core::mem::size_of::<uint8_t>() as ptrdiff_t,
        stride,
        4 as ::core::ffi::c_int,
        alpha,
        beta,
        tc0,
    );
}




unsafe extern "C" fn h264_h_loop_filter_luma_mbaff_8_c(
    mut pix: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc0: *mut int8_t,
) {
    h264_loop_filter_luma_8_c(
        pix,
        ::core::mem::size_of::<uint8_t>() as ptrdiff_t,
        stride,
        2 as ::core::ffi::c_int,
        alpha,
        beta,
        tc0,
    );
}

#[inline(always)]
unsafe extern "C" fn h264_loop_filter_luma_intra_8_c(
    mut p_pix: *mut uint8_t,
    mut xstride: ptrdiff_t,
    mut ystride: ptrdiff_t,
    mut inner_iters: ::core::ffi::c_int,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    let mut pix: *mut uint8_t = p_pix;
    let mut d: ::core::ffi::c_int = 0;
    xstride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    ystride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    alpha <<= 8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int;
    beta <<= 8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int;
    d = 0 as ::core::ffi::c_int;
    while d < 4 as ::core::ffi::c_int * inner_iters {
        let p2: ::core::ffi::c_int = *pix.offset((-(3 as ptrdiff_t) * xstride) as isize)
            as ::core::ffi::c_int;
        let p1: ::core::ffi::c_int = *pix.offset((-(2 as ptrdiff_t) * xstride) as isize)
            as ::core::ffi::c_int;
        let p0: ::core::ffi::c_int = *pix.offset((-(1 as ptrdiff_t) * xstride) as isize)
            as ::core::ffi::c_int;
        let q0: ::core::ffi::c_int = *pix.offset((0 as ptrdiff_t * xstride) as isize)
            as ::core::ffi::c_int;
        let q1: ::core::ffi::c_int = *pix.offset((1 as ptrdiff_t * xstride) as isize)
            as ::core::ffi::c_int;
        let q2: ::core::ffi::c_int = *pix.offset((2 as ptrdiff_t * xstride) as isize)
            as ::core::ffi::c_int;
        if (if p0 - q0 >= 0 as ::core::ffi::c_int { p0 - q0 } else { -(p0 - q0) })
            < alpha
            && (if p1 - p0 >= 0 as ::core::ffi::c_int { p1 - p0 } else { -(p1 - p0) })
                < beta
            && (if q1 - q0 >= 0 as ::core::ffi::c_int { q1 - q0 } else { -(q1 - q0) })
                < beta
        {
            if (if p0 - q0 >= 0 as ::core::ffi::c_int { p0 - q0 } else { -(p0 - q0) })
                < (alpha >> 2 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int
            {
                if (if p2 - p0 >= 0 as ::core::ffi::c_int {
                    p2 - p0
                } else {
                    -(p2 - p0)
                }) < beta
                {
                    let p3: ::core::ffi::c_int = *pix
                        .offset((-(4 as ptrdiff_t) * xstride) as isize)
                        as ::core::ffi::c_int;
                    *pix.offset((-(1 as ptrdiff_t) * xstride) as isize) = (p2
                        + 2 as ::core::ffi::c_int * p1 + 2 as ::core::ffi::c_int * p0
                        + 2 as ::core::ffi::c_int * q0 + q1 + 4 as ::core::ffi::c_int
                        >> 3 as ::core::ffi::c_int) as uint8_t;
                    *pix.offset((-(2 as ptrdiff_t) * xstride) as isize) = (p2 + p1 + p0
                        + q0 + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                        as uint8_t;
                    *pix.offset((-(3 as ptrdiff_t) * xstride) as isize) = (2
                        as ::core::ffi::c_int * p3 + 3 as ::core::ffi::c_int * p2 + p1
                        + p0 + q0 + 4 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int)
                        as uint8_t;
                } else {
                    *pix.offset((-(1 as ptrdiff_t) * xstride) as isize) = (2
                        as ::core::ffi::c_int * p1 + p0 + q1 + 2 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int) as uint8_t;
                }
                if (if q2 - q0 >= 0 as ::core::ffi::c_int {
                    q2 - q0
                } else {
                    -(q2 - q0)
                }) < beta
                {
                    let q3: ::core::ffi::c_int = *pix
                        .offset((3 as ptrdiff_t * xstride) as isize)
                        as ::core::ffi::c_int;
                    *pix.offset((0 as ptrdiff_t * xstride) as isize) = (p1
                        + 2 as ::core::ffi::c_int * p0 + 2 as ::core::ffi::c_int * q0
                        + 2 as ::core::ffi::c_int * q1 + q2 + 4 as ::core::ffi::c_int
                        >> 3 as ::core::ffi::c_int) as uint8_t;
                    *pix.offset((1 as ptrdiff_t * xstride) as isize) = (p0 + q0 + q1 + q2
                        + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as uint8_t;
                    *pix.offset((2 as ptrdiff_t * xstride) as isize) = (2
                        as ::core::ffi::c_int * q3 + 3 as ::core::ffi::c_int * q2 + q1
                        + q0 + p0 + 4 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int)
                        as uint8_t;
                } else {
                    *pix.offset((0 as ptrdiff_t * xstride) as isize) = (2
                        as ::core::ffi::c_int * q1 + q0 + p1 + 2 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int) as uint8_t;
                }
            } else {
                *pix.offset((-(1 as ptrdiff_t) * xstride) as isize) = (2
                    as ::core::ffi::c_int * p1 + p0 + q1 + 2 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int) as uint8_t;
                *pix.offset((0 as ptrdiff_t * xstride) as isize) = (2
                    as ::core::ffi::c_int * q1 + q0 + p1 + 2 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int) as uint8_t;
            }
        }
        pix = pix.offset(ystride as isize);
        d += 1;
    }
}




#[inline(always)]
unsafe extern "C" fn av_clip_c(
    mut a: ::core::ffi::c_int,
    mut amin: ::core::ffi::c_int,
    mut amax: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if a < amin { return amin } else if a > amax { return amax } else { return a };
}
#[inline(always)]
unsafe extern "C" fn av_clip_uint8_c(mut a: ::core::ffi::c_int) -> uint8_t {
    if a & !(0xff as ::core::ffi::c_int) != 0 {
        return (!a >> 31 as ::core::ffi::c_int) as uint8_t
    } else {
        return a as uint8_t
    };
}

unsafe extern "C" fn h264_v_loop_filter_luma_intra_8_c(
    mut pix: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    h264_loop_filter_luma_intra_8_c(
        pix,
        stride,
        ::core::mem::size_of::<uint8_t>() as ptrdiff_t,
        4 as ::core::ffi::c_int,
        alpha,
        beta,
    );
}



unsafe extern "C" fn h264_h_loop_filter_luma_intra_8_c(
    mut pix: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    h264_loop_filter_luma_intra_8_c(
        pix,
        ::core::mem::size_of::<uint8_t>() as ptrdiff_t,
        stride,
        4 as ::core::ffi::c_int,
        alpha,
        beta,
    );
}








unsafe extern "C" fn h264_h_loop_filter_luma_mbaff_intra_8_c(
    mut pix: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    h264_loop_filter_luma_intra_8_c(
        pix,
        ::core::mem::size_of::<uint8_t>() as ptrdiff_t,
        stride,
        2 as ::core::ffi::c_int,
        alpha,
        beta,
    );
}




#[inline(always)]
unsafe extern "C" fn h264_loop_filter_chroma_8_c(
    mut p_pix: *mut uint8_t,
    mut xstride: ptrdiff_t,
    mut ystride: ptrdiff_t,
    mut inner_iters: ::core::ffi::c_int,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc0: *mut int8_t,
) {
    let mut pix: *mut uint8_t = p_pix;
    let mut i: ::core::ffi::c_int = 0;
    let mut d: ::core::ffi::c_int = 0;
    alpha <<= 8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int;
    beta <<= 8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int;
    xstride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    ystride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        let tc: ::core::ffi::c_int = ((*tc0.offset(i as isize) as ::core::ffi::c_uint)
            .wrapping_sub(1 as ::core::ffi::c_uint)
            << 8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
            .wrapping_add(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
        if tc <= 0 as ::core::ffi::c_int {
            pix = pix.offset((inner_iters as ptrdiff_t * ystride) as isize);
        } else {
            d = 0 as ::core::ffi::c_int;
            while d < inner_iters {
                let p0: ::core::ffi::c_int = *pix
                    .offset((-(1 as ptrdiff_t) * xstride) as isize)
                    as ::core::ffi::c_int;
                let p1: ::core::ffi::c_int = *pix
                    .offset((-(2 as ptrdiff_t) * xstride) as isize)
                    as ::core::ffi::c_int;
                let q0: ::core::ffi::c_int = *pix
                    .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
                let q1: ::core::ffi::c_int = *pix
                    .offset((1 as ptrdiff_t * xstride) as isize) as ::core::ffi::c_int;
                if (if p0 - q0 >= 0 as ::core::ffi::c_int {
                    p0 - q0
                } else {
                    -(p0 - q0)
                }) < alpha
                    && (if p1 - p0 >= 0 as ::core::ffi::c_int {
                        p1 - p0
                    } else {
                        -(p1 - p0)
                    }) < beta
                    && (if q1 - q0 >= 0 as ::core::ffi::c_int {
                        q1 - q0
                    } else {
                        -(q1 - q0)
                    }) < beta
                {
                    let mut delta: ::core::ffi::c_int = av_clip_c(
                        (q0 - p0) * 4 as ::core::ffi::c_int + (p1 - q1)
                            + 4 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int,
                        -tc,
                        tc,
                    );
                    *pix.offset(-xstride as isize) = av_clip_uint8_c(p0 + delta);
                    *pix.offset(0 as ::core::ffi::c_int as isize) = av_clip_uint8_c(
                        q0 - delta,
                    );
                }
                pix = pix.offset(ystride as isize);
                d += 1;
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn h264_v_loop_filter_chroma_8_c(
    mut pix: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc0: *mut int8_t,
) {
    h264_loop_filter_chroma_8_c(
        pix,
        stride,
        ::core::mem::size_of::<uint8_t>() as ptrdiff_t,
        2 as ::core::ffi::c_int,
        alpha,
        beta,
        tc0,
    );
}







unsafe extern "C" fn h264_h_loop_filter_chroma_8_c(
    mut pix: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc0: *mut int8_t,
) {
    h264_loop_filter_chroma_8_c(
        pix,
        ::core::mem::size_of::<uint8_t>() as ptrdiff_t,
        stride,
        2 as ::core::ffi::c_int,
        alpha,
        beta,
        tc0,
    );
}



unsafe extern "C" fn h264_h_loop_filter_chroma_mbaff_8_c(
    mut pix: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc0: *mut int8_t,
) {
    h264_loop_filter_chroma_8_c(
        pix,
        ::core::mem::size_of::<uint8_t>() as ptrdiff_t,
        stride,
        1 as ::core::ffi::c_int,
        alpha,
        beta,
        tc0,
    );
}


unsafe extern "C" fn h264_h_loop_filter_chroma422_8_c(
    mut pix: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc0: *mut int8_t,
) {
    h264_loop_filter_chroma_8_c(
        pix,
        ::core::mem::size_of::<uint8_t>() as ptrdiff_t,
        stride,
        4 as ::core::ffi::c_int,
        alpha,
        beta,
        tc0,
    );
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


unsafe extern "C" fn h264_h_loop_filter_chroma422_mbaff_8_c(
    mut pix: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc0: *mut int8_t,
) {
    h264_loop_filter_chroma_8_c(
        pix,
        ::core::mem::size_of::<uint8_t>() as ptrdiff_t,
        stride,
        2 as ::core::ffi::c_int,
        alpha,
        beta,
        tc0,
    );
}

#[inline(always)]
unsafe extern "C" fn h264_loop_filter_chroma_intra_8_c(
    mut p_pix: *mut uint8_t,
    mut xstride: ptrdiff_t,
    mut ystride: ptrdiff_t,
    mut inner_iters: ::core::ffi::c_int,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    let mut pix: *mut uint8_t = p_pix;
    let mut d: ::core::ffi::c_int = 0;
    xstride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    ystride >>= (::core::mem::size_of::<uint8_t>() as usize).wrapping_sub(1 as usize);
    alpha <<= 8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int;
    beta <<= 8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int;
    d = 0 as ::core::ffi::c_int;
    while d < 4 as ::core::ffi::c_int * inner_iters {
        let p0: ::core::ffi::c_int = *pix.offset((-(1 as ptrdiff_t) * xstride) as isize)
            as ::core::ffi::c_int;
        let p1: ::core::ffi::c_int = *pix.offset((-(2 as ptrdiff_t) * xstride) as isize)
            as ::core::ffi::c_int;
        let q0: ::core::ffi::c_int = *pix.offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int;
        let q1: ::core::ffi::c_int = *pix.offset((1 as ptrdiff_t * xstride) as isize)
            as ::core::ffi::c_int;
        if (if p0 - q0 >= 0 as ::core::ffi::c_int { p0 - q0 } else { -(p0 - q0) })
            < alpha
            && (if p1 - p0 >= 0 as ::core::ffi::c_int { p1 - p0 } else { -(p1 - p0) })
                < beta
            && (if q1 - q0 >= 0 as ::core::ffi::c_int { q1 - q0 } else { -(q1 - q0) })
                < beta
        {
            *pix.offset(-xstride as isize) = (2 as ::core::ffi::c_int * p1 + p0 + q1
                + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as uint8_t;
            *pix.offset(0 as ::core::ffi::c_int as isize) = (2 as ::core::ffi::c_int * q1
                + q0 + p1 + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                as uint8_t;
        }
        pix = pix.offset(ystride as isize);
        d += 1;
    }
}






unsafe extern "C" fn h264_v_loop_filter_chroma_intra_8_c(
    mut pix: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    h264_loop_filter_chroma_intra_8_c(
        pix,
        stride,
        ::core::mem::size_of::<uint8_t>() as ptrdiff_t,
        2 as ::core::ffi::c_int,
        alpha,
        beta,
    );
}





unsafe extern "C" fn h264_h_loop_filter_chroma_intra_8_c(
    mut pix: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    h264_loop_filter_chroma_intra_8_c(
        pix,
        ::core::mem::size_of::<uint8_t>() as ptrdiff_t,
        stride,
        2 as ::core::ffi::c_int,
        alpha,
        beta,
    );
}

unsafe extern "C" fn h264_h_loop_filter_chroma_mbaff_intra_8_c(
    mut pix: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    h264_loop_filter_chroma_intra_8_c(
        pix,
        ::core::mem::size_of::<uint8_t>() as ptrdiff_t,
        stride,
        1 as ::core::ffi::c_int,
        alpha,
        beta,
    );
}





unsafe extern "C" fn h264_h_loop_filter_chroma422_intra_8_c(
    mut pix: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    h264_loop_filter_chroma_intra_8_c(
        pix,
        ::core::mem::size_of::<uint8_t>() as ptrdiff_t,
        stride,
        4 as ::core::ffi::c_int,
        alpha,
        beta,
    );
}



unsafe extern "C" fn h264_h_loop_filter_chroma422_mbaff_intra_8_c(
    mut pix: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    h264_loop_filter_chroma_intra_8_c(
        pix,
        ::core::mem::size_of::<uint8_t>() as ptrdiff_t,
        stride,
        2 as ::core::ffi::c_int,
        alpha,
        beta,
    );
}
