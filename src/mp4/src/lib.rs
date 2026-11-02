//! Minimal fragmented-MP4 demuxer for YouTube DASH files (H.264 video, AAC audio).
//!
//! A YouTube MP4 format is `ftyp`, `moov` (the "init" range), `sidx` (the
//! "index" range), then `moof`+`mdat` fragments. `initRange`/`indexRange` in
//! the /player reply give the byte ranges, so a player needs:
//!   1. [`parse_init`] on the init range (codec config, timescale, defaults),
//!   2. [`parse_sidx`] on the index range (where each fragment is, and when),
//!   3. [`parse_fragments`] on each fragment's bytes (the samples).
//! All parsing is bounds-checked; malformed input yields [`Error`], not a panic.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error(pub String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mp4: {}", self.0)
    }
}
impl std::error::Error for Error {}
pub type Result<T> = std::result::Result<T, Error>;

fn err<T>(msg: impl Into<String>) -> Result<T> {
    Err(Error(msg.into()))
}

/// Big-endian reader over a byte slice with bounds checks.
#[derive(Clone)]
struct Rd<'a> {
    b: &'a [u8],
    p: usize,
}

impl<'a> Rd<'a> {
    fn new(b: &'a [u8]) -> Self {
        Rd { b, p: 0 }
    }
    fn take(&mut self, n: usize) -> Result<&'a [u8]> {
        let end = self.p.checked_add(n).filter(|&e| e <= self.b.len())
            .ok_or_else(|| Error(format!("truncated: need {n} bytes at {}", self.p)))?;
        let s = &self.b[self.p..end];
        self.p = end;
        Ok(s)
    }
    fn skip(&mut self, n: usize) -> Result<()> {
        self.take(n).map(|_| ())
    }
    fn u8(&mut self) -> Result<u8> {
        Ok(self.take(1)?[0])
    }
    fn u16(&mut self) -> Result<u16> {
        Ok(u16::from_be_bytes(self.take(2)?.try_into().unwrap()))
    }
    fn u24(&mut self) -> Result<u32> {
        let b = self.take(3)?;
        Ok(((b[0] as u32) << 16) | ((b[1] as u32) << 8) | b[2] as u32)
    }
    fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_be_bytes(self.take(4)?.try_into().unwrap()))
    }
    fn u64(&mut self) -> Result<u64> {
        Ok(u64::from_be_bytes(self.take(8)?.try_into().unwrap()))
    }
    fn left(&self) -> usize {
        self.b.len() - self.p
    }
    /// version + 24-bit flags of a FullBox.
    fn full(&mut self) -> Result<(u8, u32)> {
        Ok((self.u8()?, self.u24()?))
    }
}

/// One box: its type, payload, and where the box started in the parent buffer.
struct Bx<'a> {
    ty: [u8; 4],
    body: &'a [u8],
    start: usize,
    end: usize,
}

/// Iterate the boxes laid out back to back in `b`.
fn boxes(b: &[u8]) -> impl Iterator<Item = Result<Bx<'_>>> {
    let mut p = 0usize;
    std::iter::from_fn(move || {
        if p >= b.len() {
            return None;
        }
        let start = p;
        let r = (|| {
            let mut rd = Rd::new(&b[p..]);
            let size32 = rd.u32()?;
            let ty: [u8; 4] = rd.take(4)?.try_into().unwrap();
            let (hdr, size) = match size32 {
                0 => (8, (b.len() - p) as u64),
                1 => (16, rd.u64()?),
                n => (8, n as u64),
            };
            if size < hdr as u64 || size > (b.len() - p) as u64 {
                return err(format!("box {} size {size} out of bounds", String::from_utf8_lossy(&ty)));
            }
            let end = p + size as usize;
            Ok(Bx { ty, body: &b[p + hdr..end], start, end })
        })();
        match r {
            Ok(bx) => {
                p = bx.end;
                Some(Ok(bx))
            }
            Err(e) => {
                p = b.len();
                Some(Err(e))
            }
        }
    })
}

fn find<'a>(b: &'a [u8], ty: &[u8; 4]) -> Result<Option<Bx<'a>>> {
    for bx in boxes(b) {
        let bx = bx?;
        if &bx.ty == ty {
            return Ok(Some(bx));
        }
    }
    Ok(None)
}

fn need<'a>(b: &'a [u8], ty: &[u8; 4]) -> Result<Bx<'a>> {
    find(b, ty)?.ok_or_else(|| Error(format!("missing box {}", String::from_utf8_lossy(ty))))
}

#[derive(Debug, Clone)]
pub enum Codec {
    /// H.264. `avcc` is the raw avcC record (decoder "extradata").
    Avc { width: u16, height: u16, nal_length_size: u8, profile: u8, level: u8,
          sps: Vec<Vec<u8>>, pps: Vec<Vec<u8>>, avcc: Vec<u8> },
    /// AAC. `asc` is the AudioSpecificConfig.
    Aac { channels: u16, sample_rate: u32, object_type: u8, asc: Vec<u8> },
    Other([u8; 4]),
}

#[derive(Debug, Clone, Default)]
pub struct Defaults {
    pub duration: u32,
    pub size: u32,
    pub flags: u32,
}

#[derive(Debug, Clone)]
pub struct Init {
    pub track_id: u32,
    pub timescale: u32,
    pub codec: Codec,
    pub defaults: Defaults,
    /// Media time where presentation starts (first non-empty `elst` edit), in
    /// the track timescale. Subtract it from sample pts to get presentation time.
    pub edit_offset: i64,
}

/// Parse the init range (`ftyp` + `moov`). Expects a single track, as YouTube serves.
pub fn parse_init(b: &[u8]) -> Result<Init> {
    let moov = need(b, b"moov")?;
    let trak = need(moov.body, b"trak")?;
    let mut tkhd = Rd::new(need(trak.body, b"tkhd")?.body);
    let (v, _) = tkhd.full()?;
    tkhd.skip(if v == 1 { 16 } else { 8 })?;
    let track_id = tkhd.u32()?;

    let mut edit_offset = 0i64;
    if let Some(edts) = find(trak.body, b"edts")? {
        if let Some(elst) = find(edts.body, b"elst")? {
            let mut r = Rd::new(elst.body);
            let (v, _) = r.full()?;
            for _ in 0..r.u32()? {
                let media_time = if v == 1 { r.skip(8)?; r.u64()? as i64 } else { r.skip(4)?; r.u32()? as i32 as i64 };
                r.skip(4)?; // media_rate
                if media_time >= 0 {
                    edit_offset = media_time;
                    break;
                }
            }
        }
    }

    let mdia = need(trak.body, b"mdia")?;
    let mut mdhd = Rd::new(need(mdia.body, b"mdhd")?.body);
    let (v, _) = mdhd.full()?;
    mdhd.skip(if v == 1 { 16 } else { 8 })?;
    let timescale = mdhd.u32()?;
    if timescale == 0 {
        return err("timescale 0");
    }
    let stbl = need(need(mdia.body, b"minf")?.body, b"stbl")?;
    let mut stsd = Rd::new(need(stbl.body, b"stsd")?.body);
    stsd.full()?;
    if stsd.u32()? < 1 {
        return err("stsd has no entries");
    }
    let entry = boxes(&stsd.b[stsd.p..]).next().ok_or_else(|| Error("empty stsd".into()))??;
    let codec = match &entry.ty {
        b"avc1" | b"avc3" => parse_avc(entry.body)?,
        b"mp4a" => parse_mp4a(entry.body)?,
        other => Codec::Other(*other),
    };

    let mut defaults = Defaults::default();
    if let Some(mvex) = find(moov.body, b"mvex")? {
        if let Some(trex) = find(mvex.body, b"trex")? {
            let mut r = Rd::new(trex.body);
            r.full()?;
            r.skip(8)?; // track_ID, default_sample_description_index
            defaults = Defaults { duration: r.u32()?, size: r.u32()?, flags: r.u32()? };
        }
    }
    Ok(Init { track_id, timescale, codec, defaults, edit_offset })
}

fn parse_avc(body: &[u8]) -> Result<Codec> {
    let mut r = Rd::new(body);
    r.skip(8 + 16)?; // SampleEntry + pre_defined/reserved
    let width = r.u16()?;
    let height = r.u16()?;
    r.skip(4 + 4 + 4 + 2 + 32 + 2 + 2)?; // resolutions, reserved, frame_count, name, depth, pre_defined
    let avcc = need(&body[r.p..], b"avcC")?.body;
    let mut a = Rd::new(avcc);
    let _version = a.u8()?;
    let profile = a.u8()?;
    let _compat = a.u8()?;
    let level = a.u8()?;
    let nal_length_size = (a.u8()? & 3) + 1;
    let mut sps = Vec::new();
    for _ in 0..(a.u8()? & 0x1f) {
        let n = a.u16()? as usize;
        sps.push(a.take(n)?.to_vec());
    }
    let mut pps = Vec::new();
    for _ in 0..a.u8()? {
        let n = a.u16()? as usize;
        pps.push(a.take(n)?.to_vec());
    }
    Ok(Codec::Avc { width, height, nal_length_size, profile, level, sps, pps, avcc: avcc.to_vec() })
}

/// MPEG-4 descriptor header: tag and variable-length size.
fn descriptor<'a>(r: &mut Rd<'a>) -> Result<(u8, Rd<'a>)> {
    let tag = r.u8()?;
    let mut len = 0usize;
    for _ in 0..4 {
        let c = r.u8()?;
        len = (len << 7) | (c & 0x7f) as usize;
        if c & 0x80 == 0 {
            break;
        }
    }
    Ok((tag, Rd::new(r.take(len)?)))
}

fn parse_mp4a(body: &[u8]) -> Result<Codec> {
    let mut r = Rd::new(body);
    r.skip(8 + 8)?; // SampleEntry + reserved
    let channels = r.u16()?;
    r.skip(2 + 2 + 2)?; // samplesize, pre_defined, reserved
    let sample_rate = r.u32()? >> 16;
    let mut e = Rd::new(need(&body[r.p..], b"esds")?.body);
    e.full()?;
    let (tag, mut es) = descriptor(&mut e)?;
    if tag != 3 {
        return err(format!("esds: expected ES_Descriptor, got tag {tag}"));
    }
    es.skip(2)?; // ES_ID
    let flags = es.u8()?;
    if flags & 0x80 != 0 {
        es.skip(2)?;
    }
    if flags & 0x40 != 0 {
        let n = es.u8()? as usize;
        es.skip(n)?;
    }
    if flags & 0x20 != 0 {
        es.skip(2)?;
    }
    let (tag, mut dc) = descriptor(&mut es)?;
    if tag != 4 {
        return err(format!("esds: expected DecoderConfigDescriptor, got tag {tag}"));
    }
    dc.skip(13)?; // objectTypeIndication .. avgBitrate
    let (tag, dsi) = descriptor(&mut dc)?;
    if tag != 5 {
        return err(format!("esds: expected DecoderSpecificInfo, got tag {tag}"));
    }
    let asc = dsi.b.to_vec();
    let object_type = asc.first().map_or(0, |b| b >> 3);
    Ok(Codec::Aac { channels, sample_rate, object_type, asc })
}

/// One `sidx` reference: a fragment's absolute byte range and time span.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SegmentRef {
    pub offset: u64,
    pub size: u32,
    /// Earliest presentation time, in the sidx timescale.
    pub start: u64,
    pub duration: u32,
    pub starts_with_sap: bool,
}

#[derive(Debug, Clone)]
pub struct Index {
    pub timescale: u32,
    pub segments: Vec<SegmentRef>,
}

/// Parse the index range. `file_offset` is where `b` starts in the file
/// (`indexRange.start`), since segment offsets are relative to the sidx end.
pub fn parse_sidx(b: &[u8], file_offset: u64) -> Result<Index> {
    let sidx = need(b, b"sidx")?;
    let mut r = Rd::new(sidx.body);
    let (v, _) = r.full()?;
    r.skip(4)?; // reference_ID
    let timescale = r.u32()?;
    let (mut t, first) = if v == 0 { (r.u32()? as u64, r.u32()? as u64) } else { (r.u64()?, r.u64()?) };
    r.skip(2)?;
    let count = r.u16()?;
    let mut offset = file_offset + sidx.end as u64 + first;
    let mut segments = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let a = r.u32()?;
        if a & 0x8000_0000 != 0 {
            return err("hierarchical sidx not supported");
        }
        let size = a & 0x7fff_ffff;
        let duration = r.u32()?;
        let sap = r.u32()?;
        segments.push(SegmentRef { offset, size, start: t, duration,
                                   starts_with_sap: sap & 0x8000_0000 != 0 });
        offset += size as u64;
        t += duration as u64;
    }
    Ok(Index { timescale, segments })
}

/// A coded sample. Times are in the track timescale; `pts = dts + composition offset`.
#[derive(Debug, Clone)]
pub struct Sample {
    pub dts: u64,
    pub pts: i64,
    pub duration: u32,
    pub keyframe: bool,
    pub data: Vec<u8>,
}

/// Parse every `moof`+`mdat` pair in `b` (the bytes of one or more fragments).
pub fn parse_fragments(b: &[u8], init: &Init) -> Result<Vec<Sample>> {
    let mut out = Vec::new();
    for bx in boxes(b) {
        let bx = bx?;
        if &bx.ty != b"moof" {
            continue;
        }
        let moof_start = bx.start;
        for tb in boxes(bx.body) {
            let traf = tb?;
            if &traf.ty != b"traf" {
                continue;
            }
            parse_traf(b, moof_start, traf.body, init, &mut out)?;
        }
    }
    Ok(out)
}

fn parse_traf(file: &[u8], moof_start: usize, traf: &[u8], init: &Init, out: &mut Vec<Sample>) -> Result<()> {
    let mut t = Rd::new(need(traf, b"tfhd")?.body);
    let (_, fl) = t.full()?;
    let track = t.u32()?;
    if track != init.track_id {
        return Ok(());
    }
    let mut base = moof_start as u64;
    if fl & 0x01 != 0 {
        base = t.u64()?; // explicit base-data-offset, relative to `file` here
    }
    if fl & 0x02 != 0 {
        t.skip(4)?;
    }
    let dur = if fl & 0x08 != 0 { t.u32()? } else { init.defaults.duration };
    let size = if fl & 0x10 != 0 { t.u32()? } else { init.defaults.size };
    let flags = if fl & 0x20 != 0 { t.u32()? } else { init.defaults.flags };

    let mut dts = match find(traf, b"tfdt")? {
        Some(b) => {
            let mut r = Rd::new(b.body);
            let (v, _) = r.full()?;
            if v == 1 { r.u64()? } else { r.u32()? as u64 }
        }
        None => out.last().map_or(0, |s: &Sample| s.dts + s.duration as u64),
    };
    let mut data_pos = base;
    for tb in boxes(traf) {
        let trun = tb?;
        if &trun.ty != b"trun" {
            continue;
        }
        let mut r = Rd::new(trun.body);
        let (v, tf) = r.full()?;
        let count = r.u32()?;
        if tf & 0x01 != 0 {
            data_pos = (base as i64 + r.u32()? as i32 as i64) as u64;
        }
        let first_flags = if tf & 0x04 != 0 { Some(r.u32()?) } else { None };
        let per = [(tf & 0x100) != 0, (tf & 0x200) != 0, (tf & 0x400) != 0, (tf & 0x800) != 0];
        let per_len = per.iter().filter(|&&x| x).count() * 4;
        if per_len > 0 && (count as usize).checked_mul(per_len).is_none_or(|n| n > r.left()) {
            return err(format!("trun claims {count} samples but has {} bytes", r.left()));
        }
        for i in 0..count {
            let d = if per[0] { r.u32()? } else { dur };
            let s = if per[1] { r.u32()? } else { size };
            let f = if per[2] { r.u32()? } else if i == 0 { first_flags.unwrap_or(flags) } else { flags };
            let cto = if per[3] { if v == 0 { r.u32()? as i64 } else { r.u32()? as i32 as i64 } } else { 0 };
            let start = data_pos as usize;
            let end = start.checked_add(s as usize).filter(|&e| e <= file.len())
                .ok_or_else(|| Error(format!("sample {i} [{start}+{s}] outside the fragment")))?;
            out.push(Sample { dts, pts: dts as i64 + cto, duration: d,
                              keyframe: f & 0x0001_0000 == 0, data: file[start..end].to_vec() });
            data_pos += s as u64;
            dts += d as u64;
        }
    }
    Ok(())
}

/// Split an AVC sample (length-prefixed NAL units) into NAL unit payloads.
pub fn avc_nals(sample: &[u8], nal_length_size: u8) -> Result<Vec<&[u8]>> {
    let mut r = Rd::new(sample);
    let mut nals = Vec::new();
    while r.left() > 0 {
        let mut n = 0usize;
        for _ in 0..nal_length_size {
            n = (n << 8) | r.u8()? as usize;
        }
        nals.push(r.take(n)?);
    }
    Ok(nals)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bx(ty: &[u8; 4], body: &[u8]) -> Vec<u8> {
        let mut v = ((body.len() + 8) as u32).to_be_bytes().to_vec();
        v.extend_from_slice(ty);
        v.extend_from_slice(body);
        v
    }

    #[test]
    fn rejects_box_larger_than_buffer() {
        let mut b = bx(b"moov", &[0; 4]);
        b[3] = 200;
        assert!(parse_init(&b).is_err());
    }

    #[test]
    fn rejects_truncated_input_without_panicking() {
        for n in 0..64 {
            let b = bx(b"sidx", &[0xff; 56]);
            let _ = parse_sidx(&b[..n.min(b.len())], 0);
        }
    }

    #[test]
    fn splits_length_prefixed_nals() {
        let s = [0, 0, 0, 2, 0x65, 1, 0, 0, 0, 1, 0x41];
        let n = avc_nals(&s, 4).unwrap();
        assert_eq!(n, vec![&[0x65, 1][..], &[0x41][..]]);
        assert!(avc_nals(&s[..5], 4).is_err());
    }
}
