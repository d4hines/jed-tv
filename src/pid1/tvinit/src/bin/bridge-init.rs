//! QEMU-only adapter: exercise the TV's shared-frame and PCM protocols.
//! This is not vendor hardware emulation and is never installed on the TV.
use platform::{fb, net, sys};
use std::{process::Command, time::Duration};

fn main() {
    assert!(sys::is_pid1());
    sys::init_mounts();
    std::fs::create_dir_all("/tmp").unwrap();
    net::configure_ipv4("eth0", std::net::Ipv4Addr::new(192, 168, 77, 2), 24).unwrap();
    let mut output = loop {
        match fb::Framebuffer::open("/dev/fb0") {
            Ok(fb) => break fb,
            Err(_) => std::thread::sleep(Duration::from_millis(50)),
        }
    };
    let audio = sys::cmdline_arg("bridge.audio").expect("bridge.audio");
    let mut child = Command::new("/bin/tvinit")
        .env("TV_FB_PATH", "/tmp/app.frames")
        .env("TV_AUDIO_ADDR", audio)
        .spawn().unwrap();
    let mut input = fb::Framebuffer::shared_file("/tmp/app.frames", 1280, 720).unwrap();
    let mut count = 0u64;
    println!("bridge-init: shared RGBA and TCP PCM adapters active");
    loop {
        // Reopen to pick up the app's currently published front page.
        let frame = input.read_presented().unwrap();
        if !frame.pixels().iter().all(|p| p >> 24 == 255) {
            std::thread::sleep(Duration::from_millis(33));
            continue;
        }
        let var = output.var;
        let mut target = output.back_buffer();
        for y in 0..720 {
            for x in 0..1280 {
                let pixel = frame.pixels()[y * 1280 + x];
                target.px[y * target.stride + x] = var.pixel(pixel as u8, (pixel >> 8) as u8, (pixel >> 16) as u8);
            }
        }
        drop(target);
        output.present().unwrap();
        count += 1;
        if count == 30 { println!("bridge-init: opaque shared frames presented"); }
        assert!(child.try_wait().unwrap().is_none(), "launcher exited");
        std::thread::sleep(Duration::from_millis(33));
    }
}
