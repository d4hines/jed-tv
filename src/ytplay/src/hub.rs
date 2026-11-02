//! YouTube's own entry screen; the TV launcher only needs one app tile.
use crate::{browse, fb, input, log, play_video};
use platform::remote::{self, Action};
use std::{io, time::Duration};

const ITEMS: [(&str, &str, &str); 5] = [
    ("Search", "Find a video with the remote", ""),
    ("Coffee Run", "Blender Studio - 3 minutes", "PVGeM40dABA"),
    ("Big Buck Bunny", "Blender Foundation - 10 minutes", "aqz-KE-bpKQ"),
    ("Spring", "Blender Studio - 8 minutes", "WhWc3b3KhnY"),
    ("Sprite Fright", "Blender Studio - 10 minutes", "_cMxraX_5RE"),
];

fn draw(screen: &mut fb::Framebuffer, selected: usize, status: &str) {
    let v = screen.var;
    let mut bb = screen.back_buffer();
    bb.fill_rect(0, 0, bb.width, bb.height, v.pixel(16, 18, 25));
    let white = v.pixel(245, 245, 250);
    let grey = v.pixel(175, 180, 195);
    browse::text(&mut bb, 72, 36, 5, white, "YouTube");
    browse::text(&mut bb, 72, 98, 2, grey, "Search and suggestions");
    for (i, (title, subtitle, _)) in ITEMS.iter().enumerate() {
        let y = 148 + i * 94;
        let color = if i == selected { v.pixel(170, 24, 38) } else { v.pixel(33, 37, 48) };
        bb.fill_rect(64, y, 1152, 82, color);
        browse::text(&mut bb, 88, y + 12, 3, white, title);
        browse::text(&mut bb, 88, y + 48, 2, grey, subtitle);
    }
    browse::text(&mut bb, 72, 646, 2, grey, "Up/Down: choose   OK: open   Back: home");
    let status: String = status.chars().take(70).collect();
    browse::text(&mut bb, 72, 682, 2, white, &status);
    drop(bb);
    let _ = screen.present();
}

pub fn run(server: &str, limit: usize, threads: u32) -> io::Result<()> {
    let mut screen = fb::Framebuffer::open("/dev/fb0")?;
    let mut devices = input::open_all();
    let mut selected = 0;
    let mut status = String::new();
    let mut search = browse::Browse::new(server, "");
    log("YouTube app: search and four suggestions");
    draw(&mut screen, selected, &status);
    loop {
        for (action, repeat) in remote::poll(&mut devices) {
            match action {
                Action::Up => selected = selected.saturating_sub(1),
                Action::Down => selected = (selected + 1).min(ITEMS.len() - 1),
                Action::Back | Action::Exit if !repeat => return Ok(()),
                Action::Ok if !repeat => {
                    status.clear();
                    let result = if selected == 0 {
                        (|| -> io::Result<()> {
                            while let Some(item) = search.interact()? {
                                let result = play_video(server, &item.id, "play", limit, threads);
                                if let Err(e) = &result {
                                    log(format!("YouTube: {} failed: {e}", item.id));
                                }
                                search.played(&result);
                            }
                            Ok(())
                        })()
                    } else {
                        status = format!("Loading {}...", ITEMS[selected].0);
                        draw(&mut screen, selected, &status);
                        play_video(server, ITEMS[selected].2, "play", limit, threads)
                    };
                    status = match result {
                        Ok(()) => String::new(),
                        Err(e) => { log(format!("YouTube: {e}")); format!("Could not open: {e}") }
                    };
                    // Reopen after playback: another framebuffer owner may have flipped pages.
                    screen = fb::Framebuffer::open("/dev/fb0")?;
                    // Each evdev handle has its own queue. The nested screen consumed
                    // Back on its handles; ours still contains that same press.
                    devices = input::open_all();
                    log("YouTube app: returned to suggestions");
                    draw(&mut screen, selected, &status);
                    break;
                }
                _ => continue,
            }
            draw(&mut screen, selected, &status);
        }
        std::thread::sleep(Duration::from_millis(16));
    }
}
