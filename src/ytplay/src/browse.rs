//! Search and browse: an on-screen keyboard driven by the remote, and a row of
//! result tiles with thumbnails. OK on a result plays it; Back returns here.
//!
//! Layout and colours follow tvinit's home screen. Thumbnails are YouTube's
//! fixed-URL 320x180 JPEGs, decoded with zune-jpeg (pure Rust).

use crate::{fb, sys, yt};
use platform::remote::{self, Action};
use std::io;
use std::time::Duration;

const TILE_W: usize = 256;
const TILE_H: usize = 144;
const ART_W: usize = 288; // thumbnails are scaled once to the focused size
const ART_H: usize = 162;
const GAP: usize = 32;
const LEFT: usize = 72;

/// Keyboard: 4 rows x 10 cells. Row 3 ends with SPACE (two cells), DEL, GO.
const ROWS: [&str; 4] = ["abcdefghij", "klmnopqrst", "uvwxyz0123", "456789"];

#[derive(Clone, Copy, PartialEq)]
enum Key {
    Char(char),
    Space,
    Del,
    Go,
}

fn key_at(r: usize, c: usize) -> Key {
    if r == 3 && c >= 6 {
        return match c {
            6 | 7 => Key::Space,
            8 => Key::Del,
            _ => Key::Go,
        };
    }
    Key::Char(ROWS[r].as_bytes()[c] as char)
}

#[derive(PartialEq)]
enum Focus {
    Keys,
    Results,
}

struct Hit {
    item: yt::SearchItem,
    art: Option<Vec<u8>>, // RGB24, ART_W x ART_H
}

pub struct Browse {
    server: String,
    query: String,
    kr: usize,
    kc: usize,
    focus: Focus,
    hits: Vec<Hit>,
    sel: usize,
    status: String,
    /// The last search's summary, restored after a chosen result has played.
    summary: String,
}

fn log(msg: impl AsRef<str>) {
    crate::log(msg)
}

/// Fetch and decode one thumbnail, scaled to ART_W x ART_H.
fn thumbnail(url: &str) -> Option<Vec<u8>> {
    let r = crate::http::request("GET", url, &[], &[]).ok()?;
    if r.status != 200 {
        return None;
    }
    let mut dec = zune_jpeg::JpegDecoder::new(&r.body);
    let rgb = dec.decode().ok()?;
    let (w, h) = dec.dimensions()?;
    let (w, h) = (w as usize, h as usize);
    if rgb.len() < w * h * 3 {
        return None;
    }
    let mut out = vec![0u8; ART_W * ART_H * 3];
    for y in 0..ART_H {
        for x in 0..ART_W {
            let s = ((y * h / ART_H) * w + x * w / ART_W) * 3;
            out[(y * ART_W + x) * 3..][..3].copy_from_slice(&rgb[s..s + 3]);
        }
    }
    Some(out)
}

impl Browse {
    pub fn new(server: &str, query: &str) -> Self {
        Browse { server: server.to_owned(), query: query.to_owned(), kr: 0, kc: 0,
                 focus: Focus::Keys, hits: Vec::new(), sel: 0,
                 status: "Type with the remote; number keys work too".into(),
                 summary: String::new() }
    }

    /// Run the search for the current query and fetch thumbnails in parallel.
    fn search(&mut self, screen: &mut fb::Framebuffer) {
        let q = self.query.trim().to_owned();
        if q.is_empty() {
            return;
        }
        self.status = format!("Searching for \"{q}\"...");
        self.draw(screen);
        let t0 = sys::monotonic();
        match yt::search(&self.server, &q) {
            Ok(items) => {
                let items: Vec<_> = items.into_iter().take(16).collect();
                let arts: Vec<Option<Vec<u8>>> = std::thread::scope(|s| {
                    let handles: Vec<_> = items.iter()
                        .map(|it| { let url = it.thumb_url(); s.spawn(move || thumbnail(&url)) })
                        .collect();
                    handles.into_iter().map(|h| h.join().ok().flatten()).collect()
                });
                self.hits = items.into_iter().zip(arts).map(|(item, art)| Hit { item, art }).collect();
                self.sel = 0;
                let ms = (sys::monotonic() - t0).as_millis();
                self.status = format!("{} results for \"{q}\" ({ms} ms)", self.hits.len());
                self.summary = self.status.clone();
                log(&self.status);
                if !self.hits.is_empty() {
                    self.focus = Focus::Results;
                }
            }
            Err(e) => {
                // Results always belong to the query on screen.
                self.hits.clear();
                self.sel = 0;
                self.status = format!("Search failed: {e}");
                log(&self.status);
            }
        }
    }

    fn draw(&self, screen: &mut fb::Framebuffer) {
        let var = screen.var;
        let pack = move |r: u8, g: u8, b: u8| var.pixel(r,g,b);
        let (white, grey, red, key_bg, ink) = (pack(240, 240, 245), pack(150, 154, 170),
                                               pack(230, 40, 50), pack(34, 36, 50), pack(14, 16, 28));
        let bg_top = (22u8, 26u8, 44u8);
        let mut bb = screen.back_buffer();
        let (w, h) = (bb.width, bb.height);
        for y in 0..h {
            let f = |a: u8, b: u8| (a as isize + (b as isize - a as isize) * y as isize / h as isize) as u8;
            bb.fill_rect(0, y, w, 1, pack(f(bg_top.0, 6), f(bg_top.1, 6), f(bg_top.2, 12)));
        }
        // Header.
        bb.fill_rect(0, 0, 10, 86, red);
        text(&mut bb, LEFT - 30, 28, 4, white, "Search YouTube");
        // Query box: the newest characters that fit, leaving room for the cursor.
        let box_w = w - 2 * LEFT;
        bb.fill_rect(LEFT, 96, box_w, 56, key_bg);
        let fit = (box_w - 2 * 18) / 24 - 1;
        let shown: String = self.query.chars().rev().take(fit).collect::<Vec<_>>().into_iter().rev().collect();
        let cursor = if self.focus == Focus::Keys { "_" } else { "" };
        text(&mut bb, LEFT + 18, 112, 3, white, &format!("{shown}{cursor}"));
        // Keyboard.
        let (cw, ch, gap, ky) = (64usize, 52usize, 6usize, 176usize);
        for r in 0..4 {
            for c in 0..10 {
                let k = key_at(r, c);
                if c == 7 && r == 3 {
                    continue; // second half of SPACE
                }
                let wide = k == Key::Space;
                let x = LEFT + c * (cw + gap);
                let y = ky + r * (ch + gap);
                let kw = if wide { 2 * cw + gap } else { cw };
                let focused = self.focus == Focus::Keys && self.kr == r
                    && (self.kc == c || (wide && self.kc == 7));
                bb.fill_rect(x, y, kw, ch, if focused { white } else { key_bg });
                let fg = if focused { ink } else { white };
                let label = match k {
                    Key::Char(c) => c.to_string(),
                    Key::Space => "space".into(),
                    Key::Del => "del".into(),
                    Key::Go => "GO".into(),
                };
                let scale = if label.len() > 1 { 2 } else { 3 };
                let tw = label.len() * 8 * scale;
                text(&mut bb, x + (kw - tw.min(kw)) / 2, y + (ch - 8 * scale) / 2, scale, fg, &label);
            }
        }
        // Help / status, right of the keyboard.
        let sx = LEFT + 10 * (cw + gap) + 30;
        text(&mut bb, sx, ky + 4, 2, grey, "OK types, GO searches");
        text(&mut bb, sx, ky + 34, 2, grey, "Down: results  Back: exit");
        for (i, line) in wrap(&self.status, (w - sx - 20) / 16).iter().take(5).enumerate() {
            text(&mut bb, sx, ky + 94 + i * 24, 2, white, line);
        }
        // Results row.
        let ry = 430;
        text(&mut bb, LEFT, ry, 3, if self.focus == Focus::Results { white } else { grey }, "Results");
        let ty = ry + 44;
        let first = self.sel.saturating_sub(3);
        for (i, hit) in self.hits.iter().enumerate().skip(first) {
            let x = LEFT + (i - first) * (TILE_W + GAP);
            if x + TILE_W > w {
                break;
            }
            let focused = self.focus == Focus::Results && i == self.sel;
            let (tw, th, tx, tyy) = if focused {
                (ART_W, ART_H, x - (ART_W - TILE_W) / 2, ty - (ART_H - TILE_H) / 2)
            } else {
                (TILE_W, TILE_H, x, ty)
            };
            match &hit.art {
                Some(a) => blit(&mut bb, var, tx, tyy, tw, th, a),
                None => bb.fill_rect(tx, tyy, tw, th, key_bg),
            }
            if focused {
                frame(&mut bb, tx, tyy, tw, th, 5, white);
            }
        }
        if let Some(hit) = self.hits.get(self.sel) {
            let dy = ty + ART_H + 4;
            let title: String = hit.item.title.chars().take((w - 2 * LEFT) / 16).collect();
            text(&mut bb, LEFT, dy, 2, white, &title);
            let meta = format!("{}  -  {}", hit.item.channel, hit.item.length);
            let meta: String = meta.chars().take((w - 2 * LEFT) / 16).collect();
            text(&mut bb, LEFT, dy + 26, 2, grey, &meta);
        }
        drop(bb);
        let _ = screen.present();
    }

    /// Report how the chosen result played, for the next `interact`.
    pub fn played(&mut self, result: &io::Result<()>) {
        self.status = match result {
            Ok(()) => self.summary.clone(),
            Err(e) => format!("Could not play: {e}"),
        };
    }

    /// Run the screen until a result is chosen (Some) or the user leaves (None).
    pub fn interact(&mut self) -> io::Result<Option<yt::SearchItem>> {
        let mut screen = fb::Framebuffer::open("/dev/fb0")?;
        let mut devs = crate::input::open_all();
        if !self.query.is_empty() && self.hits.is_empty() {
            self.search(&mut screen);
        }
        self.draw(&mut screen);
        self.draw(&mut screen);
        loop {
            let mut dirty = false;
            for (a, repeat) in remote::poll(&mut devs) {
                dirty = true;
                match self.focus {
                    Focus::Keys => match a {
                        Action::Up => self.kr = self.kr.saturating_sub(1),
                        Action::Down if self.kr < 3 => self.kr += 1,
                        Action::Down if !self.hits.is_empty() => self.focus = Focus::Results,
                        Action::Left => self.kc = self.kc.saturating_sub(1),
                        Action::Right => self.kc = (self.kc + 1).min(9),
                        Action::Digit(d) if !repeat => self.query.push((b'0' + d) as char),
                        Action::Ok if !repeat => match key_at(self.kr, self.kc) {
                            Key::Char(c) => self.query.push(c),
                            Key::Space => self.query.push(' '),
                            Key::Del => { self.query.pop(); }
                            Key::Go => self.search(&mut screen),
                        },
                        Action::Back | Action::Exit if !repeat => return Ok(None),
                        _ => dirty = false,
                    },
                    Focus::Results => match a {
                        Action::Left => self.sel = self.sel.saturating_sub(1),
                        Action::Right => self.sel = (self.sel + 1).min(self.hits.len().saturating_sub(1)),
                        Action::Up | Action::Back if !repeat => self.focus = Focus::Keys,
                        Action::Ok | Action::PlayPause if !repeat => {
                            let Some(item) = self.hits.get(self.sel).map(|h| h.item.clone()) else { continue };
                            self.status = format!("Opening \"{}\"...", item.title);
                            self.draw(&mut screen);
                            return Ok(Some(item));
                        }
                        Action::Exit if !repeat => return Ok(None),
                        _ => dirty = false,
                    },
                }
            }
            if dirty {
                self.draw(&mut screen);
            }
            std::thread::sleep(Duration::from_millis(16));
        }
    }
}

/// Greedy word wrap to `width` characters; longer words are split.
fn wrap(s: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line = String::new();
    for word in s.split(' ') {
        let mut word: Vec<char> = word.chars().collect();
        while !word.is_empty() {
            let used = line.chars().count();
            let room = if used == 0 { width } else { width.saturating_sub(used + 1) };
            if word.len() <= room {
                if used > 0 { line.push(' '); }
                line.extend(word.drain(..));
            } else if used > 0 {
                lines.push(std::mem::take(&mut line));
            } else {
                lines.push(word.drain(..width.max(1)).collect());
            }
        }
    }
    if !line.is_empty() { lines.push(line); }
    lines
}

pub(crate) fn text(bb: &mut fb::BackBuffer, x: usize, y: usize, scale: usize, color: u32, s: &str) {
    use font8x8::legacy::BASIC_LEGACY;
    for (i, ch) in s.chars().enumerate() {
        let glyph = BASIC_LEGACY.get(ch as usize).copied().unwrap_or([0; 8]);
        for (row, bits) in glyph.iter().enumerate() {
            for col in 0..8 {
                if bits & (1 << col) != 0 {
                    bb.fill_rect(x + (i * 8 + col) * scale, y + row * scale, scale, scale, color);
                }
            }
        }
    }
}

fn frame(bb: &mut fb::BackBuffer, x: usize, y: usize, w: usize, h: usize, t: usize, c: u32) {
    bb.fill_rect(x.saturating_sub(t), y.saturating_sub(t), w + 2 * t, t, c);
    bb.fill_rect(x.saturating_sub(t), y + h, w + 2 * t, t, c);
    bb.fill_rect(x.saturating_sub(t), y, t, h, c);
    bb.fill_rect(x + w, y, t, h, c);
}

/// Nearest-neighbour blit of ART_W x ART_H RGB24 into a w x h rectangle.
fn blit(bb: &mut fb::BackBuffer, v: fb::VarInfo, x: usize, y: usize, w: usize, h: usize, art: &[u8]) {
    for dy in 0..h {
        let sy = y + dy;
        if sy >= bb.height {
            break;
        }
        let row = (dy * ART_H / h) * ART_W * 3;
        for dx in 0..w {
            let sx = x + dx;
            if sx >= bb.width {
                break;
            }
            let p = row + (dx * ART_W / w) * 3;
            let c = v.pixel(art[p],art[p+1],art[p+2]);
            bb.px[sy * bb.stride + sx] = c;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::wrap;

    #[test]
    fn status_wraps_without_losing_text() {
        assert_eq!(wrap("Search failed: search HTTP 500", 28), ["Search failed: search HTTP", "500"]);
        assert_eq!(wrap("0 results for \"none1\" (88 ms)", 28), ["0 results for \"none1\" (88", "ms)"]);
        assert_eq!(wrap("abcdefghij", 4), ["abcd", "efgh", "ij"]);
        assert_eq!(wrap("short", 28), ["short"]);
        assert!(wrap("", 28).is_empty());
        let long = "Type with the remote; number keys work too";
        assert_eq!(wrap(long, 28).join(" "), long);
    }
}
