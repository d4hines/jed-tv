//! Framebuffer and Wi-Fi primitives for the Lean menu. No menu state, policy,
//! PIN comparison, navigation, or layout decisions live in this adapter.
use super::{
    about_text, arg, fb,
    lean_runtime::{self, Object},
    load_art, log, settings_wifi, Painter, FOCUS_H, FOCUS_W,
};
use std::{cell::Cell, collections::HashMap, io, sync::OnceLock};

type Launch = (&'static str, &'static [&'static str]);
const EXECUTABLES: [&str; 9] = [
    "/bin/ytplay",
    "/bin/slides",
    "/bin/doomtv",
    "/bin/rpgtv",
    "/bin/flighttv",
    "/bin/lean/ytplay",
    "/bin/lean/doomtv",
    "/bin/axiomtv",
    "/bin/snaketv",
];
extern "C" {
    fn initialize_tv_TV_MenuView_Native(builtin: u8) -> Object;
    fn tv_menu_create(installed: u32, demo: u32, width: u32, height: u32, offset: i32) -> Object;
    fn tv_menu_config(kind: u32, value: u32) -> Object;
    fn tv_menu_show(active: u32) -> Object;
    fn tv_menu_key(kind: u32, code: u32, value: u32) -> Object;
    fn tv_menu_notice(kind: u32, app: u32) -> Object;
    fn tv_menu_tick(unix: u64, dirty: u32) -> Object;
    fn tv_menu_launch(app: u32) -> Object;
    fn tv_menu_wifi_begin(status: Object, connection: Object) -> Object;
    fn tv_menu_wifi_network(ssid: Object, signal: Object, secure: u32) -> Object;
    fn tv_menu_about_begin() -> Object;
    fn tv_menu_about_line(line: Object) -> Object;
    fn tv_menu_io_unit(unused: u32) -> Object;
    fn lean_mk_string_from_bytes(data: *const u8, length: usize) -> Object;
}

// Exact lean_string_object layout in our pinned Lean 4.35 lean.h. This is the
// sole borrowed-string decoder; callers never retain a slice beyond the FFI call.
#[repr(C)]
struct LeanString {
    header: [u8; 8],
    size: usize,
    capacity: usize,
    length: usize,
}
unsafe fn borrowed_string<'a>(object: Object) -> &'a str {
    let string = &*object.cast::<LeanString>();
    debug_assert_eq!(string.header[7], 249);
    let data = object.cast::<u8>().add(std::mem::size_of::<LeanString>());
    std::str::from_utf8_unchecked(std::slice::from_raw_parts(data, string.size - 1))
}
fn owned_string(text: &str) -> Object {
    unsafe { lean_mk_string_from_bytes(text.as_ptr(), text.len()) }
}

static INITIALIZED: OnceLock<Result<(), String>> = OnceLock::new();
thread_local! { static CURRENT:Cell<*mut CoreMenu> = const { Cell::new(std::ptr::null_mut()) }; }
struct Context(*mut CoreMenu);
impl Drop for Context {
    fn drop(&mut self) {
        CURRENT.with(|c| c.set(self.0));
    }
}

pub(super) struct CoreMenu {
    screen: fb::Framebuffer,
    art: HashMap<String, Option<Vec<u8>>>,
    wifi: settings_wifi::Wifi,
    render_error: Option<io::Error>,
    reply: u64,
    demo: bool,
    reported_ready: bool,
}
impl CoreMenu {
    pub fn new() -> io::Result<Self> {
        lean_runtime::initialize_runtime()?;
        match INITIALIZED.get_or_init(|| unsafe {
            lean_runtime::consume_io_unit(initialize_tv_TV_MenuView_Native(1))
                .map_err(|e| e.to_string())
        }) {
            Ok(()) => {}
            Err(e) => return Err(io::Error::other(e.clone())),
        }
        let screen = if std::env::var_os("TV_FB_PATH").is_some() {
            let dir = std::path::Path::new("/run/tv-core");
            std::fs::create_dir_all(dir)?;
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(dir, std::fs::Permissions::from_mode(0o700))?;
            let path = dir.join("menu.frames");
            let screen = fb::Framebuffer::shared_file(path.to_str().unwrap(), 1280, 720)?;
            std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))?;
            screen
        } else {
            fb::Framebuffer::open("/dev/fb0")?
        };
        let demo = arg("tvinit.demo").as_deref() == Some("1");
        let installed = EXECUTABLES.iter().enumerate().fold(0, |mask, (i, path)| {
            mask | if std::path::Path::new(path).is_file() {
                1 << i
            } else {
                0
            }
        });
        let offset = arg("tvinit.utc_offset")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let (width, height) = (screen.var.xres, screen.var.yres);
        let mut this = Self {
            screen,
            art: HashMap::new(),
            wifi: settings_wifi::Wifi::new(),
            render_error: None,
            reply: 0,
            demo,
            reported_ready: false,
        };
        this.call(|| unsafe { tv_menu_create(installed, demo as u32, width, height, offset) })?;
        this.call(|| unsafe { tv_menu_about_begin() })?;
        for line in about_text() {
            this.call(|| unsafe { tv_menu_about_line(owned_string(&line)) })?;
        }
        this.sync_wifi()?;
        Ok(this)
    }
    fn call(&mut self, f: impl FnOnce() -> Object) -> io::Result<u64> {
        let previous = CURRENT.with(|c| c.replace(self));
        let _context = Context(previous);
        self.reply = 0;
        let result = unsafe { lean_runtime::consume_io_unit(f()) }.map(|()| self.reply);
        if let Some(error) = self.render_error.take() {
            return Err(error);
        }
        result
    }
    fn invoke(&mut self, f: impl FnOnce() -> Object) -> u64 {
        match self.call(f) {
            Ok(value) => value,
            Err(error) => {
                log(format!("Lean menu: {error}"));
                0
            }
        }
    }
    fn sync_wifi(&mut self) -> io::Result<()> {
        let status = owned_string(&self.wifi.status);
        let connection = owned_string(&self.wifi.connection);
        self.call(|| unsafe { tv_menu_wifi_begin(status, connection) })?;
        for network in self.wifi.networks.clone() {
            self.call(|| unsafe {
                tv_menu_wifi_network(
                    owned_string(&network.ssid),
                    owned_string(&network.signal),
                    network.secure as u32,
                )
            })?;
        }
        Ok(())
    }
    pub fn set_storage_available(&mut self, available: bool) {
        self.invoke(|| unsafe { tv_menu_config(1, available as u32) });
    }
    pub fn set_enabled(&mut self, mask: u32) {
        self.invoke(|| unsafe { tv_menu_config(0, mask) });
    }
    pub fn show_home(&mut self) {
        let counts = self.invoke(|| unsafe { tv_menu_show(1) });
        if !self.reported_ready {
            self.reported_ready = true;
            log(format!(
                "home screen: {} rows, {} tiles; Lean Core",
                counts >> 32,
                counts as u32
            ));
        }
    }
    pub fn show_app(&mut self) {
        self.invoke(|| unsafe { tv_menu_show(0) });
    }
    pub fn handle_key(&mut self, kind: u8, code: u32, value: i32) -> Option<u64> {
        let before = (self.wifi.status.clone(), self.wifi.connection.clone());
        let choice = self.invoke(|| unsafe { tv_menu_key(kind as u32, code, value as u32) });
        if before != (self.wifi.status.clone(), self.wifi.connection.clone()) {
            if let Err(e) = self.sync_wifi() {
                log(format!("Lean Wi-Fi view: {e}"));
            }
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0, |v| v.as_secs());
            self.invoke(|| unsafe { tv_menu_tick(now, 1) });
        }
        if choice == 0 {
            None
        } else {
            Some(choice)
        }
    }
    pub fn tell(&mut self, kind: u8, app: u32) {
        self.invoke(|| unsafe { tv_menu_notice(kind as u32, app) });
    }
    pub fn poll(&mut self) -> bool {
        let dirty = self.wifi.poll();
        if dirty {
            if let Err(e) = self.sync_wifi() {
                log(format!("Lean Wi-Fi view: {e}"));
            }
        }
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |v| v.as_secs());
        self.invoke(|| unsafe { tv_menu_tick(now, dirty as u32) }) != 0
    }
    pub fn take_launch(&mut self, app: u32) -> Option<Launch> {
        let token = self.invoke(|| unsafe { tv_menu_launch(app) }) as u32;
        let path = match (token & 255, token & 256 != 0) {
            (1, false) => "/bin/ytplay",
            (1, true) => "/bin/lean/ytplay",
            (2, false) => "/bin/doomtv",
            (2, true) => "/bin/lean/doomtv",
            (3, _) => "/bin/slides",
            (5, _) => "/bin/rpgtv",
            (6, _) => "/bin/flighttv",
            (7, _) => "/bin/axiomtv",
            (8, _) => "/bin/snaketv",
            _ => return None,
        };
        let args: &'static [&'static str] = if token & 512 != 0 {
            &["ytplay.video=demo"]
        } else if token & 255 == 1 {
            if self.demo {
                &["ytplay.mode=app"]
            } else {
                &["ytplay.mode=app", "ytplay.server=https://www.youtube.com"]
            }
        } else {
            &[]
        };
        Some((path, args))
    }
    fn paint(&mut self, op: u32, x: i32, y: i32, w: u32, h: u32, a: u32, b: u32, text: &str) {
        if op == 5 && !self.art.contains_key(text) {
            self.art.insert(text.into(), load_art(text));
        }
        let format = self.screen.var;
        let mut p = Painter {
            bb: self.screen.back_buffer(),
            format,
        };
        let rgb = |c: u32| ((c >> 16) as u8, (c >> 8) as u8, c as u8);
        let color = |c: u32| {
            let (r, g, b) = rgb(c);
            format.pixel(r, g, b)
        };
        let (x, y, w, h) = (x as isize, y as isize, w as usize, h as usize);
        match op {
            0 => p.gradient(rgb(a), rgb(b)),
            1 => p.rect(x, y, w, h, color(a)),
            2 => p.frame(x, y, w, h, b as usize, color(a)),
            3 => {
                for yy in y.max(0)..(y + h as isize).min(p.bb.height as isize) {
                    for xx in x.max(0)..(x + w as isize).min(p.bb.width as isize) {
                        let i = yy as usize * p.bb.stride + xx as usize;
                        p.bb.px[i] = format.dim(p.bb.px[i]);
                    }
                }
            }
            4 => {
                p.text(x, y, w, color(a), text);
            }
            5 => {
                if let Some(Some(art)) = self.art.get(text) {
                    p.art(x, y, w, h, art, FOCUS_W, FOCUS_H);
                }
            }
            _ => {}
        }
    }
}

// The scoped context exists only during a synchronous Lean call on the Core
// thread. Callbacks return Lean-created IO results and never retain its strings.
#[no_mangle]
unsafe extern "C" fn tv_menu_paint(
    op: u32,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    a: u32,
    b: u32,
    text: Object,
) -> Object {
    CURRENT.with(|c| {
        if let Some(menu) = c.get().as_mut() {
            menu.paint(op, x, y, w, h, a, b, borrowed_string(text));
        }
    });
    tv_menu_io_unit(0)
}
#[no_mangle]
unsafe extern "C" fn tv_menu_present() -> Object {
    CURRENT.with(|c| {
        if let Some(menu) = c.get().as_mut() {
            if let Err(e) = menu.screen.present() {
                menu.render_error = Some(e);
            }
        }
    });
    tv_menu_io_unit(0)
}
#[no_mangle]
unsafe extern "C" fn tv_menu_wifi(op: u32, secure: u32, ssid: Object, password: Object) -> Object {
    CURRENT.with(|c| {
        if let Some(menu) = c.get().as_mut() {
            match op {
                0 => menu.wifi.scan(),
                1 => menu.wifi.disconnect(),
                2 => menu.wifi.connect(
                    borrowed_string(ssid).into(),
                    borrowed_string(password).into(),
                    secure != 0,
                ),
                _ => {}
            }
        }
    });
    tv_menu_io_unit(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lean_menu_drives_rendering_variants_and_typed_pin_requests() {
        lean_runtime::initialize_runtime().unwrap();
        unsafe {
            lean_runtime::consume_io_unit(initialize_tv_TV_MenuView_Native(1)).unwrap();
        }
        let path = std::env::temp_dir().join(format!("lean-menu-{}", std::process::id()));
        let screen = fb::Framebuffer::shared_file(path.to_str().unwrap(), 1280, 720).unwrap();
        let mut menu = CoreMenu {
            screen,
            art: HashMap::new(),
            wifi: settings_wifi::Wifi::new(),
            render_error: None,
            reply: 0,
            demo: false,
            reported_ready: false,
        };
        menu.call(|| unsafe { tv_menu_create(511, 0, 1280, 720, -420) })
            .unwrap();
        menu.set_storage_available(true);
        menu.set_enabled(55);
        let counts = menu.call(|| unsafe { tv_menu_show(1) }).unwrap();
        assert_eq!(counts, (2u64 << 32) | 12);
        let bytes = std::fs::read(&path).unwrap();
        let word = |offset| u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
        let page = word(16) as usize;
        assert_eq!(
            word(4096 + page * 1280 * 720 * 4 + (20 * 1280 + 20) * 4),
            menu.screen.var.pixel(14, 16, 28)
        );
        assert_eq!(menu.handle_key(2, 106, 0), None);
        assert_eq!(menu.handle_key(2, 28, 1), Some(2 | (1 << 16)));
        assert_eq!(menu.take_launch(1).unwrap().0, "/bin/ytplay");
        for _ in 0..5 {
            menu.handle_key(2, 106, 1);
        }
        assert_eq!(menu.handle_key(2, 28, 1), Some(2 | (1 << 16)));
        assert_eq!(menu.take_launch(1).unwrap().0, "/bin/lean/ytplay");
        menu.handle_key(0, 0, 1);
        menu.handle_key(2, 108, 1);
        assert_eq!(menu.handle_key(2, 28, 1), None);
        menu.handle_key(2, 108, 1);
        assert_eq!(menu.handle_key(2, 28, 1), None);
        for _ in 0..4 {
            menu.handle_key(2, 2, 1);
        }
        assert_eq!(menu.handle_key(2, 28, 1), None);
        for _ in 0..4 {
            menu.handle_key(2, 3, 1);
        }
        assert_eq!(menu.handle_key(2, 28, 1), None);
        for _ in 0..4 {
            menu.handle_key(2, 3, 1);
        }
        assert_eq!(
            menu.handle_key(2, 28, 1),
            Some(2 | (1 << 8) | (1111u64 << 32) | (2222u64 << 48))
        );
        menu.handle_key(0, 0, 1);
        for _ in 0..7 {
            menu.handle_key(2, 106, 1);
        }
        assert_eq!(menu.handle_key(2, 28, 1), Some(2 | (7 << 16)));
        assert_eq!(menu.take_launch(7).unwrap().0, "/bin/axiomtv");
        menu.handle_key(2, 106, 1);
        assert_eq!(menu.handle_key(2, 28, 1), Some(2 | (8 << 16)));
        assert_eq!(menu.take_launch(8).unwrap().0, "/bin/snaketv");
        menu.wifi.networks.push(settings_wifi::Network {
            ssid: "Test Wi-Fi".into(),
            signal: "-42".into(),
            secure: true,
        });
        menu.sync_wifi().unwrap();
        menu.handle_key(0, 0, 1);
        menu.handle_key(2, 108, 1);
        menu.handle_key(2, 28, 1);
        for _ in 0..2 {
            menu.handle_key(2, 108, 1);
        }
        menu.handle_key(2, 28, 1);
        for _ in 0..2 {
            menu.handle_key(2, 108, 1);
        }
        assert_eq!(menu.handle_key(2, 28, 1), None);
        for code in [2, 3, 4, 399, 158] {
            assert_eq!(menu.handle_key(2, code, 1), None);
        }
        menu.handle_key(0, 0, 1);
        menu.handle_key(2, 108, 1);
        menu.handle_key(2, 28, 1);
        for _ in 0..3 {
            menu.handle_key(2, 108, 1);
        }
        assert_eq!(menu.handle_key(2, 28, 1), None);
        let text = "A☃\0B";
        let object = owned_string(text);
        assert_eq!(unsafe { borrowed_string(object) }, text);
        menu.call(|| unsafe { tv_menu_about_line(object) }).unwrap();
        menu.show_app();
        assert_eq!(menu.handle_key(2, 28, 1), None);
        drop(menu);
        std::fs::remove_file(path).unwrap();
    }
}

#[no_mangle]
unsafe extern "C" fn tv_menu_reply(low: u32, high: u32) -> Object {
    CURRENT.with(|c| {
        if let Some(menu) = c.get().as_mut() {
            menu.reply = (high as u64) << 32 | low as u64;
        }
    });
    tv_menu_io_unit(0)
}
