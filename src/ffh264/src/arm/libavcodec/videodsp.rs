// Translated from FFmpeg 9.0.1 libavcodec/videodsp.c
// (preprocessed for armv7-unknown-linux-musleabihf) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// Copyright (C) 2012 Ronald S. Bultje
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
    fn ff_videodsp_init_arm(ctx: *mut VideoDSPContext, bpc: ::core::ffi::c_int);
}
pub type size_t = ::core::ffi::c_uint;
pub type ptrdiff_t = ::core::ffi::c_int;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint16_t = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoDSPContext {
    pub emulated_edge_mc: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            *const uint8_t,
            ptrdiff_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch: Option<
        unsafe extern "C" fn(*const uint8_t, ptrdiff_t, ::core::ffi::c_int) -> (),
    >,
}
unsafe extern "C" fn emulated_edge_mc_16(
    mut buf: *mut uint8_t,
    mut src: *const uint8_t,
    mut buf_linesize: ptrdiff_t,
    mut src_linesize: ptrdiff_t,
    mut block_w: ::core::ffi::c_int,
    mut block_h: ::core::ffi::c_int,
    mut src_x: ::core::ffi::c_int,
    mut src_y: ::core::ffi::c_int,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
) {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut start_y: ::core::ffi::c_int = 0;
    let mut start_x: ::core::ffi::c_int = 0;
    let mut end_y: ::core::ffi::c_int = 0;
    let mut end_x: ::core::ffi::c_int = 0;
    if w == 0 || h == 0 {
        return;
    }
    if src_y >= h {
        src = src.offset(-((src_y as ptrdiff_t * src_linesize) as isize));
        src = src.offset(((h as ptrdiff_t - 1 as ptrdiff_t) * src_linesize) as isize);
        src_y = h - 1 as ::core::ffi::c_int;
    } else if src_y <= -block_h {
        src = src.offset(-((src_y as ptrdiff_t * src_linesize) as isize));
        src = src
            .offset(((1 as ptrdiff_t - block_h as ptrdiff_t) * src_linesize) as isize);
        src_y = 1 as ::core::ffi::c_int - block_h;
    }
    if src_x >= w {
        src = src
            .offset(
                -(((1 as ::core::ffi::c_int + src_x - w) as usize)
                    .wrapping_mul(::core::mem::size_of::<uint16_t>() as usize) as isize),
            );
        src_x = w - 1 as ::core::ffi::c_int;
    } else if src_x <= -block_w {
        src = src
            .offset(
                ((1 as ::core::ffi::c_int - block_w - src_x) as usize)
                    .wrapping_mul(::core::mem::size_of::<uint16_t>() as usize) as isize,
            );
        src_x = 1 as ::core::ffi::c_int - block_w;
    }
    start_y = if 0 as ::core::ffi::c_int > -src_y {
        0 as ::core::ffi::c_int
    } else {
        -src_y
    };
    start_x = if 0 as ::core::ffi::c_int > -src_x {
        0 as ::core::ffi::c_int
    } else {
        -src_x
    };
    end_y = if block_h > h - src_y { h - src_y } else { block_h };
    end_x = if block_w > w - src_x { w - src_x } else { block_w };
    w = end_x - start_x;
    src = src
        .offset(
            (start_y as ptrdiff_t * src_linesize
                + start_x as ptrdiff_t * ::core::mem::size_of::<uint16_t>() as ptrdiff_t)
                as isize,
        );
    buf = buf
        .offset(
            (start_x as usize).wrapping_mul(::core::mem::size_of::<uint16_t>() as usize)
                as isize,
        );
    y = 0 as ::core::ffi::c_int;
    while y < start_y {
        memcpy(
            buf as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            (w as size_t).wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        );
        buf = buf.offset(buf_linesize as isize);
        y += 1;
    }
    while y < end_y {
        memcpy(
            buf as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            (w as size_t).wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        );
        src = src.offset(src_linesize as isize);
        buf = buf.offset(buf_linesize as isize);
        y += 1;
    }
    src = src.offset(-(src_linesize as isize));
    while y < block_h {
        memcpy(
            buf as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            (w as size_t).wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        );
        buf = buf.offset(buf_linesize as isize);
        y += 1;
    }
    buf = buf
        .offset(
            -((block_h as ptrdiff_t * buf_linesize
                + start_x as ptrdiff_t * ::core::mem::size_of::<uint16_t>() as ptrdiff_t)
                as isize),
        );
    loop {
        let fresh0 = block_h;
        block_h = block_h - 1;
        if !(fresh0 != 0) {
            break;
        }
        let mut bufp: *mut uint16_t = buf as *mut uint16_t;
        x = 0 as ::core::ffi::c_int;
        while x < start_x {
            *bufp.offset(x as isize) = *bufp.offset(start_x as isize);
            x += 1;
        }
        x = end_x;
        while x < block_w {
            *bufp.offset(x as isize) = *bufp
                .offset((end_x - 1 as ::core::ffi::c_int) as isize);
            x += 1;
        }
        buf = buf.offset(buf_linesize as isize);
    };
}
unsafe extern "C" fn emulated_edge_mc_8(
    mut buf: *mut uint8_t,
    mut src: *const uint8_t,
    mut buf_linesize: ptrdiff_t,
    mut src_linesize: ptrdiff_t,
    mut block_w: ::core::ffi::c_int,
    mut block_h: ::core::ffi::c_int,
    mut src_x: ::core::ffi::c_int,
    mut src_y: ::core::ffi::c_int,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
) {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut start_y: ::core::ffi::c_int = 0;
    let mut start_x: ::core::ffi::c_int = 0;
    let mut end_y: ::core::ffi::c_int = 0;
    let mut end_x: ::core::ffi::c_int = 0;
    if w == 0 || h == 0 {
        return;
    }
    if src_y >= h {
        src = src.offset(-((src_y as ptrdiff_t * src_linesize) as isize));
        src = src.offset(((h as ptrdiff_t - 1 as ptrdiff_t) * src_linesize) as isize);
        src_y = h - 1 as ::core::ffi::c_int;
    } else if src_y <= -block_h {
        src = src.offset(-((src_y as ptrdiff_t * src_linesize) as isize));
        src = src
            .offset(((1 as ptrdiff_t - block_h as ptrdiff_t) * src_linesize) as isize);
        src_y = 1 as ::core::ffi::c_int - block_h;
    }
    if src_x >= w {
        src = src
            .offset(
                -(((1 as ::core::ffi::c_int + src_x - w) as usize)
                    .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize),
            );
        src_x = w - 1 as ::core::ffi::c_int;
    } else if src_x <= -block_w {
        src = src
            .offset(
                ((1 as ::core::ffi::c_int - block_w - src_x) as usize)
                    .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as isize,
            );
        src_x = 1 as ::core::ffi::c_int - block_w;
    }
    start_y = if 0 as ::core::ffi::c_int > -src_y {
        0 as ::core::ffi::c_int
    } else {
        -src_y
    };
    start_x = if 0 as ::core::ffi::c_int > -src_x {
        0 as ::core::ffi::c_int
    } else {
        -src_x
    };
    end_y = if block_h > h - src_y { h - src_y } else { block_h };
    end_x = if block_w > w - src_x { w - src_x } else { block_w };
    w = end_x - start_x;
    src = src
        .offset(
            (start_y as ptrdiff_t * src_linesize
                + start_x as ptrdiff_t * ::core::mem::size_of::<uint8_t>() as ptrdiff_t)
                as isize,
        );
    buf = buf
        .offset(
            (start_x as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                as isize,
        );
    y = 0 as ::core::ffi::c_int;
    while y < start_y {
        memcpy(
            buf as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            (w as size_t).wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        );
        buf = buf.offset(buf_linesize as isize);
        y += 1;
    }
    while y < end_y {
        memcpy(
            buf as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            (w as size_t).wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        );
        src = src.offset(src_linesize as isize);
        buf = buf.offset(buf_linesize as isize);
        y += 1;
    }
    src = src.offset(-(src_linesize as isize));
    while y < block_h {
        memcpy(
            buf as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            (w as size_t).wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        );
        buf = buf.offset(buf_linesize as isize);
        y += 1;
    }
    buf = buf
        .offset(
            -((block_h as ptrdiff_t * buf_linesize
                + start_x as ptrdiff_t * ::core::mem::size_of::<uint8_t>() as ptrdiff_t)
                as isize),
        );
    loop {
        let fresh1 = block_h;
        block_h = block_h - 1;
        if !(fresh1 != 0) {
            break;
        }
        let mut bufp: *mut uint8_t = buf;
        x = 0 as ::core::ffi::c_int;
        while x < start_x {
            *bufp.offset(x as isize) = *bufp.offset(start_x as isize);
            x += 1;
        }
        x = end_x;
        while x < block_w {
            *bufp.offset(x as isize) = *bufp
                .offset((end_x - 1 as ::core::ffi::c_int) as isize);
            x += 1;
        }
        buf = buf.offset(buf_linesize as isize);
    };
}
unsafe extern "C" fn just_return(
    mut buf: *const uint8_t,
    mut stride: ptrdiff_t,
    mut h: ::core::ffi::c_int,
) {}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ff_videodsp_init(
    mut ctx: *mut VideoDSPContext,
    mut bpc: ::core::ffi::c_int,
) {
    (*ctx).prefetch = Some(
        just_return
            as unsafe extern "C" fn(*const uint8_t, ptrdiff_t, ::core::ffi::c_int) -> (),
    )
        as Option<
            unsafe extern "C" fn(*const uint8_t, ptrdiff_t, ::core::ffi::c_int) -> (),
        >;
    if bpc <= 8 as ::core::ffi::c_int {
        (*ctx).emulated_edge_mc = Some(
            emulated_edge_mc_8
                as unsafe extern "C" fn(
                    *mut uint8_t,
                    *const uint8_t,
                    ptrdiff_t,
                    ptrdiff_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut uint8_t,
                    *const uint8_t,
                    ptrdiff_t,
                    ptrdiff_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
            >;
    } else {
        (*ctx).emulated_edge_mc = Some(
            emulated_edge_mc_16
                as unsafe extern "C" fn(
                    *mut uint8_t,
                    *const uint8_t,
                    ptrdiff_t,
                    ptrdiff_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut uint8_t,
                    *const uint8_t,
                    ptrdiff_t,
                    ptrdiff_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
            >;
    }
    ff_videodsp_init_arm(ctx, bpc);
}
