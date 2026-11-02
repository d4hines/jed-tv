//! Host-only differential oracle using the retained Rust reference processor.
#[path = "../../../../src/pid1/audio/protocol.rs"]
#[allow(dead_code)]
mod protocol;
#[path = "../../../../research/probes/audio/pcm-converter.rs"]
mod converter;
use std::io::{self, Read, Write};
use std::sync::{Arc, Mutex, atomic::AtomicU32};
use std::time::Duration;

#[derive(Default)]
struct State { output: Vec<u8>, blocks: Vec<Vec<i16>>, opens: u32, drops: u32 }
struct Wire { input: io::Cursor<Vec<u8>>, state: Arc<Mutex<State>> }
impl Read for Wire {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> { self.input.read(out) }
}
impl Write for Wire {
    fn write(&mut self, input: &[u8]) -> io::Result<usize> {
        self.state.lock().unwrap().output.extend_from_slice(input); Ok(input.len())
    }
    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}
impl protocol::Session for Wire {
    fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> { Ok(()) }
    fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> { Ok(()) }
}
struct Backend(Arc<Mutex<State>>);
struct Track { state: Arc<Mutex<State>>, channels: u32, frames: u64 }
impl Drop for Track { fn drop(&mut self) { self.state.lock().unwrap().drops += 1; } }
impl protocol::Stream for Track {
    fn write(&mut self, samples: &[i16]) -> io::Result<()> {
        self.frames += samples.len() as u64 / self.channels as u64;
        self.state.lock().unwrap().blocks.push(samples.to_vec()); Ok(())
    }
    fn written(&self) -> u64 { self.frames }
    fn played(&self) -> io::Result<u64> { Ok(self.frames) }
}
impl protocol::Backend for Backend {
    type Track = Track;
    fn supports(&self, rate: u32, channels: u32) -> bool {
        [44100, 48000].contains(&rate) && [1, 2].contains(&channels)
    }
    fn open(&self, _: u32, channels: u32) -> io::Result<Track> {
        self.0.lock().unwrap().opens += 1;
        Ok(Track { state: self.0.clone(), channels, frames: 0 })
    }
    fn name(&self) -> &'static str { "oracle" }
}
fn put_u32(out: &mut Vec<u8>, n: u32) { out.extend(n.to_le_bytes()); }
fn convert_file(input: &str, output: &str) -> io::Result<()> {
    let data = std::fs::read(input)?;
    let word = |at: usize| u32::from_le_bytes(data[at..at+4].try_into().unwrap());
    let (rate, channels, chunks) = (word(0), word(4), word(8));
    let mut converter = converter::Converter::new(rate, channels)?;
    let mut offset = 12;
    let (mut input_frames, mut output_frames) = (0, 0);
    let mut out = Vec::new();
    for _ in 0..chunks {
        let size = word(offset) as usize;
        offset += 4;
        let samples: Vec<_> = data[offset..offset+size].chunks_exact(2)
            .map(|v| i16::from_le_bytes(v.try_into().unwrap())).collect();
        input_frames += samples.len() as u64 / channels as u64;
        offset += size;
        let converted = converter.convert(&samples)?;
        output_frames += converted.len() as u64 / 4;
        put_u32(&mut out, converted.len() as u32);
        out.extend(converted);
    }
    for frames in 0..=output_frames {
        out.extend(converter::consumed_input_frames(frames, rate, input_frames).to_le_bytes());
    }
    std::fs::write(output, out)
}
fn main() -> io::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args[1] == "convert" { return convert_file(&args[2], &args[3]); }
    let data = std::fs::read(&args[1])?;
    let gain = u32::from_le_bytes(data[..4].try_into().unwrap());
    let state = Arc::new(Mutex::new(State::default()));
    let result = protocol::client(Wire { input: io::Cursor::new(data[4..].to_vec()), state: state.clone() },
        &Backend(state.clone()), &AtomicU32::new(gain));
    let s = state.lock().unwrap();
    let mut out = vec![u8::from(result.is_err())];
    put_u32(&mut out, s.opens);
    put_u32(&mut out, s.drops);
    put_u32(&mut out, s.output.len() as u32);
    out.extend(&s.output);
    put_u32(&mut out, s.blocks.len() as u32);
    for block in &s.blocks {
        put_u32(&mut out, block.len() as u32 * 2);
        for sample in block { out.extend(sample.to_le_bytes()); }
    }
    std::fs::write(&args[2], out)
}
