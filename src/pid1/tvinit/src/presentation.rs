//! QEMU's scanout adapter uses the same source gate as the physical TV.
use super::{fb, Control};
use g06_display::{frames::Frames, service::{Output, Service}};
use std::{io, sync::Arc, thread, time::Duration};

struct Framebuffer(fb::Framebuffer);
impl Output for Framebuffer {
    fn present(&mut self, pixels: &[u32]) -> io::Result<()> {
        let format = self.0.var;
        let target = self.0.back_buffer();
        for (dst, src) in target.px.iter_mut().zip(pixels) {
            *dst = format.pixel(*src as u8, (*src >> 8) as u8, (*src >> 16) as u8);
        }
        self.0.present()
    }
    fn hide(&mut self) -> io::Result<()> { Ok(()) }
}

pub(super) fn start(control: Arc<Control>, output: Option<String>) {
    thread::spawn(move || run(&control, output.as_deref()));
}

pub(super) fn run(control: &Control, external: Option<&str>) -> ! {
        let mut service = None;
        let mut source_name = String::new();
        let mut generation = u64::MAX;
        loop {
            let result = control.with_presentation(|path, epoch| -> io::Result<()> {
                if service.is_none() {
                    let frames = Frames::open(path)?;
                    let output = Framebuffer(if let Some(path) = external {
                        use std::os::unix::fs::PermissionsExt;
                        let output = fb::Framebuffer::shared_file(path,1280,720)?;
                        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))?;
                        output
                    } else { fb::Framebuffer::device("/dev/fb0")? });
                    service = Some(Service::from_parts(frames, output));
                    source_name = path.into();
                }
                let service = service.as_mut().unwrap();
                if source_name != path || generation != epoch {
                    service.set_source(Frames::open(path)?);
                    source_name = path.into(); generation = epoch;
                }
                service.tick(false)?;
                Ok(())
            });
            if let Err(error) = result {
                if !matches!(error.kind(), io::ErrorKind::NotFound | io::ErrorKind::InvalidData | io::ErrorKind::UnexpectedEof | io::ErrorKind::TimedOut | io::ErrorKind::WouldBlock) { super::log(format!("presentation: {error}")); }
            }
            thread::sleep(Duration::from_millis(3));
        }
}
