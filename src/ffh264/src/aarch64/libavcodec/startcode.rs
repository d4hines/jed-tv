// Translated from FFmpeg 9.0.1 libavcodec/startcode.c
// (preprocessed for aarch64-unknown-linux-musl) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
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

pub type uint8_t = ::core::ffi::c_uchar;
pub type uint64_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union unaligned_64 {
    pub l: uint64_t,
}
#[no_mangle]
pub unsafe extern "C" fn ff_startcode_find_candidate_c(
    mut buf: *const uint8_t,
    mut size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < size
        && !(*(buf.offset(i as isize) as *const unaligned_64)).l
            as ::core::ffi::c_ulonglong
            & ((*(buf.offset(i as isize) as *const unaligned_64)).l
                as ::core::ffi::c_ulonglong)
                .wrapping_sub(0x101010101010101 as ::core::ffi::c_ulonglong)
            & 0x8080808080808080 as ::core::ffi::c_ulonglong == 0
    {
        i += 8 as ::core::ffi::c_int;
    }
    while i < size {
        if *buf.offset(i as isize) == 0 {
            break;
        }
        i += 1;
    }
    return i;
}
