//! Supervisor-owned WPA control. No app gets this control socket or credentials.
use super::settings_store::{atomic_write, storage_root};
use std::os::unix::fs::MetadataExt;
use std::{
    fs, io,
    process::{Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, TryRecvError},
    },
    thread,
    time::Duration,
};

pub use super::wifi_codec::Network;
pub enum Reply {
    Scan(String),
    Status(String),
    Connection(String),
    Error(String),
}
pub struct Wifi {
    pub networks: Vec<Network>,
    pub status: String,
    pub connection: String,
    pending: Option<Receiver<Reply>>,
    needs_status: bool,
}
// Home can leave a settings page while connection setup finishes. A new page
// must not start a second controller operation over the same WPA/DHCP owners.
static BUSY: AtomicBool = AtomicBool::new(false);
struct Operation;
impl Drop for Operation {
    fn drop(&mut self) {
        BUSY.store(false, Ordering::Release);
    }
}

fn private_runtime() -> io::Result<()> {
    let meta = fs::symlink_metadata("/run/tv-network")?;
    if !meta.is_dir() || meta.uid() != 0 || meta.gid() != 0 || meta.mode() & 0o777 != 0o700 {
        return Err(io::Error::other("Network helper directory is not private to root"));
    }
    Ok(())
}
fn control(args: &[&str]) -> io::Result<String> {
    private_runtime()?;
    let output = Command::new("/run/tv-network/wpa_cli")
        .args(["-p", "/run/tv-network/wpa", "-s", "/run/tv-network", "-i", "wlan0"])
        .args(args)
        .output()?;
    let text = String::from_utf8_lossy(&output.stdout).into_owned();
    if !output.status.success() || text.lines().any(|s| s == "FAIL") {
        return Err(io::Error::other("Wi-Fi control failed"));
    }
    Ok(text)
}
fn prepare() -> io::Result<()> {
    private_runtime()?;
    if !std::path::Path::new("/run/tv-network/wpa_supplicant").is_file() {
        return Err(io::Error::other("Wi-Fi tools have not been staged"));
    }
    if !Command::new("/bin/sh")
        .arg("/run/tv-network/prepare.sh")
        .output()?
        .status
        .success()
    {
        return Err(io::Error::other("Wi-Fi adapter could not start"));
    }
    Ok(())
}

// Only signal the daemon whose recorded argv identifies our wlan0 DHCP owner.
fn stop_dhcp() -> io::Result<()> {
    private_runtime()?;
    let text = match fs::read_to_string("/run/tv-network/udhcpc.pid") {
        Ok(text) => text,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(e),
    };
    let pid: i32 = text
        .trim()
        .parse()
        .map_err(|_| io::Error::other("Invalid Wi-Fi DHCP pid"))?;
    if pid <= 1 {
        return Err(io::Error::other("Invalid Wi-Fi DHCP pid"));
    }
    let cmdline = match fs::read(format!("/proc/{pid}/cmdline")) {
        Ok(line) => line,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            let _ = fs::remove_file("/run/tv-network/udhcpc.pid");
            return Ok(());
        }
        Err(e) => return Err(e),
    };
    let args: Vec<_> = cmdline
        .split(|b| *b == 0)
        .filter(|a| !a.is_empty())
        .collect();
    if !args.first().is_some_and(|a| a.ends_with(b"udhcpc"))
        || !args
            .windows(2)
            .any(|a| a == [b"-i".as_slice(), b"wlan0".as_slice()])
    {
        return Err(io::Error::other("Wi-Fi DHCP owner did not match"));
    }
    if unsafe { libc::kill(pid, libc::SIGTERM) } != 0 {
        return Err(io::Error::last_os_error());
    }
    // Give the old daemon time to exit before a new lease hook can run.
    for _ in 0..100 {
        if !std::path::Path::new(&format!("/proc/{pid}")).exists() {
            let _ = fs::remove_file("/run/tv-network/udhcpc.pid");
            return Ok(());
        }
        thread::sleep(Duration::from_millis(10));
    }
    Err(io::Error::other(
        "Previous Wi-Fi DHCP process has not exited",
    ))
}
fn clear_address() -> io::Result<()> {
    if !Command::new("/bin/ip")
        .args(["addr", "flush", "dev", "wlan0"])
        .status()?
        .success()
    {
        return Err(io::Error::other("Could not clear Wi-Fi address"));
    }
    Ok(())
}

fn connection_status() -> String {
    if !std::path::Path::new("/sys/class/net/wlan0").exists() {
        return "Wi-Fi adapter is not active".into();
    }
    let status = match control(&["status"]) {
        Ok(status) => status,
        Err(_) => return "Wi-Fi is not connected".into(),
    };
    let field = |name: &str| status.lines().find_map(|line| line.strip_prefix(name));
    if field("wpa_state=") != Some("COMPLETED") {
        return format!(
            "Wi-Fi state: {}",
            field("wpa_state=").unwrap_or("disconnected")
        );
    }
    let ip = Command::new("/bin/ip")
        .args(["-4", "-o", "addr", "show", "dev", "wlan0"])
        .output()
        .ok()
        .filter(|p| p.status.success())
        .map(|p| String::from_utf8_lossy(&p.stdout).into_owned())
        .and_then(|text| {
            text.split_whitespace()
                .collect::<Vec<_>>()
                .windows(2)
                .find(|pair| pair[0] == "inet")
                .map(|pair| pair[1].to_owned())
        });
    format!(
        "Connected to {}: {}",
        field("ssid=").unwrap_or("saved network"),
        ip.as_deref().unwrap_or("waiting for IP address")
    )
}

impl Wifi {
    pub fn new() -> Self {
        Self {
            networks: Vec::new(),
            status: "Select Scan to find Wi-Fi networks".into(),
            connection: "Wi-Fi state has not been read".into(),
            pending: None,
            needs_status: true,
        }
    }
    fn start(&mut self, f: impl FnOnce() -> io::Result<Reply> + Send + 'static) {
        if BUSY
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            self.status = "A Wi-Fi operation is already running".into();
            return;
        }
        let (tx, rx) = mpsc::channel();
        self.pending = Some(rx);
        self.status = "Working... Home and Back still work".into();
        thread::spawn(move || {
            let _operation = Operation;
            let reply = f().unwrap_or_else(|e| Reply::Error(e.to_string()));
            let _ = tx.send(reply);
        });
    }
    pub fn poll(&mut self) -> bool {
        let Some(rx) = &self.pending else {
            // A previous page can leave a connection operation running. Once
            // it finishes, retry this page's read-only status request exactly
            // once, rather than leaving an obsolete "busy" message forever.
            if self.needs_status && !BUSY.load(Ordering::Acquire) {
                self.refresh_status();
            }
            return false;
        };
        let reply = match rx.try_recv() {
            Ok(reply) => reply,
            Err(TryRecvError::Empty) => return false,
            Err(TryRecvError::Disconnected) => Reply::Error("Wi-Fi worker stopped".into()),
        };
        self.pending = None;
        self.needs_status = false;
        match reply {
            Reply::Scan(text) => match super::wifi_codec::parse_scan(&text) {
                Ok(networks) => {
                    self.networks = networks;
                    self.status = format!("{} networks found", self.networks.len());
                }
                Err(error) => self.status = error.to_string(),
            },
            Reply::Connection(text) => {
                self.connection = text;
                self.status = "Select Scan to find Wi-Fi networks".into();
            }
            Reply::Status(text) => {
                self.connection = text.clone();
                self.status = text;
            }
            Reply::Error(text) => self.status = text,
        }
        true
    }
    pub fn refresh_status(&mut self) {
        self.start(|| Ok(Reply::Connection(connection_status())));
    }
    pub fn scan(&mut self) {
        self.start(|| {
            prepare()?;
            control(&["scan"])?;
            thread::sleep(Duration::from_secs(3));
            Ok(Reply::Scan(control(&["scan_results"])?))
        });
    }
    pub fn disconnect(&mut self) {
        self.start(|| {
            control(&["disconnect"])?;
            stop_dhcp()?;
            clear_address()?;
            Ok(Reply::Status(
                "Wi-Fi disconnected; wired debug link retained".into(),
            ))
        });
    }
    pub fn connect(&mut self, ssid: String, password: String, secure: bool) {
        let config = match super::wifi_codec::configuration(&ssid, &password, secure) {
            Ok(config) => config,
            Err(error) => { self.status = error.to_string(); return; }
        };
        self.start(move || {
            private_runtime()?;
            atomic_write(std::path::Path::new("/run/tv-network/wifi.conf"), config.as_bytes())?;
            prepare()?;
            stop_dhcp()?;
            clear_address()?;
            control(&["reconfigure"])?;
            control(&["reconnect"])?;
            for _ in 0..30 {
                let status = control(&["status"])?;
                if status.lines().any(|s| s == "wpa_state=COMPLETED") {
                    if !Command::new("/bin/ip")
                        .args([
                            "route",
                            "replace",
                            "192.168.77.1/32",
                            "dev",
                            "eth0",
                            "src",
                            "192.168.77.2",
                        ])
                        .status()?
                        .success()
                    {
                        return Err(io::Error::other("Could not preserve the wired debug route"));
                    }
                    // udhcpc remains as a renewal daemon after obtaining the lease;
                    // do not pipe stdout/stderr, whose inherited descriptors would
                    // keep Command::output waiting after that fork.
                    let result = Command::new("/bin/udhcpc")
                        .args([
                            "-i",
                            "wlan0",
                            "-s",
                            "/run/tv-network/lease.sh",
                            "-p",
                            "/run/tv-network/udhcpc.pid",
                            "-n",
                            "-t",
                            "5",
                            "-T",
                            "3",
                        ])
                        .stdin(Stdio::null())
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .status()?;
                    if !result.success() {
                        return Err(io::Error::other(
                            "Connected, but DHCP did not provide an address",
                        ));
                    }
                    atomic_write(&storage_root()?.join("wifi.conf"), config.as_bytes())?;
                    return Ok(Reply::Status(connection_status()));
                }
                thread::sleep(Duration::from_secs(1));
            }
            Err(io::Error::other(
                "Connection timed out; check the password and router",
            ))
        });
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn raw_scan_worker_result_runs_policy_on_the_supervisor_thread() {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || tx.send(Reply::Scan(
            "heading\na\t2412\t-40\t[WPA2-PSK]\tsynthetic\n".into())).unwrap()).join().unwrap();
        let mut wifi = Wifi::new();
        wifi.pending = Some(rx);
        assert!(wifi.poll());
        assert_eq!(wifi.networks.len(), 1);
        assert_eq!(wifi.networks[0].ssid, "synthetic");
        assert!(wifi.networks[0].secure);
        assert_eq!(wifi.status, "1 networks found");
        // Invalid config is rejected on this thread before any worker starts.
        wifi.connect(String::new(), String::new(), false);
        assert!(wifi.pending.is_none());
        assert_eq!(wifi.status, "Invalid Wi-Fi network name");
    }
    #[test]
    fn disconnected_worker_is_reported_instead_of_staying_busy() {
        let (tx, rx) = mpsc::channel();
        drop(tx);
        let mut wifi = Wifi::new();
        wifi.pending = Some(rx);
        assert!(wifi.poll());
        assert!(wifi.pending.is_none());
        assert_eq!(wifi.status, "Wi-Fi worker stopped");
    }
}
