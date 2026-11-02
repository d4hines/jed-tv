// Translated from FFmpeg 9.0.1 libavutil/mem.c
// (preprocessed for armv7-unknown-linux-musleabihf) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// default memory allocator for libavutil
// Copyright (c) 2002 Fabrice Bellard
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
    fn realloc(_: *mut ::core::ffi::c_void, _: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
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
    fn abort() -> !;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn posix_memalign(
        _: *mut *mut ::core::ffi::c_void,
        _: size_t,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
}
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint16_t = ::core::ffi::c_ushort;
pub type uint32_t = ::core::ffi::c_uint;
pub type size_t = ::core::ffi::c_uint;
pub type memory_order = ::core::ffi::c_uint;
pub const memory_order_seq_cst: memory_order = 5;
pub const memory_order_acq_rel: memory_order = 4;
pub const memory_order_release: memory_order = 3;
pub const memory_order_acquire: memory_order = 2;
pub const memory_order_consume: memory_order = 1;
pub const memory_order_relaxed: memory_order = 0;
pub type atomic_size_t = size_t;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union unaligned_16 {
    pub l: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union unaligned_32 {
    pub l: uint32_t,
}
static mut max_alloc_size: atomic_size_t = 2147483647 as ::core::ffi::c_int
    as ::core::ffi::c_uint;
#[no_mangle]
pub unsafe extern "C" fn av_max_alloc(mut max: size_t) {
    crate::rt::atomic_store_relaxed(&raw mut max_alloc_size, max);
}
unsafe extern "C" fn size_mult(
    mut a: size_t,
    mut b: size_t,
    mut r: *mut size_t,
) -> ::core::ffi::c_int {
    let mut t: size_t = 0;
    let (fresh0, fresh1) = a.overflowing_mul(b);
    *&raw mut t = fresh0;
    if fresh1 {
        return -(22 as ::core::ffi::c_int);
    }
    *r = t;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_malloc(mut size: size_t) -> *mut ::core::ffi::c_void {
    let mut ptr: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    if size > crate::rt::atomic_load_relaxed(&raw mut max_alloc_size) {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if size != 0 {
        if posix_memalign(
            &raw mut ptr,
            (if 0 as ::core::ffi::c_int != 0 {
                64 as ::core::ffi::c_int
            } else if 0 as ::core::ffi::c_int != 0 {
                32 as ::core::ffi::c_int
            } else {
                16 as ::core::ffi::c_int
            }) as size_t,
            size as size_t,
        ) != 0
        {
            ptr = ::core::ptr::null_mut::<::core::ffi::c_void>();
        }
    }
    if ptr.is_null() && size == 0 {
        size = 1 as size_t;
        ptr = av_malloc(1 as size_t);
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn av_realloc(
    mut ptr: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    let mut ret: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    if size > crate::rt::atomic_load_relaxed(&raw mut max_alloc_size) {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    ret = realloc(ptr, size.wrapping_add((size == 0) as ::core::ffi::c_int as size_t));
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_realloc_f(
    mut ptr: *mut ::core::ffi::c_void,
    mut nelem: size_t,
    mut elsize: size_t,
) -> *mut ::core::ffi::c_void {
    let mut size: size_t = 0;
    let mut r: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if size_mult(elsize, nelem, &raw mut size) != 0 {
        av_free(ptr);
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    r = av_realloc(ptr, size);
    if r.is_null() {
        av_free(ptr);
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn av_reallocp(
    mut ptr: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> ::core::ffi::c_int {
    let mut val: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    if size == 0 {
        av_freep(ptr);
        return 0 as ::core::ffi::c_int;
    }
    memcpy(
        &raw mut val as *mut ::core::ffi::c_void,
        ptr,
        ::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t,
    );
    val = av_realloc(val, size);
    if val.is_null() {
        av_freep(ptr);
        return -(12 as ::core::ffi::c_int);
    }
    memcpy(
        ptr,
        &raw mut val as *const ::core::ffi::c_void,
        ::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t,
    );
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_malloc_array(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    let mut result: size_t = 0;
    if size_mult(nmemb, size, &raw mut result) < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return av_malloc(result);
}
#[no_mangle]
pub unsafe extern "C" fn av_realloc_array(
    mut ptr: *mut ::core::ffi::c_void,
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    let mut result: size_t = 0;
    if size_mult(nmemb, size, &raw mut result) < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return av_realloc(ptr, result);
}
#[no_mangle]
pub unsafe extern "C" fn av_reallocp_array(
    mut ptr: *mut ::core::ffi::c_void,
    mut nmemb: size_t,
    mut size: size_t,
) -> ::core::ffi::c_int {
    let mut val: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    memcpy(
        &raw mut val as *mut ::core::ffi::c_void,
        ptr,
        ::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t,
    );
    val = av_realloc_f(val, nmemb, size);
    memcpy(
        ptr,
        &raw mut val as *const ::core::ffi::c_void,
        ::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t,
    );
    if val.is_null() && nmemb != 0 && size != 0 {
        return -(12 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_free(mut ptr: *mut ::core::ffi::c_void) {
    free(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn av_freep(mut arg: *mut ::core::ffi::c_void) {
    let mut val: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    memcpy(
        &raw mut val as *mut ::core::ffi::c_void,
        arg,
        ::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t,
    );
    let mut fresh2: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    memcpy(
        arg,
        &raw mut fresh2 as *const ::core::ffi::c_void,
        ::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t,
    );
    av_free(val);
}
#[no_mangle]
pub unsafe extern "C" fn av_mallocz(mut size: size_t) -> *mut ::core::ffi::c_void {
    let mut ptr: *mut ::core::ffi::c_void = av_malloc(size);
    if !ptr.is_null() {
        memset(ptr, 0 as ::core::ffi::c_int, size);
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn av_calloc(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    let mut result: size_t = 0;
    if size_mult(nmemb, size, &raw mut result) < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return av_mallocz(result);
}
#[no_mangle]
pub unsafe extern "C" fn av_strdup(
    mut s: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if !s.is_null() {
        let mut len: size_t = strlen(s).wrapping_add(1 as size_t);
        ptr = av_realloc(::core::ptr::null_mut::<::core::ffi::c_void>(), len)
            as *mut ::core::ffi::c_char;
        if !ptr.is_null() {
            memcpy(
                ptr as *mut ::core::ffi::c_void,
                s as *const ::core::ffi::c_void,
                len,
            );
        }
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn av_strndup(
    mut s: *const ::core::ffi::c_char,
    mut len: size_t,
) -> *mut ::core::ffi::c_char {
    let mut ret: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if s.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    end = memchr(s as *const ::core::ffi::c_void, 0 as ::core::ffi::c_int, len)
        as *mut ::core::ffi::c_char;
    if !end.is_null() {
        len = end.offset_from(s) as ::core::ffi::c_int as size_t;
    }
    ret = av_realloc(
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        len.wrapping_add(1 as size_t),
    ) as *mut ::core::ffi::c_char;
    if ret.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    memcpy(ret as *mut ::core::ffi::c_void, s as *const ::core::ffi::c_void, len);
    *ret.offset(len as isize) = 0 as ::core::ffi::c_char;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_memdup(
    mut p: *const ::core::ffi::c_void,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    let mut ptr: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    if !p.is_null() {
        ptr = av_malloc(size);
        if !ptr.is_null() {
            memcpy(ptr, p, size);
        }
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn av_dynarray_add_nofree(
    mut tab_ptr: *mut ::core::ffi::c_void,
    mut nb_ptr: *mut ::core::ffi::c_int,
    mut elem: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut tab: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        *mut ::core::ffi::c_void,
    >();
    memcpy(
        &raw mut tab as *mut ::core::ffi::c_void,
        tab_ptr,
        ::core::mem::size_of::<*mut *mut ::core::ffi::c_void>() as size_t,
    );
    let mut av_size_new: size_t = *nb_ptr as size_t;
    if *nb_ptr & *nb_ptr - 1 as ::core::ffi::c_int == 0 {
        av_size_new = (if *nb_ptr != 0 {
            *nb_ptr << 1 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) as size_t;
        if av_size_new as usize
            > (2147483647 as ::core::ffi::c_int as usize)
                .wrapping_div(
                    ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize,
                )
        {
            av_size_new = 0 as size_t;
        } else {
            let mut av_array_new: *mut ::core::ffi::c_void = av_realloc(
                tab as *mut ::core::ffi::c_void,
                av_size_new
                    .wrapping_mul(
                        ::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t,
                    ),
            );
            if av_array_new.is_null() {
                av_size_new = 0 as size_t;
            } else {
                tab = av_array_new as *mut *mut ::core::ffi::c_void;
            }
        }
    }
    if av_size_new != 0 {
        let ref mut fresh7 = *tab.offset(*nb_ptr as isize);
        *fresh7 = elem;
        memcpy(
            tab_ptr,
            &raw mut tab as *const ::core::ffi::c_void,
            ::core::mem::size_of::<*mut *mut ::core::ffi::c_void>() as size_t,
        );
        *nb_ptr += 1;
    } else {
        return -(12 as ::core::ffi::c_int)
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_dynarray_add(
    mut tab_ptr: *mut ::core::ffi::c_void,
    mut nb_ptr: *mut ::core::ffi::c_int,
    mut elem: *mut ::core::ffi::c_void,
) {
    let mut tab: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        *mut ::core::ffi::c_void,
    >();
    memcpy(
        &raw mut tab as *mut ::core::ffi::c_void,
        tab_ptr,
        ::core::mem::size_of::<*mut *mut ::core::ffi::c_void>() as size_t,
    );
    let mut av_size_new: size_t = *nb_ptr as size_t;
    if *nb_ptr & *nb_ptr - 1 as ::core::ffi::c_int == 0 {
        av_size_new = (if *nb_ptr != 0 {
            *nb_ptr << 1 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) as size_t;
        if av_size_new as usize
            > (2147483647 as ::core::ffi::c_int as usize)
                .wrapping_div(
                    ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize,
                )
        {
            av_size_new = 0 as size_t;
        } else {
            let mut av_array_new: *mut ::core::ffi::c_void = av_realloc(
                tab as *mut ::core::ffi::c_void,
                av_size_new
                    .wrapping_mul(
                        ::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t,
                    ),
            );
            if av_array_new.is_null() {
                av_size_new = 0 as size_t;
            } else {
                tab = av_array_new as *mut *mut ::core::ffi::c_void;
            }
        }
    }
    if av_size_new != 0 {
        let ref mut fresh6 = *tab.offset(*nb_ptr as isize);
        *fresh6 = elem;
        memcpy(
            tab_ptr,
            &raw mut tab as *const ::core::ffi::c_void,
            ::core::mem::size_of::<*mut *mut ::core::ffi::c_void>() as size_t,
        );
        *nb_ptr += 1;
    } else {
        *nb_ptr = 0 as ::core::ffi::c_int;
        av_freep(tab_ptr);
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_dynarray2_add(
    mut tab_ptr: *mut *mut ::core::ffi::c_void,
    mut nb_ptr: *mut ::core::ffi::c_int,
    mut elem_size: size_t,
    mut elem_data: *const uint8_t,
) -> *mut ::core::ffi::c_void {
    let mut tab_elem_data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut av_size_new: size_t = *nb_ptr as size_t;
    if *nb_ptr & *nb_ptr - 1 as ::core::ffi::c_int == 0 {
        av_size_new = (if *nb_ptr != 0 {
            *nb_ptr << 1 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) as size_t;
        if av_size_new
            > (2147483647 as ::core::ffi::c_int as size_t).wrapping_div(elem_size)
        {
            av_size_new = 0 as size_t;
        } else {
            let mut av_array_new: *mut ::core::ffi::c_void = av_realloc(
                *tab_ptr,
                av_size_new.wrapping_mul(elem_size),
            );
            if av_array_new.is_null() {
                av_size_new = 0 as size_t;
            } else {
                *tab_ptr = av_array_new;
            }
        }
    }
    if av_size_new != 0 {
        tab_elem_data = (*tab_ptr as *mut uint8_t)
            .offset((*nb_ptr as size_t).wrapping_mul(elem_size) as isize);
        if !elem_data.is_null() {
            memcpy(
                tab_elem_data as *mut ::core::ffi::c_void,
                elem_data as *const ::core::ffi::c_void,
                elem_size,
            );
        }
        *nb_ptr += 1;
    } else {
        av_freep(tab_ptr as *mut ::core::ffi::c_void);
        *nb_ptr = 0 as ::core::ffi::c_int;
    }
    return tab_elem_data as *mut ::core::ffi::c_void;
}
unsafe extern "C" fn fill16(mut dst: *mut uint8_t, mut len: ::core::ffi::c_int) {
    let mut v: uint32_t = (*(dst.offset(-(2 as ::core::ffi::c_int as isize))
        as *const unaligned_16))
        .l as uint32_t;
    v |= v << 16 as ::core::ffi::c_int;
    while len >= 4 as ::core::ffi::c_int {
        (*(dst as *mut unaligned_32)).l = v;
        dst = dst.offset(4 as ::core::ffi::c_int as isize);
        len -= 4 as ::core::ffi::c_int;
    }
    loop {
        let fresh5 = len;
        len = len - 1;
        if !(fresh5 != 0) {
            break;
        }
        *dst = *dst.offset(-(2 as ::core::ffi::c_int) as isize);
        dst = dst.offset(1);
    };
}
unsafe extern "C" fn fill24(mut dst: *mut uint8_t, mut len: ::core::ffi::c_int) {
    let mut v: uint32_t = ((*(dst.offset(-(3 as ::core::ffi::c_int as isize))
        as *const uint8_t)
        .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        << 16 as ::core::ffi::c_int
        | (*(dst.offset(-(3 as ::core::ffi::c_int as isize)) as *const uint8_t)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
        | *(dst.offset(-(3 as ::core::ffi::c_int as isize)) as *const uint8_t)
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int) as uint32_t;
    let mut a: uint32_t = v | v << 24 as ::core::ffi::c_int;
    let mut b: uint32_t = v >> 8 as ::core::ffi::c_int | v << 16 as ::core::ffi::c_int;
    let mut c: uint32_t = v >> 16 as ::core::ffi::c_int | v << 8 as ::core::ffi::c_int;
    while len >= 12 as ::core::ffi::c_int {
        (*(dst as *mut unaligned_32)).l = a;
        (*(dst.offset(4 as ::core::ffi::c_int as isize) as *mut unaligned_32)).l = b;
        (*(dst.offset(8 as ::core::ffi::c_int as isize) as *mut unaligned_32)).l = c;
        dst = dst.offset(12 as ::core::ffi::c_int as isize);
        len -= 12 as ::core::ffi::c_int;
    }
    if len >= 4 as ::core::ffi::c_int {
        (*(dst as *mut unaligned_32)).l = a;
        dst = dst.offset(4 as ::core::ffi::c_int as isize);
        len -= 4 as ::core::ffi::c_int;
    }
    if len >= 4 as ::core::ffi::c_int {
        (*(dst as *mut unaligned_32)).l = b;
        dst = dst.offset(4 as ::core::ffi::c_int as isize);
        len -= 4 as ::core::ffi::c_int;
    }
    loop {
        let fresh4 = len;
        len = len - 1;
        if !(fresh4 != 0) {
            break;
        }
        *dst = *dst.offset(-(3 as ::core::ffi::c_int) as isize);
        dst = dst.offset(1);
    };
}
unsafe extern "C" fn fill32(mut dst: *mut uint8_t, mut len: ::core::ffi::c_int) {
    let mut v: uint32_t = (*(dst.offset(-(4 as ::core::ffi::c_int as isize))
        as *const unaligned_32))
        .l;
    while len >= 4 as ::core::ffi::c_int {
        (*(dst as *mut unaligned_32)).l = v;
        dst = dst.offset(4 as ::core::ffi::c_int as isize);
        len -= 4 as ::core::ffi::c_int;
    }
    loop {
        let fresh3 = len;
        len = len - 1;
        if !(fresh3 != 0) {
            break;
        }
        *dst = *dst.offset(-(4 as ::core::ffi::c_int) as isize);
        dst = dst.offset(1);
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_memcpy_backptr(
    mut dst: *mut uint8_t,
    mut back: ::core::ffi::c_int,
    mut cnt: ::core::ffi::c_int,
) {
    let mut src: *const uint8_t = dst.offset(-back as isize) as *mut uint8_t;
    if back == 0 {
        return;
    }
    if back == 1 as ::core::ffi::c_int {
        memset(
            dst as *mut ::core::ffi::c_void,
            *src as ::core::ffi::c_int,
            cnt as size_t,
        );
    } else if back == 2 as ::core::ffi::c_int {
        fill16(dst, cnt);
    } else if back == 3 as ::core::ffi::c_int {
        fill24(dst, cnt);
    } else if back == 4 as ::core::ffi::c_int {
        fill32(dst, cnt);
    } else {
        if cnt >= 16 as ::core::ffi::c_int {
            let mut blocklen: ::core::ffi::c_int = back;
            while cnt > blocklen {
                memcpy(
                    dst as *mut ::core::ffi::c_void,
                    src as *const ::core::ffi::c_void,
                    blocklen as size_t,
                );
                dst = dst.offset(blocklen as isize);
                cnt -= blocklen;
                blocklen <<= 1 as ::core::ffi::c_int;
            }
            memcpy(
                dst as *mut ::core::ffi::c_void,
                src as *const ::core::ffi::c_void,
                cnt as size_t,
            );
            return;
        }
        if cnt >= 8 as ::core::ffi::c_int {
            (*(dst as *mut unaligned_32)).l = (*(src as *const unaligned_32)).l;
            (*(dst.offset(4 as ::core::ffi::c_int as isize) as *mut unaligned_32)).l = (*(src
                .offset(4 as ::core::ffi::c_int as isize) as *const unaligned_32))
                .l;
            src = src.offset(8 as ::core::ffi::c_int as isize);
            dst = dst.offset(8 as ::core::ffi::c_int as isize);
            cnt -= 8 as ::core::ffi::c_int;
        }
        if cnt >= 4 as ::core::ffi::c_int {
            (*(dst as *mut unaligned_32)).l = (*(src as *const unaligned_32)).l;
            src = src.offset(4 as ::core::ffi::c_int as isize);
            dst = dst.offset(4 as ::core::ffi::c_int as isize);
            cnt -= 4 as ::core::ffi::c_int;
        }
        if cnt >= 2 as ::core::ffi::c_int {
            (*(dst as *mut unaligned_16)).l = (*(src as *const unaligned_16)).l;
            src = src.offset(2 as ::core::ffi::c_int as isize);
            dst = dst.offset(2 as ::core::ffi::c_int as isize);
            cnt -= 2 as ::core::ffi::c_int;
        }
        if cnt != 0 {
            *dst = *src;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_fast_realloc(
    mut ptr: *mut ::core::ffi::c_void,
    mut size: *mut ::core::ffi::c_uint,
    mut min_size: size_t,
) -> *mut ::core::ffi::c_void {
    let mut max_size: size_t = 0;
    if min_size <= *size {
        return ptr;
    }
    max_size = crate::rt::atomic_load_relaxed(&raw mut max_alloc_size);
    max_size = (if max_size
        > (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
            .wrapping_mul(2 as ::core::ffi::c_uint)
            .wrapping_add(1 as ::core::ffi::c_uint)
    {
        (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
            .wrapping_mul(2 as ::core::ffi::c_uint)
            .wrapping_add(1 as ::core::ffi::c_uint)
    } else {
        max_size as ::core::ffi::c_uint
    }) as size_t;
    if min_size > max_size {
        *size = 0 as ::core::ffi::c_uint;
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    min_size = if max_size
        > (if min_size
            .wrapping_add(min_size.wrapping_div(16 as size_t))
            .wrapping_add(32 as size_t) > min_size
        {
            min_size
                .wrapping_add(min_size.wrapping_div(16 as size_t))
                .wrapping_add(32 as size_t)
        } else {
            min_size
        })
    {
        if min_size
            .wrapping_add(min_size.wrapping_div(16 as size_t))
            .wrapping_add(32 as size_t) > min_size
        {
            min_size
                .wrapping_add(min_size.wrapping_div(16 as size_t))
                .wrapping_add(32 as size_t)
        } else {
            min_size
        }
    } else {
        max_size
    };
    ptr = av_realloc(ptr, min_size);
    if ptr.is_null() {
        min_size = 0 as size_t;
    }
    *size = min_size as ::core::ffi::c_uint;
    return ptr;
}
#[inline]
unsafe extern "C" fn fast_malloc(
    mut ptr: *mut ::core::ffi::c_void,
    mut size: *mut ::core::ffi::c_uint,
    mut min_size: size_t,
    mut zero_realloc: ::core::ffi::c_int,
) {
    let mut max_size: size_t = 0;
    let mut val: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    memcpy(
        &raw mut val as *mut ::core::ffi::c_void,
        ptr,
        ::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t,
    );
    if min_size <= *size {
        if !(!val.is_null() || min_size == 0) {
            av_log(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                0 as ::core::ffi::c_int,
                b"Assertion %s failed at %s:%d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"val || !min_size\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/libavutil/mem.c\0" as *const u8 as *const ::core::ffi::c_char,
                534 as ::core::ffi::c_int,
            );
            abort();
        }
        return;
    }
    max_size = crate::rt::atomic_load_relaxed(&raw mut max_alloc_size);
    max_size = (if max_size
        > (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
            .wrapping_mul(2 as ::core::ffi::c_uint)
            .wrapping_add(1 as ::core::ffi::c_uint)
    {
        (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
            .wrapping_mul(2 as ::core::ffi::c_uint)
            .wrapping_add(1 as ::core::ffi::c_uint)
    } else {
        max_size as ::core::ffi::c_uint
    }) as size_t;
    if min_size > max_size {
        av_freep(ptr);
        *size = 0 as ::core::ffi::c_uint;
        return;
    }
    min_size = if max_size
        > (if min_size
            .wrapping_add(min_size.wrapping_div(16 as size_t))
            .wrapping_add(32 as size_t) > min_size
        {
            min_size
                .wrapping_add(min_size.wrapping_div(16 as size_t))
                .wrapping_add(32 as size_t)
        } else {
            min_size
        })
    {
        if min_size
            .wrapping_add(min_size.wrapping_div(16 as size_t))
            .wrapping_add(32 as size_t) > min_size
        {
            min_size
                .wrapping_add(min_size.wrapping_div(16 as size_t))
                .wrapping_add(32 as size_t)
        } else {
            min_size
        }
    } else {
        max_size
    };
    av_freep(ptr);
    val = if zero_realloc != 0 { av_mallocz(min_size) } else { av_malloc(min_size) };
    memcpy(
        ptr,
        &raw mut val as *const ::core::ffi::c_void,
        ::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t,
    );
    if val.is_null() {
        min_size = 0 as size_t;
    }
    *size = min_size as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn av_fast_malloc(
    mut ptr: *mut ::core::ffi::c_void,
    mut size: *mut ::core::ffi::c_uint,
    mut min_size: size_t,
) {
    fast_malloc(ptr, size, min_size, 0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn av_fast_mallocz(
    mut ptr: *mut ::core::ffi::c_void,
    mut size: *mut ::core::ffi::c_uint,
    mut min_size: size_t,
) {
    fast_malloc(ptr, size, min_size, 1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn av_size_mult(
    mut a: size_t,
    mut b: size_t,
    mut r: *mut size_t,
) -> ::core::ffi::c_int {
    return size_mult(a, b, r);
}
