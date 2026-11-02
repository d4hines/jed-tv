//! Silent, static ARM32 test client for the native/Java PCM bridge protocol.
use std::io::{self, Read, Write};
use std::net::{Shutdown, TcpStream};
use std::time::Duration;
struct Client { socket: TcpStream }
impl Client {
    fn connect(address: &str) -> io::Result<Self> {
        let socket=TcpStream::connect(address)?;
        socket.set_nodelay(true)?;
        socket.set_read_timeout(Some(Duration::from_secs(7)))?;
        socket.set_write_timeout(Some(Duration::from_secs(4)))?;
        Ok(Self{socket})
    }
    fn send(&mut self, bytes: &[u8], fragmented: bool) -> io::Result<()> {
        if !fragmented {return self.socket.write_all(bytes)}
        let mut at=0; let mut index=0;
        while at<bytes.len() {
            let count=[1,2,7,31,997][index%5].min(bytes.len()-at);
            self.socket.write_all(&bytes[at..at+count])?;
            at+=count;index+=1;
        }
        Ok(())
    }
    fn status(&mut self)->io::Result<i32> {let mut b=[0;4];self.socket.read_exact(&mut b)?;Ok(i32::from_le_bytes(b))}
    fn open(&mut self,rate:u32,channels:u32)->io::Result<()> {
        self.raw_open(rate,channels,1)?;
        assert_eq!(self.status()?,0,"open {rate}/{channels}");Ok(())
    }
    fn raw_open(&mut self,rate:u32,channels:u32,boost:u32)->io::Result<()> {
        let mut b=vec![b'O'];for n in [rate,channels,boost]{b.extend(n.to_le_bytes());} self.send(&b,true)
    }
    fn write_silence(&mut self,frames:u32,channels:u32)->io::Result<()> {
        let count=frames*channels*2;let mut b=vec![b'W'];b.extend(count.to_le_bytes());b.resize(5+count as usize,0);
        self.send(&b,true)?;assert_eq!(self.status()?,0,"PCM write");Ok(())
    }
    fn write_pcm(&mut self,samples:&[i16])->io::Result<()> {
        let mut bytes=vec![b'W'];bytes.extend(((samples.len()*2) as u32).to_le_bytes());
        for sample in samples {bytes.extend(sample.to_le_bytes());}
        self.send(&bytes,false)?;assert_eq!(self.status()?,0,"PCM write");Ok(())
    }
    fn counters(&mut self)->io::Result<(u64,u64)> {
        self.send(b"D",false)?;let mut b=[0;16];self.socket.read_exact(&mut b)?;
        Ok((u64::from_le_bytes(b[..8].try_into().unwrap()),u64::from_le_bytes(b[8..].try_into().unwrap())))
    }
    fn command(&mut self,c:u8)->io::Result<()> {self.send(&[c],false)?;assert_eq!(self.status()?,0,"command {c}");Ok(())}
    fn rejected(&mut self)->io::Result<()> {
        match self.status() {
            Ok(n)=>assert_ne!(n,0,"malformed request accepted"),
            Err(e) if matches!(e.kind(),io::ErrorKind::UnexpectedEof|io::ErrorKind::ConnectionReset|io::ErrorKind::BrokenPipe)=>{},
            Err(e)=>return Err(e),
        } Ok(())
    }
}
fn main()->io::Result<()> {
    let args:Vec<String>=std::env::args().skip(1).collect();
    let address=args.iter().find(|a| !a.starts_with("--")).cloned().unwrap_or_else(||"127.0.0.1:8878".into());
    if args.iter().any(|a| a=="--jingle") {
        let mut client=Client::connect(&address)?;client.raw_open(48000,2,4)?;assert_eq!(client.status()?,0);
        let mut samples=Vec::with_capacity(96000);
        for n in 0..48000 {
            let local=n%12000;let frequency=[523.25,659.25,783.99,1046.50][n/12000];
            let edge=((local.min(11999-local)) as f64/480.0).min(1.0);
            let sample=((2.0*std::f64::consts::PI*frequency*local as f64/48000.0).sin()*32767.0*0.6*edge) as i16;
            samples.extend([sample,sample]);
        }
        println!("JINGLE: exactly one second, Doom boost4, existing server gain unchanged");
        client.write_pcm(&samples)?;client.command(b'F')?;
        println!("JINGLE PASS: {} frames",client.counters()?.0);return Ok(());
    }
    for rate in [44100,48000] {
        for channels in [1,2] {
            let mut c=Client::connect(&address)?;c.open(rate,channels)?;
            assert_eq!(c.counters()?,(0,0),"new counters");
            let frames=rate/4;c.write_silence(frames,channels)?;
            let first=c.counters()?;assert_eq!(first.0,frames as u64);assert!(first.1<=first.0);
            std::thread::sleep(Duration::from_millis(20));let second=c.counters()?;
            assert_eq!(second.0,first.0);assert!(second.1>=first.1&&second.1<=second.0);
            c.command(b'F')?;assert_eq!(c.counters()?,(frames as u64,frames as u64));
            c.command(b'R')?;assert_eq!(c.counters()?,(0,0),"reset counters");
            c.write_silence(frames,channels)?;c.command(b'F')?;
            println!("PASS: {rate}Hz/{channels}ch fragmented silent PCM, exact counters, reset, drain");
        }
    }
    {
        let mut c=Client::connect(&address)?;c.send(b"X",false)?;c.rejected()?;
        println!("PASS: invalid first command");
    }
    for (name,rate,channels,boost) in [("sample rate",0,2,1),("channel count",48000,3,1),("boost",48000,2,0)] {
        let mut c=Client::connect(&address)?;c.raw_open(rate,channels,boost)?;c.rejected()?;
        println!("PASS: invalid {name}");
    }
    {
        let mut c=Client::connect(&address)?;c.send(b"O\0\0",false)?;c.socket.shutdown(Shutdown::Write)?;c.rejected()?;
        println!("PASS: truncated open");
    }
    for (name,command,length,payload) in [("unknown command",b'Z',None,0),("oversized PCM",b'W',Some(1048580u32),0),("misaligned PCM",b'W',Some(1),0),("truncated PCM",b'W',Some(32),10)] {
        let mut c=Client::connect(&address)?;c.open(48000,2)?;
        let mut b=vec![command];if let Some(n)=length{b.extend(n.to_le_bytes());}b.resize(b.len()+payload,0);
        c.send(&b,true)?;if payload>0{c.socket.shutdown(Shutdown::Write)?;}c.rejected()?;
        println!("PASS: {name}");
    }
    {
        let mut c=Client::connect(&address)?;c.open(48000,2)?;c.write_silence(12000,2)?;c.command(b'F')?;
        println!("PASS: healthy client after rejected requests");
    }
    println!("AUDIO PROTOCOL PASS: 14 cases, silent PCM only");Ok(())
}
