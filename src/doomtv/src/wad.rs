//! Just enough of the WAD format to draw text with Doom's own font (STCFN*
//! patches, PLAYPAL colours) when the engine is not running, e.g. on the
//! screen shown after Doom quits. Reads only the directory and a few lumps.

use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

pub struct Wad {
    file: File,
    dir: Vec<([u8; 8], u32, u32)>, // name, file offset, size
}

impl Wad {
    pub fn open(path: &str) -> io::Result<Wad> {
        let mut file = File::open(path)?;
        let mut hdr = [0u8; 12];
        file.read_exact(&mut hdr)?;
        if &hdr[1..4] != b"WAD" {
            return Err(io::Error::other("not a WAD file"));
        }
        let n = u32::from_le_bytes(hdr[4..8].try_into().unwrap()) as usize;
        let off = u32::from_le_bytes(hdr[8..12].try_into().unwrap()) as u64;
        let mut raw = vec![0u8; n.min(1 << 20) * 16];
        file.seek(SeekFrom::Start(off))?;
        file.read_exact(&mut raw)?;
        let dir = raw.chunks_exact(16).map(|e| {
            let name: [u8; 8] = e[8..16].try_into().unwrap();
            (name, u32::from_le_bytes(e[0..4].try_into().unwrap()),
             u32::from_le_bytes(e[4..8].try_into().unwrap()))
        }).collect();
        Ok(Wad { file, dir })
    }

    /// Last lump with this name (later lumps override earlier ones).
    pub fn lump(&mut self, name: &str) -> Option<Vec<u8>> {
        let mut key = [0u8; 8];
        key[..name.len().min(8)].copy_from_slice(&name.as_bytes()[..name.len().min(8)]);
        let &(_, off, size) = self.dir.iter().rev().find(|e| e.0 == key)?;
        let mut buf = vec![0u8; size as usize];
        self.file.seek(SeekFrom::Start(off as u64)).ok()?;
        self.file.read_exact(&mut buf).ok()?;
        Some(buf)
    }
}

/// A decoded patch: palette indices, None = transparent.
pub struct Patch {
    pub width: usize,
    pub height: usize,
    pub px: Vec<Option<u8>>,
}

impl Patch {
    /// Doom picture format: header, column offsets, then posts per column.
    pub fn decode(d: &[u8]) -> Option<Patch> {
        let u16at = |i: usize| d.get(i..i + 2).map(|b| u16::from_le_bytes([b[0], b[1]]) as usize);
        let (width, height) = (u16at(0)?, u16at(2)?);
        if width == 0 || height == 0 || width > 1024 || height > 1024 {
            return None;
        }
        let mut px = vec![None; width * height];
        for x in 0..width {
            let o = 8 + 4 * x;
            let mut p = u32::from_le_bytes(d.get(o..o + 4)?.try_into().ok()?) as usize;
            loop {
                let top = *d.get(p)? as usize;
                if top == 0xff {
                    break;
                }
                let len = *d.get(p + 1)? as usize;
                let data = d.get(p + 3..p + 3 + len)?;
                for (i, &c) in data.iter().enumerate() {
                    if top + i < height {
                        px[(top + i) * width + x] = Some(c);
                    }
                }
                p += len + 4;
            }
        }
        Some(Patch { width, height, px })
    }
}

/// Doom's HUD/menu font: STCFN033..STCFN095 cover ASCII '!'..'_' (upper case).
pub struct Font {
    pub glyphs: Vec<Option<Patch>>, // index = ch - '!'
    pub palette: [[u8; 3]; 256],
}

impl Font {
    pub fn load(wad: &mut Wad) -> Option<Font> {
        let pal = wad.lump("PLAYPAL")?;
        let mut palette = [[0u8; 3]; 256];
        for (i, c) in pal.chunks_exact(3).take(256).enumerate() {
            palette[i] = [c[0], c[1], c[2]];
        }
        let glyphs = (33..=95).map(|c| wad.lump(&format!("STCFN{c:03}"))
            .and_then(|d| Patch::decode(&d))).collect();
        Some(Font { glyphs, palette })
    }

    fn glyph(&self, ch: char) -> Option<&Patch> {
        let c = ch.to_ascii_uppercase() as usize;
        if (33..=95).contains(&c) { self.glyphs[c - 33].as_ref() } else { None }
    }

    /// Width in font pixels (unscaled); spaces are 4 wide as in Doom.
    pub fn width(&self, s: &str) -> usize {
        s.chars().map(|c| self.glyph(c).map_or(4, |g| g.width)).sum()
    }

    /// Draw `s` with its top-left at (x, y), each font pixel a scale x scale block.
    /// `put(x, y, [r, g, b])` writes one screen pixel.
    pub fn draw(&self, s: &str, x: usize, y: usize, scale: usize,
                put: &mut dyn FnMut(usize, usize, [u8; 3])) {
        let mut cx = x;
        for ch in s.chars() {
            let Some(g) = self.glyph(ch) else { cx += 4 * scale; continue };
            for gy in 0..g.height {
                for gx in 0..g.width {
                    if let Some(i) = g.px[gy * g.width + gx] {
                        let rgb = self.palette[i as usize];
                        for dy in 0..scale {
                            for dx in 0..scale {
                                put(cx + gx * scale + dx, y + gy * scale + dy, rgb);
                            }
                        }
                    }
                }
            }
            cx += g.width * scale;
        }
    }
}
