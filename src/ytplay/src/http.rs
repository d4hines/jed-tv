//! Minimal HTTP/1.1 client over TCP: http:// (the replay server) and https://
//! (real YouTube) via rustls with the pure-Rust RustCrypto provider and
//! compiled-in Mozilla roots.

use std::io::{self, BufRead, BufReader, Read, Write};
use std::sync::{Arc, OnceLock};
use std::time::Duration;

pub struct Url<'a> {
    pub tls: bool,
    pub host: &'a str,
    pub port: u16,
    pub path: &'a str,
}

pub fn parse_url(url: &str) -> io::Result<Url<'_>> {
    let (tls, rest) = if let Some(r) = url.strip_prefix("https://") {
        (true, r)
    } else if let Some(r) = url.strip_prefix("http://") {
        (false, r)
    } else {
        return Err(io::Error::other(format!("unsupported URL: {url}")));
    };
    let (authority, path) = rest.find('/').map_or((rest, "/"), |i| (&rest[..i], &rest[i..]));
    let (host, port) = match authority.rsplit_once(':') {
        Some((h, p)) => (h, p.parse().map_err(io::Error::other)?),
        None => (authority, if tls { 443 } else { 80 }),
    };
    Ok(Url { tls, host, port, path })
}

fn tls_config() -> Arc<rustls::ClientConfig> {
    static CFG: OnceLock<Arc<rustls::ClientConfig>> = OnceLock::new();
    CFG.get_or_init(|| {
        let roots = rustls::RootCertStore { roots: webpki_roots::TLS_SERVER_ROOTS.to_vec() };
        let provider = Arc::new(rustls_rustcrypto::provider()); // pure Rust (RustCrypto)
        let cfg = rustls::ClientConfig::builder_with_provider(provider)
            .with_safe_default_protocol_versions().expect("TLS versions")
            .with_root_certificates(roots)
            .with_no_client_auth();
        Arc::new(cfg)
    }).clone()
}

trait Stream: Read + Write {}
impl<T: Read + Write> Stream for T {}

fn connect(u: &Url) -> io::Result<Box<dyn Stream>> {
    let tcp = if let Some(proxy) = std::env::var("TV_HTTP_PROXY").ok().filter(|_| u.tls) {
        let mut stream = platform::broker::connect_addr(&proxy)?;
        stream.set_read_timeout(Some(Duration::from_secs(20)))?;
        write!(stream,"CONNECT {}:{} HTTP/1.1\r\nHost: {}:{}\r\n\r\n",u.host,u.port,u.host,u.port)?;
        let mut header=Vec::new();
        while !header.ends_with(b"\r\n\r\n") && header.len()<8192 {
            let mut b=[0]; stream.read_exact(&mut b)?; header.push(b[0]);
        }
        if !header.starts_with(b"HTTP/1.1 200 ") {
            return Err(io::Error::other("HTTP tunnel failed"));
        }
        stream
    } else { platform::broker::connect(u.host, u.port)? };
    tcp.set_read_timeout(Some(Duration::from_secs(20)))?;
    tcp.set_nodelay(true)?;
    if !u.tls {
        return Ok(Box::new(tcp));
    }
    let name = rustls::pki_types::ServerName::try_from(u.host.to_owned()).map_err(io::Error::other)?;
    let conn = rustls::ClientConnection::new(tls_config(), name).map_err(io::Error::other)?;
    Ok(Box::new(rustls::StreamOwned::new(conn, tcp)))
}

pub struct Response {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers.iter().find(|(k, _)| k.eq_ignore_ascii_case(name)).map(|(_, v)| v.as_str())
    }
}

/// One request per connection (Connection: close). Handles Content-Length and chunked.
pub fn request(method: &str, url: &str, headers: &[(&str, &str)], body: &[u8]) -> io::Result<Response> {
    let u = parse_url(url)?;
    let mut s = connect(&u)?;
    // Host must carry the port when it isn't the default, or virtual hosts and
    // servers that build absolute URLs from it (our replay server) go wrong.
    let default_port = if u.tls { 443 } else { 80 };
    let host = if u.port == default_port { u.host.to_owned() } else { format!("{}:{}", u.host, u.port) };
    let mut req = format!("{method} {} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n", u.path);
    for (k, v) in headers {
        req.push_str(&format!("{k}: {v}\r\n"));
    }
    if !body.is_empty() || method == "POST" {
        req.push_str(&format!("Content-Length: {}\r\n", body.len()));
    }
    req.push_str("\r\n");
    s.write_all(req.as_bytes())?;
    s.write_all(body)?;

    let mut r = BufReader::new(s);
    let mut line = String::new();
    r.read_line(&mut line)?;
    let status: u16 = line.split_whitespace().nth(1).and_then(|c| c.parse().ok())
        .ok_or_else(|| io::Error::other(format!("bad status line: {line:?}")))?;
    let mut hdrs = Vec::new();
    loop {
        line.clear();
        r.read_line(&mut line)?;
        let l = line.trim_end();
        if l.is_empty() {
            break;
        }
        if let Some((k, v)) = l.split_once(':') {
            hdrs.push((k.trim().to_owned(), v.trim().to_owned()));
        }
    }
    let mut resp = Response { status, headers: hdrs, body: Vec::new() };
    if resp.header("transfer-encoding").is_some_and(|v| v.eq_ignore_ascii_case("chunked")) {
        loop {
            line.clear();
            r.read_line(&mut line)?;
            let n = usize::from_str_radix(line.trim().split(';').next().unwrap_or("0"), 16)
                .map_err(io::Error::other)?;
            if n == 0 {
                break;
            }
            let start = resp.body.len();
            resp.body.resize(start + n, 0);
            r.read_exact(&mut resp.body[start..])?;
            line.clear();
            r.read_line(&mut line)?; // CRLF after each chunk
        }
    } else if let Some(n) = resp.header("content-length").and_then(|v| v.parse::<usize>().ok()) {
        resp.body.resize(n, 0);
        r.read_exact(&mut resp.body)?;
    } else {
        r.read_to_end(&mut resp.body)?;
    }
    Ok(resp)
}
