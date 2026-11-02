//! The Drowned Lighthouse runs locally in this process using its original Lean
//! engine and rules. No server, polling, credentials or network access.
//! Scene pictures are RGB24 files in /share/rpg, converted at build time.

#[cfg(target_os = "linux")]
fn main() {
    tv::main()
}

#[cfg(not(target_os = "linux"))]
fn main() {
    use std::io::Write;
    let mut engine = rpgtv::engine::Engine::new(12345);
    for step in std::env::args().skip(2) {
        engine.choose(step.parse().expect("option index"));
    }
    let screen = engine.screen();
    let out = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "rpgtv.ppm".into());
    let canvas = rpgtv::draw::Painter::new().render(&screen, 0, None, "Offline - Lean engine");
    let mut f = std::io::BufWriter::new(std::fs::File::create(&out).unwrap());
    write!(f, "P6\n{} {}\n255\n", rpgtv::draw::W, rpgtv::draw::H).unwrap();
    for p in canvas.px {
        f.write_all(&[(p >> 16) as u8, (p >> 8) as u8, p as u8])
            .unwrap();
    }
    println!("{out}: phase {}", screen.phase);
}

#[cfg(target_os = "linux")]
mod tv {
    use platform::remote::{self, Action};
    use platform::{fb, input, seccomp, sys};
    use rpgtv::draw::{Painter, H, W};
    use rpgtv::{engine::Engine, game::Screen};
    use std::time::Duration;

    fn log(msg: impl AsRef<str>) {
        let t = sys::monotonic();
        println!(
            "[{:>4}.{:03}] rpgtv: {}",
            t.as_secs(),
            t.subsec_millis(),
            msg.as_ref()
        );
    }

    struct App {
        engine: Engine,
        screen: Screen,
        selected: usize,
    }
    impl App {
        fn new(seed: u32) -> Self {
            let engine = Engine::new(seed);
            let screen = engine.screen();
            log(format!(
                "phase {} \"{}\", {} options",
                screen.phase,
                screen.heading,
                screen.options.len()
            ));
            Self {
                engine,
                screen,
                selected: 0,
            }
        }
        fn choose(&mut self, index: usize) {
            self.engine.choose(index);
            self.screen = self.engine.screen();
            self.selected = 0;
            log(format!(
                "phase {} \"{}\", {} options",
                self.screen.phase,
                self.screen.heading,
                self.screen.options.len()
            ));
        }
    }

    fn picture(name: &str) -> Option<Vec<u8>> {
        if name.is_empty()
            || !name
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            return None;
        }
        std::fs::read(format!("/share/rpg/{name}.rgb")).ok()
    }

    fn draw(screen: &mut fb::Framebuffer, painter: &mut Painter, app: &App) {
        let pic = app.screen.picture.as_deref().and_then(picture);
        let cv = painter.render(
            &app.screen,
            app.selected,
            pic.as_deref(),
            "Offline - Lean engine",
        );
        let var = screen.var;
        let native = var.red.offset == 16 && var.green.offset == 8 && var.blue.offset == 0;
        let bb = screen.back_buffer();
        let (w, h) = (bb.width.min(W), bb.height.min(H));
        let (ox, oy) = ((bb.width - w) / 2, (bb.height - h) / 2);
        for y in 0..h {
            let src = &cv.px[y * W..y * W + w];
            let dst = &mut bb.px[(y + oy) * bb.stride + ox..(y + oy) * bb.stride + ox + w];
            if native {
                dst.copy_from_slice(src);
            } else {
                for (d, &p) in dst.iter_mut().zip(src) {
                    *d = var.pixel((p >> 16) as u8, (p >> 8) as u8, p as u8);
                }
            }
        }
        let _ = screen.present();
    }

    pub fn main() {
        std::panic::set_hook(Box::new(|info| {
            eprintln!("rpgtv: PANIC: {info}");
            sys::power_off(101);
        }));
        let syscalls = seccomp::base_syscalls();
        match seccomp::install(&syscalls, &seccomp::platform_ioctls()) {
            Ok(()) => {
                log("seccomp allowlist installed (base syscalls + platform ioctls; no network)")
            }
            Err(e) => log(format!("seccomp: {e}")),
        }
        let mut screen = match fb::Framebuffer::open("/dev/fb0") {
            Ok(s) => s,
            Err(e) => {
                log(format!("display: {e}"));
                sys::power_off(1);
            }
        };
        let mut devs = input::open_all();
        let mut painter = Painter::new();
        let mut app = App::new(sys::monotonic().as_nanos() as u32);
        log("offline Lean engine ready; no server or sockets");
        draw(&mut screen, &mut painter, &app);
        loop {
            let mut dirty = false;
            for (action, _repeat) in remote::poll(&mut devs) {
                let n = app.screen.options.len();
                match action {
                    Action::Up if n > 0 => {
                        app.selected = (app.selected + n - 1) % n;
                        dirty = true;
                    }
                    Action::Down if n > 0 => {
                        app.selected = (app.selected + 1) % n;
                        dirty = true;
                    }
                    Action::Ok | Action::Digit(_) => {
                        let i = match action {
                            Action::Digit(d) => (d as usize).wrapping_sub(1),
                            _ => app.selected,
                        };
                        if i < n {
                            app.choose(i);
                        }
                        dirty = true;
                    }
                    Action::Back | Action::Exit => {
                        log("quit");
                        sys::power_off(0);
                    }
                    _ => {}
                }
            }
            if dirty {
                draw(&mut screen, &mut painter, &app);
            }
            std::thread::sleep(Duration::from_millis(20));
        }
    }
}
