//! Narrow PCM protocol for the temporary Android AudioTrack bridge.
use std::io::{self, Read, Write};
use std::net::TcpStream;
use crate::broker;
use std::sync::Mutex;

pub struct NativePcm { socket: Mutex<TcpStream>, rate: u32, channels: u32 }

fn status(stream: &mut TcpStream) -> io::Result<()> {
    let mut b = [0; 4];
    stream.read_exact(&mut b)?;
    if i32::from_le_bytes(b) == 0 { Ok(()) }
    else { Err(io::Error::other("Android PCM operation failed")) }
}

impl NativePcm {
    pub fn open(address: &str, rate: u32, channels: u32) -> io::Result<Self> {
        let mut socket = broker::connect_addr(address)?;
        socket.set_nodelay(true)?;
        socket.set_read_timeout(Some(std::time::Duration::from_secs(10)))?;
        socket.write_all(b"O")?;
        socket.write_all(&rate.to_le_bytes())?;
        socket.write_all(&channels.to_le_bytes())?;
        let boost = std::env::var("TV_AUDIO_BOOST").ok().and_then(|v| v.parse::<u32>().ok()).unwrap_or(1).clamp(1,4);
        socket.write_all(&boost.to_le_bytes())?;
        status(&mut socket)?;
        let pcm=Self { socket: Mutex::new(socket), rate, channels };
        pcm.prime()?;
        Ok(pcm)
    }

    fn prime(&self) -> io::Result<()> {
        // Android's start threshold can exceed Doom's steady-state 60 ms queue.
        self.write(&vec![0; (self.rate*self.channels/4) as usize])
    }

    pub fn write(&self, samples: &[i16]) -> io::Result<()> {
        let bytes: Vec<u8> = samples.iter().flat_map(|v| v.to_le_bytes()).collect();
        if bytes.len() > 1024*1024 { return Err(io::Error::other("PCM block too large")); }
        let mut socket = self.socket.lock().unwrap();
        socket.write_all(b"W")?;
        socket.write_all(&(bytes.len() as u32).to_le_bytes())?;
        socket.write_all(&bytes)?;
        status(&mut socket)
    }

    pub fn delay(&self) -> io::Result<i64> {
        let mut socket = self.socket.lock().unwrap();
        socket.write_all(b"D")?;
        let mut b=[0;16];
        socket.read_exact(&mut b)?;
        let sent=u64::from_le_bytes(b[..8].try_into().unwrap());
        let played=u64::from_le_bytes(b[8..].try_into().unwrap());
        Ok(sent.saturating_sub(played) as i64)
    }

    pub fn command(&self, command: u8) -> io::Result<()> {
        let mut socket = self.socket.lock().unwrap();
        socket.write_all(&[command])?;
        status(&mut socket)?;
        drop(socket);
        if command==b'R' { self.prime()?; }
        Ok(())
    }
}
