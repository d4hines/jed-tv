// Translated from FFmpeg 9.0.1 libavutil/random_seed.c
// (preprocessed for aarch64-unknown-linux-musl) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// Copyright (c) 2009 Baptiste Coudurier <baptiste.coudurier@gmail.com>
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
    fn avpriv_fopen_utf8(
        path: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn abort() -> !;
    static av_sha_size: ::core::ffi::c_int;
    fn clock() -> clock_t;
    fn av_sha_init(context: *mut AVSHA, bits: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn fclose(_: *mut FILE) -> ::core::ffi::c_int;
    fn av_sha_update(ctx: *mut AVSHA, data: *const uint8_t, len: size_t);
    fn av_sha_final(context: *mut AVSHA, digest: *mut uint8_t);
    fn fread(
        _: *mut ::core::ffi::c_void,
        _: size_t,
        _: size_t,
        _: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn clock_gettime(_: clockid_t, _: *mut timespec) -> ::core::ffi::c_int;
    fn setvbuf(
        _: *mut FILE,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
}
pub type size_t = ::core::ffi::c_ulong;
pub type time_t = ::core::ffi::c_long;
pub type clockid_t = ::core::ffi::c_int;
pub type clock_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: ::core::ffi::c_long,
}
impl timespec {
}
pub type FILE = _IO_FILE;
pub type int64_t = ::core::ffi::c_long;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint64_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union unaligned_32 {
    pub l: uint32_t,
}
unsafe extern "C" fn read_random(
    mut dst: *mut uint8_t,
    mut len: size_t,
    mut file: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut fp: *mut FILE = avpriv_fopen_utf8(
        file,
        b"r\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut err: size_t = 0;
    if fp.is_null() {
        return -((('U' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('K' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('N' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    setvbuf(
        fp,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        2 as ::core::ffi::c_int,
        0 as size_t,
    );
    err = fread(dst as *mut ::core::ffi::c_void, 1 as size_t, len as size_t, fp)
        as size_t;
    fclose(fp);
    if err != len {
        return -((('U' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('K' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('N' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
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
unsafe extern "C" fn get_generic_seed() -> uint32_t {
    let mut tmp: [uint64_t; 15] = [0; 15];
    let mut sha: *mut AVSHA = &raw mut tmp as *mut uint64_t as *mut ::core::ffi::c_void
        as *mut AVSHA;
    let mut last_t: clock_t = 0 as clock_t;
    let mut last_td: clock_t = 0 as clock_t;
    let mut init_t: clock_t = 0 as clock_t;
    static mut i: uint64_t = 0 as uint64_t;
    static mut buffer: [uint32_t; 512] = [
        0 as ::core::ffi::c_int as uint32_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut digest: [::core::ffi::c_uchar; 20] = [0; 20];
    let mut last_i: uint64_t = i;
    let mut repeats: [::core::ffi::c_int; 3] = [0 as ::core::ffi::c_int; 3];
    if !(::core::mem::size_of::<[uint64_t; 15]>() as usize >= av_sha_size as usize) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"sizeof(tmp) >= av_sha_size\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavutil/random_seed.c\0" as *const u8 as *const ::core::ffi::c_char,
            88 as ::core::ffi::c_int,
        );
        abort();
    }
    buffer[13 as ::core::ffi::c_int as usize] = (buffer[13 as ::core::ffi::c_int
        as usize] as int64_t ^ ff_read_time()) as uint32_t;
    buffer[41 as ::core::ffi::c_int as usize] = (buffer[41 as ::core::ffi::c_int
        as usize] as int64_t ^ ff_read_time() >> 32 as ::core::ffi::c_int) as uint32_t;
    loop {
        let mut t: clock_t = clock();
        let mut incremented_i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut cur_td: ::core::ffi::c_int = (t - last_t) as ::core::ffi::c_int;
        if (last_t + 2 as clock_t * last_td
            + (1000000 as ::core::ffi::c_long > 1000 as ::core::ffi::c_long)
                as ::core::ffi::c_int as clock_t) < t
        {
            i = i.wrapping_add(1);
            buffer[(i & 511 as uint64_t) as usize] = (buffer[(i & 511 as uint64_t)
                as usize] as ::core::ffi::c_uint)
                .wrapping_add(
                    (cur_td as ::core::ffi::c_uint)
                        .wrapping_rem(3294638521 as ::core::ffi::c_uint),
                ) as uint32_t as uint32_t;
            incremented_i = 1 as ::core::ffi::c_int;
        } else if t != last_t
            && repeats[0 as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int
            && repeats[1 as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int
            && repeats[2 as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int
            && repeats[0 as ::core::ffi::c_int as usize]
                != repeats[1 as ::core::ffi::c_int as usize]
            && repeats[0 as ::core::ffi::c_int as usize]
                != repeats[2 as ::core::ffi::c_int as usize]
        {
            i = i.wrapping_add(1);
            buffer[(i & 511 as uint64_t) as usize] = (buffer[(i & 511 as uint64_t)
                as usize] as ::core::ffi::c_uint)
                .wrapping_add(
                    ((repeats[0 as ::core::ffi::c_int as usize]
                        + repeats[1 as ::core::ffi::c_int as usize]
                        + repeats[2 as ::core::ffi::c_int as usize])
                        as ::core::ffi::c_uint)
                        .wrapping_rem(3294638521 as ::core::ffi::c_uint),
                ) as uint32_t as uint32_t;
            incremented_i = 1 as ::core::ffi::c_int;
        } else {
            buffer[(i & 511 as uint64_t) as usize] = (1664525 as ::core::ffi::c_int
                as ::core::ffi::c_uint)
                .wrapping_mul(buffer[(i & 511 as uint64_t) as usize])
                .wrapping_add(1013904223 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    (cur_td as ::core::ffi::c_uint)
                        .wrapping_rem(3294638521 as ::core::ffi::c_uint),
                ) as uint32_t;
        }
        if incremented_i != 0
            && t - init_t >= 1000000 as ::core::ffi::c_long >> 5 as ::core::ffi::c_int
        {
            if last_i != 0 && i.wrapping_sub(last_i) > 4 as uint64_t
                || i.wrapping_sub(last_i) > 64 as uint64_t
                || 0 as ::core::ffi::c_int != 0 && i.wrapping_sub(last_i) > 8 as uint64_t
            {
                break;
            }
        }
        if t == last_t {
            repeats[0 as ::core::ffi::c_int as usize] += 1;
        } else {
            if repeats[0 as ::core::ffi::c_int as usize]
                != repeats[1 as ::core::ffi::c_int as usize]
            {
                repeats[2 as ::core::ffi::c_int as usize] = repeats[1
                    as ::core::ffi::c_int as usize];
                repeats[1 as ::core::ffi::c_int as usize] = repeats[0
                    as ::core::ffi::c_int as usize];
            }
            repeats[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
        }
        last_t = t;
        last_td = cur_td as clock_t;
        if init_t == 0 {
            init_t = t;
        }
    }
    buffer[111 as ::core::ffi::c_int as usize] = (buffer[111 as ::core::ffi::c_int
        as usize] as int64_t + ff_read_time()) as uint32_t;
    av_sha_init(sha, 160 as ::core::ffi::c_int);
    av_sha_update(
        sha,
        &raw mut buffer as *mut uint32_t as *const uint8_t,
        ::core::mem::size_of::<[uint32_t; 512]>() as size_t,
    );
    av_sha_final(sha, &raw mut digest as *mut uint8_t);
    return av_bswap32(
            (*(&raw mut digest as *mut ::core::ffi::c_uchar as *const unaligned_32)).l,
        )
        .wrapping_add(
            av_bswap32(
                (*((&raw mut digest as *mut ::core::ffi::c_uchar)
                    .offset(16 as ::core::ffi::c_int as isize) as *const unaligned_32))
                    .l,
            ),
        );
}
#[inline]
unsafe extern "C" fn ff_read_time() -> int64_t {
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(1 as clockid_t, &raw mut ts);
    return ts.tv_sec as int64_t * 1000000000 as int64_t + ts.tv_nsec as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn av_random_bytes(
    mut buf: *mut uint8_t,
    mut len: size_t,
) -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int = 0;
    err = read_random(
        buf,
        len,
        b"/dev/urandom\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if err == 0 {
        return err;
    }
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn av_get_random_seed() -> uint32_t {
    let mut seed: uint32_t = 0;
    if av_random_bytes(
        &raw mut seed as *mut uint8_t,
        ::core::mem::size_of::<uint32_t>() as size_t,
    ) < 0 as ::core::ffi::c_int
    {
        return get_generic_seed();
    }
    return seed;
}
#[repr(C)]
pub struct _IO_FILE {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVSHA {
    _opaque: [u8; 0],
}
