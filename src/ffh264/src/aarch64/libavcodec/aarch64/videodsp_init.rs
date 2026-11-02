// Translated from FFmpeg 9.0.1 libavcodec/aarch64/videodsp_init.c
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
    fn ff_prefetch_aarch64(
        mem: *const uint8_t,
        stride: ptrdiff_t,
        h: ::core::ffi::c_int,
    );
    fn av_get_cpu_flags() -> ::core::ffi::c_int;
}
pub type ptrdiff_t = ::core::ffi::c_long;
pub type uint8_t = ::core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoDSPContext {
    pub emulated_edge_mc: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            *const uint8_t,
            ptrdiff_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch: Option<
        unsafe extern "C" fn(*const uint8_t, ptrdiff_t, ::core::ffi::c_int) -> (),
    >,
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ff_videodsp_init_aarch64(
    mut ctx: *mut VideoDSPContext,
    mut bpc: ::core::ffi::c_int,
) {
    let mut cpu_flags: ::core::ffi::c_int = av_get_cpu_flags();
    if 1 as ::core::ffi::c_int != 0
        && cpu_flags & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int != 0
    {
        (*ctx).prefetch = Some(
            ff_prefetch_aarch64
                as unsafe extern "C" fn(
                    *const uint8_t,
                    ptrdiff_t,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(*const uint8_t, ptrdiff_t, ::core::ffi::c_int) -> (),
            >;
    }
}
