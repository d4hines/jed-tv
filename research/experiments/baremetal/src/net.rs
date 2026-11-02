//! virtio-net as a smoltcp device, DHCP, and one HTTP GET with a Range header
//! against the replay server, driven from the main loop (no blocking).

use crate::hal::HalImpl;
use crate::{println, timer};
use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefCell;
use smoltcp::iface::{Config, Interface, SocketHandle, SocketSet};
use smoltcp::phy::{Device, DeviceCapabilities, Medium, RxToken, TxToken};
use smoltcp::socket::{dhcpv4, tcp};
use smoltcp::time::Instant;
use smoltcp::wire::{EthernetAddress, HardwareAddress, IpAddress, IpCidr};
use virtio_drivers::device::net::{RxBuffer, VirtIONet};
use virtio_drivers::transport::mmio::MmioTransport;

type Nic = VirtIONet<HalImpl, MmioTransport<'static>, 16>;

pub struct NetDev(RefCell<Nic>);

pub struct Rx<'a>(&'a RefCell<Nic>, RxBuffer);
pub struct Tx<'a>(&'a RefCell<Nic>);

impl Device for NetDev {
    type RxToken<'a> = Rx<'a> where Self: 'a;
    type TxToken<'a> = Tx<'a> where Self: 'a;

    fn receive(&mut self, _: Instant) -> Option<(Rx<'_>, Tx<'_>)> {
        let buf = self.0.borrow_mut().receive().ok()?;
        Some((Rx(&self.0, buf), Tx(&self.0)))
    }
    fn transmit(&mut self, _: Instant) -> Option<Tx<'_>> {
        Some(Tx(&self.0))
    }
    fn capabilities(&self) -> DeviceCapabilities {
        let mut c = DeviceCapabilities::default();
        c.max_transmission_unit = 1514;
        // smoltcp caps the advertised TCP window at max_burst_size * MSS;
        // Some(1) (its docs example) limited us to one segment per RTT
        // (86 KiB/s). QEMU queues frames while our 16 RX buffers are full.
        c.max_burst_size = None;
        c.medium = Medium::Ethernet;
        c
    }
}

impl RxToken for Rx<'_> {
    fn consume<R, F: FnOnce(&[u8]) -> R>(self, f: F) -> R {
        let r = f(self.1.packet());
        self.0.borrow_mut().recycle_rx_buffer(self.1).expect("recycle rx buffer");
        r
    }
}

impl TxToken for Tx<'_> {
    fn consume<R, F: FnOnce(&mut [u8]) -> R>(self, len: usize, f: F) -> R {
        let mut nic = self.0.borrow_mut();
        let mut buf = nic.new_tx_buffer(len);
        let r = f(buf.packet_mut());
        nic.send(buf).expect("virtio-net send");
        r
    }
}

#[derive(PartialEq, Eq, Debug)]
enum State {
    Dhcp,
    Connecting,
    Receiving,
    Done,
}

pub struct Net {
    dev: NetDev,
    iface: Interface,
    sockets: SocketSet<'static>,
    dhcp: SocketHandle,
    tcp: SocketHandle,
    state: State,
    server: (IpAddress, u16),
    request: Vec<u8>,
    sent: bool,
    header: Vec<u8>,
    body_len: usize,
    hash: u64,
    t_start: u64,
    t_connect: u64,
    pub done: bool,
}

fn now() -> Instant {
    Instant::from_micros(timer::micros() as i64)
}

impl Net {
    pub fn new(transport: MmioTransport<'static>, path: &str, range: &str) -> Net {
        let nic = Nic::new(transport, 2048).expect("virtio-net init");
        let mac = nic.mac_address();
        let mut dev = NetDev(RefCell::new(nic));
        let hw = HardwareAddress::Ethernet(EthernetAddress(mac));
        let mut cfg = Config::new(hw);
        cfg.random_seed = timer::ticks();
        let iface = Interface::new(cfg, &mut dev, now());
        let mut sockets = SocketSet::new(Vec::new());
        let dhcp = sockets.add(dhcpv4::Socket::new());
        let tcp = sockets.add(tcp::Socket::new(
            tcp::SocketBuffer::new(vec![0; 256 * 1024]),
            tcp::SocketBuffer::new(vec![0; 4096]),
        ));
        let request = alloc::format!(
            "GET {path} HTTP/1.1\r\nHost: 10.0.2.2:8090\r\nRange: bytes={range}\r\n\
             User-Agent: tvbare/0.1\r\nConnection: close\r\n\r\n"
        );
        println!("net: virtio-net MAC {:02x?}, starting DHCP", mac);
        Net {
            dev, iface, sockets, dhcp, tcp, state: State::Dhcp,
            server: (IpAddress::v4(10, 0, 2, 2), 8090),
            request: request.into_bytes(), sent: false, header: Vec::new(), body_len: 0,
            hash: 0xcbf2_9ce4_8422_2325, t_start: timer::ticks(), t_connect: 0, done: false,
        }
    }

    pub fn ack_interrupt(&mut self) {
        self.dev.0.borrow_mut().ack_interrupt();
    }

    /// One-line progress for the periodic status log.
    pub fn status(&mut self) -> alloc::string::String {
        let s = self.sockets.get_mut::<tcp::Socket>(self.tcp);
        alloc::format!("{:?}/{} body {} B rxq {}", self.state, s.state(), self.body_len, s.recv_queue())
    }

    /// Advance DHCP / TCP / HTTP as far as possible without blocking.
    pub fn poll(&mut self) {
        if self.done {
            return;
        }
        self.iface.poll(now(), &mut self.dev, &mut self.sockets);
        if self.state == State::Dhcp {
            if let Some(dhcpv4::Event::Configured(c)) =
                self.sockets.get_mut::<dhcpv4::Socket>(self.dhcp).poll()
            {
                let ms = timer::ticks_to_us(timer::ticks() - self.t_start) / 1000;
                println!("net: DHCP lease {} router {:?} dns {:?} after {} ms",
                         c.address, c.router, c.dns_servers, ms);
                self.iface.update_ip_addrs(|a| {
                    a.clear();
                    a.push(IpCidr::Ipv4(c.address)).unwrap();
                });
                if let Some(r) = c.router {
                    self.iface.routes_mut().add_default_ipv4_route(r).unwrap();
                }
                let s = self.sockets.get_mut::<tcp::Socket>(self.tcp);
                s.connect(self.iface.context(), self.server, 49152 + (timer::ticks() % 1000) as u16)
                    .expect("tcp connect");
                self.t_connect = timer::ticks();
                self.state = State::Connecting;
                println!("net: connecting to {}:{}", self.server.0, self.server.1);
            }
        }
        if matches!(self.state, State::Connecting | State::Receiving) {
            let s = self.sockets.get_mut::<tcp::Socket>(self.tcp);
            if !self.sent && s.may_send() {
                s.send_slice(&self.request).expect("send request");
                self.sent = true;
                self.state = State::Receiving;
                let ms = timer::ticks_to_us(timer::ticks() - self.t_connect) / 1000;
                println!("net: connected in {} ms, sent {} byte request", ms, self.request.len());
            }
            while s.can_recv() {
                let (header, body_len, hash) = (&mut self.header, &mut self.body_len, &mut self.hash);
                s.recv(|buf| {
                    let mut body: &[u8] = buf;
                    if !header.ends_with(b"\r\n\r\n") {
                        let mut n = 0;
                        while n < body.len() && !header.ends_with(b"\r\n\r\n") {
                            header.push(body[n]);
                            n += 1;
                        }
                        body = &body[n..];
                    }
                    for &b in body {
                        *hash = (*hash ^ b as u64).wrapping_mul(0x100_0000_01b3);
                    }
                    *body_len += body.len();
                    (buf.len(), ())
                })
                .unwrap();
            }
            if self.sent && !s.may_recv() && s.state() != tcp::State::SynSent {
                s.close();
                self.finish();
            }
            if self.state == State::Done {
                self.iface.poll(now(), &mut self.dev, &mut self.sockets);
            }
        }
    }

    fn finish(&mut self) {
        let us = timer::ticks_to_us(timer::ticks() - self.t_connect).max(1);
        let head = core::str::from_utf8(&self.header).unwrap_or("?");
        for line in head.lines().filter(|l| {
            l.starts_with("HTTP/") || l.starts_with("Content-Range") || l.starts_with("Content-Length")
                || l.starts_with("Content-Type")
        }) {
            println!("net:   {}", line);
        }
        println!("net: body {} bytes in {}.{:03} s ({} KiB/s), FNV-1a64 {:016x}",
                 self.body_len, us / 1_000_000, us / 1000 % 1000,
                 self.body_len as u64 * 1_000_000 / 1024 / us, self.hash);
        self.state = State::Done;
        self.done = true;
    }
}
