use crate::{
    model::{phase, Births, Phase},
    wire::{self, Envelope, Memory, Snapshot},
};
use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::{c_char, c_int, c_long, c_void, CString},
    fs::{File, OpenOptions},
    io::{self, Read, Write},
    os::unix::{
        ffi::OsStrExt,
        fs::{MetadataExt, OpenOptionsExt},
    },
    path::Path,
};

const TRACEME: c_int = 0;
const PEEKDATA: c_int = 2;
const GETREGS: c_int = 12;
const SYSCALL: c_int = 24;
const SETOPTIONS: c_int = 0x4200;
const GETEVENTMSG: c_int = 0x4201;
const GETSIGINFO: c_int = 0x4202;
const OPTIONS: usize = 1 | 2 | 4 | 8 | 16 | 0x0010_0000; // TRACESYSGOOD,FORK,VFORK,CLONE,EXEC,EXITKILL
const WALL: c_int = 0x4000_0000;
const SIGSTOP: c_int = 19;
const SIGTRAP: c_int = 5;
const SIGKILL: c_int = 9;
const NR_IOCTL: u32 = 54;
const NR_CLONE: u32 = 120;
const CLONE_UNTRACED: u32 = 0x0080_0000;
const MAX_LOG: u64 = 64 * 1024 * 1024;
extern "C" {
    fn ptrace(request: c_int, pid: c_int, address: *mut c_void, data: *mut c_void) -> c_long;
    fn fork() -> c_int;
    fn getpid() -> c_int;
    fn getppid() -> c_int;
    fn prctl(option: c_int, ...) -> c_int;
    fn raise(signal: c_int) -> c_int;
    fn kill(pid: c_int, signal: c_int) -> c_int;
    fn execv(path: *const c_char, argv: *const *const c_char) -> c_int;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
    fn _exit(status: c_int) -> !;
    fn __errno_location() -> *mut c_int;
}
fn fail(s: &str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, s)
}
fn trace(request: c_int, pid: c_int, address: usize, data: usize) -> io::Result<c_long> {
    unsafe {
        *__errno_location() = 0;
        let ret = ptrace(request, pid, address as *mut _, data as *mut _);
        if ret == -1 && *__errno_location() != 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(ret)
        }
    }
}
fn resume(pid: c_int, signal: c_int) -> io::Result<()> {
    trace(SYSCALL, pid, 0, signal as usize).map(|_| ())
}
fn wait(pid: c_int, options: c_int) -> io::Result<(c_int, c_int)> {
    loop {
        let mut status = 0;
        let n = unsafe { waitpid(pid, &mut status, options) };
        if n < 0 {
            let e = io::Error::last_os_error();
            if e.kind() == io::ErrorKind::Interrupted {
                continue;
            }
            return Err(e);
        }
        return Ok((n, status));
    }
}
fn regs(pid: c_int) -> io::Result<[u32; 18]> {
    let mut r = [0; 18];
    trace(GETREGS, pid, 0, r.as_mut_ptr() as usize)?;
    Ok(r)
}
fn eventmsg(pid: c_int) -> io::Result<u32> {
    let mut n = 0u32;
    trace(GETEVENTMSG, pid, 0, &mut n as *mut _ as usize)?;
    Ok(n)
}
fn quote(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c < ' ' => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
struct Log {
    file: File,
    bytes: u64,
}
impl Log {
    fn line(&mut self, s: &str) -> io::Result<()> {
        let n = s.len() as u64 + 1;
        if self.bytes + n > MAX_LOG {
            return Err(fail(
                "64 MiB log bound reached; refusing partial unreported tracing",
            ));
        }
        self.file.write_all(s.as_bytes())?;
        self.file.write_all(b"\n")?;
        self.file.flush()?;
        self.bytes += n;
        Ok(())
    }
    fn rpc(
        &mut self,
        tid: i32,
        seq: u64,
        stage: &str,
        p: &Pending,
        ret: Option<i32>,
        s: Option<&Snapshot>,
        error: Option<String>,
    ) -> io::Result<()> {
        let mut params = String::new();
        if let Some(s) = s {
            for (i, v) in s.params.iter().enumerate() {
                if i > 0 {
                    params.push(',')
                }
                params.push_str(&format!(
                    "{{\"attr\":{},\"a\":{},\"b\":{},\"c\":{}}}",
                    v.attr, v.a, v.b, v.c
                ));
            }
        }
        self.line(&format!("{{\"event\":\"ioctl\",\"tid\":{tid},\"seq\":{seq},\"stage\":{},\"request\":{},\"fd\":{},\"fd_path\":{},\"return\":{},\"header_word\":{},\"num_params\":{},\"params\":[{}],\"error\":{}}}",quote(stage),quote(if p.request==wire::RECV{"recv"}else{"send"}),p.fd,quote(&p.fd_path),ret.map_or("null".into(),|v|v.to_string()),s.map_or("null".into(),|s|s.word.to_string()),s.map_or("null".into(),|s|s.count.to_string()),params,error.map_or("null".into(),|e|quote(&e))))
    }
}
struct Remote {
    pid: i32,
}
impl Memory for Remote {
    fn read(&mut self, address: u32, length: usize) -> io::Result<Vec<u8>> {
        wire::range(address as u64, length as u64)?;
        // Always fetch aligned ARM words; never use target lengths as allocations.
        let mut out = Vec::with_capacity(length);
        let end = address as u64 + length as u64;
        let mut pos = (address & !3) as u64;
        while pos < end {
            let word = trace(PEEKDATA, self.pid, pos as usize, 0)? as u32;
            let b = word.to_le_bytes();
            let start = (address as u64).saturating_sub(pos).min(4) as usize;
            let finish = (end - pos).min(4) as usize;
            out.extend_from_slice(&b[start..finish]);
            pos += 4;
        }
        Ok(out)
    }
}
struct Pending {
    seq: u64,
    request: u32,
    fd: u32,
    fd_path: String,
    envelope: Option<Envelope>,
    capacity: Option<u32>,
}
struct Guard {
    armed: bool,
    pids: BTreeSet<i32>,
}
impl Drop for Guard {
    fn drop(&mut self) {
        if self.armed {
            for &p in &self.pids {
                unsafe {
                    kill(p, SIGKILL);
                }
            }
        }
    }
}

/// Launch only an absolute, ARM32 ELF path. No shell, PATH lookup or late attach.
pub fn run(log_path: &Path, args: &[std::ffi::OsString]) -> io::Result<i32> {
    let program = Path::new(&args[0]);
    if !program.is_absolute() {
        return Err(fail("program must be an absolute ARM32 ELF path"));
    }
    if std::fs::metadata(program)?.mode() & 0o6000 != 0 {
        return Err(fail(
            "setuid/setgid targets cannot preserve exec semantics under ptrace",
        ));
    }
    let mut header = [0u8; 20];
    File::open(program)?.read_exact(&mut header)?;
    if &header[..6] != b"\x7fELF\x01\x01" || u16::from_le_bytes([header[18], header[19]]) != 40 {
        return Err(fail("program is not a little-endian ARM32 ELF"));
    }
    let cargs: Vec<CString> = args
        .iter()
        .map(|a| CString::new(a.as_os_str().as_bytes()).map_err(|_| fail("NUL in argument")))
        .collect::<io::Result<_>>()?;
    let mut pointers: Vec<*const c_char> = cargs.iter().map(|s| s.as_ptr()).collect();
    pointers.push(std::ptr::null());
    let mut log = Log {
        file: OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(log_path)?,
        bytes: 0,
    };
    log.line("{\"event\":\"start\",\"arch\":\"armv7\",\"reads\":\"metadata_only\",\"max_params\":64,\"max_log_bytes\":67108864}")?;
    let parent_pid = unsafe { getpid() };
    let root = unsafe { fork() };
    if root < 0 {
        return Err(io::Error::last_os_error());
    }
    if root == 0 {
        unsafe {
            // Cover tracer death before SETOPTIONS has installed EXITKILL.
            // The parent check closes the fork-to-prctl race.
            if prctl(1, SIGKILL as usize, 0usize, 0usize, 0usize) != 0 || getppid() != parent_pid {
                _exit(126);
            }
            if ptrace(TRACEME, 0, std::ptr::null_mut(), std::ptr::null_mut()) < 0 {
                _exit(126)
            }
            if raise(SIGSTOP) != 0 {
                _exit(126)
            }
            execv(cargs[0].as_ptr(), pointers.as_ptr());
            _exit(127)
        }
    }
    let mut guard = Guard {
        armed: true,
        pids: BTreeSet::from([root]),
    };
    let (_, status) = wait(root, 0)?;
    if status & 0xff != 0x7f || (status >> 8) & 0xff != SIGSTOP {
        return Err(fail("child did not reach initial TRACEME stop"));
    }
    trace(SETOPTIONS, root, 0, OPTIONS)?;
    let mut births = Births::default();
    births.root(root);
    let mut pending: BTreeMap<i32, Pending> = BTreeMap::new();
    let mut seq = 0;
    let mut root_result = None;
    resume(root, 0)?;
    while !births.empty() {
        let (tid, status) = wait(-1, WALL)?;
        if status & 0x7f == 0 || (status & 0x7f) != 0x7f {
            let code = if status & 0x7f == 0 {
                (status >> 8) & 0xff
            } else {
                128 + (status & 0x7f)
            };
            if tid == root {
                root_result = Some(code)
            }
            log.line(&format!(
                "{{\"event\":\"exit\",\"tid\":{tid},\"status\":{code}}}"
            ))?;
            pending.remove(&tid);
            births.exit(tid);
            guard.pids.remove(&tid);
            continue;
        }
        let signal = (status >> 8) & 0xff;
        let event = (status as u32) >> 16;
        if !births.known(tid) {
            if signal == SIGSTOP && event == 0 {
                births.initial_stop(tid)?;
                guard.pids.insert(tid);
                continue;
            }
            // A nonleader exec assumes the old leader TID. That leader may
            // already have produced its exit status; authenticate via the
            // former executing TID returned by GETEVENTMSG.
            let renamed_exec =
                signal == SIGTRAP && event == 4 && births.known(eventmsg(tid)? as i32);
            if !renamed_exec {
                return Err(fail(
                    "unannounced ptrace task stopped without initial SIGSTOP",
                ));
            }
        }
        if event != 0 {
            if signal != SIGTRAP {
                return Err(fail("ptrace event without SIGTRAP"));
            }
            match event {
                1 | 2 | 3 => {
                    let child = eventmsg(tid)? as i32;
                    if child <= 0 {
                        return Err(fail("invalid child event PID"));
                    }
                    guard.pids.insert(child);
                    let held = births.announce(child)?;
                    log.line(&format!(
                        "{{\"event\":\"child\",\"parent\":{tid},\"tid\":{child},\"kind\":{event}}}"
                    ))?;
                    if held {
                        resume(child, 0)?;
                    }
                }
                4 => {
                    let old = eventmsg(tid)? as i32;
                    births.exec(tid, old);
                    pending.remove(&old);
                    pending.remove(&tid);
                    guard.pids.remove(&old);
                    guard.pids.insert(tid);
                    log.line(&format!(
                        "{{\"event\":\"exec\",\"tid\":{tid},\"old_tid\":{old}}}"
                    ))?;
                }
                _ => return Err(fail("unsupported ptrace event")),
            }
            resume(tid, 0)?;
            continue;
        }
        if signal == (SIGTRAP | 0x80) {
            let r = regs(tid)?;
            match phase(r[12])? {
                Phase::Entry => {
                    if pending.contains_key(&tid) {
                        return Err(fail("new syscall entry before pending ioctl exit"));
                    }
                    if r[7] == NR_CLONE && r[0] & CLONE_UNTRACED != 0 {
                        return Err(fail("CLONE_UNTRACED would escape observation"));
                    }
                    if r[7] == NR_IOCTL && (r[1] == wire::RECV || r[1] == wire::SEND) {
                        seq += 1;
                        let mut p = Pending {
                            seq,
                            request: r[1],
                            fd: r[0],
                            fd_path: std::fs::read_link(format!("/proc/{tid}/fd/{}", r[0]))
                                .map(|p| p.to_string_lossy().into_owned())
                                .unwrap_or_default(),
                            envelope: None,
                            capacity: None,
                        };
                        let mut memory = Remote { pid: tid };
                        let snap = wire::envelope(&mut memory, r[2]).and_then(|e| {
                            p.envelope = Some(e);
                            wire::snapshot(&mut memory, e, p.request == wire::SEND, None)
                        });
                        match snap {
                            Ok(s) => {
                                p.capacity = Some(s.count);
                                log.rpc(tid, seq, "entry", &p, None, Some(&s), None)?
                            }
                            Err(e) => {
                                log.rpc(tid, seq, "entry", &p, None, None, Some(e.to_string()))?
                            }
                        }
                        pending.insert(tid, p);
                    }
                }
                Phase::Exit => {
                    if let Some(p) = pending.remove(&tid) {
                        let ret = r[0] as i32;
                        if ret >= 0 {
                            let snapshot = p
                                .envelope
                                .ok_or_else(|| fail("entry envelope unavailable"))
                                .and_then(|e| {
                                    wire::snapshot(&mut Remote { pid: tid }, e, true, p.capacity)
                                });
                            match snapshot {
                                Ok(s) => {
                                    log.rpc(tid, p.seq, "exit", &p, Some(ret), Some(&s), None)?;
                                    if p.request == wire::RECV && matches!(s.word, 1 | 100 | 101) {
                                        let header = (|| {
                                            let maps = std::fs::read_to_string(format!(
                                                "/proc/{tid}/maps"
                                            ))?;
                                            let mut memory = Remote { pid: tid };
                                            let head = crate::headers::profile(&mut memory, &maps)?;
                                            crate::headers::capture(&mut memory, head, &s)
                                        })();
                                        let detail = match header {
                                            Ok(Some(crate::headers::Header::Rpmb { command, device, blocks, frame_type })) => format!("\"kind\":\"rpmb\",\"command\":{command},\"device\":{device},\"blocks\":{blocks},\"frame_type\":{}",frame_type.map_or("null".into(),|n|n.to_string())),
                                            Ok(Some(crate::headers::Header::Object { uuid, offset, file_size, bytes_read })) => format!("\"kind\":\"object\",\"uuid\":{},\"offset\":{offset},\"file_size\":{file_size},\"bytes_read\":{bytes_read}", quote(&uuid)),
                                            Ok(None) => "\"kind\":\"none\"".into(),
                                            Err(error) => format!("\"unavailable\":{}", quote(&error.to_string())),
                                        };
                                        log.line(&format!("{{\"event\":\"rpc_header\",\"tid\":{tid},\"seq\":{},\"function\":{},{detail}}}",p.seq,s.word))?;
                                    }
                                }
                                Err(e) => log.rpc(
                                    tid,
                                    p.seq,
                                    "exit",
                                    &p,
                                    Some(ret),
                                    None,
                                    Some(e.to_string()),
                                )?,
                            }
                        } else {
                            log.rpc(tid, p.seq, "exit", &p, Some(ret), None, None)?
                        }
                    }
                }
            }
            resume(tid, 0)?;
            continue;
        }
        if signal == SIGSTOP && births.initial_stop(tid)? {
            resume(tid, 0)?;
            continue;
        }
        let mut info = [0u64; 16]; // siginfo_t is128bytes on ARM; aligned buffer.
        if let Err(e) = trace(GETSIGINFO, tid, 0, info.as_mut_ptr() as usize) {
            if e.raw_os_error() == Some(22) {
                return Err(fail(
                    "job-control group stop: TRACEME cannot transparently LISTEN; refusing",
                ));
            }
            return Err(e);
        }
        log.line(&format!(
            "{{\"event\":\"signal\",\"tid\":{tid},\"signal\":{signal}}}"
        ))?;
        resume(tid, signal)?; // Real SIGTRAP and all signal-delivery stops preserved.
    }
    guard.armed = false;
    let result = root_result.ok_or_else(|| fail("root exit status missing"))?;
    log.line(&format!(
        "{{\"event\":\"complete\",\"root_status\":{result},\"ioctls\":{seq}}}"
    ))?;
    Ok(result)
}
