// Translated from FFmpeg 9.0.1 libavutil/error.c
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
    fn strerror_r(
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn av_strlcpy(
        dst: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        size: size_t,
    ) -> size_t;
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}
pub type size_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ErrorEntry {
    pub num: ::core::ffi::c_int,
    pub offset: ::core::ffi::c_uint,
}
pub const ERROR_HTTP_SERVER_ERROR_OFFSET: C2RustUnnamed = 746;
pub const ERROR_HTTP_OTHER_4XX_OFFSET: C2RustUnnamed = 685;
pub const ERROR_HTTP_TOO_MANY_REQUESTS_OFFSET: C2RustUnnamed = 647;
pub const ERROR_HTTP_NOT_FOUND_OFFSET: C2RustUnnamed = 617;
pub const ERROR_HTTP_FORBIDDEN_OFFSET: C2RustUnnamed = 571;
pub const ERROR_HTTP_UNAUTHORIZED_OFFSET: C2RustUnnamed = 515;
pub const ERROR_HTTP_BAD_REQUEST_OFFSET: C2RustUnnamed = 483;
pub const ERROR_INPUT_AND_OUTPUT_CHANGED_OFFSET: C2RustUnnamed = 458;
pub const ERROR_EXPERIMENTAL_OFFSET: C2RustUnnamed = 437;
pub const ERROR_UNKNOWN_OFFSET: C2RustUnnamed = 414;
pub const ERROR_STREAM_NOT_FOUND_OFFSET: C2RustUnnamed = 397;
pub const ERROR_PROTOCOL_NOT_FOUND_OFFSET: C2RustUnnamed = 378;
pub const ERROR_PATCHWELCOME_OFFSET: C2RustUnnamed = 331;
pub const ERROR_OUTPUT_CHANGED_OFFSET: C2RustUnnamed = 316;
pub const ERROR_OPTION_NOT_FOUND_OFFSET: C2RustUnnamed = 299;
pub const ERROR_MUXER_NOT_FOUND_OFFSET: C2RustUnnamed = 283;
pub const ERROR_INVALIDDATA_OFFSET: C2RustUnnamed = 242;
pub const ERROR_INPUT_CHANGED_OFFSET: C2RustUnnamed = 228;
pub const ERROR_FILTER_NOT_FOUND_OFFSET: C2RustUnnamed = 211;
pub const ERROR_EXTERNAL_OFFSET: C2RustUnnamed = 174;
pub const ERROR_EXIT_OFFSET: C2RustUnnamed = 149;
pub const ERROR_EOF_OFFSET: C2RustUnnamed = 137;
pub const ERROR_ENCODER_NOT_FOUND_OFFSET: C2RustUnnamed = 119;
pub const ERROR_DEMUXER_NOT_FOUND_OFFSET: C2RustUnnamed = 101;
pub const ERROR_DECODER_NOT_FOUND_OFFSET: C2RustUnnamed = 83;
pub const ERROR_BUFFER_TOO_SMALL_OFFSET: C2RustUnnamed = 66;
pub const ERROR_BUG_OFFSET: C2RustUnnamed = 27;
pub const ERROR_BSF_NOT_FOUND_OFFSET: C2RustUnnamed = 0;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const ERROR_HTTP_SERVER_ERROR_END_OFFSET: C2RustUnnamed = 784;
pub const ERROR_HTTP_OTHER_4XX_END_OFFSET: C2RustUnnamed = 745;
pub const ERROR_HTTP_TOO_MANY_REQUESTS_END_OFFSET: C2RustUnnamed = 684;
pub const ERROR_HTTP_NOT_FOUND_END_OFFSET: C2RustUnnamed = 646;
pub const ERROR_HTTP_FORBIDDEN_END_OFFSET: C2RustUnnamed = 616;
pub const ERROR_HTTP_UNAUTHORIZED_END_OFFSET: C2RustUnnamed = 570;
pub const ERROR_HTTP_BAD_REQUEST_END_OFFSET: C2RustUnnamed = 514;
pub const ERROR_INPUT_AND_OUTPUT_CHANGED_END_OFFSET: C2RustUnnamed = 482;
pub const ERROR_EXPERIMENTAL_END_OFFSET: C2RustUnnamed = 457;
pub const ERROR_UNKNOWN_END_OFFSET: C2RustUnnamed = 436;
pub const ERROR_STREAM_NOT_FOUND_END_OFFSET: C2RustUnnamed = 413;
pub const ERROR_PROTOCOL_NOT_FOUND_END_OFFSET: C2RustUnnamed = 396;
pub const ERROR_PATCHWELCOME_END_OFFSET: C2RustUnnamed = 377;
pub const ERROR_OUTPUT_CHANGED_END_OFFSET: C2RustUnnamed = 330;
pub const ERROR_OPTION_NOT_FOUND_END_OFFSET: C2RustUnnamed = 315;
pub const ERROR_MUXER_NOT_FOUND_END_OFFSET: C2RustUnnamed = 298;
pub const ERROR_INVALIDDATA_END_OFFSET: C2RustUnnamed = 282;
pub const ERROR_INPUT_CHANGED_END_OFFSET: C2RustUnnamed = 241;
pub const ERROR_FILTER_NOT_FOUND_END_OFFSET: C2RustUnnamed = 227;
pub const ERROR_EXTERNAL_END_OFFSET: C2RustUnnamed = 210;
pub const ERROR_EXIT_END_OFFSET: C2RustUnnamed = 173;
pub const ERROR_EOF_END_OFFSET: C2RustUnnamed = 148;
pub const ERROR_ENCODER_NOT_FOUND_END_OFFSET: C2RustUnnamed = 136;
pub const ERROR_DEMUXER_NOT_FOUND_END_OFFSET: C2RustUnnamed = 118;
pub const ERROR_DECODER_NOT_FOUND_END_OFFSET: C2RustUnnamed = 100;
pub const ERROR_BUFFER_TOO_SMALL_END_OFFSET: C2RustUnnamed = 82;
pub const ERROR_BUG_END_OFFSET: C2RustUnnamed = 65;
pub const ERROR_BSF_NOT_FOUND_END_OFFSET: C2RustUnnamed = 26;
static mut error_stringtable: [::core::ffi::c_char; 786] = unsafe {
    ::core::mem::transmute::<
        [u8; 786],
        [::core::ffi::c_char; 786],
    >(
        *b"Bitstream filter not found\0Internal bug, should not have happened\0Buffer too small\0Decoder not found\0Demuxer not found\0Encoder not found\0End of file\0Immediate exit requested\0Generic error in an external library\0Filter not found\0Input changed\0Invalid data found when processing input\0Muxer not found\0Option not found\0Output changed\0Not yet implemented in FFmpeg, patches welcome\0Protocol not found\0Stream not found\0Unknown error occurred\0Experimental feature\0Input and output changed\0Server returned 400 Bad Request\0Server returned 401 Unauthorized (authorization failed)\0Server returned 403 Forbidden (access denied)\0Server returned 404 Not Found\0Server returned 429 Too Many Requests\0Server returned 4XX Client Error, but not one of 40{0,1,3,4}\0Server returned 5XX Server Error reply\0\0",
    )
};
static mut error_entries: [ErrorEntry; 29] = [
    ErrorEntry {
        num: -(((0xf8 as ::core::ffi::c_int | ('B' as i32) << 8 as ::core::ffi::c_int
            | ('S' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('F' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int),
        offset: ERROR_BSF_NOT_FOUND_OFFSET as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    ErrorEntry {
        num: -((('B' as i32 | ('U' as i32) << 8 as ::core::ffi::c_int
            | ('G' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('!' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int),
        offset: ERROR_BUG_OFFSET as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    ErrorEntry {
        num: -((('B' as i32 | ('U' as i32) << 8 as ::core::ffi::c_int
            | ('G' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | (' ' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int),
        offset: ERROR_BUG_OFFSET as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    ErrorEntry {
        num: -((('B' as i32 | ('U' as i32) << 8 as ::core::ffi::c_int
            | ('F' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('S' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int),
        offset: ERROR_BUFFER_TOO_SMALL_OFFSET as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    },
    ErrorEntry {
        num: -(((0xf8 as ::core::ffi::c_int | ('D' as i32) << 8 as ::core::ffi::c_int
            | ('E' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('C' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int),
        offset: ERROR_DECODER_NOT_FOUND_OFFSET as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    },
    ErrorEntry {
        num: -(((0xf8 as ::core::ffi::c_int | ('D' as i32) << 8 as ::core::ffi::c_int
            | ('E' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('M' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int),
        offset: ERROR_DEMUXER_NOT_FOUND_OFFSET as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    },
    ErrorEntry {
        num: -(((0xf8 as ::core::ffi::c_int | ('E' as i32) << 8 as ::core::ffi::c_int
            | ('N' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('C' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int),
        offset: ERROR_ENCODER_NOT_FOUND_OFFSET as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    },
    ErrorEntry {
        num: -((('E' as i32 | ('O' as i32) << 8 as ::core::ffi::c_int
            | ('F' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | (' ' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int),
        offset: ERROR_EOF_OFFSET as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    ErrorEntry {
        num: -((('E' as i32 | ('X' as i32) << 8 as ::core::ffi::c_int
            | ('I' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('T' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int),
        offset: ERROR_EXIT_OFFSET as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    ErrorEntry {
        num: -((('E' as i32 | ('X' as i32) << 8 as ::core::ffi::c_int
            | ('T' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | (' ' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int),
        offset: ERROR_EXTERNAL_OFFSET as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    ErrorEntry {
        num: -(((0xf8 as ::core::ffi::c_int | ('F' as i32) << 8 as ::core::ffi::c_int
            | ('I' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('L' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int),
        offset: ERROR_FILTER_NOT_FOUND_OFFSET as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    },
    ErrorEntry {
        num: -(0x636e6701 as ::core::ffi::c_int),
        offset: ERROR_INPUT_CHANGED_OFFSET as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    ErrorEntry {
        num: -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int),
        offset: ERROR_INVALIDDATA_OFFSET as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    ErrorEntry {
        num: -(((0xf8 as ::core::ffi::c_int | ('M' as i32) << 8 as ::core::ffi::c_int
            | ('U' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('X' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int),
        offset: ERROR_MUXER_NOT_FOUND_OFFSET as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    ErrorEntry {
        num: -(((0xf8 as ::core::ffi::c_int | ('O' as i32) << 8 as ::core::ffi::c_int
            | ('P' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('T' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int),
        offset: ERROR_OPTION_NOT_FOUND_OFFSET as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    },
    ErrorEntry {
        num: -(0x636e6702 as ::core::ffi::c_int),
        offset: ERROR_OUTPUT_CHANGED_OFFSET as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    ErrorEntry {
        num: -((('P' as i32 | ('A' as i32) << 8 as ::core::ffi::c_int
            | ('W' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('E' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int),
        offset: ERROR_PATCHWELCOME_OFFSET as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    ErrorEntry {
        num: -(((0xf8 as ::core::ffi::c_int | ('P' as i32) << 8 as ::core::ffi::c_int
            | ('R' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('O' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int),
        offset: ERROR_PROTOCOL_NOT_FOUND_OFFSET as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    },
    ErrorEntry {
        num: -(((0xf8 as ::core::ffi::c_int | ('S' as i32) << 8 as ::core::ffi::c_int
            | ('T' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('R' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int),
        offset: ERROR_STREAM_NOT_FOUND_OFFSET as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    },
    ErrorEntry {
        num: -((('U' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('K' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('N' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int),
        offset: ERROR_UNKNOWN_OFFSET as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    ErrorEntry {
        num: -(0x2bb2afa8 as ::core::ffi::c_int),
        offset: ERROR_EXPERIMENTAL_OFFSET as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    ErrorEntry {
        num: -(0x636e6701 as ::core::ffi::c_int) | -(0x636e6702 as ::core::ffi::c_int),
        offset: ERROR_INPUT_AND_OUTPUT_CHANGED_OFFSET as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    },
    ErrorEntry {
        num: -(((0xf8 as ::core::ffi::c_int | ('4' as i32) << 8 as ::core::ffi::c_int
            | ('0' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('0' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int),
        offset: ERROR_HTTP_BAD_REQUEST_OFFSET as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    },
    ErrorEntry {
        num: -(((0xf8 as ::core::ffi::c_int | ('4' as i32) << 8 as ::core::ffi::c_int
            | ('0' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('1' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int),
        offset: ERROR_HTTP_UNAUTHORIZED_OFFSET as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    },
    ErrorEntry {
        num: -(((0xf8 as ::core::ffi::c_int | ('4' as i32) << 8 as ::core::ffi::c_int
            | ('0' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('3' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int),
        offset: ERROR_HTTP_FORBIDDEN_OFFSET as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    ErrorEntry {
        num: -(((0xf8 as ::core::ffi::c_int | ('4' as i32) << 8 as ::core::ffi::c_int
            | ('0' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('4' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int),
        offset: ERROR_HTTP_NOT_FOUND_OFFSET as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    ErrorEntry {
        num: -(((0xf8 as ::core::ffi::c_int | ('4' as i32) << 8 as ::core::ffi::c_int
            | ('2' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('9' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int),
        offset: ERROR_HTTP_TOO_MANY_REQUESTS_OFFSET as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    },
    ErrorEntry {
        num: -(((0xf8 as ::core::ffi::c_int | ('4' as i32) << 8 as ::core::ffi::c_int
            | ('X' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('X' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int),
        offset: ERROR_HTTP_OTHER_4XX_OFFSET as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    ErrorEntry {
        num: -(((0xf8 as ::core::ffi::c_int | ('5' as i32) << 8 as ::core::ffi::c_int
            | ('X' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('X' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int),
        offset: ERROR_HTTP_SERVER_ERROR_OFFSET as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    },
];
#[no_mangle]
pub unsafe extern "C" fn av_strerror(
    mut errnum: ::core::ffi::c_int,
    mut errbuf: *mut ::core::ffi::c_char,
    mut errbuf_size: size_t,
) -> ::core::ffi::c_int {
    let mut i: size_t = 0 as size_t;
    while (i as usize)
        < (::core::mem::size_of::<[ErrorEntry; 29]>() as usize)
            .wrapping_div(::core::mem::size_of::<ErrorEntry>() as usize)
    {
        if errnum == error_entries[i as usize].num {
            av_strlcpy(
                errbuf,
                (&raw const error_stringtable as *const ::core::ffi::c_char)
                    .offset(error_entries[i as usize].offset as isize),
                errbuf_size,
            );
            return 0 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    let mut ret: ::core::ffi::c_int = -strerror_r(
        (errnum as ::core::ffi::c_uint).wrapping_neg() as ::core::ffi::c_int,
        errbuf,
        errbuf_size as size_t,
    );
    if ret < 0 as ::core::ffi::c_int {
        snprintf(
            errbuf,
            errbuf_size as size_t,
            b"Error number %d occurred\0" as *const u8 as *const ::core::ffi::c_char,
            errnum,
        );
    }
    return ret;
}
