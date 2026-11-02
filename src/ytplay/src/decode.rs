//! Video and audio decoders.
//! H.264: FFmpeg's decoder and our small shim, translated to Rust in src/ffh264
//! (it exports the ytdec_* functions declared below).
//! AAC: symphonia (pure Rust).

use ffh264 as _; // link the crate; its ytdec_* are #[no_mangle]

use libc::{c_int, c_void};
use std::io;

#[repr(C)]
struct RawFrame {
    y: *const u8,
    u: *const u8,
    v: *const u8,
    y_stride: c_int,
    u_stride: c_int,
    v_stride: c_int,
    width: c_int,
    height: c_int,
    pts: i64,
    keyframe: c_int,
    pix_fmt: c_int,
}

extern "C" {
    fn ytdec_open(avcc: *const u8, len: c_int, threads: c_int) -> *mut c_void;
    fn ytdec_send(d: *mut c_void, data: *const u8, len: c_int, pts: i64, dts: i64) -> c_int;
    fn ytdec_receive(d: *mut c_void, f: *mut RawFrame) -> c_int;
    fn ytdec_close(d: *mut c_void);
}

const AVERROR_EAGAIN: c_int = -libc::EAGAIN;

/// A decoded 8-bit YUV 4:2:0 picture, borrowed from the decoder until the next call.
pub struct Frame<'a> {
    pub y: &'a [u8],
    pub u: &'a [u8],
    pub v: &'a [u8],
    pub y_stride: usize,
    pub c_stride: usize,
    pub width: usize,
    pub height: usize,
    pub pts: i64,
    pub keyframe: bool,
}

// Only borrow bytes actually addressed by picture rows. A cropped frame can
// start inside an allocation, so including a whole final stride over-borrows.
fn plane_len(stride: c_int, width: usize, height: usize) -> Option<usize> {
    let stride = usize::try_from(stride).ok()?;
    if width == 0 || height == 0 || stride < width { return None; }
    stride.checked_mul(height - 1)?.checked_add(width).filter(|&n| n <= isize::MAX as usize)
}

pub struct H264 {
    d: *mut c_void,
    raw: RawFrame,
}

// The decoder is used from one thread at a time; FFmpeg's internal threads are its own.
unsafe impl Send for H264 {}

impl H264 {
    /// `avcc`: the avcC record from the MP4 init segment. `threads`: 0 = auto.
    pub fn new(avcc: &[u8], threads: u32) -> io::Result<Self> {
        let d = unsafe { ytdec_open(avcc.as_ptr(), avcc.len() as c_int, threads as c_int) };
        if d.is_null() {
            return Err(io::Error::other("ytdec_open failed"));
        }
        Ok(H264 { d, raw: unsafe { std::mem::zeroed() } })
    }

    /// Submit one MP4 sample (length-prefixed NAL units). Returns false if the
    /// decoder is full: call `receive` until it yields None, then send again.
    pub fn send(&mut self, data: &[u8], pts: i64, dts: i64) -> io::Result<bool> {
        match unsafe { ytdec_send(self.d, data.as_ptr(), data.len() as c_int, pts, dts) } {
            0 => Ok(true),
            AVERROR_EAGAIN => Ok(false),
            e => Err(io::Error::other(format!("h264 send error {e}"))),
        }
    }

    /// Signal end of stream so buffered frames come out.
    pub fn flush(&mut self) -> io::Result<()> {
        match unsafe { ytdec_send(self.d, std::ptr::null(), 0, 0, 0) } {
            0 | AVERROR_EAGAIN => Ok(()),
            e => Err(io::Error::other(format!("h264 flush error {e}"))),
        }
    }

    /// Next decoded frame, or None when more input is needed (or the stream ended).
    pub fn receive(&mut self) -> io::Result<Option<Frame<'_>>> {
        match unsafe { ytdec_receive(self.d, &mut self.raw) } {
            0 => {}
            1 | 2 => return Ok(None),
            e => return Err(io::Error::other(format!("h264 receive error {e}"))),
        }
        let r = &self.raw;
        if r.pix_fmt != 0 {
            return Err(io::Error::other(format!("unsupported pixel format {}", r.pix_fmt)));
        }
        let (w, h) = (r.width.max(0) as usize, r.height.max(0) as usize);
        let (ys, cs) = (r.y_stride as usize, r.u_stride as usize);
        let (cw, ch) = (w.div_ceil(2), h.div_ceil(2));
        let lengths = plane_len(r.y_stride, w, h)
            .zip(plane_len(r.u_stride, cw, ch)).zip(plane_len(r.v_stride, cw, ch));
        let Some(((yl, ul), vl)) = lengths else {
            return Err(io::Error::other("invalid decoded plane geometry"));
        };
        if r.y.is_null() || r.u.is_null() || r.v.is_null() || r.u_stride != r.v_stride {
            return Err(io::Error::other("invalid decoded plane pointers or chroma strides"));
        }
        unsafe {
            Ok(Some(Frame {
                y: std::slice::from_raw_parts(r.y, yl),
                u: std::slice::from_raw_parts(r.u, ul),
                v: std::slice::from_raw_parts(r.v, vl),
                y_stride: ys,
                c_stride: cs,
                width: w,
                height: h,
                pts: r.pts,
                keyframe: r.keyframe != 0,
            }))
        }
    }
}

impl Drop for H264 {
    fn drop(&mut self) {
        unsafe { ytdec_close(self.d) };
    }
}

impl Frame<'_> {
    /// MD5 of the packed planes, as `ffmpeg -f framemd5` computes for yuv420p.
    pub fn md5(&self) -> String {
        use md5::{Digest, Md5};
        let mut h = Md5::new();
        for row in 0..self.height {
            h.update(&self.y[row * self.y_stride..][..self.width]);
        }
        let (cw, ch) = (self.width.div_ceil(2), self.height.div_ceil(2));
        for plane in [self.u, self.v] {
            for row in 0..ch {
                h.update(&plane[row * self.c_stride..][..cw]);
            }
        }
        h.finalize().iter().map(|b| format!("{b:02x}")).collect()
    }
}

/// AAC-LC decoder producing interleaved i16 samples.
pub struct Aac {
    dec: Box<dyn symphonia::core::codecs::Decoder>,
    pub sample_rate: u32,
    pub channels: usize,
    buf: Option<symphonia::core::audio::SampleBuffer<i16>>,
}

impl Aac {
    pub fn new(asc: &[u8], sample_rate: u32, channels: u16) -> io::Result<Self> {
        use symphonia::core::audio::Channels;
        use symphonia::core::codecs::{CodecParameters, DecoderOptions, CODEC_TYPE_AAC};
        let layout = if channels == 1 { Channels::FRONT_LEFT }
                     else { Channels::FRONT_LEFT | Channels::FRONT_RIGHT };
        let mut p = CodecParameters::new();
        p.for_codec(CODEC_TYPE_AAC).with_sample_rate(sample_rate).with_channels(layout)
            .with_extra_data(asc.to_vec().into_boxed_slice());
        let dec = symphonia::default::get_codecs().make(&p, &DecoderOptions::default())
            .map_err(|e| io::Error::other(format!("aac init: {e}")))?;
        Ok(Aac { dec, sample_rate, channels: channels as usize, buf: None })
    }

    /// Decode one AAC frame into interleaved i16 samples.
    pub fn decode(&mut self, data: &[u8], ts: u64) -> io::Result<&[i16]> {
        use symphonia::core::audio::SampleBuffer;
        use symphonia::core::formats::Packet;
        let pkt = Packet::new_from_slice(0, ts, 1024, data);
        let decoded = self.dec.decode(&pkt).map_err(|e| io::Error::other(format!("aac: {e}")))?;
        let spec = *decoded.spec();
        let cap = decoded.capacity() as u64;
        let buf = self.buf.get_or_insert_with(|| SampleBuffer::new(cap, spec));
        if (buf.capacity() as u64) < cap * spec.channels.count() as u64 {
            *buf = SampleBuffer::new(cap, spec);
        }
        buf.copy_interleaved_ref(decoded);
        Ok(buf.samples())
    }
}

#[cfg(test)]
mod tests {
    use super::plane_len;
    #[test]
    fn cropped_planes_do_not_borrow_last_row_padding() {
        assert_eq!(plane_len(128, 120, 3), Some(376));
        assert_eq!(plane_len(128, 120, 1), Some(120));
        assert_eq!(plane_len(-128, 120, 3), None);
        assert_eq!(plane_len(64, 120, 3), None);
        assert_eq!(plane_len(128, 0, 3), None);
        assert_eq!(plane_len(128, 120, usize::MAX), None);
    }
}
