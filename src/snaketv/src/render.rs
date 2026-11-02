//! Drawing the torus on a rectangular screen.
//!
//! `Screen::new` computes once which cell each pixel belongs to: the MAP that
//! `tools/tables.py` computed on the host for the round panel, here built on
//! the TV at start-up for whatever size the framebuffer is. `draw` then paints
//! a whole frame from the grid (the App API says to redraw the complete frame,
//! so the four-window repaint of `main.S` is not needed) and the text.
//!
//! Sector 0 starts at 12 o'clock and sectors increase clockwise on the
//! screen; ring 0 is just outside the UI disc.

use crate::{Game, OUTSIDE_CELL, RINGS, SECTORS, UI_CELL};
use std::f32::consts::TAU;

/// Bit 15 of a map entry: this pixel lies on a cell boundary and is drawn
/// darker, so the lattice shows. Cell indices need 10 bits.
const EDGE: u16 = 1 << 15;
const CELL_MASK: u16 = 1023;

// The round panel had a 36 px UI disc and 5 px rings on 240 lines. On a 720
// line screen the same look is about this; both scale with the height.
const INNER_720: f32 = 120.0;
const RING_720: f32 = 14.0;

/// Colours (R, G, B) per GRID value: EMPTY, BODY, HEAD, APPLE, BLACK, UI_PLAY, UI_OVER, unused.
pub const CELL_RGB: [(u8, u8, u8); 8] = [
    (28, 32, 48),
    (70, 200, 90),
    (235, 255, 235),
    (235, 60, 60),
    (8, 10, 18),
    (72, 76, 92),
    (190, 40, 50),
    (8, 10, 18),
];
pub const BACKGROUND_RGB: (u8, u8, u8) = (8, 10, 18);
pub const TEXT_RGB: (u8, u8, u8) = (240, 240, 245);
pub const TEXT_DIM_RGB: (u8, u8, u8) = (150, 154, 170);

/// Pixels in the framebuffer's own format, built once from the RGB values.
pub struct Colors {
    pub background: u32,
    /// One pixel per GRID value.
    pub cells: [u32; 8],
    /// The same, darker, for pixels on a cell boundary.
    pub edges: [u32; 8],
    pub text: u32,
    pub text_dim: u32,
}

impl Colors {
    /// `pack` turns (R, G, B) into the screen's pixel format.
    pub fn new(pack: impl Fn(u8, u8, u8) -> u32) -> Colors {
        let darker = |c: u8| (c as u32 * 55 / 100) as u8;
        Colors {
            background: pack(BACKGROUND_RGB.0, BACKGROUND_RGB.1, BACKGROUND_RGB.2),
            cells: CELL_RGB.map(|(r, g, b)| pack(r, g, b)),
            edges: CELL_RGB.map(|(r, g, b)| pack(darker(r), darker(g), darker(b))),
            text: pack(TEXT_RGB.0, TEXT_RGB.1, TEXT_RGB.2),
            text_dim: pack(TEXT_DIM_RGB.0, TEXT_DIM_RGB.1, TEXT_DIM_RGB.2),
        }
    }
}

pub struct Screen {
    pub width: usize,
    pub height: usize,
    /// The rectangle the disc occupies: top-left corner and size. Only these
    /// pixels have a map entry; the rest is background.
    pub x0: usize,
    pub y0: usize,
    pub box_w: usize,
    pub box_h: usize,
    map: Vec<u16>,
    pub cx: f32,
    pub cy: f32,
    pub inner: f32,
    pub ring: f32,
    pub outer: f32,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Screen {
        let unit = height as f32 / 720.0;
        let inner = INNER_720 * unit;
        let ring = RING_720 * unit;
        let outer = inner + RINGS as f32 * ring;
        let (cx, cy) = (width as f32 / 2.0, height as f32 / 2.0);
        let x0 = (cx - outer).floor().max(0.0) as usize;
        let y0 = (cy - outer).floor().max(0.0) as usize;
        let x1 = ((cx + outer).ceil() as usize).min(width);
        let y1 = ((cy + outer).ceil() as usize).min(height);
        let mut s = Screen {
            width, height, x0, y0, box_w: x1 - x0, box_h: y1 - y0, map: Vec::new(), cx, cy, inner, ring, outer,
        };
        s.map = (y0..y1).flat_map(|y| (x0..x1).map(move |x| (x, y))).map(|(x, y)| s.entry(x, y)).collect();
        s
    }

    /// The map entry for a pixel: its cell, plus EDGE on a boundary.
    fn entry(&self, x: usize, y: usize) -> u16 {
        let dx = x as f32 + 0.5 - self.cx;
        let dy = y as f32 + 0.5 - self.cy;
        let r = (dx * dx + dy * dy).sqrt();
        if r < self.inner {
            return UI_CELL as u16;
        }
        if r >= self.outer {
            return OUTSIDE_CELL as u16;
        }
        let ring_pos = (r - self.inner) / self.ring;
        let ring = (ring_pos as usize).min(RINGS - 1);
        // 0 at 12 o'clock, increasing clockwise on the screen (y grows downward).
        let mut angle = dx.atan2(-dy);
        if angle < 0.0 {
            angle += TAU;
        }
        let sector_pos = angle / TAU * SECTORS as f32;
        let sector = (sector_pos as usize) % SECTORS;
        let cell = (ring * SECTORS + sector) as u16;
        // Within one pixel of the ring's inner edge or the sector's leading edge.
        let into_ring = (ring_pos - ring as f32) * self.ring;
        let into_sector = (sector_pos - sector as f32) * (TAU * r / SECTORS as f32);
        if into_ring < 1.0 || into_sector < 1.0 { cell | EDGE } else { cell }
    }

    /// The cell under a pixel and whether it is on a boundary; None outside
    /// the disc's rectangle.
    pub fn cell_at(&self, x: usize, y: usize) -> Option<(usize, bool)> {
        if x < self.x0 || y < self.y0 || x >= self.x0 + self.box_w || y >= self.y0 + self.box_h {
            return None;
        }
        let e = self.map[(y - self.y0) * self.box_w + (x - self.x0)];
        Some(((e & CELL_MASK) as usize, e & EDGE != 0))
    }

    /// Paint the disc from the grid into `px` (rows `stride` pixels apart).
    pub fn paint(&self, grid: &[u8], colors: &Colors, px: &mut [u32], stride: usize) {
        for row in 0..self.box_h {
            let map = &self.map[row * self.box_w..(row + 1) * self.box_w];
            let start = (self.y0 + row) * stride + self.x0;
            let out = &mut px[start..start + self.box_w];
            for (p, &e) in out.iter_mut().zip(map) {
                let what = (grid[(e & CELL_MASK) as usize] & 7) as usize;
                *p = if e & EDGE != 0 { colors.edges[what] } else { colors.cells[what] };
            }
        }
    }
}

/// 8x8 bitmap font at integer `scale`, clipped to the screen; returns the
/// width the string takes.
#[allow(clippy::too_many_arguments)]
pub fn text(px: &mut [u32], stride: usize, width: usize, height: usize, x: i32, y: i32, scale: i32, color: u32, s: &str) -> i32 {
    use font8x8::legacy::BASIC_LEGACY;
    for (i, ch) in s.chars().enumerate() {
        let glyph = BASIC_LEGACY.get(ch as usize).copied().unwrap_or([0; 8]);
        for (row, bits) in glyph.iter().enumerate() {
            for col in 0..8 {
                if bits & (1 << col) != 0 {
                    for dy in 0..scale {
                        for dx in 0..scale {
                            let (sx, sy) = (x + (i as i32 * 8 + col) * scale + dx, y + row as i32 * scale + dy);
                            if sx >= 0 && sy >= 0 && (sx as usize) < width && (sy as usize) < height {
                                px[sy as usize * stride + sx as usize] = color;
                            }
                        }
                    }
                }
            }
        }
    }
    s.chars().count() as i32 * 8 * scale
}

/// One whole frame: background, the disc, the text.
pub fn draw(screen: &Screen, game: &Game, colors: &Colors, px: &mut [u32], stride: usize) {
    let (w, h) = (screen.width, screen.height);
    for y in 0..h {
        px[y * stride..y * stride + w].fill(colors.background);
    }
    screen.paint(&game.grid, colors, px, stride);

    let unit = h as f32 / 720.0;
    let s = |n: f32| (n * unit).round().max(1.0) as i32;
    let margin = s(72.0);
    let mut t = |x: i32, y: i32, scale: i32, color: u32, string: &str| text(px, stride, w, h, x, y, scale, color, string);
    // The left column, clear of the disc; the status lines are kept short so
    // they never reach it.
    t(margin, s(30.0), s(3.0), colors.text, "Snake on a torus");
    t(margin, s(72.0), s(2.0), colors.text_dim, &format!("length {}", game.length()));
    t(margin, s(96.0), s(2.0), colors.text_dim, &format!("step {}", game.steps));
    t(margin, s(120.0), s(2.0), colors.text_dim, &format!("heading {}", game.heading.name()));
    t(margin, h as i32 - s(30.0), s(2.0), colors.text_dim, "Left/Right turn - OK plays again - Back exits");

    // Inside the UI disc: the length, big; after a crash, OVER and how to go on.
    let (cx, cy) = (screen.cx as i32, screen.cy as i32);
    let centred = |t: &mut dyn FnMut(i32, i32, i32, u32, &str) -> i32, y: i32, scale: i32, color: u32, string: &str| {
        let width = string.chars().count() as i32 * 8 * scale;
        t(cx - width / 2, y, scale, color, string);
    };
    let length = game.length().to_string();
    if game.over {
        centred(&mut t, cy - s(56.0), s(4.0), colors.text, "OVER");
        centred(&mut t, cy - s(4.0), s(4.0), colors.text, &length);
        centred(&mut t, cy + s(44.0), s(2.0), colors.text, "OK: again");
    } else {
        centred(&mut t, cy - s(24.0), s(6.0), colors.text, &length);
        centred(&mut t, cy + s(32.0), s(2.0), colors.text_dim, "cells");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CELLS, START_RING};

    fn screen() -> Screen {
        Screen::new(1280, 720)
    }

    #[test]
    fn the_disc_sits_in_the_middle_of_a_720p_frame() {
        let s = screen();
        assert_eq!((s.inner, s.ring, s.outer), (120.0, 14.0, 330.0));
        assert_eq!((s.x0, s.y0), (310, 30));
        assert_eq!((s.box_w, s.box_h), (660, 660));
    }

    #[test]
    fn the_centre_is_ui_and_the_corners_are_outside() {
        let s = screen();
        assert_eq!(s.cell_at(640, 360), Some((UI_CELL, false)));
        assert_eq!(s.cell_at(310, 30), Some((OUTSIDE_CELL, false)));
        assert_eq!(s.cell_at(969, 689), Some((OUTSIDE_CELL, false)));
        assert_eq!(s.cell_at(0, 0), None);
        assert_eq!(s.cell_at(1279, 719), None);
    }

    #[test]
    fn sector_0_is_at_12_oclock_and_sectors_go_clockwise() {
        let s = screen();
        let r = (s.inner + (START_RING as f32 + 0.5) * s.ring) as usize; // middle of ring 7
        let mid = |sector: usize| START_RING * SECTORS + sector;
        // Just clockwise of each quarter, so the pixel is inside the sector and not on its edge.
        assert_eq!(s.cell_at(640 + 3, 360 - r).map(|c| c.0), Some(mid(0)));
        assert_eq!(s.cell_at(640 + r, 360 + 3).map(|c| c.0), Some(mid(15)));
        assert_eq!(s.cell_at(640 - 3, 360 + r).map(|c| c.0), Some(mid(30)));
        assert_eq!(s.cell_at(640 - r, 360 - 3).map(|c| c.0), Some(mid(45)));
    }

    #[test]
    fn ring_0_is_just_outside_the_disc_and_rings_grow_outward() {
        let s = screen();
        let at = |r: f32| s.cell_at(640 + 3, 360 - r as usize).map(|c| c.0 / SECTORS);
        assert_eq!(at(s.inner + 2.0), Some(0));
        assert_eq!(at(s.inner + 14.0 * 7.0 + 7.0), Some(7));
        assert_eq!(at(s.outer - 2.0), Some(RINGS - 1));
    }

    #[test]
    fn every_cell_has_pixels_and_every_pixel_has_a_cell() {
        let s = screen();
        let mut count = vec![0usize; CELLS + 2];
        let mut edges = 0;
        for y in s.y0..s.y0 + s.box_h {
            for x in s.x0..s.x0 + s.box_w {
                let (cell, edge) = s.cell_at(x, y).unwrap();
                assert!(cell < CELLS + 2);
                count[cell] += 1;
                edges += edge as usize;
            }
        }
        // The smallest cell (inner ring) is about 12.6 x 14 px; most of it is not boundary.
        assert!(count[..CELLS].iter().all(|&n| n >= 100), "min {}", count[..CELLS].iter().min().unwrap());
        assert!(count[UI_CELL] > 40_000 && count[OUTSIDE_CELL] > 50_000);
        assert!(edges > 0);
    }

    /// Every pixel of the torus band shows its cell's colour: the text (title,
    /// status, footer, the numbers in the disc) never covers the game.
    fn band_is_intact(g: &Game) {
        let s = screen();
        let colors = Colors::new(|r, g, b| (r as u32) << 16 | (g as u32) << 8 | b as u32);
        let mut px = vec![0u32; 1280 * 720];
        draw(&s, g, &colors, &mut px, 1280);
        let mut seen = [0usize; 8];
        for y in s.y0..s.y0 + s.box_h {
            for x in s.x0..s.x0 + s.box_w {
                let (cell, edge) = s.cell_at(x, y).unwrap();
                if cell >= CELLS {
                    continue;
                }
                let what = g.grid[cell] as usize;
                let expect = if edge { colors.edges[what] } else { colors.cells[what] };
                assert_eq!(px[y * 1280 + x], expect, "pixel ({x}, {y}) cell {cell}");
                seen[what] += 1;
            }
        }
        assert!(seen[crate::BODY as usize] > 0 && seen[crate::HEAD as usize] > 0);
        assert_eq!(px[0], colors.background);
    }

    #[test]
    fn a_frame_shows_the_snake_in_its_colours() {
        band_is_intact(&Game::new());
    }

    #[test]
    fn text_stays_off_the_torus_in_a_long_game_and_after_a_crash() {
        let mut g = Game::new();
        for _ in 0..999 {
            g.step();
        }
        band_is_intact(&g); // "step 999", "heading clockwise": the longest status lines
        // A U-shaped snake that turns in and runs into itself: the OVER text.
        let r = START_RING * SECTORS;
        let cells = [r + 10, r + 11, r + 12, r + 13, r + 13 + SECTORS, r + 13 + 2 * SECTORS, r + 12 + 2 * SECTORS];
        let mut g = Game::from_cells(&cells, crate::Heading::Anticlockwise);
        g.turn(crate::Turn::Left);
        g.step();
        assert_eq!(g.step(), Some(crate::Outcome::Crashed));
        band_is_intact(&g);
    }
}
