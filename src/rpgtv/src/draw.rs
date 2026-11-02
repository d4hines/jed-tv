//! Draw one game screen into a 1280x720 canvas (0x00RRGGBB), in the home
//! screen's palette. Text uses the vendored DejaVu fonts.

use crate::game::{Bar, Mood, Screen};
use ab_glyph::{Font, FontRef, GlyphId, ScaleFont};
use std::collections::HashMap;

pub const W: usize = 1280;
pub const H: usize = 720;
/// Scene Pictures are converted on the build host to raw RGB24 of this size.
pub const PIC_W: usize = 720;
pub const PIC_H: usize = 240;

type Rgb = (u8, u8, u8);
const BG_TOP: Rgb = (22, 26, 44);
const BG_BOTTOM: Rgb = (6, 6, 12);
const BAND: Rgb = (14, 16, 28);
const PANEL: Rgb = (24, 28, 46);
const ACCENT: Rgb = (230, 40, 50);
const WHITE: Rgb = (244, 244, 248);
const TEXT: Rgb = (222, 225, 235);
const GREY: Rgb = (150, 154, 170);
const SELECT: Rgb = (61, 111, 209);
const ROW: Rgb = (36, 40, 62);
const GOLD: Rgb = (240, 196, 90);
const GREEN: Rgb = (70, 180, 100);

const LEFT: i32 = 56;
const COL_W: i32 = 740;
const SIDE_X: i32 = 836;
const SIDE_W: i32 = W as i32 - SIDE_X - 40;
const TOP: i32 = 96;
const FOOT: i32 = H as i32 - 44;
const OPT_H: i32 = 46;

#[derive(Clone, Copy)]
pub enum Face {
    Sans = 0,
    Bold = 1,
}

pub struct Canvas {
    pub px: Vec<u32>,
}

fn pack(c: Rgb) -> u32 {
    (c.0 as u32) << 16 | (c.1 as u32) << 8 | c.2 as u32
}

impl Canvas {
    fn new() -> Self {
        let mut px = vec![0; W * H];
        for y in 0..H {
            let f = |a: u8, b: u8| (a as i32 + (b as i32 - a as i32) * y as i32 / H as i32) as u8;
            px[y * W..(y + 1) * W].fill(pack((f(BG_TOP.0, BG_BOTTOM.0), f(BG_TOP.1, BG_BOTTOM.1), f(BG_TOP.2, BG_BOTTOM.2))));
        }
        Canvas { px }
    }

    fn fill(&mut self, x: i32, y: i32, w: i32, h: i32, c: Rgb) {
        let (x0, x1) = (x.clamp(0, W as i32) as usize, (x + w).clamp(0, W as i32) as usize);
        let (y0, y1) = (y.clamp(0, H as i32) as usize, (y + h).clamp(0, H as i32) as usize);
        if x0 < x1 {
            for row in y0..y1 {
                self.px[row * W + x0..row * W + x1].fill(pack(c));
            }
        }
    }

    fn blend(&mut self, x: i32, y: i32, c: Rgb, a: u32) {
        if a == 0 || !(0..W as i32).contains(&x) || !(0..H as i32).contains(&y) {
            return;
        }
        let p = &mut self.px[y as usize * W + x as usize];
        let mix = |shift: u32, v: u8| ((*p >> shift & 255) * (255 - a) + v as u32 * a + 127) / 255;
        *p = mix(16, c.0) << 16 | mix(8, c.1) << 8 | mix(0, c.2);
    }

    fn picture(&mut self, x: i32, y: i32, rgb: &[u8]) {
        for py in 0..PIC_H {
            let row = &rgb[py * PIC_W * 3..(py + 1) * PIC_W * 3];
            for px in 0..PIC_W {
                let (cx, cy) = (x + px as i32, y + py as i32);
                if (0..W as i32).contains(&cx) && (0..H as i32).contains(&cy) {
                    self.px[cy as usize * W + cx as usize] = pack((row[px * 3], row[px * 3 + 1], row[px * 3 + 2]));
                }
            }
        }
    }

    fn hp_bar(&mut self, x: i32, y: i32, w: i32, b: &Bar) {
        self.fill(x, y, w, 8, ROW);
        let frac = if b.max > 0 { b.hp.clamp(0, b.max) as f32 / b.max as f32 } else { 0.0 };
        let colour = if frac > 0.5 { GREEN } else if frac > 0.2 { GOLD } else { ACCENT };
        self.fill(x, y, (w as f32 * frac) as i32, 8, colour);
    }
}

struct Raster {
    left: i32,
    top: i32,
    w: usize,
    cov: Vec<u8>,
}

pub struct Painter {
    fonts: [FontRef<'static>; 2],
    cache: HashMap<(u8, u32, GlyphId), Raster>,
}

impl Default for Painter {
    fn default() -> Self {
        Self::new()
    }
}

impl Painter {
    pub fn new() -> Self {
        let load = |d: &'static [u8]| FontRef::try_from_slice(d).expect("embedded font");
        Painter {
            fonts: [load(include_bytes!("../fonts/dejavu-2.37/DejaVuSans.ttf")),
                    load(include_bytes!("../fonts/dejavu-2.37/DejaVuSans-Bold.ttf"))],
            cache: HashMap::new(),
        }
    }

    fn width(&self, face: Face, px: f32, text: &str) -> f32 {
        let f = self.fonts[face as usize].as_scaled(px);
        let mut w = 0.0;
        let mut prev = None;
        for ch in text.chars() {
            let id = f.glyph_id(ch);
            if let Some(p) = prev {
                w += f.kern(p, id);
            }
            w += f.h_advance(id);
            prev = Some(id);
        }
        w
    }

    /// Greedy word wrap; '\n' starts a new paragraph.
    fn wrap(&self, face: Face, px: f32, text: &str, width: f32) -> Vec<String> {
        let mut lines = vec![];
        for para in text.split('\n') {
            let mut line = String::new();
            for word in para.split_whitespace() {
                let candidate = if line.is_empty() { word.to_owned() } else { format!("{line} {word}") };
                if !line.is_empty() && self.width(face, px, &candidate) > width {
                    lines.push(std::mem::replace(&mut line, word.to_owned()));
                } else {
                    line = candidate;
                }
            }
            lines.push(line);
        }
        lines
    }

    /// Draw one line with its top at `y`; returns its width.
    fn line(&mut self, cv: &mut Canvas, face: Face, px: f32, x: i32, y: i32, c: Rgb, text: &str) -> f32 {
        let font = self.fonts[face as usize].clone();
        let f = font.as_scaled(px);
        let base = y as f32 + f.ascent();
        let mut pen = x as f32;
        let mut prev = None;
        for ch in text.chars() {
            let id = f.glyph_id(ch);
            if let Some(p) = prev {
                pen += f.kern(p, id);
            }
            let key = (face as u8, (px * 4.0) as u32, id);
            let r = self.cache.entry(key).or_insert_with(|| {
                match font.outline_glyph(id.with_scale_and_position(px, ab_glyph::point(0.0, 0.0))) {
                    None => Raster { left: 0, top: 0, w: 0, cov: vec![] },
                    Some(og) => {
                        let b = og.px_bounds();
                        let w = b.width() as usize;
                        let mut cov = vec![0u8; w * b.height() as usize];
                        og.draw(|gx, gy, a| {
                            if let Some(v) = cov.get_mut(gy as usize * w + gx as usize) {
                                *v = (a.clamp(0.0, 1.0) * 255.0) as u8;
                            }
                        });
                        Raster { left: b.min.x as i32, top: b.min.y as i32, w, cov }
                    }
                }
            });
            let (ox, oy) = (pen.round() as i32 + r.left, base.round() as i32 + r.top);
            if r.w > 0 {
                for (i, &a) in r.cov.iter().enumerate() {
                    cv.blend(ox + (i % r.w) as i32, oy + (i / r.w) as i32, c, a as u32);
                }
            }
            pen += f.h_advance(id);
            prev = Some(id);
        }
        pen - x as f32
    }

    /// Wrapped text in a box; shrinks from `max_px` to `min_px` until it fits, and cuts
    /// the last lines at the smallest size. Returns the bottom of the text.
    #[allow(clippy::too_many_arguments)]
    fn paragraph(&mut self, cv: &mut Canvas, face: Face, max_px: f32, min_px: f32, x: i32, y: i32, w: i32, h: i32, c: Rgb, text: &str) -> i32 {
        let mut px = max_px;
        let mut lines = self.wrap(face, px, text, w as f32);
        while (lines.len() as f32 * px * 1.3) as i32 > h && px > min_px {
            px -= 1.0;
            lines = self.wrap(face, px, text, w as f32);
        }
        let step = (px * 1.3) as i32;
        let fits = (h / step).max(0) as usize;
        if lines.len() > fits {
            lines.truncate(fits);
            if let Some(last) = lines.last_mut() {
                last.push_str(" ...");
            }
        }
        let mut yy = y;
        for l in &lines {
            self.line(cv, face, px, x, yy, c, l);
            yy += step;
        }
        yy
    }

    /// The whole screen. `selected` indexes `sc.options`; `status` is shown in the footer.
    pub fn render(&mut self, sc: &Screen, selected: usize, picture: Option<&[u8]>, status: &str) -> Canvas {
        let mut cv = Canvas::new();
        // Header: app name, Adventure, and the Character's HP.
        cv.fill(0, 0, W as i32, 80, BAND);
        cv.fill(0, 0, 10, 80, ACCENT);
        let w = self.line(&mut cv, Face::Bold, 30.0, LEFT - 26, 22, WHITE, "Infinite RPG");
        self.line(&mut cv, Face::Sans, 22.0, LEFT - 26 + w as i32 + 24, 30, GREY, &sc.adventure);
        if !sc.hero.name.is_empty() && sc.phase != "title" {
            let hp = format!("{}  HP {} / {}", sc.hero.name, sc.hero.hp, sc.hero.max);
            let hw = self.width(Face::Bold, 22.0, &hp) as i32;
            self.line(&mut cv, Face::Bold, 22.0, W as i32 - 40 - hw.max(220), 18, WHITE, &hp);
            cv.hp_bar(W as i32 - 40 - hw.max(220), 52, hw.max(220), &sc.hero);
        }

        // Left column: Picture, heading, story, options.
        let mut y = TOP;
        if let Some(rgb) = picture.filter(|p| p.len() == PIC_W * PIC_H * 3) {
            cv.picture(LEFT + (COL_W - PIC_W as i32) / 2, y, rgb);
            y += PIC_H as i32 + 14;
        }
        let heading = match sc.mood { Mood::Dead => ACCENT, Mood::Ended => GOLD, Mood::Normal => WHITE };
        y = self.paragraph(&mut cv, Face::Bold, 30.0, 22.0, LEFT, y, COL_W, 80, heading, &sc.heading) + 6;
        let opts = sc.options.len().min(6) as i32;
        let help = sc.options.get(selected).map(|o| o.help.as_str()).unwrap_or("");
        let help_h = if help.is_empty() { 0 } else { 30 };
        let opts_top = FOOT - 12 - opts * OPT_H - help_h;
        self.paragraph(&mut cv, Face::Sans, 24.0, 15.0, LEFT, y, COL_W, opts_top - 10 - y, TEXT, &sc.body);
        let first = selected.saturating_sub(5);
        for (i, o) in sc.options.iter().enumerate().skip(first).take(6) {
            let oy = opts_top + (i - first) as i32 * OPT_H;
            let on = i == selected;
            cv.fill(LEFT - 8, oy, COL_W + 16, OPT_H - 6, if on { SELECT } else { ROW });
            let label = format!("{}  {}", i + 1, o.label);
            let fit = self.wrap(Face::Bold, 21.0, &label, (COL_W - 16) as f32);
            let text = if fit.len() > 1 { format!("{} ...", fit[0]) } else { label };
            self.line(&mut cv, Face::Bold, 21.0, LEFT + 4, oy + 8, if on { WHITE } else { TEXT }, &text);
        }
        if !help.is_empty() {
            self.line(&mut cv, Face::Sans, 18.0, LEFT, FOOT - 12 - help_h + 4, GREY, help);
        }

        // Side panel: the Fight, then what happened (newest last).
        cv.fill(SIDE_X - 20, TOP - 8, SIDE_W + 40, FOOT - TOP, PANEL);
        let mut sy = TOP + 4;
        if !sc.foes.is_empty() {
            for b in std::iter::once(&sc.hero).chain(&sc.foes) {
                let t = format!("{}{}  {} / {} HP", b.name, if b.alive { "" } else { " (down)" }, b.hp, b.max);
                self.line(&mut cv, Face::Bold, 19.0, SIDE_X, sy, if b.alive { WHITE } else { GREY }, &t);
                cv.hp_bar(SIDE_X, sy + 26, SIDE_W, b);
                sy += 44;
            }
            sy += 8;
        }
        self.line(&mut cv, Face::Bold, 20.0, SIDE_X, sy, GOLD, &sc.side_title);
        sy += 32;
        let px = 16.0;
        let step = (px * 1.3) as i32;
        let room = ((FOOT - 12 - sy) / step).max(0) as usize;
        let mut lines: Vec<String> = vec![];
        for l in sc.side.iter().rev() {
            let mut wrapped = self.wrap(Face::Sans, px, l, SIDE_W as f32);
            if lines.len() + wrapped.len() > room {
                break;
            }
            wrapped.append(&mut lines);
            lines = wrapped;
        }
        for l in &lines {
            self.line(&mut cv, Face::Sans, px, SIDE_X, sy, TEXT, l);
            sy += step;
        }

        // Footer.
        cv.fill(0, FOOT, W as i32, 44, BAND);
        self.line(&mut cv, Face::Sans, 17.0, LEFT - 26, FOOT + 12, GREY, "Up/Down choose - OK confirms - Back exits   (keyboard: 1-9 pick)");
        let right = sc.error.as_deref().map(|e| format!("Refused: {e}")).unwrap_or_else(|| status.to_owned());
        if !right.is_empty() {
            let rw = self.width(Face::Sans, 17.0, &right) as i32;
            self.line(&mut cv, Face::Sans, 17.0, W as i32 - 40 - rw, FOOT + 12, if sc.error.is_some() { ACCENT } else { GREY }, &right);
        }
        cv
    }
}
