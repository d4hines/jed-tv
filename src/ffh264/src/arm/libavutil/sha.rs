// Translated from FFmpeg 9.0.1 libavutil/sha.c
// (preprocessed for armv7-unknown-linux-musleabihf) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// Copyright (C) 2007 Michael Niedermayer <michaelni@gmx.at>
// Copyright (C) 2009 Konstantin Shishkov
// based on public domain SHA-1 code by Steve Reid <steve@edmweb.com>
// and on BSD-licensed SHA-2 code by Aaron D. Gifford
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
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn av_mallocz(size: size_t) -> *mut ::core::ffi::c_void;
}
pub type size_t = ::core::ffi::c_uint;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint64_t = ::core::ffi::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVSHA {
    pub digest_len: uint8_t,
    pub count: uint64_t,
    pub buffer: [uint8_t; 64],
    pub state: [uint32_t; 8],
    pub transform: Option<unsafe extern "C" fn(*mut uint32_t, *const uint8_t) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union unaligned_32 {
    pub l: uint32_t,
}
#[no_mangle]
pub static mut av_sha_size: ::core::ffi::c_int = ::core::mem::size_of::<AVSHA>()
    as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn av_sha_alloc() -> *mut AVSHA {
    return av_mallocz(::core::mem::size_of::<AVSHA>() as size_t) as *mut AVSHA;
}
#[inline(always)]
unsafe extern "C" fn av_bswap32(mut x: uint32_t) -> uint32_t {
    return (x << 8 as ::core::ffi::c_int & 0xff00 as uint32_t
        | x >> 8 as ::core::ffi::c_int & 0xff as uint32_t) << 16 as ::core::ffi::c_int
        | ((x >> 16 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            & 0xff00 as uint32_t
            | x >> 16 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
                & 0xff as uint32_t);
}
unsafe extern "C" fn sha1_transform(
    mut state: *mut uint32_t,
    mut buffer: *const uint8_t,
) {
    let mut block: [uint32_t; 80] = [0; 80];
    let mut i: ::core::ffi::c_uint = 0;
    let mut a: ::core::ffi::c_uint = 0;
    let mut b: ::core::ffi::c_uint = 0;
    let mut c: ::core::ffi::c_uint = 0;
    let mut d: ::core::ffi::c_uint = 0;
    let mut e: ::core::ffi::c_uint = 0;
    a = *state.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    b = *state.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    c = *state.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    d = *state.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    e = *state.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    i = 0 as ::core::ffi::c_uint;
    block[(0 as ::core::ffi::c_uint).wrapping_add(i) as usize] = av_bswap32(
        (*(buffer
            .offset(
                (4 as ::core::ffi::c_uint)
                    .wrapping_mul((0 as ::core::ffi::c_uint).wrapping_add(i)) as isize,
            ) as *const unaligned_32))
            .l,
    );
    e = e
        .wrapping_add(
            (b & (c ^ d) ^ d)
                .wrapping_add(block[(0 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x5a827999 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    a << 5 as ::core::ffi::c_int
                        | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    b = b << 30 as ::core::ffi::c_int
        | b >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(1 as ::core::ffi::c_uint).wrapping_add(i) as usize] = av_bswap32(
        (*(buffer
            .offset(
                (4 as ::core::ffi::c_uint)
                    .wrapping_mul((1 as ::core::ffi::c_uint).wrapping_add(i)) as isize,
            ) as *const unaligned_32))
            .l,
    );
    d = d
        .wrapping_add(
            (a & (b ^ c) ^ c)
                .wrapping_add(block[(1 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x5a827999 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    e << 5 as ::core::ffi::c_int
                        | e >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    a = a << 30 as ::core::ffi::c_int
        | a >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(2 as ::core::ffi::c_uint).wrapping_add(i) as usize] = av_bswap32(
        (*(buffer
            .offset(
                (4 as ::core::ffi::c_uint)
                    .wrapping_mul((2 as ::core::ffi::c_uint).wrapping_add(i)) as isize,
            ) as *const unaligned_32))
            .l,
    );
    c = c
        .wrapping_add(
            (e & (a ^ b) ^ b)
                .wrapping_add(block[(2 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x5a827999 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    d << 5 as ::core::ffi::c_int
                        | d >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    e = e << 30 as ::core::ffi::c_int
        | e >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(3 as ::core::ffi::c_uint).wrapping_add(i) as usize] = av_bswap32(
        (*(buffer
            .offset(
                (4 as ::core::ffi::c_uint)
                    .wrapping_mul((3 as ::core::ffi::c_uint).wrapping_add(i)) as isize,
            ) as *const unaligned_32))
            .l,
    );
    b = b
        .wrapping_add(
            (d & (e ^ a) ^ a)
                .wrapping_add(block[(3 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x5a827999 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    c << 5 as ::core::ffi::c_int
                        | c >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    d = d << 30 as ::core::ffi::c_int
        | d >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(4 as ::core::ffi::c_uint).wrapping_add(i) as usize] = av_bswap32(
        (*(buffer
            .offset(
                (4 as ::core::ffi::c_uint)
                    .wrapping_mul((4 as ::core::ffi::c_uint).wrapping_add(i)) as isize,
            ) as *const unaligned_32))
            .l,
    );
    a = a
        .wrapping_add(
            (c & (d ^ e) ^ e)
                .wrapping_add(block[(4 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x5a827999 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    b << 5 as ::core::ffi::c_int
                        | b >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    c = c << 30 as ::core::ffi::c_int
        | c >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    i = i.wrapping_add(5 as ::core::ffi::c_uint);
    block[(0 as ::core::ffi::c_uint).wrapping_add(i) as usize] = av_bswap32(
        (*(buffer
            .offset(
                (4 as ::core::ffi::c_uint)
                    .wrapping_mul((0 as ::core::ffi::c_uint).wrapping_add(i)) as isize,
            ) as *const unaligned_32))
            .l,
    );
    e = e
        .wrapping_add(
            (b & (c ^ d) ^ d)
                .wrapping_add(block[(0 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x5a827999 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    a << 5 as ::core::ffi::c_int
                        | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    b = b << 30 as ::core::ffi::c_int
        | b >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(1 as ::core::ffi::c_uint).wrapping_add(i) as usize] = av_bswap32(
        (*(buffer
            .offset(
                (4 as ::core::ffi::c_uint)
                    .wrapping_mul((1 as ::core::ffi::c_uint).wrapping_add(i)) as isize,
            ) as *const unaligned_32))
            .l,
    );
    d = d
        .wrapping_add(
            (a & (b ^ c) ^ c)
                .wrapping_add(block[(1 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x5a827999 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    e << 5 as ::core::ffi::c_int
                        | e >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    a = a << 30 as ::core::ffi::c_int
        | a >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(2 as ::core::ffi::c_uint).wrapping_add(i) as usize] = av_bswap32(
        (*(buffer
            .offset(
                (4 as ::core::ffi::c_uint)
                    .wrapping_mul((2 as ::core::ffi::c_uint).wrapping_add(i)) as isize,
            ) as *const unaligned_32))
            .l,
    );
    c = c
        .wrapping_add(
            (e & (a ^ b) ^ b)
                .wrapping_add(block[(2 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x5a827999 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    d << 5 as ::core::ffi::c_int
                        | d >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    e = e << 30 as ::core::ffi::c_int
        | e >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(3 as ::core::ffi::c_uint).wrapping_add(i) as usize] = av_bswap32(
        (*(buffer
            .offset(
                (4 as ::core::ffi::c_uint)
                    .wrapping_mul((3 as ::core::ffi::c_uint).wrapping_add(i)) as isize,
            ) as *const unaligned_32))
            .l,
    );
    b = b
        .wrapping_add(
            (d & (e ^ a) ^ a)
                .wrapping_add(block[(3 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x5a827999 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    c << 5 as ::core::ffi::c_int
                        | c >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    d = d << 30 as ::core::ffi::c_int
        | d >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(4 as ::core::ffi::c_uint).wrapping_add(i) as usize] = av_bswap32(
        (*(buffer
            .offset(
                (4 as ::core::ffi::c_uint)
                    .wrapping_mul((4 as ::core::ffi::c_uint).wrapping_add(i)) as isize,
            ) as *const unaligned_32))
            .l,
    );
    a = a
        .wrapping_add(
            (c & (d ^ e) ^ e)
                .wrapping_add(block[(4 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x5a827999 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    b << 5 as ::core::ffi::c_int
                        | b >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    c = c << 30 as ::core::ffi::c_int
        | c >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    i = i.wrapping_add(5 as ::core::ffi::c_uint);
    block[(0 as ::core::ffi::c_uint).wrapping_add(i) as usize] = av_bswap32(
        (*(buffer
            .offset(
                (4 as ::core::ffi::c_uint)
                    .wrapping_mul((0 as ::core::ffi::c_uint).wrapping_add(i)) as isize,
            ) as *const unaligned_32))
            .l,
    );
    e = e
        .wrapping_add(
            (b & (c ^ d) ^ d)
                .wrapping_add(block[(0 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x5a827999 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    a << 5 as ::core::ffi::c_int
                        | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    b = b << 30 as ::core::ffi::c_int
        | b >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(1 as ::core::ffi::c_uint).wrapping_add(i) as usize] = av_bswap32(
        (*(buffer
            .offset(
                (4 as ::core::ffi::c_uint)
                    .wrapping_mul((1 as ::core::ffi::c_uint).wrapping_add(i)) as isize,
            ) as *const unaligned_32))
            .l,
    );
    d = d
        .wrapping_add(
            (a & (b ^ c) ^ c)
                .wrapping_add(block[(1 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x5a827999 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    e << 5 as ::core::ffi::c_int
                        | e >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    a = a << 30 as ::core::ffi::c_int
        | a >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(2 as ::core::ffi::c_uint).wrapping_add(i) as usize] = av_bswap32(
        (*(buffer
            .offset(
                (4 as ::core::ffi::c_uint)
                    .wrapping_mul((2 as ::core::ffi::c_uint).wrapping_add(i)) as isize,
            ) as *const unaligned_32))
            .l,
    );
    c = c
        .wrapping_add(
            (e & (a ^ b) ^ b)
                .wrapping_add(block[(2 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x5a827999 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    d << 5 as ::core::ffi::c_int
                        | d >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    e = e << 30 as ::core::ffi::c_int
        | e >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(3 as ::core::ffi::c_uint).wrapping_add(i) as usize] = av_bswap32(
        (*(buffer
            .offset(
                (4 as ::core::ffi::c_uint)
                    .wrapping_mul((3 as ::core::ffi::c_uint).wrapping_add(i)) as isize,
            ) as *const unaligned_32))
            .l,
    );
    b = b
        .wrapping_add(
            (d & (e ^ a) ^ a)
                .wrapping_add(block[(3 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x5a827999 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    c << 5 as ::core::ffi::c_int
                        | c >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    d = d << 30 as ::core::ffi::c_int
        | d >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(4 as ::core::ffi::c_uint).wrapping_add(i) as usize] = av_bswap32(
        (*(buffer
            .offset(
                (4 as ::core::ffi::c_uint)
                    .wrapping_mul((4 as ::core::ffi::c_uint).wrapping_add(i)) as isize,
            ) as *const unaligned_32))
            .l,
    );
    a = a
        .wrapping_add(
            (c & (d ^ e) ^ e)
                .wrapping_add(block[(4 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x5a827999 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    b << 5 as ::core::ffi::c_int
                        | b >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    c = c << 30 as ::core::ffi::c_int
        | c >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    i = i.wrapping_add(5 as ::core::ffi::c_uint);
    block[15 as ::core::ffi::c_int as usize] = av_bswap32(
        (*(buffer.offset((4 as ::core::ffi::c_int * 15 as ::core::ffi::c_int) as isize)
            as *const unaligned_32))
            .l,
    );
    e = e
        .wrapping_add(
            (b & (c ^ d) ^ d)
                .wrapping_add(block[15 as ::core::ffi::c_int as usize])
                .wrapping_add(0x5a827999 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    a << 5 as ::core::ffi::c_int
                        | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    b = b << 30 as ::core::ffi::c_int
        | b >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[16 as ::core::ffi::c_int as usize] = (block[(16 as ::core::ffi::c_int
        - 3 as ::core::ffi::c_int) as usize]
        ^ block[(16 as ::core::ffi::c_int - 8 as ::core::ffi::c_int) as usize]
        ^ block[(16 as ::core::ffi::c_int - 14 as ::core::ffi::c_int) as usize]
        ^ block[(16 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(16 as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as usize]
            ^ block[(16 as ::core::ffi::c_int - 8 as ::core::ffi::c_int) as usize]
            ^ block[(16 as ::core::ffi::c_int - 14 as ::core::ffi::c_int) as usize]
            ^ block[(16 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    d = d
        .wrapping_add(
            (a & (b ^ c) ^ c)
                .wrapping_add(block[16 as ::core::ffi::c_int as usize])
                .wrapping_add(0x5a827999 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    e << 5 as ::core::ffi::c_int
                        | e >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    a = a << 30 as ::core::ffi::c_int
        | a >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[17 as ::core::ffi::c_int as usize] = (block[(17 as ::core::ffi::c_int
        - 3 as ::core::ffi::c_int) as usize]
        ^ block[(17 as ::core::ffi::c_int - 8 as ::core::ffi::c_int) as usize]
        ^ block[(17 as ::core::ffi::c_int - 14 as ::core::ffi::c_int) as usize]
        ^ block[(17 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(17 as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as usize]
            ^ block[(17 as ::core::ffi::c_int - 8 as ::core::ffi::c_int) as usize]
            ^ block[(17 as ::core::ffi::c_int - 14 as ::core::ffi::c_int) as usize]
            ^ block[(17 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    c = c
        .wrapping_add(
            (e & (a ^ b) ^ b)
                .wrapping_add(block[17 as ::core::ffi::c_int as usize])
                .wrapping_add(0x5a827999 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    d << 5 as ::core::ffi::c_int
                        | d >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    e = e << 30 as ::core::ffi::c_int
        | e >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[18 as ::core::ffi::c_int as usize] = (block[(18 as ::core::ffi::c_int
        - 3 as ::core::ffi::c_int) as usize]
        ^ block[(18 as ::core::ffi::c_int - 8 as ::core::ffi::c_int) as usize]
        ^ block[(18 as ::core::ffi::c_int - 14 as ::core::ffi::c_int) as usize]
        ^ block[(18 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(18 as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as usize]
            ^ block[(18 as ::core::ffi::c_int - 8 as ::core::ffi::c_int) as usize]
            ^ block[(18 as ::core::ffi::c_int - 14 as ::core::ffi::c_int) as usize]
            ^ block[(18 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    b = b
        .wrapping_add(
            (d & (e ^ a) ^ a)
                .wrapping_add(block[18 as ::core::ffi::c_int as usize])
                .wrapping_add(0x5a827999 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    c << 5 as ::core::ffi::c_int
                        | c >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    d = d << 30 as ::core::ffi::c_int
        | d >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[19 as ::core::ffi::c_int as usize] = (block[(19 as ::core::ffi::c_int
        - 3 as ::core::ffi::c_int) as usize]
        ^ block[(19 as ::core::ffi::c_int - 8 as ::core::ffi::c_int) as usize]
        ^ block[(19 as ::core::ffi::c_int - 14 as ::core::ffi::c_int) as usize]
        ^ block[(19 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(19 as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as usize]
            ^ block[(19 as ::core::ffi::c_int - 8 as ::core::ffi::c_int) as usize]
            ^ block[(19 as ::core::ffi::c_int - 14 as ::core::ffi::c_int) as usize]
            ^ block[(19 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    a = a
        .wrapping_add(
            (c & (d ^ e) ^ e)
                .wrapping_add(block[19 as ::core::ffi::c_int as usize])
                .wrapping_add(0x5a827999 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    b << 5 as ::core::ffi::c_int
                        | b >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    c = c << 30 as ::core::ffi::c_int
        | c >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    i = 20 as ::core::ffi::c_uint;
    block[(0 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(0
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    e = e
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(block[(0 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x6ed9eba1 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    a << 5 as ::core::ffi::c_int
                        | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    b = b << 30 as ::core::ffi::c_int
        | b >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(1 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(1
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    d = d
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(block[(1 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x6ed9eba1 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    e << 5 as ::core::ffi::c_int
                        | e >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    a = a << 30 as ::core::ffi::c_int
        | a >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(2 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(2
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    c = c
        .wrapping_add(
            (e ^ a ^ b)
                .wrapping_add(block[(2 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x6ed9eba1 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    d << 5 as ::core::ffi::c_int
                        | d >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    e = e << 30 as ::core::ffi::c_int
        | e >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(3 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(3
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    b = b
        .wrapping_add(
            (d ^ e ^ a)
                .wrapping_add(block[(3 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x6ed9eba1 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    c << 5 as ::core::ffi::c_int
                        | c >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    d = d << 30 as ::core::ffi::c_int
        | d >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(4 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(4
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    a = a
        .wrapping_add(
            (c ^ d ^ e)
                .wrapping_add(block[(4 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x6ed9eba1 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    b << 5 as ::core::ffi::c_int
                        | b >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    c = c << 30 as ::core::ffi::c_int
        | c >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    i = i.wrapping_add(5 as ::core::ffi::c_uint);
    block[(0 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(0
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    e = e
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(block[(0 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x6ed9eba1 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    a << 5 as ::core::ffi::c_int
                        | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    b = b << 30 as ::core::ffi::c_int
        | b >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(1 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(1
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    d = d
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(block[(1 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x6ed9eba1 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    e << 5 as ::core::ffi::c_int
                        | e >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    a = a << 30 as ::core::ffi::c_int
        | a >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(2 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(2
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    c = c
        .wrapping_add(
            (e ^ a ^ b)
                .wrapping_add(block[(2 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x6ed9eba1 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    d << 5 as ::core::ffi::c_int
                        | d >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    e = e << 30 as ::core::ffi::c_int
        | e >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(3 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(3
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    b = b
        .wrapping_add(
            (d ^ e ^ a)
                .wrapping_add(block[(3 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x6ed9eba1 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    c << 5 as ::core::ffi::c_int
                        | c >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    d = d << 30 as ::core::ffi::c_int
        | d >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(4 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(4
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    a = a
        .wrapping_add(
            (c ^ d ^ e)
                .wrapping_add(block[(4 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x6ed9eba1 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    b << 5 as ::core::ffi::c_int
                        | b >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    c = c << 30 as ::core::ffi::c_int
        | c >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    i = i.wrapping_add(5 as ::core::ffi::c_uint);
    block[(0 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(0
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    e = e
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(block[(0 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x6ed9eba1 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    a << 5 as ::core::ffi::c_int
                        | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    b = b << 30 as ::core::ffi::c_int
        | b >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(1 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(1
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    d = d
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(block[(1 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x6ed9eba1 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    e << 5 as ::core::ffi::c_int
                        | e >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    a = a << 30 as ::core::ffi::c_int
        | a >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(2 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(2
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    c = c
        .wrapping_add(
            (e ^ a ^ b)
                .wrapping_add(block[(2 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x6ed9eba1 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    d << 5 as ::core::ffi::c_int
                        | d >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    e = e << 30 as ::core::ffi::c_int
        | e >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(3 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(3
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    b = b
        .wrapping_add(
            (d ^ e ^ a)
                .wrapping_add(block[(3 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x6ed9eba1 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    c << 5 as ::core::ffi::c_int
                        | c >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    d = d << 30 as ::core::ffi::c_int
        | d >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(4 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(4
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    a = a
        .wrapping_add(
            (c ^ d ^ e)
                .wrapping_add(block[(4 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x6ed9eba1 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    b << 5 as ::core::ffi::c_int
                        | b >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    c = c << 30 as ::core::ffi::c_int
        | c >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    i = i.wrapping_add(5 as ::core::ffi::c_uint);
    block[(0 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(0
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    e = e
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(block[(0 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x6ed9eba1 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    a << 5 as ::core::ffi::c_int
                        | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    b = b << 30 as ::core::ffi::c_int
        | b >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(1 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(1
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    d = d
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(block[(1 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x6ed9eba1 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    e << 5 as ::core::ffi::c_int
                        | e >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    a = a << 30 as ::core::ffi::c_int
        | a >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(2 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(2
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    c = c
        .wrapping_add(
            (e ^ a ^ b)
                .wrapping_add(block[(2 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x6ed9eba1 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    d << 5 as ::core::ffi::c_int
                        | d >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    e = e << 30 as ::core::ffi::c_int
        | e >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(3 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(3
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    b = b
        .wrapping_add(
            (d ^ e ^ a)
                .wrapping_add(block[(3 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x6ed9eba1 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    c << 5 as ::core::ffi::c_int
                        | c >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    d = d << 30 as ::core::ffi::c_int
        | d >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(4 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(4
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    a = a
        .wrapping_add(
            (c ^ d ^ e)
                .wrapping_add(block[(4 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x6ed9eba1 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    b << 5 as ::core::ffi::c_int
                        | b >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    c = c << 30 as ::core::ffi::c_int
        | c >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    i = i.wrapping_add(5 as ::core::ffi::c_uint);
    block[(0 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(0
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    e = e
        .wrapping_add(
            ((b | c) & d | b & c)
                .wrapping_add(block[(0 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x8f1bbcdc as ::core::ffi::c_uint)
                .wrapping_add(
                    a << 5 as ::core::ffi::c_int
                        | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    b = b << 30 as ::core::ffi::c_int
        | b >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(1 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(1
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    d = d
        .wrapping_add(
            ((a | b) & c | a & b)
                .wrapping_add(block[(1 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x8f1bbcdc as ::core::ffi::c_uint)
                .wrapping_add(
                    e << 5 as ::core::ffi::c_int
                        | e >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    a = a << 30 as ::core::ffi::c_int
        | a >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(2 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(2
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    c = c
        .wrapping_add(
            ((e | a) & b | e & a)
                .wrapping_add(block[(2 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x8f1bbcdc as ::core::ffi::c_uint)
                .wrapping_add(
                    d << 5 as ::core::ffi::c_int
                        | d >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    e = e << 30 as ::core::ffi::c_int
        | e >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(3 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(3
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    b = b
        .wrapping_add(
            ((d | e) & a | d & e)
                .wrapping_add(block[(3 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x8f1bbcdc as ::core::ffi::c_uint)
                .wrapping_add(
                    c << 5 as ::core::ffi::c_int
                        | c >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    d = d << 30 as ::core::ffi::c_int
        | d >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(4 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(4
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    a = a
        .wrapping_add(
            ((c | d) & e | c & d)
                .wrapping_add(block[(4 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x8f1bbcdc as ::core::ffi::c_uint)
                .wrapping_add(
                    b << 5 as ::core::ffi::c_int
                        | b >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    c = c << 30 as ::core::ffi::c_int
        | c >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    i = i.wrapping_add(5 as ::core::ffi::c_uint);
    block[(0 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(0
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    e = e
        .wrapping_add(
            ((b | c) & d | b & c)
                .wrapping_add(block[(0 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x8f1bbcdc as ::core::ffi::c_uint)
                .wrapping_add(
                    a << 5 as ::core::ffi::c_int
                        | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    b = b << 30 as ::core::ffi::c_int
        | b >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(1 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(1
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    d = d
        .wrapping_add(
            ((a | b) & c | a & b)
                .wrapping_add(block[(1 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x8f1bbcdc as ::core::ffi::c_uint)
                .wrapping_add(
                    e << 5 as ::core::ffi::c_int
                        | e >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    a = a << 30 as ::core::ffi::c_int
        | a >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(2 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(2
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    c = c
        .wrapping_add(
            ((e | a) & b | e & a)
                .wrapping_add(block[(2 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x8f1bbcdc as ::core::ffi::c_uint)
                .wrapping_add(
                    d << 5 as ::core::ffi::c_int
                        | d >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    e = e << 30 as ::core::ffi::c_int
        | e >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(3 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(3
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    b = b
        .wrapping_add(
            ((d | e) & a | d & e)
                .wrapping_add(block[(3 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x8f1bbcdc as ::core::ffi::c_uint)
                .wrapping_add(
                    c << 5 as ::core::ffi::c_int
                        | c >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    d = d << 30 as ::core::ffi::c_int
        | d >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(4 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(4
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    a = a
        .wrapping_add(
            ((c | d) & e | c & d)
                .wrapping_add(block[(4 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x8f1bbcdc as ::core::ffi::c_uint)
                .wrapping_add(
                    b << 5 as ::core::ffi::c_int
                        | b >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    c = c << 30 as ::core::ffi::c_int
        | c >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    i = i.wrapping_add(5 as ::core::ffi::c_uint);
    block[(0 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(0
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    e = e
        .wrapping_add(
            ((b | c) & d | b & c)
                .wrapping_add(block[(0 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x8f1bbcdc as ::core::ffi::c_uint)
                .wrapping_add(
                    a << 5 as ::core::ffi::c_int
                        | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    b = b << 30 as ::core::ffi::c_int
        | b >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(1 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(1
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    d = d
        .wrapping_add(
            ((a | b) & c | a & b)
                .wrapping_add(block[(1 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x8f1bbcdc as ::core::ffi::c_uint)
                .wrapping_add(
                    e << 5 as ::core::ffi::c_int
                        | e >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    a = a << 30 as ::core::ffi::c_int
        | a >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(2 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(2
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    c = c
        .wrapping_add(
            ((e | a) & b | e & a)
                .wrapping_add(block[(2 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x8f1bbcdc as ::core::ffi::c_uint)
                .wrapping_add(
                    d << 5 as ::core::ffi::c_int
                        | d >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    e = e << 30 as ::core::ffi::c_int
        | e >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(3 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(3
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    b = b
        .wrapping_add(
            ((d | e) & a | d & e)
                .wrapping_add(block[(3 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x8f1bbcdc as ::core::ffi::c_uint)
                .wrapping_add(
                    c << 5 as ::core::ffi::c_int
                        | c >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    d = d << 30 as ::core::ffi::c_int
        | d >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(4 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(4
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    a = a
        .wrapping_add(
            ((c | d) & e | c & d)
                .wrapping_add(block[(4 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x8f1bbcdc as ::core::ffi::c_uint)
                .wrapping_add(
                    b << 5 as ::core::ffi::c_int
                        | b >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    c = c << 30 as ::core::ffi::c_int
        | c >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    i = i.wrapping_add(5 as ::core::ffi::c_uint);
    block[(0 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(0
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    e = e
        .wrapping_add(
            ((b | c) & d | b & c)
                .wrapping_add(block[(0 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x8f1bbcdc as ::core::ffi::c_uint)
                .wrapping_add(
                    a << 5 as ::core::ffi::c_int
                        | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    b = b << 30 as ::core::ffi::c_int
        | b >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(1 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(1
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    d = d
        .wrapping_add(
            ((a | b) & c | a & b)
                .wrapping_add(block[(1 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x8f1bbcdc as ::core::ffi::c_uint)
                .wrapping_add(
                    e << 5 as ::core::ffi::c_int
                        | e >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    a = a << 30 as ::core::ffi::c_int
        | a >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(2 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(2
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    c = c
        .wrapping_add(
            ((e | a) & b | e & a)
                .wrapping_add(block[(2 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x8f1bbcdc as ::core::ffi::c_uint)
                .wrapping_add(
                    d << 5 as ::core::ffi::c_int
                        | d >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    e = e << 30 as ::core::ffi::c_int
        | e >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(3 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(3
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    b = b
        .wrapping_add(
            ((d | e) & a | d & e)
                .wrapping_add(block[(3 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x8f1bbcdc as ::core::ffi::c_uint)
                .wrapping_add(
                    c << 5 as ::core::ffi::c_int
                        | c >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    d = d << 30 as ::core::ffi::c_int
        | d >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(4 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(4
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    a = a
        .wrapping_add(
            ((c | d) & e | c & d)
                .wrapping_add(block[(4 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0x8f1bbcdc as ::core::ffi::c_uint)
                .wrapping_add(
                    b << 5 as ::core::ffi::c_int
                        | b >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    c = c << 30 as ::core::ffi::c_int
        | c >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    i = i.wrapping_add(5 as ::core::ffi::c_uint);
    block[(0 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(0
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    e = e
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(block[(0 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0xca62c1d6 as ::core::ffi::c_uint)
                .wrapping_add(
                    a << 5 as ::core::ffi::c_int
                        | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    b = b << 30 as ::core::ffi::c_int
        | b >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(1 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(1
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    d = d
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(block[(1 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0xca62c1d6 as ::core::ffi::c_uint)
                .wrapping_add(
                    e << 5 as ::core::ffi::c_int
                        | e >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    a = a << 30 as ::core::ffi::c_int
        | a >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(2 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(2
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    c = c
        .wrapping_add(
            (e ^ a ^ b)
                .wrapping_add(block[(2 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0xca62c1d6 as ::core::ffi::c_uint)
                .wrapping_add(
                    d << 5 as ::core::ffi::c_int
                        | d >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    e = e << 30 as ::core::ffi::c_int
        | e >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(3 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(3
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    b = b
        .wrapping_add(
            (d ^ e ^ a)
                .wrapping_add(block[(3 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0xca62c1d6 as ::core::ffi::c_uint)
                .wrapping_add(
                    c << 5 as ::core::ffi::c_int
                        | c >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    d = d << 30 as ::core::ffi::c_int
        | d >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(4 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(4
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    a = a
        .wrapping_add(
            (c ^ d ^ e)
                .wrapping_add(block[(4 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0xca62c1d6 as ::core::ffi::c_uint)
                .wrapping_add(
                    b << 5 as ::core::ffi::c_int
                        | b >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    c = c << 30 as ::core::ffi::c_int
        | c >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    i = i.wrapping_add(5 as ::core::ffi::c_uint);
    block[(0 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(0
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    e = e
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(block[(0 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0xca62c1d6 as ::core::ffi::c_uint)
                .wrapping_add(
                    a << 5 as ::core::ffi::c_int
                        | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    b = b << 30 as ::core::ffi::c_int
        | b >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(1 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(1
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    d = d
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(block[(1 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0xca62c1d6 as ::core::ffi::c_uint)
                .wrapping_add(
                    e << 5 as ::core::ffi::c_int
                        | e >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    a = a << 30 as ::core::ffi::c_int
        | a >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(2 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(2
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    c = c
        .wrapping_add(
            (e ^ a ^ b)
                .wrapping_add(block[(2 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0xca62c1d6 as ::core::ffi::c_uint)
                .wrapping_add(
                    d << 5 as ::core::ffi::c_int
                        | d >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    e = e << 30 as ::core::ffi::c_int
        | e >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(3 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(3
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    b = b
        .wrapping_add(
            (d ^ e ^ a)
                .wrapping_add(block[(3 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0xca62c1d6 as ::core::ffi::c_uint)
                .wrapping_add(
                    c << 5 as ::core::ffi::c_int
                        | c >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    d = d << 30 as ::core::ffi::c_int
        | d >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(4 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(4
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    a = a
        .wrapping_add(
            (c ^ d ^ e)
                .wrapping_add(block[(4 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0xca62c1d6 as ::core::ffi::c_uint)
                .wrapping_add(
                    b << 5 as ::core::ffi::c_int
                        | b >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    c = c << 30 as ::core::ffi::c_int
        | c >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    i = i.wrapping_add(5 as ::core::ffi::c_uint);
    block[(0 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(0
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    e = e
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(block[(0 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0xca62c1d6 as ::core::ffi::c_uint)
                .wrapping_add(
                    a << 5 as ::core::ffi::c_int
                        | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    b = b << 30 as ::core::ffi::c_int
        | b >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(1 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(1
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    d = d
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(block[(1 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0xca62c1d6 as ::core::ffi::c_uint)
                .wrapping_add(
                    e << 5 as ::core::ffi::c_int
                        | e >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    a = a << 30 as ::core::ffi::c_int
        | a >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(2 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(2
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    c = c
        .wrapping_add(
            (e ^ a ^ b)
                .wrapping_add(block[(2 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0xca62c1d6 as ::core::ffi::c_uint)
                .wrapping_add(
                    d << 5 as ::core::ffi::c_int
                        | d >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    e = e << 30 as ::core::ffi::c_int
        | e >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(3 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(3
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    b = b
        .wrapping_add(
            (d ^ e ^ a)
                .wrapping_add(block[(3 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0xca62c1d6 as ::core::ffi::c_uint)
                .wrapping_add(
                    c << 5 as ::core::ffi::c_int
                        | c >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    d = d << 30 as ::core::ffi::c_int
        | d >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(4 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(4
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    a = a
        .wrapping_add(
            (c ^ d ^ e)
                .wrapping_add(block[(4 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0xca62c1d6 as ::core::ffi::c_uint)
                .wrapping_add(
                    b << 5 as ::core::ffi::c_int
                        | b >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    c = c << 30 as ::core::ffi::c_int
        | c >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    i = i.wrapping_add(5 as ::core::ffi::c_uint);
    block[(0 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(0
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(0 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(0 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    e = e
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(block[(0 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0xca62c1d6 as ::core::ffi::c_uint)
                .wrapping_add(
                    a << 5 as ::core::ffi::c_int
                        | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    b = b << 30 as ::core::ffi::c_int
        | b >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(1 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(1
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(1 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(1 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    d = d
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(block[(1 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0xca62c1d6 as ::core::ffi::c_uint)
                .wrapping_add(
                    e << 5 as ::core::ffi::c_int
                        | e >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    a = a << 30 as ::core::ffi::c_int
        | a >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(2 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(2
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(2 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(2 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    c = c
        .wrapping_add(
            (e ^ a ^ b)
                .wrapping_add(block[(2 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0xca62c1d6 as ::core::ffi::c_uint)
                .wrapping_add(
                    d << 5 as ::core::ffi::c_int
                        | d >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    e = e << 30 as ::core::ffi::c_int
        | e >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(3 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(3
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(3 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(3 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    b = b
        .wrapping_add(
            (d ^ e ^ a)
                .wrapping_add(block[(3 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0xca62c1d6 as ::core::ffi::c_uint)
                .wrapping_add(
                    c << 5 as ::core::ffi::c_int
                        | c >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    d = d << 30 as ::core::ffi::c_int
        | d >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    block[(4 as ::core::ffi::c_uint).wrapping_add(i) as usize] = (block[(4
        as ::core::ffi::c_uint)
        .wrapping_add(i)
        .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
        ^ block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
        << 1 as ::core::ffi::c_int
        | (block[(4 as ::core::ffi::c_uint)
            .wrapping_add(i)
            .wrapping_sub(3 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(8 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(14 as ::core::ffi::c_uint) as usize]
            ^ block[(4 as ::core::ffi::c_uint)
                .wrapping_add(i)
                .wrapping_sub(16 as ::core::ffi::c_uint) as usize])
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    a = a
        .wrapping_add(
            (c ^ d ^ e)
                .wrapping_add(block[(4 as ::core::ffi::c_uint).wrapping_add(i) as usize])
                .wrapping_add(0xca62c1d6 as ::core::ffi::c_uint)
                .wrapping_add(
                    b << 5 as ::core::ffi::c_int
                        | b >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                ),
        );
    c = c << 30 as ::core::ffi::c_int
        | c >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    i = i.wrapping_add(5 as ::core::ffi::c_uint);
    let ref mut fresh8 = *state.offset(0 as ::core::ffi::c_int as isize);
    *fresh8 = (*fresh8 as ::core::ffi::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    let ref mut fresh9 = *state.offset(1 as ::core::ffi::c_int as isize);
    *fresh9 = (*fresh9 as ::core::ffi::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    let ref mut fresh10 = *state.offset(2 as ::core::ffi::c_int as isize);
    *fresh10 = (*fresh10 as ::core::ffi::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    let ref mut fresh11 = *state.offset(3 as ::core::ffi::c_int as isize);
    *fresh11 = (*fresh11 as ::core::ffi::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    let ref mut fresh12 = *state.offset(4 as ::core::ffi::c_int as isize);
    *fresh12 = (*fresh12 as ::core::ffi::c_uint).wrapping_add(e) as uint32_t as uint32_t;
}
#[inline]
unsafe extern "C" fn av_bswap64(mut x: uint64_t) -> uint64_t {
    return (av_bswap32(x as uint32_t) as uint64_t) << 32 as ::core::ffi::c_int
        | av_bswap32((x >> 32 as ::core::ffi::c_int) as uint32_t) as uint64_t;
}
static mut K256: [uint32_t; 64] = [
    0x428a2f98 as ::core::ffi::c_int as uint32_t,
    0x71374491 as ::core::ffi::c_int as uint32_t,
    0xb5c0fbcf as ::core::ffi::c_uint,
    0xe9b5dba5 as ::core::ffi::c_uint,
    0x3956c25b as ::core::ffi::c_int as uint32_t,
    0x59f111f1 as ::core::ffi::c_int as uint32_t,
    0x923f82a4 as ::core::ffi::c_uint,
    0xab1c5ed5 as ::core::ffi::c_uint,
    0xd807aa98 as ::core::ffi::c_uint,
    0x12835b01 as ::core::ffi::c_int as uint32_t,
    0x243185be as ::core::ffi::c_int as uint32_t,
    0x550c7dc3 as ::core::ffi::c_int as uint32_t,
    0x72be5d74 as ::core::ffi::c_int as uint32_t,
    0x80deb1fe as ::core::ffi::c_uint,
    0x9bdc06a7 as ::core::ffi::c_uint,
    0xc19bf174 as ::core::ffi::c_uint,
    0xe49b69c1 as ::core::ffi::c_uint,
    0xefbe4786 as ::core::ffi::c_uint,
    0xfc19dc6 as ::core::ffi::c_int as uint32_t,
    0x240ca1cc as ::core::ffi::c_int as uint32_t,
    0x2de92c6f as ::core::ffi::c_int as uint32_t,
    0x4a7484aa as ::core::ffi::c_int as uint32_t,
    0x5cb0a9dc as ::core::ffi::c_int as uint32_t,
    0x76f988da as ::core::ffi::c_int as uint32_t,
    0x983e5152 as ::core::ffi::c_uint,
    0xa831c66d as ::core::ffi::c_uint,
    0xb00327c8 as ::core::ffi::c_uint,
    0xbf597fc7 as ::core::ffi::c_uint,
    0xc6e00bf3 as ::core::ffi::c_uint,
    0xd5a79147 as ::core::ffi::c_uint,
    0x6ca6351 as ::core::ffi::c_int as uint32_t,
    0x14292967 as ::core::ffi::c_int as uint32_t,
    0x27b70a85 as ::core::ffi::c_int as uint32_t,
    0x2e1b2138 as ::core::ffi::c_int as uint32_t,
    0x4d2c6dfc as ::core::ffi::c_int as uint32_t,
    0x53380d13 as ::core::ffi::c_int as uint32_t,
    0x650a7354 as ::core::ffi::c_int as uint32_t,
    0x766a0abb as ::core::ffi::c_int as uint32_t,
    0x81c2c92e as ::core::ffi::c_uint,
    0x92722c85 as ::core::ffi::c_uint,
    0xa2bfe8a1 as ::core::ffi::c_uint,
    0xa81a664b as ::core::ffi::c_uint,
    0xc24b8b70 as ::core::ffi::c_uint,
    0xc76c51a3 as ::core::ffi::c_uint,
    0xd192e819 as ::core::ffi::c_uint,
    0xd6990624 as ::core::ffi::c_uint,
    0xf40e3585 as ::core::ffi::c_uint,
    0x106aa070 as ::core::ffi::c_int as uint32_t,
    0x19a4c116 as ::core::ffi::c_int as uint32_t,
    0x1e376c08 as ::core::ffi::c_int as uint32_t,
    0x2748774c as ::core::ffi::c_int as uint32_t,
    0x34b0bcb5 as ::core::ffi::c_int as uint32_t,
    0x391c0cb3 as ::core::ffi::c_int as uint32_t,
    0x4ed8aa4a as ::core::ffi::c_int as uint32_t,
    0x5b9cca4f as ::core::ffi::c_int as uint32_t,
    0x682e6ff3 as ::core::ffi::c_int as uint32_t,
    0x748f82ee as ::core::ffi::c_int as uint32_t,
    0x78a5636f as ::core::ffi::c_int as uint32_t,
    0x84c87814 as ::core::ffi::c_uint,
    0x8cc70208 as ::core::ffi::c_uint,
    0x90befffa as ::core::ffi::c_uint,
    0xa4506ceb as ::core::ffi::c_uint,
    0xbef9a3f7 as ::core::ffi::c_uint,
    0xc67178f2 as ::core::ffi::c_uint,
];
unsafe extern "C" fn sha256_transform(
    mut state: *mut uint32_t,
    mut buffer: *const uint8_t,
) {
    let mut i: ::core::ffi::c_uint = 0;
    let mut a: ::core::ffi::c_uint = 0;
    let mut b: ::core::ffi::c_uint = 0;
    let mut c: ::core::ffi::c_uint = 0;
    let mut d: ::core::ffi::c_uint = 0;
    let mut e: ::core::ffi::c_uint = 0;
    let mut f: ::core::ffi::c_uint = 0;
    let mut g: ::core::ffi::c_uint = 0;
    let mut h: ::core::ffi::c_uint = 0;
    let mut block: [uint32_t; 64] = [0; 64];
    let mut T1: uint32_t = 0;
    a = *state.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    b = *state.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    c = *state.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    d = *state.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    e = *state.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    f = *state.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    g = *state.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    h = *state.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
    i = 0 as ::core::ffi::c_uint;
    block[i as usize] = av_bswap32(
        (*(buffer.offset((4 as ::core::ffi::c_uint).wrapping_mul(i) as isize)
            as *const unaligned_32))
            .l,
    );
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (h as uint32_t)
                .wrapping_add(
                    ((e as uint32_t) << 26 as ::core::ffi::c_int
                        | e as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((e as uint32_t) << 21 as ::core::ffi::c_int
                            | e as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((e as uint32_t) << 7 as ::core::ffi::c_int
                            | e as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    e as uint32_t & (f as uint32_t ^ g as uint32_t) ^ g as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    d = d.wrapping_add(T1 as ::core::ffi::c_uint);
    h = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (a << 30 as ::core::ffi::c_int
                | a >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (a << 19 as ::core::ffi::c_int
                    | a >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (a << 10 as ::core::ffi::c_int
                    | a >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((c | b) & a | c & b);
    i = i.wrapping_add(1);
    block[i as usize] = av_bswap32(
        (*(buffer.offset((4 as ::core::ffi::c_uint).wrapping_mul(i) as isize)
            as *const unaligned_32))
            .l,
    );
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (g as uint32_t)
                .wrapping_add(
                    ((d as uint32_t) << 26 as ::core::ffi::c_int
                        | d as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((d as uint32_t) << 21 as ::core::ffi::c_int
                            | d as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((d as uint32_t) << 7 as ::core::ffi::c_int
                            | d as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    d as uint32_t & (e as uint32_t ^ f as uint32_t) ^ f as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    c = c.wrapping_add(T1 as ::core::ffi::c_uint);
    g = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (h << 30 as ::core::ffi::c_int
                | h >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (h << 19 as ::core::ffi::c_int
                    | h >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (h << 10 as ::core::ffi::c_int
                    | h >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((b | a) & h | b & a);
    i = i.wrapping_add(1);
    block[i as usize] = av_bswap32(
        (*(buffer.offset((4 as ::core::ffi::c_uint).wrapping_mul(i) as isize)
            as *const unaligned_32))
            .l,
    );
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (f as uint32_t)
                .wrapping_add(
                    ((c as uint32_t) << 26 as ::core::ffi::c_int
                        | c as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((c as uint32_t) << 21 as ::core::ffi::c_int
                            | c as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((c as uint32_t) << 7 as ::core::ffi::c_int
                            | c as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    c as uint32_t & (d as uint32_t ^ e as uint32_t) ^ e as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    b = b.wrapping_add(T1 as ::core::ffi::c_uint);
    f = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (g << 30 as ::core::ffi::c_int
                | g >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (g << 19 as ::core::ffi::c_int
                    | g >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (g << 10 as ::core::ffi::c_int
                    | g >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((a | h) & g | a & h);
    i = i.wrapping_add(1);
    block[i as usize] = av_bswap32(
        (*(buffer.offset((4 as ::core::ffi::c_uint).wrapping_mul(i) as isize)
            as *const unaligned_32))
            .l,
    );
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (e as uint32_t)
                .wrapping_add(
                    ((b as uint32_t) << 26 as ::core::ffi::c_int
                        | b as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((b as uint32_t) << 21 as ::core::ffi::c_int
                            | b as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((b as uint32_t) << 7 as ::core::ffi::c_int
                            | b as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    b as uint32_t & (c as uint32_t ^ d as uint32_t) ^ d as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    a = a.wrapping_add(T1 as ::core::ffi::c_uint);
    e = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (f << 30 as ::core::ffi::c_int
                | f >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (f << 19 as ::core::ffi::c_int
                    | f >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (f << 10 as ::core::ffi::c_int
                    | f >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((h | g) & f | h & g);
    i = i.wrapping_add(1);
    block[i as usize] = av_bswap32(
        (*(buffer.offset((4 as ::core::ffi::c_uint).wrapping_mul(i) as isize)
            as *const unaligned_32))
            .l,
    );
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (d as uint32_t)
                .wrapping_add(
                    ((a as uint32_t) << 26 as ::core::ffi::c_int
                        | a as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((a as uint32_t) << 21 as ::core::ffi::c_int
                            | a as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((a as uint32_t) << 7 as ::core::ffi::c_int
                            | a as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    a as uint32_t & (b as uint32_t ^ c as uint32_t) ^ c as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    h = h.wrapping_add(T1 as ::core::ffi::c_uint);
    d = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (e << 30 as ::core::ffi::c_int
                | e >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (e << 19 as ::core::ffi::c_int
                    | e >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (e << 10 as ::core::ffi::c_int
                    | e >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((g | f) & e | g & f);
    i = i.wrapping_add(1);
    block[i as usize] = av_bswap32(
        (*(buffer.offset((4 as ::core::ffi::c_uint).wrapping_mul(i) as isize)
            as *const unaligned_32))
            .l,
    );
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (c as uint32_t)
                .wrapping_add(
                    ((h as uint32_t) << 26 as ::core::ffi::c_int
                        | h as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((h as uint32_t) << 21 as ::core::ffi::c_int
                            | h as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((h as uint32_t) << 7 as ::core::ffi::c_int
                            | h as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    h as uint32_t & (a as uint32_t ^ b as uint32_t) ^ b as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    g = g.wrapping_add(T1 as ::core::ffi::c_uint);
    c = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (d << 30 as ::core::ffi::c_int
                | d >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (d << 19 as ::core::ffi::c_int
                    | d >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (d << 10 as ::core::ffi::c_int
                    | d >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((f | e) & d | f & e);
    i = i.wrapping_add(1);
    block[i as usize] = av_bswap32(
        (*(buffer.offset((4 as ::core::ffi::c_uint).wrapping_mul(i) as isize)
            as *const unaligned_32))
            .l,
    );
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (b as uint32_t)
                .wrapping_add(
                    ((g as uint32_t) << 26 as ::core::ffi::c_int
                        | g as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((g as uint32_t) << 21 as ::core::ffi::c_int
                            | g as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((g as uint32_t) << 7 as ::core::ffi::c_int
                            | g as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    g as uint32_t & (h as uint32_t ^ a as uint32_t) ^ a as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    f = f.wrapping_add(T1 as ::core::ffi::c_uint);
    b = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (c << 30 as ::core::ffi::c_int
                | c >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (c << 19 as ::core::ffi::c_int
                    | c >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (c << 10 as ::core::ffi::c_int
                    | c >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((e | d) & c | e & d);
    i = i.wrapping_add(1);
    block[i as usize] = av_bswap32(
        (*(buffer.offset((4 as ::core::ffi::c_uint).wrapping_mul(i) as isize)
            as *const unaligned_32))
            .l,
    );
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (a as uint32_t)
                .wrapping_add(
                    ((f as uint32_t) << 26 as ::core::ffi::c_int
                        | f as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((f as uint32_t) << 21 as ::core::ffi::c_int
                            | f as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((f as uint32_t) << 7 as ::core::ffi::c_int
                            | f as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    f as uint32_t & (g as uint32_t ^ h as uint32_t) ^ h as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    e = e.wrapping_add(T1 as ::core::ffi::c_uint);
    a = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (b << 30 as ::core::ffi::c_int
                | b >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (b << 19 as ::core::ffi::c_int
                    | b >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (b << 10 as ::core::ffi::c_int
                    | b >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((d | c) & b | d & c);
    i = i.wrapping_add(1);
    block[i as usize] = av_bswap32(
        (*(buffer.offset((4 as ::core::ffi::c_uint).wrapping_mul(i) as isize)
            as *const unaligned_32))
            .l,
    );
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (h as uint32_t)
                .wrapping_add(
                    ((e as uint32_t) << 26 as ::core::ffi::c_int
                        | e as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((e as uint32_t) << 21 as ::core::ffi::c_int
                            | e as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((e as uint32_t) << 7 as ::core::ffi::c_int
                            | e as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    e as uint32_t & (f as uint32_t ^ g as uint32_t) ^ g as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    d = d.wrapping_add(T1 as ::core::ffi::c_uint);
    h = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (a << 30 as ::core::ffi::c_int
                | a >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (a << 19 as ::core::ffi::c_int
                    | a >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (a << 10 as ::core::ffi::c_int
                    | a >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((c | b) & a | c & b);
    i = i.wrapping_add(1);
    block[i as usize] = av_bswap32(
        (*(buffer.offset((4 as ::core::ffi::c_uint).wrapping_mul(i) as isize)
            as *const unaligned_32))
            .l,
    );
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (g as uint32_t)
                .wrapping_add(
                    ((d as uint32_t) << 26 as ::core::ffi::c_int
                        | d as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((d as uint32_t) << 21 as ::core::ffi::c_int
                            | d as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((d as uint32_t) << 7 as ::core::ffi::c_int
                            | d as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    d as uint32_t & (e as uint32_t ^ f as uint32_t) ^ f as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    c = c.wrapping_add(T1 as ::core::ffi::c_uint);
    g = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (h << 30 as ::core::ffi::c_int
                | h >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (h << 19 as ::core::ffi::c_int
                    | h >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (h << 10 as ::core::ffi::c_int
                    | h >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((b | a) & h | b & a);
    i = i.wrapping_add(1);
    block[i as usize] = av_bswap32(
        (*(buffer.offset((4 as ::core::ffi::c_uint).wrapping_mul(i) as isize)
            as *const unaligned_32))
            .l,
    );
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (f as uint32_t)
                .wrapping_add(
                    ((c as uint32_t) << 26 as ::core::ffi::c_int
                        | c as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((c as uint32_t) << 21 as ::core::ffi::c_int
                            | c as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((c as uint32_t) << 7 as ::core::ffi::c_int
                            | c as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    c as uint32_t & (d as uint32_t ^ e as uint32_t) ^ e as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    b = b.wrapping_add(T1 as ::core::ffi::c_uint);
    f = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (g << 30 as ::core::ffi::c_int
                | g >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (g << 19 as ::core::ffi::c_int
                    | g >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (g << 10 as ::core::ffi::c_int
                    | g >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((a | h) & g | a & h);
    i = i.wrapping_add(1);
    block[i as usize] = av_bswap32(
        (*(buffer.offset((4 as ::core::ffi::c_uint).wrapping_mul(i) as isize)
            as *const unaligned_32))
            .l,
    );
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (e as uint32_t)
                .wrapping_add(
                    ((b as uint32_t) << 26 as ::core::ffi::c_int
                        | b as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((b as uint32_t) << 21 as ::core::ffi::c_int
                            | b as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((b as uint32_t) << 7 as ::core::ffi::c_int
                            | b as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    b as uint32_t & (c as uint32_t ^ d as uint32_t) ^ d as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    a = a.wrapping_add(T1 as ::core::ffi::c_uint);
    e = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (f << 30 as ::core::ffi::c_int
                | f >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (f << 19 as ::core::ffi::c_int
                    | f >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (f << 10 as ::core::ffi::c_int
                    | f >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((h | g) & f | h & g);
    i = i.wrapping_add(1);
    block[i as usize] = av_bswap32(
        (*(buffer.offset((4 as ::core::ffi::c_uint).wrapping_mul(i) as isize)
            as *const unaligned_32))
            .l,
    );
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (d as uint32_t)
                .wrapping_add(
                    ((a as uint32_t) << 26 as ::core::ffi::c_int
                        | a as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((a as uint32_t) << 21 as ::core::ffi::c_int
                            | a as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((a as uint32_t) << 7 as ::core::ffi::c_int
                            | a as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    a as uint32_t & (b as uint32_t ^ c as uint32_t) ^ c as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    h = h.wrapping_add(T1 as ::core::ffi::c_uint);
    d = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (e << 30 as ::core::ffi::c_int
                | e >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (e << 19 as ::core::ffi::c_int
                    | e >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (e << 10 as ::core::ffi::c_int
                    | e >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((g | f) & e | g & f);
    i = i.wrapping_add(1);
    block[i as usize] = av_bswap32(
        (*(buffer.offset((4 as ::core::ffi::c_uint).wrapping_mul(i) as isize)
            as *const unaligned_32))
            .l,
    );
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (c as uint32_t)
                .wrapping_add(
                    ((h as uint32_t) << 26 as ::core::ffi::c_int
                        | h as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((h as uint32_t) << 21 as ::core::ffi::c_int
                            | h as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((h as uint32_t) << 7 as ::core::ffi::c_int
                            | h as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    h as uint32_t & (a as uint32_t ^ b as uint32_t) ^ b as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    g = g.wrapping_add(T1 as ::core::ffi::c_uint);
    c = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (d << 30 as ::core::ffi::c_int
                | d >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (d << 19 as ::core::ffi::c_int
                    | d >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (d << 10 as ::core::ffi::c_int
                    | d >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((f | e) & d | f & e);
    i = i.wrapping_add(1);
    block[i as usize] = av_bswap32(
        (*(buffer.offset((4 as ::core::ffi::c_uint).wrapping_mul(i) as isize)
            as *const unaligned_32))
            .l,
    );
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (b as uint32_t)
                .wrapping_add(
                    ((g as uint32_t) << 26 as ::core::ffi::c_int
                        | g as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((g as uint32_t) << 21 as ::core::ffi::c_int
                            | g as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((g as uint32_t) << 7 as ::core::ffi::c_int
                            | g as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    g as uint32_t & (h as uint32_t ^ a as uint32_t) ^ a as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    f = f.wrapping_add(T1 as ::core::ffi::c_uint);
    b = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (c << 30 as ::core::ffi::c_int
                | c >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (c << 19 as ::core::ffi::c_int
                    | c >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (c << 10 as ::core::ffi::c_int
                    | c >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((e | d) & c | e & d);
    i = i.wrapping_add(1);
    block[i as usize] = av_bswap32(
        (*(buffer.offset((4 as ::core::ffi::c_uint).wrapping_mul(i) as isize)
            as *const unaligned_32))
            .l,
    );
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (a as uint32_t)
                .wrapping_add(
                    ((f as uint32_t) << 26 as ::core::ffi::c_int
                        | f as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((f as uint32_t) << 21 as ::core::ffi::c_int
                            | f as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((f as uint32_t) << 7 as ::core::ffi::c_int
                            | f as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    f as uint32_t & (g as uint32_t ^ h as uint32_t) ^ h as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    e = e.wrapping_add(T1 as ::core::ffi::c_uint);
    a = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (b << 30 as ::core::ffi::c_int
                | b >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (b << 19 as ::core::ffi::c_int
                    | b >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (b << 10 as ::core::ffi::c_int
                    | b >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((d | c) & b | d & c);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (h as uint32_t)
                .wrapping_add(
                    ((e as uint32_t) << 26 as ::core::ffi::c_int
                        | e as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((e as uint32_t) << 21 as ::core::ffi::c_int
                            | e as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((e as uint32_t) << 7 as ::core::ffi::c_int
                            | e as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    e as uint32_t & (f as uint32_t ^ g as uint32_t) ^ g as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    d = d.wrapping_add(T1 as ::core::ffi::c_uint);
    h = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (a << 30 as ::core::ffi::c_int
                | a >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (a << 19 as ::core::ffi::c_int
                    | a >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (a << 10 as ::core::ffi::c_int
                    | a >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((c | b) & a | c & b);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (g as uint32_t)
                .wrapping_add(
                    ((d as uint32_t) << 26 as ::core::ffi::c_int
                        | d as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((d as uint32_t) << 21 as ::core::ffi::c_int
                            | d as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((d as uint32_t) << 7 as ::core::ffi::c_int
                            | d as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    d as uint32_t & (e as uint32_t ^ f as uint32_t) ^ f as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    c = c.wrapping_add(T1 as ::core::ffi::c_uint);
    g = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (h << 30 as ::core::ffi::c_int
                | h >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (h << 19 as ::core::ffi::c_int
                    | h >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (h << 10 as ::core::ffi::c_int
                    | h >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((b | a) & h | b & a);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (f as uint32_t)
                .wrapping_add(
                    ((c as uint32_t) << 26 as ::core::ffi::c_int
                        | c as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((c as uint32_t) << 21 as ::core::ffi::c_int
                            | c as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((c as uint32_t) << 7 as ::core::ffi::c_int
                            | c as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    c as uint32_t & (d as uint32_t ^ e as uint32_t) ^ e as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    b = b.wrapping_add(T1 as ::core::ffi::c_uint);
    f = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (g << 30 as ::core::ffi::c_int
                | g >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (g << 19 as ::core::ffi::c_int
                    | g >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (g << 10 as ::core::ffi::c_int
                    | g >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((a | h) & g | a & h);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (e as uint32_t)
                .wrapping_add(
                    ((b as uint32_t) << 26 as ::core::ffi::c_int
                        | b as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((b as uint32_t) << 21 as ::core::ffi::c_int
                            | b as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((b as uint32_t) << 7 as ::core::ffi::c_int
                            | b as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    b as uint32_t & (c as uint32_t ^ d as uint32_t) ^ d as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    a = a.wrapping_add(T1 as ::core::ffi::c_uint);
    e = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (f << 30 as ::core::ffi::c_int
                | f >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (f << 19 as ::core::ffi::c_int
                    | f >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (f << 10 as ::core::ffi::c_int
                    | f >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((h | g) & f | h & g);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (d as uint32_t)
                .wrapping_add(
                    ((a as uint32_t) << 26 as ::core::ffi::c_int
                        | a as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((a as uint32_t) << 21 as ::core::ffi::c_int
                            | a as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((a as uint32_t) << 7 as ::core::ffi::c_int
                            | a as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    a as uint32_t & (b as uint32_t ^ c as uint32_t) ^ c as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    h = h.wrapping_add(T1 as ::core::ffi::c_uint);
    d = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (e << 30 as ::core::ffi::c_int
                | e >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (e << 19 as ::core::ffi::c_int
                    | e >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (e << 10 as ::core::ffi::c_int
                    | e >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((g | f) & e | g & f);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (c as uint32_t)
                .wrapping_add(
                    ((h as uint32_t) << 26 as ::core::ffi::c_int
                        | h as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((h as uint32_t) << 21 as ::core::ffi::c_int
                            | h as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((h as uint32_t) << 7 as ::core::ffi::c_int
                            | h as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    h as uint32_t & (a as uint32_t ^ b as uint32_t) ^ b as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    g = g.wrapping_add(T1 as ::core::ffi::c_uint);
    c = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (d << 30 as ::core::ffi::c_int
                | d >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (d << 19 as ::core::ffi::c_int
                    | d >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (d << 10 as ::core::ffi::c_int
                    | d >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((f | e) & d | f & e);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (b as uint32_t)
                .wrapping_add(
                    ((g as uint32_t) << 26 as ::core::ffi::c_int
                        | g as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((g as uint32_t) << 21 as ::core::ffi::c_int
                            | g as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((g as uint32_t) << 7 as ::core::ffi::c_int
                            | g as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    g as uint32_t & (h as uint32_t ^ a as uint32_t) ^ a as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    f = f.wrapping_add(T1 as ::core::ffi::c_uint);
    b = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (c << 30 as ::core::ffi::c_int
                | c >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (c << 19 as ::core::ffi::c_int
                    | c >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (c << 10 as ::core::ffi::c_int
                    | c >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((e | d) & c | e & d);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (a as uint32_t)
                .wrapping_add(
                    ((f as uint32_t) << 26 as ::core::ffi::c_int
                        | f as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((f as uint32_t) << 21 as ::core::ffi::c_int
                            | f as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((f as uint32_t) << 7 as ::core::ffi::c_int
                            | f as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    f as uint32_t & (g as uint32_t ^ h as uint32_t) ^ h as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    e = e.wrapping_add(T1 as ::core::ffi::c_uint);
    a = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (b << 30 as ::core::ffi::c_int
                | b >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (b << 19 as ::core::ffi::c_int
                    | b >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (b << 10 as ::core::ffi::c_int
                    | b >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((d | c) & b | d & c);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (h as uint32_t)
                .wrapping_add(
                    ((e as uint32_t) << 26 as ::core::ffi::c_int
                        | e as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((e as uint32_t) << 21 as ::core::ffi::c_int
                            | e as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((e as uint32_t) << 7 as ::core::ffi::c_int
                            | e as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    e as uint32_t & (f as uint32_t ^ g as uint32_t) ^ g as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    d = d.wrapping_add(T1 as ::core::ffi::c_uint);
    h = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (a << 30 as ::core::ffi::c_int
                | a >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (a << 19 as ::core::ffi::c_int
                    | a >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (a << 10 as ::core::ffi::c_int
                    | a >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((c | b) & a | c & b);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (g as uint32_t)
                .wrapping_add(
                    ((d as uint32_t) << 26 as ::core::ffi::c_int
                        | d as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((d as uint32_t) << 21 as ::core::ffi::c_int
                            | d as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((d as uint32_t) << 7 as ::core::ffi::c_int
                            | d as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    d as uint32_t & (e as uint32_t ^ f as uint32_t) ^ f as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    c = c.wrapping_add(T1 as ::core::ffi::c_uint);
    g = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (h << 30 as ::core::ffi::c_int
                | h >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (h << 19 as ::core::ffi::c_int
                    | h >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (h << 10 as ::core::ffi::c_int
                    | h >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((b | a) & h | b & a);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (f as uint32_t)
                .wrapping_add(
                    ((c as uint32_t) << 26 as ::core::ffi::c_int
                        | c as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((c as uint32_t) << 21 as ::core::ffi::c_int
                            | c as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((c as uint32_t) << 7 as ::core::ffi::c_int
                            | c as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    c as uint32_t & (d as uint32_t ^ e as uint32_t) ^ e as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    b = b.wrapping_add(T1 as ::core::ffi::c_uint);
    f = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (g << 30 as ::core::ffi::c_int
                | g >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (g << 19 as ::core::ffi::c_int
                    | g >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (g << 10 as ::core::ffi::c_int
                    | g >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((a | h) & g | a & h);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (e as uint32_t)
                .wrapping_add(
                    ((b as uint32_t) << 26 as ::core::ffi::c_int
                        | b as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((b as uint32_t) << 21 as ::core::ffi::c_int
                            | b as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((b as uint32_t) << 7 as ::core::ffi::c_int
                            | b as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    b as uint32_t & (c as uint32_t ^ d as uint32_t) ^ d as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    a = a.wrapping_add(T1 as ::core::ffi::c_uint);
    e = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (f << 30 as ::core::ffi::c_int
                | f >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (f << 19 as ::core::ffi::c_int
                    | f >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (f << 10 as ::core::ffi::c_int
                    | f >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((h | g) & f | h & g);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (d as uint32_t)
                .wrapping_add(
                    ((a as uint32_t) << 26 as ::core::ffi::c_int
                        | a as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((a as uint32_t) << 21 as ::core::ffi::c_int
                            | a as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((a as uint32_t) << 7 as ::core::ffi::c_int
                            | a as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    a as uint32_t & (b as uint32_t ^ c as uint32_t) ^ c as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    h = h.wrapping_add(T1 as ::core::ffi::c_uint);
    d = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (e << 30 as ::core::ffi::c_int
                | e >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (e << 19 as ::core::ffi::c_int
                    | e >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (e << 10 as ::core::ffi::c_int
                    | e >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((g | f) & e | g & f);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (c as uint32_t)
                .wrapping_add(
                    ((h as uint32_t) << 26 as ::core::ffi::c_int
                        | h as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((h as uint32_t) << 21 as ::core::ffi::c_int
                            | h as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((h as uint32_t) << 7 as ::core::ffi::c_int
                            | h as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    h as uint32_t & (a as uint32_t ^ b as uint32_t) ^ b as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    g = g.wrapping_add(T1 as ::core::ffi::c_uint);
    c = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (d << 30 as ::core::ffi::c_int
                | d >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (d << 19 as ::core::ffi::c_int
                    | d >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (d << 10 as ::core::ffi::c_int
                    | d >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((f | e) & d | f & e);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (b as uint32_t)
                .wrapping_add(
                    ((g as uint32_t) << 26 as ::core::ffi::c_int
                        | g as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((g as uint32_t) << 21 as ::core::ffi::c_int
                            | g as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((g as uint32_t) << 7 as ::core::ffi::c_int
                            | g as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    g as uint32_t & (h as uint32_t ^ a as uint32_t) ^ a as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    f = f.wrapping_add(T1 as ::core::ffi::c_uint);
    b = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (c << 30 as ::core::ffi::c_int
                | c >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (c << 19 as ::core::ffi::c_int
                    | c >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (c << 10 as ::core::ffi::c_int
                    | c >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((e | d) & c | e & d);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (a as uint32_t)
                .wrapping_add(
                    ((f as uint32_t) << 26 as ::core::ffi::c_int
                        | f as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((f as uint32_t) << 21 as ::core::ffi::c_int
                            | f as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((f as uint32_t) << 7 as ::core::ffi::c_int
                            | f as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    f as uint32_t & (g as uint32_t ^ h as uint32_t) ^ h as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    e = e.wrapping_add(T1 as ::core::ffi::c_uint);
    a = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (b << 30 as ::core::ffi::c_int
                | b >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (b << 19 as ::core::ffi::c_int
                    | b >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (b << 10 as ::core::ffi::c_int
                    | b >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((d | c) & b | d & c);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (h as uint32_t)
                .wrapping_add(
                    ((e as uint32_t) << 26 as ::core::ffi::c_int
                        | e as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((e as uint32_t) << 21 as ::core::ffi::c_int
                            | e as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((e as uint32_t) << 7 as ::core::ffi::c_int
                            | e as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    e as uint32_t & (f as uint32_t ^ g as uint32_t) ^ g as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    d = d.wrapping_add(T1 as ::core::ffi::c_uint);
    h = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (a << 30 as ::core::ffi::c_int
                | a >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (a << 19 as ::core::ffi::c_int
                    | a >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (a << 10 as ::core::ffi::c_int
                    | a >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((c | b) & a | c & b);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (g as uint32_t)
                .wrapping_add(
                    ((d as uint32_t) << 26 as ::core::ffi::c_int
                        | d as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((d as uint32_t) << 21 as ::core::ffi::c_int
                            | d as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((d as uint32_t) << 7 as ::core::ffi::c_int
                            | d as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    d as uint32_t & (e as uint32_t ^ f as uint32_t) ^ f as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    c = c.wrapping_add(T1 as ::core::ffi::c_uint);
    g = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (h << 30 as ::core::ffi::c_int
                | h >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (h << 19 as ::core::ffi::c_int
                    | h >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (h << 10 as ::core::ffi::c_int
                    | h >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((b | a) & h | b & a);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (f as uint32_t)
                .wrapping_add(
                    ((c as uint32_t) << 26 as ::core::ffi::c_int
                        | c as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((c as uint32_t) << 21 as ::core::ffi::c_int
                            | c as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((c as uint32_t) << 7 as ::core::ffi::c_int
                            | c as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    c as uint32_t & (d as uint32_t ^ e as uint32_t) ^ e as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    b = b.wrapping_add(T1 as ::core::ffi::c_uint);
    f = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (g << 30 as ::core::ffi::c_int
                | g >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (g << 19 as ::core::ffi::c_int
                    | g >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (g << 10 as ::core::ffi::c_int
                    | g >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((a | h) & g | a & h);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (e as uint32_t)
                .wrapping_add(
                    ((b as uint32_t) << 26 as ::core::ffi::c_int
                        | b as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((b as uint32_t) << 21 as ::core::ffi::c_int
                            | b as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((b as uint32_t) << 7 as ::core::ffi::c_int
                            | b as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    b as uint32_t & (c as uint32_t ^ d as uint32_t) ^ d as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    a = a.wrapping_add(T1 as ::core::ffi::c_uint);
    e = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (f << 30 as ::core::ffi::c_int
                | f >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (f << 19 as ::core::ffi::c_int
                    | f >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (f << 10 as ::core::ffi::c_int
                    | f >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((h | g) & f | h & g);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (d as uint32_t)
                .wrapping_add(
                    ((a as uint32_t) << 26 as ::core::ffi::c_int
                        | a as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((a as uint32_t) << 21 as ::core::ffi::c_int
                            | a as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((a as uint32_t) << 7 as ::core::ffi::c_int
                            | a as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    a as uint32_t & (b as uint32_t ^ c as uint32_t) ^ c as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    h = h.wrapping_add(T1 as ::core::ffi::c_uint);
    d = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (e << 30 as ::core::ffi::c_int
                | e >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (e << 19 as ::core::ffi::c_int
                    | e >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (e << 10 as ::core::ffi::c_int
                    | e >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((g | f) & e | g & f);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (c as uint32_t)
                .wrapping_add(
                    ((h as uint32_t) << 26 as ::core::ffi::c_int
                        | h as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((h as uint32_t) << 21 as ::core::ffi::c_int
                            | h as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((h as uint32_t) << 7 as ::core::ffi::c_int
                            | h as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    h as uint32_t & (a as uint32_t ^ b as uint32_t) ^ b as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    g = g.wrapping_add(T1 as ::core::ffi::c_uint);
    c = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (d << 30 as ::core::ffi::c_int
                | d >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (d << 19 as ::core::ffi::c_int
                    | d >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (d << 10 as ::core::ffi::c_int
                    | d >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((f | e) & d | f & e);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (b as uint32_t)
                .wrapping_add(
                    ((g as uint32_t) << 26 as ::core::ffi::c_int
                        | g as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((g as uint32_t) << 21 as ::core::ffi::c_int
                            | g as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((g as uint32_t) << 7 as ::core::ffi::c_int
                            | g as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    g as uint32_t & (h as uint32_t ^ a as uint32_t) ^ a as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    f = f.wrapping_add(T1 as ::core::ffi::c_uint);
    b = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (c << 30 as ::core::ffi::c_int
                | c >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (c << 19 as ::core::ffi::c_int
                    | c >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (c << 10 as ::core::ffi::c_int
                    | c >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((e | d) & c | e & d);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (a as uint32_t)
                .wrapping_add(
                    ((f as uint32_t) << 26 as ::core::ffi::c_int
                        | f as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((f as uint32_t) << 21 as ::core::ffi::c_int
                            | f as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((f as uint32_t) << 7 as ::core::ffi::c_int
                            | f as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    f as uint32_t & (g as uint32_t ^ h as uint32_t) ^ h as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    e = e.wrapping_add(T1 as ::core::ffi::c_uint);
    a = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (b << 30 as ::core::ffi::c_int
                | b >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (b << 19 as ::core::ffi::c_int
                    | b >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (b << 10 as ::core::ffi::c_int
                    | b >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((d | c) & b | d & c);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (h as uint32_t)
                .wrapping_add(
                    ((e as uint32_t) << 26 as ::core::ffi::c_int
                        | e as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((e as uint32_t) << 21 as ::core::ffi::c_int
                            | e as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((e as uint32_t) << 7 as ::core::ffi::c_int
                            | e as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    e as uint32_t & (f as uint32_t ^ g as uint32_t) ^ g as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    d = d.wrapping_add(T1 as ::core::ffi::c_uint);
    h = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (a << 30 as ::core::ffi::c_int
                | a >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (a << 19 as ::core::ffi::c_int
                    | a >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (a << 10 as ::core::ffi::c_int
                    | a >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((c | b) & a | c & b);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (g as uint32_t)
                .wrapping_add(
                    ((d as uint32_t) << 26 as ::core::ffi::c_int
                        | d as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((d as uint32_t) << 21 as ::core::ffi::c_int
                            | d as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((d as uint32_t) << 7 as ::core::ffi::c_int
                            | d as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    d as uint32_t & (e as uint32_t ^ f as uint32_t) ^ f as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    c = c.wrapping_add(T1 as ::core::ffi::c_uint);
    g = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (h << 30 as ::core::ffi::c_int
                | h >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (h << 19 as ::core::ffi::c_int
                    | h >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (h << 10 as ::core::ffi::c_int
                    | h >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((b | a) & h | b & a);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (f as uint32_t)
                .wrapping_add(
                    ((c as uint32_t) << 26 as ::core::ffi::c_int
                        | c as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((c as uint32_t) << 21 as ::core::ffi::c_int
                            | c as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((c as uint32_t) << 7 as ::core::ffi::c_int
                            | c as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    c as uint32_t & (d as uint32_t ^ e as uint32_t) ^ e as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    b = b.wrapping_add(T1 as ::core::ffi::c_uint);
    f = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (g << 30 as ::core::ffi::c_int
                | g >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (g << 19 as ::core::ffi::c_int
                    | g >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (g << 10 as ::core::ffi::c_int
                    | g >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((a | h) & g | a & h);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (e as uint32_t)
                .wrapping_add(
                    ((b as uint32_t) << 26 as ::core::ffi::c_int
                        | b as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((b as uint32_t) << 21 as ::core::ffi::c_int
                            | b as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((b as uint32_t) << 7 as ::core::ffi::c_int
                            | b as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    b as uint32_t & (c as uint32_t ^ d as uint32_t) ^ d as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    a = a.wrapping_add(T1 as ::core::ffi::c_uint);
    e = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (f << 30 as ::core::ffi::c_int
                | f >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (f << 19 as ::core::ffi::c_int
                    | f >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (f << 10 as ::core::ffi::c_int
                    | f >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((h | g) & f | h & g);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (d as uint32_t)
                .wrapping_add(
                    ((a as uint32_t) << 26 as ::core::ffi::c_int
                        | a as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((a as uint32_t) << 21 as ::core::ffi::c_int
                            | a as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((a as uint32_t) << 7 as ::core::ffi::c_int
                            | a as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    a as uint32_t & (b as uint32_t ^ c as uint32_t) ^ c as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    h = h.wrapping_add(T1 as ::core::ffi::c_uint);
    d = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (e << 30 as ::core::ffi::c_int
                | e >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (e << 19 as ::core::ffi::c_int
                    | e >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (e << 10 as ::core::ffi::c_int
                    | e >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((g | f) & e | g & f);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (c as uint32_t)
                .wrapping_add(
                    ((h as uint32_t) << 26 as ::core::ffi::c_int
                        | h as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((h as uint32_t) << 21 as ::core::ffi::c_int
                            | h as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((h as uint32_t) << 7 as ::core::ffi::c_int
                            | h as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    h as uint32_t & (a as uint32_t ^ b as uint32_t) ^ b as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    g = g.wrapping_add(T1 as ::core::ffi::c_uint);
    c = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (d << 30 as ::core::ffi::c_int
                | d >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (d << 19 as ::core::ffi::c_int
                    | d >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (d << 10 as ::core::ffi::c_int
                    | d >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((f | e) & d | f & e);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (b as uint32_t)
                .wrapping_add(
                    ((g as uint32_t) << 26 as ::core::ffi::c_int
                        | g as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((g as uint32_t) << 21 as ::core::ffi::c_int
                            | g as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((g as uint32_t) << 7 as ::core::ffi::c_int
                            | g as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    g as uint32_t & (h as uint32_t ^ a as uint32_t) ^ a as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    f = f.wrapping_add(T1 as ::core::ffi::c_uint);
    b = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (c << 30 as ::core::ffi::c_int
                | c >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (c << 19 as ::core::ffi::c_int
                    | c >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (c << 10 as ::core::ffi::c_int
                    | c >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((e | d) & c | e & d);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (a as uint32_t)
                .wrapping_add(
                    ((f as uint32_t) << 26 as ::core::ffi::c_int
                        | f as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((f as uint32_t) << 21 as ::core::ffi::c_int
                            | f as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((f as uint32_t) << 7 as ::core::ffi::c_int
                            | f as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    f as uint32_t & (g as uint32_t ^ h as uint32_t) ^ h as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    e = e.wrapping_add(T1 as ::core::ffi::c_uint);
    a = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (b << 30 as ::core::ffi::c_int
                | b >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (b << 19 as ::core::ffi::c_int
                    | b >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (b << 10 as ::core::ffi::c_int
                    | b >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((d | c) & b | d & c);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (h as uint32_t)
                .wrapping_add(
                    ((e as uint32_t) << 26 as ::core::ffi::c_int
                        | e as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((e as uint32_t) << 21 as ::core::ffi::c_int
                            | e as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((e as uint32_t) << 7 as ::core::ffi::c_int
                            | e as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    e as uint32_t & (f as uint32_t ^ g as uint32_t) ^ g as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    d = d.wrapping_add(T1 as ::core::ffi::c_uint);
    h = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (a << 30 as ::core::ffi::c_int
                | a >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (a << 19 as ::core::ffi::c_int
                    | a >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (a << 10 as ::core::ffi::c_int
                    | a >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((c | b) & a | c & b);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (g as uint32_t)
                .wrapping_add(
                    ((d as uint32_t) << 26 as ::core::ffi::c_int
                        | d as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((d as uint32_t) << 21 as ::core::ffi::c_int
                            | d as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((d as uint32_t) << 7 as ::core::ffi::c_int
                            | d as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    d as uint32_t & (e as uint32_t ^ f as uint32_t) ^ f as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    c = c.wrapping_add(T1 as ::core::ffi::c_uint);
    g = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (h << 30 as ::core::ffi::c_int
                | h >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (h << 19 as ::core::ffi::c_int
                    | h >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (h << 10 as ::core::ffi::c_int
                    | h >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((b | a) & h | b & a);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (f as uint32_t)
                .wrapping_add(
                    ((c as uint32_t) << 26 as ::core::ffi::c_int
                        | c as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((c as uint32_t) << 21 as ::core::ffi::c_int
                            | c as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((c as uint32_t) << 7 as ::core::ffi::c_int
                            | c as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    c as uint32_t & (d as uint32_t ^ e as uint32_t) ^ e as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    b = b.wrapping_add(T1 as ::core::ffi::c_uint);
    f = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (g << 30 as ::core::ffi::c_int
                | g >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (g << 19 as ::core::ffi::c_int
                    | g >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (g << 10 as ::core::ffi::c_int
                    | g >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((a | h) & g | a & h);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (e as uint32_t)
                .wrapping_add(
                    ((b as uint32_t) << 26 as ::core::ffi::c_int
                        | b as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((b as uint32_t) << 21 as ::core::ffi::c_int
                            | b as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((b as uint32_t) << 7 as ::core::ffi::c_int
                            | b as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    b as uint32_t & (c as uint32_t ^ d as uint32_t) ^ d as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    a = a.wrapping_add(T1 as ::core::ffi::c_uint);
    e = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (f << 30 as ::core::ffi::c_int
                | f >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (f << 19 as ::core::ffi::c_int
                    | f >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (f << 10 as ::core::ffi::c_int
                    | f >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((h | g) & f | h & g);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (d as uint32_t)
                .wrapping_add(
                    ((a as uint32_t) << 26 as ::core::ffi::c_int
                        | a as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((a as uint32_t) << 21 as ::core::ffi::c_int
                            | a as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((a as uint32_t) << 7 as ::core::ffi::c_int
                            | a as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    a as uint32_t & (b as uint32_t ^ c as uint32_t) ^ c as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    h = h.wrapping_add(T1 as ::core::ffi::c_uint);
    d = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (e << 30 as ::core::ffi::c_int
                | e >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (e << 19 as ::core::ffi::c_int
                    | e >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (e << 10 as ::core::ffi::c_int
                    | e >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((g | f) & e | g & f);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (c as uint32_t)
                .wrapping_add(
                    ((h as uint32_t) << 26 as ::core::ffi::c_int
                        | h as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((h as uint32_t) << 21 as ::core::ffi::c_int
                            | h as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((h as uint32_t) << 7 as ::core::ffi::c_int
                            | h as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    h as uint32_t & (a as uint32_t ^ b as uint32_t) ^ b as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    g = g.wrapping_add(T1 as ::core::ffi::c_uint);
    c = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (d << 30 as ::core::ffi::c_int
                | d >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (d << 19 as ::core::ffi::c_int
                    | d >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (d << 10 as ::core::ffi::c_int
                    | d >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((f | e) & d | f & e);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (b as uint32_t)
                .wrapping_add(
                    ((g as uint32_t) << 26 as ::core::ffi::c_int
                        | g as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((g as uint32_t) << 21 as ::core::ffi::c_int
                            | g as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((g as uint32_t) << 7 as ::core::ffi::c_int
                            | g as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    g as uint32_t & (h as uint32_t ^ a as uint32_t) ^ a as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    f = f.wrapping_add(T1 as ::core::ffi::c_uint);
    b = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (c << 30 as ::core::ffi::c_int
                | c >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (c << 19 as ::core::ffi::c_int
                    | c >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (c << 10 as ::core::ffi::c_int
                    | c >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((e | d) & c | e & d);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (a as uint32_t)
                .wrapping_add(
                    ((f as uint32_t) << 26 as ::core::ffi::c_int
                        | f as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((f as uint32_t) << 21 as ::core::ffi::c_int
                            | f as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((f as uint32_t) << 7 as ::core::ffi::c_int
                            | f as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    f as uint32_t & (g as uint32_t ^ h as uint32_t) ^ h as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    e = e.wrapping_add(T1 as ::core::ffi::c_uint);
    a = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (b << 30 as ::core::ffi::c_int
                | b >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (b << 19 as ::core::ffi::c_int
                    | b >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (b << 10 as ::core::ffi::c_int
                    | b >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((d | c) & b | d & c);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (h as uint32_t)
                .wrapping_add(
                    ((e as uint32_t) << 26 as ::core::ffi::c_int
                        | e as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((e as uint32_t) << 21 as ::core::ffi::c_int
                            | e as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((e as uint32_t) << 7 as ::core::ffi::c_int
                            | e as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    e as uint32_t & (f as uint32_t ^ g as uint32_t) ^ g as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    d = d.wrapping_add(T1 as ::core::ffi::c_uint);
    h = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (a << 30 as ::core::ffi::c_int
                | a >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (a << 19 as ::core::ffi::c_int
                    | a >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (a << 10 as ::core::ffi::c_int
                    | a >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((c | b) & a | c & b);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (g as uint32_t)
                .wrapping_add(
                    ((d as uint32_t) << 26 as ::core::ffi::c_int
                        | d as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((d as uint32_t) << 21 as ::core::ffi::c_int
                            | d as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((d as uint32_t) << 7 as ::core::ffi::c_int
                            | d as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    d as uint32_t & (e as uint32_t ^ f as uint32_t) ^ f as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    c = c.wrapping_add(T1 as ::core::ffi::c_uint);
    g = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (h << 30 as ::core::ffi::c_int
                | h >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (h << 19 as ::core::ffi::c_int
                    | h >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (h << 10 as ::core::ffi::c_int
                    | h >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((b | a) & h | b & a);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (f as uint32_t)
                .wrapping_add(
                    ((c as uint32_t) << 26 as ::core::ffi::c_int
                        | c as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((c as uint32_t) << 21 as ::core::ffi::c_int
                            | c as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((c as uint32_t) << 7 as ::core::ffi::c_int
                            | c as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    c as uint32_t & (d as uint32_t ^ e as uint32_t) ^ e as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    b = b.wrapping_add(T1 as ::core::ffi::c_uint);
    f = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (g << 30 as ::core::ffi::c_int
                | g >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (g << 19 as ::core::ffi::c_int
                    | g >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (g << 10 as ::core::ffi::c_int
                    | g >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((a | h) & g | a & h);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (e as uint32_t)
                .wrapping_add(
                    ((b as uint32_t) << 26 as ::core::ffi::c_int
                        | b as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((b as uint32_t) << 21 as ::core::ffi::c_int
                            | b as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((b as uint32_t) << 7 as ::core::ffi::c_int
                            | b as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    b as uint32_t & (c as uint32_t ^ d as uint32_t) ^ d as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    a = a.wrapping_add(T1 as ::core::ffi::c_uint);
    e = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (f << 30 as ::core::ffi::c_int
                | f >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (f << 19 as ::core::ffi::c_int
                    | f >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (f << 10 as ::core::ffi::c_int
                    | f >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((h | g) & f | h & g);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (d as uint32_t)
                .wrapping_add(
                    ((a as uint32_t) << 26 as ::core::ffi::c_int
                        | a as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((a as uint32_t) << 21 as ::core::ffi::c_int
                            | a as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((a as uint32_t) << 7 as ::core::ffi::c_int
                            | a as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    a as uint32_t & (b as uint32_t ^ c as uint32_t) ^ c as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    h = h.wrapping_add(T1 as ::core::ffi::c_uint);
    d = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (e << 30 as ::core::ffi::c_int
                | e >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (e << 19 as ::core::ffi::c_int
                    | e >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (e << 10 as ::core::ffi::c_int
                    | e >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((g | f) & e | g & f);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (c as uint32_t)
                .wrapping_add(
                    ((h as uint32_t) << 26 as ::core::ffi::c_int
                        | h as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((h as uint32_t) << 21 as ::core::ffi::c_int
                            | h as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((h as uint32_t) << 7 as ::core::ffi::c_int
                            | h as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    h as uint32_t & (a as uint32_t ^ b as uint32_t) ^ b as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    g = g.wrapping_add(T1 as ::core::ffi::c_uint);
    c = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (d << 30 as ::core::ffi::c_int
                | d >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (d << 19 as ::core::ffi::c_int
                    | d >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (d << 10 as ::core::ffi::c_int
                    | d >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((f | e) & d | f & e);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (b as uint32_t)
                .wrapping_add(
                    ((g as uint32_t) << 26 as ::core::ffi::c_int
                        | g as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((g as uint32_t) << 21 as ::core::ffi::c_int
                            | g as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((g as uint32_t) << 7 as ::core::ffi::c_int
                            | g as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    g as uint32_t & (h as uint32_t ^ a as uint32_t) ^ a as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    f = f.wrapping_add(T1 as ::core::ffi::c_uint);
    b = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (c << 30 as ::core::ffi::c_int
                | c >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (c << 19 as ::core::ffi::c_int
                    | c >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (c << 10 as ::core::ffi::c_int
                    | c >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((e | d) & c | e & d);
    i = i.wrapping_add(1);
    block[i as usize] = block[i.wrapping_sub(16 as ::core::ffi::c_uint) as usize]
        .wrapping_add(
            (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                << 25 as ::core::ffi::c_int
                | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    << 14 as ::core::ffi::c_int
                    | block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(15 as ::core::ffi::c_uint) as usize]
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                << 15 as ::core::ffi::c_int
                | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                ^ (block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    << 13 as ::core::ffi::c_int
                    | block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                        >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
                ^ block[i.wrapping_sub(2 as ::core::ffi::c_uint) as usize]
                    >> 10 as ::core::ffi::c_int,
        )
        .wrapping_add(block[i.wrapping_sub(7 as ::core::ffi::c_uint) as usize]);
    T1 = block[i as usize];
    T1 = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (a as uint32_t)
                .wrapping_add(
                    ((f as uint32_t) << 26 as ::core::ffi::c_int
                        | f as uint32_t
                            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
                        ^ ((f as uint32_t) << 21 as ::core::ffi::c_int
                            | f as uint32_t
                                >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int)
                        ^ ((f as uint32_t) << 7 as ::core::ffi::c_int
                            | f as uint32_t
                                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int),
                )
                .wrapping_add(
                    f as uint32_t & (g as uint32_t ^ h as uint32_t) ^ h as uint32_t,
                )
                .wrapping_add(K256[i as usize]) as ::core::ffi::c_uint,
        ) as uint32_t as uint32_t;
    e = e.wrapping_add(T1 as ::core::ffi::c_uint);
    a = (T1 as ::core::ffi::c_uint)
        .wrapping_add(
            (b << 30 as ::core::ffi::c_int
                | b >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int)
                ^ (b << 19 as ::core::ffi::c_int
                    | b >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (b << 10 as ::core::ffi::c_int
                    | b >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int),
        )
        .wrapping_add((d | c) & b | d & c);
    i = i.wrapping_add(1);
    let ref mut fresh0 = *state.offset(0 as ::core::ffi::c_int as isize);
    *fresh0 = (*fresh0 as ::core::ffi::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    let ref mut fresh1 = *state.offset(1 as ::core::ffi::c_int as isize);
    *fresh1 = (*fresh1 as ::core::ffi::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    let ref mut fresh2 = *state.offset(2 as ::core::ffi::c_int as isize);
    *fresh2 = (*fresh2 as ::core::ffi::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    let ref mut fresh3 = *state.offset(3 as ::core::ffi::c_int as isize);
    *fresh3 = (*fresh3 as ::core::ffi::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    let ref mut fresh4 = *state.offset(4 as ::core::ffi::c_int as isize);
    *fresh4 = (*fresh4 as ::core::ffi::c_uint).wrapping_add(e) as uint32_t as uint32_t;
    let ref mut fresh5 = *state.offset(5 as ::core::ffi::c_int as isize);
    *fresh5 = (*fresh5 as ::core::ffi::c_uint).wrapping_add(f) as uint32_t as uint32_t;
    let ref mut fresh6 = *state.offset(6 as ::core::ffi::c_int as isize);
    *fresh6 = (*fresh6 as ::core::ffi::c_uint).wrapping_add(g) as uint32_t as uint32_t;
    let ref mut fresh7 = *state.offset(7 as ::core::ffi::c_int as isize);
    *fresh7 = (*fresh7 as ::core::ffi::c_uint).wrapping_add(h) as uint32_t as uint32_t;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn av_sha_init(
    mut ctx: *mut AVSHA,
    mut bits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    (*ctx).digest_len = (bits >> 5 as ::core::ffi::c_int) as uint8_t;
    match bits {
        160 => {
            (*ctx).state[0 as ::core::ffi::c_int as usize] = 0x67452301
                as ::core::ffi::c_int as uint32_t;
            (*ctx).state[1 as ::core::ffi::c_int as usize] = 0xefcdab89
                as ::core::ffi::c_uint as uint32_t;
            (*ctx).state[2 as ::core::ffi::c_int as usize] = 0x98badcfe
                as ::core::ffi::c_uint as uint32_t;
            (*ctx).state[3 as ::core::ffi::c_int as usize] = 0x10325476
                as ::core::ffi::c_int as uint32_t;
            (*ctx).state[4 as ::core::ffi::c_int as usize] = 0xc3d2e1f0
                as ::core::ffi::c_uint as uint32_t;
            (*ctx).transform = Some(
                sha1_transform
                    as unsafe extern "C" fn(*mut uint32_t, *const uint8_t) -> (),
            ) as Option<unsafe extern "C" fn(*mut uint32_t, *const uint8_t) -> ()>;
        }
        224 => {
            (*ctx).state[0 as ::core::ffi::c_int as usize] = 0xc1059ed8
                as ::core::ffi::c_uint as uint32_t;
            (*ctx).state[1 as ::core::ffi::c_int as usize] = 0x367cd507
                as ::core::ffi::c_int as uint32_t;
            (*ctx).state[2 as ::core::ffi::c_int as usize] = 0x3070dd17
                as ::core::ffi::c_int as uint32_t;
            (*ctx).state[3 as ::core::ffi::c_int as usize] = 0xf70e5939
                as ::core::ffi::c_uint as uint32_t;
            (*ctx).state[4 as ::core::ffi::c_int as usize] = 0xffc00b31
                as ::core::ffi::c_uint as uint32_t;
            (*ctx).state[5 as ::core::ffi::c_int as usize] = 0x68581511
                as ::core::ffi::c_int as uint32_t;
            (*ctx).state[6 as ::core::ffi::c_int as usize] = 0x64f98fa7
                as ::core::ffi::c_int as uint32_t;
            (*ctx).state[7 as ::core::ffi::c_int as usize] = 0xbefa4fa4
                as ::core::ffi::c_uint as uint32_t;
            (*ctx).transform = Some(
                sha256_transform
                    as unsafe extern "C" fn(*mut uint32_t, *const uint8_t) -> (),
            ) as Option<unsafe extern "C" fn(*mut uint32_t, *const uint8_t) -> ()>;
        }
        256 => {
            (*ctx).state[0 as ::core::ffi::c_int as usize] = 0x6a09e667
                as ::core::ffi::c_int as uint32_t;
            (*ctx).state[1 as ::core::ffi::c_int as usize] = 0xbb67ae85
                as ::core::ffi::c_uint as uint32_t;
            (*ctx).state[2 as ::core::ffi::c_int as usize] = 0x3c6ef372
                as ::core::ffi::c_int as uint32_t;
            (*ctx).state[3 as ::core::ffi::c_int as usize] = 0xa54ff53a
                as ::core::ffi::c_uint as uint32_t;
            (*ctx).state[4 as ::core::ffi::c_int as usize] = 0x510e527f
                as ::core::ffi::c_int as uint32_t;
            (*ctx).state[5 as ::core::ffi::c_int as usize] = 0x9b05688c
                as ::core::ffi::c_uint as uint32_t;
            (*ctx).state[6 as ::core::ffi::c_int as usize] = 0x1f83d9ab
                as ::core::ffi::c_int as uint32_t;
            (*ctx).state[7 as ::core::ffi::c_int as usize] = 0x5be0cd19
                as ::core::ffi::c_int as uint32_t;
            (*ctx).transform = Some(
                sha256_transform
                    as unsafe extern "C" fn(*mut uint32_t, *const uint8_t) -> (),
            ) as Option<unsafe extern "C" fn(*mut uint32_t, *const uint8_t) -> ()>;
        }
        _ => return -(22 as ::core::ffi::c_int),
    }
    (*ctx).count = 0 as uint64_t;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_sha_update(
    mut ctx: *mut AVSHA,
    mut data: *const uint8_t,
    mut len: size_t,
) {
    let mut j: ::core::ffi::c_uint = 0;
    let mut i: size_t = 0;
    j = ((*ctx).count & 63 as uint64_t) as ::core::ffi::c_uint;
    (*ctx).count = (*ctx).count.wrapping_add(len as uint64_t);
    if len >= (64 as ::core::ffi::c_uint).wrapping_sub(j) {
        let mut end: *const uint8_t = ::core::ptr::null::<uint8_t>();
        i = (64 as ::core::ffi::c_uint).wrapping_sub(j) as size_t;
        memcpy(
            (&raw mut (*ctx).buffer as *mut uint8_t).offset(j as isize) as *mut uint8_t
                as *mut ::core::ffi::c_void,
            data as *const ::core::ffi::c_void,
            i,
        );
        (*ctx)
            .transform
            .expect(
                "non-null function pointer",
            )(
            &raw mut (*ctx).state as *mut uint32_t,
            &raw mut (*ctx).buffer as *mut uint8_t as *const uint8_t,
        );
        data = data.offset(i as isize);
        len = len.wrapping_sub(i);
        end = data.offset((len & !(63 as ::core::ffi::c_int) as size_t) as isize);
        len = len.wrapping_rem(64 as size_t);
        while data < end {
            (*ctx)
                .transform
                .expect(
                    "non-null function pointer",
                )(&raw mut (*ctx).state as *mut uint32_t, data as *const uint8_t);
            data = data.offset(64 as ::core::ffi::c_int as isize);
        }
        j = 0 as ::core::ffi::c_uint;
    }
    memcpy(
        (&raw mut (*ctx).buffer as *mut uint8_t).offset(j as isize) as *mut uint8_t
            as *mut ::core::ffi::c_void,
        data as *const ::core::ffi::c_void,
        len as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_sha_final(mut ctx: *mut AVSHA, mut digest: *mut uint8_t) {
    let mut i: ::core::ffi::c_int = 0;
    let mut finalcount: uint64_t = av_bswap64((*ctx).count << 3 as ::core::ffi::c_int);
    av_sha_update(
        ctx as *mut AVSHA,
        b"\x80\0" as *const u8 as *const ::core::ffi::c_char as *const uint8_t,
        1 as size_t,
    );
    while (*ctx).count & 63 as uint64_t != 56 as uint64_t {
        av_sha_update(
            ctx as *mut AVSHA,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *const uint8_t,
            1 as size_t,
        );
    }
    av_sha_update(ctx as *mut AVSHA, &raw mut finalcount as *mut uint8_t, 8 as size_t);
    i = 0 as ::core::ffi::c_int;
    while i < (*ctx).digest_len as ::core::ffi::c_int {
        (*(digest.offset((i * 4 as ::core::ffi::c_int) as isize) as *mut unaligned_32))
            .l = av_bswap32((*ctx).state[i as usize]);
        i += 1;
    }
}
