//! Standalone candidate transport for the owned V109 TEE RPC subset.
use g06_tee_rpc::{Allocator, Param, Provider, TaStore, BAD_PARAMETERS};
use std::io;
#[cfg(all(target_os = "linux", target_arch = "arm"))]
pub mod linux;
pub mod pins;

pub const MAX_PARAMS: usize = 31; // exact V109 TEE_MAX_ARG_SIZE=1024, not newer 4096.
pub const META: u64 = 0x100;
pub const PACKET_BYTES: usize = 8 + 32 * MAX_PARAMS;
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Packet {
    pub word: u32,
    pub count: u32,
    pub params: [Param; MAX_PARAMS],
}
impl Packet {
    pub fn receive_buffer() -> Self {
        let mut params = [Param::default(); MAX_PARAMS];
        params[0].attr = META;
        Self {
            word: 0,
            count: MAX_PARAMS as u32,
            params,
        }
    }
    pub fn byte_len(&self) -> io::Result<usize> {
        if self.count as usize > MAX_PARAMS {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "TEE response count exceeds fixed capacity",
            ));
        }
        Ok(8 + 32 * self.count as usize)
    }
}
pub trait Transport {
    fn receive(&mut self, packet: &mut Packet) -> io::Result<()>;
    fn send(&mut self, packet: &Packet) -> io::Result<()>;
}
pub trait Handler {
    fn handle(&mut self, function: u32, params: &mut [Param]) -> u32;
}
impl<A: Allocator, S: TaStore> Handler for Provider<A, S> {
    fn handle(&mut self, function: u32, params: &mut [Param]) -> u32 {
        // The library mirrors the stock null-output query rule, but this
        // candidate must not treat an external, unimported nonempty backing
        // buffer as if it had received the TA bytes.
        if function == g06_tee_rpc::LOAD_TA && params.len() == 2 {
            let p = params[1];
            if !self.owns_region(p.c) && (p.a != 0 || p.b != 0) {
                return BAD_PARAMETERS;
            }
        }
        Provider::handle(self, function, params)
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct Served {
    pub function: u32,
    pub meta_count: usize,
    pub logical_count: usize,
    pub status: u32,
}
pub fn serve_one<T: Transport, H: Handler>(
    transport: &mut T,
    handler: &mut H,
) -> io::Result<Served> {
    let mut packet = Packet::receive_buffer();
    transport.receive(&mut packet)?;
    packet.byte_len()?;
    let function = packet.word;
    let n = packet.count as usize;
    let meta = packet.params[..n]
        .iter()
        .take_while(|p| p.attr & META != 0)
        .count();
    let misplaced = packet.params[meta..n].iter().any(|p| p.attr & META != 0);
    let status = if misplaced {
        BAD_PARAMETERS
    } else {
        handler.handle(function, &mut packet.params[meta..n])
    };
    packet.word = status;
    transport.send(&packet)?;
    Ok(Served {
        function,
        meta_count: meta,
        logical_count: n - meta,
        status,
    })
}

pub const SERIAL: &str = "androidboot.serialno=4C0490290FA05A0C0";
pub fn validate_identity(uid: u32, cmdline: &str, release: &str) -> io::Result<()> {
    let serial: Vec<_> = cmdline
        .split_whitespace()
        .filter(|s| s.starts_with("androidboot.serialno="))
        .collect();
    let tee: Vec<_> = cmdline
        .split_whitespace()
        .filter(|s| s.starts_with("tee_mode="))
        .collect();
    let kernel = release.trim();
    if uid != 0
        || serial != [SERIAL]
        || tee != ["tee_mode=optee"]
        || !(kernel == "4.19.116"
            || kernel.starts_with("4.19.116+")
            || kernel.starts_with("4.19.116-"))
    {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "requires root on the original G06 serial, stock4.19.116 kernel and OP-TEE mode",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Io {
        input: Packet,
        output: Option<Packet>,
        recv_fail: bool,
        send_fail: bool,
    }
    impl Transport for Io {
        fn receive(&mut self, p: &mut Packet) -> io::Result<()> {
            assert_eq!(p.count, 31);
            assert_eq!(p.params[0].attr, META);
            if self.recv_fail {
                return Err(io::Error::other("recv"));
            }
            *p = self.input.clone();
            Ok(())
        }
        fn send(&mut self, p: &Packet) -> io::Result<()> {
            if self.send_fail {
                return Err(io::Error::other("send"));
            }
            self.output = Some(p.clone());
            Ok(())
        }
    }
    struct Engine {
        calls: usize,
        result: u32,
    }
    impl Handler for Engine {
        fn handle(&mut self, f: u32, p: &mut [Param]) -> u32 {
            self.calls += 1;
            assert_eq!(f, 6);
            if !p.is_empty() {
                p[0].c = 42
            }
            self.result
        }
    }
    fn input() -> Io {
        let mut p = Packet::receive_buffer();
        p.word = 6;
        p.count = 2;
        p.params[0] = Param {
            attr: META | 3,
            a: 77,
            b: 88,
            c: 99,
        };
        p.params[1] = Param {
            attr: 1,
            a: 0,
            b: 4096,
            c: 0,
        };
        Io {
            input: p,
            output: None,
            recv_fail: false,
            send_fail: false,
        }
    }
    struct Buffer(Vec<u8>);
    impl g06_tee_rpc::Region for Buffer {
        fn id(&self) -> i32 {
            7
        }
        fn len(&self) -> usize {
            self.0.len()
        }
        fn read(&self, offset: usize, out: &mut [u8]) -> io::Result<()> {
            let end = offset
                .checked_add(out.len())
                .ok_or_else(|| io::Error::other("offset overflow"))?;
            let source = self
                .0
                .get(offset..end)
                .ok_or_else(|| io::Error::other("outside region"))?;
            out.copy_from_slice(source);
            Ok(())
        }
        fn clear(&mut self) -> io::Result<()> {
            self.0.fill(0);
            Ok(())
        }
        fn write(&mut self, o: usize, b: &[u8]) -> io::Result<()> {
            self.0[o..o + b.len()].copy_from_slice(b);
            Ok(())
        }
    }
    struct Alloc;
    impl Allocator for Alloc {
        type Region = Buffer;
        fn allocate(&mut self, n: usize) -> io::Result<Buffer> {
            Ok(Buffer(vec![0; n]))
        }
    }
    struct Store;
    impl TaStore for Store {
        fn load(
            &mut self,
            _: g06_tee_rpc::Uuid,
            _: usize,
        ) -> Result<Vec<u8>, g06_tee_rpc::LoadError> {
            Ok(vec![1, 2, 3])
        }
    }
    #[test]
    fn unknown_nonempty_backing_never_gets_query_success() {
        let mut p = Provider::new(Alloc, Store, g06_tee_rpc::Limits::default()).unwrap();
        let mut args = [
            Param {
                attr: 1,
                ..Param::default()
            },
            Param {
                attr: 6,
                b: 128,
                c: 9,
                ..Param::default()
            },
        ];
        assert_eq!(
            Handler::handle(&mut p, g06_tee_rpc::LOAD_TA, &mut args),
            BAD_PARAMETERS
        );
        args[1].b = 0;
        assert_eq!(Handler::handle(&mut p, g06_tee_rpc::LOAD_TA, &mut args), 0);
        assert_eq!(args[1].b, 3);
    }
    #[test]
    fn owned_backing_still_supports_copy() {
        let mut p = Provider::new(Alloc, Store, g06_tee_rpc::Limits::default()).unwrap();
        let mut alloc = [Param {
            attr: 1,
            b: 128,
            ..Param::default()
        }];
        assert_eq!(
            Handler::handle(&mut p, g06_tee_rpc::SHM_ALLOC, &mut alloc),
            0
        );
        let mut args = [
            Param {
                attr: 1,
                ..Param::default()
            },
            Param {
                attr: 6,
                b: 128,
                c: alloc[0].c,
                ..Param::default()
            },
        ];
        assert_eq!(Handler::handle(&mut p, g06_tee_rpc::LOAD_TA, &mut args), 0);
        assert_eq!(args[1].b, 3);
    }

    #[test]
    fn exact_framing() {
        assert_eq!(std::mem::size_of::<Packet>(), 1000);
        assert_eq!(PACKET_BYTES, 1000);
        assert_eq!(std::mem::offset_of!(Packet, params), 8);
    }
    #[test]
    fn metadata_is_stripped_and_returned_unchanged() {
        let mut io = input();
        let original = io.input.params[0];
        let mut h = Engine {
            calls: 0,
            result: 0,
        };
        let out = serve_one(&mut io, &mut h).unwrap();
        assert_eq!(out.meta_count, 1);
        assert_eq!(h.calls, 1);
        let p = io.output.unwrap();
        assert_eq!(p.params[0], original);
        assert_eq!(p.params[1].c, 42);
        assert_eq!(p.count, 2);
        assert_eq!(p.word, 0);
    }
    #[test]
    fn every_leading_meta_is_preserved() {
        let mut io = input();
        io.input.count = 3;
        io.input.params[1] = Param {
            attr: META | 1,
            a: 1,
            b: 2,
            c: 3,
        };
        io.input.params[2] = Param {
            attr: 1,
            b: 9,
            ..Param::default()
        };
        let before = io.input.params[..2].to_vec();
        let mut h = Engine {
            calls: 0,
            result: 0,
        };
        assert_eq!(serve_one(&mut io, &mut h).unwrap().meta_count, 2);
        assert_eq!(&io.output.unwrap().params[..2], before);
    }
    #[test]
    fn nonleading_meta_gets_error_reply_without_handler() {
        let mut io = input();
        io.input.count = 3;
        io.input.params[2].attr = META;
        let mut h = Engine {
            calls: 0,
            result: 0,
        };
        assert_eq!(serve_one(&mut io, &mut h).unwrap().status, BAD_PARAMETERS);
        assert_eq!(h.calls, 0);
        assert_eq!(io.output.unwrap().count, 3);
    }
    #[test]
    fn handler_error_is_sent_verbatim() {
        let mut io = input();
        let mut h = Engine {
            calls: 0,
            result: 0xffff000a,
        };
        serve_one(&mut io, &mut h).unwrap();
        assert_eq!(io.output.unwrap().word, 0xffff000a);
    }
    #[test]
    fn transport_errors_stop_without_fake_reply() {
        let mut io = input();
        io.recv_fail = true;
        let mut h = Engine {
            calls: 0,
            result: 0,
        };
        assert!(serve_one(&mut io, &mut h).is_err());
        assert_eq!(h.calls, 0);
        assert!(io.output.is_none());
        io.recv_fail = false;
        io.send_fail = true;
        assert!(serve_one(&mut io, &mut h).is_err());
        assert_eq!(h.calls, 1);
    }
    #[test]
    fn oversize_kernel_count_is_fatal_without_send() {
        let mut io = input();
        io.input.count = 32;
        let mut h = Engine {
            calls: 0,
            result: 0,
        };
        assert!(serve_one(&mut io, &mut h).is_err());
        assert!(io.output.is_none());
        assert_eq!(h.calls, 0);
    }
    #[test]
    fn zero_or_all_meta_frames_have_empty_logical_slice() {
        for count in [0, 1] {
            let mut io = input();
            io.input.count = count;
            let mut h = Engine {
                calls: 0,
                result: BAD_PARAMETERS,
            };
            assert_eq!(serve_one(&mut io, &mut h).unwrap().logical_count, 0);
            assert_eq!(h.calls, 1);
        }
    }
    #[test]
    fn identity_must_match_exact_single_serial_and_mode() {
        assert!(validate_identity(0, &format!("{SERIAL} tee_mode=optee"), "4.19.116+").is_ok());
        for (uid, c, k) in [
            (1, format!("{SERIAL} tee_mode=optee"), "4.19.116+"),
            (0, format!("{SERIAL} tee_mode=notee"), "4.19.116+"),
            (0, format!("{SERIAL} {SERIAL} tee_mode=optee"), "4.19.116+"),
            (0, format!("{SERIAL} tee_mode=optee"), "4.19.1166"),
            (
                0,
                "androidboot.serialno=OTHER tee_mode=optee".into(),
                "4.19.116+",
            ),
        ] {
            assert!(validate_identity(uid, &c, k).is_err());
        }
    }
}
