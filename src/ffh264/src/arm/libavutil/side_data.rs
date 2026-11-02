// Translated from FFmpeg 9.0.1 libavutil/side_data.c
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
    fn av_buffer_alloc(size: size_t) -> *mut AVBufferRef;
    fn av_mallocz(size: size_t) -> *mut ::core::ffi::c_void;
    fn av_buffer_ref(buf: *const AVBufferRef) -> *mut AVBufferRef;
    fn av_buffer_unref(buf: *mut *mut AVBufferRef);
    fn av_dict_copy(
        dst: *mut *mut AVDictionary,
        src: *const AVDictionary,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn av_buffer_replace(
        dst: *mut *mut AVBufferRef,
        src: *const AVBufferRef,
    ) -> ::core::ffi::c_int;
    fn av_dict_free(m: *mut *mut AVDictionary);
    fn av_realloc_array(
        ptr: *mut ::core::ffi::c_void,
        nmemb: size_t,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn av_freep(ptr: *mut ::core::ffi::c_void);
}
pub type size_t = ::core::ffi::c_uint;
pub type uint8_t = ::core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBufferRef {
    pub buffer: *mut AVBuffer,
    pub data: *mut uint8_t,
    pub size: size_t,
}
pub type AVFrameSideDataType = ::core::ffi::c_uint;
pub const AV_FRAME_DATA_RAW_COLOR_PARAMS: AVFrameSideDataType = 36;
pub const AV_FRAME_DATA_IAMF_RECON_GAIN_INFO_PARAM: AVFrameSideDataType = 35;
pub const AV_FRAME_DATA_IAMF_DEMIXING_INFO_PARAM: AVFrameSideDataType = 34;
pub const AV_FRAME_DATA_IAMF_MIX_GAIN_PARAM: AVFrameSideDataType = 33;
pub const AV_FRAME_DATA_DYNAMIC_HDR_SMPTE_2094_APP5: AVFrameSideDataType = 32;
pub const AV_FRAME_DATA_EXIF: AVFrameSideDataType = 31;
pub const AV_FRAME_DATA_3D_REFERENCE_DISPLAYS: AVFrameSideDataType = 30;
pub const AV_FRAME_DATA_VIEW_ID: AVFrameSideDataType = 29;
pub const AV_FRAME_DATA_LCEVC: AVFrameSideDataType = 28;
pub const AV_FRAME_DATA_VIDEO_HINT: AVFrameSideDataType = 27;
pub const AV_FRAME_DATA_AMBIENT_VIEWING_ENVIRONMENT: AVFrameSideDataType = 26;
pub const AV_FRAME_DATA_DYNAMIC_HDR_VIVID: AVFrameSideDataType = 25;
pub const AV_FRAME_DATA_DOVI_METADATA: AVFrameSideDataType = 24;
pub const AV_FRAME_DATA_DOVI_RPU_BUFFER: AVFrameSideDataType = 23;
pub const AV_FRAME_DATA_DETECTION_BBOXES: AVFrameSideDataType = 22;
pub const AV_FRAME_DATA_FILM_GRAIN_PARAMS: AVFrameSideDataType = 21;
pub const AV_FRAME_DATA_SEI_UNREGISTERED: AVFrameSideDataType = 20;
pub const AV_FRAME_DATA_VIDEO_ENC_PARAMS: AVFrameSideDataType = 19;
pub const AV_FRAME_DATA_REGIONS_OF_INTEREST: AVFrameSideDataType = 18;
pub const AV_FRAME_DATA_DYNAMIC_HDR_PLUS: AVFrameSideDataType = 17;
pub const AV_FRAME_DATA_S12M_TIMECODE: AVFrameSideDataType = 16;
pub const AV_FRAME_DATA_ICC_PROFILE: AVFrameSideDataType = 15;
pub const AV_FRAME_DATA_CONTENT_LIGHT_LEVEL: AVFrameSideDataType = 14;
pub const AV_FRAME_DATA_SPHERICAL: AVFrameSideDataType = 13;
pub const AV_FRAME_DATA_GOP_TIMECODE: AVFrameSideDataType = 12;
pub const AV_FRAME_DATA_MASTERING_DISPLAY_METADATA: AVFrameSideDataType = 11;
pub const AV_FRAME_DATA_AUDIO_SERVICE_TYPE: AVFrameSideDataType = 10;
pub const AV_FRAME_DATA_SKIP_SAMPLES: AVFrameSideDataType = 9;
pub const AV_FRAME_DATA_MOTION_VECTORS: AVFrameSideDataType = 8;
pub const AV_FRAME_DATA_AFD: AVFrameSideDataType = 7;
pub const AV_FRAME_DATA_DISPLAYMATRIX: AVFrameSideDataType = 6;
pub const AV_FRAME_DATA_REPLAYGAIN: AVFrameSideDataType = 5;
pub const AV_FRAME_DATA_DOWNMIX_INFO: AVFrameSideDataType = 4;
pub const AV_FRAME_DATA_MATRIXENCODING: AVFrameSideDataType = 3;
pub const AV_FRAME_DATA_STEREO3D: AVFrameSideDataType = 2;
pub const AV_FRAME_DATA_A53_CC: AVFrameSideDataType = 1;
pub const AV_FRAME_DATA_PANSCAN: AVFrameSideDataType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFrameSideData {
    pub type_0: AVFrameSideDataType,
    pub data: *mut uint8_t,
    pub size: size_t,
    pub metadata: *mut AVDictionary,
    pub buf: *mut AVBufferRef,
}
pub type AVSideDataProps = ::core::ffi::c_uint;
pub const AV_SIDE_DATA_PROP_CHANNEL_DEPENDENT: AVSideDataProps = 16;
pub const AV_SIDE_DATA_PROP_COLOR_DEPENDENT: AVSideDataProps = 8;
pub const AV_SIDE_DATA_PROP_SIZE_DEPENDENT: AVSideDataProps = 4;
pub const AV_SIDE_DATA_PROP_MULTI: AVSideDataProps = 2;
pub const AV_SIDE_DATA_PROP_GLOBAL: AVSideDataProps = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVSideDataDescriptor {
    pub name: *const ::core::ffi::c_char,
    pub props: ::core::ffi::c_uint,
}
static mut sd_props: [AVSideDataDescriptor; 37] = [
    AVSideDataDescriptor {
        name: b"AVPanScan\0" as *const u8 as *const ::core::ffi::c_char,
        props: AV_SIDE_DATA_PROP_SIZE_DEPENDENT as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    },
    AVSideDataDescriptor {
        name: b"ATSC A53 Part 4 Closed Captions\0" as *const u8
            as *const ::core::ffi::c_char,
        props: 0,
    },
    AVSideDataDescriptor {
        name: b"Stereo 3D\0" as *const u8 as *const ::core::ffi::c_char,
        props: AV_SIDE_DATA_PROP_GLOBAL as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    AVSideDataDescriptor {
        name: b"AVMatrixEncoding\0" as *const u8 as *const ::core::ffi::c_char,
        props: AV_SIDE_DATA_PROP_CHANNEL_DEPENDENT as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    },
    AVSideDataDescriptor {
        name: b"Metadata relevant to a downmix procedure\0" as *const u8
            as *const ::core::ffi::c_char,
        props: AV_SIDE_DATA_PROP_CHANNEL_DEPENDENT as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    },
    AVSideDataDescriptor {
        name: b"AVReplayGain\0" as *const u8 as *const ::core::ffi::c_char,
        props: AV_SIDE_DATA_PROP_GLOBAL as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    AVSideDataDescriptor {
        name: b"3x3 displaymatrix\0" as *const u8 as *const ::core::ffi::c_char,
        props: AV_SIDE_DATA_PROP_GLOBAL as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    AVSideDataDescriptor {
        name: b"Active format description\0" as *const u8 as *const ::core::ffi::c_char,
        props: 0,
    },
    AVSideDataDescriptor {
        name: b"Motion vectors\0" as *const u8 as *const ::core::ffi::c_char,
        props: AV_SIDE_DATA_PROP_SIZE_DEPENDENT as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    },
    AVSideDataDescriptor {
        name: b"Skip samples\0" as *const u8 as *const ::core::ffi::c_char,
        props: 0,
    },
    AVSideDataDescriptor {
        name: b"Audio service type\0" as *const u8 as *const ::core::ffi::c_char,
        props: AV_SIDE_DATA_PROP_GLOBAL as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    AVSideDataDescriptor {
        name: b"Mastering display metadata\0" as *const u8 as *const ::core::ffi::c_char,
        props: (AV_SIDE_DATA_PROP_GLOBAL as ::core::ffi::c_int
            | AV_SIDE_DATA_PROP_COLOR_DEPENDENT as ::core::ffi::c_int)
            as ::core::ffi::c_uint,
    },
    AVSideDataDescriptor {
        name: b"GOP timecode\0" as *const u8 as *const ::core::ffi::c_char,
        props: 0,
    },
    AVSideDataDescriptor {
        name: b"Spherical Mapping\0" as *const u8 as *const ::core::ffi::c_char,
        props: (AV_SIDE_DATA_PROP_GLOBAL as ::core::ffi::c_int
            | AV_SIDE_DATA_PROP_SIZE_DEPENDENT as ::core::ffi::c_int)
            as ::core::ffi::c_uint,
    },
    AVSideDataDescriptor {
        name: b"Content light level metadata\0" as *const u8
            as *const ::core::ffi::c_char,
        props: (AV_SIDE_DATA_PROP_GLOBAL as ::core::ffi::c_int
            | AV_SIDE_DATA_PROP_COLOR_DEPENDENT as ::core::ffi::c_int)
            as ::core::ffi::c_uint,
    },
    AVSideDataDescriptor {
        name: b"ICC profile\0" as *const u8 as *const ::core::ffi::c_char,
        props: (AV_SIDE_DATA_PROP_GLOBAL as ::core::ffi::c_int
            | AV_SIDE_DATA_PROP_COLOR_DEPENDENT as ::core::ffi::c_int)
            as ::core::ffi::c_uint,
    },
    AVSideDataDescriptor {
        name: b"SMPTE 12-1 timecode\0" as *const u8 as *const ::core::ffi::c_char,
        props: 0,
    },
    AVSideDataDescriptor {
        name: b"HDR Dynamic Metadata SMPTE2094-40 (HDR10+)\0" as *const u8
            as *const ::core::ffi::c_char,
        props: AV_SIDE_DATA_PROP_COLOR_DEPENDENT as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    },
    AVSideDataDescriptor {
        name: b"Regions Of Interest\0" as *const u8 as *const ::core::ffi::c_char,
        props: AV_SIDE_DATA_PROP_SIZE_DEPENDENT as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    },
    AVSideDataDescriptor {
        name: b"Video encoding parameters\0" as *const u8 as *const ::core::ffi::c_char,
        props: 0,
    },
    AVSideDataDescriptor {
        name: b"H.26[45] User Data Unregistered SEI message\0" as *const u8
            as *const ::core::ffi::c_char,
        props: AV_SIDE_DATA_PROP_MULTI as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    AVSideDataDescriptor {
        name: b"Film grain parameters\0" as *const u8 as *const ::core::ffi::c_char,
        props: 0,
    },
    AVSideDataDescriptor {
        name: b"Bounding boxes for object detection and classification\0" as *const u8
            as *const ::core::ffi::c_char,
        props: AV_SIDE_DATA_PROP_SIZE_DEPENDENT as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    },
    AVSideDataDescriptor {
        name: b"Dolby Vision RPU Data\0" as *const u8 as *const ::core::ffi::c_char,
        props: AV_SIDE_DATA_PROP_COLOR_DEPENDENT as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    },
    AVSideDataDescriptor {
        name: b"Dolby Vision Metadata\0" as *const u8 as *const ::core::ffi::c_char,
        props: AV_SIDE_DATA_PROP_COLOR_DEPENDENT as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    },
    AVSideDataDescriptor {
        name: b"HDR Dynamic Metadata CUVA 005.1 2021 (Vivid)\0" as *const u8
            as *const ::core::ffi::c_char,
        props: AV_SIDE_DATA_PROP_COLOR_DEPENDENT as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    },
    AVSideDataDescriptor {
        name: b"Ambient viewing environment\0" as *const u8
            as *const ::core::ffi::c_char,
        props: AV_SIDE_DATA_PROP_GLOBAL as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    AVSideDataDescriptor {
        name: b"Encoding video hint\0" as *const u8 as *const ::core::ffi::c_char,
        props: AV_SIDE_DATA_PROP_SIZE_DEPENDENT as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    },
    AVSideDataDescriptor {
        name: b"LCEVC NAL data\0" as *const u8 as *const ::core::ffi::c_char,
        props: AV_SIDE_DATA_PROP_SIZE_DEPENDENT as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    },
    AVSideDataDescriptor {
        name: b"View ID\0" as *const u8 as *const ::core::ffi::c_char,
        props: 0,
    },
    AVSideDataDescriptor {
        name: b"3D Reference Displays Information\0" as *const u8
            as *const ::core::ffi::c_char,
        props: AV_SIDE_DATA_PROP_GLOBAL as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    AVSideDataDescriptor {
        name: b"EXIF metadata\0" as *const u8 as *const ::core::ffi::c_char,
        props: AV_SIDE_DATA_PROP_GLOBAL as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    AVSideDataDescriptor {
        name: b"HDR Dynamic Metadata SMPTE2094-50\0" as *const u8
            as *const ::core::ffi::c_char,
        props: AV_SIDE_DATA_PROP_COLOR_DEPENDENT as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    },
    AVSideDataDescriptor {
        name: b"IAMF Mix Gain Parameter Data\0" as *const u8
            as *const ::core::ffi::c_char,
        props: 0,
    },
    AVSideDataDescriptor {
        name: b"IAMF Demixing Info Parameter Data\0" as *const u8
            as *const ::core::ffi::c_char,
        props: AV_SIDE_DATA_PROP_CHANNEL_DEPENDENT as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    },
    AVSideDataDescriptor {
        name: b"IAMF Recon Gain Info Parameter Data\0" as *const u8
            as *const ::core::ffi::c_char,
        props: 0,
    },
    AVSideDataDescriptor {
        name: b"RAW camera color parameters\0" as *const u8
            as *const ::core::ffi::c_char,
        props: (AV_SIDE_DATA_PROP_GLOBAL as ::core::ffi::c_int
            | AV_SIDE_DATA_PROP_COLOR_DEPENDENT as ::core::ffi::c_int)
            as ::core::ffi::c_uint,
    },
];
#[no_mangle]
pub unsafe extern "C" fn av_frame_side_data_desc(
    mut type_0: AVFrameSideDataType,
) -> *const AVSideDataDescriptor {
    let mut t: ::core::ffi::c_uint = type_0 as ::core::ffi::c_uint;
    if (t as usize)
        < (::core::mem::size_of::<[AVSideDataDescriptor; 37]>() as usize)
            .wrapping_div(::core::mem::size_of::<AVSideDataDescriptor>() as usize)
        && !sd_props[t as usize].name.is_null()
    {
        return (&raw const sd_props as *const AVSideDataDescriptor).offset(t as isize)
            as *const AVSideDataDescriptor;
    }
    return ::core::ptr::null::<AVSideDataDescriptor>();
}
#[no_mangle]
pub unsafe extern "C" fn av_frame_side_data_name(
    mut type_0: AVFrameSideDataType,
) -> *const ::core::ffi::c_char {
    let mut desc: *const AVSideDataDescriptor = av_frame_side_data_desc(type_0);
    return if !desc.is_null() {
        (*desc).name
    } else {
        ::core::ptr::null::<::core::ffi::c_char>()
    };
}
unsafe extern "C" fn free_side_data_entry(mut ptr_sd: *mut *mut AVFrameSideData) {
    let mut sd: *mut AVFrameSideData = *ptr_sd;
    av_buffer_unref(&raw mut (*sd).buf);
    av_dict_free(&raw mut (*sd).metadata);
    av_freep(ptr_sd as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn remove_side_data_by_entry(
    mut sd: *mut *mut *mut AVFrameSideData,
    mut nb_sd: *mut ::core::ffi::c_int,
    mut target: *const AVFrameSideData,
) {
    let mut i: ::core::ffi::c_int = *nb_sd - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        let mut entry: *mut AVFrameSideData = *(*sd).offset(i as isize);
        if entry != target as *mut AVFrameSideData {
            i -= 1;
        } else {
            free_side_data_entry(&raw mut entry);
            let ref mut fresh3 = *(*sd).offset(i as isize);
            *fresh3 = *(*sd).offset((*nb_sd - 1 as ::core::ffi::c_int) as isize);
            *nb_sd -= 1;
            return;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn av_frame_side_data_remove(
    mut sd: *mut *mut *mut AVFrameSideData,
    mut nb_sd: *mut ::core::ffi::c_int,
    mut type_0: AVFrameSideDataType,
) {
    let mut i: ::core::ffi::c_int = *nb_sd - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        let mut entry: *mut AVFrameSideData = *(*sd).offset(i as isize);
        if !((*entry).type_0 as ::core::ffi::c_uint != type_0 as ::core::ffi::c_uint) {
            free_side_data_entry(&raw mut entry);
            let ref mut fresh2 = *(*sd).offset(i as isize);
            *fresh2 = *(*sd).offset((*nb_sd - 1 as ::core::ffi::c_int) as isize);
            *nb_sd -= 1;
        }
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn av_frame_side_data_remove_by_props(
    mut sd: *mut *mut *mut AVFrameSideData,
    mut nb_sd: *mut ::core::ffi::c_int,
    mut props: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = *nb_sd - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        let mut entry: *mut AVFrameSideData = *(*sd).offset(i as isize);
        let mut desc: *const AVSideDataDescriptor = av_frame_side_data_desc(
            (*entry).type_0,
        );
        if !(desc.is_null() || (*desc).props & props as ::core::ffi::c_uint == 0) {
            free_side_data_entry(&raw mut entry);
            let ref mut fresh4 = *(*sd).offset(i as isize);
            *fresh4 = *(*sd).offset((*nb_sd - 1 as ::core::ffi::c_int) as isize);
            *nb_sd -= 1;
        }
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn av_frame_side_data_free(
    mut sd: *mut *mut *mut AVFrameSideData,
    mut nb_sd: *mut ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < *nb_sd {
        free_side_data_entry((*sd).offset(i as isize) as *mut *mut AVFrameSideData);
        i += 1;
    }
    *nb_sd = 0 as ::core::ffi::c_int;
    av_freep(sd as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn add_side_data_from_buf_ext(
    mut sd: *mut *mut *mut AVFrameSideData,
    mut nb_sd: *mut ::core::ffi::c_int,
    mut type_0: AVFrameSideDataType,
    mut buf: *mut AVBufferRef,
    mut data: *mut uint8_t,
    mut size: size_t,
) -> *mut AVFrameSideData {
    let mut ret: *mut AVFrameSideData = ::core::ptr::null_mut::<AVFrameSideData>();
    let mut tmp: *mut *mut AVFrameSideData = ::core::ptr::null_mut::<
        *mut AVFrameSideData,
    >();
    if *nb_sd as ::core::ffi::c_uint
        >= (if 2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint
            > 0xffffffff as ::core::ffi::c_uint
        {
            0xffffffff as ::core::ffi::c_uint
        } else {
            2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint
        })
    {
        return ::core::ptr::null_mut::<AVFrameSideData>();
    }
    tmp = av_realloc_array(
        *sd as *mut ::core::ffi::c_void,
        (*nb_sd + 1 as ::core::ffi::c_int) as size_t,
        ::core::mem::size_of::<*mut AVFrameSideData>() as size_t,
    ) as *mut *mut AVFrameSideData;
    if tmp.is_null() {
        return ::core::ptr::null_mut::<AVFrameSideData>();
    }
    *sd = tmp;
    ret = av_mallocz(::core::mem::size_of::<AVFrameSideData>() as size_t)
        as *mut AVFrameSideData;
    if ret.is_null() {
        return ::core::ptr::null_mut::<AVFrameSideData>();
    }
    (*ret).buf = buf;
    (*ret).data = data;
    (*ret).size = size;
    (*ret).type_0 = type_0;
    let fresh0 = *nb_sd;
    *nb_sd = *nb_sd + 1;
    let ref mut fresh1 = *(*sd).offset(fresh0 as isize);
    *fresh1 = ret;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ff_frame_side_data_add_from_buf(
    mut sd: *mut *mut *mut AVFrameSideData,
    mut nb_sd: *mut ::core::ffi::c_int,
    mut type_0: AVFrameSideDataType,
    mut buf: *mut AVBufferRef,
) -> *mut AVFrameSideData {
    if buf.is_null() {
        return ::core::ptr::null_mut::<AVFrameSideData>();
    }
    return add_side_data_from_buf_ext(sd, nb_sd, type_0, buf, (*buf).data, (*buf).size);
}
unsafe extern "C" fn replace_side_data_from_buf(
    mut dst: *mut AVFrameSideData,
    mut buf: *mut AVBufferRef,
    mut flags: ::core::ffi::c_int,
) -> *mut AVFrameSideData {
    if flags & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int == 0 {
        return ::core::ptr::null_mut::<AVFrameSideData>();
    }
    av_dict_free(&raw mut (*dst).metadata);
    av_buffer_unref(&raw mut (*dst).buf);
    (*dst).buf = buf;
    (*dst).data = (*buf).data;
    (*dst).size = (*buf).size;
    return dst;
}
#[no_mangle]
pub unsafe extern "C" fn av_frame_side_data_new(
    mut sd: *mut *mut *mut AVFrameSideData,
    mut nb_sd: *mut ::core::ffi::c_int,
    mut type_0: AVFrameSideDataType,
    mut size: size_t,
    mut flags: ::core::ffi::c_uint,
) -> *mut AVFrameSideData {
    let mut desc: *const AVSideDataDescriptor = av_frame_side_data_desc(type_0);
    let mut buf: *mut AVBufferRef = av_buffer_alloc(size);
    let mut ret: *mut AVFrameSideData = ::core::ptr::null_mut::<AVFrameSideData>();
    if flags
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as ::core::ffi::c_uint
        != 0
    {
        av_frame_side_data_remove(sd, nb_sd, type_0);
    }
    if (desc.is_null()
        || (*desc).props
            & AV_SIDE_DATA_PROP_MULTI as ::core::ffi::c_int as ::core::ffi::c_uint == 0)
        && {
            ret = av_frame_side_data_get(*sd, *nb_sd, type_0) as *mut AVFrameSideData;
            !ret.is_null()
        }
    {
        ret = replace_side_data_from_buf(ret, buf, flags as ::core::ffi::c_int);
        if ret.is_null() {
            av_buffer_unref(&raw mut buf);
        }
        return ret;
    }
    ret = ff_frame_side_data_add_from_buf(sd, nb_sd, type_0, buf);
    if ret.is_null() {
        av_buffer_unref(&raw mut buf);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_frame_side_data_add(
    mut sd: *mut *mut *mut AVFrameSideData,
    mut nb_sd: *mut ::core::ffi::c_int,
    mut type_0: AVFrameSideDataType,
    mut pbuf: *mut *mut AVBufferRef,
    mut flags: ::core::ffi::c_uint,
) -> *mut AVFrameSideData {
    let mut desc: *const AVSideDataDescriptor = av_frame_side_data_desc(type_0);
    let mut sd_dst: *mut AVFrameSideData = ::core::ptr::null_mut::<AVFrameSideData>();
    let mut buf: *mut AVBufferRef = *pbuf;
    if flags
        & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as ::core::ffi::c_uint
        != 0
        && {
            buf = av_buffer_ref(*pbuf);
            buf.is_null()
        }
    {
        return ::core::ptr::null_mut::<AVFrameSideData>();
    }
    if flags
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as ::core::ffi::c_uint
        != 0
    {
        av_frame_side_data_remove(sd, nb_sd, type_0);
    }
    if (desc.is_null()
        || (*desc).props
            & AV_SIDE_DATA_PROP_MULTI as ::core::ffi::c_int as ::core::ffi::c_uint == 0)
        && {
            sd_dst = av_frame_side_data_get(*sd, *nb_sd, type_0) as *mut AVFrameSideData;
            !sd_dst.is_null()
        }
    {
        sd_dst = replace_side_data_from_buf(sd_dst, buf, flags as ::core::ffi::c_int);
    } else {
        sd_dst = ff_frame_side_data_add_from_buf(sd, nb_sd, type_0, buf);
    }
    if !sd_dst.is_null()
        && flags
            & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                as ::core::ffi::c_uint == 0
    {
        *pbuf = ::core::ptr::null_mut::<AVBufferRef>();
    } else if sd_dst.is_null()
        && flags
            & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                as ::core::ffi::c_uint != 0
    {
        av_buffer_unref(&raw mut buf);
    }
    return sd_dst;
}
#[no_mangle]
pub unsafe extern "C" fn av_frame_side_data_clone(
    mut sd: *mut *mut *mut AVFrameSideData,
    mut nb_sd: *mut ::core::ffi::c_int,
    mut src: *const AVFrameSideData,
    mut flags: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut desc: *const AVSideDataDescriptor = ::core::ptr::null::<
        AVSideDataDescriptor,
    >();
    let mut buf: *mut AVBufferRef = ::core::ptr::null_mut::<AVBufferRef>();
    let mut sd_dst: *mut AVFrameSideData = ::core::ptr::null_mut::<AVFrameSideData>();
    let mut ret: ::core::ffi::c_int = -((('B' as i32
        | ('U' as i32) << 8 as ::core::ffi::c_int
        | ('G' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
        | ('!' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
        as ::core::ffi::c_int);
    if sd.is_null() || src.is_null() || nb_sd.is_null() || *nb_sd != 0 && (*sd).is_null()
    {
        return -(22 as ::core::ffi::c_int);
    }
    desc = av_frame_side_data_desc((*src).type_0);
    if flags
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as ::core::ffi::c_uint
        != 0
    {
        av_frame_side_data_remove(sd, nb_sd, (*src).type_0);
    }
    if (desc.is_null()
        || (*desc).props
            & AV_SIDE_DATA_PROP_MULTI as ::core::ffi::c_int as ::core::ffi::c_uint == 0)
        && {
            sd_dst = av_frame_side_data_get(*sd, *nb_sd, (*src).type_0)
                as *mut AVFrameSideData;
            !sd_dst.is_null()
        }
    {
        let mut dict: *mut AVDictionary = ::core::ptr::null_mut::<AVDictionary>();
        if flags
            & ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                as ::core::ffi::c_uint == 0
        {
            return -(17 as ::core::ffi::c_int);
        }
        ret = av_dict_copy(&raw mut dict, (*src).metadata, 0 as ::core::ffi::c_int);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        ret = av_buffer_replace(&raw mut (*sd_dst).buf, (*src).buf);
        if ret < 0 as ::core::ffi::c_int {
            av_dict_free(&raw mut dict);
            return ret;
        }
        av_dict_free(&raw mut (*sd_dst).metadata);
        (*sd_dst).metadata = dict;
        (*sd_dst).data = (*src).data;
        (*sd_dst).size = (*src).size;
        return 0 as ::core::ffi::c_int;
    }
    buf = av_buffer_ref((*src).buf);
    if buf.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    sd_dst = add_side_data_from_buf_ext(
        sd,
        nb_sd,
        (*src).type_0,
        buf,
        (*src).data,
        (*src).size,
    );
    if sd_dst.is_null() {
        av_buffer_unref(&raw mut buf);
        return -(12 as ::core::ffi::c_int);
    }
    ret = av_dict_copy(
        &raw mut (*sd_dst).metadata,
        (*src).metadata,
        0 as ::core::ffi::c_int,
    );
    if ret < 0 as ::core::ffi::c_int {
        remove_side_data_by_entry(sd, nb_sd, sd_dst);
        return ret;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_frame_side_data_get_c(
    mut sd: *const *const AVFrameSideData,
    nb_sd: ::core::ffi::c_int,
    mut type_0: AVFrameSideDataType,
) -> *const AVFrameSideData {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < nb_sd {
        if (**sd.offset(i as isize)).type_0 as ::core::ffi::c_uint
            == type_0 as ::core::ffi::c_uint
        {
            return *sd.offset(i as isize);
        }
        i += 1;
    }
    return ::core::ptr::null::<AVFrameSideData>();
}
#[inline]
unsafe extern "C" fn av_frame_side_data_get(
    mut sd: *const *mut AVFrameSideData,
    nb_sd: ::core::ffi::c_int,
    mut type_0: AVFrameSideDataType,
) -> *const AVFrameSideData {
    return av_frame_side_data_get_c(sd as *const *const AVFrameSideData, nb_sd, type_0);
}
#[repr(C)]
pub struct AVBuffer {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVDictionary {
    _opaque: [u8; 0],
}
