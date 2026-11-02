//! YUV 4:2:0 (BT.709, limited range) to packed RGB, scaled to fit with
//! letterboxing. Software only: there is no GPU in the platform API.
//!
//! The per-row kernels use plain integer arithmetic (no lookup tables) so the
//! compiler can vectorize them (NEON on the TV), and rows are split across
//! threads.

use crate::decode::Frame;
use crate::fb::BackBuffer;

// BT.709 limited-range coefficients in 10-bit fixed point.
const KY: i32 = 1192; // 1.164383
const KRV: i32 = 1836; // 1.792741
const KGU: i32 = 218; // 0.213249
const KGV: i32 = 546; // 0.532909
const KBU: i32 = 2163; // 2.112402

#[derive(Clone, Copy)]
struct Layout {
    rs: u32,
    gs: u32,
    bs: u32,
    alpha: u32,
}

#[inline(always)]
fn pack(l: Layout, y: i32, rr: i32, gg: i32, bb: i32) -> u32 {
    let r = ((y + rr) >> 10).clamp(0, 255) as u32;
    let g = ((y + gg) >> 10).clamp(0, 255) as u32;
    let b = ((y + bb) >> 10).clamp(0, 255) as u32;
    (r << l.rs) | (g << l.gs) | (b << l.bs) | l.alpha
}

/// One output row at the source width (no horizontal scaling): 2 pixels per chroma sample.
fn row_unscaled(l: Layout, y: &[u8], u: &[u8], v: &[u8], out: &mut [u32]) {
    let pairs = out.len() / 2;
    let (y, u, v) = (&y[..pairs * 2], &u[..pairs], &v[..pairs]);
    for (i, o) in out[..pairs * 2].chunks_exact_mut(2).enumerate() {
        let cu = u[i] as i32 - 128;
        let cv = v[i] as i32 - 128;
        let (rr, gg, bb) = (KRV * cv, -KGU * cu - KGV * cv, KBU * cu);
        let y0 = (y[2 * i] as i32 - 16) * KY + 512;
        let y1 = (y[2 * i + 1] as i32 - 16) * KY + 512;
        o[0] = pack(l, y0, rr, gg, bb);
        o[1] = pack(l, y1, rr, gg, bb);
    }
    if out.len() % 2 == 1 {
        let x = out.len() - 1;
        let (cu, cv) = (u.get(x / 2).copied().unwrap_or(128) as i32 - 128,
                        v.get(x / 2).copied().unwrap_or(128) as i32 - 128);
        let yy = (y.get(x).copied().unwrap_or(16) as i32 - 16) * KY + 512;
        out[x] = pack(l, yy, KRV * cv, -KGU * cu - KGV * cv, KBU * cu);
    }
}

/// One output row with nearest-neighbour horizontal scaling through `xmap`.
fn row_scaled(l: Layout, y: &[u8], u: &[u8], v: &[u8], xmap: &[u32], out: &mut [u32]) {
    for (o, &sx) in out.iter_mut().zip(xmap) {
        let sx = sx as usize;
        let cu = u[sx / 2] as i32 - 128;
        let cv = v[sx / 2] as i32 - 128;
        let yy = (y[sx] as i32 - 16) * KY + 512;
        *o = pack(l, yy, KRV * cv, -KGU * cu - KGV * cv, KBU * cu);
    }
}

pub struct Converter {
    layout: Layout,
    pool: crate::pool::Pool,
    key: (usize, usize, usize, usize),
    xmap: Vec<u32>,
    ymap: Vec<u32>,
    dst: (usize, usize, usize, usize), // x, y, w, h of the picture on screen
}

impl Converter {
    /// `shifts`: bit offsets of R, G, B in the 32-bit pixel; `alpha`: bits to OR in.
    pub fn new(shifts: (u32, u32, u32), alpha: u32) -> Self {
        let cpus = std::thread::available_parallelism().map_or(1, |n| n.get()).min(4);
        Self::with_threads(shifts, alpha, cpus)
    }

    pub fn with_threads(shifts: (u32, u32, u32), alpha: u32, threads: usize) -> Self {
        Converter {
            layout: Layout { rs: shifts.0, gs: shifts.1, bs: shifts.2, alpha },
            pool: crate::pool::Pool::new(threads),
            key: (0, 0, 0, 0), xmap: Vec::new(), ymap: Vec::new(), dst: (0, 0, 0, 0),
        }
    }

    fn fit(&mut self, fw: usize, fh: usize, sw: usize, sh: usize) {
        if self.key == (fw, fh, sw, sh) {
            return;
        }
        self.key = (fw, fh, sw, sh);
        // Largest size that fits the screen with the frame's aspect ratio.
        let (w, h) = if fw * sh >= fh * sw { (sw, (fh * sw / fw).max(1)) } else { ((fw * sh / fh).max(1), sh) };
        self.dst = ((sw - w) / 2, (sh - h) / 2, w, h);
        self.xmap = (0..w).map(|x| (x * fw / w) as u32).collect();
        self.ymap = (0..h).map(|y| (y * fh / h) as u32).collect();
    }

    /// Draw `f` into `bb`, clearing the letterbox bars to black.
    pub fn draw(&mut self, f: &Frame, bb: &mut BackBuffer) {
        self.fit(f.width, f.height, bb.width, bb.height);
        let (dx, dy, w, h) = self.dst;
        let l = self.layout;
        let black = l.alpha;
        bb.fill_rect(0, 0, bb.width, dy, black);
        bb.fill_rect(0, dy + h, bb.width, bb.height - dy - h, black);
        bb.fill_rect(0, dy, dx, h, black);
        bb.fill_rect(dx + w, dy, bb.width - dx - w, h, black);

        let unscaled = w == f.width;
        let stride = bb.stride;
        let (xmap, ymap) = (&self.xmap, &self.ymap);
        let rows = &mut bb.px[dy * stride..(dy + h) * stride];
        let band = h.div_ceil(self.pool.threads()).max(16);
        let work = |first: usize, chunk: &mut [u32]| {
            for (k, line) in chunk.chunks_mut(stride).enumerate() {
                let sy = ymap[first + k] as usize;
                let yr = &f.y[sy * f.y_stride..];
                let ur = &f.u[(sy / 2) * f.c_stride..];
                let vr = &f.v[(sy / 2) * f.c_stride..];
                let out = &mut line[dx..dx + w];
                if unscaled {
                    row_unscaled(l, yr, ur, vr, out);
                } else {
                    row_scaled(l, yr, ur, vr, xmap, out);
                }
            }
        };
        if self.pool.threads() <= 1 || h <= band {
            work(0, rows);
            return;
        }
        // Bands are disjoint row ranges of `rows`, so each call gets its own slice.
        let base = rows.as_mut_ptr() as usize;
        let bands = h.div_ceil(band);
        self.pool.run(bands, &|b| {
            let first = b * band;
            let n = band.min(h - first);
            let chunk = unsafe {
                std::slice::from_raw_parts_mut((base as *mut u32).add(first * stride), n * stride)
            };
            work(first, chunk);
        });
    }
}
