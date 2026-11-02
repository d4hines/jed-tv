// Translated from FFmpeg 9.0.1 libavutil/refstruct.c
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
    fn pthread_mutex_init(
        _: *mut pthread_mutex_t,
        _: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_destroy(_: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn av_malloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn av_free(ptr: *mut ::core::ffi::c_void);
}
pub type size_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::core::ffi::c_longlong,
    pub __clang_max_align_nonce2: crate::rt::F128,
}
pub type uintptr_t = ::core::ffi::c_ulong;
pub type memory_order = ::core::ffi::c_uint;
pub const memory_order_seq_cst: memory_order = 5;
pub const memory_order_acq_rel: memory_order = 4;
pub const memory_order_release: memory_order = 3;
pub const memory_order_acquire: memory_order = 2;
pub const memory_order_consume: memory_order = 1;
pub const memory_order_relaxed: memory_order = 0;
pub type atomic_uintptr_t = uintptr_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union AVRefStructOpaque {
    pub nc: *mut ::core::ffi::c_void,
    pub c: *const ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RefCount {
    pub refcount: atomic_uintptr_t,
    pub opaque: AVRefStructOpaque,
    pub free_cb: Option<
        unsafe extern "C" fn(AVRefStructOpaque, *mut ::core::ffi::c_void) -> (),
    >,
    pub free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRefStructPool {
    pub size: size_t,
    pub opaque: AVRefStructOpaque,
    pub init_cb: Option<
        unsafe extern "C" fn(
            AVRefStructOpaque,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub reset_cb: Option<
        unsafe extern "C" fn(AVRefStructOpaque, *mut ::core::ffi::c_void) -> (),
    >,
    pub free_entry_cb: Option<
        unsafe extern "C" fn(AVRefStructOpaque, *mut ::core::ffi::c_void) -> (),
    >,
    pub free_cb: Option<unsafe extern "C" fn(AVRefStructOpaque) -> ()>,
    pub uninited: ::core::ffi::c_int,
    pub entry_flags: ::core::ffi::c_uint,
    pub pool_flags: ::core::ffi::c_uint,
    pub refcount: atomic_uintptr_t,
    pub available_entries: *mut RefCount,
    pub mutex: pthread_mutex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pthread_mutex_t {
    pub __u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __i: [::core::ffi::c_int; 10],
    pub __vi: [::core::ffi::c_int; 10],
    pub __p: [*mut ::core::ffi::c_void; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pthread_mutexattr_t {
    pub __attr: ::core::ffi::c_uint,
}
unsafe extern "C" fn get_refcount(mut obj: *mut ::core::ffi::c_void) -> *mut RefCount {
    let mut ref_0: *mut RefCount = (obj as *mut ::core::ffi::c_char)
        .offset(
            -(((::core::mem::size_of::<RefCount>() as usize)
                .wrapping_add(
                    (if 16 as usize > ::core::mem::align_of::<max_align_t>() {
                        16 as usize
                    } else {
                        ::core::mem::align_of::<max_align_t>()
                    }),
                )
                .wrapping_sub(1 as usize)
                & !(if 16 as usize > ::core::mem::align_of::<max_align_t>() {
                    16 as usize
                } else {
                    ::core::mem::align_of::<max_align_t>()
                })
                    .wrapping_sub(1 as usize)) as isize),
        ) as *mut RefCount;
    return ref_0;
}
unsafe extern "C" fn cget_refcount(
    mut obj: *const ::core::ffi::c_void,
) -> *const RefCount {
    let mut ref_0: *const RefCount = (obj as *const ::core::ffi::c_char)
        .offset(
            -(((::core::mem::size_of::<RefCount>() as usize)
                .wrapping_add(
                    (if 16 as usize > ::core::mem::align_of::<max_align_t>() {
                        16 as usize
                    } else {
                        ::core::mem::align_of::<max_align_t>()
                    }),
                )
                .wrapping_sub(1 as usize)
                & !(if 16 as usize > ::core::mem::align_of::<max_align_t>() {
                    16 as usize
                } else {
                    ::core::mem::align_of::<max_align_t>()
                })
                    .wrapping_sub(1 as usize)) as isize),
        ) as *const RefCount;
    return ref_0;
}
unsafe extern "C" fn get_userdata(
    mut buf: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    return (buf as *mut ::core::ffi::c_char)
        .offset(
            ((::core::mem::size_of::<RefCount>() as usize)
                .wrapping_add(
                    (if 16 as usize > ::core::mem::align_of::<max_align_t>() {
                        16 as usize
                    } else {
                        ::core::mem::align_of::<max_align_t>()
                    }),
                )
                .wrapping_sub(1 as usize)
                & !(if 16 as usize > ::core::mem::align_of::<max_align_t>() {
                    16 as usize
                } else {
                    ::core::mem::align_of::<max_align_t>()
                })
                    .wrapping_sub(1 as usize)) as isize,
        ) as *mut ::core::ffi::c_void;
}
unsafe extern "C" fn refcount_init(
    mut ref_0: *mut RefCount,
    mut opaque: AVRefStructOpaque,
    mut free_cb: Option<
        unsafe extern "C" fn(AVRefStructOpaque, *mut ::core::ffi::c_void) -> (),
    >,
) {
    *&raw mut (*ref_0).refcount = 1 as ::core::ffi::c_int as uintptr_t;
    (*ref_0).opaque = opaque;
    (*ref_0).free_cb = free_cb;
    (*ref_0).free = Some(av_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ())
        as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
}
#[inline]
unsafe extern "C" fn av_refstruct_alloc_ext(
    mut size: size_t,
    mut flags: ::core::ffi::c_uint,
    mut opaque: *mut ::core::ffi::c_void,
    mut free_cb: Option<
        unsafe extern "C" fn(AVRefStructOpaque, *mut ::core::ffi::c_void) -> (),
    >,
) -> *mut ::core::ffi::c_void {
    return av_refstruct_alloc_ext_c(
        size,
        flags,
        AVRefStructOpaque { nc: opaque },
        free_cb,
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_refstruct_alloc_ext_c(
    mut size: size_t,
    mut flags: ::core::ffi::c_uint,
    mut opaque: AVRefStructOpaque,
    mut free_cb: Option<
        unsafe extern "C" fn(AVRefStructOpaque, *mut ::core::ffi::c_void) -> (),
    >,
) -> *mut ::core::ffi::c_void {
    let mut buf: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut obj: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    if size as usize
        > (0xffffffffffffffff as usize)
            .wrapping_sub(
                (::core::mem::size_of::<RefCount>() as usize)
                    .wrapping_add(
                        (if 16 as usize > ::core::mem::align_of::<max_align_t>() {
                            16 as usize
                        } else {
                            ::core::mem::align_of::<max_align_t>()
                        }),
                    )
                    .wrapping_sub(1 as usize)
                    & !(if 16 as usize > ::core::mem::align_of::<max_align_t>() {
                        16 as usize
                    } else {
                        ::core::mem::align_of::<max_align_t>()
                    })
                        .wrapping_sub(1 as usize),
            )
    {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    buf = av_malloc(
        size
            .wrapping_add(
                (::core::mem::size_of::<RefCount>() as size_t)
                    .wrapping_add(
                        (if 16 as usize > ::core::mem::align_of::<max_align_t>() {
                            16 as size_t
                        } else {
                            ::core::mem::align_of::<max_align_t>() as size_t
                        }),
                    )
                    .wrapping_sub(1 as size_t)
                    & !(if 16 as usize > ::core::mem::align_of::<max_align_t>() {
                        16 as size_t
                    } else {
                        ::core::mem::align_of::<max_align_t>() as size_t
                    })
                        .wrapping_sub(1 as size_t),
            ),
    );
    if buf.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    refcount_init(buf as *mut RefCount, opaque, free_cb);
    obj = get_userdata(buf);
    if flags
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as ::core::ffi::c_uint
        == 0
    {
        memset(obj, 0 as ::core::ffi::c_int, size);
    }
    return obj;
}
#[no_mangle]
pub unsafe extern "C" fn av_refstruct_unref(mut objp: *mut ::core::ffi::c_void) {
    let mut obj: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut ref_0: *mut RefCount = ::core::ptr::null_mut::<RefCount>();
    memcpy(
        &raw mut obj as *mut ::core::ffi::c_void,
        objp,
        ::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t,
    );
    if obj.is_null() {
        return;
    }
    let mut fresh0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    memcpy(
        objp,
        &raw mut fresh0 as *const ::core::ffi::c_void,
        ::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t,
    );
    ref_0 = get_refcount(obj);
    if crate::rt::atomic_xsub_acqrel(
        &raw mut (*ref_0).refcount,
        1 as ::core::ffi::c_int as uintptr_t,
    ) == 1 as uintptr_t
    {
        if (*ref_0).free_cb.is_some() {
            (*ref_0).free_cb.expect("non-null function pointer")((*ref_0).opaque, obj);
        }
        (*ref_0)
            .free
            .expect("non-null function pointer")(ref_0 as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn av_refstruct_ref(
    mut obj: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut ref_0: *mut RefCount = get_refcount(obj);
    crate::rt::atomic_xadd_relaxed(
        &raw mut (*ref_0).refcount,
        1 as ::core::ffi::c_int as uintptr_t,
    );
    return obj;
}
#[no_mangle]
pub unsafe extern "C" fn av_refstruct_ref_c(
    mut obj: *const ::core::ffi::c_void,
) -> *const ::core::ffi::c_void {
    let mut ref_0: *mut RefCount = get_refcount(obj as *mut ::core::ffi::c_void);
    crate::rt::atomic_xadd_relaxed(
        &raw mut (*ref_0).refcount,
        1 as ::core::ffi::c_int as uintptr_t,
    );
    return obj;
}
#[no_mangle]
pub unsafe extern "C" fn av_refstruct_replace(
    mut dstp: *mut ::core::ffi::c_void,
    mut src: *const ::core::ffi::c_void,
) {
    let mut dst: *const ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>();
    memcpy(
        &raw mut dst as *mut ::core::ffi::c_void,
        dstp,
        ::core::mem::size_of::<*const ::core::ffi::c_void>() as size_t,
    );
    if src == dst {
        return;
    }
    av_refstruct_unref(dstp);
    if !src.is_null() {
        dst = av_refstruct_ref_c(src);
        memcpy(
            dstp,
            &raw mut dst as *const ::core::ffi::c_void,
            ::core::mem::size_of::<*const ::core::ffi::c_void>() as size_t,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn av_refstruct_exclusive(
    mut obj: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ref_0: *const RefCount = cget_refcount(obj);
    return (crate::rt::atomic_load_acquire(
        &raw const (*ref_0).refcount as *mut atomic_uintptr_t,
    ) == 1 as uintptr_t) as ::core::ffi::c_int;
}
unsafe extern "C" fn pool_free(mut pool: *mut AVRefStructPool) {
    pthread_mutex_destroy(&raw mut (*pool).mutex);
    if (*pool).free_cb.is_some() {
        (*pool).free_cb.expect("non-null function pointer")((*pool).opaque);
    }
    av_free(get_refcount(pool as *mut ::core::ffi::c_void) as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn pool_free_entry(
    mut pool: *mut AVRefStructPool,
    mut ref_0: *mut RefCount,
) {
    if (*pool).free_entry_cb.is_some() {
        (*pool)
            .free_entry_cb
            .expect(
                "non-null function pointer",
            )((*pool).opaque, get_userdata(ref_0 as *mut ::core::ffi::c_void));
    }
    av_free(ref_0 as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn pool_return_entry(mut ref_: *mut ::core::ffi::c_void) {
    let mut ref_0: *mut RefCount = ref_ as *mut RefCount;
    let mut pool: *mut AVRefStructPool = (*ref_0).opaque.nc as *mut AVRefStructPool;
    pthread_mutex_lock(&raw mut (*pool).mutex);
    if (*pool).uninited == 0 {
        (*ref_0).opaque.nc = (*pool).available_entries as *mut ::core::ffi::c_void;
        (*pool).available_entries = ref_0;
        ref_0 = ::core::ptr::null_mut::<RefCount>();
    }
    pthread_mutex_unlock(&raw mut (*pool).mutex);
    if !ref_0.is_null() {
        pool_free_entry(pool, ref_0);
    }
    if crate::rt::atomic_xsub_acqrel(
        &raw mut (*pool).refcount,
        1 as ::core::ffi::c_int as uintptr_t,
    ) == 1 as uintptr_t
    {
        pool_free(pool);
    }
}
unsafe extern "C" fn pool_reset_entry(
    mut opaque: AVRefStructOpaque,
    mut entry: *mut ::core::ffi::c_void,
) {
    let mut pool: *mut AVRefStructPool = opaque.nc as *mut AVRefStructPool;
    (*pool).reset_cb.expect("non-null function pointer")((*pool).opaque, entry);
}
unsafe extern "C" fn refstruct_pool_get_ext(
    mut datap: *mut ::core::ffi::c_void,
    mut pool: *mut AVRefStructPool,
) -> ::core::ffi::c_int {
    let mut ret: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut fresh2: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    memcpy(
        datap,
        &raw mut fresh2 as *const ::core::ffi::c_void,
        ::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t,
    );
    pthread_mutex_lock(&raw mut (*pool).mutex);
    if !(*pool).available_entries.is_null() {
        let mut ref_0: *mut RefCount = (*pool).available_entries;
        ret = get_userdata(ref_0 as *mut ::core::ffi::c_void);
        (*pool).available_entries = (*ref_0).opaque.nc as *mut RefCount;
        (*ref_0).opaque.nc = pool as *mut ::core::ffi::c_void;
        *&raw mut (*ref_0).refcount = 1 as ::core::ffi::c_int as uintptr_t;
    }
    pthread_mutex_unlock(&raw mut (*pool).mutex);
    if ret.is_null() {
        let mut ref_1: *mut RefCount = ::core::ptr::null_mut::<RefCount>();
        ret = av_refstruct_alloc_ext(
            (*pool).size,
            (*pool).entry_flags,
            pool as *mut ::core::ffi::c_void,
            if (*pool).reset_cb.is_some() {
                Some(
                    pool_reset_entry
                        as unsafe extern "C" fn(
                            AVRefStructOpaque,
                            *mut ::core::ffi::c_void,
                        ) -> (),
                )
            } else {
                None
            },
        );
        if ret.is_null() {
            return -(12 as ::core::ffi::c_int);
        }
        ref_1 = get_refcount(ret);
        (*ref_1).free = Some(
            pool_return_entry as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
        ) as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
        if (*pool).init_cb.is_some() {
            let mut err: ::core::ffi::c_int = (*pool)
                .init_cb
                .expect("non-null function pointer")((*pool).opaque, ret);
            if err < 0 as ::core::ffi::c_int {
                if (*pool).pool_flags
                    & ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint != 0
                {
                    (*pool)
                        .reset_cb
                        .expect("non-null function pointer")((*pool).opaque, ret);
                }
                if (*pool).pool_flags
                    & ((1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint != 0
                {
                    (*pool)
                        .free_entry_cb
                        .expect("non-null function pointer")((*pool).opaque, ret);
                }
                av_free(ref_1 as *mut ::core::ffi::c_void);
                return err;
            }
        }
    }
    crate::rt::atomic_xadd_relaxed(
        &raw mut (*pool).refcount,
        1 as ::core::ffi::c_int as uintptr_t,
    );
    if (*pool).pool_flags
        & ((1 as ::core::ffi::c_int) << 18 as ::core::ffi::c_int) as ::core::ffi::c_uint
        != 0
    {
        memset(ret, 0 as ::core::ffi::c_int, (*pool).size);
    }
    memcpy(
        datap,
        &raw mut ret as *const ::core::ffi::c_void,
        ::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t,
    );
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn av_refstruct_pool_alloc_ext(
    mut size: size_t,
    mut flags: ::core::ffi::c_uint,
    mut opaque: *mut ::core::ffi::c_void,
    mut init_cb: Option<
        unsafe extern "C" fn(
            AVRefStructOpaque,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    mut reset_cb: Option<
        unsafe extern "C" fn(AVRefStructOpaque, *mut ::core::ffi::c_void) -> (),
    >,
    mut free_entry_cb: Option<
        unsafe extern "C" fn(AVRefStructOpaque, *mut ::core::ffi::c_void) -> (),
    >,
    mut free_cb: Option<unsafe extern "C" fn(AVRefStructOpaque) -> ()>,
) -> *mut AVRefStructPool {
    return av_refstruct_pool_alloc_ext_c(
        size,
        flags,
        AVRefStructOpaque { nc: opaque },
        init_cb,
        reset_cb,
        free_entry_cb,
        free_cb,
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_refstruct_pool_get(
    mut pool: *mut AVRefStructPool,
) -> *mut ::core::ffi::c_void {
    let mut ret: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    refstruct_pool_get_ext(&raw mut ret as *mut ::core::ffi::c_void, pool);
    return ret;
}
unsafe extern "C" fn pool_unref(mut ref_0: *mut ::core::ffi::c_void) {
    let mut pool: *mut AVRefStructPool = get_userdata(ref_0) as *mut AVRefStructPool;
    if crate::rt::atomic_xsub_acqrel(
        &raw mut (*pool).refcount,
        1 as ::core::ffi::c_int as uintptr_t,
    ) == 1 as uintptr_t
    {
        pool_free(pool);
    }
}
unsafe extern "C" fn refstruct_pool_uninit(
    mut unused: AVRefStructOpaque,
    mut obj: *mut ::core::ffi::c_void,
) {
    let mut pool: *mut AVRefStructPool = obj as *mut AVRefStructPool;
    let mut entry: *mut RefCount = ::core::ptr::null_mut::<RefCount>();
    pthread_mutex_lock(&raw mut (*pool).mutex);
    (*pool).uninited = 1 as ::core::ffi::c_int;
    entry = (*pool).available_entries;
    (*pool).available_entries = ::core::ptr::null_mut::<RefCount>();
    pthread_mutex_unlock(&raw mut (*pool).mutex);
    while !entry.is_null() {
        let mut next: *mut ::core::ffi::c_void = (*entry).opaque.nc;
        pool_free_entry(pool, entry);
        entry = next as *mut RefCount;
    }
}
#[no_mangle]
pub unsafe extern "C" fn av_refstruct_pool_alloc(
    mut size: size_t,
    mut flags: ::core::ffi::c_uint,
) -> *mut AVRefStructPool {
    return av_refstruct_pool_alloc_ext(
        size,
        flags,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        None,
        None,
        None,
        None,
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_refstruct_pool_alloc_ext_c(
    mut size: size_t,
    mut flags: ::core::ffi::c_uint,
    mut opaque: AVRefStructOpaque,
    mut init_cb: Option<
        unsafe extern "C" fn(
            AVRefStructOpaque,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    mut reset_cb: Option<
        unsafe extern "C" fn(AVRefStructOpaque, *mut ::core::ffi::c_void) -> (),
    >,
    mut free_entry_cb: Option<
        unsafe extern "C" fn(AVRefStructOpaque, *mut ::core::ffi::c_void) -> (),
    >,
    mut free_cb: Option<unsafe extern "C" fn(AVRefStructOpaque) -> ()>,
) -> *mut AVRefStructPool {
    let mut pool: *mut AVRefStructPool = av_refstruct_alloc_ext(
        ::core::mem::size_of::<AVRefStructPool>() as size_t,
        0 as ::core::ffi::c_uint,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        Some(
            refstruct_pool_uninit
                as unsafe extern "C" fn(
                    AVRefStructOpaque,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
    ) as *mut AVRefStructPool;
    let mut err: ::core::ffi::c_int = 0;
    if pool.is_null() {
        return ::core::ptr::null_mut::<AVRefStructPool>();
    }
    let ref mut fresh1 = (*get_refcount(pool as *mut ::core::ffi::c_void)).free;
    *fresh1 = Some(pool_unref as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ())
        as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    (*pool).size = size;
    (*pool).opaque = opaque;
    (*pool).init_cb = init_cb;
    (*pool).reset_cb = reset_cb;
    (*pool).free_entry_cb = free_entry_cb;
    (*pool).free_cb = free_cb;
    (*pool).entry_flags = flags
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    if (*pool).reset_cb.is_none() {
        flags
            &= !((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
                as ::core::ffi::c_uint;
    }
    if (*pool).free_entry_cb.is_none() {
        flags
            &= !((1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int)
                as ::core::ffi::c_uint;
    }
    (*pool).pool_flags = flags;
    if flags
        & ((1 as ::core::ffi::c_int) << 18 as ::core::ffi::c_int) as ::core::ffi::c_uint
        != 0
    {
        (*pool).entry_flags
            |= ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                as ::core::ffi::c_uint;
    }
    *&raw mut (*pool).refcount = 1 as ::core::ffi::c_int as uintptr_t;
    err = pthread_mutex_init(
        &raw mut (*pool).mutex,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    );
    if err != 0 {
        av_free(
            get_refcount(pool as *mut ::core::ffi::c_void) as *mut ::core::ffi::c_void,
        );
        return ::core::ptr::null_mut::<AVRefStructPool>();
    }
    return pool;
}
