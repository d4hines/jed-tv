// Translated from FFmpeg 9.0.1 libavutil/arm/cpu.c
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
    fn av_strstart(
        str: *const ::core::ffi::c_char,
        pfx: *const ::core::ffi::c_char,
        ptr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strstr(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn ff_getauxval(type_0: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    fn fopen(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char) -> *mut FILE;
    fn fclose(_: *mut FILE) -> ::core::ffi::c_int;
    fn fread(
        _: *mut ::core::ffi::c_void,
        _: size_t,
        _: size_t,
        _: *mut FILE,
    ) -> ::core::ffi::c_uint;
    fn fgets(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn av_get_cpu_flags() -> ::core::ffi::c_int;
}
pub type size_t = ::core::ffi::c_uint;
pub type uint32_t = ::core::ffi::c_uint;
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub a_type: uint32_t,
    pub a_val: uint32_t,
}
unsafe extern "C" fn get_auxval(mut hwcap: *mut uint32_t) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_ulong = ff_getauxval(16 as ::core::ffi::c_ulong);
    if ret == 0 as ::core::ffi::c_ulong {
        return -(1 as ::core::ffi::c_int);
    }
    *hwcap = ret as uint32_t;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn get_hwcap(mut hwcap: *mut uint32_t) -> ::core::ffi::c_int {
    let mut auxv: C2RustUnnamed = C2RustUnnamed {
        a_type: 0,
        a_val: 0,
    };
    let mut f: *mut FILE = fopen(
        b"/proc/self/auxv\0" as *const u8 as *const ::core::ffi::c_char,
        b"r\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    let mut err: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if f.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    while fread(
        &raw mut auxv as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<C2RustUnnamed>() as size_t,
        1 as size_t,
        f,
    ) > 0 as ::core::ffi::c_uint
    {
        if !(auxv.a_type == 16 as uint32_t) {
            continue;
        }
        *hwcap = auxv.a_val;
        err = 0 as ::core::ffi::c_int;
        break;
    }
    fclose(f);
    return err;
}
unsafe extern "C" fn get_cpuinfo(mut hwcap: *mut uint32_t) -> ::core::ffi::c_int {
    let mut f: *mut FILE = fopen(
        b"/proc/cpuinfo\0" as *const u8 as *const ::core::ffi::c_char,
        b"r\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    let mut buf: [::core::ffi::c_char; 200] = [0; 200];
    if f.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    *hwcap = 0 as uint32_t;
    while !fgets(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 200]>() as ::core::ffi::c_int,
            f,
        )
        .is_null()
    {
        if !(av_strstart(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"Features\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        ) != 0)
        {
            continue;
        }
        if !strstr(
                &raw mut buf as *mut ::core::ffi::c_char,
                b" edsp \0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null()
        {
            *hwcap |= ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint32_t;
        }
        if !strstr(
                &raw mut buf as *mut ::core::ffi::c_char,
                b" tls \0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null()
        {
            *hwcap
                |= ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as uint32_t;
        }
        if !strstr(
                &raw mut buf as *mut ::core::ffi::c_char,
                b" thumbee \0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null()
        {
            *hwcap
                |= ((1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int) as uint32_t;
        }
        if !strstr(
                &raw mut buf as *mut ::core::ffi::c_char,
                b" vfp \0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null()
        {
            *hwcap |= ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as uint32_t;
        }
        if !strstr(
                &raw mut buf as *mut ::core::ffi::c_char,
                b" vfpv3 \0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null()
        {
            *hwcap
                |= ((1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as uint32_t;
        }
        if !strstr(
                &raw mut buf as *mut ::core::ffi::c_char,
                b" neon \0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null()
            || !strstr(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    b" asimd \0" as *const u8 as *const ::core::ffi::c_char,
                )
                .is_null()
        {
            *hwcap
                |= ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int) as uint32_t;
        }
        if !strstr(
                &raw mut buf as *mut ::core::ffi::c_char,
                b" fp \0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null()
        {
            *hwcap
                |= ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as uint32_t;
        }
        break;
    }
    fclose(f);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_get_cpu_flags_arm() -> ::core::ffi::c_int {
    let mut flags: ::core::ffi::c_int = ((1 as ::core::ffi::c_int)
        << 0 as ::core::ffi::c_int)
        * (1 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0)
            as ::core::ffi::c_int
        | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
            * (1 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0)
                as ::core::ffi::c_int
        | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            * (1 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0)
                as ::core::ffi::c_int
        | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            * (1 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0)
                as ::core::ffi::c_int
        | ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
            * (1 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0)
                as ::core::ffi::c_int
        | ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
            * (1 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0)
                as ::core::ffi::c_int;
    let mut hwcap: uint32_t = 0;
    if get_auxval(&raw mut hwcap) < 0 as ::core::ffi::c_int {
        if get_hwcap(&raw mut hwcap) < 0 as ::core::ffi::c_int {
            if get_cpuinfo(&raw mut hwcap) < 0 as ::core::ffi::c_int {
                return flags;
            }
        }
    }
    if hwcap & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint32_t != 0 {
        flags |= (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
    }
    if hwcap & ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as uint32_t != 0 {
        flags |= (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
    }
    if hwcap & ((1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int) as uint32_t != 0 {
        flags |= (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
    }
    if hwcap & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as uint32_t != 0 {
        flags |= (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
    }
    if hwcap & ((1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as uint32_t != 0 {
        flags |= (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
    }
    if hwcap & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int) as uint32_t != 0 {
        flags |= (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int;
    }
    if flags
        & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) != 0
    {
        flags |= (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
    } else if flags
        & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) != 0
    {
        flags |= (1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int;
    }
    if flags & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0 {
        flags |= (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
    }
    if flags & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
        && flags
            & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) == 0
    {
        flags |= (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int;
    }
    return flags;
}
#[no_mangle]
pub unsafe extern "C" fn ff_get_cpu_max_align_arm() -> size_t {
    let mut flags: ::core::ffi::c_int = av_get_cpu_flags();
    if flags & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0 {
        return 16 as size_t;
    }
    return 8 as size_t;
}
#[repr(C)]
pub struct _IO_FILE {
    _opaque: [u8; 0],
}
