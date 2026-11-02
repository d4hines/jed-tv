//! Software drawing into the virtio-gpu framebuffer (B8G8R8A8 in memory,
//! i.e. 0xAARRGGBB as a little-endian u32: the same layout as fbdev XRGB8888).

#[derive(Clone, Copy)]
pub struct Frame {
    pub ptr: *mut u32,
    pub width: usize,
    pub height: usize,
    /// Only rows clip.0..clip.1 are touched (one band per core).
    pub clip: (usize, usize),
}

unsafe impl Send for Frame {}
unsafe impl Sync for Frame {}

pub const fn pixel(r: u32, g: u32, b: u32) -> u32 {
    0xff00_0000 | (r << 16) | (g << 8) | b
}

/// ytplay self-test colour bars (src/ytplay/src/selftest.rs).
pub const BARS: [u32; 8] = [
    pixel(255, 255, 255), pixel(255, 255, 0), pixel(0, 255, 255), pixel(0, 255, 0),
    pixel(255, 0, 255), pixel(255, 0, 0), pixel(0, 0, 255), pixel(0, 0, 0),
];
pub const WHITE: u32 = pixel(255, 255, 255);

impl Frame {
    pub fn band(&self, y0: usize, y1: usize) -> Frame {
        Frame { clip: (y0, y1), ..*self }
    }

    pub fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, c: u32) {
        let (x0, x1) = (x.min(self.width), (x + w).min(self.width));
        let (y0, y1) = (y.max(self.clip.0), (y + h).min(self.clip.1).min(self.height));
        for row in y0..y1 {
            let line = unsafe {
                core::slice::from_raw_parts_mut(self.ptr.add(row * self.width), self.width)
            };
            line[x0..x1].fill(c);
        }
    }

    /// The self-test frame: eight bars over the top 2/3, black below, and an
    /// 80x80 square at `sx` (`sy` rows into the black band).
    pub fn test_pattern(&mut self, sx: usize, sy: usize, square: u32) {
        let (w, h) = (self.width, self.height);
        for (i, &c) in BARS.iter().enumerate() {
            self.fill_rect(i * w / 8, 0, w / 8 + 1, h * 2 / 3, c);
        }
        self.fill_rect(0, h * 2 / 3, w, h - h * 2 / 3, BARS[7]);
        self.fill_rect(sx, h * 2 / 3 + sy, 80, 80, square);
    }
}
