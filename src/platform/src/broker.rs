//! TCP connections from PID 1. A brokered app has no `socket` or `connect`
//! syscall: it asks PID 1 on its request socket (descriptor TVAPP_REQUESTS,
//! 4), PID 1 checks the host against the app's manifest, connects, and sends
//! back the connected socket. Outside PID 1 (a host run) there is no request
//! socket and this connects directly.
//!
//! Wire format, one datagram each way:
//!   app -> PID 1: a 4-byte request number, then `connect <host> <port>`
//!   PID 1 -> app: the same 4 bytes, then a little-endian i32: 0 with the
//!   socket attached, or an errno. Replies with another number are stale.
use std::io;
use std::net::TcpStream;
use std::os::fd::{FromRawFd, RawFd};
use std::sync::Mutex;

/// How long to wait for PID 1's answer, which includes its DNS lookup and
/// TCP connect.
const REPLY_TIMEOUT_MS: i32 = 20_000;

fn request_socket() -> Option<RawFd> {
    std::env::var("TVAPP_REQUESTS").ok()?.parse().ok()
}

/// Connect to `host:port`, through PID 1 when running under it.
pub fn connect(host: &str, port: u16) -> io::Result<TcpStream> {
    match request_socket() {
        Some(fd) => ask(fd, host, port),
        None => TcpStream::connect((host, port)),
    }
}

/// `connect` for a `host:port` string.
pub fn connect_addr(address: &str) -> io::Result<TcpStream> {
    let (host, port) = address.rsplit_once(':')
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "expected host:port"))?;
    let port = port.parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "bad port"))?;
    connect(host, port)
}

#[repr(C, align(8))]
struct Control([u8; 64]);

fn ask(socket: RawFd, host: &str, port: u16) -> io::Result<TcpStream> {
    // One request in flight per app, so a reply always answers the request
    // just sent. Replies to an earlier, timed-out request are discarded first.
    static ONE_AT_A_TIME: Mutex<u32> = Mutex::new(0);
    let mut counter = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
    *counter = counter.wrapping_add(1);
    let number = *counter;
    let mut request = number.to_le_bytes().to_vec();
    request.extend_from_slice(format!("connect {host} {port}").as_bytes());
    if unsafe { libc::send(socket, request.as_ptr().cast(), request.len(), libc::MSG_NOSIGNAL) } < 0 {
        return Err(io::Error::last_os_error());
    }
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(REPLY_TIMEOUT_MS as u64);
    let mut reply = loop {
        let left = deadline.saturating_duration_since(std::time::Instant::now()).as_millis() as i32;
        match receive(socket, left)? {
            Some(reply) if reply.number == number => break reply,
            Some(_stale) => continue,
            None => return Err(io::Error::new(io::ErrorKind::TimedOut, "PID 1 did not answer the connection request")),
        }
    };
    // Take the descriptor out first: dropping the reply closes any it still holds.
    match (reply.status, reply.socket.take()) {
        (0, Some(fd)) => Ok(unsafe { TcpStream::from_raw_fd(fd) }),
        (0, None) => Err(io::Error::other("PID 1 granted a connection without a socket")),
        (status, fd) => {
            if let Some(fd) = fd { unsafe { libc::close(fd); } }
            Err(io::Error::from_raw_os_error(status))
        }
    }
}

struct Reply { number: u32, status: i32, socket: Option<RawFd> }
impl Drop for Reply {
    fn drop(&mut self) {
        if let Some(fd) = self.socket.take() { unsafe { libc::close(fd); } }
    }
}

/// One reply from PID 1, waiting at most `timeout_ms`; None if none came.
fn receive(socket: RawFd, timeout_ms: i32) -> io::Result<Option<Reply>> {
    let mut poll = libc::pollfd { fd: socket, events: libc::POLLIN, revents: 0 };
    match unsafe { libc::poll(&mut poll, 1, timeout_ms) } {
        0 => return Ok(None),
        n if n < 0 => return Err(io::Error::last_os_error()),
        _ => (),
    }
    let mut payload = [0u8; 8];
    let mut iov = libc::iovec { iov_base: payload.as_mut_ptr().cast(), iov_len: payload.len() };
    let mut control = Control([0; 64]);
    let mut message: libc::msghdr = unsafe { std::mem::zeroed() };
    message.msg_iov = &mut iov;
    message.msg_iovlen = 1;
    message.msg_control = control.0.as_mut_ptr().cast();
    message.msg_controllen = control.0.len() as _;
    let n = unsafe { libc::recvmsg(socket, &mut message, libc::MSG_DONTWAIT | libc::MSG_CMSG_CLOEXEC) };
    if n < 0 {
        let error = io::Error::last_os_error();
        return if error.kind() == io::ErrorKind::WouldBlock { Ok(None) } else { Err(error) };
    }
    let number = u32::from_le_bytes(payload[..4].try_into().unwrap());
    let status = if n == 8 { i32::from_le_bytes(payload[4..].try_into().unwrap()) } else { libc::EPROTO };
    let mut reply = Reply { number, status, socket: None };
    let mut header = unsafe { libc::CMSG_FIRSTHDR(&message) };
    while !header.is_null() {
        let h = unsafe { &*header };
        if h.cmsg_level == libc::SOL_SOCKET && h.cmsg_type == libc::SCM_RIGHTS {
            let count = (h.cmsg_len as usize - unsafe { libc::CMSG_LEN(0) } as usize) / std::mem::size_of::<RawFd>();
            for i in 0..count {
                let fd = unsafe { std::ptr::read_unaligned((libc::CMSG_DATA(header) as *const RawFd).add(i)) };
                if reply.socket.is_none() { reply.socket = Some(fd) } else { unsafe { libc::close(fd); } }
            }
        }
        header = unsafe { libc::CMSG_NXTHDR(&message, header) };
    }
    Ok(Some(reply))
}
