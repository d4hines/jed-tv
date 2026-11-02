// Translated from FFmpeg 9.0.1 libavcodec/dynamic_hdr_vivid.c
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

pub type int32_t = ::core::ffi::c_int;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint32_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: ::core::ffi::c_int,
    pub den: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVHDRVivid3SplineParams {
    pub th_mode: ::core::ffi::c_int,
    pub th_enable_mb: AVRational,
    pub th_enable: AVRational,
    pub th_delta1: AVRational,
    pub th_delta2: AVRational,
    pub enable_strength: AVRational,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVHDRVividColorToneMappingParams {
    pub targeted_system_display_maximum_luminance: AVRational,
    pub base_enable_flag: ::core::ffi::c_int,
    pub base_param_m_p: AVRational,
    pub base_param_m_m: AVRational,
    pub base_param_m_a: AVRational,
    pub base_param_m_b: AVRational,
    pub base_param_m_n: AVRational,
    pub base_param_k1: ::core::ffi::c_int,
    pub base_param_k2: ::core::ffi::c_int,
    pub base_param_k3: ::core::ffi::c_int,
    pub base_param_Delta_enable_mode: ::core::ffi::c_int,
    pub base_param_Delta: AVRational,
    pub three_Spline_enable_flag: ::core::ffi::c_int,
    pub three_Spline_num: ::core::ffi::c_int,
    pub three_spline: [AVHDRVivid3SplineParams; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVHDRVividColorTransformParams {
    pub minimum_maxrgb: AVRational,
    pub average_maxrgb: AVRational,
    pub variance_maxrgb: AVRational,
    pub maximum_maxrgb: AVRational,
    pub tone_mapping_mode_flag: ::core::ffi::c_int,
    pub tone_mapping_param_num: ::core::ffi::c_int,
    pub tm_params: [AVHDRVividColorToneMappingParams; 2],
    pub color_saturation_mapping_flag: ::core::ffi::c_int,
    pub color_saturation_num: ::core::ffi::c_int,
    pub color_saturation_gain: [AVRational; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDynamicHDRVivid {
    pub system_start_code: uint8_t,
    pub num_windows: uint8_t,
    pub params: [AVHDRVividColorTransformParams; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GetBitContext {
    pub buffer: *const uint8_t,
    pub index: ::core::ffi::c_int,
    pub size_in_bits: ::core::ffi::c_int,
    pub size_in_bits_plus8: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union unaligned_32 {
    pub l: uint32_t,
}
static mut maxrgb_den: int32_t = 4095 as int32_t;
static mut color_saturation_gain_den: int32_t = 128 as int32_t;
static mut maximum_luminance_den: int32_t = 4095 as int32_t;
static mut base_param_m_p_den: int32_t = 16383 as int32_t;
static mut base_param_m_m_den: int32_t = 10 as int32_t;
static mut base_param_m_a_den: int32_t = 1023 as int32_t;
static mut base_param_m_b_den: int32_t = 1023 as int32_t;
static mut base_param_m_n_den: int32_t = 10 as int32_t;
static mut base_param_Delta_den: int32_t = 127 as int32_t;
#[no_mangle]
pub unsafe extern "C" fn ff_parse_itu_t_t35_to_dynamic_hdr_vivid(
    mut s: *mut AVDynamicHDRVivid,
    mut data: *const uint8_t,
    mut size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut gbc: GetBitContext = GetBitContext {
        buffer: ::core::ptr::null::<uint8_t>(),
        index: 0,
        size_in_bits: 0,
        size_in_bits_plus8: 0,
    };
    let mut gb: *mut GetBitContext = &raw mut gbc;
    let mut ret: ::core::ffi::c_int = 0;
    if s.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    ret = init_get_bits8(gb, data, size);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    if get_bits_left(gb) < 8 as ::core::ffi::c_int {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    (*s).system_start_code = get_bits(gb, 8 as ::core::ffi::c_int) as uint8_t;
    if (*s).system_start_code as ::core::ffi::c_int >= 0x1 as ::core::ffi::c_int
        && (*s).system_start_code as ::core::ffi::c_int <= 0x7 as ::core::ffi::c_int
    {
        (*s).num_windows = 1 as uint8_t;
        if get_bits_left(gb)
            < 12 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
                * (*s).num_windows as ::core::ffi::c_int
        {
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        let mut w: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while w < (*s).num_windows as ::core::ffi::c_int {
            let mut params: *mut AVHDRVividColorTransformParams = (&raw mut (*s).params
                as *mut AVHDRVividColorTransformParams)
                .offset(w as isize) as *mut AVHDRVividColorTransformParams;
            (*params).minimum_maxrgb = AVRational {
                num: get_bits(gb, 12 as ::core::ffi::c_int) as ::core::ffi::c_int,
                den: maxrgb_den as ::core::ffi::c_int,
            };
            (*params).average_maxrgb = AVRational {
                num: get_bits(gb, 12 as ::core::ffi::c_int) as ::core::ffi::c_int,
                den: maxrgb_den as ::core::ffi::c_int,
            };
            (*params).variance_maxrgb = AVRational {
                num: get_bits(gb, 12 as ::core::ffi::c_int) as ::core::ffi::c_int,
                den: maxrgb_den as ::core::ffi::c_int,
            };
            (*params).maximum_maxrgb = AVRational {
                num: get_bits(gb, 12 as ::core::ffi::c_int) as ::core::ffi::c_int,
                den: maxrgb_den as ::core::ffi::c_int,
            };
            w += 1;
        }
        if get_bits_left(gb)
            < 2 as ::core::ffi::c_int * (*s).num_windows as ::core::ffi::c_int
        {
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        let mut w_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while w_0 < (*s).num_windows as ::core::ffi::c_int {
            let mut params_0: *mut AVHDRVividColorTransformParams = (&raw mut (*s).params
                as *mut AVHDRVividColorTransformParams)
                .offset(w_0 as isize) as *mut AVHDRVividColorTransformParams;
            (*params_0).tone_mapping_mode_flag = get_bits(gb, 1 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            if (*params_0).tone_mapping_mode_flag != 0 {
                if get_bits_left(gb) < 1 as ::core::ffi::c_int {
                    return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                        | ('D' as i32) << 16 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint
                        | ('A' as i32 as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                }
                (*params_0).tone_mapping_param_num = get_bits(
                        gb,
                        1 as ::core::ffi::c_int,
                    )
                    .wrapping_add(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i < (*params_0).tone_mapping_param_num {
                    let mut tm_params: *mut AVHDRVividColorToneMappingParams = (&raw mut (*params_0)
                        .tm_params as *mut AVHDRVividColorToneMappingParams)
                        .offset(i as isize) as *mut AVHDRVividColorToneMappingParams;
                    if get_bits_left(gb) < 13 as ::core::ffi::c_int {
                        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                            | ('D' as i32) << 16 as ::core::ffi::c_int)
                            as ::core::ffi::c_uint
                            | ('A' as i32 as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                    }
                    (*tm_params).targeted_system_display_maximum_luminance = AVRational {
                        num: get_bits(gb, 12 as ::core::ffi::c_int)
                            as ::core::ffi::c_int,
                        den: maximum_luminance_den as ::core::ffi::c_int,
                    };
                    (*tm_params).base_enable_flag = get_bits(gb, 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                    if (*tm_params).base_enable_flag != 0 {
                        if get_bits_left(gb)
                            < 14 as ::core::ffi::c_int + 6 as ::core::ffi::c_int
                                + 10 as ::core::ffi::c_int + 10 as ::core::ffi::c_int
                                + 6 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                                + 10 as ::core::ffi::c_int
                        {
                            return -((('I' as i32
                                | ('N' as i32) << 8 as ::core::ffi::c_int
                                | ('D' as i32) << 16 as ::core::ffi::c_int)
                                as ::core::ffi::c_uint
                                | ('A' as i32 as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                        }
                        (*tm_params).base_param_m_p = AVRational {
                            num: get_bits(gb, 14 as ::core::ffi::c_int)
                                as ::core::ffi::c_int,
                            den: base_param_m_p_den as ::core::ffi::c_int,
                        };
                        (*tm_params).base_param_m_m = AVRational {
                            num: get_bits(gb, 6 as ::core::ffi::c_int)
                                as ::core::ffi::c_int,
                            den: base_param_m_m_den as ::core::ffi::c_int,
                        };
                        (*tm_params).base_param_m_a = AVRational {
                            num: get_bits(gb, 10 as ::core::ffi::c_int)
                                as ::core::ffi::c_int,
                            den: base_param_m_a_den as ::core::ffi::c_int,
                        };
                        (*tm_params).base_param_m_b = AVRational {
                            num: get_bits(gb, 10 as ::core::ffi::c_int)
                                as ::core::ffi::c_int,
                            den: base_param_m_b_den as ::core::ffi::c_int,
                        };
                        (*tm_params).base_param_m_n = AVRational {
                            num: get_bits(gb, 6 as ::core::ffi::c_int)
                                as ::core::ffi::c_int,
                            den: base_param_m_n_den as ::core::ffi::c_int,
                        };
                        (*tm_params).base_param_k1 = get_bits(
                            gb,
                            2 as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int;
                        (*tm_params).base_param_k2 = get_bits(
                            gb,
                            2 as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int;
                        (*tm_params).base_param_k3 = get_bits(
                            gb,
                            4 as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int;
                        (*tm_params).base_param_Delta_enable_mode = get_bits(
                            gb,
                            3 as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int;
                        (*tm_params).base_param_Delta = AVRational {
                            num: get_bits(gb, 7 as ::core::ffi::c_int)
                                as ::core::ffi::c_int,
                            den: base_param_Delta_den as ::core::ffi::c_int,
                        };
                    }
                    if get_bits_left(gb) < 1 as ::core::ffi::c_int {
                        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                            | ('D' as i32) << 16 as ::core::ffi::c_int)
                            as ::core::ffi::c_uint
                            | ('A' as i32 as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                    }
                    (*tm_params).three_Spline_enable_flag = get_bits(
                        gb,
                        1 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int;
                    if (*tm_params).three_Spline_enable_flag != 0 {
                        let mut three_spline: *mut AVHDRVivid3SplineParams = ::core::ptr::null_mut::<
                            AVHDRVivid3SplineParams,
                        >();
                        if get_bits_left(gb)
                            < 1 as ::core::ffi::c_int
                                + (*tm_params).three_Spline_num
                                    * (2 as ::core::ffi::c_int + 12 as ::core::ffi::c_int
                                        + 28 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        {
                            return -((('I' as i32
                                | ('N' as i32) << 8 as ::core::ffi::c_int
                                | ('D' as i32) << 16 as ::core::ffi::c_int)
                                as ::core::ffi::c_uint
                                | ('A' as i32 as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                        }
                        (*tm_params).three_Spline_num = get_bits(
                                gb,
                                1 as ::core::ffi::c_int,
                            )
                            .wrapping_add(1 as ::core::ffi::c_uint)
                            as ::core::ffi::c_int;
                        if (*tm_params).three_Spline_num as usize
                            > (::core::mem::size_of::<[AVHDRVivid3SplineParams; 2]>()
                                as usize)
                                .wrapping_div(
                                    ::core::mem::size_of::<AVHDRVivid3SplineParams>() as usize,
                                )
                        {
                            return -((('I' as i32
                                | ('N' as i32) << 8 as ::core::ffi::c_int
                                | ('D' as i32) << 16 as ::core::ffi::c_int)
                                as ::core::ffi::c_uint
                                | ('A' as i32 as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                        }
                        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while j < (*tm_params).three_Spline_num {
                            three_spline = (&raw mut (*tm_params).three_spline
                                as *mut AVHDRVivid3SplineParams)
                                .offset(j as isize) as *mut AVHDRVivid3SplineParams;
                            (*three_spline).th_mode = get_bits(
                                gb,
                                2 as ::core::ffi::c_int,
                            ) as ::core::ffi::c_int;
                            if (*three_spline).th_mode == 0 as ::core::ffi::c_int
                                || (*three_spline).th_mode == 2 as ::core::ffi::c_int
                            {
                                if get_bits_left(gb) < 8 as ::core::ffi::c_int {
                                    return -((('I' as i32
                                        | ('N' as i32) << 8 as ::core::ffi::c_int
                                        | ('D' as i32) << 16 as ::core::ffi::c_int)
                                        as ::core::ffi::c_uint
                                        | ('A' as i32 as ::core::ffi::c_uint)
                                            << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                                }
                                (*three_spline).th_enable_mb = AVRational {
                                    num: get_bits(gb, 8 as ::core::ffi::c_int)
                                        as ::core::ffi::c_int,
                                    den: 255 as ::core::ffi::c_int,
                                };
                            }
                            (*three_spline).th_enable = AVRational {
                                num: get_bits(gb, 12 as ::core::ffi::c_int)
                                    as ::core::ffi::c_int,
                                den: 4095 as ::core::ffi::c_int,
                            };
                            (*three_spline).th_delta1 = AVRational {
                                num: get_bits(gb, 10 as ::core::ffi::c_int)
                                    as ::core::ffi::c_int,
                                den: 1023 as ::core::ffi::c_int,
                            };
                            (*three_spline).th_delta2 = AVRational {
                                num: get_bits(gb, 10 as ::core::ffi::c_int)
                                    as ::core::ffi::c_int,
                                den: 1023 as ::core::ffi::c_int,
                            };
                            (*three_spline).enable_strength = AVRational {
                                num: get_bits(gb, 8 as ::core::ffi::c_int)
                                    as ::core::ffi::c_int,
                                den: 255 as ::core::ffi::c_int,
                            };
                            j += 1;
                        }
                    }
                    i += 1;
                }
            }
            (*params_0).color_saturation_mapping_flag = get_bits(
                gb,
                1 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int;
            if (*params_0).color_saturation_mapping_flag != 0 {
                if get_bits_left(gb)
                    < 3 as ::core::ffi::c_int
                        + (*params_0).color_saturation_num * 8 as ::core::ffi::c_int
                {
                    return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                        | ('D' as i32) << 16 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint
                        | ('A' as i32 as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                }
                (*params_0).color_saturation_num = get_bits(gb, 3 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
                let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_0 < (*params_0).color_saturation_num {
                    (*params_0).color_saturation_gain[i_0 as usize] = AVRational {
                        num: get_bits(gb, 8 as ::core::ffi::c_int) as ::core::ffi::c_int,
                        den: color_saturation_gain_den as ::core::ffi::c_int,
                    };
                    i_0 += 1;
                }
            }
            w_0 += 1;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn av_bswap32(mut x: uint32_t) -> uint32_t {
    return (x << 8 as ::core::ffi::c_int & 0xff00 as uint32_t
        | x >> 8 as ::core::ffi::c_int & 0xff as uint32_t) << 16 as ::core::ffi::c_int
        | ((x >> 16 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            & 0xff00 as uint32_t
            | x >> 16 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
                & 0xff as uint32_t);
}
#[inline]
unsafe extern "C" fn get_bits_count(mut s: *const GetBitContext) -> ::core::ffi::c_int {
    return (*s).index;
}
#[inline]
unsafe extern "C" fn get_bits(
    mut s: *mut GetBitContext,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    let mut tmp: ::core::ffi::c_uint = 0;
    let mut re_index: ::core::ffi::c_uint = (*s).index as ::core::ffi::c_uint;
    let mut re_cache: ::core::ffi::c_uint = 0;
    let mut re_size_plus8: ::core::ffi::c_uint = (*s).size_in_bits_plus8
        as ::core::ffi::c_uint;
    re_cache = (av_bswap32(
        (*((*s).buffer.offset((re_index >> 3 as ::core::ffi::c_int) as isize)
            as *const unaligned_32))
            .l,
    ) << (re_index & 7 as ::core::ffi::c_uint)
        >> 32 as ::core::ffi::c_int - 32 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    tmp = (re_cache >> 32 as ::core::ffi::c_int - n) as ::core::ffi::c_uint;
    re_index = if re_size_plus8 > re_index.wrapping_add(n as ::core::ffi::c_uint) {
        re_index.wrapping_add(n as ::core::ffi::c_uint)
    } else {
        re_size_plus8
    };
    (*s).index = re_index as ::core::ffi::c_int;
    return tmp;
}
#[inline]
unsafe extern "C" fn init_get_bits(
    mut s: *mut GetBitContext,
    mut buffer: *const uint8_t,
    mut bit_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if bit_size
        >= 2147483647 as ::core::ffi::c_int
            - (if 7 as ::core::ffi::c_int
                > 64 as ::core::ffi::c_int * 8 as ::core::ffi::c_int
            {
                7 as ::core::ffi::c_int
            } else {
                64 as ::core::ffi::c_int * 8 as ::core::ffi::c_int
            }) || bit_size < 0 as ::core::ffi::c_int || buffer.is_null()
    {
        bit_size = 0 as ::core::ffi::c_int;
        buffer = ::core::ptr::null::<uint8_t>();
        ret = -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    (*s).buffer = buffer;
    (*s).size_in_bits = bit_size;
    (*s).size_in_bits_plus8 = bit_size + 8 as ::core::ffi::c_int;
    (*s).index = 0 as ::core::ffi::c_int;
    return ret;
}
#[inline]
unsafe extern "C" fn init_get_bits8(
    mut s: *mut GetBitContext,
    mut buffer: *const uint8_t,
    mut byte_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if byte_size > 2147483647 as ::core::ffi::c_int / 8 as ::core::ffi::c_int
        || byte_size < 0 as ::core::ffi::c_int
    {
        byte_size = -(1 as ::core::ffi::c_int);
    }
    return init_get_bits(s, buffer, byte_size * 8 as ::core::ffi::c_int);
}
#[inline]
unsafe extern "C" fn get_bits_left(mut gb: *mut GetBitContext) -> ::core::ffi::c_int {
    return (*gb).size_in_bits - get_bits_count(gb);
}
