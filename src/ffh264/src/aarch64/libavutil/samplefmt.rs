// Translated from FFmpeg 9.0.1 libavutil/samplefmt.c
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
    fn memmove(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn av_malloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn av_calloc(nmemb: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn av_free(ptr: *mut ::core::ffi::c_void);
    fn av_freep(ptr: *mut ::core::ffi::c_void);
}
pub type size_t = ::core::ffi::c_ulong;
pub type int64_t = ::core::ffi::c_long;
pub type uint8_t = ::core::ffi::c_uchar;
pub type AVSampleFormat = ::core::ffi::c_int;
pub const AV_SAMPLE_FMT_NB: AVSampleFormat = 12;
pub const AV_SAMPLE_FMT_S64P: AVSampleFormat = 11;
pub const AV_SAMPLE_FMT_S64: AVSampleFormat = 10;
pub const AV_SAMPLE_FMT_DBLP: AVSampleFormat = 9;
pub const AV_SAMPLE_FMT_FLTP: AVSampleFormat = 8;
pub const AV_SAMPLE_FMT_S32P: AVSampleFormat = 7;
pub const AV_SAMPLE_FMT_S16P: AVSampleFormat = 6;
pub const AV_SAMPLE_FMT_U8P: AVSampleFormat = 5;
pub const AV_SAMPLE_FMT_DBL: AVSampleFormat = 4;
pub const AV_SAMPLE_FMT_FLT: AVSampleFormat = 3;
pub const AV_SAMPLE_FMT_S32: AVSampleFormat = 2;
pub const AV_SAMPLE_FMT_S16: AVSampleFormat = 1;
pub const AV_SAMPLE_FMT_U8: AVSampleFormat = 0;
pub const AV_SAMPLE_FMT_NONE: AVSampleFormat = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SampleFmtInfo {
    pub name: [::core::ffi::c_char; 8],
    pub bits: ::core::ffi::c_int,
    pub planar: ::core::ffi::c_int,
    pub altform: AVSampleFormat,
}
static mut sample_fmt_info: [SampleFmtInfo; 12] = unsafe {
    [
        SampleFmtInfo {
            name: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"u8\0\0\0\0\0\0"),
            bits: 8 as ::core::ffi::c_int,
            planar: 0 as ::core::ffi::c_int,
            altform: AV_SAMPLE_FMT_U8P,
        },
        SampleFmtInfo {
            name: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"s16\0\0\0\0\0"),
            bits: 16 as ::core::ffi::c_int,
            planar: 0 as ::core::ffi::c_int,
            altform: AV_SAMPLE_FMT_S16P,
        },
        SampleFmtInfo {
            name: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"s32\0\0\0\0\0"),
            bits: 32 as ::core::ffi::c_int,
            planar: 0 as ::core::ffi::c_int,
            altform: AV_SAMPLE_FMT_S32P,
        },
        SampleFmtInfo {
            name: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"flt\0\0\0\0\0"),
            bits: 32 as ::core::ffi::c_int,
            planar: 0 as ::core::ffi::c_int,
            altform: AV_SAMPLE_FMT_FLTP,
        },
        SampleFmtInfo {
            name: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"dbl\0\0\0\0\0"),
            bits: 64 as ::core::ffi::c_int,
            planar: 0 as ::core::ffi::c_int,
            altform: AV_SAMPLE_FMT_DBLP,
        },
        SampleFmtInfo {
            name: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"u8p\0\0\0\0\0"),
            bits: 8 as ::core::ffi::c_int,
            planar: 1 as ::core::ffi::c_int,
            altform: AV_SAMPLE_FMT_U8,
        },
        SampleFmtInfo {
            name: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"s16p\0\0\0\0"),
            bits: 16 as ::core::ffi::c_int,
            planar: 1 as ::core::ffi::c_int,
            altform: AV_SAMPLE_FMT_S16,
        },
        SampleFmtInfo {
            name: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"s32p\0\0\0\0"),
            bits: 32 as ::core::ffi::c_int,
            planar: 1 as ::core::ffi::c_int,
            altform: AV_SAMPLE_FMT_S32,
        },
        SampleFmtInfo {
            name: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"fltp\0\0\0\0"),
            bits: 32 as ::core::ffi::c_int,
            planar: 1 as ::core::ffi::c_int,
            altform: AV_SAMPLE_FMT_FLT,
        },
        SampleFmtInfo {
            name: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"dblp\0\0\0\0"),
            bits: 64 as ::core::ffi::c_int,
            planar: 1 as ::core::ffi::c_int,
            altform: AV_SAMPLE_FMT_DBL,
        },
        SampleFmtInfo {
            name: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"s64\0\0\0\0\0"),
            bits: 64 as ::core::ffi::c_int,
            planar: 0 as ::core::ffi::c_int,
            altform: AV_SAMPLE_FMT_S64P,
        },
        SampleFmtInfo {
            name: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"s64p\0\0\0\0"),
            bits: 64 as ::core::ffi::c_int,
            planar: 1 as ::core::ffi::c_int,
            altform: AV_SAMPLE_FMT_S64,
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn av_get_sample_fmt_name(
    mut sample_fmt: AVSampleFormat,
) -> *const ::core::ffi::c_char {
    if (sample_fmt as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        || sample_fmt as ::core::ffi::c_int >= AV_SAMPLE_FMT_NB as ::core::ffi::c_int
    {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return &raw const (*(&raw const sample_fmt_info as *const SampleFmtInfo)
        .offset(sample_fmt as isize))
        .name as *const ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn av_get_sample_fmt(
    mut name: *const ::core::ffi::c_char,
) -> AVSampleFormat {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < AV_SAMPLE_FMT_NB as ::core::ffi::c_int {
        if strcmp(
            &raw const (*(&raw const sample_fmt_info as *const SampleFmtInfo)
                .offset(i as isize))
                .name as *const ::core::ffi::c_char,
            name,
        ) == 0
        {
            return i as AVSampleFormat;
        }
        i += 1;
    }
    return AV_SAMPLE_FMT_NONE;
}
#[no_mangle]
pub unsafe extern "C" fn av_get_alt_sample_fmt(
    mut sample_fmt: AVSampleFormat,
    mut planar: ::core::ffi::c_int,
) -> AVSampleFormat {
    if (sample_fmt as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        || sample_fmt as ::core::ffi::c_int >= AV_SAMPLE_FMT_NB as ::core::ffi::c_int
    {
        return AV_SAMPLE_FMT_NONE;
    }
    if sample_fmt_info[sample_fmt as usize].planar == planar {
        return sample_fmt;
    }
    return sample_fmt_info[sample_fmt as usize].altform;
}
#[no_mangle]
pub unsafe extern "C" fn av_get_packed_sample_fmt(
    mut sample_fmt: AVSampleFormat,
) -> AVSampleFormat {
    if (sample_fmt as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        || sample_fmt as ::core::ffi::c_int >= AV_SAMPLE_FMT_NB as ::core::ffi::c_int
    {
        return AV_SAMPLE_FMT_NONE;
    }
    if sample_fmt_info[sample_fmt as usize].planar != 0 {
        return sample_fmt_info[sample_fmt as usize].altform;
    }
    return sample_fmt;
}
#[no_mangle]
pub unsafe extern "C" fn av_get_planar_sample_fmt(
    mut sample_fmt: AVSampleFormat,
) -> AVSampleFormat {
    if (sample_fmt as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        || sample_fmt as ::core::ffi::c_int >= AV_SAMPLE_FMT_NB as ::core::ffi::c_int
    {
        return AV_SAMPLE_FMT_NONE;
    }
    if sample_fmt_info[sample_fmt as usize].planar != 0 {
        return sample_fmt;
    }
    return sample_fmt_info[sample_fmt as usize].altform;
}
#[no_mangle]
pub unsafe extern "C" fn av_get_sample_fmt_string(
    mut buf: *mut ::core::ffi::c_char,
    mut buf_size: ::core::ffi::c_int,
    mut sample_fmt: AVSampleFormat,
) -> *mut ::core::ffi::c_char {
    if (sample_fmt as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        snprintf(
            buf,
            buf_size as size_t,
            b"name   depth\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if (sample_fmt as ::core::ffi::c_int) < AV_SAMPLE_FMT_NB as ::core::ffi::c_int
    {
        let mut info: SampleFmtInfo = sample_fmt_info[sample_fmt as usize];
        snprintf(
            buf,
            buf_size as size_t,
            b"%-6s   %2d \0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut info.name as *mut ::core::ffi::c_char,
            info.bits,
        );
    }
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn av_get_bytes_per_sample(
    mut sample_fmt: AVSampleFormat,
) -> ::core::ffi::c_int {
    return if (sample_fmt as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        || sample_fmt as ::core::ffi::c_int >= AV_SAMPLE_FMT_NB as ::core::ffi::c_int
    {
        0 as ::core::ffi::c_int
    } else {
        sample_fmt_info[sample_fmt as usize].bits >> 3 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_sample_fmt_is_planar(
    mut sample_fmt: AVSampleFormat,
) -> ::core::ffi::c_int {
    if (sample_fmt as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        || sample_fmt as ::core::ffi::c_int >= AV_SAMPLE_FMT_NB as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    return sample_fmt_info[sample_fmt as usize].planar;
}
#[no_mangle]
pub unsafe extern "C" fn av_samples_get_buffer_size(
    mut linesize: *mut ::core::ffi::c_int,
    mut nb_channels: ::core::ffi::c_int,
    mut nb_samples: ::core::ffi::c_int,
    mut sample_fmt: AVSampleFormat,
    mut align: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut line_size: ::core::ffi::c_int = 0;
    let mut sample_size: ::core::ffi::c_int = av_get_bytes_per_sample(sample_fmt);
    let mut planar: ::core::ffi::c_int = av_sample_fmt_is_planar(sample_fmt);
    if sample_size == 0 || nb_samples <= 0 as ::core::ffi::c_int
        || nb_channels <= 0 as ::core::ffi::c_int
    {
        return -(22 as ::core::ffi::c_int);
    }
    if align == 0 {
        if nb_samples > 2147483647 as ::core::ffi::c_int - 31 as ::core::ffi::c_int {
            return -(22 as ::core::ffi::c_int);
        }
        align = 1 as ::core::ffi::c_int;
        nb_samples = nb_samples + 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
            & !(32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int);
    }
    if nb_channels > 2147483647 as ::core::ffi::c_int / align
        || nb_channels as int64_t * nb_samples as int64_t
            > ((2147483647 as ::core::ffi::c_int - align * nb_channels) / sample_size)
                as int64_t
    {
        return -(22 as ::core::ffi::c_int);
    }
    line_size = if planar != 0 {
        nb_samples * sample_size + align - 1 as ::core::ffi::c_int
            & !(align - 1 as ::core::ffi::c_int)
    } else {
        nb_samples * sample_size * nb_channels + align - 1 as ::core::ffi::c_int
            & !(align - 1 as ::core::ffi::c_int)
    };
    if !linesize.is_null() {
        *linesize = line_size;
    }
    return if planar != 0 { line_size * nb_channels } else { line_size };
}
#[no_mangle]
pub unsafe extern "C" fn av_samples_fill_arrays(
    mut audio_data: *mut *mut uint8_t,
    mut linesize: *mut ::core::ffi::c_int,
    mut buf: *const uint8_t,
    mut nb_channels: ::core::ffi::c_int,
    mut nb_samples: ::core::ffi::c_int,
    mut sample_fmt: AVSampleFormat,
    mut align: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ch: ::core::ffi::c_int = 0;
    let mut planar: ::core::ffi::c_int = 0;
    let mut buf_size: ::core::ffi::c_int = 0;
    let mut line_size: ::core::ffi::c_int = 0;
    planar = av_sample_fmt_is_planar(sample_fmt);
    buf_size = av_samples_get_buffer_size(
        &raw mut line_size,
        nb_channels,
        nb_samples,
        sample_fmt,
        align,
    );
    if buf_size < 0 as ::core::ffi::c_int {
        return buf_size;
    }
    if !linesize.is_null() {
        *linesize = line_size;
    }
    memset(
        audio_data as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        if planar != 0 {
            (::core::mem::size_of::<*mut uint8_t>() as size_t)
                .wrapping_mul(nb_channels as size_t)
        } else {
            ::core::mem::size_of::<*mut uint8_t>() as size_t
        },
    );
    if buf.is_null() {
        return buf_size;
    }
    let ref mut fresh0 = *audio_data.offset(0 as ::core::ffi::c_int as isize);
    *fresh0 = buf as *mut uint8_t;
    ch = 1 as ::core::ffi::c_int;
    while planar != 0 && ch < nb_channels {
        let ref mut fresh1 = *audio_data.offset(ch as isize);
        *fresh1 = (*audio_data.offset((ch - 1 as ::core::ffi::c_int) as isize))
            .offset(line_size as isize);
        ch += 1;
    }
    return buf_size;
}
#[no_mangle]
pub unsafe extern "C" fn av_samples_alloc(
    mut audio_data: *mut *mut uint8_t,
    mut linesize: *mut ::core::ffi::c_int,
    mut nb_channels: ::core::ffi::c_int,
    mut nb_samples: ::core::ffi::c_int,
    mut sample_fmt: AVSampleFormat,
    mut align: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut buf: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut size: ::core::ffi::c_int = av_samples_get_buffer_size(
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        nb_channels,
        nb_samples,
        sample_fmt,
        align,
    );
    if size < 0 as ::core::ffi::c_int {
        return size;
    }
    buf = av_malloc(size as size_t) as *mut uint8_t;
    if buf.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    size = av_samples_fill_arrays(
        audio_data,
        linesize,
        buf,
        nb_channels,
        nb_samples,
        sample_fmt,
        align,
    );
    if size < 0 as ::core::ffi::c_int {
        av_free(buf as *mut ::core::ffi::c_void);
        return size;
    }
    av_samples_set_silence(
        audio_data,
        0 as ::core::ffi::c_int,
        nb_samples,
        nb_channels,
        sample_fmt,
    );
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn av_samples_alloc_array_and_samples(
    mut audio_data: *mut *mut *mut uint8_t,
    mut linesize: *mut ::core::ffi::c_int,
    mut nb_channels: ::core::ffi::c_int,
    mut nb_samples: ::core::ffi::c_int,
    mut sample_fmt: AVSampleFormat,
    mut align: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut nb_planes: ::core::ffi::c_int = if av_sample_fmt_is_planar(sample_fmt) != 0 {
        nb_channels
    } else {
        1 as ::core::ffi::c_int
    };
    *audio_data = av_calloc(
        nb_planes as size_t,
        ::core::mem::size_of::<*mut uint8_t>() as size_t,
    ) as *mut *mut uint8_t;
    if (*audio_data).is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    ret = av_samples_alloc(
        *audio_data,
        linesize,
        nb_channels,
        nb_samples,
        sample_fmt,
        align,
    );
    if ret < 0 as ::core::ffi::c_int {
        av_freep(audio_data as *mut ::core::ffi::c_void);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_samples_copy(
    mut dst: *const *mut uint8_t,
    mut src: *const *mut uint8_t,
    mut dst_offset: ::core::ffi::c_int,
    mut src_offset: ::core::ffi::c_int,
    mut nb_samples: ::core::ffi::c_int,
    mut nb_channels: ::core::ffi::c_int,
    mut sample_fmt: AVSampleFormat,
) -> ::core::ffi::c_int {
    let mut planar: ::core::ffi::c_int = av_sample_fmt_is_planar(sample_fmt);
    let mut planes: ::core::ffi::c_int = if planar != 0 {
        nb_channels
    } else {
        1 as ::core::ffi::c_int
    };
    let mut block_align: ::core::ffi::c_int = av_get_bytes_per_sample(sample_fmt)
        * (if planar != 0 { 1 as ::core::ffi::c_int } else { nb_channels });
    let mut data_size: ::core::ffi::c_int = nb_samples * block_align;
    let mut i: ::core::ffi::c_int = 0;
    dst_offset *= block_align;
    src_offset *= block_align;
    if (if *dst.offset(0 as ::core::ffi::c_int as isize)
        < *src.offset(0 as ::core::ffi::c_int as isize)
    {
        (*src.offset(0 as ::core::ffi::c_int as isize))
            .offset_from(*dst.offset(0 as ::core::ffi::c_int as isize))
            as ::core::ffi::c_long
    } else {
        (*dst.offset(0 as ::core::ffi::c_int as isize))
            .offset_from(*src.offset(0 as ::core::ffi::c_int as isize))
            as ::core::ffi::c_long
    }) >= data_size as ::core::ffi::c_long
    {
        i = 0 as ::core::ffi::c_int;
        while i < planes {
            memcpy(
                (*dst.offset(i as isize)).offset(dst_offset as isize)
                    as *mut ::core::ffi::c_void,
                (*src.offset(i as isize)).offset(src_offset as isize)
                    as *const ::core::ffi::c_void,
                data_size as size_t,
            );
            i += 1;
        }
    } else {
        i = 0 as ::core::ffi::c_int;
        while i < planes {
            memmove(
                (*dst.offset(i as isize)).offset(dst_offset as isize)
                    as *mut ::core::ffi::c_void,
                (*src.offset(i as isize)).offset(src_offset as isize)
                    as *const ::core::ffi::c_void,
                data_size as size_t,
            );
            i += 1;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_samples_set_silence(
    mut audio_data: *const *mut uint8_t,
    mut offset: ::core::ffi::c_int,
    mut nb_samples: ::core::ffi::c_int,
    mut nb_channels: ::core::ffi::c_int,
    mut sample_fmt: AVSampleFormat,
) -> ::core::ffi::c_int {
    let mut planar: ::core::ffi::c_int = av_sample_fmt_is_planar(sample_fmt);
    let mut planes: ::core::ffi::c_int = if planar != 0 {
        nb_channels
    } else {
        1 as ::core::ffi::c_int
    };
    let mut block_align: ::core::ffi::c_int = av_get_bytes_per_sample(sample_fmt)
        * (if planar != 0 { 1 as ::core::ffi::c_int } else { nb_channels });
    let mut data_size: ::core::ffi::c_int = nb_samples * block_align;
    let mut fill_char: ::core::ffi::c_int = if sample_fmt as ::core::ffi::c_int
        == AV_SAMPLE_FMT_U8 as ::core::ffi::c_int
        || sample_fmt as ::core::ffi::c_int == AV_SAMPLE_FMT_U8P as ::core::ffi::c_int
    {
        0x80 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    let mut i: ::core::ffi::c_int = 0;
    offset *= block_align;
    i = 0 as ::core::ffi::c_int;
    while i < planes {
        memset(
            (*audio_data.offset(i as isize)).offset(offset as isize)
                as *mut ::core::ffi::c_void,
            fill_char,
            data_size as size_t,
        );
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
