// Translated from FFmpeg 9.0.1 libavutil/slicethread.c
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
    fn abort() -> !;
    fn pthread_create(
        _: *mut pthread_t,
        _: *const pthread_attr_t,
        _: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn pthread_join(
        _: pthread_t,
        _: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn pthread_mutex_init(
        _: *mut pthread_mutex_t,
        _: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_destroy(_: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_cond_init(
        _: *mut pthread_cond_t,
        _: *const pthread_condattr_t,
    ) -> ::core::ffi::c_int;
    fn pthread_cond_destroy(_: *mut pthread_cond_t) -> ::core::ffi::c_int;
    fn pthread_cond_wait(
        _: *mut pthread_cond_t,
        _: *mut pthread_mutex_t,
    ) -> ::core::ffi::c_int;
    fn pthread_cond_signal(_: *mut pthread_cond_t) -> ::core::ffi::c_int;
    fn av_mallocz(size: size_t) -> *mut ::core::ffi::c_void;
    fn av_cpu_count() -> ::core::ffi::c_int;
    fn av_calloc(nmemb: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn av_freep(ptr: *mut ::core::ffi::c_void);
}
pub type size_t = ::core::ffi::c_ulong;
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
pub struct AVSliceThread {
    pub workers: *mut WorkerContext,
    pub nb_threads: ::core::ffi::c_int,
    pub nb_active_threads: ::core::ffi::c_int,
    pub nb_jobs: ::core::ffi::c_int,
    pub first_job: atomic_uint,
    pub current_job: atomic_uint,
    pub done_mutex: pthread_mutex_t,
    pub done_cond: pthread_cond_t,
    pub done: ::core::ffi::c_int,
    pub finished: ::core::ffi::c_int,
    pub priv_0: *mut ::core::ffi::c_void,
    pub worker_func: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub main_func: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pthread_cond_t {
    pub __u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __i: [::core::ffi::c_int; 12],
    pub __vi: [::core::ffi::c_int; 12],
    pub __p: [*mut ::core::ffi::c_void; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pthread_mutex_t {
    pub __u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub __i: [::core::ffi::c_int; 10],
    pub __vi: [::core::ffi::c_int; 10],
    pub __p: [*mut ::core::ffi::c_void; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WorkerContext {
    pub ctx: *mut AVSliceThread,
    pub mutex: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub thread: pthread_t,
    pub done: ::core::ffi::c_int,
}
pub type pthread_t = *mut __pthread;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pthread_attr_t {
    pub __u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub __i: [::core::ffi::c_int; 14],
    pub __vi: [::core::ffi::c_int; 14],
    pub __s: [::core::ffi::c_ulong; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pthread_condattr_t {
    pub __attr: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pthread_mutexattr_t {
    pub __attr: ::core::ffi::c_uint,
}
unsafe extern "C" fn run_jobs(mut ctx: *mut AVSliceThread) -> ::core::ffi::c_int {
    let mut nb_jobs: ::core::ffi::c_uint = (*ctx).nb_jobs as ::core::ffi::c_uint;
    let mut nb_active_threads: ::core::ffi::c_uint = (*ctx).nb_active_threads
        as ::core::ffi::c_uint;
    let mut first_job: ::core::ffi::c_uint = crate::rt::atomic_xadd_acqrel(
        &raw mut (*ctx).first_job,
        1 as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    let mut current_job: ::core::ffi::c_uint = first_job;
    loop {
        (*ctx)
            .worker_func
            .expect(
                "non-null function pointer",
            )(
            (*ctx).priv_0,
            current_job as ::core::ffi::c_int,
            first_job as ::core::ffi::c_int,
            nb_jobs as ::core::ffi::c_int,
            nb_active_threads as ::core::ffi::c_int,
        );
        current_job = crate::rt::atomic_xadd_acqrel(
            &raw mut (*ctx).current_job,
            1 as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        if !(current_job < nb_jobs) {
            break;
        }
    }
    return (current_job
        == nb_jobs
            .wrapping_add(nb_active_threads)
            .wrapping_sub(1 as ::core::ffi::c_uint)) as ::core::ffi::c_int;
}
unsafe extern "C" fn thread_worker(
    mut v: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut w: *mut WorkerContext = v as *mut WorkerContext;
    let mut ctx: *mut AVSliceThread = (*w).ctx;
    pthread_mutex_lock(&raw mut (*w).mutex);
    pthread_cond_signal(&raw mut (*w).cond);
    loop {
        (*w).done = 1 as ::core::ffi::c_int;
        while (*w).done != 0 {
            pthread_cond_wait(&raw mut (*w).cond, &raw mut (*w).mutex);
        }
        if (*ctx).finished != 0 {
            pthread_mutex_unlock(&raw mut (*w).mutex);
            return ::core::ptr::null_mut::<::core::ffi::c_void>();
        }
        if run_jobs(ctx) != 0 {
            pthread_mutex_lock(&raw mut (*ctx).done_mutex);
            (*ctx).done = 1 as ::core::ffi::c_int;
            pthread_cond_signal(&raw mut (*ctx).done_cond);
            pthread_mutex_unlock(&raw mut (*ctx).done_mutex);
        }
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn avpriv_slicethread_create(
    mut pctx: *mut *mut AVSliceThread,
    mut priv_0: *mut ::core::ffi::c_void,
    mut worker_func: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    mut main_func: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    mut nb_threads: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ctx: *mut AVSliceThread = ::core::ptr::null_mut::<AVSliceThread>();
    let mut nb_workers: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    if !(nb_threads >= 0 as ::core::ffi::c_int) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"nb_threads >= 0\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavutil/slicethread.c\0" as *const u8 as *const ::core::ffi::c_char,
            108 as ::core::ffi::c_int,
        );
        abort();
    }
    if nb_threads == 0 {
        let mut nb_cpus: ::core::ffi::c_int = av_cpu_count();
        if nb_cpus > 1 as ::core::ffi::c_int {
            nb_threads = if nb_cpus + 1 as ::core::ffi::c_int > 16 as ::core::ffi::c_int
            {
                16 as ::core::ffi::c_int
            } else {
                nb_cpus + 1 as ::core::ffi::c_int
            };
        } else {
            nb_threads = 1 as ::core::ffi::c_int;
        }
    }
    nb_workers = nb_threads;
    if main_func.is_none() {
        nb_workers -= 1;
    }
    ctx = av_mallocz(::core::mem::size_of::<AVSliceThread>() as size_t)
        as *mut AVSliceThread;
    *pctx = ctx;
    if ctx.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    if nb_workers != 0
        && {
            (*ctx).workers = av_calloc(
                nb_workers as size_t,
                ::core::mem::size_of::<WorkerContext>() as size_t,
            ) as *mut WorkerContext;
            (*ctx).workers.is_null()
        }
    {
        av_freep(pctx as *mut ::core::ffi::c_void);
        return -(12 as ::core::ffi::c_int);
    }
    (*ctx).priv_0 = priv_0;
    (*ctx).worker_func = worker_func;
    (*ctx).main_func = main_func;
    (*ctx).nb_threads = nb_threads;
    (*ctx).nb_active_threads = 0 as ::core::ffi::c_int;
    (*ctx).nb_jobs = 0 as ::core::ffi::c_int;
    (*ctx).finished = 0 as ::core::ffi::c_int;
    *&raw mut (*ctx).first_job = 0 as ::core::ffi::c_int as ::core::ffi::c_uint;
    *&raw mut (*ctx).current_job = 0 as ::core::ffi::c_int as ::core::ffi::c_uint;
    ret = pthread_mutex_init(
        &raw mut (*ctx).done_mutex,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    );
    if ret != 0 {
        av_freep(&raw mut (*ctx).workers as *mut ::core::ffi::c_void);
        av_freep(pctx as *mut ::core::ffi::c_void);
        return -ret;
    }
    ret = pthread_cond_init(
        &raw mut (*ctx).done_cond,
        ::core::ptr::null::<pthread_condattr_t>(),
    );
    if ret != 0 {
        (*ctx).nb_threads = if main_func.is_some() {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        };
        avpriv_slicethread_free(pctx);
        return -ret;
    }
    (*ctx).done = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < nb_workers {
        let mut w: *mut WorkerContext = (*ctx).workers.offset(i as isize)
            as *mut WorkerContext;
        (*w).ctx = ctx;
        ret = pthread_mutex_init(
            &raw mut (*w).mutex,
            ::core::ptr::null::<pthread_mutexattr_t>(),
        );
        if ret != 0 {
            (*ctx).nb_threads = if main_func.is_some() {
                i
            } else {
                i + 1 as ::core::ffi::c_int
            };
            avpriv_slicethread_free(pctx);
            return -ret;
        }
        ret = pthread_cond_init(
            &raw mut (*w).cond,
            ::core::ptr::null::<pthread_condattr_t>(),
        );
        if ret != 0 {
            pthread_mutex_destroy(&raw mut (*w).mutex);
            (*ctx).nb_threads = if main_func.is_some() {
                i
            } else {
                i + 1 as ::core::ffi::c_int
            };
            avpriv_slicethread_free(pctx);
            return -ret;
        }
        pthread_mutex_lock(&raw mut (*w).mutex);
        (*w).done = 0 as ::core::ffi::c_int;
        ret = pthread_create(
            &raw mut (*w).thread,
            ::core::ptr::null::<pthread_attr_t>(),
            Some(
                thread_worker
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                    ) -> *mut ::core::ffi::c_void,
            ),
            w as *mut ::core::ffi::c_void,
        );
        if ret != 0 {
            (*ctx).nb_threads = if main_func.is_some() {
                i
            } else {
                i + 1 as ::core::ffi::c_int
            };
            pthread_mutex_unlock(&raw mut (*w).mutex);
            pthread_cond_destroy(&raw mut (*w).cond);
            pthread_mutex_destroy(&raw mut (*w).mutex);
            avpriv_slicethread_free(pctx);
            return -ret;
        }
        while (*w).done == 0 {
            pthread_cond_wait(&raw mut (*w).cond, &raw mut (*w).mutex);
        }
        pthread_mutex_unlock(&raw mut (*w).mutex);
        i += 1;
    }
    return nb_threads;
}
#[no_mangle]
pub unsafe extern "C" fn avpriv_slicethread_execute(
    mut ctx: *mut AVSliceThread,
    mut nb_jobs: ::core::ffi::c_int,
    mut execute_main: ::core::ffi::c_int,
) {
    let mut nb_workers: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut is_last: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !(nb_jobs > 0 as ::core::ffi::c_int) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"nb_jobs > 0\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavutil/slicethread.c\0" as *const u8 as *const ::core::ffi::c_char,
            194 as ::core::ffi::c_int,
        );
        abort();
    }
    (*ctx).nb_jobs = nb_jobs;
    (*ctx).nb_active_threads = if nb_jobs > (*ctx).nb_threads {
        (*ctx).nb_threads
    } else {
        nb_jobs
    };
    crate::rt::atomic_store_relaxed(
        &raw mut (*ctx).first_job,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    crate::rt::atomic_store_relaxed(
        &raw mut (*ctx).current_job,
        (*ctx).nb_active_threads as ::core::ffi::c_uint,
    );
    nb_workers = (*ctx).nb_active_threads;
    if (*ctx).main_func.is_none() || execute_main == 0 {
        nb_workers -= 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < nb_workers {
        let mut w: *mut WorkerContext = (*ctx).workers.offset(i as isize)
            as *mut WorkerContext;
        pthread_mutex_lock(&raw mut (*w).mutex);
        (*w).done = 0 as ::core::ffi::c_int;
        pthread_cond_signal(&raw mut (*w).cond);
        pthread_mutex_unlock(&raw mut (*w).mutex);
        i += 1;
    }
    if (*ctx).main_func.is_some() && execute_main != 0 {
        (*ctx).main_func.expect("non-null function pointer")((*ctx).priv_0);
    } else {
        is_last = run_jobs(ctx);
    }
    if is_last == 0 {
        pthread_mutex_lock(&raw mut (*ctx).done_mutex);
        while (*ctx).done == 0 {
            pthread_cond_wait(&raw mut (*ctx).done_cond, &raw mut (*ctx).done_mutex);
        }
        (*ctx).done = 0 as ::core::ffi::c_int;
        pthread_mutex_unlock(&raw mut (*ctx).done_mutex);
    }
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn avpriv_slicethread_free(mut pctx: *mut *mut AVSliceThread) {
    let mut ctx: *mut AVSliceThread = *pctx;
    let mut nb_workers: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    if ctx.is_null() {
        return;
    }
    nb_workers = (*ctx).nb_threads;
    if (*ctx).main_func.is_none() {
        nb_workers -= 1;
    }
    (*ctx).finished = 1 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < nb_workers {
        let mut w: *mut WorkerContext = (*ctx).workers.offset(i as isize)
            as *mut WorkerContext;
        pthread_mutex_lock(&raw mut (*w).mutex);
        (*w).done = 0 as ::core::ffi::c_int;
        pthread_cond_signal(&raw mut (*w).cond);
        pthread_mutex_unlock(&raw mut (*w).mutex);
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < nb_workers {
        let mut w_0: *mut WorkerContext = (*ctx).workers.offset(i as isize)
            as *mut WorkerContext;
        pthread_join((*w_0).thread, ::core::ptr::null_mut::<*mut ::core::ffi::c_void>());
        pthread_cond_destroy(&raw mut (*w_0).cond);
        pthread_mutex_destroy(&raw mut (*w_0).mutex);
        i += 1;
    }
    pthread_cond_destroy(&raw mut (*ctx).done_cond);
    pthread_mutex_destroy(&raw mut (*ctx).done_mutex);
    av_freep(&raw mut (*ctx).workers as *mut ::core::ffi::c_void);
    av_freep(pctx as *mut ::core::ffi::c_void);
}
#[repr(C)]
pub struct __pthread {
    _opaque: [u8; 0],
}
