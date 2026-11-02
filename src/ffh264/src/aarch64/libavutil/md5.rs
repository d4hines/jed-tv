// Translated from FFmpeg 9.0.1 libavutil/md5.c
// (preprocessed for aarch64-unknown-linux-musl) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// Copyright (C) 2006 Michael Niedermayer (michaelni@gmx.at)
// Copyright (C) 2003-2005 by Christopher R. Hertel (crh@ubiqx.mn.org)
//
// References:
//  IETF RFC 1321: The MD5 Message-Digest Algorithm
//       Ron Rivest. IETF, April, 1992
//
// based on http://ubiqx.org/libcifs/source/Auth/MD5.c
//          from Christopher R. Hertel (crh@ubiqx.mn.org)
// Simplified, cleaned and IMO redundant comments removed by Michael.
//
// If you use gcc, then version 4.1 or later and -fomit-frame-pointer is
// strongly recommended.
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
pub type intptr_t = ::core::ffi::c_long;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint64_t = ::core::ffi::c_ulong;
pub type size_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union unaligned_32 {
    pub l: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVMD5 {
    pub len: uint64_t,
    pub block: [uint8_t; 64],
    pub ABCD: [uint32_t; 4],
}
#[no_mangle]
pub static mut av_md5_size: ::core::ffi::c_int = ::core::mem::size_of::<AVMD5>()
    as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn av_md5_alloc() -> *mut AVMD5 {
    return av_mallocz(::core::mem::size_of::<AVMD5>() as size_t) as *mut AVMD5;
}
static mut S: [[uint8_t; 4]; 4] = [
    [
        7 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
        17 as ::core::ffi::c_int as uint8_t,
        22 as ::core::ffi::c_int as uint8_t,
    ],
    [
        5 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
        20 as ::core::ffi::c_int as uint8_t,
    ],
    [
        4 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        16 as ::core::ffi::c_int as uint8_t,
        23 as ::core::ffi::c_int as uint8_t,
    ],
    [
        6 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        21 as ::core::ffi::c_int as uint8_t,
    ],
];
static mut T: [uint32_t; 64] = [
    0xd76aa478 as ::core::ffi::c_uint,
    0xe8c7b756 as ::core::ffi::c_uint,
    0x242070db as ::core::ffi::c_int as uint32_t,
    0xc1bdceee as ::core::ffi::c_uint,
    0xf57c0faf as ::core::ffi::c_uint,
    0x4787c62a as ::core::ffi::c_int as uint32_t,
    0xa8304613 as ::core::ffi::c_uint,
    0xfd469501 as ::core::ffi::c_uint,
    0x698098d8 as ::core::ffi::c_int as uint32_t,
    0x8b44f7af as ::core::ffi::c_uint,
    0xffff5bb1 as ::core::ffi::c_uint,
    0x895cd7be as ::core::ffi::c_uint,
    0x6b901122 as ::core::ffi::c_int as uint32_t,
    0xfd987193 as ::core::ffi::c_uint,
    0xa679438e as ::core::ffi::c_uint,
    0x49b40821 as ::core::ffi::c_int as uint32_t,
    0xf61e2562 as ::core::ffi::c_uint,
    0xc040b340 as ::core::ffi::c_uint,
    0x265e5a51 as ::core::ffi::c_int as uint32_t,
    0xe9b6c7aa as ::core::ffi::c_uint,
    0xd62f105d as ::core::ffi::c_uint,
    0x2441453 as ::core::ffi::c_int as uint32_t,
    0xd8a1e681 as ::core::ffi::c_uint,
    0xe7d3fbc8 as ::core::ffi::c_uint,
    0x21e1cde6 as ::core::ffi::c_int as uint32_t,
    0xc33707d6 as ::core::ffi::c_uint,
    0xf4d50d87 as ::core::ffi::c_uint,
    0x455a14ed as ::core::ffi::c_int as uint32_t,
    0xa9e3e905 as ::core::ffi::c_uint,
    0xfcefa3f8 as ::core::ffi::c_uint,
    0x676f02d9 as ::core::ffi::c_int as uint32_t,
    0x8d2a4c8a as ::core::ffi::c_uint,
    0xfffa3942 as ::core::ffi::c_uint,
    0x8771f681 as ::core::ffi::c_uint,
    0x6d9d6122 as ::core::ffi::c_int as uint32_t,
    0xfde5380c as ::core::ffi::c_uint,
    0xa4beea44 as ::core::ffi::c_uint,
    0x4bdecfa9 as ::core::ffi::c_int as uint32_t,
    0xf6bb4b60 as ::core::ffi::c_uint,
    0xbebfbc70 as ::core::ffi::c_uint,
    0x289b7ec6 as ::core::ffi::c_int as uint32_t,
    0xeaa127fa as ::core::ffi::c_uint,
    0xd4ef3085 as ::core::ffi::c_uint,
    0x4881d05 as ::core::ffi::c_int as uint32_t,
    0xd9d4d039 as ::core::ffi::c_uint,
    0xe6db99e5 as ::core::ffi::c_uint,
    0x1fa27cf8 as ::core::ffi::c_int as uint32_t,
    0xc4ac5665 as ::core::ffi::c_uint,
    0xf4292244 as ::core::ffi::c_uint,
    0x432aff97 as ::core::ffi::c_int as uint32_t,
    0xab9423a7 as ::core::ffi::c_uint,
    0xfc93a039 as ::core::ffi::c_uint,
    0x655b59c3 as ::core::ffi::c_int as uint32_t,
    0x8f0ccc92 as ::core::ffi::c_uint,
    0xffeff47d as ::core::ffi::c_uint,
    0x85845dd1 as ::core::ffi::c_uint,
    0x6fa87e4f as ::core::ffi::c_int as uint32_t,
    0xfe2ce6e0 as ::core::ffi::c_uint,
    0xa3014314 as ::core::ffi::c_uint,
    0x4e0811a1 as ::core::ffi::c_int as uint32_t,
    0xf7537e82 as ::core::ffi::c_uint,
    0xbd3af235 as ::core::ffi::c_uint,
    0x2ad7d2bb as ::core::ffi::c_int as uint32_t,
    0xeb86d391 as ::core::ffi::c_uint,
];
unsafe extern "C" fn body(
    mut ABCD: *mut uint32_t,
    mut src: *const uint8_t,
    mut nblocks: size_t,
) {
    let mut X: *const uint32_t = ::core::ptr::null::<uint32_t>();
    let mut a: uint32_t = 0;
    let mut b: uint32_t = 0;
    let mut c: uint32_t = 0;
    let mut d: uint32_t = 0;
    let mut t: uint32_t = 0;
    let mut n: size_t = 0 as size_t;
    while n < nblocks {
        a = *ABCD.offset(3 as ::core::ffi::c_int as isize);
        b = *ABCD.offset(2 as ::core::ffi::c_int as isize);
        c = *ABCD.offset(1 as ::core::ffi::c_int as isize);
        d = *ABCD.offset(0 as ::core::ffi::c_int as isize);
        X = (src as *const uint32_t).offset(n.wrapping_mul(16 as size_t) as isize);
        t = S[(0 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(0 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize]
            as uint32_t;
        a = a.wrapping_add(T[0 as ::core::ffi::c_int as usize]);
        if (0 as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
            if (0 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
                a = a
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (0 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                a = a
                    .wrapping_add(
                        (d & b | !d & c)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int * 0 as ::core::ffi::c_int
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (0 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int {
            a = a
                .wrapping_add(
                    (b ^ c ^ d)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int * 0 as ::core::ffi::c_int
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            a = a
                .wrapping_add(
                    (c ^ (b | !d))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int * 0 as ::core::ffi::c_int
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        a = b.wrapping_add(a << t | a >> (32 as uint32_t).wrapping_sub(t));
        t = S[(0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            as usize][(0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        d = d
            .wrapping_add(
                T[(0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize],
            );
        if (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
        {
            if (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                < 16 as ::core::ffi::c_int
            {
                d = d
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                d = d
                    .wrapping_add(
                        (c & a | !c & b)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            < 48 as ::core::ffi::c_int
        {
            d = d
                .wrapping_add(
                    (a ^ b ^ c)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            d = d
                .wrapping_add(
                    (b ^ (a | !c))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        d = a.wrapping_add(d << t | d >> (32 as uint32_t).wrapping_sub(t));
        t = S[(0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            as usize][(0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        c = c
            .wrapping_add(
                T[(0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize],
            );
        if (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
        {
            if (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                < 16 as ::core::ffi::c_int
            {
                c = c
                    .wrapping_add(
                        (b ^ d & (a ^ b))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                c = c
                    .wrapping_add(
                        (b & d | !b & a)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            < 48 as ::core::ffi::c_int
        {
            c = c
                .wrapping_add(
                    (d ^ a ^ b)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            c = c
                .wrapping_add(
                    (a ^ (d | !b))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        c = d.wrapping_add(c << t | c >> (32 as uint32_t).wrapping_sub(t));
        t = S[(0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            as usize][(0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
            & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        b = b
            .wrapping_add(
                T[(0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize],
            );
        if (0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
        {
            if (0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
                < 16 as ::core::ffi::c_int
            {
                b = b
                    .wrapping_add(
                        (a ^ c & (d ^ a))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                b = b
                    .wrapping_add(
                        (a & c | !a & d)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
            < 48 as ::core::ffi::c_int
        {
            b = b
                .wrapping_add(
                    (c ^ d ^ a)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            b = b
                .wrapping_add(
                    (d ^ (c | !a))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        b = c.wrapping_add(b << t | b >> (32 as uint32_t).wrapping_sub(t));
        t = S[(0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            as usize][(0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        a = a
            .wrapping_add(
                T[(0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as usize],
            );
        if (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
        {
            if (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                < 16 as ::core::ffi::c_int
            {
                a = a
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                a = a
                    .wrapping_add(
                        (d & b | !d & c)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
            < 48 as ::core::ffi::c_int
        {
            a = a
                .wrapping_add(
                    (b ^ c ^ d)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            a = a
                .wrapping_add(
                    (c ^ (b | !d))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        a = b.wrapping_add(a << t | a >> (32 as uint32_t).wrapping_sub(t));
        t = S[(0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        d = d
            .wrapping_add(
                T[(0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as usize],
            );
        if (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                d = d
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                d = d
                    .wrapping_add(
                        (c & a | !c & b)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                                    + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            d = d
                .wrapping_add(
                    (a ^ b ^ c)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                                + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            d = d
                .wrapping_add(
                    (b ^ (a | !c))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        d = a.wrapping_add(d << t | d >> (32 as uint32_t).wrapping_sub(t));
        t = S[(0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        c = c
            .wrapping_add(
                T[(0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as usize],
            );
        if (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                c = c
                    .wrapping_add(
                        (b ^ d & (a ^ b))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                            + 2 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                c = c
                    .wrapping_add(
                        (b & d | !b & a)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                                    + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            c = c
                .wrapping_add(
                    (d ^ a ^ b)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                                + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            c = c
                .wrapping_add(
                    (a ^ (d | !b))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                            + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        c = d.wrapping_add(c << t | c >> (32 as uint32_t).wrapping_sub(t));
        t = S[(0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        b = b
            .wrapping_add(
                T[(0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                    + 3 as ::core::ffi::c_int) as usize],
            );
        if (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                b = b
                    .wrapping_add(
                        (a ^ c & (d ^ a))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                            + 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                b = b
                    .wrapping_add(
                        (a & c | !a & d)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                                    + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            b = b
                .wrapping_add(
                    (c ^ d ^ a)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                                + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            b = b
                .wrapping_add(
                    (d ^ (c | !a))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                            + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        b = c.wrapping_add(b << t | b >> (32 as uint32_t).wrapping_sub(t));
        t = S[(0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            as usize][(0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        a = a
            .wrapping_add(
                T[(0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as usize],
            );
        if (0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
        {
            if (0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
                < 16 as ::core::ffi::c_int
            {
                a = a
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                a = a
                    .wrapping_add(
                        (d & b | !d & c)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
            < 48 as ::core::ffi::c_int
        {
            a = a
                .wrapping_add(
                    (b ^ c ^ d)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            a = a
                .wrapping_add(
                    (c ^ (b | !d))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        a = b.wrapping_add(a << t | a >> (32 as uint32_t).wrapping_sub(t));
        t = S[(0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        d = d
            .wrapping_add(
                T[(0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as usize],
            );
        if (0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                d = d
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                d = d
                    .wrapping_add(
                        (c & a | !c & b)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                                    + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            d = d
                .wrapping_add(
                    (a ^ b ^ c)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                                + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            d = d
                .wrapping_add(
                    (b ^ (a | !c))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        d = a.wrapping_add(d << t | d >> (32 as uint32_t).wrapping_sub(t));
        t = S[(0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        c = c
            .wrapping_add(
                T[(0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as usize],
            );
        if (0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                c = c
                    .wrapping_add(
                        (b ^ d & (a ^ b))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                            + 2 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                c = c
                    .wrapping_add(
                        (b & d | !b & a)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                                    + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            c = c
                .wrapping_add(
                    (d ^ a ^ b)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                                + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            c = c
                .wrapping_add(
                    (a ^ (d | !b))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                            + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        c = d.wrapping_add(c << t | c >> (32 as uint32_t).wrapping_sub(t));
        t = S[(0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        b = b
            .wrapping_add(
                T[(0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                    + 3 as ::core::ffi::c_int) as usize],
            );
        if (0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                b = b
                    .wrapping_add(
                        (a ^ c & (d ^ a))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                            + 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                b = b
                    .wrapping_add(
                        (a & c | !a & d)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                                    + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            b = b
                .wrapping_add(
                    (c ^ d ^ a)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                                + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            b = b
                .wrapping_add(
                    (d ^ (c | !a))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                            + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        b = c.wrapping_add(b << t | b >> (32 as uint32_t).wrapping_sub(t));
        t = S[(0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            as usize][(0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        a = a
            .wrapping_add(
                T[(0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as usize],
            );
        if (0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
                < 16 as ::core::ffi::c_int
            {
                a = a
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                a = a
                    .wrapping_add(
                        (d & b | !d & c)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
            < 48 as ::core::ffi::c_int
        {
            a = a
                .wrapping_add(
                    (b ^ c ^ d)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            a = a
                .wrapping_add(
                    (c ^ (b | !d))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        a = b.wrapping_add(a << t | a >> (32 as uint32_t).wrapping_sub(t));
        t = S[(0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        d = d
            .wrapping_add(
                T[(0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as usize],
            );
        if (0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                d = d
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                d = d
                    .wrapping_add(
                        (c & a | !c & b)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                                    + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            d = d
                .wrapping_add(
                    (a ^ b ^ c)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                                + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            d = d
                .wrapping_add(
                    (b ^ (a | !c))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        d = a.wrapping_add(d << t | d >> (32 as uint32_t).wrapping_sub(t));
        t = S[(0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        c = c
            .wrapping_add(
                T[(0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as usize],
            );
        if (0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                c = c
                    .wrapping_add(
                        (b ^ d & (a ^ b))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                            + 2 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                c = c
                    .wrapping_add(
                        (b & d | !b & a)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                                    + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            c = c
                .wrapping_add(
                    (d ^ a ^ b)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                                + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            c = c
                .wrapping_add(
                    (a ^ (d | !b))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                            + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        c = d.wrapping_add(c << t | c >> (32 as uint32_t).wrapping_sub(t));
        t = S[(0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        b = b
            .wrapping_add(
                T[(0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                    + 3 as ::core::ffi::c_int) as usize],
            );
        if (0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                b = b
                    .wrapping_add(
                        (a ^ c & (d ^ a))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                            + 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                b = b
                    .wrapping_add(
                        (a & c | !a & d)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                                    + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            b = b
                .wrapping_add(
                    (c ^ d ^ a)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                                + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            b = b
                .wrapping_add(
                    (d ^ (c | !a))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (0 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                            + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        b = c.wrapping_add(b << t | b >> (32 as uint32_t).wrapping_sub(t));
        t = S[(16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(16 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize]
            as uint32_t;
        a = a.wrapping_add(T[16 as ::core::ffi::c_int as usize]);
        if (16 as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
            if (16 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
                a = a
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (16 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                a = a
                    .wrapping_add(
                        (d & b | !d & c)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (16 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int {
            a = a
                .wrapping_add(
                    (b ^ c ^ d)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            a = a
                .wrapping_add(
                    (c ^ (b | !d))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        a = b.wrapping_add(a << t | a >> (32 as uint32_t).wrapping_sub(t));
        t = S[(16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            as usize][(16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        d = d
            .wrapping_add(
                T[(16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize],
            );
        if (16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                < 16 as ::core::ffi::c_int
            {
                d = d
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                d = d
                    .wrapping_add(
                        (c & a | !c & b)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            < 48 as ::core::ffi::c_int
        {
            d = d
                .wrapping_add(
                    (a ^ b ^ c)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            d = d
                .wrapping_add(
                    (b ^ (a | !c))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        d = a.wrapping_add(d << t | d >> (32 as uint32_t).wrapping_sub(t));
        t = S[(16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            as usize][(16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        c = c
            .wrapping_add(
                T[(16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize],
            );
        if (16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                < 16 as ::core::ffi::c_int
            {
                c = c
                    .wrapping_add(
                        (b ^ d & (a ^ b))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                c = c
                    .wrapping_add(
                        (b & d | !b & a)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            < 48 as ::core::ffi::c_int
        {
            c = c
                .wrapping_add(
                    (d ^ a ^ b)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            c = c
                .wrapping_add(
                    (a ^ (d | !b))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        c = d.wrapping_add(c << t | c >> (32 as uint32_t).wrapping_sub(t));
        t = S[(16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            as usize][(16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
            & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        b = b
            .wrapping_add(
                T[(16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize],
            );
        if (16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
                < 16 as ::core::ffi::c_int
            {
                b = b
                    .wrapping_add(
                        (a ^ c & (d ^ a))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                b = b
                    .wrapping_add(
                        (a & c | !a & d)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
            < 48 as ::core::ffi::c_int
        {
            b = b
                .wrapping_add(
                    (c ^ d ^ a)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            b = b
                .wrapping_add(
                    (d ^ (c | !a))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        b = c.wrapping_add(b << t | b >> (32 as uint32_t).wrapping_sub(t));
        t = S[(16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            as usize][(16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        a = a
            .wrapping_add(
                T[(16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as usize],
            );
        if (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                < 16 as ::core::ffi::c_int
            {
                a = a
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                a = a
                    .wrapping_add(
                        (d & b | !d & c)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
            < 48 as ::core::ffi::c_int
        {
            a = a
                .wrapping_add(
                    (b ^ c ^ d)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            a = a
                .wrapping_add(
                    (c ^ (b | !d))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        a = b.wrapping_add(a << t | a >> (32 as uint32_t).wrapping_sub(t));
        t = S[(16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        d = d
            .wrapping_add(
                T[(16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as usize],
            );
        if (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                d = d
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                d = d
                    .wrapping_add(
                        (c & a | !c & b)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                                    + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            d = d
                .wrapping_add(
                    (a ^ b ^ c)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                                + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            d = d
                .wrapping_add(
                    (b ^ (a | !c))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        d = a.wrapping_add(d << t | d >> (32 as uint32_t).wrapping_sub(t));
        t = S[(16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        c = c
            .wrapping_add(
                T[(16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as usize],
            );
        if (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                c = c
                    .wrapping_add(
                        (b ^ d & (a ^ b))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                            + 2 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                c = c
                    .wrapping_add(
                        (b & d | !b & a)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                                    + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            c = c
                .wrapping_add(
                    (d ^ a ^ b)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                                + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            c = c
                .wrapping_add(
                    (a ^ (d | !b))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                            + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        c = d.wrapping_add(c << t | c >> (32 as uint32_t).wrapping_sub(t));
        t = S[(16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        b = b
            .wrapping_add(
                T[(16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                    + 3 as ::core::ffi::c_int) as usize],
            );
        if (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                b = b
                    .wrapping_add(
                        (a ^ c & (d ^ a))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                            + 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                b = b
                    .wrapping_add(
                        (a & c | !a & d)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                                    + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            b = b
                .wrapping_add(
                    (c ^ d ^ a)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                                + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            b = b
                .wrapping_add(
                    (d ^ (c | !a))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                            + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        b = c.wrapping_add(b << t | b >> (32 as uint32_t).wrapping_sub(t));
        t = S[(16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            as usize][(16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        a = a
            .wrapping_add(
                T[(16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as usize],
            );
        if (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
                < 16 as ::core::ffi::c_int
            {
                a = a
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                a = a
                    .wrapping_add(
                        (d & b | !d & c)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
            < 48 as ::core::ffi::c_int
        {
            a = a
                .wrapping_add(
                    (b ^ c ^ d)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            a = a
                .wrapping_add(
                    (c ^ (b | !d))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        a = b.wrapping_add(a << t | a >> (32 as uint32_t).wrapping_sub(t));
        t = S[(16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        d = d
            .wrapping_add(
                T[(16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as usize],
            );
        if (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                d = d
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                d = d
                    .wrapping_add(
                        (c & a | !c & b)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                                    + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            d = d
                .wrapping_add(
                    (a ^ b ^ c)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                                + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            d = d
                .wrapping_add(
                    (b ^ (a | !c))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        d = a.wrapping_add(d << t | d >> (32 as uint32_t).wrapping_sub(t));
        t = S[(16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        c = c
            .wrapping_add(
                T[(16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as usize],
            );
        if (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                c = c
                    .wrapping_add(
                        (b ^ d & (a ^ b))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                            + 2 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                c = c
                    .wrapping_add(
                        (b & d | !b & a)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                                    + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            c = c
                .wrapping_add(
                    (d ^ a ^ b)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                                + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            c = c
                .wrapping_add(
                    (a ^ (d | !b))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                            + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        c = d.wrapping_add(c << t | c >> (32 as uint32_t).wrapping_sub(t));
        t = S[(16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        b = b
            .wrapping_add(
                T[(16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                    + 3 as ::core::ffi::c_int) as usize],
            );
        if (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                b = b
                    .wrapping_add(
                        (a ^ c & (d ^ a))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                            + 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                b = b
                    .wrapping_add(
                        (a & c | !a & d)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                                    + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            b = b
                .wrapping_add(
                    (c ^ d ^ a)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                                + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            b = b
                .wrapping_add(
                    (d ^ (c | !a))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                            + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        b = c.wrapping_add(b << t | b >> (32 as uint32_t).wrapping_sub(t));
        t = S[(16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            as usize][(16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        a = a
            .wrapping_add(
                T[(16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as usize],
            );
        if (16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
                < 16 as ::core::ffi::c_int
            {
                a = a
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                a = a
                    .wrapping_add(
                        (d & b | !d & c)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
            < 48 as ::core::ffi::c_int
        {
            a = a
                .wrapping_add(
                    (b ^ c ^ d)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            a = a
                .wrapping_add(
                    (c ^ (b | !d))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        a = b.wrapping_add(a << t | a >> (32 as uint32_t).wrapping_sub(t));
        t = S[(16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        d = d
            .wrapping_add(
                T[(16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as usize],
            );
        if (16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
        {
            if (16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                d = d
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                d = d
                    .wrapping_add(
                        (c & a | !c & b)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                                    + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            d = d
                .wrapping_add(
                    (a ^ b ^ c)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                                + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            d = d
                .wrapping_add(
                    (b ^ (a | !c))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        d = a.wrapping_add(d << t | d >> (32 as uint32_t).wrapping_sub(t));
        t = S[(16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        c = c
            .wrapping_add(
                T[(16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as usize],
            );
        if (16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
        {
            if (16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                c = c
                    .wrapping_add(
                        (b ^ d & (a ^ b))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                            + 2 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                c = c
                    .wrapping_add(
                        (b & d | !b & a)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                                    + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            c = c
                .wrapping_add(
                    (d ^ a ^ b)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                                + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            c = c
                .wrapping_add(
                    (a ^ (d | !b))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                            + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        c = d.wrapping_add(c << t | c >> (32 as uint32_t).wrapping_sub(t));
        t = S[(16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        b = b
            .wrapping_add(
                T[(16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                    + 3 as ::core::ffi::c_int) as usize],
            );
        if (16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
        {
            if (16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                b = b
                    .wrapping_add(
                        (a ^ c & (d ^ a))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                            + 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                b = b
                    .wrapping_add(
                        (a & c | !a & d)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                                    + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            b = b
                .wrapping_add(
                    (c ^ d ^ a)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                                + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            b = b
                .wrapping_add(
                    (d ^ (c | !a))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                            + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        b = c.wrapping_add(b << t | b >> (32 as uint32_t).wrapping_sub(t));
        t = S[(32 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(32 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize]
            as uint32_t;
        a = a.wrapping_add(T[32 as ::core::ffi::c_int as usize]);
        if (32 as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
            if (32 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
                a = a
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (32 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                a = a
                    .wrapping_add(
                        (d & b | !d & c)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int * 32 as ::core::ffi::c_int
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (32 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int {
            a = a
                .wrapping_add(
                    (b ^ c ^ d)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            a = a
                .wrapping_add(
                    (c ^ (b | !d))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int * 32 as ::core::ffi::c_int
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        a = b.wrapping_add(a << t | a >> (32 as uint32_t).wrapping_sub(t));
        t = S[(32 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            as usize][(32 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        d = d
            .wrapping_add(
                T[(32 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize],
            );
        if (32 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (32 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                < 16 as ::core::ffi::c_int
            {
                d = d
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (32 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                d = d
                    .wrapping_add(
                        (c & a | !c & b)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (32 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (32 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            < 48 as ::core::ffi::c_int
        {
            d = d
                .wrapping_add(
                    (a ^ b ^ c)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (32 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            d = d
                .wrapping_add(
                    (b ^ (a | !c))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (32 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        d = a.wrapping_add(d << t | d >> (32 as uint32_t).wrapping_sub(t));
        t = S[(32 as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            as usize][(32 as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        c = c
            .wrapping_add(
                T[(32 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize],
            );
        if (32 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (32 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                < 16 as ::core::ffi::c_int
            {
                c = c
                    .wrapping_add(
                        (b ^ d & (a ^ b))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (32 as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                c = c
                    .wrapping_add(
                        (b & d | !b & a)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (32 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (32 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            < 48 as ::core::ffi::c_int
        {
            c = c
                .wrapping_add(
                    (d ^ a ^ b)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (32 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            c = c
                .wrapping_add(
                    (a ^ (d | !b))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (32 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        c = d.wrapping_add(c << t | c >> (32 as uint32_t).wrapping_sub(t));
        t = S[(32 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            as usize][(32 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
            & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        b = b
            .wrapping_add(
                T[(32 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize],
            );
        if (32 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (32 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
                < 16 as ::core::ffi::c_int
            {
                b = b
                    .wrapping_add(
                        (a ^ c & (d ^ a))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (32 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                b = b
                    .wrapping_add(
                        (a & c | !a & d)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (32 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (32 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
            < 48 as ::core::ffi::c_int
        {
            b = b
                .wrapping_add(
                    (c ^ d ^ a)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (32 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            b = b
                .wrapping_add(
                    (d ^ (c | !a))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (32 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        b = c.wrapping_add(b << t | b >> (32 as uint32_t).wrapping_sub(t));
        t = S[(32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            as usize][(32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        a = a
            .wrapping_add(
                T[(32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as usize],
            );
        if (32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                < 16 as ::core::ffi::c_int
            {
                a = a
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                a = a
                    .wrapping_add(
                        (d & b | !d & c)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
            < 48 as ::core::ffi::c_int
        {
            a = a
                .wrapping_add(
                    (b ^ c ^ d)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            a = a
                .wrapping_add(
                    (c ^ (b | !d))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        a = b.wrapping_add(a << t | a >> (32 as uint32_t).wrapping_sub(t));
        t = S[(32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        d = d
            .wrapping_add(
                T[(32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as usize],
            );
        if (32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                d = d
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                d = d
                    .wrapping_add(
                        (c & a | !c & b)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                                    + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            d = d
                .wrapping_add(
                    (a ^ b ^ c)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                                + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            d = d
                .wrapping_add(
                    (b ^ (a | !c))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        d = a.wrapping_add(d << t | d >> (32 as uint32_t).wrapping_sub(t));
        t = S[(32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        c = c
            .wrapping_add(
                T[(32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as usize],
            );
        if (32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                c = c
                    .wrapping_add(
                        (b ^ d & (a ^ b))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                            + 2 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                c = c
                    .wrapping_add(
                        (b & d | !b & a)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                                    + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            c = c
                .wrapping_add(
                    (d ^ a ^ b)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                                + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            c = c
                .wrapping_add(
                    (a ^ (d | !b))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                            + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        c = d.wrapping_add(c << t | c >> (32 as uint32_t).wrapping_sub(t));
        t = S[(32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        b = b
            .wrapping_add(
                T[(32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                    + 3 as ::core::ffi::c_int) as usize],
            );
        if (32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                b = b
                    .wrapping_add(
                        (a ^ c & (d ^ a))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                            + 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                b = b
                    .wrapping_add(
                        (a & c | !a & d)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                                    + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            b = b
                .wrapping_add(
                    (c ^ d ^ a)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                                + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            b = b
                .wrapping_add(
                    (d ^ (c | !a))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (32 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                            + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        b = c.wrapping_add(b << t | b >> (32 as uint32_t).wrapping_sub(t));
        t = S[(32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            as usize][(32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        a = a
            .wrapping_add(
                T[(32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as usize],
            );
        if (32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
                < 16 as ::core::ffi::c_int
            {
                a = a
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                a = a
                    .wrapping_add(
                        (d & b | !d & c)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
            < 48 as ::core::ffi::c_int
        {
            a = a
                .wrapping_add(
                    (b ^ c ^ d)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            a = a
                .wrapping_add(
                    (c ^ (b | !d))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        a = b.wrapping_add(a << t | a >> (32 as uint32_t).wrapping_sub(t));
        t = S[(32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        d = d
            .wrapping_add(
                T[(32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as usize],
            );
        if (32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                d = d
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                d = d
                    .wrapping_add(
                        (c & a | !c & b)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                                    + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            d = d
                .wrapping_add(
                    (a ^ b ^ c)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                                + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            d = d
                .wrapping_add(
                    (b ^ (a | !c))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        d = a.wrapping_add(d << t | d >> (32 as uint32_t).wrapping_sub(t));
        t = S[(32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        c = c
            .wrapping_add(
                T[(32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as usize],
            );
        if (32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                c = c
                    .wrapping_add(
                        (b ^ d & (a ^ b))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                            + 2 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                c = c
                    .wrapping_add(
                        (b & d | !b & a)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                                    + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            c = c
                .wrapping_add(
                    (d ^ a ^ b)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                                + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            c = c
                .wrapping_add(
                    (a ^ (d | !b))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                            + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        c = d.wrapping_add(c << t | c >> (32 as uint32_t).wrapping_sub(t));
        t = S[(32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        b = b
            .wrapping_add(
                T[(32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                    + 3 as ::core::ffi::c_int) as usize],
            );
        if (32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                b = b
                    .wrapping_add(
                        (a ^ c & (d ^ a))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                            + 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                b = b
                    .wrapping_add(
                        (a & c | !a & d)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                                    + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            b = b
                .wrapping_add(
                    (c ^ d ^ a)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                                + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            b = b
                .wrapping_add(
                    (d ^ (c | !a))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                            + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        b = c.wrapping_add(b << t | b >> (32 as uint32_t).wrapping_sub(t));
        t = S[(32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            as usize][(32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        a = a
            .wrapping_add(
                T[(32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as usize],
            );
        if (32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
                < 16 as ::core::ffi::c_int
            {
                a = a
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                a = a
                    .wrapping_add(
                        (d & b | !d & c)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
            < 48 as ::core::ffi::c_int
        {
            a = a
                .wrapping_add(
                    (b ^ c ^ d)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            a = a
                .wrapping_add(
                    (c ^ (b | !d))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        a = b.wrapping_add(a << t | a >> (32 as uint32_t).wrapping_sub(t));
        t = S[(32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        d = d
            .wrapping_add(
                T[(32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as usize],
            );
        if (32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
        {
            if (32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                d = d
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                d = d
                    .wrapping_add(
                        (c & a | !c & b)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                                    + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            d = d
                .wrapping_add(
                    (a ^ b ^ c)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                                + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            d = d
                .wrapping_add(
                    (b ^ (a | !c))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        d = a.wrapping_add(d << t | d >> (32 as uint32_t).wrapping_sub(t));
        t = S[(32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        c = c
            .wrapping_add(
                T[(32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as usize],
            );
        if (32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
        {
            if (32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                c = c
                    .wrapping_add(
                        (b ^ d & (a ^ b))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                            + 2 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                c = c
                    .wrapping_add(
                        (b & d | !b & a)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                                    + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            c = c
                .wrapping_add(
                    (d ^ a ^ b)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                                + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            c = c
                .wrapping_add(
                    (a ^ (d | !b))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                            + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        c = d.wrapping_add(c << t | c >> (32 as uint32_t).wrapping_sub(t));
        t = S[(32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        b = b
            .wrapping_add(
                T[(32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                    + 3 as ::core::ffi::c_int) as usize],
            );
        if (32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
        {
            if (32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                b = b
                    .wrapping_add(
                        (a ^ c & (d ^ a))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                            + 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                b = b
                    .wrapping_add(
                        (a & c | !a & d)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                                    + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            b = b
                .wrapping_add(
                    (c ^ d ^ a)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                                + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            b = b
                .wrapping_add(
                    (d ^ (c | !a))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (32 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                            + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        b = c.wrapping_add(b << t | b >> (32 as uint32_t).wrapping_sub(t));
        t = S[(48 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(48 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize]
            as uint32_t;
        a = a.wrapping_add(T[48 as ::core::ffi::c_int as usize]);
        if (48 as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
            if (48 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
                a = a
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (48 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                a = a
                    .wrapping_add(
                        (d & b | !d & c)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int * 48 as ::core::ffi::c_int
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (48 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int {
            a = a
                .wrapping_add(
                    (b ^ c ^ d)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int * 48 as ::core::ffi::c_int
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            a = a
                .wrapping_add(
                    (c ^ (b | !d))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int * 48 as ::core::ffi::c_int
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        a = b.wrapping_add(a << t | a >> (32 as uint32_t).wrapping_sub(t));
        t = S[(48 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            as usize][(48 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        d = d
            .wrapping_add(
                T[(48 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize],
            );
        if (48 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (48 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                < 16 as ::core::ffi::c_int
            {
                d = d
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (48 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                d = d
                    .wrapping_add(
                        (c & a | !c & b)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (48 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (48 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            < 48 as ::core::ffi::c_int
        {
            d = d
                .wrapping_add(
                    (a ^ b ^ c)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (48 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            d = d
                .wrapping_add(
                    (b ^ (a | !c))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (48 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        d = a.wrapping_add(d << t | d >> (32 as uint32_t).wrapping_sub(t));
        t = S[(48 as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            as usize][(48 as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        c = c
            .wrapping_add(
                T[(48 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize],
            );
        if (48 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (48 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                < 16 as ::core::ffi::c_int
            {
                c = c
                    .wrapping_add(
                        (b ^ d & (a ^ b))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (48 as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                c = c
                    .wrapping_add(
                        (b & d | !b & a)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (48 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (48 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            < 48 as ::core::ffi::c_int
        {
            c = c
                .wrapping_add(
                    (d ^ a ^ b)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (48 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            c = c
                .wrapping_add(
                    (a ^ (d | !b))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (48 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        c = d.wrapping_add(c << t | c >> (32 as uint32_t).wrapping_sub(t));
        t = S[(48 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            as usize][(48 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
            & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        b = b
            .wrapping_add(
                T[(48 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize],
            );
        if (48 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (48 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
                < 16 as ::core::ffi::c_int
            {
                b = b
                    .wrapping_add(
                        (a ^ c & (d ^ a))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (48 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                b = b
                    .wrapping_add(
                        (a & c | !a & d)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (48 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (48 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
            < 48 as ::core::ffi::c_int
        {
            b = b
                .wrapping_add(
                    (c ^ d ^ a)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (48 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            b = b
                .wrapping_add(
                    (d ^ (c | !a))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (48 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        b = c.wrapping_add(b << t | b >> (32 as uint32_t).wrapping_sub(t));
        t = S[(48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            as usize][(48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        a = a
            .wrapping_add(
                T[(48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as usize],
            );
        if (48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                < 16 as ::core::ffi::c_int
            {
                a = a
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                a = a
                    .wrapping_add(
                        (d & b | !d & c)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
            < 48 as ::core::ffi::c_int
        {
            a = a
                .wrapping_add(
                    (b ^ c ^ d)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            a = a
                .wrapping_add(
                    (c ^ (b | !d))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        a = b.wrapping_add(a << t | a >> (32 as uint32_t).wrapping_sub(t));
        t = S[(48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        d = d
            .wrapping_add(
                T[(48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as usize],
            );
        if (48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                d = d
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                d = d
                    .wrapping_add(
                        (c & a | !c & b)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                                    + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            d = d
                .wrapping_add(
                    (a ^ b ^ c)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                                + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            d = d
                .wrapping_add(
                    (b ^ (a | !c))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        d = a.wrapping_add(d << t | d >> (32 as uint32_t).wrapping_sub(t));
        t = S[(48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        c = c
            .wrapping_add(
                T[(48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as usize],
            );
        if (48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                c = c
                    .wrapping_add(
                        (b ^ d & (a ^ b))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                            + 2 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                c = c
                    .wrapping_add(
                        (b & d | !b & a)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                                    + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            c = c
                .wrapping_add(
                    (d ^ a ^ b)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                                + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            c = c
                .wrapping_add(
                    (a ^ (d | !b))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                            + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        c = d.wrapping_add(c << t | c >> (32 as uint32_t).wrapping_sub(t));
        t = S[(48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        b = b
            .wrapping_add(
                T[(48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                    + 3 as ::core::ffi::c_int) as usize],
            );
        if (48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                b = b
                    .wrapping_add(
                        (a ^ c & (d ^ a))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                            + 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                b = b
                    .wrapping_add(
                        (a & c | !a & d)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                                    + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            b = b
                .wrapping_add(
                    (c ^ d ^ a)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                                + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            b = b
                .wrapping_add(
                    (d ^ (c | !a))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                            + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        b = c.wrapping_add(b << t | b >> (32 as uint32_t).wrapping_sub(t));
        t = S[(48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            as usize][(48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        a = a
            .wrapping_add(
                T[(48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as usize],
            );
        if (48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
                < 16 as ::core::ffi::c_int
            {
                a = a
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                a = a
                    .wrapping_add(
                        (d & b | !d & c)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
            < 48 as ::core::ffi::c_int
        {
            a = a
                .wrapping_add(
                    (b ^ c ^ d)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            a = a
                .wrapping_add(
                    (c ^ (b | !d))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        a = b.wrapping_add(a << t | a >> (32 as uint32_t).wrapping_sub(t));
        t = S[(48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        d = d
            .wrapping_add(
                T[(48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as usize],
            );
        if (48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                d = d
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                d = d
                    .wrapping_add(
                        (c & a | !c & b)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                                    + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            d = d
                .wrapping_add(
                    (a ^ b ^ c)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                                + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            d = d
                .wrapping_add(
                    (b ^ (a | !c))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        d = a.wrapping_add(d << t | d >> (32 as uint32_t).wrapping_sub(t));
        t = S[(48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        c = c
            .wrapping_add(
                T[(48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as usize],
            );
        if (48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                c = c
                    .wrapping_add(
                        (b ^ d & (a ^ b))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                            + 2 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                c = c
                    .wrapping_add(
                        (b & d | !b & a)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                                    + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            c = c
                .wrapping_add(
                    (d ^ a ^ b)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                                + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            c = c
                .wrapping_add(
                    (a ^ (d | !b))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                            + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        c = d.wrapping_add(c << t | c >> (32 as uint32_t).wrapping_sub(t));
        t = S[(48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        b = b
            .wrapping_add(
                T[(48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                    + 3 as ::core::ffi::c_int) as usize],
            );
        if (48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                b = b
                    .wrapping_add(
                        (a ^ c & (d ^ a))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                            + 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                b = b
                    .wrapping_add(
                        (a & c | !a & d)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                                    + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            b = b
                .wrapping_add(
                    (c ^ d ^ a)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                                + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            b = b
                .wrapping_add(
                    (d ^ (c | !a))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                            + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        b = c.wrapping_add(b << t | b >> (32 as uint32_t).wrapping_sub(t));
        t = S[(48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            as usize][(48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        a = a
            .wrapping_add(
                T[(48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as usize],
            );
        if (48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
        {
            if (48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
                < 16 as ::core::ffi::c_int
            {
                a = a
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                a = a
                    .wrapping_add(
                        (d & b | !d & c)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
                                            & 15 as ::core::ffi::c_int) as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
            < 48 as ::core::ffi::c_int
        {
            a = a
                .wrapping_add(
                    (b ^ c ^ d)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            a = a
                .wrapping_add(
                    (c ^ (b | !d))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
                                        & 15 as ::core::ffi::c_int) as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        a = b.wrapping_add(a << t | a >> (32 as uint32_t).wrapping_sub(t));
        t = S[(48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        d = d
            .wrapping_add(
                T[(48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as usize],
            );
        if (48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
        {
            if (48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                d = d
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                d = d
                    .wrapping_add(
                        (c & a | !c & b)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                                    + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            d = d
                .wrapping_add(
                    (a ^ b ^ c)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                                + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            d = d
                .wrapping_add(
                    (b ^ (a | !c))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        d = a.wrapping_add(d << t | d >> (32 as uint32_t).wrapping_sub(t));
        t = S[(48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        c = c
            .wrapping_add(
                T[(48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as usize],
            );
        if (48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
        {
            if (48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                c = c
                    .wrapping_add(
                        (b ^ d & (a ^ b))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                            + 2 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                c = c
                    .wrapping_add(
                        (b & d | !b & a)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                                    + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            c = c
                .wrapping_add(
                    (d ^ a ^ b)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                                + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            c = c
                .wrapping_add(
                    (a ^ (d | !b))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                            + 2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        c = d.wrapping_add(c << t | c >> (32 as uint32_t).wrapping_sub(t));
        t = S[(48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as usize][(48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize] as uint32_t;
        b = b
            .wrapping_add(
                T[(48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                    + 3 as ::core::ffi::c_int) as usize],
            );
        if (48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
        {
            if (48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int) < 16 as ::core::ffi::c_int
            {
                b = b
                    .wrapping_add(
                        (a ^ c & (d ^ a))
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                            + 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            } else {
                b = b
                    .wrapping_add(
                        (a & c | !a & d)
                            .wrapping_add(
                                (*(X
                                    .offset(
                                        (1 as ::core::ffi::c_int
                                            + 5 as ::core::ffi::c_int
                                                * (48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                                    + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const unaligned_32))
                                    .l,
                            ),
                    );
            }
        } else if (48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int) < 48 as ::core::ffi::c_int
        {
            b = b
                .wrapping_add(
                    (c ^ d ^ a)
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (5 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int
                                            * (48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                                + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        } else {
            b = b
                .wrapping_add(
                    (d ^ (c | !a))
                        .wrapping_add(
                            (*(X
                                .offset(
                                    (7 as ::core::ffi::c_int
                                        * (48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                            + 3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int)
                                        as isize,
                                ) as *const unaligned_32))
                                .l,
                        ),
                );
        }
        b = c.wrapping_add(b << t | b >> (32 as uint32_t).wrapping_sub(t));
        let ref mut fresh0 = *ABCD.offset(0 as ::core::ffi::c_int as isize);
        *fresh0 = (*fresh0).wrapping_add(d);
        let ref mut fresh1 = *ABCD.offset(1 as ::core::ffi::c_int as isize);
        *fresh1 = (*fresh1).wrapping_add(c);
        let ref mut fresh2 = *ABCD.offset(2 as ::core::ffi::c_int as isize);
        *fresh2 = (*fresh2).wrapping_add(b);
        let ref mut fresh3 = *ABCD.offset(3 as ::core::ffi::c_int as isize);
        *fresh3 = (*fresh3).wrapping_add(a);
        n = n.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn av_md5_init(mut ctx: *mut AVMD5) {
    (*ctx).len = 0 as uint64_t;
    (*ctx).ABCD[0 as ::core::ffi::c_int as usize] = 0x10325476 as ::core::ffi::c_int
        as uint32_t;
    (*ctx).ABCD[1 as ::core::ffi::c_int as usize] = 0x98badcfe as ::core::ffi::c_uint
        as uint32_t;
    (*ctx).ABCD[2 as ::core::ffi::c_int as usize] = 0xefcdab89 as ::core::ffi::c_uint
        as uint32_t;
    (*ctx).ABCD[3 as ::core::ffi::c_int as usize] = 0x67452301 as ::core::ffi::c_int
        as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn av_md5_update(
    mut ctx: *mut AVMD5,
    mut src: *const uint8_t,
    mut len: size_t,
) {
    let mut end: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut j: ::core::ffi::c_int = 0;
    j = ((*ctx).len & 63 as uint64_t) as ::core::ffi::c_int;
    (*ctx).len = ((*ctx).len as ::core::ffi::c_ulong)
        .wrapping_add(len as ::core::ffi::c_ulong) as uint64_t as uint64_t;
    if j != 0 {
        let mut cnt: ::core::ffi::c_int = (if len
            > (64 as ::core::ffi::c_int - j) as size_t
        {
            (64 as ::core::ffi::c_int - j) as size_t
        } else {
            len
        }) as ::core::ffi::c_int;
        memcpy(
            (&raw mut (*ctx).block as *mut uint8_t).offset(j as isize)
                as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            cnt as size_t,
        );
        src = src.offset(cnt as isize);
        len = len.wrapping_sub(cnt as size_t);
        if j + cnt < 64 as ::core::ffi::c_int {
            return;
        }
        body(
            &raw mut (*ctx).ABCD as *mut uint32_t,
            &raw mut (*ctx).block as *mut uint8_t,
            1 as size_t,
        );
    }
    end = src.offset((len & !(63 as ::core::ffi::c_int) as size_t) as isize);
    if 1 as ::core::ffi::c_int == 0 && src as intptr_t & 3 as intptr_t != 0 {
        while src < end {
            memcpy(
                &raw mut (*ctx).block as *mut uint8_t as *mut ::core::ffi::c_void,
                src as *const ::core::ffi::c_void,
                64 as size_t,
            );
            body(
                &raw mut (*ctx).ABCD as *mut uint32_t,
                &raw mut (*ctx).block as *mut uint8_t,
                1 as size_t,
            );
            src = src.offset(64 as ::core::ffi::c_int as isize);
        }
    } else {
        let mut nblocks: size_t = len.wrapping_div(64 as size_t);
        body(&raw mut (*ctx).ABCD as *mut uint32_t, src, nblocks);
        src = end;
    }
    len &= 63 as size_t;
    if len > 0 as size_t {
        memcpy(
            &raw mut (*ctx).block as *mut uint8_t as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            len as size_t,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn av_md5_final(mut ctx: *mut AVMD5, mut dst: *mut uint8_t) {
    let mut i: ::core::ffi::c_int = 0;
    let mut finalcount: uint64_t = (*ctx).len << 3 as ::core::ffi::c_int;
    av_md5_update(
        ctx,
        b"\x80\0" as *const u8 as *const ::core::ffi::c_char as *const uint8_t,
        1 as size_t,
    );
    while (*ctx).len & 63 as uint64_t != 56 as uint64_t {
        av_md5_update(
            ctx,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *const uint8_t,
            1 as size_t,
        );
    }
    av_md5_update(ctx, &raw mut finalcount as *mut uint8_t, 8 as size_t);
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        (*(dst.offset((4 as ::core::ffi::c_int * i) as isize) as *mut unaligned_32)).l = (*ctx)
            .ABCD[(3 as ::core::ffi::c_int - i) as usize];
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn av_md5_sum(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut len: size_t,
) {
    let mut ctx: AVMD5 = AVMD5 {
        len: 0,
        block: [0; 64],
        ABCD: [0; 4],
    };
    av_md5_init(&raw mut ctx);
    av_md5_update(&raw mut ctx, src, len);
    av_md5_final(&raw mut ctx, dst);
}
