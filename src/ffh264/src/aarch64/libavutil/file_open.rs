// Translated from FFmpeg 9.0.1 libavutil/file_open.c
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
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn fcntl(_: ::core::ffi::c_int, _: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn open(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn mkstemp(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn av_malloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn fdopen(_: ::core::ffi::c_int, _: *const ::core::ffi::c_char) -> *mut FILE;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn av_default_item_name(ctx: *mut ::core::ffi::c_void) -> *const ::core::ffi::c_char;
    fn av_freep(ptr: *mut ::core::ffi::c_void);
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
pub type size_t = ::core::ffi::c_ulong;
pub type FILE = _IO_FILE;
pub type va_list = __builtin_va_list;
pub type AVClassCategory = ::core::ffi::c_uint;
pub const AV_CLASS_CATEGORY_NB: AVClassCategory = 46;
pub const AV_CLASS_CATEGORY_DEVICE_INPUT: AVClassCategory = 45;
pub const AV_CLASS_CATEGORY_DEVICE_OUTPUT: AVClassCategory = 44;
pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_INPUT: AVClassCategory = 43;
pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_OUTPUT: AVClassCategory = 42;
pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_INPUT: AVClassCategory = 41;
pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_OUTPUT: AVClassCategory = 40;
pub const AV_CLASS_CATEGORY_HWDEVICE: AVClassCategory = 11;
pub const AV_CLASS_CATEGORY_SWRESAMPLER: AVClassCategory = 10;
pub const AV_CLASS_CATEGORY_SWSCALER: AVClassCategory = 9;
pub const AV_CLASS_CATEGORY_BITSTREAM_FILTER: AVClassCategory = 8;
pub const AV_CLASS_CATEGORY_FILTER: AVClassCategory = 7;
pub const AV_CLASS_CATEGORY_DECODER: AVClassCategory = 6;
pub const AV_CLASS_CATEGORY_ENCODER: AVClassCategory = 5;
pub const AV_CLASS_CATEGORY_DEMUXER: AVClassCategory = 4;
pub const AV_CLASS_CATEGORY_MUXER: AVClassCategory = 3;
pub const AV_CLASS_CATEGORY_OUTPUT: AVClassCategory = 2;
pub const AV_CLASS_CATEGORY_INPUT: AVClassCategory = 1;
pub const AV_CLASS_CATEGORY_NA: AVClassCategory = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVClass {
    pub class_name: *const ::core::ffi::c_char,
    pub item_name: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const ::core::ffi::c_char,
    >,
    pub option: *const AVOption,
    pub version: ::core::ffi::c_int,
    pub log_level_offset_offset: ::core::ffi::c_int,
    pub parent_log_context_offset: ::core::ffi::c_int,
    pub category: AVClassCategory,
    pub get_category: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> AVClassCategory,
    >,
    pub query_ranges: Option<
        unsafe extern "C" fn(
            *mut *mut AVOptionRanges,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub child_next: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub child_class_iterate: Option<
        unsafe extern "C" fn(*mut *mut ::core::ffi::c_void) -> *const AVClass,
    >,
    pub state_flags_offset: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FileLogContext {
    pub class: *const AVClass,
    pub log_offset: ::core::ffi::c_int,
    pub log_ctx: *mut ::core::ffi::c_void,
}
extern "C" {
    // asm/varargs-aarch64.S: builds the va_list, calls avpriv_open__va
    pub fn avpriv_open(
        filename: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn avpriv_open__va(
    mut filename: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
    mut args: crate::rt::VaList,
) -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = 0;
    let mut mode: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut ap: crate::rt::VaList;
    ap = args.clone();
    if flags & 0o100 as ::core::ffi::c_int != 0 {
        mode = ap.arg::<::core::ffi::c_uint>();
    }
    flags |= 0o2000000 as ::core::ffi::c_int;
    fd = open(filename, flags, mode);
    if fd != -(1 as ::core::ffi::c_int) {
        if fcntl(fd, 2 as ::core::ffi::c_int, 1 as ::core::ffi::c_int)
            == -(1 as ::core::ffi::c_int)
        {
            av_log(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                48 as ::core::ffi::c_int,
                b"Failed to set close on exec\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    }
    return fd;
}
static mut file_log_ctx_class: AVClass = unsafe {
    AVClass {
        class_name: b"TEMPFILE\0" as *const u8 as *const ::core::ffi::c_char,
        item_name: Some(
            av_default_item_name
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                ) -> *const ::core::ffi::c_char,
        ),
        option: ::core::ptr::null::<AVOption>(),
        version: (61 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            | 101 as ::core::ffi::c_int,
        log_level_offset_offset: 8 as ::core::ffi::c_ulong as ::core::ffi::c_int,
        parent_log_context_offset: 16 as ::core::ffi::c_ulong as ::core::ffi::c_int,
        category: AV_CLASS_CATEGORY_NA,
        get_category: None,
        query_ranges: None,
        child_next: None,
        child_class_iterate: None,
        state_flags_offset: 0,
    }
};
#[no_mangle]
pub unsafe extern "C" fn avpriv_tempfile(
    mut prefix: *const ::core::ffi::c_char,
    mut filename: *mut *mut ::core::ffi::c_char,
    mut log_offset: ::core::ffi::c_int,
    mut log_ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut file_log_ctx: FileLogContext = FileLogContext {
        class: &raw const file_log_ctx_class,
        log_offset: log_offset,
        log_ctx: log_ctx,
    };
    let mut fd: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut len: size_t = (strlen(prefix) as size_t).wrapping_add(12 as size_t);
    *filename = av_malloc(len) as *mut ::core::ffi::c_char;
    if (*filename).is_null() {
        av_log(
            &raw mut file_log_ctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"ff_tempfile: Cannot allocate file name\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(12 as ::core::ffi::c_int);
    }
    snprintf(
        *filename,
        len as size_t,
        b"/tmp/%sXXXXXX\0" as *const u8 as *const ::core::ffi::c_char,
        prefix,
    );
    fd = mkstemp(*filename);
    if fd < 0 as ::core::ffi::c_int {
        let mut err: ::core::ffi::c_int = -*__errno_location();
        av_log(
            &raw mut file_log_ctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"ff_tempfile: Cannot open temporary file %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            *filename,
        );
        av_freep(filename as *mut ::core::ffi::c_void);
        return err;
    }
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn avpriv_fopen_utf8(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> *mut FILE {
    let mut fd: ::core::ffi::c_int = 0;
    let mut access: ::core::ffi::c_int = 0;
    let mut m: *const ::core::ffi::c_char = mode;
    let fresh0 = m;
    m = m.offset(1);
    match *fresh0 as ::core::ffi::c_int {
        114 => {
            access = 0 as ::core::ffi::c_int;
        }
        119 => {
            access = 0o100 as ::core::ffi::c_int | 0o1 as ::core::ffi::c_int
                | 0o1000 as ::core::ffi::c_int;
        }
        97 => {
            access = 0o100 as ::core::ffi::c_int | 0o1 as ::core::ffi::c_int
                | 0o2000 as ::core::ffi::c_int;
        }
        _ => {
            *__errno_location() = 22 as ::core::ffi::c_int;
            return ::core::ptr::null_mut::<FILE>();
        }
    }
    while *m != 0 {
        if *m as ::core::ffi::c_int == '+' as i32 {
            access &= !(0 as ::core::ffi::c_int | 0o1 as ::core::ffi::c_int);
            access |= 0o2 as ::core::ffi::c_int;
        } else if !(*m as ::core::ffi::c_int == 'b' as i32) {
            if *m != 0 {
                *__errno_location() = 22 as ::core::ffi::c_int;
                return ::core::ptr::null_mut::<FILE>();
            }
        }
        m = m.offset(1);
    }
    fd = avpriv_open(path, access, 0o666 as ::core::ffi::c_int);
    if fd == -(1 as ::core::ffi::c_int) {
        return ::core::ptr::null_mut::<FILE>();
    }
    return fdopen(fd, mode);
}
#[repr(C)]
pub struct _IO_FILE {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVOptionRanges {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVOption {
    _opaque: [u8; 0],
}
