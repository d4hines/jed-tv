// Translated from FFmpeg 9.0.1 libavutil/buffer.c
// (preprocessed for armv7-unknown-linux-musleabihf) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
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
    fn abort() -> !;
    fn pthread_mutex_init(
        _: *mut pthread_mutex_t,
        _: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_destroy(_: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn av_malloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn av_mallocz(size: size_t) -> *mut ::core::ffi::c_void;
    fn av_realloc(
        ptr: *mut ::core::ffi::c_void,
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
pub type size_t = ::core::ffi::c_uint;
pub type uint8_t = ::core::ffi::c_uchar;
pub type memory_order = ::core::ffi::c_uint;
pub const memory_order_seq_cst: memory_order = 5;
pub const memory_order_acq_rel: memory_order = 4;
pub const memory_order_release: memory_order = 3;
pub const memory_order_acquire: memory_order = 2;
pub const memory_order_consume: memory_order = 1;
pub const memory_order_relaxed: memory_order = 0;
pub type atomic_uint = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBuffer {
    pub data: *mut uint8_t,
    pub size: size_t,
    pub refcount: atomic_uint,
    pub free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint8_t) -> ()>,
    pub opaque: *mut ::core::ffi::c_void,
    pub flags: ::core::ffi::c_int,
    pub flags_internal: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBufferRef {
    pub buffer: *mut AVBuffer,
    pub data: *mut uint8_t,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBufferPool {
    pub mutex: pthread_mutex_t,
    pub pool: *mut BufferPoolEntry,
    pub refcount: atomic_uint,
    pub size: size_t,
    pub opaque: *mut ::core::ffi::c_void,
    pub alloc: Option<unsafe extern "C" fn(size_t) -> *mut AVBufferRef>,
    pub alloc2: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut AVBufferRef,
    >,
    pub pool_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufferPoolEntry {
    pub data: *mut uint8_t,
    pub opaque: *mut ::core::ffi::c_void,
    pub free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint8_t) -> ()>,
    pub pool: *mut AVBufferPool,
    pub next: *mut BufferPoolEntry,
    pub buffer: AVBuffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pthread_mutex_t {
    pub __u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __i: [::core::ffi::c_int; 6],
    pub __vi: [::core::ffi::c_int; 6],
    pub __p: [*mut ::core::ffi::c_void; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pthread_mutexattr_t {
    pub __attr: ::core::ffi::c_uint,
}
unsafe extern "C" fn buffer_create(
    mut buf: *mut AVBuffer,
    mut data: *mut uint8_t,
    mut size: size_t,
    mut free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint8_t) -> ()>,
    mut opaque: *mut ::core::ffi::c_void,
    mut flags: ::core::ffi::c_int,
) -> *mut AVBufferRef {
    let mut ref_0: *mut AVBufferRef = ::core::ptr::null_mut::<AVBufferRef>();
    (*buf).data = data;
    (*buf).size = size;
    (*buf).free = (if free.is_some() {
        free
            as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint8_t) -> ()>
    } else {
        Some(
            av_buffer_default_free
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint8_t) -> (),
        )
    }) as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint8_t) -> ()>;
    (*buf).opaque = opaque;
    *&raw mut (*buf).refcount = 1 as ::core::ffi::c_int as ::core::ffi::c_uint;
    (*buf).flags = flags;
    ref_0 = av_mallocz(::core::mem::size_of::<AVBufferRef>() as size_t)
        as *mut AVBufferRef;
    if ref_0.is_null() {
        return ::core::ptr::null_mut::<AVBufferRef>();
    }
    (*ref_0).buffer = buf;
    (*ref_0).data = data;
    (*ref_0).size = size;
    return ref_0;
}
#[no_mangle]
pub unsafe extern "C" fn av_buffer_create(
    mut data: *mut uint8_t,
    mut size: size_t,
    mut free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint8_t) -> ()>,
    mut opaque: *mut ::core::ffi::c_void,
    mut flags: ::core::ffi::c_int,
) -> *mut AVBufferRef {
    let mut ret: *mut AVBufferRef = ::core::ptr::null_mut::<AVBufferRef>();
    let mut buf: *mut AVBuffer = av_mallocz(::core::mem::size_of::<AVBuffer>() as size_t)
        as *mut AVBuffer;
    if buf.is_null() {
        return ::core::ptr::null_mut::<AVBufferRef>();
    }
    ret = buffer_create(buf, data, size, free, opaque, flags);
    if ret.is_null() {
        av_free(buf as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<AVBufferRef>();
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_buffer_default_free(
    mut opaque: *mut ::core::ffi::c_void,
    mut data: *mut uint8_t,
) {
    av_free(data as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn av_buffer_alloc(mut size: size_t) -> *mut AVBufferRef {
    let mut ret: *mut AVBufferRef = ::core::ptr::null_mut::<AVBufferRef>();
    let mut data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    data = av_malloc(size) as *mut uint8_t;
    if data.is_null() {
        return ::core::ptr::null_mut::<AVBufferRef>();
    }
    ret = av_buffer_create(
        data,
        size,
        Some(
            av_buffer_default_free
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint8_t) -> (),
        ),
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        0 as ::core::ffi::c_int,
    );
    if ret.is_null() {
        av_freep(&raw mut data as *mut ::core::ffi::c_void);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_buffer_allocz(mut size: size_t) -> *mut AVBufferRef {
    let mut ret: *mut AVBufferRef = av_buffer_alloc(size);
    if ret.is_null() {
        return ::core::ptr::null_mut::<AVBufferRef>();
    }
    memset((*ret).data as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, size);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_buffer_ref(mut buf: *const AVBufferRef) -> *mut AVBufferRef {
    let mut ret: *mut AVBufferRef = av_mallocz(
        ::core::mem::size_of::<AVBufferRef>() as size_t,
    ) as *mut AVBufferRef;
    if ret.is_null() {
        return ::core::ptr::null_mut::<AVBufferRef>();
    }
    *ret = *buf;
    crate::rt::atomic_xadd_relaxed(
        &raw mut (*(*buf).buffer).refcount,
        1 as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    return ret;
}
unsafe extern "C" fn buffer_replace(
    mut dst: *mut *mut AVBufferRef,
    mut src: *mut *mut AVBufferRef,
) {
    let mut b: *mut AVBuffer = ::core::ptr::null_mut::<AVBuffer>();
    b = (**dst).buffer;
    if !src.is_null() {
        **dst = **src;
        av_freep(src as *mut ::core::ffi::c_void);
    } else {
        av_freep(dst as *mut ::core::ffi::c_void);
    }
    if crate::rt::atomic_xsub_acqrel(
        &raw mut (*b).refcount,
        1 as ::core::ffi::c_int as ::core::ffi::c_uint,
    ) == 1 as ::core::ffi::c_uint
    {
        let mut free_avbuffer: ::core::ffi::c_int = ((*b).flags_internal
            & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int == 0)
            as ::core::ffi::c_int;
        (*b).free.expect("non-null function pointer")((*b).opaque, (*b).data);
        if free_avbuffer != 0 {
            av_free(b as *mut ::core::ffi::c_void);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn av_buffer_unref(mut buf: *mut *mut AVBufferRef) {
    if buf.is_null() || (*buf).is_null() {
        return;
    }
    buffer_replace(buf, ::core::ptr::null_mut::<*mut AVBufferRef>());
}
#[no_mangle]
pub unsafe extern "C" fn av_buffer_is_writable(
    mut buf: *const AVBufferRef,
) -> ::core::ffi::c_int {
    if (*(*buf).buffer).flags & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    return (crate::rt::atomic_load_seqcst(&raw mut (*(*buf).buffer).refcount)
        == 1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_buffer_get_opaque(
    mut buf: *const AVBufferRef,
) -> *mut ::core::ffi::c_void {
    return (*(*buf).buffer).opaque;
}
#[no_mangle]
pub unsafe extern "C" fn av_buffer_get_ref_count(
    mut buf: *const AVBufferRef,
) -> ::core::ffi::c_int {
    return crate::rt::atomic_load_seqcst(&raw mut (*(*buf).buffer).refcount)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_buffer_make_writable(
    mut pbuf: *mut *mut AVBufferRef,
) -> ::core::ffi::c_int {
    let mut newbuf: *mut AVBufferRef = ::core::ptr::null_mut::<AVBufferRef>();
    let mut buf: *mut AVBufferRef = *pbuf;
    if av_buffer_is_writable(buf) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    newbuf = av_buffer_alloc((*buf).size);
    if newbuf.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    memcpy(
        (*newbuf).data as *mut ::core::ffi::c_void,
        (*buf).data as *const ::core::ffi::c_void,
        (*buf).size,
    );
    buffer_replace(pbuf, &raw mut newbuf);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_buffer_realloc(
    mut pbuf: *mut *mut AVBufferRef,
    mut size: size_t,
) -> ::core::ffi::c_int {
    let mut buf: *mut AVBufferRef = *pbuf;
    let mut tmp: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut ret: ::core::ffi::c_int = 0;
    if buf.is_null() {
        let mut data: *mut uint8_t = av_realloc(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            size,
        ) as *mut uint8_t;
        if data.is_null() {
            return -(12 as ::core::ffi::c_int);
        }
        buf = av_buffer_create(
            data,
            size,
            Some(
                av_buffer_default_free
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint8_t) -> (),
            ),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
        );
        if buf.is_null() {
            av_freep(&raw mut data as *mut ::core::ffi::c_void);
            return -(12 as ::core::ffi::c_int);
        }
        (*(*buf).buffer).flags_internal
            |= (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
        *pbuf = buf;
        return 0 as ::core::ffi::c_int;
    } else if (*buf).size == size {
        return 0 as ::core::ffi::c_int
    }
    if (*(*buf).buffer).flags_internal
        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int == 0
        || av_buffer_is_writable(buf) == 0 || (*buf).data != (*(*buf).buffer).data
    {
        let mut new: *mut AVBufferRef = ::core::ptr::null_mut::<AVBufferRef>();
        ret = av_buffer_realloc(&raw mut new, size);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        memcpy(
            (*new).data as *mut ::core::ffi::c_void,
            (*buf).data as *const ::core::ffi::c_void,
            if size > (*buf).size { (*buf).size } else { size },
        );
        buffer_replace(pbuf, &raw mut new);
        return 0 as ::core::ffi::c_int;
    }
    tmp = av_realloc((*(*buf).buffer).data as *mut ::core::ffi::c_void, size)
        as *mut uint8_t;
    if tmp.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    (*buf).data = tmp;
    (*(*buf).buffer).data = (*buf).data;
    (*buf).size = size;
    (*(*buf).buffer).size = (*buf).size;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_buffer_replace(
    mut pdst: *mut *mut AVBufferRef,
    mut src: *const AVBufferRef,
) -> ::core::ffi::c_int {
    let mut dst: *mut AVBufferRef = *pdst;
    let mut tmp: *mut AVBufferRef = ::core::ptr::null_mut::<AVBufferRef>();
    if src.is_null() {
        av_buffer_unref(pdst);
        return 0 as ::core::ffi::c_int;
    }
    if !dst.is_null() && (*dst).buffer == (*src).buffer {
        (*dst).data = (*src).data;
        (*dst).size = (*src).size;
        return 0 as ::core::ffi::c_int;
    }
    tmp = av_buffer_ref(src);
    if tmp.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    av_buffer_unref(pdst);
    *pdst = tmp;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_buffer_pool_init2(
    mut size: size_t,
    mut opaque: *mut ::core::ffi::c_void,
    mut alloc: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut AVBufferRef,
    >,
    mut pool_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) -> *mut AVBufferPool {
    let mut pool: *mut AVBufferPool = av_mallocz(
        ::core::mem::size_of::<AVBufferPool>() as size_t,
    ) as *mut AVBufferPool;
    if pool.is_null() {
        return ::core::ptr::null_mut::<AVBufferPool>();
    }
    if pthread_mutex_init(
        &raw mut (*pool).mutex,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    ) != 0
    {
        av_free(pool as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<AVBufferPool>();
    }
    (*pool).size = size;
    (*pool).opaque = opaque;
    (*pool).alloc2 = alloc;
    (*pool).alloc = Some(
        av_buffer_alloc as unsafe extern "C" fn(size_t) -> *mut AVBufferRef,
    ) as Option<unsafe extern "C" fn(size_t) -> *mut AVBufferRef>;
    (*pool).pool_free = pool_free;
    *&raw mut (*pool).refcount = 1 as ::core::ffi::c_int as ::core::ffi::c_uint;
    return pool;
}
#[no_mangle]
pub unsafe extern "C" fn av_buffer_pool_init(
    mut size: size_t,
    mut alloc: Option<unsafe extern "C" fn(size_t) -> *mut AVBufferRef>,
) -> *mut AVBufferPool {
    let mut pool: *mut AVBufferPool = av_mallocz(
        ::core::mem::size_of::<AVBufferPool>() as size_t,
    ) as *mut AVBufferPool;
    if pool.is_null() {
        return ::core::ptr::null_mut::<AVBufferPool>();
    }
    if pthread_mutex_init(
        &raw mut (*pool).mutex,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    ) != 0
    {
        av_free(pool as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<AVBufferPool>();
    }
    (*pool).size = size;
    (*pool).alloc = (if alloc.is_some() {
        alloc as Option<unsafe extern "C" fn(size_t) -> *mut AVBufferRef>
    } else {
        Some(av_buffer_alloc as unsafe extern "C" fn(size_t) -> *mut AVBufferRef)
    }) as Option<unsafe extern "C" fn(size_t) -> *mut AVBufferRef>;
    *&raw mut (*pool).refcount = 1 as ::core::ffi::c_int as ::core::ffi::c_uint;
    return pool;
}
unsafe extern "C" fn buffer_pool_flush(mut pool: *mut AVBufferPool) {
    while !(*pool).pool.is_null() {
        let mut buf: *mut BufferPoolEntry = (*pool).pool;
        (*pool).pool = (*buf).next as *mut BufferPoolEntry;
        (*buf).free.expect("non-null function pointer")((*buf).opaque, (*buf).data);
        av_freep(&raw mut buf as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn buffer_pool_free(mut pool: *mut AVBufferPool) {
    buffer_pool_flush(pool);
    pthread_mutex_destroy(&raw mut (*pool).mutex);
    if (*pool).pool_free.is_some() {
        (*pool).pool_free.expect("non-null function pointer")((*pool).opaque);
    }
    av_freep(&raw mut pool as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn av_buffer_pool_uninit(mut ppool: *mut *mut AVBufferPool) {
    let mut pool: *mut AVBufferPool = ::core::ptr::null_mut::<AVBufferPool>();
    if ppool.is_null() || (*ppool).is_null() {
        return;
    }
    pool = *ppool;
    *ppool = ::core::ptr::null_mut::<AVBufferPool>();
    pthread_mutex_lock(&raw mut (*pool).mutex);
    buffer_pool_flush(pool);
    pthread_mutex_unlock(&raw mut (*pool).mutex);
    if crate::rt::atomic_xsub_acqrel(
        &raw mut (*pool).refcount,
        1 as ::core::ffi::c_int as ::core::ffi::c_uint,
    ) == 1 as ::core::ffi::c_uint
    {
        buffer_pool_free(pool);
    }
}
unsafe extern "C" fn pool_release_buffer(
    mut opaque: *mut ::core::ffi::c_void,
    mut data: *mut uint8_t,
) {
    let mut buf: *mut BufferPoolEntry = opaque as *mut BufferPoolEntry;
    let mut pool: *mut AVBufferPool = (*buf).pool;
    pthread_mutex_lock(&raw mut (*pool).mutex);
    (*buf).next = (*pool).pool as *mut BufferPoolEntry;
    (*pool).pool = buf;
    pthread_mutex_unlock(&raw mut (*pool).mutex);
    if crate::rt::atomic_xsub_acqrel(
        &raw mut (*pool).refcount,
        1 as ::core::ffi::c_int as ::core::ffi::c_uint,
    ) == 1 as ::core::ffi::c_uint
    {
        buffer_pool_free(pool);
    }
}
unsafe extern "C" fn pool_alloc_buffer(mut pool: *mut AVBufferPool) -> *mut AVBufferRef {
    let mut buf: *mut BufferPoolEntry = ::core::ptr::null_mut::<BufferPoolEntry>();
    let mut ret: *mut AVBufferRef = ::core::ptr::null_mut::<AVBufferRef>();
    if !((*pool).alloc.is_some() || (*pool).alloc2.is_some()) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"pool->alloc || pool->alloc2\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavutil/buffer.c\0" as *const u8 as *const ::core::ffi::c_char,
            366 as ::core::ffi::c_int,
        );
        abort();
    }
    ret = if (*pool).alloc2.is_some() {
        (*pool).alloc2.expect("non-null function pointer")((*pool).opaque, (*pool).size)
    } else {
        (*pool).alloc.expect("non-null function pointer")((*pool).size)
    };
    if ret.is_null() {
        return ::core::ptr::null_mut::<AVBufferRef>();
    }
    buf = av_mallocz(::core::mem::size_of::<BufferPoolEntry>() as size_t)
        as *mut BufferPoolEntry;
    if buf.is_null() {
        av_buffer_unref(&raw mut ret);
        return ::core::ptr::null_mut::<AVBufferRef>();
    }
    (*buf).data = (*(*ret).buffer).data;
    (*buf).opaque = (*(*ret).buffer).opaque;
    (*buf).free = (*(*ret).buffer).free;
    (*buf).pool = pool;
    (*(*ret).buffer).opaque = buf as *mut ::core::ffi::c_void;
    (*(*ret).buffer).free = Some(
        pool_release_buffer
            as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint8_t) -> (),
    ) as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint8_t) -> ()>;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_buffer_pool_get(
    mut pool: *mut AVBufferPool,
) -> *mut AVBufferRef {
    let mut ret: *mut AVBufferRef = ::core::ptr::null_mut::<AVBufferRef>();
    let mut buf: *mut BufferPoolEntry = ::core::ptr::null_mut::<BufferPoolEntry>();
    pthread_mutex_lock(&raw mut (*pool).mutex);
    buf = (*pool).pool;
    if !buf.is_null() {
        memset(
            &raw mut (*buf).buffer as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<AVBuffer>() as size_t,
        );
        ret = buffer_create(
            &raw mut (*buf).buffer,
            (*buf).data,
            (*pool).size,
            Some(
                pool_release_buffer
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint8_t) -> (),
            ),
            buf as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
        );
        if !ret.is_null() {
            (*pool).pool = (*buf).next as *mut BufferPoolEntry;
            (*buf).next = ::core::ptr::null_mut::<BufferPoolEntry>();
            (*buf).buffer.flags_internal
                |= (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
        }
    } else {
        ret = pool_alloc_buffer(pool);
    }
    pthread_mutex_unlock(&raw mut (*pool).mutex);
    if !ret.is_null() {
        crate::rt::atomic_xadd_relaxed(
            &raw mut (*pool).refcount,
            1 as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_buffer_pool_buffer_get_opaque(
    mut ref_0: *const AVBufferRef,
) -> *mut ::core::ffi::c_void {
    let mut buf: *mut BufferPoolEntry = (*(*ref_0).buffer).opaque
        as *mut BufferPoolEntry;
    if buf.is_null() {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"buf\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavutil/buffer.c\0" as *const u8 as *const ::core::ffi::c_char,
            420 as ::core::ffi::c_int,
        );
        abort();
    }
    return (*buf).opaque;
}
