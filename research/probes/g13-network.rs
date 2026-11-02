//! Compare Android socket permission for the launcher and confined app IDs.
use std::{io::{self, Read, Write}, net::TcpStream, time::Duration};
extern "C" {
    fn setgroups(n: usize, groups: *const u32) -> i32;
    fn setgid(gid: u32) -> i32;
    fn setuid(uid: u32) -> i32;
}
fn main() -> io::Result<()> {
    let mut args = std::env::args().skip(1);
    let uid: u32 = args.next().expect("UID").parse().unwrap();
    let groups: Vec<u32> = args.map(|s| s.parse().unwrap()).collect();
    unsafe {
        if setgroups(groups.len(), groups.as_ptr()) != 0 || setgid(4000) != 0 || setuid(uid) != 0 {
            return Err(io::Error::last_os_error());
        }
    }
    let connection = TcpStream::connect("127.0.0.1:8899");
    println!("uid={uid} groups={groups:?} connect={connection:?}");
    if let Ok(mut stream) = connection {
        stream.set_read_timeout(Some(Duration::from_secs(5)))?;
        stream.write_all(b"CONNECT www.youtube.com:443 HTTP/1.1\r\nHost: www.youtube.com:443\r\n\r\n")?;
        let mut reply = [0; 256];
        match stream.read(&mut reply) {
            Ok(n) => println!("reply={:?}", String::from_utf8_lossy(&reply[..n])),
            Err(e) => println!("reply error={e}"),
        }
    }
    Ok(())
}
