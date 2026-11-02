//! memcpy and friends: word-at-a-time Rust. The crate is `#![no_builtins]`,
//! so LLVM does not turn these loops back into calls to themselves; nothing
//! in this file may use ptr::copy*/write_bytes (they lower to these calls).
//! Source reads may be unaligned (ARMv7 and AArch64 allow that on normal
//! memory); destination writes are word-aligned.

use core::mem::size_of;

type W = usize;
const WS: usize = size_of::<W>();
const ONES: W = W::MAX / 255; // 0x0101...
const HIGHS: W = ONES << 7; // 0x8080...

#[inline(always)]
unsafe fn ld(p: *const u8) -> W {
    (p as *const W).read_unaligned()
}

#[inline(always)]
unsafe fn copy_fwd(mut d: *mut u8, mut s: *const u8, mut n: usize) {
    if n >= 2 * WS {
        let mis = (d as usize).wrapping_neg() & (WS - 1);
        n -= mis;
        for _ in 0..mis {
            *d = *s;
            d = d.add(1);
            s = s.add(1);
        }
        while n >= 4 * WS {
            let (a, b, c, e) = (ld(s), ld(s.add(WS)), ld(s.add(2 * WS)), ld(s.add(3 * WS)));
            let dw = d as *mut W;
            *dw = a;
            *dw.add(1) = b;
            *dw.add(2) = c;
            *dw.add(3) = e;
            d = d.add(4 * WS);
            s = s.add(4 * WS);
            n -= 4 * WS;
        }
        while n >= WS {
            *(d as *mut W) = ld(s);
            d = d.add(WS);
            s = s.add(WS);
            n -= WS;
        }
    }
    while n > 0 {
        *d = *s;
        d = d.add(1);
        s = s.add(1);
        n -= 1;
    }
}

#[inline(always)]
unsafe fn copy_bwd(d: *mut u8, s: *const u8, mut n: usize) {
    let mut d = d.add(n);
    let mut s = s.add(n);
    if n >= 2 * WS {
        let mis = (d as usize) & (WS - 1);
        n -= mis;
        for _ in 0..mis {
            d = d.sub(1);
            s = s.sub(1);
            *d = *s;
        }
        while n >= 4 * WS {
            d = d.sub(4 * WS);
            s = s.sub(4 * WS);
            let (a, b, c, e) = (ld(s), ld(s.add(WS)), ld(s.add(2 * WS)), ld(s.add(3 * WS)));
            let dw = d as *mut W;
            *dw.add(3) = e;
            *dw.add(2) = c;
            *dw.add(1) = b;
            *dw = a;
            n -= 4 * WS;
        }
        while n >= WS {
            d = d.sub(WS);
            s = s.sub(WS);
            *(d as *mut W) = ld(s);
            n -= WS;
        }
    }
    while n > 0 {
        d = d.sub(1);
        s = s.sub(1);
        *d = *s;
        n -= 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(d: *mut u8, s: *const u8, n: usize) -> *mut u8 {
    copy_fwd(d, s, n);
    d
}

#[no_mangle]
pub unsafe extern "C" fn memmove(d: *mut u8, s: *const u8, n: usize) -> *mut u8 {
    if (d as usize).wrapping_sub(s as usize) >= n {
        copy_fwd(d, s, n); // no overlap that forward copying could clobber
    } else {
        copy_bwd(d, s, n);
    }
    d
}

#[no_mangle]
pub unsafe extern "C" fn memset(d: *mut u8, c: i32, mut n: usize) -> *mut u8 {
    let b = c as u8;
    let mut p = d;
    if n >= 2 * WS {
        let mis = (p as usize).wrapping_neg() & (WS - 1);
        n -= mis;
        for _ in 0..mis {
            *p = b;
            p = p.add(1);
        }
        let w = ONES * b as W;
        while n >= 4 * WS {
            let pw = p as *mut W;
            *pw = w;
            *pw.add(1) = w;
            *pw.add(2) = w;
            *pw.add(3) = w;
            p = p.add(4 * WS);
            n -= 4 * WS;
        }
        while n >= WS {
            *(p as *mut W) = w;
            p = p.add(WS);
            n -= WS;
        }
    }
    while n > 0 {
        *p = b;
        p = p.add(1);
        n -= 1;
    }
    d
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(a: *const u8, b: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i + WS <= n && ld(a.add(i)) == ld(b.add(i)) {
        i += WS;
    }
    while i < n {
        let (x, y) = (*a.add(i), *b.add(i));
        if x != y {
            return x as i32 - y as i32;
        }
        i += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn bcmp(a: *const u8, b: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i + WS <= n {
        if ld(a.add(i)) != ld(b.add(i)) {
            return 1;
        }
        i += WS;
    }
    while i < n {
        if *a.add(i) != *b.add(i) {
            return 1;
        }
        i += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn strlen(s: *const u8) -> usize {
    let mut p = s;
    // Bytes until aligned, then whole aligned words (never crosses a page).
    while (p as usize) & (WS - 1) != 0 {
        if *p == 0 {
            return p as usize - s as usize;
        }
        p = p.add(1);
    }
    loop {
        let w = *(p as *const W);
        if (w.wrapping_sub(ONES) & !w & HIGHS) != 0 {
            break;
        }
        p = p.add(WS);
    }
    while *p != 0 {
        p = p.add(1);
    }
    p as usize - s as usize
}

#[no_mangle]
pub unsafe extern "C" fn memchr(s: *const u8, c: i32, n: usize) -> *mut u8 {
    for i in 0..n {
        if *s.add(i) == c as u8 {
            return s.add(i) as *mut u8;
        }
    }
    core::ptr::null_mut()
}
