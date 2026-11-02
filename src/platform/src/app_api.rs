//! Small C ABI for Lean/native app clients. Handles stay inside this library.
//! Callers own handle lifetimes and must serialize calls on each handle.
use crate::{alsa, fb, input};
use std::{collections::VecDeque, io};

#[repr(C)]
pub struct Frame {
    pub pixels: *mut u32,
    pub width: u32,
    pub height: u32,
    pub stride_bytes: u32,
    pub red_shift: u32,
    pub green_shift: u32,
    pub blue_shift: u32,
    pub opaque_alpha: u32,
}

pub struct Display { inner: fb::Framebuffer, acquired: bool }
pub struct Audio(alsa::Pcm);
pub struct Input { devices: Vec<input::Device>, pending: VecDeque<input::Key> }

fn error(e: io::Error) -> i32 { -e.raw_os_error().unwrap_or(libc::EIO) }
fn result(r: io::Result<()>) -> i32 { r.map_or_else(error, |_| 0) }

#[no_mangle]
pub extern "C" fn tv_app_api_version() -> u32 { 0x0001_0000 }

#[no_mangle]
pub unsafe extern "C" fn tv_display_open(out: *mut *mut Display) -> i32 {
    if out.is_null() { return -libc::EINVAL; }
    out.write(std::ptr::null_mut());
    match fb::Framebuffer::open_default() {
        Ok(inner) => { out.write(Box::into_raw(Box::new(Display { inner, acquired: false }))); 0 }
        Err(e) => error(e),
    }
}

#[no_mangle]
pub unsafe extern "C" fn tv_display_acquire(display: *mut Display, out: *mut Frame) -> i32 {
    let Some(d) = display.as_mut() else { return -libc::EINVAL };
    if out.is_null() { return -libc::EINVAL; }
    if d.acquired { return -libc::EBUSY; }
    let v = d.inner.var;
    let mut lease = match d.inner.acquire() { Ok(v) => v, Err(e) => return error(e) };
    let pixels = lease.pixels();
    out.write(Frame { pixels: pixels.px.as_mut_ptr(), width: v.xres, height: v.yres,
        stride_bytes: (pixels.stride * 4) as u32, red_shift: v.red.offset,
        green_shift: v.green.offset, blue_shift: v.blue.offset,
        opaque_alpha: v.pixel(0, 0, 0) });
    // The C ABI keeps the lease until present/cancel/close instead of lexical drop.
    std::mem::forget(lease);
    d.acquired = true;
    0
}

#[no_mangle]
pub unsafe extern "C" fn tv_display_present(display: *mut Display) -> i32 {
    let Some(d) = display.as_mut() else { return -libc::EINVAL };
    if !d.acquired { return -libc::EINVAL; }
    d.acquired = false;
    let status = result(d.inner.present());
    d.inner.cancel_frame();
    status
}

#[no_mangle]
pub unsafe extern "C" fn tv_display_cancel(display: *mut Display) {
    if let Some(d) = display.as_mut() { d.inner.cancel_frame(); d.acquired = false; }
}

#[no_mangle]
pub unsafe extern "C" fn tv_display_close(display: *mut Display) {
    if !display.is_null() { drop(Box::from_raw(display)); }
}

#[no_mangle]
pub unsafe extern "C" fn tv_audio_open(rate: u32, channels: u32, out: *mut *mut Audio) -> i32 {
    if out.is_null() || !(8000..=96000).contains(&rate) || !(1..=2).contains(&channels) { return -libc::EINVAL; }
    out.write(std::ptr::null_mut());
    match alsa::Pcm::open("/dev/snd/pcmC0D0p", rate, channels) {
        Ok(pcm) => { out.write(Box::into_raw(Box::new(Audio(pcm)))); 0 }
        Err(e) => error(e),
    }
}

#[no_mangle]
pub unsafe extern "C" fn tv_audio_write(audio: *mut Audio, samples: *const i16, count: usize) -> i32 {
    let Some(a) = audio.as_mut() else { return -libc::EINVAL };
    if samples.is_null() || count > 524288 || count % a.0.channels as usize != 0 { return -libc::EINVAL; }
    result(a.0.write(std::slice::from_raw_parts(samples, count)))
}

#[no_mangle]
pub unsafe extern "C" fn tv_audio_queued(audio: *mut Audio, frames: *mut i64) -> i32 {
    let Some(a) = audio.as_mut() else { return -libc::EINVAL };
    if frames.is_null() { return -libc::EINVAL; }
    match a.0.delay() { Ok(n) => { frames.write(n); 0 }, Err(e) => error(e) }
}

#[no_mangle]
pub unsafe extern "C" fn tv_audio_reset(audio: *mut Audio) -> i32 {
    let Some(a) = audio.as_mut() else { return -libc::EINVAL };
    result(a.0.reset())
}

#[no_mangle]
pub unsafe extern "C" fn tv_audio_drain(audio: *mut Audio) -> i32 {
    let Some(a) = audio.as_mut() else { return -libc::EINVAL };
    result(a.0.drain())
}

#[no_mangle]
pub unsafe extern "C" fn tv_audio_close(audio: *mut Audio) {
    if !audio.is_null() { drop(Box::from_raw(audio)); }
}

#[no_mangle]
pub unsafe extern "C" fn tv_input_open(out: *mut *mut Input) -> i32 {
    if out.is_null() { return -libc::EINVAL; }
    out.write(std::ptr::null_mut());
    let devices = input::open_all();
    if devices.is_empty() { return -libc::ENODEV; }
    out.write(Box::into_raw(Box::new(Input { devices, pending: VecDeque::new() })));
    0
}

#[no_mangle]
pub unsafe extern "C" fn tv_input_poll(input: *mut Input, code: *mut u32, value: *mut i32) -> i32 {
    let Some(i) = input.as_mut() else { return -libc::EINVAL };
    if code.is_null() || value.is_null() { return -libc::EINVAL; }
    if i.pending.is_empty() {
        for d in &mut i.devices { match d.poll_keys() { Ok(keys) => i.pending.extend(keys), Err(e) => return error(e) } }
    }
    match i.pending.pop_front() {
        Some(key) => { code.write(key.code as u32); value.write(key.value); 1 }
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn tv_input_close(input: *mut Input) {
    if !input.is_null() { drop(Box::from_raw(input)); }
}
