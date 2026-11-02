// Translated from FFmpeg 9.0.1 libavutil/aarch64/cpu.c
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
    fn ff_getauxval(type_0: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    fn av_get_cpu_flags() -> ::core::ffi::c_int;
}
pub type size_t = ::core::ffi::c_ulong;
unsafe extern "C" fn detect_flags() -> ::core::ffi::c_int {
    let mut flags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut hwcap: ::core::ffi::c_ulong = ff_getauxval(16 as ::core::ffi::c_ulong);
    let mut hwcap2: ::core::ffi::c_ulong = ff_getauxval(26 as ::core::ffi::c_ulong);
    if hwcap
        & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as ::core::ffi::c_ulong
        != 0
    {
        flags |= (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int;
    }
    if hwcap
        & ((1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int) as ::core::ffi::c_ulong
        != 0
    {
        flags |= (1 as ::core::ffi::c_int) << 18 as ::core::ffi::c_int;
    }
    if hwcap
        & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as ::core::ffi::c_ulong
        != 0
    {
        flags |= (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int;
    }
    if hwcap
        & ((1 as ::core::ffi::c_int) << 20 as ::core::ffi::c_int) as ::core::ffi::c_ulong
        != 0
    {
        flags |= (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    }
    if hwcap
        & ((1 as ::core::ffi::c_int) << 22 as ::core::ffi::c_int) as ::core::ffi::c_ulong
        != 0
    {
        flags |= (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int;
    }
    if hwcap2
        & ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong
        != 0
    {
        flags |= (1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int;
    }
    if hwcap2
        & ((1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as ::core::ffi::c_ulong
        != 0
    {
        flags |= (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int;
    }
    if hwcap2
        & ((1 as ::core::ffi::c_int) << 23 as ::core::ffi::c_int) as ::core::ffi::c_ulong
        != 0
    {
        flags |= (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int;
    }
    if hwcap2
        & ((1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int) as ::core::ffi::c_ulong
        != 0
    {
        flags |= (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int;
    }
    if hwcap2 as ::core::ffi::c_ulonglong
        & (1 as ::core::ffi::c_ulonglong) << 37 as ::core::ffi::c_int != 0
    {
        flags |= (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int;
    }
    return flags;
}
#[no_mangle]
pub unsafe extern "C" fn ff_get_cpu_flags_aarch64() -> ::core::ffi::c_int {
    let mut flags: ::core::ffi::c_int = ((1 as ::core::ffi::c_int)
        << 6 as ::core::ffi::c_int) * 1 as ::core::ffi::c_int
        | ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
            * 1 as ::core::ffi::c_int;
    flags |= (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    flags |= (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int;
    flags |= (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int;
    flags |= detect_flags();
    return flags;
}
#[no_mangle]
pub unsafe extern "C" fn ff_get_cpu_max_align_aarch64() -> size_t {
    let mut flags: ::core::ffi::c_int = av_get_cpu_flags();
    if flags & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0 {
        return 16 as size_t;
    }
    return 8 as size_t;
}
