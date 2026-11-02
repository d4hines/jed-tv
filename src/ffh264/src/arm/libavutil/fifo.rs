// Translated from FFmpeg 9.0.1 libavutil/fifo.c
// (preprocessed for armv7-unknown-linux-musleabihf) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// a very simple circular buffer FIFO implementation
// Copyright (c) 2000, 2001, 2002 Fabrice Bellard
// Copyright (c) 2006 Roman Shaposhnik
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
    fn memmove(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn abort() -> !;
    fn av_mallocz(size: size_t) -> *mut ::core::ffi::c_void;
    fn av_realloc_array(
        ptr: *mut ::core::ffi::c_void,
        nmemb: size_t,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn av_free(ptr: *mut ::core::ffi::c_void);
    fn av_freep(ptr: *mut ::core::ffi::c_void);
}
pub type uint8_t = ::core::ffi::c_uchar;
pub type size_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFifo {
    pub buffer: *mut uint8_t,
    pub elem_size: size_t,
    pub nb_elems: size_t,
    pub offset_r: size_t,
    pub offset_w: size_t,
    pub is_empty: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_uint,
    pub auto_grow_limit: size_t,
}
pub type AVFifoCB = unsafe extern "C" fn(
    *mut ::core::ffi::c_void,
    *mut ::core::ffi::c_void,
    *mut size_t,
) -> ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn av_fifo_alloc2(
    mut nb_elems: size_t,
    mut elem_size: size_t,
    mut flags: ::core::ffi::c_uint,
) -> *mut AVFifo {
    let mut f: *mut AVFifo = ::core::ptr::null_mut::<AVFifo>();
    let mut buffer: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    if elem_size == 0 {
        return ::core::ptr::null_mut::<AVFifo>();
    }
    if nb_elems != 0 {
        buffer = av_realloc_array(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            nb_elems,
            elem_size,
        );
        if buffer.is_null() {
            return ::core::ptr::null_mut::<AVFifo>();
        }
    }
    f = av_mallocz(::core::mem::size_of::<AVFifo>() as size_t) as *mut AVFifo;
    if f.is_null() {
        av_free(buffer);
        return ::core::ptr::null_mut::<AVFifo>();
    }
    (*f).buffer = buffer as *mut uint8_t;
    (*f).nb_elems = nb_elems;
    (*f).elem_size = elem_size;
    (*f).is_empty = 1 as ::core::ffi::c_int;
    (*f).flags = flags;
    (*f).auto_grow_limit = if ((1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int)
        as size_t)
        .wrapping_div(elem_size) > 1 as size_t
    {
        ((1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as size_t)
            .wrapping_div(elem_size)
    } else {
        1 as size_t
    };
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn av_fifo_auto_grow_limit(
    mut f: *mut AVFifo,
    mut max_elems: size_t,
) {
    (*f).auto_grow_limit = max_elems;
}
#[no_mangle]
pub unsafe extern "C" fn av_fifo_elem_size(mut f: *const AVFifo) -> size_t {
    return (*f).elem_size;
}
#[no_mangle]
pub unsafe extern "C" fn av_fifo_can_read(mut f: *const AVFifo) -> size_t {
    if (*f).offset_w <= (*f).offset_r && (*f).is_empty == 0 {
        return (*f).nb_elems.wrapping_sub((*f).offset_r).wrapping_add((*f).offset_w);
    }
    return (*f).offset_w.wrapping_sub((*f).offset_r);
}
#[no_mangle]
pub unsafe extern "C" fn av_fifo_can_write(mut f: *const AVFifo) -> size_t {
    return (*f).nb_elems.wrapping_sub(av_fifo_can_read(f));
}
#[no_mangle]
pub unsafe extern "C" fn av_fifo_grow2(
    mut f: *mut AVFifo,
    mut inc: size_t,
) -> ::core::ffi::c_int {
    let mut tmp: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    if inc > (0xffffffff as size_t).wrapping_sub((*f).nb_elems) {
        return -(22 as ::core::ffi::c_int);
    }
    tmp = av_realloc_array(
        (*f).buffer as *mut ::core::ffi::c_void,
        (*f).nb_elems.wrapping_add(inc),
        (*f).elem_size,
    ) as *mut uint8_t;
    if tmp.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    (*f).buffer = tmp;
    if (*f).offset_w <= (*f).offset_r && (*f).is_empty == 0 {
        let copy: size_t = if inc > (*f).offset_w { (*f).offset_w } else { inc };
        memcpy(
            tmp.offset((*f).nb_elems.wrapping_mul((*f).elem_size) as isize)
                as *mut ::core::ffi::c_void,
            tmp as *const ::core::ffi::c_void,
            (copy as size_t).wrapping_mul((*f).elem_size as size_t),
        );
        if copy < (*f).offset_w {
            memmove(
                tmp as *mut ::core::ffi::c_void,
                tmp.offset(copy.wrapping_mul((*f).elem_size) as isize)
                    as *const ::core::ffi::c_void,
                ((*f).offset_w as size_t)
                    .wrapping_sub(copy as size_t)
                    .wrapping_mul((*f).elem_size as size_t),
            );
            (*f).offset_w = (*f).offset_w.wrapping_sub(copy);
        } else {
            (*f).offset_w = if copy == inc {
                0 as size_t
            } else {
                (*f).nb_elems.wrapping_add(copy)
            };
        }
    }
    (*f).nb_elems = (*f).nb_elems.wrapping_add(inc);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn fifo_check_space(
    mut f: *mut AVFifo,
    mut to_write: size_t,
) -> ::core::ffi::c_int {
    let can_write: size_t = av_fifo_can_write(f) as size_t;
    let need_grow: size_t = if to_write > can_write {
        to_write.wrapping_sub(can_write)
    } else {
        0 as size_t
    };
    let mut can_grow: size_t = 0;
    if need_grow == 0 {
        return 0 as ::core::ffi::c_int;
    }
    can_grow = if (*f).auto_grow_limit > (*f).nb_elems {
        (*f).auto_grow_limit.wrapping_sub((*f).nb_elems)
    } else {
        0 as size_t
    };
    if (*f).flags
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as ::core::ffi::c_uint
        != 0 && need_grow <= can_grow
    {
        let inc: size_t = if need_grow < can_grow.wrapping_div(2 as size_t) {
            need_grow.wrapping_mul(2 as size_t)
        } else {
            can_grow
        };
        return av_fifo_grow2(f, inc);
    }
    return -(28 as ::core::ffi::c_int);
}
unsafe extern "C" fn fifo_write_common(
    mut f: *mut AVFifo,
    mut buf: *const uint8_t,
    mut nb_elems: *mut size_t,
    mut read_cb: Option<AVFifoCB>,
    mut opaque: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut to_write: size_t = *nb_elems;
    let mut offset_w: size_t = 0;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    ret = fifo_check_space(f, to_write);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    offset_w = (*f).offset_w;
    while to_write > 0 as size_t {
        let mut len: size_t = if (*f).nb_elems.wrapping_sub(offset_w) > to_write {
            to_write
        } else {
            (*f).nb_elems.wrapping_sub(offset_w)
        };
        let mut wptr: *mut uint8_t = (*f)
            .buffer
            .offset(offset_w.wrapping_mul((*f).elem_size) as isize);
        if read_cb.is_some() {
            ret = read_cb
                .expect(
                    "non-null function pointer",
                )(opaque, wptr as *mut ::core::ffi::c_void, &raw mut len);
            if ret < 0 as ::core::ffi::c_int || len == 0 as size_t {
                break;
            }
        } else {
            memcpy(
                wptr as *mut ::core::ffi::c_void,
                buf as *const ::core::ffi::c_void,
                (len as size_t).wrapping_mul((*f).elem_size as size_t),
            );
            buf = buf.offset(len.wrapping_mul((*f).elem_size) as isize);
        }
        offset_w = offset_w.wrapping_add(len);
        if offset_w >= (*f).nb_elems {
            offset_w = 0 as size_t;
        }
        to_write = to_write.wrapping_sub(len);
    }
    (*f).offset_w = offset_w;
    if *nb_elems != to_write {
        (*f).is_empty = 0 as ::core::ffi::c_int;
    }
    *nb_elems = (*nb_elems).wrapping_sub(to_write);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_fifo_write(
    mut f: *mut AVFifo,
    mut buf: *const ::core::ffi::c_void,
    mut nb_elems: size_t,
) -> ::core::ffi::c_int {
    return fifo_write_common(
        f,
        buf as *const uint8_t,
        &raw mut nb_elems,
        None,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_fifo_write_from_cb(
    mut f: *mut AVFifo,
    mut read_cb: Option<AVFifoCB>,
    mut opaque: *mut ::core::ffi::c_void,
    mut nb_elems: *mut size_t,
) -> ::core::ffi::c_int {
    return fifo_write_common(
        f,
        ::core::ptr::null::<uint8_t>(),
        nb_elems,
        read_cb,
        opaque,
    );
}
unsafe extern "C" fn fifo_peek_common(
    mut f: *const AVFifo,
    mut buf: *mut uint8_t,
    mut nb_elems: *mut size_t,
    mut offset: size_t,
    mut write_cb: Option<AVFifoCB>,
    mut opaque: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut to_read: size_t = *nb_elems;
    let mut offset_r: size_t = (*f).offset_r;
    let mut can_read: size_t = av_fifo_can_read(f);
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if offset > can_read || to_read > can_read.wrapping_sub(offset) {
        *nb_elems = 0 as size_t;
        return -(22 as ::core::ffi::c_int);
    }
    if offset_r >= (*f).nb_elems.wrapping_sub(offset) {
        offset_r = offset_r.wrapping_sub((*f).nb_elems.wrapping_sub(offset));
    } else {
        offset_r = offset_r.wrapping_add(offset);
    }
    while to_read > 0 as size_t {
        let mut len: size_t = if (*f).nb_elems.wrapping_sub(offset_r) > to_read {
            to_read
        } else {
            (*f).nb_elems.wrapping_sub(offset_r)
        };
        let mut rptr: *mut uint8_t = (*f)
            .buffer
            .offset(offset_r.wrapping_mul((*f).elem_size) as isize);
        if write_cb.is_some() {
            ret = write_cb
                .expect(
                    "non-null function pointer",
                )(opaque, rptr as *mut ::core::ffi::c_void, &raw mut len);
            if ret < 0 as ::core::ffi::c_int || len == 0 as size_t {
                break;
            }
        } else {
            memcpy(
                buf as *mut ::core::ffi::c_void,
                rptr as *const ::core::ffi::c_void,
                (len as size_t).wrapping_mul((*f).elem_size as size_t),
            );
            buf = buf.offset(len.wrapping_mul((*f).elem_size) as isize);
        }
        offset_r = offset_r.wrapping_add(len);
        if offset_r >= (*f).nb_elems {
            offset_r = 0 as size_t;
        }
        to_read = to_read.wrapping_sub(len);
    }
    *nb_elems = (*nb_elems).wrapping_sub(to_read);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_fifo_read(
    mut f: *mut AVFifo,
    mut buf: *mut ::core::ffi::c_void,
    mut nb_elems: size_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = fifo_peek_common(
        f,
        buf as *mut uint8_t,
        &raw mut nb_elems,
        0 as size_t,
        None,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    av_fifo_drain2(f, nb_elems);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_fifo_read_to_cb(
    mut f: *mut AVFifo,
    mut write_cb: Option<AVFifoCB>,
    mut opaque: *mut ::core::ffi::c_void,
    mut nb_elems: *mut size_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = fifo_peek_common(
        f,
        ::core::ptr::null_mut::<uint8_t>(),
        nb_elems,
        0 as size_t,
        write_cb,
        opaque,
    );
    av_fifo_drain2(f, *nb_elems);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_fifo_peek(
    mut f: *const AVFifo,
    mut buf: *mut ::core::ffi::c_void,
    mut nb_elems: size_t,
    mut offset: size_t,
) -> ::core::ffi::c_int {
    return fifo_peek_common(
        f,
        buf as *mut uint8_t,
        &raw mut nb_elems,
        offset,
        None,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_fifo_peek_to_cb(
    mut f: *const AVFifo,
    mut write_cb: Option<AVFifoCB>,
    mut opaque: *mut ::core::ffi::c_void,
    mut nb_elems: *mut size_t,
    mut offset: size_t,
) -> ::core::ffi::c_int {
    return fifo_peek_common(
        f,
        ::core::ptr::null_mut::<uint8_t>(),
        nb_elems,
        offset,
        write_cb,
        opaque,
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_fifo_drain2(mut f: *mut AVFifo, mut size: size_t) {
    let cur_size: size_t = av_fifo_can_read(f) as size_t;
    if !(cur_size >= size) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"cur_size >= size\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavutil/fifo.c\0" as *const u8 as *const ::core::ffi::c_char,
            270 as ::core::ffi::c_int,
        );
        abort();
    }
    if cur_size == size {
        (*f).is_empty = 1 as ::core::ffi::c_int;
    }
    if (*f).offset_r >= (*f).nb_elems.wrapping_sub(size) {
        (*f).offset_r = (*f).offset_r.wrapping_sub((*f).nb_elems.wrapping_sub(size));
    } else {
        (*f).offset_r = (*f).offset_r.wrapping_add(size);
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_fifo_reset2(mut f: *mut AVFifo) {
    (*f).offset_w = 0 as size_t;
    (*f).offset_r = (*f).offset_w;
    (*f).is_empty = 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_fifo_freep2(mut f: *mut *mut AVFifo) {
    if !(*f).is_null() {
        av_freep(&raw mut (**f).buffer as *mut ::core::ffi::c_void);
        av_freep(f as *mut ::core::ffi::c_void);
    }
}
