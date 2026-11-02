//! Raw-byte transport for the scalar Lean sysfs parsers. No UTF-8 or numeric
//! grammar is interpreted here, including on native worker threads.
extern "C" {
    fn tv_storage_decimal_step(state: u32, number: u64, byte: u32) -> u32;
    fn tv_storage_decimal_value(state: u32, number: u64, byte: u32) -> u64;
    fn tv_storage_decimal_finish(state: u32) -> u8;
    fn tv_tee_device_step(state: u32, major: u64, minor: u64, byte: u32) -> u32;
    fn tv_tee_device_major(state: u32, major: u64, byte: u32) -> u64;
    fn tv_tee_device_minor(state: u32, minor: u64, byte: u32) -> u64;
    fn tv_storage_device_finish(state: u32, major: u64, minor: u64) -> u8;
}

pub(super) fn number(bytes: &[u8]) -> Option<u64> {
    let (mut state, mut value) = (0, 0);
    for &byte in bytes {
        let next = unsafe { tv_storage_decimal_step(state, value, byte.into()) };
        value = unsafe { tv_storage_decimal_value(state, value, byte.into()) };
        state = next;
    }
    (unsafe { tv_storage_decimal_finish(state) } != 0).then_some(value)
}

pub(super) fn device(bytes: &[u8]) -> Option<(u32, u32)> {
    let (mut state, mut major, mut minor) = (0, 0, 0);
    for &byte in bytes {
        let next = unsafe { tv_tee_device_step(state, major, minor, byte.into()) };
        major = unsafe { tv_tee_device_major(state, major, byte.into()) };
        minor = unsafe { tv_tee_device_minor(state, minor, byte.into()) };
        state = next;
    }
    (unsafe { tv_storage_device_finish(state, major, minor) } != 0)
        .then_some((major as u32, minor as u32))
}
