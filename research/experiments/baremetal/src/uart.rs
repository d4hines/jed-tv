//! PL011 console (QEMU virt: 0x0900_0000), `println!`, and a `log` backend
//! so virtio-drivers' own messages show up. Lines carry a timestamp from the
//! generic timer (seconds since the counter started, i.e. VM reset).

use core::fmt::{self, Write};
use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{AtomicBool, Ordering};

const UART_DR: *mut u32 = 0x0900_0000 as *mut u32;
const UART_FR: *const u32 = 0x0900_0018 as *const u32;
const FR_TXFF: u32 = 1 << 5;

static LOCK: AtomicBool = AtomicBool::new(false);
/// Drop console output while set (see `quiet=1` in main.rs).
pub static QUIET: AtomicBool = AtomicBool::new(false);

pub struct Uart;

fn putc(b: u8) {
    unsafe {
        while read_volatile(UART_FR) & FR_TXFF != 0 {}
        write_volatile(UART_DR, b as u32);
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            if b == b'\n' {
                putc(b'\r');
            }
            putc(b);
        }
        Ok(())
    }
}

pub fn print_line(args: fmt::Arguments) {
    if QUIET.load(Ordering::Relaxed) {
        return;
    }
    while LOCK.compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
        core::hint::spin_loop();
    }
    let us = crate::timer::micros();
    let _ = write!(Uart, "[{:4}.{:06}] {}\n", us / 1_000_000, us % 1_000_000, args);
    LOCK.store(false, Ordering::Release);
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => { $crate::uart::print_line(format_args!($($arg)*)) };
}

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }
    fn log(&self, r: &log::Record) {
        print_line(format_args!("{} {}: {}", r.level(), r.target(), r.args()));
    }
    fn flush(&self) {}
}

static LOGGER: Logger = Logger;

pub fn init_logger(level: log::LevelFilter) {
    let _ = log::set_logger(&LOGGER);
    log::set_max_level(level);
}
