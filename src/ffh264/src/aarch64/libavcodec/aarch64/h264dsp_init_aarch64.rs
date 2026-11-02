// Translated from FFmpeg 9.0.1 libavcodec/aarch64/h264dsp_init_aarch64.c
// (preprocessed for aarch64-unknown-linux-musl) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// Copyright (c) 2010 Mans Rullgard <mans@mansr.com>
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
    fn ff_h264_v_loop_filter_luma_neon(
        pix: *mut uint8_t,
        stride: ptrdiff_t,
        alpha: ::core::ffi::c_int,
        beta: ::core::ffi::c_int,
        tc0: *mut int8_t,
    );
    fn ff_h264_h_loop_filter_luma_neon(
        pix: *mut uint8_t,
        stride: ptrdiff_t,
        alpha: ::core::ffi::c_int,
        beta: ::core::ffi::c_int,
        tc0: *mut int8_t,
    );
    fn ff_h264_v_loop_filter_luma_intra_neon(
        pix: *mut uint8_t,
        stride: ptrdiff_t,
        alpha: ::core::ffi::c_int,
        beta: ::core::ffi::c_int,
    );
    fn ff_h264_h_loop_filter_luma_intra_neon(
        pix: *mut uint8_t,
        stride: ptrdiff_t,
        alpha: ::core::ffi::c_int,
        beta: ::core::ffi::c_int,
    );
    fn ff_h264_v_loop_filter_chroma_neon(
        pix: *mut uint8_t,
        stride: ptrdiff_t,
        alpha: ::core::ffi::c_int,
        beta: ::core::ffi::c_int,
        tc0: *mut int8_t,
    );
    fn ff_h264_h_loop_filter_chroma_neon(
        pix: *mut uint8_t,
        stride: ptrdiff_t,
        alpha: ::core::ffi::c_int,
        beta: ::core::ffi::c_int,
        tc0: *mut int8_t,
    );
    fn ff_h264_h_loop_filter_chroma422_neon(
        pix: *mut uint8_t,
        stride: ptrdiff_t,
        alpha: ::core::ffi::c_int,
        beta: ::core::ffi::c_int,
        tc0: *mut int8_t,
    );
    fn ff_h264_v_loop_filter_chroma_intra_neon(
        pix: *mut uint8_t,
        stride: ptrdiff_t,
        alpha: ::core::ffi::c_int,
        beta: ::core::ffi::c_int,
    );
    fn ff_h264_h_loop_filter_chroma_intra_neon(
        pix: *mut uint8_t,
        stride: ptrdiff_t,
        alpha: ::core::ffi::c_int,
        beta: ::core::ffi::c_int,
    );
    fn ff_h264_h_loop_filter_chroma422_intra_neon(
        pix: *mut uint8_t,
        stride: ptrdiff_t,
        alpha: ::core::ffi::c_int,
        beta: ::core::ffi::c_int,
    );
    fn ff_h264_h_loop_filter_chroma_mbaff_intra_neon(
        pix: *mut uint8_t,
        stride: ptrdiff_t,
        alpha: ::core::ffi::c_int,
        beta: ::core::ffi::c_int,
    );
    fn ff_weight_h264_pixels_16_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        height: ::core::ffi::c_int,
        log2_den: ::core::ffi::c_int,
        weight: ::core::ffi::c_int,
        offset: ::core::ffi::c_int,
    );
    fn ff_weight_h264_pixels_8_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        height: ::core::ffi::c_int,
        log2_den: ::core::ffi::c_int,
        weight: ::core::ffi::c_int,
        offset: ::core::ffi::c_int,
    );
    fn ff_weight_h264_pixels_4_neon(
        dst: *mut uint8_t,
        stride: ptrdiff_t,
        height: ::core::ffi::c_int,
        log2_den: ::core::ffi::c_int,
        weight: ::core::ffi::c_int,
        offset: ::core::ffi::c_int,
    );
    fn ff_biweight_h264_pixels_16_neon(
        dst: *mut uint8_t,
        src: *mut uint8_t,
        stride: ptrdiff_t,
        height: ::core::ffi::c_int,
        log2_den: ::core::ffi::c_int,
        weightd: ::core::ffi::c_int,
        weights: ::core::ffi::c_int,
        offset: ::core::ffi::c_int,
    );
    fn ff_biweight_h264_pixels_8_neon(
        dst: *mut uint8_t,
        src: *mut uint8_t,
        stride: ptrdiff_t,
        height: ::core::ffi::c_int,
        log2_den: ::core::ffi::c_int,
        weightd: ::core::ffi::c_int,
        weights: ::core::ffi::c_int,
        offset: ::core::ffi::c_int,
    );
    fn ff_biweight_h264_pixels_4_neon(
        dst: *mut uint8_t,
        src: *mut uint8_t,
        stride: ptrdiff_t,
        height: ::core::ffi::c_int,
        log2_den: ::core::ffi::c_int,
        weightd: ::core::ffi::c_int,
        weights: ::core::ffi::c_int,
        offset: ::core::ffi::c_int,
    );
    fn ff_h264_idct_add_neon(
        dst: *mut uint8_t,
        block: *mut int16_t,
        stride: ::core::ffi::c_int,
    );
    fn ff_h264_idct_dc_add_neon(
        dst: *mut uint8_t,
        block: *mut int16_t,
        stride: ::core::ffi::c_int,
    );
    fn ff_h264_idct_add16_neon(
        dst: *mut uint8_t,
        block_offset: *const ::core::ffi::c_int,
        block: *mut int16_t,
        stride: ::core::ffi::c_int,
        nnzc: *const uint8_t,
    );
    fn ff_h264_idct_add16intra_neon(
        dst: *mut uint8_t,
        block_offset: *const ::core::ffi::c_int,
        block: *mut int16_t,
        stride: ::core::ffi::c_int,
        nnzc: *const uint8_t,
    );
    fn ff_h264_idct_add8_neon(
        dest: *mut *mut uint8_t,
        block_offset: *const ::core::ffi::c_int,
        block: *mut int16_t,
        stride: ::core::ffi::c_int,
        nnzc: *const uint8_t,
    );
    fn ff_h264_idct8_add_neon(
        dst: *mut uint8_t,
        block: *mut int16_t,
        stride: ::core::ffi::c_int,
    );
    fn ff_h264_idct8_dc_add_neon(
        dst: *mut uint8_t,
        block: *mut int16_t,
        stride: ::core::ffi::c_int,
    );
    fn ff_h264_idct8_add4_neon(
        dst: *mut uint8_t,
        block_offset: *const ::core::ffi::c_int,
        block: *mut int16_t,
        stride: ::core::ffi::c_int,
        nnzc: *const uint8_t,
    );
    fn ff_h264_v_loop_filter_chroma_neon_10(
        pix: *mut uint8_t,
        stride: ptrdiff_t,
        alpha: ::core::ffi::c_int,
        beta: ::core::ffi::c_int,
        tc0: *mut int8_t,
    );
    fn ff_h264_h_loop_filter_chroma_neon_10(
        pix: *mut uint8_t,
        stride: ptrdiff_t,
        alpha: ::core::ffi::c_int,
        beta: ::core::ffi::c_int,
        tc0: *mut int8_t,
    );
    fn ff_h264_h_loop_filter_chroma422_neon_10(
        pix: *mut uint8_t,
        stride: ptrdiff_t,
        alpha: ::core::ffi::c_int,
        beta: ::core::ffi::c_int,
        tc0: *mut int8_t,
    );
    fn ff_h264_v_loop_filter_chroma_intra_neon_10(
        pix: *mut uint8_t,
        stride: ptrdiff_t,
        alpha: ::core::ffi::c_int,
        beta: ::core::ffi::c_int,
    );
    fn ff_h264_h_loop_filter_chroma_intra_neon_10(
        pix: *mut uint8_t,
        stride: ptrdiff_t,
        alpha: ::core::ffi::c_int,
        beta: ::core::ffi::c_int,
    );
    fn ff_h264_h_loop_filter_chroma422_intra_neon_10(
        pix: *mut uint8_t,
        stride: ptrdiff_t,
        alpha: ::core::ffi::c_int,
        beta: ::core::ffi::c_int,
    );
    fn ff_h264_h_loop_filter_chroma_mbaff_intra_neon_10(
        pix: *mut uint8_t,
        stride: ptrdiff_t,
        alpha: ::core::ffi::c_int,
        beta: ::core::ffi::c_int,
    );
    fn av_get_cpu_flags() -> ::core::ffi::c_int;
}
pub type int8_t = ::core::ffi::c_schar;
pub type int16_t = ::core::ffi::c_short;
pub type uint8_t = ::core::ffi::c_uchar;
pub type ptrdiff_t = ::core::ffi::c_long;
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
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ff_h264dsp_init_aarch64(
    mut c: *mut H264DSPContext,
    bit_depth: ::core::ffi::c_int,
    chroma_format_idc: ::core::ffi::c_int,
) {
    let mut cpu_flags: ::core::ffi::c_int = av_get_cpu_flags();
    if 1 as ::core::ffi::c_int != 0
        && cpu_flags & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0
        && bit_depth == 8 as ::core::ffi::c_int
    {
        (*c).v_loop_filter_luma = Some(
            ff_h264_v_loop_filter_luma_neon
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
            ff_h264_h_loop_filter_luma_neon
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
            ff_h264_v_loop_filter_luma_intra_neon
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
            ff_h264_h_loop_filter_luma_intra_neon
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
            ff_h264_v_loop_filter_chroma_neon
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
        (*c).v_loop_filter_chroma_intra = Some(
            ff_h264_v_loop_filter_chroma_intra_neon
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
            (*c).h_loop_filter_chroma = Some(
                ff_h264_h_loop_filter_chroma_neon
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
            (*c).h_loop_filter_chroma_intra = Some(
                ff_h264_h_loop_filter_chroma_intra_neon
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
            (*c).h_loop_filter_chroma_mbaff_intra = Some(
                ff_h264_h_loop_filter_chroma_mbaff_intra_neon
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
            (*c).h_loop_filter_chroma = Some(
                ff_h264_h_loop_filter_chroma422_neon
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
            (*c).h_loop_filter_chroma_mbaff = Some(
                ff_h264_h_loop_filter_chroma_neon
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
            (*c).h_loop_filter_chroma_intra = Some(
                ff_h264_h_loop_filter_chroma422_intra_neon
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
            (*c).h_loop_filter_chroma_mbaff_intra = Some(
                ff_h264_h_loop_filter_chroma_intra_neon
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
        (*c).weight_pixels_tab[0 as ::core::ffi::c_int as usize] = Some(
            ff_weight_h264_pixels_16_neon
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
            ff_weight_h264_pixels_8_neon
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
            ff_weight_h264_pixels_4_neon
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
            ff_biweight_h264_pixels_16_neon
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
            ff_biweight_h264_pixels_8_neon
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
            ff_biweight_h264_pixels_4_neon
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
        (*c).idct_add = Some(
            ff_h264_idct_add_neon
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
            ff_h264_idct_dc_add_neon
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
            ff_h264_idct_add16_neon
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
        (*c).idct_add16intra = Some(
            ff_h264_idct_add16intra_neon
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
                ff_h264_idct_add8_neon
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
        (*c).idct8_add = Some(
            ff_h264_idct8_add_neon
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
            ff_h264_idct8_dc_add_neon
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
        (*c).idct8_add4 = Some(
            ff_h264_idct8_add4_neon
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
    } else if 1 as ::core::ffi::c_int != 0
        && cpu_flags & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0
        && bit_depth == 10 as ::core::ffi::c_int
    {
        (*c).v_loop_filter_chroma = Some(
            ff_h264_v_loop_filter_chroma_neon_10
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
        (*c).v_loop_filter_chroma_intra = Some(
            ff_h264_v_loop_filter_chroma_intra_neon_10
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
            (*c).h_loop_filter_chroma = Some(
                ff_h264_h_loop_filter_chroma_neon_10
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
            (*c).h_loop_filter_chroma_intra = Some(
                ff_h264_h_loop_filter_chroma_intra_neon_10
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
            (*c).h_loop_filter_chroma_mbaff_intra = Some(
                ff_h264_h_loop_filter_chroma_mbaff_intra_neon_10
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
            (*c).h_loop_filter_chroma = Some(
                ff_h264_h_loop_filter_chroma422_neon_10
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
            (*c).h_loop_filter_chroma_mbaff = Some(
                ff_h264_h_loop_filter_chroma_neon_10
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
            (*c).h_loop_filter_chroma_intra = Some(
                ff_h264_h_loop_filter_chroma422_intra_neon_10
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
            (*c).h_loop_filter_chroma_mbaff_intra = Some(
                ff_h264_h_loop_filter_chroma_intra_neon_10
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
    }
}
