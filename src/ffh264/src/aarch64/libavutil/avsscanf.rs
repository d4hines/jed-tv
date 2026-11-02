// Translated from FFmpeg 9.0.1 libavutil/avsscanf.c
// (preprocessed for aarch64-unknown-linux-musl) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// Copyright (c) 2005-2014 Rich Felker, et al.
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
// TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
// SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

extern "C" {
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memchr(
        _: *const ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn copysign(
        _: ::core::ffi::c_double,
        _: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn fabs(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fmod(_: ::core::ffi::c_double, _: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn pow(_: ::core::ffi::c_double, _: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn scalbn(_: ::core::ffi::c_double, _: ::core::ffi::c_int) -> ::core::ffi::c_double;
}
pub type __builtin_va_list = __va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list {
    pub __stack: *mut ::core::ffi::c_void,
    pub __gr_top: *mut ::core::ffi::c_void,
    pub __vr_top: *mut ::core::ffi::c_void,
    pub __gr_offs: ::core::ffi::c_int,
    pub __vr_offs: ::core::ffi::c_int,
}
pub type va_list = __builtin_va_list;
pub type ptrdiff_t = ::core::ffi::c_long;
pub type size_t = ::core::ffi::c_ulong;
pub type uintptr_t = ::core::ffi::c_ulong;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint64_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFFILE {
    pub buf_size: size_t,
    pub buf: *mut ::core::ffi::c_uchar,
    pub rpos: *mut ::core::ffi::c_uchar,
    pub rend: *mut ::core::ffi::c_uchar,
    pub shend: *mut ::core::ffi::c_uchar,
    pub shlim: ptrdiff_t,
    pub shcnt: ptrdiff_t,
    pub cookie: *mut ::core::ffi::c_void,
    pub read: Option<
        unsafe extern "C" fn(*mut FFFILE, *mut ::core::ffi::c_uchar, size_t) -> size_t,
    >,
}
unsafe extern "C" fn fftoread(mut f: *mut FFFILE) -> ::core::ffi::c_int {
    (*f).rend = (*f).buf.offset((*f).buf_size as isize);
    (*f).rpos = (*f).rend;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn ffstring_read(
    mut f: *mut FFFILE,
    mut buf: *mut ::core::ffi::c_uchar,
    mut len: size_t,
) -> size_t {
    let mut src: *mut ::core::ffi::c_char = (*f).cookie as *mut ::core::ffi::c_char;
    let mut k: size_t = len.wrapping_add(256 as size_t);
    let mut end: *mut ::core::ffi::c_char = memchr(
        src as *const ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        k,
    ) as *mut ::core::ffi::c_char;
    if !end.is_null() {
        k = end.offset_from(src) as ::core::ffi::c_long as size_t;
    }
    if k < len {
        len = k;
    }
    memcpy(buf as *mut ::core::ffi::c_void, src as *const ::core::ffi::c_void, len);
    (*f).rpos = src.offset(len as isize) as *mut ::core::ffi::c_void
        as *mut ::core::ffi::c_uchar;
    (*f).rend = src.offset(k as isize) as *mut ::core::ffi::c_void
        as *mut ::core::ffi::c_uchar;
    (*f).cookie = src.offset(k as isize) as *mut ::core::ffi::c_void;
    return len;
}
unsafe extern "C" fn ffuflow(mut f: *mut FFFILE) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_uchar = 0;
    if fftoread(f) == 0
        && (*f)
            .read
            .expect(
                "non-null function pointer",
            )(f as *mut FFFILE, &raw mut c, 1 as size_t) == 1 as size_t
    {
        return c as ::core::ffi::c_int;
    }
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn ffshlim(mut f: *mut FFFILE, mut lim: ptrdiff_t) {
    (*f).shlim = lim;
    (*f).shcnt = (*f).buf.offset_from((*f).rpos) as ::core::ffi::c_long as ptrdiff_t;
    if lim != 0 && (*f).rend.offset_from((*f).rpos) as ::core::ffi::c_long > lim {
        (*f).shend = (*f).rpos.offset(lim as isize);
    } else {
        (*f).shend = (*f).rend;
    };
}
unsafe extern "C" fn ffshgetc(mut f: *mut FFFILE) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0;
    let mut cnt: ptrdiff_t = (*f).shcnt + (*f).rpos.offset_from((*f).buf) as ptrdiff_t;
    if (*f).shlim != 0 && cnt >= (*f).shlim
        || {
            c = ffuflow(f);
            c < 0 as ::core::ffi::c_int
        }
    {
        (*f).shcnt = (*f).buf.offset_from((*f).rpos) as ptrdiff_t + cnt;
        (*f).shend = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        return -(1 as ::core::ffi::c_int);
    }
    cnt += 1;
    if (*f).shlim != 0
        && (*f).rend.offset_from((*f).rpos) as ::core::ffi::c_long > (*f).shlim - cnt
    {
        (*f).shend = (*f).rpos.offset(((*f).shlim - cnt) as isize);
    } else {
        (*f).shend = (*f).rend;
    }
    (*f).shcnt = (*f).buf.offset_from((*f).rpos) as ptrdiff_t + cnt;
    if *(*f).rpos.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int != c
    {
        *(*f).rpos.offset(-(1 as ::core::ffi::c_int) as isize) = c
            as ::core::ffi::c_uchar;
    }
    return c;
}
static mut table: [::core::ffi::c_uchar; 257] = [
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    16 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    17 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    18 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    19 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    20 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    21 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    23 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    25 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    26 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    29 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    30 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    31 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    32 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    33 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    34 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    35 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    16 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    17 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    18 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    19 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    20 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    21 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    23 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    25 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    26 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    29 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    30 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    31 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    32 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    33 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    34 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    35 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
];
unsafe extern "C" fn ffintscan(
    mut f: *mut FFFILE,
    mut base: ::core::ffi::c_uint,
    mut pok: ::core::ffi::c_int,
    mut lim: ::core::ffi::c_ulonglong,
) -> ::core::ffi::c_ulonglong {
    let mut current_block: u64;
    let mut val: *const ::core::ffi::c_uchar = (&raw const table
        as *const ::core::ffi::c_uchar)
        .offset(1 as ::core::ffi::c_int as isize);
    let mut c: ::core::ffi::c_int = 0;
    let mut neg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut x: ::core::ffi::c_uint = 0;
    let mut y: ::core::ffi::c_ulonglong = 0;
    if base > 36 as ::core::ffi::c_uint || base == 1 as ::core::ffi::c_uint {
        *__errno_location() = 22 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_ulonglong;
    }
    loop {
        c = if (*f).rpos < (*f).shend {
            let fresh34 = (*f).rpos;
            (*f).rpos = (*f).rpos.offset(1);
            *fresh34 as ::core::ffi::c_int
        } else {
            ffshgetc(f)
        };
        if !(av_isspace(c) != 0) {
            break;
        }
    }
    if c == '+' as i32 || c == '-' as i32 {
        neg = -((c == '-' as i32) as ::core::ffi::c_int);
        c = if (*f).rpos < (*f).shend {
            let fresh35 = (*f).rpos;
            (*f).rpos = (*f).rpos.offset(1);
            *fresh35 as ::core::ffi::c_int
        } else {
            ffshgetc(f)
        };
    }
    if (base == 0 as ::core::ffi::c_uint || base == 16 as ::core::ffi::c_uint)
        && c == '0' as i32
    {
        c = if (*f).rpos < (*f).shend {
            let fresh36 = (*f).rpos;
            (*f).rpos = (*f).rpos.offset(1);
            *fresh36 as ::core::ffi::c_int
        } else {
            ffshgetc(f)
        };
        if c | 32 as ::core::ffi::c_int == 'x' as i32 {
            c = if (*f).rpos < (*f).shend {
                let fresh37 = (*f).rpos;
                (*f).rpos = (*f).rpos.offset(1);
                *fresh37 as ::core::ffi::c_int
            } else {
                ffshgetc(f)
            };
            if *val.offset(c as isize) as ::core::ffi::c_int >= 16 as ::core::ffi::c_int
            {
                if !(*f).shend.is_null() {
                    (*f).rpos = (*f).rpos.offset(-1);
                } else {};
                if pok != 0 {
                    if !(*f).shend.is_null() {
                        (*f).rpos = (*f).rpos.offset(-1);
                    } else {};
                } else {
                    ffshlim(f, 0 as ptrdiff_t);
                }
                return 0 as ::core::ffi::c_ulonglong;
            }
            base = 16 as ::core::ffi::c_uint;
        } else if base == 0 as ::core::ffi::c_uint {
            base = 8 as ::core::ffi::c_uint;
        }
    } else {
        if base == 0 as ::core::ffi::c_uint {
            base = 10 as ::core::ffi::c_uint;
        }
        if *val.offset(c as isize) as ::core::ffi::c_uint >= base {
            if !(*f).shend.is_null() {
                (*f).rpos = (*f).rpos.offset(-1);
            } else {};
            ffshlim(f, 0 as ptrdiff_t);
            *__errno_location() = 22 as ::core::ffi::c_int;
            return 0 as ::core::ffi::c_ulonglong;
        }
    }
    if base == 10 as ::core::ffi::c_uint {
        x = 0 as ::core::ffi::c_uint;
        while ((c - '0' as i32) as ::core::ffi::c_uint) < 10 as ::core::ffi::c_uint
            && x
                <= (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(2 as ::core::ffi::c_uint)
                    .wrapping_add(1 as ::core::ffi::c_uint)
                    .wrapping_div(10 as ::core::ffi::c_uint)
                    .wrapping_sub(1 as ::core::ffi::c_uint)
        {
            x = x
                .wrapping_mul(10 as ::core::ffi::c_uint)
                .wrapping_add((c - '0' as i32) as ::core::ffi::c_uint);
            c = if (*f).rpos < (*f).shend {
                let fresh38 = (*f).rpos;
                (*f).rpos = (*f).rpos.offset(1);
                *fresh38 as ::core::ffi::c_int
            } else {
                ffshgetc(f)
            };
        }
        y = x as ::core::ffi::c_ulonglong;
        while ((c - '0' as i32) as ::core::ffi::c_uint) < 10 as ::core::ffi::c_uint
            && y
                <= (9223372036854775807 as ::core::ffi::c_ulonglong)
                    .wrapping_mul(2 as ::core::ffi::c_ulonglong)
                    .wrapping_add(1 as ::core::ffi::c_ulonglong)
                    .wrapping_div(10 as ::core::ffi::c_ulonglong)
            && (10 as ::core::ffi::c_ulonglong).wrapping_mul(y)
                <= (9223372036854775807 as ::core::ffi::c_ulonglong)
                    .wrapping_mul(2 as ::core::ffi::c_ulonglong)
                    .wrapping_add(1 as ::core::ffi::c_ulonglong)
                    .wrapping_sub((c - '0' as i32) as ::core::ffi::c_ulonglong)
        {
            y = y
                .wrapping_mul(10 as ::core::ffi::c_ulonglong)
                .wrapping_add((c - '0' as i32) as ::core::ffi::c_ulonglong);
            c = if (*f).rpos < (*f).shend {
                let fresh39 = (*f).rpos;
                (*f).rpos = (*f).rpos.offset(1);
                *fresh39 as ::core::ffi::c_int
            } else {
                ffshgetc(f)
            };
        }
        if (c - '0' as i32) as ::core::ffi::c_uint >= 10 as ::core::ffi::c_uint {
            current_block = 1179471111763444844;
        } else {
            current_block = 11793792312832361944;
        }
    } else {
        if base & base.wrapping_sub(1 as ::core::ffi::c_uint) == 0 {
            let mut bs: ::core::ffi::c_int = ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(
                *b"\0\x01\x02\x04\x07\x03\x06\x05\0",
            )[((0x17 as ::core::ffi::c_uint).wrapping_mul(base)
                >> 5 as ::core::ffi::c_int & 7 as ::core::ffi::c_uint) as usize]
                as ::core::ffi::c_int;
            x = 0 as ::core::ffi::c_uint;
            while (*val.offset(c as isize) as ::core::ffi::c_uint) < base
                && x
                    <= (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                        .wrapping_mul(2 as ::core::ffi::c_uint)
                        .wrapping_add(1 as ::core::ffi::c_uint)
                        .wrapping_div(32 as ::core::ffi::c_uint)
            {
                x = x << bs | *val.offset(c as isize) as ::core::ffi::c_uint;
                c = if (*f).rpos < (*f).shend {
                    let fresh40 = (*f).rpos;
                    (*f).rpos = (*f).rpos.offset(1);
                    *fresh40 as ::core::ffi::c_int
                } else {
                    ffshgetc(f)
                };
            }
            y = x as ::core::ffi::c_ulonglong;
            while (*val.offset(c as isize) as ::core::ffi::c_uint) < base
                && y
                    <= (9223372036854775807 as ::core::ffi::c_ulonglong)
                        .wrapping_mul(2 as ::core::ffi::c_ulonglong)
                        .wrapping_add(1 as ::core::ffi::c_ulonglong) >> bs
            {
                y = y << bs | *val.offset(c as isize) as ::core::ffi::c_ulonglong;
                c = if (*f).rpos < (*f).shend {
                    let fresh41 = (*f).rpos;
                    (*f).rpos = (*f).rpos.offset(1);
                    *fresh41 as ::core::ffi::c_int
                } else {
                    ffshgetc(f)
                };
            }
        } else {
            x = 0 as ::core::ffi::c_uint;
            while (*val.offset(c as isize) as ::core::ffi::c_uint) < base
                && x
                    <= (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                        .wrapping_mul(2 as ::core::ffi::c_uint)
                        .wrapping_add(1 as ::core::ffi::c_uint)
                        .wrapping_div(36 as ::core::ffi::c_uint)
                        .wrapping_sub(1 as ::core::ffi::c_uint)
            {
                x = x
                    .wrapping_mul(base)
                    .wrapping_add(*val.offset(c as isize) as ::core::ffi::c_uint);
                c = if (*f).rpos < (*f).shend {
                    let fresh42 = (*f).rpos;
                    (*f).rpos = (*f).rpos.offset(1);
                    *fresh42 as ::core::ffi::c_int
                } else {
                    ffshgetc(f)
                };
            }
            y = x as ::core::ffi::c_ulonglong;
            while (*val.offset(c as isize) as ::core::ffi::c_uint) < base
                && y
                    <= (9223372036854775807 as ::core::ffi::c_ulonglong)
                        .wrapping_mul(2 as ::core::ffi::c_ulonglong)
                        .wrapping_add(1 as ::core::ffi::c_ulonglong)
                        .wrapping_div(base as ::core::ffi::c_ulonglong)
                && (base as ::core::ffi::c_ulonglong).wrapping_mul(y)
                    <= (9223372036854775807 as ::core::ffi::c_ulonglong)
                        .wrapping_mul(2 as ::core::ffi::c_ulonglong)
                        .wrapping_add(1 as ::core::ffi::c_ulonglong)
                        .wrapping_sub(
                            *val.offset(c as isize) as ::core::ffi::c_ulonglong,
                        )
            {
                y = y
                    .wrapping_mul(base as ::core::ffi::c_ulonglong)
                    .wrapping_add(*val.offset(c as isize) as ::core::ffi::c_ulonglong);
                c = if (*f).rpos < (*f).shend {
                    let fresh43 = (*f).rpos;
                    (*f).rpos = (*f).rpos.offset(1);
                    *fresh43 as ::core::ffi::c_int
                } else {
                    ffshgetc(f)
                };
            }
        }
        current_block = 11793792312832361944;
    }
    match current_block {
        11793792312832361944 => {
            if (*val.offset(c as isize) as ::core::ffi::c_uint) < base {
                while (*val.offset(c as isize) as ::core::ffi::c_uint) < base {
                    c = if (*f).rpos < (*f).shend {
                        let fresh44 = (*f).rpos;
                        (*f).rpos = (*f).rpos.offset(1);
                        *fresh44 as ::core::ffi::c_int
                    } else {
                        ffshgetc(f)
                    };
                }
                *__errno_location() = 34 as ::core::ffi::c_int;
                y = lim;
                if lim & 1 as ::core::ffi::c_ulonglong != 0 {
                    neg = 0 as ::core::ffi::c_int;
                }
            }
        }
        _ => {}
    }
    if !(*f).shend.is_null() {
        (*f).rpos = (*f).rpos.offset(-1);
    } else {};
    if y >= lim {
        if lim & 1 as ::core::ffi::c_ulonglong == 0 && neg == 0 {
            *__errno_location() = 34 as ::core::ffi::c_int;
            return lim.wrapping_sub(1 as ::core::ffi::c_ulonglong);
        } else if y > lim {
            *__errno_location() = 34 as ::core::ffi::c_int;
            return lim;
        }
    }
    return (y ^ neg as ::core::ffi::c_ulonglong)
        .wrapping_sub(neg as ::core::ffi::c_ulonglong);
}
#[inline]
unsafe extern "C" fn av_isdigit(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (c >= '0' as i32 && c <= '9' as i32) as ::core::ffi::c_int;
}
unsafe extern "C" fn scanexp(
    mut f: *mut FFFILE,
    mut pok: ::core::ffi::c_int,
) -> ::core::ffi::c_longlong {
    let mut c: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_longlong = 0;
    let mut neg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    c = if (*f).rpos < (*f).shend {
        let fresh24 = (*f).rpos;
        (*f).rpos = (*f).rpos.offset(1);
        *fresh24 as ::core::ffi::c_int
    } else {
        ffshgetc(f)
    };
    if c == '+' as i32 || c == '-' as i32 {
        neg = (c == '-' as i32) as ::core::ffi::c_int;
        c = if (*f).rpos < (*f).shend {
            let fresh25 = (*f).rpos;
            (*f).rpos = (*f).rpos.offset(1);
            *fresh25 as ::core::ffi::c_int
        } else {
            ffshgetc(f)
        };
        if (c - '0' as i32) as ::core::ffi::c_uint >= 10 as ::core::ffi::c_uint
            && pok != 0
        {
            if !(*f).shend.is_null() {
                (*f).rpos = (*f).rpos.offset(-1);
            } else {};
        }
    }
    if (c - '0' as i32) as ::core::ffi::c_uint >= 10 as ::core::ffi::c_uint {
        if !(*f).shend.is_null() {
            (*f).rpos = (*f).rpos.offset(-1);
        } else {};
        return -(9223372036854775807 as ::core::ffi::c_longlong)
            - 1 as ::core::ffi::c_longlong;
    }
    x = 0 as ::core::ffi::c_int;
    while ((c - '0' as i32) as ::core::ffi::c_uint) < 10 as ::core::ffi::c_uint
        && x < 2147483647 as ::core::ffi::c_int / 10 as ::core::ffi::c_int
    {
        x = 10 as ::core::ffi::c_int * x + (c - '0' as i32);
        c = if (*f).rpos < (*f).shend {
            let fresh26 = (*f).rpos;
            (*f).rpos = (*f).rpos.offset(1);
            *fresh26 as ::core::ffi::c_int
        } else {
            ffshgetc(f)
        };
    }
    y = x as ::core::ffi::c_longlong;
    while ((c - '0' as i32) as ::core::ffi::c_uint) < 10 as ::core::ffi::c_uint
        && y
            < 9223372036854775807 as ::core::ffi::c_longlong
                / 100 as ::core::ffi::c_longlong
    {
        y = 10 as ::core::ffi::c_longlong * y
            + (c - '0' as i32) as ::core::ffi::c_longlong;
        c = if (*f).rpos < (*f).shend {
            let fresh27 = (*f).rpos;
            (*f).rpos = (*f).rpos.offset(1);
            *fresh27 as ::core::ffi::c_int
        } else {
            ffshgetc(f)
        };
    }
    while ((c - '0' as i32) as ::core::ffi::c_uint) < 10 as ::core::ffi::c_uint {
        c = if (*f).rpos < (*f).shend {
            let fresh28 = (*f).rpos;
            (*f).rpos = (*f).rpos.offset(1);
            *fresh28 as ::core::ffi::c_int
        } else {
            ffshgetc(f)
        };
    }
    if !(*f).shend.is_null() {
        (*f).rpos = (*f).rpos.offset(-1);
    } else {};
    return if neg != 0 { -y } else { y };
}
#[inline]
unsafe extern "C" fn av_isspace(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (c == ' ' as i32 || c == '\u{c}' as i32 || c == '\n' as i32
        || c == '\r' as i32 || c == '\t' as i32 || c == '\u{b}' as i32)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn decfloat(
    mut f: *mut FFFILE,
    mut c: ::core::ffi::c_int,
    mut bits: ::core::ffi::c_int,
    mut emin: ::core::ffi::c_int,
    mut sign: ::core::ffi::c_int,
    mut pok: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut x: [uint32_t; 128] = [0; 128];
    static mut th: [uint32_t; 2] = [
        9007199 as ::core::ffi::c_int as uint32_t,
        254740991 as ::core::ffi::c_int as uint32_t,
    ];
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut a: ::core::ffi::c_int = 0;
    let mut z: ::core::ffi::c_int = 0;
    let mut lrp: ::core::ffi::c_longlong = 0 as ::core::ffi::c_longlong;
    let mut dc: ::core::ffi::c_longlong = 0 as ::core::ffi::c_longlong;
    let mut e10: ::core::ffi::c_longlong = 0 as ::core::ffi::c_longlong;
    let mut lnz: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut gotdig: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut gotrad: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rp: ::core::ffi::c_int = 0;
    let mut e2: ::core::ffi::c_int = 0;
    let mut emax: ::core::ffi::c_int = -emin - bits + 3 as ::core::ffi::c_int;
    let mut denormal: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_double = 0.;
    let mut frac: ::core::ffi::c_double = 0 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    let mut bias: ::core::ffi::c_double = 0 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    static mut p10s: [::core::ffi::c_int; 8] = [
        10 as ::core::ffi::c_int,
        100 as ::core::ffi::c_int,
        1000 as ::core::ffi::c_int,
        10000 as ::core::ffi::c_int,
        100000 as ::core::ffi::c_int,
        1000000 as ::core::ffi::c_int,
        10000000 as ::core::ffi::c_int,
        100000000 as ::core::ffi::c_int,
    ];
    j = 0 as ::core::ffi::c_int;
    k = 0 as ::core::ffi::c_int;
    while c == '0' as i32 {
        gotdig = 1 as ::core::ffi::c_int;
        c = if (*f).rpos < (*f).shend {
            let fresh19 = (*f).rpos;
            (*f).rpos = (*f).rpos.offset(1);
            *fresh19 as ::core::ffi::c_int
        } else {
            ffshgetc(f)
        };
    }
    if c == '.' as i32 {
        gotrad = 1 as ::core::ffi::c_int;
        c = if (*f).rpos < (*f).shend {
            let fresh20 = (*f).rpos;
            (*f).rpos = (*f).rpos.offset(1);
            *fresh20 as ::core::ffi::c_int
        } else {
            ffshgetc(f)
        };
        while c == '0' as i32 {
            gotdig = 1 as ::core::ffi::c_int;
            lrp -= 1;
            c = if (*f).rpos < (*f).shend {
                let fresh21 = (*f).rpos;
                (*f).rpos = (*f).rpos.offset(1);
                *fresh21 as ::core::ffi::c_int
            } else {
                ffshgetc(f)
            };
        }
    }
    x[0 as ::core::ffi::c_int as usize] = 0 as uint32_t;
    while ((c - '0' as i32) as ::core::ffi::c_uint) < 10 as ::core::ffi::c_uint
        || c == '.' as i32
    {
        if c == '.' as i32 {
            if gotrad != 0 {
                break;
            }
            gotrad = 1 as ::core::ffi::c_int;
            lrp = dc;
        } else if k < 128 as ::core::ffi::c_int - 3 as ::core::ffi::c_int {
            dc += 1;
            if c != '0' as i32 {
                lnz = dc as ::core::ffi::c_int;
            }
            if j != 0 {
                x[k as usize] = x[k as usize]
                    .wrapping_mul(10 as uint32_t)
                    .wrapping_add(c as uint32_t)
                    .wrapping_sub('0' as i32 as uint32_t);
            } else {
                x[k as usize] = (c - '0' as i32) as uint32_t;
            }
            j += 1;
            if j == 9 as ::core::ffi::c_int {
                k += 1;
                j = 0 as ::core::ffi::c_int;
            }
            gotdig = 1 as ::core::ffi::c_int;
        } else {
            dc += 1;
            if c != '0' as i32 {
                lnz = (128 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                    * 9 as ::core::ffi::c_int;
                x[(128 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as usize]
                    |= 1 as uint32_t;
            }
        }
        c = if (*f).rpos < (*f).shend {
            let fresh22 = (*f).rpos;
            (*f).rpos = (*f).rpos.offset(1);
            *fresh22 as ::core::ffi::c_int
        } else {
            ffshgetc(f)
        };
    }
    if gotrad == 0 {
        lrp = dc;
    }
    if gotdig != 0 && c | 32 as ::core::ffi::c_int == 'e' as i32 {
        e10 = scanexp(f, pok);
        if e10
            == -(9223372036854775807 as ::core::ffi::c_longlong)
                - 1 as ::core::ffi::c_longlong
        {
            if pok != 0 {
                if !(*f).shend.is_null() {
                    (*f).rpos = (*f).rpos.offset(-1);
                } else {};
            } else {
                ffshlim(f, 0 as ptrdiff_t);
                return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
            e10 = 0 as ::core::ffi::c_longlong;
        }
        lrp += e10;
    } else if c >= 0 as ::core::ffi::c_int {
        if !(*f).shend.is_null() {
            (*f).rpos = (*f).rpos.offset(-1);
        } else {};
    }
    if gotdig == 0 {
        *__errno_location() = 22 as ::core::ffi::c_int;
        ffshlim(f, 0 as ptrdiff_t);
        return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    if x[0 as ::core::ffi::c_int as usize] == 0 {
        return sign as ::core::ffi::c_double * 0.0f64;
    }
    if lrp == dc && dc < 10 as ::core::ffi::c_longlong
        && (bits > 30 as ::core::ffi::c_int
            || x[0 as ::core::ffi::c_int as usize] >> bits == 0 as uint32_t)
    {
        return sign as ::core::ffi::c_double
            * x[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_double;
    }
    if lrp > (-emin / 2 as ::core::ffi::c_int) as ::core::ffi::c_longlong {
        *__errno_location() = 34 as ::core::ffi::c_int;
        return sign as ::core::ffi::c_double * 1.7976931348623157e+308f64
            * 1.7976931348623157e+308f64;
    }
    if lrp
        < (emin - 2 as ::core::ffi::c_int * 53 as ::core::ffi::c_int)
            as ::core::ffi::c_longlong
    {
        *__errno_location() = 34 as ::core::ffi::c_int;
        return sign as ::core::ffi::c_double * 2.2250738585072014e-308f64
            * 2.2250738585072014e-308f64;
    }
    if j != 0 {
        while j < 9 as ::core::ffi::c_int {
            x[k as usize] = x[k as usize].wrapping_mul(10 as uint32_t);
            j += 1;
        }
        k += 1;
        j = 0 as ::core::ffi::c_int;
    }
    a = 0 as ::core::ffi::c_int;
    z = k;
    e2 = 0 as ::core::ffi::c_int;
    rp = lrp as ::core::ffi::c_int;
    if lnz < 9 as ::core::ffi::c_int && lnz <= rp && rp < 18 as ::core::ffi::c_int {
        let mut bitlim: ::core::ffi::c_int = 0;
        if rp == 9 as ::core::ffi::c_int {
            return sign as ::core::ffi::c_double
                * x[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_double;
        }
        if rp < 9 as ::core::ffi::c_int {
            return sign as ::core::ffi::c_double
                * x[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                / p10s[(8 as ::core::ffi::c_int - rp) as usize] as ::core::ffi::c_double;
        }
        bitlim = bits - 3 as ::core::ffi::c_int * (rp - 9 as ::core::ffi::c_int);
        if bitlim > 30 as ::core::ffi::c_int
            || x[0 as ::core::ffi::c_int as usize] >> bitlim == 0 as uint32_t
        {
            return sign as ::core::ffi::c_double
                * x[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                * p10s[(rp - 10 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_double;
        }
    }
    while x[(z - 1 as ::core::ffi::c_int) as usize] == 0 {
        z -= 1;
    }
    if rp % 9 as ::core::ffi::c_int != 0 {
        let mut rpm9: ::core::ffi::c_int = if rp >= 0 as ::core::ffi::c_int {
            rp % 9 as ::core::ffi::c_int
        } else {
            rp % 9 as ::core::ffi::c_int + 9 as ::core::ffi::c_int
        };
        let mut p10: ::core::ffi::c_int = p10s[(8 as ::core::ffi::c_int - rpm9)
            as usize];
        let mut carry: uint32_t = 0 as uint32_t;
        k = a;
        while k != z {
            let mut tmp: uint32_t = x[k as usize].wrapping_rem(p10 as uint32_t);
            x[k as usize] = x[k as usize]
                .wrapping_div(p10 as uint32_t)
                .wrapping_add(carry);
            carry = ((1000000000 as ::core::ffi::c_int / p10) as uint32_t)
                .wrapping_mul(tmp);
            if k == a && x[k as usize] == 0 {
                a = a + 1 as ::core::ffi::c_int
                    & 128 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
                rp -= 9 as ::core::ffi::c_int;
            }
            k += 1;
        }
        if carry != 0 {
            let fresh23 = z;
            z = z + 1;
            x[fresh23 as usize] = carry;
        }
        rp += 9 as ::core::ffi::c_int - rpm9;
    }
    while rp < 9 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
        || rp == 9 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            && x[a as usize] < th[0 as ::core::ffi::c_int as usize]
    {
        let mut carry_0: uint32_t = 0 as uint32_t;
        e2 -= 29 as ::core::ffi::c_int;
        k = z - 1 as ::core::ffi::c_int
            & 128 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
        loop {
            let mut tmp_0: uint64_t = ((x[k as usize] as uint64_t)
                << 29 as ::core::ffi::c_int)
                .wrapping_add(carry_0 as uint64_t);
            if tmp_0 > 1000000000 as uint64_t {
                carry_0 = tmp_0.wrapping_div(1000000000 as uint64_t) as uint32_t;
                x[k as usize] = tmp_0.wrapping_rem(1000000000 as uint64_t) as uint32_t;
            } else {
                carry_0 = 0 as uint32_t;
                x[k as usize] = tmp_0 as uint32_t;
            }
            if k
                == z - 1 as ::core::ffi::c_int
                    & 128 as ::core::ffi::c_int - 1 as ::core::ffi::c_int && k != a
                && x[k as usize] == 0
            {
                z = k;
            }
            if k == a {
                break;
            }
            k = k - 1 as ::core::ffi::c_int
                & 128 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
        }
        if carry_0 != 0 {
            rp += 9 as ::core::ffi::c_int;
            a = a - 1 as ::core::ffi::c_int
                & 128 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
            if a == z {
                z = z - 1 as ::core::ffi::c_int
                    & 128 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
                x[(z - 1 as ::core::ffi::c_int
                    & 128 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
                    |= x[z as usize];
            }
            x[a as usize] = carry_0;
        }
    }
    loop {
        let mut carry_1: uint32_t = 0 as uint32_t;
        let mut sh: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < 2 as ::core::ffi::c_int {
            k = a + i & 128 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
            if k == z || x[k as usize] < th[i as usize] {
                i = 2 as ::core::ffi::c_int;
                break;
            } else {
                if x[(a + i & 128 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    as usize] > th[i as usize]
                {
                    break;
                }
                i += 1;
            }
        }
        if i == 2 as ::core::ffi::c_int
            && rp == 9 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
        {
            break;
        }
        if rp
            > 9 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
        {
            sh = 9 as ::core::ffi::c_int;
        }
        e2 += sh;
        k = a;
        while k != z {
            let mut tmp_1: uint32_t = x[k as usize]
                & (((1 as ::core::ffi::c_int) << sh) - 1 as ::core::ffi::c_int)
                    as uint32_t;
            x[k as usize] = (x[k as usize] >> sh).wrapping_add(carry_1);
            carry_1 = ((1000000000 as ::core::ffi::c_int >> sh) as uint32_t)
                .wrapping_mul(tmp_1);
            if k == a && x[k as usize] == 0 {
                a = a + 1 as ::core::ffi::c_int
                    & 128 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
                i -= 1;
                rp -= 9 as ::core::ffi::c_int;
            }
            k = k + 1 as ::core::ffi::c_int
                & 128 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
        }
        if carry_1 != 0 {
            if z + 1 as ::core::ffi::c_int
                & 128 as ::core::ffi::c_int - 1 as ::core::ffi::c_int != a
            {
                x[z as usize] = carry_1;
                z = z + 1 as ::core::ffi::c_int
                    & 128 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
            } else {
                x[(z - 1 as ::core::ffi::c_int
                    & 128 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
                    |= 1 as uint32_t;
            }
        }
    }
    i = 0 as ::core::ffi::c_int;
    y = i as ::core::ffi::c_double;
    while i < 2 as ::core::ffi::c_int {
        if a + i & 128 as ::core::ffi::c_int - 1 as ::core::ffi::c_int == z {
            z = z + 1 as ::core::ffi::c_int
                & 128 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
            x[(z - 1 as ::core::ffi::c_int) as usize] = 0 as uint32_t;
        }
        y = (crate::rt::F128::new(1000000000.0) * crate::rt::F128::new(y)
            + crate::rt::F128::new(
                x[(a + i & 128 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize],
            ))
            .to_f64()
            .unwrap();
        i += 1;
    }
    y *= sign as ::core::ffi::c_double;
    if bits > 53 as ::core::ffi::c_int + e2 - emin {
        bits = 53 as ::core::ffi::c_int + e2 - emin;
        if bits < 0 as ::core::ffi::c_int {
            bits = 0 as ::core::ffi::c_int;
        }
        denormal = 1 as ::core::ffi::c_int;
    }
    if bits < 53 as ::core::ffi::c_int {
        bias = copysign(
            scalbn(
                1 as ::core::ffi::c_int as ::core::ffi::c_double,
                2 as ::core::ffi::c_int * 53 as ::core::ffi::c_int - bits
                    - 1 as ::core::ffi::c_int,
            ),
            y,
        );
        frac = fmod(
            y,
            scalbn(
                1 as ::core::ffi::c_int as ::core::ffi::c_double,
                53 as ::core::ffi::c_int - bits,
            ),
        );
        y -= frac;
        y += bias;
    }
    if a + i & 128 as ::core::ffi::c_int - 1 as ::core::ffi::c_int != z {
        let mut t: uint32_t = x[(a + i
            & 128 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize];
        if t < 500000000 as ::core::ffi::c_int as uint32_t
            && (t != 0
                || a + i + 1 as ::core::ffi::c_int
                    & 128 as ::core::ffi::c_int - 1 as ::core::ffi::c_int != z)
        {
            frac += 0.25f64 * sign as ::core::ffi::c_double;
        } else if t > 500000000 as ::core::ffi::c_int as uint32_t {
            frac += 0.75f64 * sign as ::core::ffi::c_double;
        } else if t == 500000000 as ::core::ffi::c_int as uint32_t {
            if a + i + 1 as ::core::ffi::c_int
                & 128 as ::core::ffi::c_int - 1 as ::core::ffi::c_int == z
            {
                frac += 0.5f64 * sign as ::core::ffi::c_double;
            } else {
                frac += 0.75f64 * sign as ::core::ffi::c_double;
            }
        }
        if 53 as ::core::ffi::c_int - bits >= 2 as ::core::ffi::c_int
            && fmod(frac, 1 as ::core::ffi::c_int as ::core::ffi::c_double) == 0.
        {
            frac += 1.;
        }
    }
    y += frac;
    y -= bias;
    if e2 + 53 as ::core::ffi::c_int & 2147483647 as ::core::ffi::c_int
        > emax - 5 as ::core::ffi::c_int
    {
        if fabs(y)
            >= pow(
                2 as ::core::ffi::c_int as ::core::ffi::c_double,
                53 as ::core::ffi::c_int as ::core::ffi::c_double,
            )
        {
            if denormal != 0 && bits == 53 as ::core::ffi::c_int + e2 - emin {
                denormal = 0 as ::core::ffi::c_int;
            }
            y *= 0.5f64;
            e2 += 1;
        }
        if e2 + 53 as ::core::ffi::c_int > emax || denormal != 0 && frac != 0. {
            *__errno_location() = 34 as ::core::ffi::c_int;
        }
    }
    return scalbn(y, e2);
}
unsafe extern "C" fn hexfloat(
    mut f: *mut FFFILE,
    mut bits: ::core::ffi::c_int,
    mut emin: ::core::ffi::c_int,
    mut sign: ::core::ffi::c_int,
    mut pok: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut x: uint32_t = 0 as uint32_t;
    let mut y: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut scale: ::core::ffi::c_double = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    let mut bias: ::core::ffi::c_double = 0 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    let mut gottail: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut gotrad: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut gotdig: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rp: ::core::ffi::c_longlong = 0 as ::core::ffi::c_longlong;
    let mut dc: ::core::ffi::c_longlong = 0 as ::core::ffi::c_longlong;
    let mut e2: ::core::ffi::c_longlong = 0 as ::core::ffi::c_longlong;
    let mut d: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    c = if (*f).rpos < (*f).shend {
        let fresh29 = (*f).rpos;
        (*f).rpos = (*f).rpos.offset(1);
        *fresh29 as ::core::ffi::c_int
    } else {
        ffshgetc(f)
    };
    while c == '0' as i32 {
        gotdig = 1 as ::core::ffi::c_int;
        c = if (*f).rpos < (*f).shend {
            let fresh30 = (*f).rpos;
            (*f).rpos = (*f).rpos.offset(1);
            *fresh30 as ::core::ffi::c_int
        } else {
            ffshgetc(f)
        };
    }
    if c == '.' as i32 {
        gotrad = 1 as ::core::ffi::c_int;
        c = if (*f).rpos < (*f).shend {
            let fresh31 = (*f).rpos;
            (*f).rpos = (*f).rpos.offset(1);
            *fresh31 as ::core::ffi::c_int
        } else {
            ffshgetc(f)
        };
        rp = 0 as ::core::ffi::c_longlong;
        while c == '0' as i32 {
            gotdig = 1 as ::core::ffi::c_int;
            c = (if (*f).rpos < (*f).shend {
                let fresh32 = (*f).rpos;
                (*f).rpos = (*f).rpos.offset(1);
                *fresh32 as ::core::ffi::c_int
            } else {
                ffshgetc(f)
            });
            rp -= 1;
        }
    }
    while ((c - '0' as i32) as ::core::ffi::c_uint) < 10 as ::core::ffi::c_uint
        || (((c | 32 as ::core::ffi::c_int) - 'a' as i32) as ::core::ffi::c_uint)
            < 6 as ::core::ffi::c_uint || c == '.' as i32
    {
        if c == '.' as i32 {
            if gotrad != 0 {
                break;
            }
            rp = dc;
            gotrad = 1 as ::core::ffi::c_int;
        } else {
            gotdig = 1 as ::core::ffi::c_int;
            if c > '9' as i32 {
                d = (c | 32 as ::core::ffi::c_int) + 10 as ::core::ffi::c_int
                    - 'a' as i32;
            } else {
                d = c - '0' as i32;
            }
            if dc < 8 as ::core::ffi::c_longlong {
                x = x.wrapping_mul(16 as uint32_t).wrapping_add(d as uint32_t);
            } else if dc
                < (53 as ::core::ffi::c_int / 4 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as ::core::ffi::c_longlong
            {
                scale /= 16 as ::core::ffi::c_int as ::core::ffi::c_double;
                y += d as ::core::ffi::c_double * scale;
            } else if d != 0 && gottail == 0 {
                y += 0.5f64 * scale;
                gottail = 1 as ::core::ffi::c_int;
            }
            dc += 1;
        }
        c = if (*f).rpos < (*f).shend {
            let fresh33 = (*f).rpos;
            (*f).rpos = (*f).rpos.offset(1);
            *fresh33 as ::core::ffi::c_int
        } else {
            ffshgetc(f)
        };
    }
    if gotdig == 0 {
        if !(*f).shend.is_null() {
            (*f).rpos = (*f).rpos.offset(-1);
        } else {};
        if pok != 0 {
            if !(*f).shend.is_null() {
                (*f).rpos = (*f).rpos.offset(-1);
            } else {};
            if gotrad != 0 {
                if !(*f).shend.is_null() {
                    (*f).rpos = (*f).rpos.offset(-1);
                } else {};
            }
        } else {
            ffshlim(f, 0 as ptrdiff_t);
        }
        return sign as ::core::ffi::c_double * 0.0f64;
    }
    if gotrad == 0 {
        rp = dc;
    }
    while dc < 8 as ::core::ffi::c_longlong {
        x = x.wrapping_mul(16 as uint32_t);
        dc += 1;
    }
    if c | 32 as ::core::ffi::c_int == 'p' as i32 {
        e2 = scanexp(f, pok);
        if e2
            == -(9223372036854775807 as ::core::ffi::c_longlong)
                - 1 as ::core::ffi::c_longlong
        {
            if pok != 0 {
                if !(*f).shend.is_null() {
                    (*f).rpos = (*f).rpos.offset(-1);
                } else {};
            } else {
                ffshlim(f, 0 as ptrdiff_t);
                return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
            e2 = 0 as ::core::ffi::c_longlong;
        }
    } else {
        if !(*f).shend.is_null() {
            (*f).rpos = (*f).rpos.offset(-1);
        } else {};
    }
    e2 += 4 as ::core::ffi::c_longlong * rp - 32 as ::core::ffi::c_longlong;
    if x == 0 {
        return sign as ::core::ffi::c_double * 0.0f64;
    }
    if e2 > -emin as ::core::ffi::c_longlong {
        *__errno_location() = 34 as ::core::ffi::c_int;
        return sign as ::core::ffi::c_double * 1.7976931348623157e+308f64
            * 1.7976931348623157e+308f64;
    }
    if e2
        < (emin - 2 as ::core::ffi::c_int * 53 as ::core::ffi::c_int)
            as ::core::ffi::c_longlong
    {
        *__errno_location() = 34 as ::core::ffi::c_int;
        return sign as ::core::ffi::c_double * 2.2250738585072014e-308f64
            * 2.2250738585072014e-308f64;
    }
    while x < 0x80000000 as ::core::ffi::c_uint {
        if y >= 0.5f64 {
            x = x.wrapping_add(x.wrapping_add(1 as uint32_t));
            y += y - 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        } else {
            x = x.wrapping_add(x);
            y += y;
        }
        e2 -= 1;
    }
    if bits as ::core::ffi::c_longlong
        > 32 as ::core::ffi::c_longlong + e2 - emin as ::core::ffi::c_longlong
    {
        bits = (32 as ::core::ffi::c_longlong + e2 - emin as ::core::ffi::c_longlong)
            as ::core::ffi::c_int;
        if bits < 0 as ::core::ffi::c_int {
            bits = 0 as ::core::ffi::c_int;
        }
    }
    if bits < 53 as ::core::ffi::c_int {
        bias = copysign(
            scalbn(
                1 as ::core::ffi::c_int as ::core::ffi::c_double,
                32 as ::core::ffi::c_int + 53 as ::core::ffi::c_int - bits
                    - 1 as ::core::ffi::c_int,
            ),
            sign as ::core::ffi::c_double,
        );
    }
    if bits < 32 as ::core::ffi::c_int && y != 0. && x & 1 as uint32_t == 0 {
        x = x.wrapping_add(1);
        y = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    y = bias + sign as ::core::ffi::c_double * x as ::core::ffi::c_double
        + sign as ::core::ffi::c_double * y;
    y -= bias;
    if y == 0. {
        *__errno_location() = 34 as ::core::ffi::c_int;
    }
    return scalbn(y, e2 as ::core::ffi::c_int);
}
unsafe extern "C" fn fffloatscan(
    mut f: *mut FFFILE,
    mut prec: ::core::ffi::c_int,
    mut pok: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut sign: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut i: size_t = 0;
    let mut bits: ::core::ffi::c_int = 0;
    let mut emin: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    match prec {
        0 => {
            bits = 24 as ::core::ffi::c_int;
            emin = -(125 as ::core::ffi::c_int) - bits;
        }
        1 => {
            bits = 53 as ::core::ffi::c_int;
            emin = -(1021 as ::core::ffi::c_int) - bits;
        }
        2 => {
            bits = 53 as ::core::ffi::c_int;
            emin = -(1021 as ::core::ffi::c_int) - bits;
        }
        _ => return 0 as ::core::ffi::c_int as ::core::ffi::c_double,
    }
    loop {
        c = if (*f).rpos < (*f).shend {
            let fresh11 = (*f).rpos;
            (*f).rpos = (*f).rpos.offset(1);
            *fresh11 as ::core::ffi::c_int
        } else {
            ffshgetc(f)
        };
        if !(av_isspace(c) != 0) {
            break;
        }
    }
    if c == '+' as i32 || c == '-' as i32 {
        sign -= 2 as ::core::ffi::c_int * (c == '-' as i32) as ::core::ffi::c_int;
        c = if (*f).rpos < (*f).shend {
            let fresh12 = (*f).rpos;
            (*f).rpos = (*f).rpos.offset(1);
            *fresh12 as ::core::ffi::c_int
        } else {
            ffshgetc(f)
        };
    }
    i = 0 as size_t;
    while i < 8 as size_t
        && c | 32 as ::core::ffi::c_int
            == ::core::mem::transmute::<
                [u8; 9],
                [::core::ffi::c_char; 9],
            >(*b"infinity\0")[i as usize] as ::core::ffi::c_int
    {
        if i < 7 as size_t {
            c = if (*f).rpos < (*f).shend {
                let fresh13 = (*f).rpos;
                (*f).rpos = (*f).rpos.offset(1);
                *fresh13 as ::core::ffi::c_int
            } else {
                ffshgetc(f)
            };
        }
        i = i.wrapping_add(1);
    }
    if i == 3 as size_t || i == 8 as size_t || i > 3 as size_t && pok != 0 {
        if i != 8 as size_t {
            if !(*f).shend.is_null() {
                (*f).rpos = (*f).rpos.offset(-1);
            } else {};
            if pok != 0 {
                while i > 3 as size_t {
                    if !(*f).shend.is_null() {
                        (*f).rpos = (*f).rpos.offset(-1);
                    } else {};
                    i = i.wrapping_sub(1);
                }
            }
        }
        return (sign as ::core::ffi::c_float * ::core::f32::INFINITY)
            as ::core::ffi::c_double;
    }
    if i == 0 {
        i = 0 as size_t;
        while i < 3 as size_t
            && c | 32 as ::core::ffi::c_int
                == ::core::mem::transmute::<
                    [u8; 4],
                    [::core::ffi::c_char; 4],
                >(*b"nan\0")[i as usize] as ::core::ffi::c_int
        {
            if i < 2 as size_t {
                c = if (*f).rpos < (*f).shend {
                    let fresh14 = (*f).rpos;
                    (*f).rpos = (*f).rpos.offset(1);
                    *fresh14 as ::core::ffi::c_int
                } else {
                    ffshgetc(f)
                };
            }
            i = i.wrapping_add(1);
        }
    }
    if i == 3 as size_t {
        if (if (*f).rpos < (*f).shend {
            let fresh15 = (*f).rpos;
            (*f).rpos = (*f).rpos.offset(1);
            *fresh15 as ::core::ffi::c_int
        } else {
            ffshgetc(f)
        }) != '(' as i32
        {
            if !(*f).shend.is_null() {
                (*f).rpos = (*f).rpos.offset(-1);
            } else {};
            return ::core::f32::NAN as ::core::ffi::c_double;
        }
        i = 1 as size_t;
        loop {
            c = if (*f).rpos < (*f).shend {
                let fresh16 = (*f).rpos;
                (*f).rpos = (*f).rpos.offset(1);
                *fresh16 as ::core::ffi::c_int
            } else {
                ffshgetc(f)
            };
            if ((c - '0' as i32) as ::core::ffi::c_uint) < 10 as ::core::ffi::c_uint
                || ((c - 'A' as i32) as ::core::ffi::c_uint) < 26 as ::core::ffi::c_uint
                || ((c - 'a' as i32) as ::core::ffi::c_uint) < 26 as ::core::ffi::c_uint
                || c == '_' as i32
            {
                i = i.wrapping_add(1);
            } else {
                if c == ')' as i32 {
                    return ::core::f32::NAN as ::core::ffi::c_double;
                }
                if !(*f).shend.is_null() {
                    (*f).rpos = (*f).rpos.offset(-1);
                } else {};
                if pok == 0 {
                    *__errno_location() = 22 as ::core::ffi::c_int;
                    ffshlim(f, 0 as ptrdiff_t);
                    return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                }
                loop {
                    let fresh17 = i;
                    i = i.wrapping_sub(1);
                    if !(fresh17 != 0) {
                        break;
                    }
                    if !(*f).shend.is_null() {
                        (*f).rpos = (*f).rpos.offset(-1);
                    } else {};
                }
                return ::core::f32::NAN as ::core::ffi::c_double;
            }
        }
    }
    if i != 0 {
        if !(*f).shend.is_null() {
            (*f).rpos = (*f).rpos.offset(-1);
        } else {};
        *__errno_location() = 22 as ::core::ffi::c_int;
        ffshlim(f, 0 as ptrdiff_t);
        return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    if c == '0' as i32 {
        c = if (*f).rpos < (*f).shend {
            let fresh18 = (*f).rpos;
            (*f).rpos = (*f).rpos.offset(1);
            *fresh18 as ::core::ffi::c_int
        } else {
            ffshgetc(f)
        };
        if c | 32 as ::core::ffi::c_int == 'x' as i32 {
            return hexfloat(f, bits, emin, sign, pok);
        }
        if !(*f).shend.is_null() {
            (*f).rpos = (*f).rpos.offset(-1);
        } else {};
        c = '0' as i32;
    }
    return decfloat(f, c, bits, emin, sign, pok);
}
unsafe extern "C" fn arg_n(
    mut ap: crate::rt::VaList,
    mut n: ::core::ffi::c_uint,
) -> *mut ::core::ffi::c_void {
    let mut p: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut i: ::core::ffi::c_uint = 0;
    let mut ap2: crate::rt::VaList;
    ap2 = ap.clone();
    i = n;
    while i > 1 as ::core::ffi::c_uint {
        ap2.arg::<*mut ::core::ffi::c_void>();
        i = i.wrapping_sub(1);
    }
    p = ap2.arg::<*mut ::core::ffi::c_void>();
    return p;
}
unsafe extern "C" fn store_int(
    mut dest: *mut ::core::ffi::c_void,
    mut size: ::core::ffi::c_int,
    mut i: ::core::ffi::c_ulonglong,
) {
    if dest.is_null() {
        return;
    }
    match size {
        -2 => {
            *(dest as *mut ::core::ffi::c_char) = i as ::core::ffi::c_char;
        }
        -1 => {
            *(dest as *mut ::core::ffi::c_short) = i as ::core::ffi::c_short;
        }
        0 => {
            *(dest as *mut ::core::ffi::c_int) = i as ::core::ffi::c_int;
        }
        1 => {
            *(dest as *mut ::core::ffi::c_long) = i as ::core::ffi::c_long;
        }
        3 => {
            *(dest as *mut ::core::ffi::c_longlong) = i as ::core::ffi::c_longlong;
        }
        _ => {}
    };
}
unsafe extern "C" fn ff_vfscanf(
    mut f: *mut FFFILE,
    mut fmt: *const ::core::ffi::c_char,
    mut ap: crate::rt::VaList,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut width: ::core::ffi::c_int = 0;
    let mut size: ::core::ffi::c_int = 0;
    let mut base: ::core::ffi::c_int = 0;
    let mut p: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut c: ::core::ffi::c_int = 0;
    let mut t: ::core::ffi::c_int = 0;
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut dest: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut invert: ::core::ffi::c_int = 0;
    let mut matches: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut x: ::core::ffi::c_ulonglong = 0;
    let mut y: ::core::ffi::c_double = 0.;
    let mut pos: ptrdiff_t = 0 as ptrdiff_t;
    let mut scanset: [::core::ffi::c_uchar; 257] = [0; 257];
    let mut i: size_t = 0;
    p = fmt as *const ::core::ffi::c_uchar;
    's_30: loop {
        if !(*p != 0) {
            current_block = 7142652332649249282;
            break;
        }
        if av_isspace(*p as ::core::ffi::c_int) != 0 {
            while av_isspace(
                *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            ) != 0
            {
                p = p.offset(1);
            }
            ffshlim(f, 0 as ptrdiff_t);
            while av_isspace(
                if (*f).rpos < (*f).shend {
                    let fresh0 = (*f).rpos;
                    (*f).rpos = (*f).rpos.offset(1);
                    *fresh0 as ::core::ffi::c_int
                } else {
                    ffshgetc(f)
                },
            ) != 0
            {}
            if !(*f).shend.is_null() {
                (*f).rpos = (*f).rpos.offset(-1);
            } else {};
            pos
                += (*f).shcnt as ::core::ffi::c_long
                    + (*f).rpos.offset_from((*f).buf) as ::core::ffi::c_long;
        } else if *p as ::core::ffi::c_int != '%' as i32
            || *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '%' as i32
        {
            ffshlim(f, 0 as ptrdiff_t);
            if *p as ::core::ffi::c_int == '%' as i32 {
                p = p.offset(1);
                loop {
                    c = if (*f).rpos < (*f).shend {
                        let fresh1 = (*f).rpos;
                        (*f).rpos = (*f).rpos.offset(1);
                        *fresh1 as ::core::ffi::c_int
                    } else {
                        ffshgetc(f)
                    };
                    if !(av_isspace(c) != 0) {
                        break;
                    }
                }
            } else {
                c = if (*f).rpos < (*f).shend {
                    let fresh2 = (*f).rpos;
                    (*f).rpos = (*f).rpos.offset(1);
                    *fresh2 as ::core::ffi::c_int
                } else {
                    ffshgetc(f)
                };
            }
            if c != *p as ::core::ffi::c_int {
                if !(*f).shend.is_null() {
                    (*f).rpos = (*f).rpos.offset(-1);
                } else {};
                if c < 0 as ::core::ffi::c_int {
                    current_block = 12294723886771465321;
                    break;
                } else {
                    current_block = 7142652332649249282;
                    break;
                }
            } else {
                pos
                    += (*f).shcnt as ::core::ffi::c_long
                        + (*f).rpos.offset_from((*f).buf) as ::core::ffi::c_long;
            }
        } else {
            p = p.offset(1);
            if *p as ::core::ffi::c_int == '*' as i32 {
                dest = ::core::ptr::null_mut::<::core::ffi::c_void>();
                p = p.offset(1);
            } else if av_isdigit(*p as ::core::ffi::c_int) != 0
                && *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '$' as i32
            {
                dest = arg_n(
                    ap.as_va_list(),
                    (*p as ::core::ffi::c_int - '0' as i32) as ::core::ffi::c_uint,
                );
                p = p.offset(2 as ::core::ffi::c_int as isize);
            } else {
                dest = ap.arg::<*mut ::core::ffi::c_void>();
            }
            width = 0 as ::core::ffi::c_int;
            while av_isdigit(*p as ::core::ffi::c_int) != 0 {
                width = 10 as ::core::ffi::c_int * width + *p as ::core::ffi::c_int
                    - '0' as i32;
                p = p.offset(1);
            }
            if *p as ::core::ffi::c_int == 'm' as i32 {
                s = ::core::ptr::null_mut::<::core::ffi::c_char>();
                p = p.offset(1);
            }
            size = 0 as ::core::ffi::c_int;
            let fresh3 = p;
            p = p.offset(1);
            match *fresh3 as ::core::ffi::c_int {
                104 => {
                    if *p as ::core::ffi::c_int == 'h' as i32 {
                        p = p.offset(1);
                        size = -(2 as ::core::ffi::c_int);
                    } else {
                        size = -(1 as ::core::ffi::c_int);
                    }
                }
                108 => {
                    if *p as ::core::ffi::c_int == 'l' as i32 {
                        p = p.offset(1);
                        size = 3 as ::core::ffi::c_int;
                    } else {
                        size = 1 as ::core::ffi::c_int;
                    }
                }
                106 => {
                    size = 3 as ::core::ffi::c_int;
                }
                122 | 116 => {
                    size = 1 as ::core::ffi::c_int;
                }
                76 => {
                    size = 2 as ::core::ffi::c_int;
                }
                100 | 105 | 111 | 117 | 120 | 97 | 101 | 102 | 103 | 65 | 69 | 70 | 71
                | 88 | 115 | 99 | 91 | 83 | 67 | 112 | 110 => {
                    p = p.offset(-1);
                }
                _ => {
                    current_block = 12294723886771465321;
                    break;
                }
            }
            t = *p as ::core::ffi::c_int;
            if t & 0x2f as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
                t |= 32 as ::core::ffi::c_int;
                size = 1 as ::core::ffi::c_int;
            }
            match t {
                99 => {
                    if width < 1 as ::core::ffi::c_int {
                        width = 1 as ::core::ffi::c_int;
                    }
                    current_block = 17441561948628420366;
                }
                91 => {
                    current_block = 17441561948628420366;
                }
                110 => {
                    store_int(dest, size, pos as ::core::ffi::c_ulonglong);
                    current_block = 15619007995458559411;
                }
                _ => {
                    ffshlim(f, 0 as ptrdiff_t);
                    while av_isspace(
                        if (*f).rpos < (*f).shend {
                            let fresh4 = (*f).rpos;
                            (*f).rpos = (*f).rpos.offset(1);
                            *fresh4 as ::core::ffi::c_int
                        } else {
                            ffshgetc(f)
                        },
                    ) != 0
                    {}
                    if !(*f).shend.is_null() {
                        (*f).rpos = (*f).rpos.offset(-1);
                    } else {};
                    pos
                        += (*f).shcnt as ::core::ffi::c_long
                            + (*f).rpos.offset_from((*f).buf) as ::core::ffi::c_long;
                    current_block = 17441561948628420366;
                }
            }
            match current_block {
                15619007995458559411 => {}
                _ => {
                    ffshlim(f, width as ptrdiff_t);
                    if (if (*f).rpos < (*f).shend {
                        let fresh5 = (*f).rpos;
                        (*f).rpos = (*f).rpos.offset(1);
                        *fresh5 as ::core::ffi::c_int
                    } else {
                        ffshgetc(f)
                    }) < 0 as ::core::ffi::c_int
                    {
                        current_block = 12294723886771465321;
                        break;
                    }
                    if !(*f).shend.is_null() {
                        (*f).rpos = (*f).rpos.offset(-1);
                    } else {};
                    match t {
                        115 | 99 | 91 => {
                            if t == 'c' as i32 || t == 's' as i32 {
                                memset(
                                    &raw mut scanset as *mut ::core::ffi::c_uchar
                                        as *mut ::core::ffi::c_void,
                                    -(1 as ::core::ffi::c_int),
                                    ::core::mem::size_of::<[::core::ffi::c_uchar; 257]>()
                                        as size_t,
                                );
                                scanset[0 as ::core::ffi::c_int as usize] = 0
                                    as ::core::ffi::c_uchar;
                                if t == 's' as i32 {
                                    scanset[(1 as ::core::ffi::c_int + '\t' as i32) as usize] = 0
                                        as ::core::ffi::c_uchar;
                                    scanset[(1 as ::core::ffi::c_int + '\n' as i32) as usize] = 0
                                        as ::core::ffi::c_uchar;
                                    scanset[(1 as ::core::ffi::c_int + '\u{b}' as i32)
                                        as usize] = 0 as ::core::ffi::c_uchar;
                                    scanset[(1 as ::core::ffi::c_int + '\u{c}' as i32)
                                        as usize] = 0 as ::core::ffi::c_uchar;
                                    scanset[(1 as ::core::ffi::c_int + '\r' as i32) as usize] = 0
                                        as ::core::ffi::c_uchar;
                                    scanset[(1 as ::core::ffi::c_int + ' ' as i32) as usize] = 0
                                        as ::core::ffi::c_uchar;
                                }
                            } else {
                                p = p.offset(1);
                                if *p as ::core::ffi::c_int == '^' as i32 {
                                    p = p.offset(1);
                                    invert = 1 as ::core::ffi::c_int;
                                } else {
                                    invert = 0 as ::core::ffi::c_int;
                                }
                                memset(
                                    &raw mut scanset as *mut ::core::ffi::c_uchar
                                        as *mut ::core::ffi::c_void,
                                    invert,
                                    ::core::mem::size_of::<[::core::ffi::c_uchar; 257]>()
                                        as size_t,
                                );
                                scanset[0 as ::core::ffi::c_int as usize] = 0
                                    as ::core::ffi::c_uchar;
                                if *p as ::core::ffi::c_int == '-' as i32 {
                                    p = p.offset(1);
                                    scanset[(1 as ::core::ffi::c_int + '-' as i32) as usize] = (1
                                        as ::core::ffi::c_int - invert) as ::core::ffi::c_uchar;
                                } else if *p as ::core::ffi::c_int == ']' as i32 {
                                    p = p.offset(1);
                                    scanset[(1 as ::core::ffi::c_int + ']' as i32) as usize] = (1
                                        as ::core::ffi::c_int - invert) as ::core::ffi::c_uchar;
                                }
                                while *p as ::core::ffi::c_int != ']' as i32 {
                                    if *p == 0 {
                                        current_block = 12294723886771465321;
                                        break 's_30;
                                    }
                                    if *p as ::core::ffi::c_int == '-' as i32
                                        && *p.offset(1 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int != 0
                                        && *p.offset(1 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int != ']' as i32
                                    {
                                        let fresh6 = p;
                                        p = p.offset(1);
                                        c = *fresh6.offset(-(1 as ::core::ffi::c_int) as isize)
                                            as ::core::ffi::c_int;
                                        while c < *p as ::core::ffi::c_int {
                                            scanset[(1 as ::core::ffi::c_int + c) as usize] = (1
                                                as ::core::ffi::c_int - invert) as ::core::ffi::c_uchar;
                                            c += 1;
                                        }
                                    }
                                    scanset[(1 as ::core::ffi::c_int + *p as ::core::ffi::c_int)
                                        as usize] = (1 as ::core::ffi::c_int - invert)
                                        as ::core::ffi::c_uchar;
                                    p = p.offset(1);
                                }
                            }
                            s = ::core::ptr::null_mut::<::core::ffi::c_char>();
                            i = 0 as size_t;
                            s = dest as *mut ::core::ffi::c_char;
                            if !s.is_null() {
                                loop {
                                    c = (if (*f).rpos < (*f).shend {
                                        let fresh7 = (*f).rpos;
                                        (*f).rpos = (*f).rpos.offset(1);
                                        *fresh7 as ::core::ffi::c_int
                                    } else {
                                        ffshgetc(f)
                                    });
                                    if !(scanset[(c + 1 as ::core::ffi::c_int) as usize] != 0) {
                                        break;
                                    }
                                    let fresh8 = i;
                                    i = i.wrapping_add(1);
                                    *s.offset(fresh8 as isize) = c as ::core::ffi::c_char;
                                }
                            } else {
                                loop {
                                    c = (if (*f).rpos < (*f).shend {
                                        let fresh9 = (*f).rpos;
                                        (*f).rpos = (*f).rpos.offset(1);
                                        *fresh9 as ::core::ffi::c_int
                                    } else {
                                        ffshgetc(f)
                                    });
                                    if !(scanset[(c + 1 as ::core::ffi::c_int) as usize] != 0) {
                                        break;
                                    }
                                }
                            }
                            if !(*f).shend.is_null() {
                                (*f).rpos = (*f).rpos.offset(-1);
                            } else {};
                            if (*f).shcnt as ::core::ffi::c_long
                                + (*f).rpos.offset_from((*f).buf) as ::core::ffi::c_long
                                == 0
                            {
                                current_block = 7142652332649249282;
                                break;
                            }
                            if t == 'c' as i32
                                && (*f).shcnt as ::core::ffi::c_long
                                    + (*f).rpos.offset_from((*f).buf) as ::core::ffi::c_long
                                    != width as ::core::ffi::c_long
                            {
                                current_block = 7142652332649249282;
                                break;
                            }
                            if t != 'c' as i32 {
                                if !s.is_null() {
                                    *s.offset(i as isize) = 0 as ::core::ffi::c_char;
                                }
                            }
                            current_block = 7315983924538012637;
                        }
                        112 | 88 | 120 => {
                            base = 16 as ::core::ffi::c_int;
                            current_block = 449316853341916097;
                        }
                        111 => {
                            base = 8 as ::core::ffi::c_int;
                            current_block = 449316853341916097;
                        }
                        100 | 117 => {
                            base = 10 as ::core::ffi::c_int;
                            current_block = 449316853341916097;
                        }
                        105 => {
                            base = 0 as ::core::ffi::c_int;
                            current_block = 449316853341916097;
                        }
                        97 | 65 | 101 | 69 | 102 | 70 | 103 | 71 => {
                            y = fffloatscan(f, size, 0 as ::core::ffi::c_int);
                            if (*f).shcnt as ::core::ffi::c_long
                                + (*f).rpos.offset_from((*f).buf) as ::core::ffi::c_long
                                == 0
                            {
                                current_block = 7142652332649249282;
                                break;
                            }
                            if !dest.is_null() {
                                match size {
                                    0 => {
                                        *(dest as *mut ::core::ffi::c_float) = y
                                            as ::core::ffi::c_float;
                                    }
                                    1 => {
                                        *(dest as *mut ::core::ffi::c_double) = y;
                                    }
                                    2 => {
                                        *(dest as *mut ::core::ffi::c_double) = y;
                                    }
                                    _ => {}
                                }
                            }
                            current_block = 7315983924538012637;
                        }
                        _ => {
                            current_block = 7315983924538012637;
                        }
                    }
                    match current_block {
                        449316853341916097 => {
                            x = ffintscan(
                                f,
                                base as ::core::ffi::c_uint,
                                0 as ::core::ffi::c_int,
                                (9223372036854775807 as ::core::ffi::c_ulonglong)
                                    .wrapping_mul(2 as ::core::ffi::c_ulonglong)
                                    .wrapping_add(1 as ::core::ffi::c_ulonglong),
                            );
                            if (*f).shcnt as ::core::ffi::c_long
                                + (*f).rpos.offset_from((*f).buf) as ::core::ffi::c_long
                                == 0
                            {
                                current_block = 7142652332649249282;
                                break;
                            }
                            if t == 'p' as i32 && !dest.is_null() {
                                let ref mut fresh10 = *(dest
                                    as *mut *mut ::core::ffi::c_void);
                                *fresh10 = x as uintptr_t as *mut ::core::ffi::c_void;
                            } else {
                                store_int(dest, size, x);
                            }
                        }
                        _ => {}
                    }
                    pos
                        += (*f).shcnt as ::core::ffi::c_long
                            + (*f).rpos.offset_from((*f).buf) as ::core::ffi::c_long;
                    if !dest.is_null() {
                        matches += 1;
                    }
                }
            }
        }
        p = p.offset(1);
    }
    match current_block {
        12294723886771465321 => {
            if matches == 0 {
                matches -= 1;
            }
        }
        _ => {}
    }
    return matches;
}
unsafe extern "C" fn ff_vsscanf(
    mut s: *const ::core::ffi::c_char,
    mut fmt: *const ::core::ffi::c_char,
    mut ap: crate::rt::VaList,
) -> ::core::ffi::c_int {
    let mut f: FFFILE = FFFILE {
        buf_size: 0,
        buf: s as *mut ::core::ffi::c_void as *mut ::core::ffi::c_uchar,
        rpos: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        rend: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        shend: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        shlim: 0,
        shcnt: 0,
        cookie: s as *mut ::core::ffi::c_void,
        read: Some(
            ffstring_read
                as unsafe extern "C" fn(
                    *mut FFFILE,
                    *mut ::core::ffi::c_uchar,
                    size_t,
                ) -> size_t,
        ),
    };
    return ff_vfscanf(&raw mut f, fmt, ap.as_va_list());
}
extern "C" {
    // asm/varargs-aarch64.S: builds the va_list, calls av_sscanf__va
    pub fn av_sscanf(
        string: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_sscanf__va(
    mut string: *const ::core::ffi::c_char,
    mut format: *const ::core::ffi::c_char,
    mut args: crate::rt::VaList,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut ap: crate::rt::VaList;
    ap = args.clone();
    ret = ff_vsscanf(string, format, ap.as_va_list());
    return ret;
}
