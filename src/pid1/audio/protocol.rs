//! Shared PCM protocol and global gain policy. Backend selection never changes app ABI.
#[path = "../../shared/audio_level.rs"]
mod level;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{
    atomic::{AtomicU32, AtomicUsize, Ordering},
    Arc,
};
use std::time::{Duration, Instant};
fn fail(message: impl Into<String>) -> io::Error {
    io::Error::other(message.into())
}
pub trait Stream {
    fn write(&mut self, samples: &[i16]) -> io::Result<()>;
    fn written(&self) -> u64;
    fn played(&self) -> io::Result<u64>;
}
pub trait Backend: Send + Sync + 'static {
    type Track: Stream;
    fn supports(&self, rate: u32, channels: u32) -> bool;
    fn open(&self, rate: u32, channels: u32) -> io::Result<Self::Track>;
    fn name(&self) -> &'static str;
}
pub trait Session: Read + Write {
    fn set_read_timeout(&self, timeout: Option<Duration>) -> io::Result<()>;
    fn set_write_timeout(&self, timeout: Option<Duration>) -> io::Result<()>;
}
impl Session for TcpStream {
    fn set_read_timeout(&self, t: Option<Duration>) -> io::Result<()> {
        TcpStream::set_read_timeout(self, t)
    }
    fn set_write_timeout(&self, t: Option<Duration>) -> io::Result<()> {
        TcpStream::set_write_timeout(self, t)
    }
}
fn u32_le(socket: &mut impl Session) -> io::Result<u32> {
    let mut bytes = [0; 4];
    socket.read_exact(&mut bytes)?;
    Ok(u32::from_le_bytes(bytes))
}
fn status(socket: &mut impl Session, value: i32) -> io::Result<()> {
    socket.write_all(&value.to_le_bytes())
}
pub fn client(socket: impl Session, backend: &impl Backend, gain: &AtomicU32) -> io::Result<()> {
    client_observed(socket, backend, gain, &|event| println!("{event}"))
}
fn client_observed(
    mut socket: impl Session,
    backend: &impl Backend,
    gain: &AtomicU32,
    report: &dyn Fn(&str),
) -> io::Result<()> {
    socket.set_read_timeout(Some(Duration::from_secs(3)))?;
    socket.set_write_timeout(Some(Duration::from_secs(3)))?;
    let mut command = [0];
    socket.read_exact(&mut command)?;
    if command[0] != b'O' {
        status(&mut socket, -22)?;
        return Err(fail("expected open"));
    }
    let rate = u32_le(&mut socket)?;
    let channels = u32_le(&mut socket)?;
    let boost = u32_le(&mut socket)?;
    if !(8000..=96000).contains(&rate)
        || !(1..=2).contains(&channels)
        || !(1..=4).contains(&boost)
        || !backend.supports(rate, channels)
    {
        status(&mut socket, -22)?;
        return Err(fail("invalid PCM format"));
    }
    let mut track = match backend.open(rate, channels) {
        Ok(t) => t,
        Err(e) => {
            status(&mut socket, -1)?;
            return Err(e);
        }
    };
    status(&mut socket, 0)?;
    let mut next_report = 1;
    let mut source_peak = 0i32;
    let mut output_peak = 0i32;
    loop {
        // A paused player may keep a valid stream idle indefinitely. Payloads
        // remain bounded by the timeout, and concurrent clients are capped.
        socket.set_read_timeout(None)?;
        if socket.read(&mut command)? == 0 {
            return Ok(());
        }
        socket.set_read_timeout(Some(Duration::from_secs(3)))?;
        match command[0] {
            b'W' => {
                let bytes = u32_le(&mut socket)? as usize;
                if bytes > 1048576 || bytes % (channels as usize * 2) != 0 {
                    status(&mut socket, -22)?;
                    return Err(fail("invalid PCM block"));
                }
                let mut data = vec![0; bytes];
                socket.read_exact(&mut data)?;
                let level = f32::from_bits(gain.load(Ordering::Relaxed));
                let samples: Vec<i16> = data
                    .chunks_exact(2)
                    .map(|b| {
                        let sample = i16::from_le_bytes([b[0], b[1]]) as f32;
                        source_peak = source_peak.max((sample as i32).abs());
                        let output = ((sample * boost as f32).clamp(-32767.0, 32767.0) * level).round() as i16;
                        output_peak = output_peak.max(i32::from(output).abs());
                        output
                    })
                    .collect();
                match track.write(&samples) {
                    Ok(()) => status(&mut socket, 0)?,
                    Err(e) => {
                        status(&mut socket, -1)?;
                        return Err(e);
                    }
                }
                if track.written() >= next_report {
                    report(&format!(
                        "RUST_AUDIO written={} played={} source_peak={} output_peak={} gain={}",
                        track.written(), track.played()?, source_peak, output_peak, level
                    ));
                    next_report = track.written() + rate as u64 * 5;
                    source_peak = 0; output_peak = 0;
                }
            }
            b'D' => {
                socket.write_all(&track.written().to_le_bytes())?;
                socket.write_all(&track.played()?.to_le_bytes())?;
            }
            b'R' => {
                drop(track);
                track = match backend.open(rate, channels) {
                    Ok(t) => t,
                    Err(e) => {
                        status(&mut socket, -1)?;
                        return Err(e);
                    }
                };
                next_report = 1;
                status(&mut socket, 0)?;
            }
            b'F' => {
                let until = Instant::now() + Duration::from_secs(5);
                loop {
                    let played = match track.played() {
                        Ok(value) => value,
                        Err(error) => {
                            status(&mut socket, -1)?;
                            return Err(error);
                        }
                    };
                    if played >= track.written() {
                        status(&mut socket, 0)?;
                        break;
                    }
                    if Instant::now() >= until {
                        status(&mut socket, -1)?;
                        break;
                    }
                    std::thread::sleep(Duration::from_millis(5));
                }
            }
            _ => {
                status(&mut socket, -22)?;
                return Err(fail("unknown command"));
            }
        }
    }
}
fn read_gain() -> f32 {
    std::fs::read_to_string(std::path::Path::new(&std::env::var("TV_RUNTIME_DIR")
        .unwrap_or_else(|_| "/tmp".into())).join("app-volume"))
        .ok()
        .and_then(|s| s.trim().parse::<f32>().ok())
        .filter(|v| v.is_finite())
        .unwrap_or(0.0)
        .clamp(0.0, level::MAX_GAIN)
}

/// Temporary standalone listener. PID1 can embed `client` and own acceptance itself.
pub fn serve(backend: impl Backend, port: u16) -> io::Result<()> {
    serve_listener(backend, TcpListener::bind(("127.0.0.1", port))?, |event| println!("{event}"))
}

/// Let the owning supervisor bind the socket and publish readiness first.
pub fn serve_listener(backend: impl Backend, listener: TcpListener, report: impl Fn(&str) + Send + Sync + 'static) -> io::Result<()> {
    let report=Arc::new(report);
    let address = listener.local_addr()?;
    let backend = Arc::new(backend);
    let gain = Arc::new(AtomicU32::new(read_gain().to_bits()));
    let levels = gain.clone();
    std::thread::spawn(move || loop {
        levels.store(read_gain().to_bits(), Ordering::Relaxed);
        std::thread::sleep(Duration::from_millis(50));
    });
    let active = Arc::new(AtomicUsize::new(0));
    report(&format!("RUST_AUDIO ready {address} {}", backend.name()));
    for connection in listener.incoming() {
        let socket = connection?;
        socket.set_nodelay(true)?;
        if active.fetch_add(1, Ordering::Relaxed) >= 4 {
            active.fetch_sub(1, Ordering::Relaxed);
            drop(socket);
            continue;
        }
        let (backend, gain, active) = (backend.clone(), gain.clone(), active.clone());
        let report=report.clone();
        std::thread::spawn(move || {
            if let Err(error) = client_observed(socket, &*backend, &gain, &*report) {
                report(&format!("RUST_AUDIO client: {error}"));
            }
            active.fetch_sub(1, Ordering::Relaxed);
        });
    }
    Ok(())
}

#[cfg(test)]
#[path = "../../../test/unit/audio/protocol-tests.rs"]
mod tests;
