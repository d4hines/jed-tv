//! Loopback TCP to an ADB reverse abstract Unix socket, without Bionic/netd.
use std::{io, net::{Shutdown, TcpListener}, os::{linux::net::SocketAddrExt,
    unix::net::{SocketAddr, UnixStream}}, sync::{Arc, atomic::{AtomicUsize, Ordering}},
    thread, time::{Duration, Instant}};

fn main() -> io::Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        return Err(io::Error::other("usage: adb-relay PORT ABSTRACT_SOCKET"));
    }
    let port: u16 = args[0].parse().map_err(io::Error::other)?;
    let address = SocketAddr::from_abstract_name(args[1].as_bytes())?;
    // Native startup can precede netd assigning loopback its address. Keep
    // this wait in the relay so display and audio can start independently.
    let deadline = Instant::now() + Duration::from_secs(30);
    let listener = loop {
        match TcpListener::bind(("127.0.0.1", port)) {
            Ok(listener) => break listener,
            Err(error) if error.kind() == io::ErrorKind::AddrNotAvailable
                && Instant::now() < deadline => thread::sleep(Duration::from_millis(100)),
            Err(error) => return Err(error),
        }
    };
    let active = Arc::new(AtomicUsize::new(0));
    println!("ADB_RELAY ready 127.0.0.1:{port} -> @{}", args[1]);
    for connection in listener.incoming() {
        let mut tcp = connection?;
        if active.fetch_add(1, Ordering::Relaxed) >= 32 {
            active.fetch_sub(1, Ordering::Relaxed);
            continue;
        }
        let address = address.clone();
        let active = active.clone();
        thread::spawn(move || {
            let result = (|| -> io::Result<()> {
                tcp.set_nodelay(true)?;
                let mut unix = UnixStream::connect_addr(&address)?;
                let mut tcp_read = tcp.try_clone()?;
                let mut unix_write = unix.try_clone()?;
                let outgoing = thread::spawn(move || {
                    let result = io::copy(&mut tcp_read, &mut unix_write);
                    let _ = unix_write.shutdown(Shutdown::Write);
                    result
                });
                let result = io::copy(&mut unix, &mut tcp);
                let _ = tcp.shutdown(Shutdown::Both);
                let _ = unix.shutdown(Shutdown::Both);
                let _ = outgoing.join();
                result.map(|_| ())
            })();
            if let Err(error) = result { eprintln!("ADB_RELAY: {error}"); }
            active.fetch_sub(1, Ordering::Relaxed);
        });
    }
    Ok(())
}
