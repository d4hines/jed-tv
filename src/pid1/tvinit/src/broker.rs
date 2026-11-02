//! The native half of the connection broker (`Request.connect` in TV.Core).
//! It moves an app's request bytes to Core, which parses and decides (TV.Core
//! .parseRequest), and, only after Core grants it, connects to the host Core
//! pointed at and passes the socket to the app. This file never grants
//! anything itself and never interprets the request.
//!
//! Wire format, one datagram each way: app -> PID 1 is a 4-byte request
//! number then `connect <host> <port>`; PID 1 -> app is the same 4 bytes, a
//! little-endian i32 (0 with the socket attached, or an errno).
use std::io;
use std::net::{IpAddr, SocketAddr, TcpStream, ToSocketAddrs};
use std::os::fd::{AsRawFd, OwnedFd, RawFd};
use std::time::Duration;

const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);

/// Read one request datagram without blocking, unread.
pub fn receive(socket: &OwnedFd) -> Option<Vec<u8>> {
    let mut buffer = [0u8; 300];
    let n = unsafe { libc::recv(socket.as_raw_fd(), buffer.as_mut_ptr().cast(), buffer.len(), libc::MSG_DONTWAIT) };
    if n <= 0 { return None; }
    Some(buffer[..n as usize].to_vec())
}

/// Answer a request Core refused.
pub fn refuse(socket: &OwnedFd, number: [u8; 4]) { let _ = reply(socket.as_raw_fd(), number, libc::EACCES, None); }

/// Carry out a request Core granted, on its own thread so a slow lookup or
/// connect never delays Home. The app gets the socket or an errno.
pub fn grant(socket: &OwnedFd, number: [u8; 4], host: Vec<u8>, port: u16, log: fn(String)) -> io::Result<()> {
    let socket = socket.try_clone()?;
    std::thread::Builder::new().name("broker".into()).spawn(move || {
        let host = String::from_utf8_lossy(&host).into_owned();
        let sent = match open(&host, port) {
            Ok(stream) => {
                log(format!("broker: connected {host}:{port} for the app"));
                reply(socket.as_raw_fd(), number, 0, Some(stream.as_raw_fd()))
            }
            Err(error) => {
                log(format!("broker: {host}:{port} failed: {error}"));
                reply(socket.as_raw_fd(), number, error.raw_os_error().unwrap_or(libc::EIO), None)
            }
        };
        if let Err(error) = sent { log(format!("broker: reply not delivered: {error}")); }
    })?;
    Ok(())
}

/// Resolve and connect. Never to this TV itself: a name that resolves to a
/// loopback or unspecified address would otherwise reach PID 1's own services.
fn open(host: &str, port: u16) -> io::Result<TcpStream> {
    let mut last = io::Error::new(io::ErrorKind::NotFound, "no address");
    for address in (host, port).to_socket_addrs()? {
        if local(&address) {
            last = io::Error::from_raw_os_error(libc::EACCES);
            continue;
        }
        match TcpStream::connect_timeout(&address, CONNECT_TIMEOUT) {
            Ok(stream) => return Ok(stream),
            Err(error) => last = error,
        }
    }
    Err(last)
}

fn local(address: &SocketAddr) -> bool {
    match address.ip() {
        IpAddr::V4(ip) => ip.is_loopback() || ip.is_unspecified(),
        IpAddr::V6(ip) => ip.is_loopback() || ip.is_unspecified()
            || ip.to_ipv4_mapped().is_some_and(|v4| v4.is_loopback() || v4.is_unspecified()),
    }
}

#[repr(C, align(8))]
struct Control([u8; 64]);

/// One datagram: the request number, a little-endian i32 status, with `fd`
/// attached if given.
fn reply(socket: RawFd, number: [u8; 4], status: i32, fd: Option<RawFd>) -> io::Result<()> {
    let mut payload = [0u8; 8];
    payload[..4].copy_from_slice(&number);
    payload[4..].copy_from_slice(&status.to_le_bytes());
    let mut iov = libc::iovec { iov_base: payload.as_ptr() as *mut _, iov_len: payload.len() };
    let mut control = Control([0; 64]);
    let mut message: libc::msghdr = unsafe { std::mem::zeroed() };
    message.msg_iov = &mut iov;
    message.msg_iovlen = 1;
    if let Some(fd) = fd {
        message.msg_control = control.0.as_mut_ptr().cast();
        message.msg_controllen = unsafe { libc::CMSG_SPACE(std::mem::size_of::<RawFd>() as u32) } as _;
        unsafe {
            let header = libc::CMSG_FIRSTHDR(&message);
            (*header).cmsg_level = libc::SOL_SOCKET;
            (*header).cmsg_type = libc::SCM_RIGHTS;
            (*header).cmsg_len = libc::CMSG_LEN(std::mem::size_of::<RawFd>() as u32) as _;
            std::ptr::write_unaligned(libc::CMSG_DATA(header) as *mut RawFd, fd);
        }
    }
    if unsafe { libc::sendmsg(socket, &message, libc::MSG_NOSIGNAL | libc::MSG_DONTWAIT) } < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn never_connects_to_this_tv() {
        for local_address in ["127.0.0.1:80", "0.0.0.0:80", "[::1]:80", "[::]:80", "[::ffff:127.0.0.1]:80"] {
            assert!(local(&local_address.parse().unwrap()), "{local_address}");
        }
        assert!(!local(&"192.168.77.1:8090".parse().unwrap()));
    }
}
