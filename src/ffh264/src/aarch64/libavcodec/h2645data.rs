// Translated from FFmpeg 9.0.1 libavcodec/h2645data.c
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
pub struct AVRational {
    pub num: ::core::ffi::c_int,
    pub den: ::core::ffi::c_int,
}
#[no_mangle]
pub static mut ff_h2645_pixel_aspect: [AVRational; 17] = [
    AVRational {
        num: 0 as ::core::ffi::c_int,
        den: 1 as ::core::ffi::c_int,
    },
    AVRational {
        num: 1 as ::core::ffi::c_int,
        den: 1 as ::core::ffi::c_int,
    },
    AVRational {
        num: 12 as ::core::ffi::c_int,
        den: 11 as ::core::ffi::c_int,
    },
    AVRational {
        num: 10 as ::core::ffi::c_int,
        den: 11 as ::core::ffi::c_int,
    },
    AVRational {
        num: 16 as ::core::ffi::c_int,
        den: 11 as ::core::ffi::c_int,
    },
    AVRational {
        num: 40 as ::core::ffi::c_int,
        den: 33 as ::core::ffi::c_int,
    },
    AVRational {
        num: 24 as ::core::ffi::c_int,
        den: 11 as ::core::ffi::c_int,
    },
    AVRational {
        num: 20 as ::core::ffi::c_int,
        den: 11 as ::core::ffi::c_int,
    },
    AVRational {
        num: 32 as ::core::ffi::c_int,
        den: 11 as ::core::ffi::c_int,
    },
    AVRational {
        num: 80 as ::core::ffi::c_int,
        den: 33 as ::core::ffi::c_int,
    },
    AVRational {
        num: 18 as ::core::ffi::c_int,
        den: 11 as ::core::ffi::c_int,
    },
    AVRational {
        num: 15 as ::core::ffi::c_int,
        den: 11 as ::core::ffi::c_int,
    },
    AVRational {
        num: 64 as ::core::ffi::c_int,
        den: 33 as ::core::ffi::c_int,
    },
    AVRational {
        num: 160 as ::core::ffi::c_int,
        den: 99 as ::core::ffi::c_int,
    },
    AVRational {
        num: 4 as ::core::ffi::c_int,
        den: 3 as ::core::ffi::c_int,
    },
    AVRational {
        num: 3 as ::core::ffi::c_int,
        den: 2 as ::core::ffi::c_int,
    },
    AVRational {
        num: 2 as ::core::ffi::c_int,
        den: 1 as ::core::ffi::c_int,
    },
];
