// Translated from FFmpeg 9.0.1 libavcodec/packet.c
// (preprocessed for armv7-unknown-linux-musleabihf) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// AVPacket functions for libavcodec
// Copyright (c) 2000, 2001, 2002 Fabrice Bellard
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
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn av_container_fifo_alloc(
        opaque: *mut ::core::ffi::c_void,
        container_alloc: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        container_reset: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
        container_free: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
        fifo_transfer: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                ::core::ffi::c_uint,
            ) -> ::core::ffi::c_int,
        >,
        flags: ::core::ffi::c_uint,
    ) -> *mut AVContainerFifo;
    fn av_buffer_alloc(size: size_t) -> *mut AVBufferRef;
    fn av_malloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn av_mallocz(size: size_t) -> *mut ::core::ffi::c_void;
    fn av_buffer_create(
        data: *mut uint8_t,
        size: size_t,
        free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint8_t) -> ()>,
        opaque: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_int,
    ) -> *mut AVBufferRef;
    fn av_dict_iterate(
        m: *const AVDictionary,
        prev: *const AVDictionaryEntry,
    ) -> *const AVDictionaryEntry;
    fn av_buffer_default_free(opaque: *mut ::core::ffi::c_void, data: *mut uint8_t);
    fn av_buffer_ref(buf: *const AVBufferRef) -> *mut AVBufferRef;
    fn av_buffer_unref(buf: *mut *mut AVBufferRef);
    fn av_buffer_is_writable(buf: *const AVBufferRef) -> ::core::ffi::c_int;
    fn av_dict_set(
        pm: *mut *mut AVDictionary,
        key: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn av_realloc(
        ptr: *mut ::core::ffi::c_void,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn av_buffer_realloc(buf: *mut *mut AVBufferRef, size: size_t) -> ::core::ffi::c_int;
    fn av_rescale_q(a: int64_t, bq: AVRational, cq: AVRational) -> int64_t;
    fn av_buffer_replace(
        dst: *mut *mut AVBufferRef,
        src: *const AVBufferRef,
    ) -> ::core::ffi::c_int;
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
pub type size_t = ::core::ffi::c_uint;
pub type int64_t = ::core::ffi::c_longlong;
pub type uint8_t = ::core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: ::core::ffi::c_int,
    pub den: ::core::ffi::c_int,
}
pub type AVContainerFifoFlags = ::core::ffi::c_uint;
pub const AV_CONTAINER_FIFO_FLAG_USER: AVContainerFifoFlags = 65536;
pub const AV_CONTAINER_FIFO_FLAG_REF: AVContainerFifoFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVProducerReferenceTime {
    pub wallclock: int64_t,
    pub flags: ::core::ffi::c_int,
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
pub struct AVDictionaryEntry {
    pub key: *mut ::core::ffi::c_char,
    pub value: *mut ::core::ffi::c_char,
}
pub type AVPacketSideDataType = ::core::ffi::c_uint;
pub const AV_PKT_DATA_NB: AVPacketSideDataType = 43;
pub const AV_PKT_DATA_HEVC_CONF: AVPacketSideDataType = 42;
pub const AV_PKT_DATA_DYNAMIC_HDR_SMPTE_2094_APP5: AVPacketSideDataType = 41;
pub const AV_PKT_DATA_EXIF: AVPacketSideDataType = 40;
pub const AV_PKT_DATA_RTCP_SR: AVPacketSideDataType = 39;
pub const AV_PKT_DATA_3D_REFERENCE_DISPLAYS: AVPacketSideDataType = 38;
pub const AV_PKT_DATA_LCEVC: AVPacketSideDataType = 37;
pub const AV_PKT_DATA_FRAME_CROPPING: AVPacketSideDataType = 36;
pub const AV_PKT_DATA_AMBIENT_VIEWING_ENVIRONMENT: AVPacketSideDataType = 35;
pub const AV_PKT_DATA_IAMF_RECON_GAIN_INFO_PARAM: AVPacketSideDataType = 34;
pub const AV_PKT_DATA_IAMF_DEMIXING_INFO_PARAM: AVPacketSideDataType = 33;
pub const AV_PKT_DATA_IAMF_MIX_GAIN_PARAM: AVPacketSideDataType = 32;
pub const AV_PKT_DATA_DYNAMIC_HDR10_PLUS: AVPacketSideDataType = 31;
pub const AV_PKT_DATA_S12M_TIMECODE: AVPacketSideDataType = 30;
pub const AV_PKT_DATA_DOVI_CONF: AVPacketSideDataType = 29;
pub const AV_PKT_DATA_ICC_PROFILE: AVPacketSideDataType = 28;
pub const AV_PKT_DATA_PRFT: AVPacketSideDataType = 27;
pub const AV_PKT_DATA_AFD: AVPacketSideDataType = 26;
pub const AV_PKT_DATA_ENCRYPTION_INFO: AVPacketSideDataType = 25;
pub const AV_PKT_DATA_ENCRYPTION_INIT_INFO: AVPacketSideDataType = 24;
pub const AV_PKT_DATA_A53_CC: AVPacketSideDataType = 23;
pub const AV_PKT_DATA_CONTENT_LIGHT_LEVEL: AVPacketSideDataType = 22;
pub const AV_PKT_DATA_SPHERICAL: AVPacketSideDataType = 21;
pub const AV_PKT_DATA_MASTERING_DISPLAY_METADATA: AVPacketSideDataType = 20;
pub const AV_PKT_DATA_MPEGTS_STREAM_ID: AVPacketSideDataType = 19;
pub const AV_PKT_DATA_METADATA_UPDATE: AVPacketSideDataType = 18;
pub const AV_PKT_DATA_WEBVTT_SETTINGS: AVPacketSideDataType = 17;
pub const AV_PKT_DATA_WEBVTT_IDENTIFIER: AVPacketSideDataType = 16;
pub const AV_PKT_DATA_MATROSKA_BLOCKADDITIONAL: AVPacketSideDataType = 15;
pub const AV_PKT_DATA_SUBTITLE_POSITION: AVPacketSideDataType = 14;
pub const AV_PKT_DATA_STRINGS_METADATA: AVPacketSideDataType = 13;
pub const AV_PKT_DATA_JP_DUALMONO: AVPacketSideDataType = 12;
pub const AV_PKT_DATA_SKIP_SAMPLES: AVPacketSideDataType = 11;
pub const AV_PKT_DATA_CPB_PROPERTIES: AVPacketSideDataType = 10;
pub const AV_PKT_DATA_FALLBACK_TRACK: AVPacketSideDataType = 9;
pub const AV_PKT_DATA_QUALITY_STATS: AVPacketSideDataType = 8;
pub const AV_PKT_DATA_AUDIO_SERVICE_TYPE: AVPacketSideDataType = 7;
pub const AV_PKT_DATA_STEREO3D: AVPacketSideDataType = 6;
pub const AV_PKT_DATA_DISPLAYMATRIX: AVPacketSideDataType = 5;
pub const AV_PKT_DATA_REPLAYGAIN: AVPacketSideDataType = 4;
pub const AV_PKT_DATA_H263_MB_INFO: AVPacketSideDataType = 3;
pub const AV_PKT_DATA_PARAM_CHANGE: AVPacketSideDataType = 2;
pub const AV_PKT_DATA_NEW_EXTRADATA: AVPacketSideDataType = 1;
pub const AV_PKT_DATA_PALETTE: AVPacketSideDataType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPacketSideData {
    pub data: *mut uint8_t,
    pub size: size_t,
    pub type_0: AVPacketSideDataType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPacket {
    pub buf: *mut AVBufferRef,
    pub pts: int64_t,
    pub dts: int64_t,
    pub data: *mut uint8_t,
    pub size: ::core::ffi::c_int,
    pub stream_index: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub side_data: *mut AVPacketSideData,
    pub side_data_elems: ::core::ffi::c_int,
    pub duration: int64_t,
    pub pos: int64_t,
    pub opaque: *mut ::core::ffi::c_void,
    pub opaque_ref: *mut AVBufferRef,
    pub time_base: AVRational,
}
#[no_mangle]
pub unsafe extern "C" fn av_init_packet(mut pkt: *mut AVPacket) {
    (*pkt).pts = 0x8000000000000000 as ::core::ffi::c_ulonglong as int64_t;
    (*pkt).dts = 0x8000000000000000 as ::core::ffi::c_ulonglong as int64_t;
    (*pkt).pos = -(1 as ::core::ffi::c_int) as int64_t;
    (*pkt).duration = 0 as int64_t;
    (*pkt).flags = 0 as ::core::ffi::c_int;
    (*pkt).stream_index = 0 as ::core::ffi::c_int;
    (*pkt).buf = ::core::ptr::null_mut::<AVBufferRef>();
    (*pkt).side_data = ::core::ptr::null_mut::<AVPacketSideData>();
    (*pkt).side_data_elems = 0 as ::core::ffi::c_int;
    (*pkt).opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
    (*pkt).opaque_ref = ::core::ptr::null_mut::<AVBufferRef>();
    (*pkt).time_base = av_make_q(0 as ::core::ffi::c_int, 1 as ::core::ffi::c_int);
}
unsafe extern "C" fn get_packet_defaults(mut pkt: *mut AVPacket) {
    memset(
        pkt as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<AVPacket>() as size_t,
    );
    (*pkt).pts = 0x8000000000000000 as ::core::ffi::c_ulonglong as int64_t;
    (*pkt).dts = 0x8000000000000000 as ::core::ffi::c_ulonglong as int64_t;
    (*pkt).pos = -(1 as ::core::ffi::c_int) as int64_t;
    (*pkt).time_base = av_make_q(0 as ::core::ffi::c_int, 1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn av_packet_alloc() -> *mut AVPacket {
    let mut pkt: *mut AVPacket = av_malloc(::core::mem::size_of::<AVPacket>() as size_t)
        as *mut AVPacket;
    if pkt.is_null() {
        return pkt;
    }
    get_packet_defaults(pkt);
    return pkt;
}
#[inline]
unsafe extern "C" fn av_make_q(
    mut num: ::core::ffi::c_int,
    mut den: ::core::ffi::c_int,
) -> AVRational {
    let mut r: AVRational = AVRational { num: num, den: den };
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn av_packet_free(mut pkt: *mut *mut AVPacket) {
    if pkt.is_null() || (*pkt).is_null() {
        return;
    }
    av_packet_unref(*pkt);
    av_freep(pkt as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn packet_alloc(
    mut buf: *mut *mut AVBufferRef,
    mut size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    if size < 0 as ::core::ffi::c_int
        || size >= 2147483647 as ::core::ffi::c_int - 64 as ::core::ffi::c_int
    {
        return -(22 as ::core::ffi::c_int);
    }
    ret = av_buffer_realloc(buf, (size + 64 as ::core::ffi::c_int) as size_t);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    memset(
        (**buf).data.offset(size as isize) as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        64 as size_t,
    );
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_new_packet(
    mut pkt: *mut AVPacket,
    mut size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut buf: *mut AVBufferRef = ::core::ptr::null_mut::<AVBufferRef>();
    let mut ret: ::core::ffi::c_int = packet_alloc(&raw mut buf, size);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    get_packet_defaults(pkt);
    (*pkt).buf = buf;
    (*pkt).data = (*buf).data;
    (*pkt).size = size;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_shrink_packet(
    mut pkt: *mut AVPacket,
    mut size: ::core::ffi::c_int,
) {
    if (*pkt).size <= size {
        return;
    }
    (*pkt).size = size;
    memset(
        (*pkt).data.offset(size as isize) as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        64 as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_grow_packet(
    mut pkt: *mut AVPacket,
    mut grow_by: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut new_size: ::core::ffi::c_int = 0;
    if !((*pkt).size as ::core::ffi::c_uint
        <= (2147483647 as ::core::ffi::c_int - 64 as ::core::ffi::c_int)
            as ::core::ffi::c_uint)
    {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"(unsigned)pkt->size <= 2147483647 - 64\0" as *const u8
                as *const ::core::ffi::c_char,
            b"src/libavcodec/packet.c\0" as *const u8 as *const ::core::ffi::c_char,
            124 as ::core::ffi::c_int,
        );
        abort();
    }
    if grow_by as ::core::ffi::c_uint
        > (2147483647 as ::core::ffi::c_int - ((*pkt).size + 64 as ::core::ffi::c_int))
            as ::core::ffi::c_uint
    {
        return -(12 as ::core::ffi::c_int);
    }
    new_size = (*pkt).size + grow_by + 64 as ::core::ffi::c_int;
    if !(*pkt).buf.is_null() {
        let mut data_offset: size_t = 0;
        let mut old_data: *mut uint8_t = (*pkt).data;
        if (*pkt).data.is_null() {
            data_offset = 0 as size_t;
            (*pkt).data = (*(*pkt).buf).data;
        } else {
            data_offset = (*pkt).data.offset_from((*(*pkt).buf).data)
                as ::core::ffi::c_int as size_t;
            if data_offset > (2147483647 as ::core::ffi::c_int - new_size) as size_t {
                return -(12 as ::core::ffi::c_int);
            }
        }
        if (new_size as size_t).wrapping_add(data_offset) > (*(*pkt).buf).size
            || av_buffer_is_writable((*pkt).buf) == 0
        {
            let mut ret: ::core::ffi::c_int = 0;
            if (new_size as size_t).wrapping_add(data_offset)
                < (2147483647 as ::core::ffi::c_int
                    - new_size / 16 as ::core::ffi::c_int) as size_t
            {
                new_size += new_size / 16 as ::core::ffi::c_int;
            }
            ret = av_buffer_realloc(
                &raw mut (*pkt).buf,
                (new_size as size_t).wrapping_add(data_offset),
            );
            if ret < 0 as ::core::ffi::c_int {
                (*pkt).data = old_data;
                return ret;
            }
            (*pkt).data = (*(*pkt).buf).data.offset(data_offset as isize);
        }
    } else {
        (*pkt).buf = av_buffer_alloc(new_size as size_t);
        if (*pkt).buf.is_null() {
            return -(12 as ::core::ffi::c_int);
        }
        if (*pkt).size > 0 as ::core::ffi::c_int {
            memcpy(
                (*(*pkt).buf).data as *mut ::core::ffi::c_void,
                (*pkt).data as *const ::core::ffi::c_void,
                (*pkt).size as size_t,
            );
        }
        (*pkt).data = (*(*pkt).buf).data;
    }
    (*pkt).size += grow_by;
    memset(
        (*pkt).data.offset((*pkt).size as isize) as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        64 as size_t,
    );
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_packet_from_data(
    mut pkt: *mut AVPacket,
    mut data: *mut uint8_t,
    mut size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if size >= 2147483647 as ::core::ffi::c_int - 64 as ::core::ffi::c_int {
        return -(22 as ::core::ffi::c_int);
    }
    (*pkt).buf = av_buffer_create(
        data,
        (size + 64 as ::core::ffi::c_int) as size_t,
        Some(
            av_buffer_default_free
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint8_t) -> (),
        ),
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        0 as ::core::ffi::c_int,
    );
    if (*pkt).buf.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    (*pkt).data = data;
    (*pkt).size = size;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_packet_free_side_data(mut pkt: *mut AVPacket) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*pkt).side_data_elems {
        av_freep(
            &raw mut (*(*pkt).side_data.offset(i as isize)).data
                as *mut ::core::ffi::c_void,
        );
        i += 1;
    }
    av_freep(&raw mut (*pkt).side_data as *mut ::core::ffi::c_void);
    (*pkt).side_data_elems = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_packet_add_side_data(
    mut pkt: *mut AVPacket,
    mut type_0: AVPacketSideDataType,
    mut data: *mut uint8_t,
    mut size: size_t,
) -> ::core::ffi::c_int {
    let mut tmp: *mut AVPacketSideData = ::core::ptr::null_mut::<AVPacketSideData>();
    let mut i: ::core::ffi::c_int = 0;
    let mut elems: ::core::ffi::c_int = (*pkt).side_data_elems;
    i = 0 as ::core::ffi::c_int;
    while i < elems {
        let mut sd: *mut AVPacketSideData = (*pkt).side_data.offset(i as isize)
            as *mut AVPacketSideData;
        if (*sd).type_0 as ::core::ffi::c_uint == type_0 as ::core::ffi::c_uint {
            av_free((*sd).data as *mut ::core::ffi::c_void);
            (*sd).data = data;
            (*sd).size = size;
            return 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    if (elems as ::core::ffi::c_uint).wrapping_add(1 as ::core::ffi::c_uint)
        > AV_PKT_DATA_NB as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return -(34 as ::core::ffi::c_int);
    }
    tmp = av_realloc(
        (*pkt).side_data as *mut ::core::ffi::c_void,
        ((elems + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<AVPacketSideData>() as size_t),
    ) as *mut AVPacketSideData;
    if tmp.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    (*pkt).side_data = tmp;
    let ref mut fresh2 = (*(*pkt).side_data.offset(elems as isize)).data;
    *fresh2 = data;
    (*(*pkt).side_data.offset(elems as isize)).size = size;
    (*(*pkt).side_data.offset(elems as isize)).type_0 = type_0;
    (*pkt).side_data_elems += 1;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_packet_new_side_data(
    mut pkt: *mut AVPacket,
    mut type_0: AVPacketSideDataType,
    mut size: size_t,
) -> *mut uint8_t {
    let mut ret: ::core::ffi::c_int = 0;
    let mut data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    if size > (0xffffffff as ::core::ffi::c_uint).wrapping_sub(64 as ::core::ffi::c_uint)
    {
        return ::core::ptr::null_mut::<uint8_t>();
    }
    data = av_mallocz(size.wrapping_add(64 as size_t)) as *mut uint8_t;
    if data.is_null() {
        return ::core::ptr::null_mut::<uint8_t>();
    }
    ret = av_packet_add_side_data(pkt, type_0, data, size);
    if ret < 0 as ::core::ffi::c_int {
        av_freep(&raw mut data as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<uint8_t>();
    }
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn av_packet_get_side_data(
    mut pkt: *const AVPacket,
    mut type_0: AVPacketSideDataType,
    mut size: *mut size_t,
) -> *mut uint8_t {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*pkt).side_data_elems {
        if (*(*pkt).side_data.offset(i as isize)).type_0 as ::core::ffi::c_uint
            == type_0 as ::core::ffi::c_uint
        {
            if !size.is_null() {
                *size = (*(*pkt).side_data.offset(i as isize)).size;
            }
            return (*(*pkt).side_data.offset(i as isize)).data;
        }
        i += 1;
    }
    if !size.is_null() {
        *size = 0 as size_t;
    }
    return ::core::ptr::null_mut::<uint8_t>();
}
#[no_mangle]
pub unsafe extern "C" fn av_packet_side_data_name(
    mut type_0: AVPacketSideDataType,
) -> *const ::core::ffi::c_char {
    match type_0 as ::core::ffi::c_uint {
        0 => return b"Palette\0" as *const u8 as *const ::core::ffi::c_char,
        1 => return b"New Extradata\0" as *const u8 as *const ::core::ffi::c_char,
        2 => return b"Param Change\0" as *const u8 as *const ::core::ffi::c_char,
        3 => return b"H263 MB Info\0" as *const u8 as *const ::core::ffi::c_char,
        4 => return b"Replay Gain\0" as *const u8 as *const ::core::ffi::c_char,
        5 => return b"Display Matrix\0" as *const u8 as *const ::core::ffi::c_char,
        6 => return b"Stereo 3D\0" as *const u8 as *const ::core::ffi::c_char,
        7 => return b"Audio Service Type\0" as *const u8 as *const ::core::ffi::c_char,
        8 => return b"Quality stats\0" as *const u8 as *const ::core::ffi::c_char,
        9 => return b"Fallback track\0" as *const u8 as *const ::core::ffi::c_char,
        10 => return b"CPB properties\0" as *const u8 as *const ::core::ffi::c_char,
        11 => return b"Skip Samples\0" as *const u8 as *const ::core::ffi::c_char,
        12 => return b"JP Dual Mono\0" as *const u8 as *const ::core::ffi::c_char,
        13 => return b"Strings Metadata\0" as *const u8 as *const ::core::ffi::c_char,
        14 => return b"Subtitle Position\0" as *const u8 as *const ::core::ffi::c_char,
        15 => {
            return b"Matroska BlockAdditional\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        16 => return b"WebVTT ID\0" as *const u8 as *const ::core::ffi::c_char,
        17 => return b"WebVTT Settings\0" as *const u8 as *const ::core::ffi::c_char,
        18 => return b"Metadata Update\0" as *const u8 as *const ::core::ffi::c_char,
        19 => return b"MPEGTS Stream ID\0" as *const u8 as *const ::core::ffi::c_char,
        20 => {
            return b"Mastering display metadata\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        22 => {
            return b"Content light level metadata\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        21 => return b"Spherical Mapping\0" as *const u8 as *const ::core::ffi::c_char,
        23 => return b"A53 Closed Captions\0" as *const u8 as *const ::core::ffi::c_char,
        24 => {
            return b"Encryption initialization data\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        25 => return b"Encryption info\0" as *const u8 as *const ::core::ffi::c_char,
        26 => {
            return b"Active Format Description data\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        27 => {
            return b"Producer Reference Time\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        28 => return b"ICC Profile\0" as *const u8 as *const ::core::ffi::c_char,
        29 => {
            return b"DOVI configuration record\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        30 => {
            return b"SMPTE ST 12-1:2014 timecode\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        31 => {
            return b"HDR10+ Dynamic Metadata (SMPTE 2094-40)\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        41 => {
            return b"HDR Dynamic Metadata (SMPTE 2094-50)\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        35 => {
            return b"Ambient viewing environment\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        32 => {
            return b"IAMF Mix Gain Parameter Data\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        33 => {
            return b"IAMF Demixing Info Parameter Data\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        34 => {
            return b"IAMF Recon Gain Info Parameter Data\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        36 => return b"Frame Cropping\0" as *const u8 as *const ::core::ffi::c_char,
        37 => return b"LCEVC NAL data\0" as *const u8 as *const ::core::ffi::c_char,
        38 => {
            return b"3D Reference Displays Info\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        39 => return b"RTCP Sender Report\0" as *const u8 as *const ::core::ffi::c_char,
        40 => return b"EXIF metadata\0" as *const u8 as *const ::core::ffi::c_char,
        42 => {
            return b"HEVC enhancement-layer decoder configuration\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn av_packet_pack_dictionary(
    mut dict: *const AVDictionary,
    mut size: *mut size_t,
) -> *mut uint8_t {
    let mut data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    *size = 0 as size_t;
    if dict.is_null() {
        return ::core::ptr::null_mut::<uint8_t>();
    }
    let mut pass: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while pass < 2 as ::core::ffi::c_int {
        let mut t: *const AVDictionaryEntry = ::core::ptr::null::<AVDictionaryEntry>();
        let mut total_length: size_t = 0 as size_t;
        loop {
            t = av_dict_iterate(dict, t);
            if t.is_null() {
                break;
            }
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < 2 as ::core::ffi::c_int {
                let mut str: *const ::core::ffi::c_char = if i != 0 {
                    (*t).value
                } else {
                    (*t).key
                };
                let len: size_t = (strlen(str) as size_t).wrapping_add(1 as size_t);
                if pass != 0 {
                    memcpy(
                        data.offset(total_length as isize) as *mut ::core::ffi::c_void,
                        str as *const ::core::ffi::c_void,
                        len as size_t,
                    );
                } else if len > (0xffffffff as size_t).wrapping_sub(total_length) {
                    return ::core::ptr::null_mut::<uint8_t>()
                }
                total_length = total_length.wrapping_add(len);
                i += 1;
            }
        }
        if pass != 0 {
            break;
        }
        data = av_malloc(total_length) as *mut uint8_t;
        if data.is_null() {
            return ::core::ptr::null_mut::<uint8_t>();
        }
        *size = total_length;
        pass += 1;
    }
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn av_packet_unpack_dictionary(
    mut data: *const uint8_t,
    mut size: size_t,
    mut dict: *mut *mut AVDictionary,
) -> ::core::ffi::c_int {
    let mut end: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut ret: ::core::ffi::c_int = 0;
    if dict.is_null() || data.is_null() || size == 0 {
        return 0 as ::core::ffi::c_int;
    }
    end = data.offset(size as isize);
    if *end.offset(-(1 as ::core::ffi::c_int) as isize) != 0 {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    while data < end {
        let mut key: *const uint8_t = data;
        let mut val: *const uint8_t = data
            .offset(strlen(key as *const ::core::ffi::c_char) as isize)
            .offset(1 as ::core::ffi::c_int as isize);
        if val >= end || *key == 0 {
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        ret = av_dict_set(
            dict,
            key as *const ::core::ffi::c_char,
            val as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        );
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        data = val
            .offset(strlen(val as *const ::core::ffi::c_char) as isize)
            .offset(1 as ::core::ffi::c_int as isize);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_packet_shrink_side_data(
    mut pkt: *mut AVPacket,
    mut type_0: AVPacketSideDataType,
    mut size: size_t,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*pkt).side_data_elems {
        if (*(*pkt).side_data.offset(i as isize)).type_0 as ::core::ffi::c_uint
            == type_0 as ::core::ffi::c_uint
        {
            if size > (*(*pkt).side_data.offset(i as isize)).size {
                return -(12 as ::core::ffi::c_int);
            }
            (*(*pkt).side_data.offset(i as isize)).size = size;
            return 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return -(2 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn av_packet_copy_props(
    mut dst: *mut AVPacket,
    mut src: *const AVPacket,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    (*dst).pts = (*src).pts;
    (*dst).dts = (*src).dts;
    (*dst).pos = (*src).pos;
    (*dst).duration = (*src).duration;
    (*dst).flags = (*src).flags;
    (*dst).stream_index = (*src).stream_index;
    (*dst).opaque = (*src).opaque;
    (*dst).time_base = (*src).time_base;
    (*dst).opaque_ref = ::core::ptr::null_mut::<AVBufferRef>();
    (*dst).side_data = ::core::ptr::null_mut::<AVPacketSideData>();
    (*dst).side_data_elems = 0 as ::core::ffi::c_int;
    ret = av_buffer_replace(&raw mut (*dst).opaque_ref, (*src).opaque_ref);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*src).side_data_elems {
        let mut type_0: AVPacketSideDataType = (*(*src).side_data.offset(i as isize))
            .type_0;
        let mut size: size_t = (*(*src).side_data.offset(i as isize)).size;
        let mut src_data: *mut uint8_t = (*(*src).side_data.offset(i as isize)).data;
        let mut dst_data: *mut uint8_t = av_packet_new_side_data(dst, type_0, size);
        if dst_data.is_null() {
            av_buffer_unref(&raw mut (*dst).opaque_ref);
            av_packet_free_side_data(dst);
            return -(12 as ::core::ffi::c_int);
        }
        memcpy(
            dst_data as *mut ::core::ffi::c_void,
            src_data as *const ::core::ffi::c_void,
            size as size_t,
        );
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_packet_unref(mut pkt: *mut AVPacket) {
    av_packet_free_side_data(pkt);
    av_buffer_unref(&raw mut (*pkt).opaque_ref);
    av_buffer_unref(&raw mut (*pkt).buf);
    get_packet_defaults(pkt);
}
#[no_mangle]
pub unsafe extern "C" fn av_packet_ref(
    mut dst: *mut AVPacket,
    mut src: *const AVPacket,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut ret: ::core::ffi::c_int = 0;
    (*dst).buf = ::core::ptr::null_mut::<AVBufferRef>();
    ret = av_packet_copy_props(dst, src);
    if !(ret < 0 as ::core::ffi::c_int) {
        if (*src).buf.is_null() {
            ret = packet_alloc(&raw mut (*dst).buf, (*src).size);
            if ret < 0 as ::core::ffi::c_int {
                current_block = 13279962923875062013;
            } else {
                if (*src).size != 0 {
                    memcpy(
                        (*(*dst).buf).data as *mut ::core::ffi::c_void,
                        (*src).data as *const ::core::ffi::c_void,
                        (*src).size as size_t,
                    );
                }
                (*dst).data = (*(*dst).buf).data;
                current_block = 5399440093318478209;
            }
        } else {
            (*dst).buf = av_buffer_ref((*src).buf);
            if (*dst).buf.is_null() {
                ret = -(12 as ::core::ffi::c_int);
                current_block = 13279962923875062013;
            } else {
                (*dst).data = (*src).data;
                current_block = 5399440093318478209;
            }
        }
        match current_block {
            13279962923875062013 => {}
            _ => {
                (*dst).size = (*src).size;
                return 0 as ::core::ffi::c_int;
            }
        }
    }
    av_packet_unref(dst);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_packet_clone(mut src: *const AVPacket) -> *mut AVPacket {
    let mut ret: *mut AVPacket = av_packet_alloc();
    if ret.is_null() {
        return ret;
    }
    if av_packet_ref(ret, src) != 0 {
        av_packet_free(&raw mut ret);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_packet_move_ref(
    mut dst: *mut AVPacket,
    mut src: *mut AVPacket,
) {
    *dst = *src;
    get_packet_defaults(src);
}
#[no_mangle]
pub unsafe extern "C" fn av_packet_make_refcounted(
    mut pkt: *mut AVPacket,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    if !(*pkt).buf.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    ret = packet_alloc(&raw mut (*pkt).buf, (*pkt).size);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    if (*pkt).size != 0 {
        memcpy(
            (*(*pkt).buf).data as *mut ::core::ffi::c_void,
            (*pkt).data as *const ::core::ffi::c_void,
            (*pkt).size as size_t,
        );
    }
    (*pkt).data = (*(*pkt).buf).data;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_packet_make_writable(
    mut pkt: *mut AVPacket,
) -> ::core::ffi::c_int {
    let mut buf: *mut AVBufferRef = ::core::ptr::null_mut::<AVBufferRef>();
    let mut ret: ::core::ffi::c_int = 0;
    if !(*pkt).buf.is_null() && av_buffer_is_writable((*pkt).buf) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    ret = packet_alloc(&raw mut buf, (*pkt).size);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    if (*pkt).size != 0 {
        memcpy(
            (*buf).data as *mut ::core::ffi::c_void,
            (*pkt).data as *const ::core::ffi::c_void,
            (*pkt).size as size_t,
        );
    }
    av_buffer_unref(&raw mut (*pkt).buf);
    (*pkt).buf = buf;
    (*pkt).data = (*buf).data;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_packet_rescale_ts(
    mut pkt: *mut AVPacket,
    mut src_tb: AVRational,
    mut dst_tb: AVRational,
) {
    if (*pkt).pts != 0x8000000000000000 as ::core::ffi::c_ulonglong as int64_t {
        (*pkt).pts = av_rescale_q((*pkt).pts, src_tb, dst_tb);
    }
    if (*pkt).dts != 0x8000000000000000 as ::core::ffi::c_ulonglong as int64_t {
        (*pkt).dts = av_rescale_q((*pkt).dts, src_tb, dst_tb);
    }
    if (*pkt).duration > 0 as int64_t {
        (*pkt).duration = av_rescale_q((*pkt).duration, src_tb, dst_tb);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ff_side_data_set_prft(
    mut pkt: *mut AVPacket,
    mut timestamp: int64_t,
) -> ::core::ffi::c_int {
    let mut prft: *mut AVProducerReferenceTime = ::core::ptr::null_mut::<
        AVProducerReferenceTime,
    >();
    let mut side_data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut side_data_size: size_t = 0;
    side_data = av_packet_get_side_data(pkt, AV_PKT_DATA_PRFT, &raw mut side_data_size);
    if side_data.is_null() {
        side_data_size = ::core::mem::size_of::<AVProducerReferenceTime>() as usize
            as size_t;
        side_data = av_packet_new_side_data(pkt, AV_PKT_DATA_PRFT, side_data_size);
    }
    if side_data.is_null()
        || (side_data_size as usize)
            < ::core::mem::size_of::<AVProducerReferenceTime>() as usize
    {
        return -(12 as ::core::ffi::c_int);
    }
    prft = side_data as *mut AVProducerReferenceTime;
    (*prft).wallclock = timestamp;
    (*prft).flags = 0 as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_packet_side_data_get(
    mut sd: *const AVPacketSideData,
    mut nb_sd: ::core::ffi::c_int,
    mut type_0: AVPacketSideDataType,
) -> *const AVPacketSideData {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < nb_sd {
        if (*sd.offset(i as isize)).type_0 as ::core::ffi::c_uint
            == type_0 as ::core::ffi::c_uint
        {
            return sd.offset(i as isize) as *const AVPacketSideData;
        }
        i += 1;
    }
    return ::core::ptr::null::<AVPacketSideData>();
}
unsafe extern "C" fn packet_side_data_add(
    mut psd: *mut *mut AVPacketSideData,
    mut pnb_sd: *mut ::core::ffi::c_int,
    mut type_0: AVPacketSideDataType,
    mut data: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> *mut AVPacketSideData {
    let mut sd: *mut AVPacketSideData = *psd;
    let mut tmp: *mut AVPacketSideData = ::core::ptr::null_mut::<AVPacketSideData>();
    let mut nb_sd: ::core::ffi::c_int = *pnb_sd;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < nb_sd {
        if (*sd.offset(i as isize)).type_0 as ::core::ffi::c_uint
            != type_0 as ::core::ffi::c_uint
        {
            i += 1;
        } else {
            av_free((*sd.offset(i as isize)).data as *mut ::core::ffi::c_void);
            let ref mut fresh0 = (*sd.offset(i as isize)).data;
            *fresh0 = data as *mut uint8_t;
            (*sd.offset(i as isize)).size = size;
            return sd.offset(i as isize) as *mut AVPacketSideData;
        }
    }
    if nb_sd == 2147483647 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<AVPacketSideData>();
    }
    tmp = av_realloc_array(
        sd as *mut ::core::ffi::c_void,
        (nb_sd + 1 as ::core::ffi::c_int) as size_t,
        ::core::mem::size_of::<AVPacketSideData>() as size_t,
    ) as *mut AVPacketSideData;
    if tmp.is_null() {
        return ::core::ptr::null_mut::<AVPacketSideData>();
    }
    sd = tmp;
    *psd = sd;
    (*sd.offset(nb_sd as isize)).type_0 = type_0;
    let ref mut fresh1 = (*sd.offset(nb_sd as isize)).data;
    *fresh1 = data as *mut uint8_t;
    (*sd.offset(nb_sd as isize)).size = size;
    *pnb_sd = nb_sd + 1 as ::core::ffi::c_int;
    return sd.offset(nb_sd as isize) as *mut AVPacketSideData;
}
#[no_mangle]
pub unsafe extern "C" fn av_packet_side_data_add(
    mut psd: *mut *mut AVPacketSideData,
    mut pnb_sd: *mut ::core::ffi::c_int,
    mut type_0: AVPacketSideDataType,
    mut data: *mut ::core::ffi::c_void,
    mut size: size_t,
    mut flags: ::core::ffi::c_int,
) -> *mut AVPacketSideData {
    return packet_side_data_add(psd, pnb_sd, type_0, data, size);
}
#[no_mangle]
pub unsafe extern "C" fn av_packet_side_data_new(
    mut psd: *mut *mut AVPacketSideData,
    mut pnb_sd: *mut ::core::ffi::c_int,
    mut type_0: AVPacketSideDataType,
    mut size: size_t,
    mut flags: ::core::ffi::c_int,
) -> *mut AVPacketSideData {
    let mut sd: *mut AVPacketSideData = ::core::ptr::null_mut::<AVPacketSideData>();
    let mut data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    if size > (0xffffffff as ::core::ffi::c_uint).wrapping_sub(64 as ::core::ffi::c_uint)
    {
        return ::core::ptr::null_mut::<AVPacketSideData>();
    }
    data = av_malloc(size.wrapping_add(64 as size_t)) as *mut uint8_t;
    if data.is_null() {
        return ::core::ptr::null_mut::<AVPacketSideData>();
    }
    memset(
        data.offset(size as isize) as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        64 as size_t,
    );
    sd = packet_side_data_add(
        psd,
        pnb_sd,
        type_0,
        data as *mut ::core::ffi::c_void,
        size,
    );
    if sd.is_null() {
        av_freep(&raw mut data as *mut ::core::ffi::c_void);
    }
    return sd;
}
#[no_mangle]
pub unsafe extern "C" fn av_packet_side_data_remove(
    mut sd: *mut AVPacketSideData,
    mut pnb_sd: *mut ::core::ffi::c_int,
    mut type_0: AVPacketSideDataType,
) {
    let mut nb_sd: ::core::ffi::c_int = *pnb_sd;
    let mut i: ::core::ffi::c_int = nb_sd - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        if (*sd.offset(i as isize)).type_0 as ::core::ffi::c_uint
            != type_0 as ::core::ffi::c_uint
        {
            i -= 1;
        } else {
            av_free((*sd.offset(i as isize)).data as *mut ::core::ffi::c_void);
            nb_sd -= 1;
            *sd.offset(i as isize) = *sd.offset(nb_sd as isize);
            break;
        }
    }
    *pnb_sd = nb_sd;
}
#[no_mangle]
pub unsafe extern "C" fn av_packet_side_data_free(
    mut psd: *mut *mut AVPacketSideData,
    mut pnb_sd: *mut ::core::ffi::c_int,
) {
    let mut sd: *mut AVPacketSideData = *psd;
    let mut nb_sd: ::core::ffi::c_int = *pnb_sd;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < nb_sd {
        av_free((*sd.offset(i as isize)).data as *mut ::core::ffi::c_void);
        i += 1;
    }
    av_freep(psd as *mut ::core::ffi::c_void);
    *pnb_sd = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn container_packet_alloc(
    mut opaque: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    return av_packet_alloc() as *mut ::core::ffi::c_void;
}
unsafe extern "C" fn container_packet_reset(
    mut opaque: *mut ::core::ffi::c_void,
    mut obj: *mut ::core::ffi::c_void,
) {
    av_packet_unref(obj as *mut AVPacket);
}
unsafe extern "C" fn container_packet_free(
    mut opaque: *mut ::core::ffi::c_void,
    mut obj: *mut ::core::ffi::c_void,
) {
    let mut pkt: *mut AVPacket = obj as *mut AVPacket;
    av_packet_free(&raw mut pkt);
}
unsafe extern "C" fn container_packet_transfer(
    mut opaque: *mut ::core::ffi::c_void,
    mut dst: *mut ::core::ffi::c_void,
    mut src: *mut ::core::ffi::c_void,
    mut flags: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if flags & AV_CONTAINER_FIFO_FLAG_REF as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        return av_packet_ref(dst as *mut AVPacket, src as *const AVPacket);
    }
    av_packet_move_ref(dst as *mut AVPacket, src as *mut AVPacket);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_container_fifo_alloc_avpacket(
    mut flags: ::core::ffi::c_uint,
) -> *mut AVContainerFifo {
    return av_container_fifo_alloc(
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        Some(
            container_packet_alloc
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                ) -> *mut ::core::ffi::c_void,
        ),
        Some(
            container_packet_reset
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        Some(
            container_packet_free
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        Some(
            container_packet_transfer
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_uint,
                ) -> ::core::ffi::c_int,
        ),
        0 as ::core::ffi::c_uint,
    ) as *mut AVContainerFifo;
}
#[repr(C)]
pub struct AVContainerFifo {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVBuffer {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVDictionary {
    _opaque: [u8; 0],
}
