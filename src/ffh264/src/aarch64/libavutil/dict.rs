// Translated from FFmpeg 9.0.1 libavutil/dict.c
// (preprocessed for aarch64-unknown-linux-musl) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// copyright (c) 2009 Michael Niedermayer
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
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strspn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn av_mallocz(size: size_t) -> *mut ::core::ffi::c_void;
    fn av_bprint_init(
        buf: *mut AVBPrint,
        size_init: ::core::ffi::c_uint,
        size_max: ::core::ffi::c_uint,
    );
    fn av_get_token(
        buf: *mut *const ::core::ffi::c_char,
        term: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn av_bprint_append_data(
        buf: *mut AVBPrint,
        data: *const ::core::ffi::c_char,
        size: ::core::ffi::c_uint,
    );
    fn av_realloc(
        ptr: *mut ::core::ffi::c_void,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn av_bprint_finalize(
        buf: *mut AVBPrint,
        ret_str: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn av_realloc_array(
        ptr: *mut ::core::ffi::c_void,
        nmemb: size_t,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn av_bprint_escape(
        dstbuf: *mut AVBPrint,
        src: *const ::core::ffi::c_char,
        special_chars: *const ::core::ffi::c_char,
        mode: AVEscapeMode,
        flags: ::core::ffi::c_int,
    );
    fn av_free(ptr: *mut ::core::ffi::c_void);
    fn av_freep(ptr: *mut ::core::ffi::c_void);
    fn av_strdup(s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}
pub type int64_t = ::core::ffi::c_long;
pub type size_t = ::core::ffi::c_ulong;
pub type AVEscapeMode = ::core::ffi::c_uint;
pub const AV_ESCAPE_MODE_XML: AVEscapeMode = 3;
pub const AV_ESCAPE_MODE_QUOTE: AVEscapeMode = 2;
pub const AV_ESCAPE_MODE_BACKSLASH: AVEscapeMode = 1;
pub const AV_ESCAPE_MODE_AUTO: AVEscapeMode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDictionaryEntry {
    pub key: *mut ::core::ffi::c_char,
    pub value: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDictionary {
    pub count: ::core::ffi::c_int,
    pub elems: *mut AVDictionaryEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBPrint {
    pub str_0: *mut ::core::ffi::c_char,
    pub len: ::core::ffi::c_uint,
    pub size: ::core::ffi::c_uint,
    pub size_max: ::core::ffi::c_uint,
    pub reserved_internal_buffer: [::core::ffi::c_char; 1],
    pub reserved_padding: [::core::ffi::c_char; 1000],
}
#[no_mangle]
pub unsafe extern "C" fn av_dict_count(
    mut m: *const AVDictionary,
) -> ::core::ffi::c_int {
    return if !m.is_null() { (*m).count } else { 0 as ::core::ffi::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn av_dict_iterate(
    mut m: *const AVDictionary,
    mut prev: *const AVDictionaryEntry,
) -> *const AVDictionaryEntry {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if m.is_null() {
        return ::core::ptr::null::<AVDictionaryEntry>();
    }
    if !prev.is_null() {
        i = (prev.offset_from((*m).elems) as ::core::ffi::c_long
            + 1 as ::core::ffi::c_long) as ::core::ffi::c_int;
    }
    if i >= (*m).count {
        return ::core::ptr::null::<AVDictionaryEntry>();
    }
    return (*m).elems.offset(i as isize) as *mut AVDictionaryEntry;
}
#[no_mangle]
pub unsafe extern "C" fn av_dict_get(
    mut m: *const AVDictionary,
    mut key: *const ::core::ffi::c_char,
    mut prev: *const AVDictionaryEntry,
    mut flags: ::core::ffi::c_int,
) -> *mut AVDictionaryEntry {
    let mut entry: *const AVDictionaryEntry = prev;
    let mut j: ::core::ffi::c_uint = 0;
    if key.is_null() {
        return ::core::ptr::null_mut::<AVDictionaryEntry>();
    }
    loop {
        entry = av_dict_iterate(m, entry);
        if entry.is_null() {
            break;
        }
        let mut s: *const ::core::ffi::c_char = (*entry).key;
        if flags & 1 as ::core::ffi::c_int != 0 {
            j = 0 as ::core::ffi::c_uint;
            while *s.offset(j as isize) as ::core::ffi::c_int
                == *key.offset(j as isize) as ::core::ffi::c_int
                && *key.offset(j as isize) as ::core::ffi::c_int != 0
            {
                j = j.wrapping_add(1);
            }
        } else {
            j = 0 as ::core::ffi::c_uint;
            while av_toupper(*s.offset(j as isize) as ::core::ffi::c_int)
                == av_toupper(*key.offset(j as isize) as ::core::ffi::c_int)
                && *key.offset(j as isize) as ::core::ffi::c_int != 0
            {
                j = j.wrapping_add(1);
            }
        }
        if *key.offset(j as isize) != 0 {
            continue;
        }
        if *s.offset(j as isize) as ::core::ffi::c_int != 0
            && flags & 2 as ::core::ffi::c_int == 0
        {
            continue;
        }
        return entry as *mut AVDictionaryEntry;
    }
    return ::core::ptr::null_mut::<AVDictionaryEntry>();
}
#[no_mangle]
pub unsafe extern "C" fn av_dict_set(
    mut pm: *mut *mut AVDictionary,
    mut key: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut m: *mut AVDictionary = *pm;
    let mut tag: *mut AVDictionaryEntry = ::core::ptr::null_mut::<AVDictionaryEntry>();
    let mut copy_key: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut copy_value: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut err: ::core::ffi::c_int = 0;
    if flags & 8 as ::core::ffi::c_int != 0 {
        copy_value = value as *mut ::core::ffi::c_void as *mut ::core::ffi::c_char;
    } else if !value.is_null() {
        copy_value = av_strdup(value);
    }
    if key.is_null() {
        err = -(22 as ::core::ffi::c_int);
        current_block = 6788809925427471631;
    } else {
        if flags & 4 as ::core::ffi::c_int != 0 {
            copy_key = key as *mut ::core::ffi::c_void as *mut ::core::ffi::c_char;
        } else {
            copy_key = av_strdup(key);
        }
        if copy_key.is_null() || !value.is_null() && copy_value.is_null() {
            current_block = 10052385298327894676;
        } else {
            if flags & 64 as ::core::ffi::c_int == 0 {
                tag = av_dict_get(
                    m,
                    key,
                    ::core::ptr::null::<AVDictionaryEntry>(),
                    flags,
                );
            } else if flags & 128 as ::core::ffi::c_int != 0 {
                loop {
                    tag = av_dict_get(m, key, tag, flags);
                    if tag.is_null() {
                        break;
                    }
                    if value.is_null() && (*tag).value.is_null()
                        || !value.is_null() && !(*tag).value.is_null()
                            && strcmp(value, (*tag).value) == 0
                    {
                        av_free(copy_key as *mut ::core::ffi::c_void);
                        av_free(copy_value as *mut ::core::ffi::c_void);
                        return 0 as ::core::ffi::c_int;
                    }
                }
            }
            if m.is_null() {
                *pm = av_mallocz(::core::mem::size_of::<AVDictionary>() as size_t)
                    as *mut AVDictionary;
                m = *pm;
            }
            if m.is_null() {
                current_block = 10052385298327894676;
            } else {
                if !tag.is_null() {
                    if flags & 16 as ::core::ffi::c_int != 0 {
                        av_free(copy_key as *mut ::core::ffi::c_void);
                        av_free(copy_value as *mut ::core::ffi::c_void);
                        return 0 as ::core::ffi::c_int;
                    }
                    if !copy_value.is_null() && flags & 32 as ::core::ffi::c_int != 0 {
                        let mut oldlen: size_t = strlen((*tag).value) as size_t;
                        let mut new_part_len: size_t = strlen(copy_value) as size_t;
                        let mut len: size_t = oldlen
                            .wrapping_add(new_part_len)
                            .wrapping_add(1 as size_t);
                        let mut newval: *mut ::core::ffi::c_char = av_realloc(
                            (*tag).value as *mut ::core::ffi::c_void,
                            len,
                        ) as *mut ::core::ffi::c_char;
                        if newval.is_null() {
                            current_block = 10052385298327894676;
                        } else {
                            memcpy(
                                newval.offset(oldlen as isize) as *mut ::core::ffi::c_void,
                                copy_value as *const ::core::ffi::c_void,
                                (new_part_len as size_t).wrapping_add(1 as size_t),
                            );
                            av_freep(&raw mut copy_value as *mut ::core::ffi::c_void);
                            copy_value = newval;
                            current_block = 11913429853522160501;
                        }
                    } else {
                        av_free((*tag).value as *mut ::core::ffi::c_void);
                        current_block = 11913429853522160501;
                    }
                    match current_block {
                        10052385298327894676 => {}
                        _ => {
                            av_free((*tag).key as *mut ::core::ffi::c_void);
                            (*m).count -= 1;
                            *tag = *(*m).elems.offset((*m).count as isize);
                            current_block = 7659304154607701039;
                        }
                    }
                } else if !copy_value.is_null() {
                    let mut tmp: *mut AVDictionaryEntry = av_realloc_array(
                        (*m).elems as *mut ::core::ffi::c_void,
                        ((*m).count + 1 as ::core::ffi::c_int) as size_t,
                        ::core::mem::size_of::<AVDictionaryEntry>() as size_t,
                    ) as *mut AVDictionaryEntry;
                    if tmp.is_null() {
                        current_block = 10052385298327894676;
                    } else {
                        (*m).elems = tmp;
                        current_block = 7659304154607701039;
                    }
                } else {
                    current_block = 7659304154607701039;
                }
                match current_block {
                    10052385298327894676 => {}
                    _ => {
                        if !copy_value.is_null() {
                            let ref mut fresh0 = (*(*m)
                                .elems
                                .offset((*m).count as isize))
                                .key;
                            *fresh0 = copy_key;
                            let ref mut fresh1 = (*(*m)
                                .elems
                                .offset((*m).count as isize))
                                .value;
                            *fresh1 = copy_value;
                            (*m).count += 1;
                            return 0 as ::core::ffi::c_int;
                        } else {
                            err = 0 as ::core::ffi::c_int;
                        }
                        current_block = 3172486688147789299;
                    }
                }
            }
        }
        match current_block {
            3172486688147789299 => {}
            _ => {
                err = -(12 as ::core::ffi::c_int);
                current_block = 6788809925427471631;
            }
        }
    }
    match current_block {
        6788809925427471631 => {
            av_free(copy_value as *mut ::core::ffi::c_void);
        }
        _ => {}
    }
    if !m.is_null() && (*m).count == 0 {
        av_freep(&raw mut (*m).elems as *mut ::core::ffi::c_void);
        av_freep(pm as *mut ::core::ffi::c_void);
    }
    av_free(copy_key as *mut ::core::ffi::c_void);
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn av_dict_set_int(
    mut pm: *mut *mut AVDictionary,
    mut key: *const ::core::ffi::c_char,
    mut value: int64_t,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut valuestr: [::core::ffi::c_char; 22] = [0; 22];
    snprintf(
        &raw mut valuestr as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 22]>() as size_t,
        b"%ld\0" as *const u8 as *const ::core::ffi::c_char,
        value,
    );
    flags &= !(8 as ::core::ffi::c_int);
    return av_dict_set(pm, key, &raw mut valuestr as *mut ::core::ffi::c_char, flags);
}
unsafe extern "C" fn parse_key_value_pair(
    mut pm: *mut *mut AVDictionary,
    mut buf: *mut *const ::core::ffi::c_char,
    mut key_val_sep: *const ::core::ffi::c_char,
    mut pairs_sep: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut key: *mut ::core::ffi::c_char = av_get_token(buf, key_val_sep);
    let mut val: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut ret: ::core::ffi::c_int = 0;
    if !key.is_null() && *key as ::core::ffi::c_int != 0
        && strspn(*buf, key_val_sep) != 0
    {
        *buf = (*buf).offset(1);
        val = av_get_token(buf, pairs_sep);
    }
    if !key.is_null() && *key as ::core::ffi::c_int != 0 && !val.is_null()
        && *val as ::core::ffi::c_int != 0
    {
        ret = av_dict_set(pm, key, val, flags);
    } else {
        ret = -(22 as ::core::ffi::c_int);
    }
    av_freep(&raw mut key as *mut ::core::ffi::c_void);
    av_freep(&raw mut val as *mut ::core::ffi::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_dict_parse_string(
    mut pm: *mut *mut AVDictionary,
    mut str: *const ::core::ffi::c_char,
    mut key_val_sep: *const ::core::ffi::c_char,
    mut pairs_sep: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    if str.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    flags &= !(4 as ::core::ffi::c_int | 8 as ::core::ffi::c_int);
    while *str != 0 {
        ret = parse_key_value_pair(pm, &raw mut str, key_val_sep, pairs_sep, flags);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        if *str != 0 {
            str = str.offset(1);
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn av_toupper(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if c >= 'a' as i32 && c <= 'z' as i32 {
        c ^= 0x20 as ::core::ffi::c_int;
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn av_dict_free(mut pm: *mut *mut AVDictionary) {
    let mut m: *mut AVDictionary = *pm;
    if !m.is_null() {
        loop {
            let fresh2 = (*m).count;
            (*m).count = (*m).count - 1;
            if !(fresh2 != 0) {
                break;
            }
            av_freep(
                &raw mut (*(*m).elems.offset((*m).count as isize)).key
                    as *mut ::core::ffi::c_void,
            );
            av_freep(
                &raw mut (*(*m).elems.offset((*m).count as isize)).value
                    as *mut ::core::ffi::c_void,
            );
        }
        av_freep(&raw mut (*m).elems as *mut ::core::ffi::c_void);
    }
    av_freep(pm as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn av_dict_copy(
    mut dst: *mut *mut AVDictionary,
    mut src: *const AVDictionary,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut t: *const AVDictionaryEntry = ::core::ptr::null::<AVDictionaryEntry>();
    loop {
        t = av_dict_iterate(src, t);
        if t.is_null() {
            break;
        }
        let mut ret: ::core::ffi::c_int = av_dict_set(dst, (*t).key, (*t).value, flags);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_dict_get_string(
    mut m: *const AVDictionary,
    mut buffer: *mut *mut ::core::ffi::c_char,
    key_val_sep: ::core::ffi::c_char,
    pairs_sep: ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut t: *const AVDictionaryEntry = ::core::ptr::null::<AVDictionaryEntry>();
    let mut bprint: AVBPrint = AVBPrint {
        str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        len: 0,
        size: 0,
        size_max: 0,
        reserved_internal_buffer: [0; 1],
        reserved_padding: [0; 1000],
    };
    let mut cnt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut special_chars: [::core::ffi::c_char; 3] = [
        pairs_sep,
        key_val_sep,
        '\0' as i32 as ::core::ffi::c_char,
    ];
    if buffer.is_null() || pairs_sep as ::core::ffi::c_int == '\0' as i32
        || key_val_sep as ::core::ffi::c_int == '\0' as i32
        || pairs_sep as ::core::ffi::c_int == key_val_sep as ::core::ffi::c_int
        || pairs_sep as ::core::ffi::c_int == '\\' as i32
        || key_val_sep as ::core::ffi::c_int == '\\' as i32
    {
        return -(22 as ::core::ffi::c_int);
    }
    if av_dict_count(m) == 0 {
        *buffer = av_strdup(b"\0" as *const u8 as *const ::core::ffi::c_char);
        return if !(*buffer).is_null() {
            0 as ::core::ffi::c_int
        } else {
            -(12 as ::core::ffi::c_int)
        };
    }
    av_bprint_init(
        &raw mut bprint,
        64 as ::core::ffi::c_uint,
        -(1 as ::core::ffi::c_int) as ::core::ffi::c_uint,
    );
    loop {
        t = av_dict_iterate(m, t);
        if t.is_null() {
            break;
        }
        let fresh3 = cnt;
        cnt = cnt + 1;
        if fresh3 != 0 {
            av_bprint_append_data(
                &raw mut bprint,
                &raw const pairs_sep,
                1 as ::core::ffi::c_uint,
            );
        }
        av_bprint_escape(
            &raw mut bprint,
            (*t).key,
            &raw mut special_chars as *mut ::core::ffi::c_char,
            AV_ESCAPE_MODE_BACKSLASH,
            0 as ::core::ffi::c_int,
        );
        av_bprint_append_data(
            &raw mut bprint,
            &raw const key_val_sep,
            1 as ::core::ffi::c_uint,
        );
        av_bprint_escape(
            &raw mut bprint,
            (*t).value,
            &raw mut special_chars as *mut ::core::ffi::c_char,
            AV_ESCAPE_MODE_BACKSLASH,
            0 as ::core::ffi::c_int,
        );
    }
    return av_bprint_finalize(&raw mut bprint, buffer);
}
