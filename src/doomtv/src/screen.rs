//! Doom's 320x200 XRGB frame -> the 1280x720 framebuffer, at 4:3 (960x720,
//! centered, nearest neighbour). Rows that repeat a source row are copied, and
//! source rows unchanged since this page was last drawn are skipped: writes to
//! the framebuffer are the expensive part (virtio-gpu's fbdev tracks dirty
//! pages by faulting, which costs ~90 ms per full 960x720 frame under TCG).

use platform::fb::{BackBuffer, Framebuffer};

/// Destination rectangle (x, y, w, h) for a 4:3 picture on a w x h screen.
pub fn fit_4_3(w: usize, h: usize) -> (usize, usize, usize, usize) {
    let (dw, dh) = if w * 3 >= h * 4 { (h * 4 / 3, h) } else { (w, w * 3 / 4) };
    ((w - dw) / 2, (h - dh) / 2, dw, dh)
}

pub struct Scaler {
    xmap: Vec<u32>,
    key: (usize, usize, usize),
    /// Pages whose letterbox borders still need clearing.
    pub clear_pages: u32,
    /// Per page: the source frame last scaled into it (empty = unknown).
    drawn: Vec<Vec<u32>>,
    /// Back page index, counting presents (DG_DrawFrame presents every blit).
    page: usize,
    /// Framebuffer channel layout is Doom's (R<<16 | G<<8 | B); copy pixels as is.
    native: bool,
    alpha: u32,
}

impl Scaler {
    pub fn new(fb: &Framebuffer) -> Scaler {
        let v = &fb.var;
        let native = (v.red.offset, v.green.offset, v.blue.offset) == (16, 8, 0)
            && (v.red.length, v.green.length, v.blue.length) == (8, 8, 8);
        let alpha = if v.transp.length > 0 {
            ((1u32 << v.transp.length.min(31)) - 1) << v.transp.offset
        } else {
            0
        };
        Scaler { xmap: Vec::new(), key: (0, 0, 0), clear_pages: fb.pages, native, alpha,
                 drawn: vec![Vec::new(); fb.pages as usize], page: 0 }
    }

    /// Scale `src` (sw x sh, XRGB8888) into the back buffer.
    pub fn blit(&mut self, fb: &mut Framebuffer, src: &[u32], sw: usize, sh: usize) {
        let fmt = PixelFormat::of(fb);
        let clear = self.clear_pages > 0;
        self.clear_pages = self.clear_pages.saturating_sub(1);
        let page = self.page % self.drawn.len();
        self.page += 1;
        let mut drawn = std::mem::take(&mut self.drawn[page]);
        let mut bb = fb.back_buffer();
        if clear {
            bb.px.fill(self.alpha);
            drawn.clear();
        }
        let (x0, y0, dw, dh) = fit_4_3(bb.width, bb.height);
        if self.key != (sw, dw, x0) {
            self.xmap = (0..dw).map(|dx| (dx * sw / dw) as u32).collect();
            self.key = (sw, dw, x0);
        }
        let a = self.alpha;
        let old = (drawn.len() == src.len()).then_some(&drawn[..]);
        if self.native {
            scale_rows(&mut bb, src, old, sw, sh, x0, y0, dw, dh, &self.xmap, |p| p | a);
        } else {
            scale_rows(&mut bb, src, old, sw, sh, x0, y0, dw, dh, &self.xmap,
                       |p| fmt.pack((p >> 16) as u8, (p >> 8) as u8, p as u8));
        }
        drawn.clear();
        drawn.extend_from_slice(src);
        self.drawn[page] = drawn;
    }
}

#[allow(clippy::too_many_arguments)]
/// `old`: the source frame already on this page, if known; its unchanged
/// rows are not rewritten.
fn scale_rows(bb: &mut BackBuffer, src: &[u32], old: Option<&[u32]>, sw: usize, sh: usize,
              x0: usize, y0: usize, dw: usize, dh: usize, xmap: &[u32],
              conv: impl Fn(u32) -> u32) {
    let stride = bb.stride;
    let mut prev: Option<(usize, usize)> = None; // (source row, destination offset)
    for dy in 0..dh {
        let sy = dy * sh / dh;
        let at = (y0 + dy) * stride + x0;
        if old.is_some_and(|o| o[sy * sw..sy * sw + sw] == src[sy * sw..sy * sw + sw]) {
            continue;
        }
        match prev {
            Some((psy, pat)) if psy == sy => bb.px.copy_within(pat..pat + dw, at),
            _ => {
                let s = &src[sy * sw..sy * sw + sw];
                for (d, &sx) in bb.px[at..at + dw].iter_mut().zip(xmap) {
                    *d = conv(s[sx as usize]);
                }
                prev = Some((sy, at));
            }
        }
    }
}

/// The framebuffer's RGB layout, copied out so it can be used while the
/// back buffer is borrowed.
#[derive(Clone, Copy)]
pub struct PixelFormat {
    r: (u32, u32),
    g: (u32, u32),
    b: (u32, u32),
    alpha: u32,
}

impl PixelFormat {
    pub fn of(fb: &Framebuffer) -> PixelFormat {
        let v = &fb.var;
        let alpha = if v.transp.length > 0 {
            ((1u32 << v.transp.length.min(31)) - 1) << v.transp.offset
        } else {
            0
        };
        PixelFormat { r: (v.red.offset, v.red.length), g: (v.green.offset, v.green.length),
                      b: (v.blue.offset, v.blue.length), alpha }
    }

    #[inline]
    pub fn pack(&self, r: u8, g: u8, b: u8) -> u32 {
        let put = |c: u8, (off, len): (u32, u32)| ((c as u32) >> (8 - len.min(8))) << off;
        put(r, self.r) | put(g, self.g) | put(b, self.b) | self.alpha
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fits_720p() {
        assert_eq!(fit_4_3(1280, 720), (160, 0, 960, 720));
        assert_eq!(fit_4_3(800, 800), (0, 100, 800, 600));
    }

    #[test]
    fn scales_rows() {
        // 2x2 source -> 4x3 destination at (1, 0) in a 6-wide buffer.
        let src = [1u32, 2, 3, 4];
        let mut px = vec![0u32; 6 * 3];
        let mut bb = BackBuffer { px: &mut px, stride: 6, width: 6, height: 3 };
        let xmap: Vec<u32> = (0..4).map(|dx| dx * 2 / 4).collect();
        scale_rows(&mut bb, &src, None, 2, 2, 1, 0, 4, 3, &xmap, |p| p);
        assert_eq!(px, [0, 1, 1, 2, 2, 0,
                        0, 1, 1, 2, 2, 0,
                        0, 3, 3, 4, 4, 0]);
        // Row 0 unchanged since this page was drawn: only row 1 is rewritten.
        let src2 = [1u32, 2, 5, 6];
        let mut bb = BackBuffer { px: &mut px, stride: 6, width: 6, height: 3 };
        bb.px[1] = 9; // would be repaired only if row 0 were redrawn
        scale_rows(&mut bb, &src2, Some(&src), 2, 2, 1, 0, 4, 3, &xmap, |p| p);
        assert_eq!(px, [0, 9, 1, 2, 2, 0,
                        0, 1, 1, 2, 2, 0,
                        0, 5, 5, 6, 6, 0]);
    }
}
