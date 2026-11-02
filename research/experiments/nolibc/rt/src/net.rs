//! Sockets, and a numeric-only getaddrinfo: no DNS resolver yet (ytplay will
//! need one for live YouTube; see research/nolibc-brief.md).

use crate::sys::{ret, ret_int};
use libc::{c_char, c_int, c_void, size_t, sockaddr, socklen_t, ssize_t};

#[no_mangle]
pub unsafe extern "C" fn socket(domain: c_int, ty: c_int, proto: c_int) -> c_int {
    ret_int(sc!(libc::SYS_socket, domain, ty, proto))
}

#[no_mangle]
pub unsafe extern "C" fn bind(fd: c_int, a: *const sockaddr, len: socklen_t) -> c_int {
    ret_int(sc!(libc::SYS_bind, fd, a, len))
}

#[no_mangle]
pub unsafe extern "C" fn listen(fd: c_int, backlog: c_int) -> c_int {
    ret_int(sc!(libc::SYS_listen, fd, backlog))
}

#[no_mangle]
pub unsafe extern "C" fn accept4(fd: c_int, a: *mut sockaddr, len: *mut socklen_t, flags: c_int) -> c_int {
    ret_int(sc!(libc::SYS_accept4, fd, a, len, flags))
}

#[no_mangle]
pub unsafe extern "C" fn connect(fd: c_int, a: *const sockaddr, len: socklen_t) -> c_int {
    ret_int(sc!(libc::SYS_connect, fd, a, len))
}

#[no_mangle]
pub unsafe extern "C" fn sendto(fd: c_int, buf: *const c_void, n: size_t, flags: c_int, a: *const sockaddr,
                                len: socklen_t) -> ssize_t {
    ret(sc!(libc::SYS_sendto, fd, buf, n, flags, a, len)) as ssize_t
}

#[no_mangle]
pub unsafe extern "C" fn send(fd: c_int, buf: *const c_void, n: size_t, flags: c_int) -> ssize_t {
    sendto(fd, buf, n, flags, core::ptr::null(), 0)
}

#[no_mangle]
pub unsafe extern "C" fn recvfrom(fd: c_int, buf: *mut c_void, n: size_t, flags: c_int, a: *mut sockaddr,
                                  len: *mut socklen_t) -> ssize_t {
    ret(sc!(libc::SYS_recvfrom, fd, buf, n, flags, a, len)) as ssize_t
}

#[no_mangle]
pub unsafe extern "C" fn recv(fd: c_int, buf: *mut c_void, n: size_t, flags: c_int) -> ssize_t {
    recvfrom(fd, buf, n, flags, core::ptr::null_mut(), core::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn setsockopt(fd: c_int, level: c_int, name: c_int, v: *const c_void, len: socklen_t)
    -> c_int {
    ret_int(sc!(libc::SYS_setsockopt, fd, level, name, v, len))
}

#[no_mangle]
pub unsafe extern "C" fn getsockopt(fd: c_int, level: c_int, name: c_int, v: *mut c_void, len: *mut socklen_t)
    -> c_int {
    ret_int(sc!(libc::SYS_getsockopt, fd, level, name, v, len))
}

#[no_mangle]
pub unsafe extern "C" fn getsockname(fd: c_int, a: *mut sockaddr, len: *mut socklen_t) -> c_int {
    ret_int(sc!(libc::SYS_getsockname, fd, a, len))
}

#[no_mangle]
pub unsafe extern "C" fn getpeername(fd: c_int, a: *mut sockaddr, len: *mut socklen_t) -> c_int {
    ret_int(sc!(libc::SYS_getpeername, fd, a, len))
}

#[no_mangle]
pub unsafe extern "C" fn shutdown(fd: c_int, how: c_int) -> c_int {
    ret_int(sc!(libc::SYS_shutdown, fd, how))
}

fn parse_ipv4(s: &[u8]) -> Option<[u8; 4]> {
    let mut out = [0u8; 4];
    let mut parts = s.split(|&c| c == b'.');
    for o in out.iter_mut() {
        let p = parts.next()?;
        if p.is_empty() || p.len() > 3 || !p.iter().all(u8::is_ascii_digit) {
            return None;
        }
        let v = p.iter().fold(0u32, |a, &d| a * 10 + (d - b'0') as u32);
        if v > 255 {
            return None;
        }
        *o = v as u8;
    }
    parts.next().is_none().then_some(out)
}

#[repr(C)]
struct AiBlock {
    ai: libc::addrinfo,
    sin: libc::sockaddr_in,
}

/// Numeric IPv4 hosts and "localhost" only. std parses IP literals itself and
/// only calls this for names, so in practice this is "no DNS" (EAI_NONAME).
#[no_mangle]
pub unsafe extern "C" fn getaddrinfo(node: *const c_char, service: *const c_char, hints: *const libc::addrinfo,
                                     res: *mut *mut libc::addrinfo) -> c_int {
    if node.is_null() {
        return libc::EAI_NONAME;
    }
    let name = core::ffi::CStr::from_ptr(node).to_bytes();
    let ip = if name == b"localhost" { Some([127, 0, 0, 1]) } else { parse_ipv4(name) };
    let Some(ip) = ip else { return libc::EAI_NONAME };
    let mut port = 0u16;
    if !service.is_null() {
        let s = core::ffi::CStr::from_ptr(service).to_bytes();
        if s.is_empty() || !s.iter().all(u8::is_ascii_digit) || s.len() > 5 {
            return libc::EAI_SERVICE;
        }
        port = s.iter().fold(0u32, |a, &d| a * 10 + (d - b'0') as u32) as u16;
    }
    let (socktype, proto) = if hints.is_null() { (0, 0) } else { ((*hints).ai_socktype, (*hints).ai_protocol) };
    if !hints.is_null() && (*hints).ai_family != libc::AF_UNSPEC && (*hints).ai_family != libc::AF_INET {
        return libc::EAI_FAMILY;
    }
    let b = crate::malloc::calloc(1, core::mem::size_of::<AiBlock>()) as *mut AiBlock;
    if b.is_null() {
        return libc::EAI_MEMORY;
    }
    (*b).sin.sin_family = libc::AF_INET as libc::sa_family_t;
    (*b).sin.sin_port = port.to_be();
    (*b).sin.sin_addr.s_addr = u32::from_ne_bytes(ip);
    (*b).ai.ai_family = libc::AF_INET;
    (*b).ai.ai_socktype = socktype;
    (*b).ai.ai_protocol = proto;
    (*b).ai.ai_addrlen = core::mem::size_of::<libc::sockaddr_in>() as socklen_t;
    (*b).ai.ai_addr = &mut (*b).sin as *mut libc::sockaddr_in as *mut sockaddr;
    *res = b as *mut libc::addrinfo;
    0
}

#[no_mangle]
pub unsafe extern "C" fn freeaddrinfo(mut ai: *mut libc::addrinfo) {
    while !ai.is_null() {
        let next = (*ai).ai_next;
        crate::malloc::free(ai as *mut c_void);
        ai = next;
    }
}

#[no_mangle]
pub extern "C" fn gai_strerror(code: c_int) -> *const c_char {
    let s: &'static core::ffi::CStr = match code {
        libc::EAI_NONAME => c"Name does not resolve (nolibc-rt: no DNS resolver)",
        libc::EAI_SERVICE => c"Unrecognized service",
        libc::EAI_FAMILY => c"Address family not supported",
        libc::EAI_MEMORY => c"Out of memory",
        _ => c"Unknown error",
    };
    s.as_ptr()
}
